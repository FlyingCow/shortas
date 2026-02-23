using System.Text;
using System.Text.Json;
using Microsoft.EntityFrameworkCore;
using RabbitMQ.Client;
using RabbitMQ.Client.Events;
using ShortasProxyApi.Domain.Entities;
using ShortasProxyApi.Infrastructure.Data;

namespace ShortasProxyApi.Infrastructure.BackgroundServices;

/// <summary>
/// Background service that consumes route status change events from RabbitMQ.
/// These events are published by the route-verifier service when routes are blocked
/// due to Safe Browsing threats.
/// </summary>
public class RouteStatusConsumer : BackgroundService
{
    private readonly IServiceProvider _serviceProvider;
    private readonly ILogger<RouteStatusConsumer> _logger;
    private readonly IConfiguration _configuration;
    private IConnection? _connection;
    private IChannel? _channel;
    private readonly string _exchangeName;
    private readonly string _queueName;

    public RouteStatusConsumer(
        IServiceProvider serviceProvider,
        ILogger<RouteStatusConsumer> logger,
        IConfiguration configuration)
    {
        _serviceProvider = serviceProvider;
        _logger = logger;
        _configuration = configuration;
        _exchangeName = configuration["RabbitMQ:RouteStatusExchange"] ?? "route.status.changed";
        _queueName = configuration["RabbitMQ:RouteStatusQueue"] ?? "management-api.route-status";
    }

    protected override async Task ExecuteAsync(CancellationToken stoppingToken)
    {
        _logger.LogInformation("Route Status Consumer starting...");

        while (!stoppingToken.IsCancellationRequested)
        {
            try
            {
                await ConnectAndConsumeAsync(stoppingToken);
            }
            catch (Exception ex)
            {
                _logger.LogError(ex, "Error in Route Status Consumer, reconnecting in 5 seconds...");
                await Task.Delay(TimeSpan.FromSeconds(5), stoppingToken);
            }
        }

        _logger.LogInformation("Route Status Consumer stopped");
    }

    private async Task ConnectAndConsumeAsync(CancellationToken stoppingToken)
    {
        var rabbitMqUri = _configuration["RabbitMQ:Uri"] ?? "amqp://guest:guest@localhost:5672/%2f";

        var factory = new ConnectionFactory
        {
            Uri = new Uri(rabbitMqUri)
        };

        _connection = await factory.CreateConnectionAsync(stoppingToken);
        _channel = await _connection.CreateChannelAsync(cancellationToken: stoppingToken);

        // Declare the exchange (should match route-verifier's exchange)
        await _channel.ExchangeDeclareAsync(
            exchange: _exchangeName,
            type: ExchangeType.Fanout,
            durable: true,
            autoDelete: false,
            cancellationToken: stoppingToken);

        // Declare the queue for this consumer
        await _channel.QueueDeclareAsync(
            queue: _queueName,
            durable: true,
            exclusive: false,
            autoDelete: false,
            cancellationToken: stoppingToken);

        // Bind queue to exchange
        await _channel.QueueBindAsync(
            queue: _queueName,
            exchange: _exchangeName,
            routingKey: "",
            cancellationToken: stoppingToken);

        var consumer = new AsyncEventingBasicConsumer(_channel);
        consumer.ReceivedAsync += async (model, ea) =>
        {
            try
            {
                var body = ea.Body.ToArray();
                var message = Encoding.UTF8.GetString(body);

                await ProcessMessageAsync(message, stoppingToken);

                await _channel.BasicAckAsync(ea.DeliveryTag, multiple: false, stoppingToken);
            }
            catch (Exception ex)
            {
                _logger.LogError(ex, "Error processing route status message");
                // Nack without requeue to avoid infinite loops on poison messages
                await _channel.BasicNackAsync(ea.DeliveryTag, multiple: false, requeue: false, stoppingToken);
            }
        };

        await _channel.BasicConsumeAsync(
            queue: _queueName,
            autoAck: false,
            consumer: consumer,
            cancellationToken: stoppingToken);

        _logger.LogInformation("Route Status Consumer connected and consuming from {Queue}", _queueName);

        // Keep the connection alive
        while (!stoppingToken.IsCancellationRequested && _connection.IsOpen)
        {
            await Task.Delay(TimeSpan.FromSeconds(10), stoppingToken);
        }
    }

    private async Task ProcessMessageAsync(string message, CancellationToken cancellationToken)
    {
        var options = new JsonSerializerOptions
        {
            PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower
        };

        var statusChange = JsonSerializer.Deserialize<RouteStatusChangedMessage>(message, options);
        if (statusChange == null)
        {
            _logger.LogWarning("Failed to deserialize route status message: {Message}", message);
            return;
        }

        _logger.LogInformation(
            "Received route status change: {RouteId} -> {Status} (reason: {Reason})",
            statusChange.RouteId,
            statusChange.NewStatus,
            statusChange.BlockedReason ?? "N/A");

        using var scope = _serviceProvider.CreateScope();
        var dbContext = scope.ServiceProvider.GetRequiredService<ApplicationDbContext>();

        if (!Guid.TryParse(statusChange.RouteId, out var routeId))
        {
            _logger.LogWarning("Invalid route ID in message: {RouteId}", statusChange.RouteId);
            return;
        }

        var route = await dbContext.Routes
            .FirstOrDefaultAsync(r => r.Id == routeId, cancellationToken);

        if (route == null)
        {
            _logger.LogWarning("Route not found: {RouteId}", statusChange.RouteId);
            return;
        }

        // Update route status
        var previousStatus = route.Status;
        route.Status = statusChange.NewStatus ?? "Active";

        // Create outbox message to propagate status change to click-router-api
        var outboxPayload = new
        {
            route_id = route.Id.ToString(),
            link = route.Link,
            status = route.Status,
            blocked_reason = statusChange.BlockedReason
        };

        var outboxMessage = new OutboxMessage
        {
            EventType = OutboxEventType.RouteStatusUpdated,
            AggregateId = route.Id.ToString(),
            Payload = JsonSerializer.Serialize(outboxPayload, new JsonSerializerOptions
            {
                PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower
            })
        };

        await dbContext.OutboxMessages.AddAsync(outboxMessage, cancellationToken);
        await dbContext.SaveChangesAsync(cancellationToken);

        _logger.LogInformation(
            "Updated route {RouteId} status: {PreviousStatus} -> {NewStatus}, outbox message created",
            route.Id,
            previousStatus,
            route.Status);
    }

    public override async Task StopAsync(CancellationToken cancellationToken)
    {
        _logger.LogInformation("Route Status Consumer stopping...");

        if (_channel != null)
        {
            await _channel.CloseAsync(cancellationToken);
        }

        if (_connection != null)
        {
            await _connection.CloseAsync(cancellationToken);
        }

        await base.StopAsync(cancellationToken);
    }

    public override void Dispose()
    {
        _channel?.Dispose();
        _connection?.Dispose();
        base.Dispose();
    }
}

/// <summary>
/// Message format from route-verifier RabbitMQ publisher
/// </summary>
internal class RouteStatusChangedMessage
{
    public string? RouteId { get; set; }
    public string? Link { get; set; }
    public string? OwnerId { get; set; }
    public string? WorkspaceId { get; set; }
    public string? PreviousStatus { get; set; }
    public string? NewStatus { get; set; }
    public string? BlockedReason { get; set; }
    public string? ThreatType { get; set; }
    public string? ThreatUrl { get; set; }
    public long? CheckedAt { get; set; }
    public long? NextCheckAt { get; set; }
}

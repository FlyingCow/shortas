using System.Text;
using System.Text.Json;
using Microsoft.EntityFrameworkCore;
using RabbitMQ.Client;
using RabbitMQ.Client.Events;
using ShortasProxyApi.Domain.Entities;
using ShortasProxyApi.Infrastructure.Data;

namespace ShortasProxyApi.Infrastructure.BackgroundServices;

/// <summary>
/// Background service that consumes domain verification state change events from RabbitMQ
/// </summary>
public class DomainVerificationConsumer : BackgroundService
{
    private readonly IServiceProvider _serviceProvider;
    private readonly ILogger<DomainVerificationConsumer> _logger;
    private readonly IConfiguration _configuration;
    private IConnection? _connection;
    private IChannel? _channel;
    private readonly string _exchangeName;
    private readonly string _queueName;

    public DomainVerificationConsumer(
        IServiceProvider serviceProvider,
        ILogger<DomainVerificationConsumer> logger,
        IConfiguration configuration)
    {
        _serviceProvider = serviceProvider;
        _logger = logger;
        _configuration = configuration;
        _exchangeName = configuration["RabbitMQ:DomainStateExchange"] ?? "domain.state.changed";
        _queueName = configuration["RabbitMQ:DomainStateQueue"] ?? "management-api.domain-state";
    }

    protected override async Task ExecuteAsync(CancellationToken stoppingToken)
    {
        _logger.LogInformation("Domain Verification Consumer starting...");

        while (!stoppingToken.IsCancellationRequested)
        {
            try
            {
                await ConnectAndConsumeAsync(stoppingToken);
            }
            catch (Exception ex)
            {
                _logger.LogError(ex, "Error in Domain Verification Consumer, reconnecting in 5 seconds...");
                await Task.Delay(TimeSpan.FromSeconds(5), stoppingToken);
            }
        }

        _logger.LogInformation("Domain Verification Consumer stopped");
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

        // Declare the exchange (should match domain-verifier's exchange)
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
                _logger.LogError(ex, "Error processing domain state message");
                // Nack without requeue to avoid infinite loops on poison messages
                await _channel.BasicNackAsync(ea.DeliveryTag, multiple: false, requeue: false, stoppingToken);
            }
        };

        await _channel.BasicConsumeAsync(
            queue: _queueName,
            autoAck: false,
            consumer: consumer,
            cancellationToken: stoppingToken);

        _logger.LogInformation("Domain Verification Consumer connected and consuming from {Queue}", _queueName);

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

        var stateChange = JsonSerializer.Deserialize<DomainStateChangedMessage>(message, options);
        if (stateChange == null)
        {
            _logger.LogWarning("Failed to deserialize domain state message: {Message}", message);
            return;
        }

        _logger.LogInformation(
            "Received domain state change: {DomainId} -> {Status}",
            stateChange.DomainId,
            stateChange.Status);

        using var scope = _serviceProvider.CreateScope();
        var dbContext = scope.ServiceProvider.GetRequiredService<ApplicationDbContext>();

        if (!Guid.TryParse(stateChange.DomainId, out var domainId))
        {
            _logger.LogWarning("Invalid domain ID in message: {DomainId}", stateChange.DomainId);
            return;
        }

        var domain = await dbContext.RouteDomains
            .FirstOrDefaultAsync(d => d.Id == domainId, cancellationToken);

        if (domain == null)
        {
            _logger.LogWarning("Domain not found: {DomainId}", stateChange.DomainId);
            return;
        }

        // Update verification status
        domain.VerificationStatus = ParseVerificationStatus(stateChange.Status);
        domain.VerificationReason = stateChange.VerificationReason ?? "unknown";

        if (stateChange.LastCheckAt.HasValue)
        {
            domain.LastVerificationCheck = DateTimeOffset.FromUnixTimeMilliseconds(stateChange.LastCheckAt.Value).UtcDateTime;
        }

        if (stateChange.NextCheckAt.HasValue)
        {
            domain.NextVerificationCheck = DateTimeOffset.FromUnixTimeMilliseconds(stateChange.NextCheckAt.Value).UtcDateTime;
        }

        await dbContext.SaveChangesAsync(cancellationToken);

        _logger.LogInformation(
            "Updated domain {DomainId} verification status to {Status}",
            domain.Id,
            domain.VerificationStatus);
    }

    private static DomainVerificationStatus ParseVerificationStatus(string? status)
    {
        return status?.ToLowerInvariant() switch
        {
            "verified" => DomainVerificationStatus.Verified,
            "failed" => DomainVerificationStatus.Failed,
            "pending" => DomainVerificationStatus.Pending,
            _ => DomainVerificationStatus.Pending
        };
    }

    public override async Task StopAsync(CancellationToken cancellationToken)
    {
        _logger.LogInformation("Domain Verification Consumer stopping...");

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
/// Message format from domain-verifier RabbitMQ publisher
/// </summary>
internal class DomainStateChangedMessage
{
    public string? DomainId { get; set; }
    public string? DomainName { get; set; }
    public string? OwnerId { get; set; }
    public string? Status { get; set; }
    public string? VerificationReason { get; set; }
    public long? LastCheckAt { get; set; }
    public long? NextCheckAt { get; set; }
}

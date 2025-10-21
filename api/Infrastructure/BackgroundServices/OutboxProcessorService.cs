using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;
using Microsoft.Extensions.Logging;
using System.Text.Json;
using ShortasProxyApi.Domain.Entities;
using ShortasProxyApi.Domain.Interfaces;

namespace ShortasProxyApi.Infrastructure.BackgroundServices;

/// <summary>
/// Background service that processes outbox messages and propagates them to click-router-api
/// </summary>
public class OutboxProcessorService : BackgroundService
{
    private readonly IServiceProvider _serviceProvider;
    private readonly ILogger<OutboxProcessorService> _logger;
    private readonly TimeSpan _pollingInterval = TimeSpan.FromSeconds(5);
    private readonly JsonSerializerOptions _jsonOptions;

    public OutboxProcessorService(
        IServiceProvider serviceProvider,
        ILogger<OutboxProcessorService> logger)
    {
        _serviceProvider = serviceProvider;
        _logger = logger;
        _jsonOptions = new JsonSerializerOptions
        {
            PropertyNamingPolicy = JsonNamingPolicy.CamelCase
        };
    }

    protected override async Task ExecuteAsync(CancellationToken stoppingToken)
    {
        _logger.LogInformation("Outbox Processor Service started");

        while (!stoppingToken.IsCancellationRequested)
        {
            try
            {
                await ProcessOutboxMessagesAsync(stoppingToken);
            }
            catch (Exception ex)
            {
                _logger.LogError(ex, "Error processing outbox messages");
            }

            await Task.Delay(_pollingInterval, stoppingToken);
        }

        _logger.LogInformation("Outbox Processor Service stopped");
    }

    private async Task ProcessOutboxMessagesAsync(CancellationToken cancellationToken)
    {
        using var scope = _serviceProvider.CreateScope();
        var outboxRepository = scope.ServiceProvider.GetRequiredService<IOutboxRepository>();
        var httpClientFactory = scope.ServiceProvider.GetRequiredService<IHttpClientFactory>();

        // Get pending messages
        var messages = await outboxRepository.GetPendingMessagesAsync(batchSize: 10);

        if (!messages.Any())
            return;

        _logger.LogInformation("Processing {Count} outbox messages", messages.Count);

        foreach (var message in messages)
        {
            if (cancellationToken.IsCancellationRequested)
                break;

            await ProcessMessageAsync(message, outboxRepository, httpClientFactory, cancellationToken);
        }
    }

    private async Task ProcessMessageAsync(
        OutboxMessage message,
        IOutboxRepository outboxRepository,
        IHttpClientFactory httpClientFactory,
        CancellationToken cancellationToken)
    {
        try
        {
            // Mark as processing
            message.Status = OutboxMessageStatus.Processing;
            await outboxRepository.UpdateAsync(message);
            await outboxRepository.SaveChangesAsync();

            // Create HTTP client
            var httpClient = httpClientFactory.CreateClient("ClickRouterApi");

            // Process based on event type
            var success = await SendToClickRouterApiAsync(message, httpClient, cancellationToken);

            if (success)
            {
                // Mark as completed
                message.Status = OutboxMessageStatus.Completed;
                message.ProcessedAt = DateTime.UtcNow;
                message.ErrorMessage = null;

                _logger.LogInformation(
                    "Successfully processed outbox message {MessageId}, Event: {EventType}",
                    message.Id,
                    message.EventType);
            }
            else
            {
                // Handle failure with retry logic
                await HandleFailureAsync(message);
            }

            await outboxRepository.UpdateAsync(message);
            await outboxRepository.SaveChangesAsync();
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error processing outbox message {MessageId}", message.Id);

            message.ErrorMessage = ex.Message;
            await HandleFailureAsync(message);
            await outboxRepository.UpdateAsync(message);
            await outboxRepository.SaveChangesAsync();
        }
    }

    private async Task<bool> SendToClickRouterApiAsync(
        OutboxMessage message,
        HttpClient httpClient,
        CancellationToken cancellationToken)
    {
        try
        {
            HttpResponseMessage? response = null;

            switch (message.EventType)
            {
                case OutboxEventType.RouteCreated:
                    var content = new StringContent(message.Payload, System.Text.Encoding.UTF8, "application/json");
                    response = await httpClient.PostAsync("/v1/routes", content, cancellationToken);
                    break;

                case OutboxEventType.RouteUpdated:
                    // Extract route from payload to get domain/path
                    var route = JsonSerializer.Deserialize<Domain.Entities.Route>(message.Payload, _jsonOptions);
                    if (route != null)
                    {
                        var updateContent = new StringContent(message.Payload, System.Text.Encoding.UTF8, "application/json");
                        // We need to extract domain and path from the link
                        var parts = route.Link.Split('/');
                        if (parts.Length >= 2)
                        {
                            var domain = parts[0];
                            var path = string.Join("/", parts.Skip(1));
                            response = await httpClient.PutAsync($"/v1/routes/{domain}/{path}", updateContent, cancellationToken);
                        }
                    }
                    break;

                case OutboxEventType.RouteDeleted:
                    // Extract domain and path from payload
                    var deleteData = JsonSerializer.Deserialize<dynamic>(message.Payload, _jsonOptions);
                    if (deleteData != null)
                    {
                        var domain = deleteData.GetProperty("domain").GetString();
                        var path = deleteData.GetProperty("path").GetString();
                        response = await httpClient.DeleteAsync($"/v1/routes/{domain}/{path}", cancellationToken);
                    }
                    break;

                case OutboxEventType.RouteBulkCreated:
                    var bulkCreateContent = new StringContent(message.Payload, System.Text.Encoding.UTF8, "application/json");
                    response = await httpClient.PostAsync("/v1/routes/bulk", bulkCreateContent, cancellationToken);
                    break;

                case OutboxEventType.RouteBulkUpdated:
                    var bulkUpdateContent = new StringContent(message.Payload, System.Text.Encoding.UTF8, "application/json");
                    response = await httpClient.PutAsync("/v1/routes/bulk", bulkUpdateContent, cancellationToken);
                    break;

                case OutboxEventType.RouteBulkDeleted:
                    var bulkDeleteData = JsonSerializer.Deserialize<dynamic>(message.Payload, _jsonOptions);
                    if (bulkDeleteData != null)
                    {
                        var bulkDeleteContent = new StringContent(message.Payload, System.Text.Encoding.UTF8, "application/json");
                        var request = new HttpRequestMessage(HttpMethod.Delete, "/v1/routes/bulk")
                        {
                            Content = bulkDeleteContent
                        };
                        response = await httpClient.SendAsync(request, cancellationToken);
                    }
                    break;

                case OutboxEventType.CertificateCreated:
                    var certCreateContent = new StringContent(message.Payload, System.Text.Encoding.UTF8, "application/json");
                    var certificate = JsonSerializer.Deserialize<Domain.Entities.Certificate>(message.Payload, _jsonOptions);
                    if (certificate != null)
                    {
                        response = await httpClient.PostAsync($"/v1/certificates/{certificate.Key}", certCreateContent, cancellationToken);
                    }
                    break;

                case OutboxEventType.CertificateUpdated:
                    var certUpdateContent = new StringContent(message.Payload, System.Text.Encoding.UTF8, "application/json");
                    var updatedCertificate = JsonSerializer.Deserialize<Domain.Entities.Certificate>(message.Payload, _jsonOptions);
                    if (updatedCertificate != null)
                    {
                        response = await httpClient.PutAsync($"/v1/certificates/{updatedCertificate.Key}", certUpdateContent, cancellationToken);
                    }
                    break;

                case OutboxEventType.CertificateDeleted:
                    var certDeleteData = JsonSerializer.Deserialize<dynamic>(message.Payload, _jsonOptions);
                    if (certDeleteData != null)
                    {
                        var domain = certDeleteData.GetProperty("domain").GetString();
                        response = await httpClient.DeleteAsync($"/v1/certificates/{domain}", cancellationToken);
                    }
                    break;

                case OutboxEventType.UserSettingsCreated:
                    var settingsCreateContent = new StringContent(message.Payload, System.Text.Encoding.UTF8, "application/json");
                    var userSettings = JsonSerializer.Deserialize<Domain.Entities.UserSettings>(message.Payload, _jsonOptions);
                    if (userSettings != null)
                    {
                        response = await httpClient.PostAsync($"/v1/user-settings/{userSettings.Email}", settingsCreateContent, cancellationToken);
                    }
                    break;

                case OutboxEventType.UserSettingsUpdated:
                    var settingsUpdateContent = new StringContent(message.Payload, System.Text.Encoding.UTF8, "application/json");
                    var updatedSettings = JsonSerializer.Deserialize<Domain.Entities.UserSettings>(message.Payload, _jsonOptions);
                    if (updatedSettings != null)
                    {
                        response = await httpClient.PutAsync($"/v1/user-settings/{updatedSettings.Email}", settingsUpdateContent, cancellationToken);
                    }
                    break;

                case OutboxEventType.UserSettingsDeleted:
                    var settingsDeleteData = JsonSerializer.Deserialize<dynamic>(message.Payload, _jsonOptions);
                    if (settingsDeleteData != null)
                    {
                        var userId = settingsDeleteData.GetProperty("userId").GetString();
                        response = await httpClient.DeleteAsync($"/v1/user-settings/{userId}", cancellationToken);
                    }
                    break;

                default:
                    _logger.LogWarning("Unknown event type: {EventType}", message.EventType);
                    return false;
            }

            if (response != null && response.IsSuccessStatusCode)
            {
                return true;
            }

            if (response != null)
            {
                var errorContent = await response.Content.ReadAsStringAsync(cancellationToken);
                message.ErrorMessage = $"HTTP {response.StatusCode}: {errorContent}";
                _logger.LogWarning(
                    "Failed to send outbox message {MessageId} to click-router-api. Status: {StatusCode}, Error: {Error}",
                    message.Id,
                    response.StatusCode,
                    errorContent);
            }

            return false;
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error sending outbox message {MessageId} to click-router-api", message.Id);
            message.ErrorMessage = ex.Message;
            return false;
        }
    }

    private async Task HandleFailureAsync(OutboxMessage message)
    {
        message.RetryCount++;

        if (message.RetryCount >= message.MaxRetries)
        {
            message.Status = OutboxMessageStatus.Failed;
            _logger.LogError(
                "Outbox message {MessageId} failed after {RetryCount} attempts. Event: {EventType}",
                message.Id,
                message.RetryCount,
                message.EventType);
        }
        else
        {
            message.Status = OutboxMessageStatus.Pending;
            // Exponential backoff: 1min, 2min, 4min, 8min, 16min
            var delayMinutes = Math.Pow(2, message.RetryCount);
            message.NextRetryAt = DateTime.UtcNow.AddMinutes(delayMinutes);

            _logger.LogWarning(
                "Outbox message {MessageId} will be retried at {NextRetryAt}. Retry {RetryCount}/{MaxRetries}",
                message.Id,
                message.NextRetryAt,
                message.RetryCount,
                message.MaxRetries);
        }

        await Task.CompletedTask;
    }
}

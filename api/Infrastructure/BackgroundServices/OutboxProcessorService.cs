using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;
using Microsoft.Extensions.Logging;
using System.Text.Json;
using System.Text.Json.Serialization;
using ShortasProxyApi.Domain.Entities;
using ShortasProxyApi.Domain.Interfaces;
using RouteSearchDoc = ShortasProxyApi.Domain.Interfaces.RouteSearchDocument;

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
        var routeSearchService = scope.ServiceProvider.GetRequiredService<IRouteSearchService>();

        // Get pending messages
        var messages = await outboxRepository.GetPendingMessagesAsync(batchSize: 10);

        if (!messages.Any())
            return;

        _logger.LogInformation("Processing {Count} outbox messages", messages.Count);

        foreach (var message in messages)
        {
            if (cancellationToken.IsCancellationRequested)
                break;

            if (IsSearchIndexEvent(message.EventType))
            {
                await ProcessSearchIndexMessageAsync(message, outboxRepository, routeSearchService, cancellationToken);
            }
            else if (IsDomainVerificationEvent(message.EventType))
            {
                await ProcessDomainVerificationMessageAsync(message, outboxRepository, httpClientFactory, cancellationToken);
            }
            else if (IsRouteVerificationEvent(message.EventType))
            {
                await ProcessRouteVerificationMessageAsync(message, outboxRepository, httpClientFactory, cancellationToken);
            }
            else if (IsRouteStatusEvent(message.EventType))
            {
                await ProcessRouteStatusMessageAsync(message, outboxRepository, httpClientFactory, cancellationToken);
            }
            else
            {
                await ProcessMessageAsync(message, outboxRepository, httpClientFactory, cancellationToken);
            }
        }
    }

    private static bool IsSearchIndexEvent(string eventType) =>
        eventType is OutboxEventType.RouteSearchIndex
                  or OutboxEventType.RouteSearchDelete
                  or OutboxEventType.RouteSearchBulkIndex
                  or OutboxEventType.RouteSearchBulkDelete;

    private static bool IsDomainVerificationEvent(string eventType) =>
        eventType is OutboxEventType.DomainVerificationRequested
                  or OutboxEventType.DomainRemovalRequested;

    private static bool IsRouteVerificationEvent(string eventType) =>
        eventType is OutboxEventType.RouteVerificationRequested
                  or OutboxEventType.RouteVerificationRemovalRequested;

    private static bool IsRouteStatusEvent(string eventType) =>
        eventType is OutboxEventType.RouteStatusUpdated;

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

    private async Task ProcessSearchIndexMessageAsync(
        OutboxMessage message,
        IOutboxRepository outboxRepository,
        IRouteSearchService routeSearchService,
        CancellationToken cancellationToken)
    {
        try
        {
            message.Status = OutboxMessageStatus.Processing;
            await outboxRepository.UpdateAsync(message);
            await outboxRepository.SaveChangesAsync();

            Domain.Common.Result? result = null;

            switch (message.EventType)
            {
                case OutboxEventType.RouteSearchIndex:
                    var doc = JsonSerializer.Deserialize<RouteSearchDoc>(message.Payload, _jsonOptions);
                    if (doc != null)
                        result = await routeSearchService.IndexRouteAsync(doc);
                    break;

                case OutboxEventType.RouteSearchDelete:
                    var deletePayload = JsonSerializer.Deserialize<JsonElement>(message.Payload, _jsonOptions);
                    var routeId = deletePayload.GetProperty("id").GetString();
                    if (!string.IsNullOrEmpty(routeId))
                        result = await routeSearchService.DeleteRouteAsync(routeId);
                    break;

                case OutboxEventType.RouteSearchBulkIndex:
                    var docs = JsonSerializer.Deserialize<List<RouteSearchDoc>>(message.Payload, _jsonOptions);
                    if (docs != null && docs.Count > 0)
                        result = await routeSearchService.IndexRoutesAsync(docs);
                    break;

                case OutboxEventType.RouteSearchBulkDelete:
                    var ids = JsonSerializer.Deserialize<List<string>>(message.Payload, _jsonOptions);
                    if (ids != null && ids.Count > 0)
                        result = await routeSearchService.DeleteRoutesAsync(ids);
                    break;
            }

            if (result != null && result.IsSuccess)
            {
                message.Status = OutboxMessageStatus.Completed;
                message.ProcessedAt = DateTime.UtcNow;
                message.ErrorMessage = null;
                _logger.LogInformation("Processed search index message {MessageId}, Event: {EventType}",
                    message.Id, message.EventType);
            }
            else
            {
                message.ErrorMessage = result?.Error ?? "Unknown search index error";
                await HandleFailureAsync(message);
            }

            await outboxRepository.UpdateAsync(message);
            await outboxRepository.SaveChangesAsync();
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error processing search index message {MessageId}", message.Id);
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
                    var userSettings = JsonSerializer.Deserialize<Domain.Entities.UserSettings>(message.Payload, _jsonOptions);
                    if (userSettings != null)
                    {
                        // Convert to Rust API format with snake_case JSON properties
                        var rustDto = RustApiUserSettingsDto.FromUserSettings(userSettings);
                        var rustJson = JsonSerializer.Serialize(rustDto, _jsonOptions);
                        var settingsCreateContent = new StringContent(rustJson, System.Text.Encoding.UTF8, "application/json");
                        response = await httpClient.PostAsync($"/v1/user-settings/{userSettings.Email}", settingsCreateContent, cancellationToken);
                    }
                    break;

                case OutboxEventType.UserSettingsUpdated:
                    var updatedSettings = JsonSerializer.Deserialize<Domain.Entities.UserSettings>(message.Payload, _jsonOptions);
                    if (updatedSettings != null)
                    {
                        // Convert to Rust API format with snake_case JSON properties
                        var rustDtoUpdate = RustApiUserSettingsDto.FromUserSettings(updatedSettings);
                        var rustJsonUpdate = JsonSerializer.Serialize(rustDtoUpdate, _jsonOptions);
                        var settingsUpdateContent = new StringContent(rustJsonUpdate, System.Text.Encoding.UTF8, "application/json");
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

    private async Task ProcessDomainVerificationMessageAsync(
        OutboxMessage message,
        IOutboxRepository outboxRepository,
        IHttpClientFactory httpClientFactory,
        CancellationToken cancellationToken)
    {
        try
        {
            message.Status = OutboxMessageStatus.Processing;
            await outboxRepository.UpdateAsync(message);
            await outboxRepository.SaveChangesAsync();

            var httpClient = httpClientFactory.CreateClient("DomainVerifier");

            var success = await SendToDomainVerifierAsync(message, httpClient, cancellationToken);

            if (success)
            {
                message.Status = OutboxMessageStatus.Completed;
                message.ProcessedAt = DateTime.UtcNow;
                message.ErrorMessage = null;

                _logger.LogInformation(
                    "Successfully processed domain verification message {MessageId}, Event: {EventType}",
                    message.Id,
                    message.EventType);
            }
            else
            {
                await HandleFailureAsync(message);
            }

            await outboxRepository.UpdateAsync(message);
            await outboxRepository.SaveChangesAsync();
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error processing domain verification message {MessageId}", message.Id);

            message.ErrorMessage = ex.Message;
            await HandleFailureAsync(message);
            await outboxRepository.UpdateAsync(message);
            await outboxRepository.SaveChangesAsync();
        }
    }

    private async Task<bool> SendToDomainVerifierAsync(
        OutboxMessage message,
        HttpClient httpClient,
        CancellationToken cancellationToken)
    {
        try
        {
            HttpResponseMessage? response = null;

            switch (message.EventType)
            {
                case OutboxEventType.DomainVerificationRequested:
                    var content = new StringContent(message.Payload, System.Text.Encoding.UTF8, "application/json");
                    response = await httpClient.PostAsync("/v1/domains", content, cancellationToken);
                    break;

                case OutboxEventType.DomainRemovalRequested:
                    var removePayload = JsonSerializer.Deserialize<JsonElement>(message.Payload, _jsonOptions);
                    var domainId = removePayload.GetProperty("id").GetString();
                    if (!string.IsNullOrEmpty(domainId))
                    {
                        response = await httpClient.DeleteAsync($"/v1/domains/{domainId}", cancellationToken);
                    }
                    break;

                default:
                    _logger.LogWarning("Unknown domain verification event type: {EventType}", message.EventType);
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
                    "Failed to send domain verification message {MessageId} to domain-verifier. Status: {StatusCode}, Error: {Error}",
                    message.Id,
                    response.StatusCode,
                    errorContent);
            }

            return false;
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error sending domain verification message {MessageId} to domain-verifier", message.Id);
            message.ErrorMessage = ex.Message;
            return false;
        }
    }

    private async Task ProcessRouteVerificationMessageAsync(
        OutboxMessage message,
        IOutboxRepository outboxRepository,
        IHttpClientFactory httpClientFactory,
        CancellationToken cancellationToken)
    {
        try
        {
            message.Status = OutboxMessageStatus.Processing;
            await outboxRepository.UpdateAsync(message);
            await outboxRepository.SaveChangesAsync();

            var httpClient = httpClientFactory.CreateClient("RouteVerifier");

            var success = await SendToRouteVerifierAsync(message, httpClient, cancellationToken);

            if (success)
            {
                message.Status = OutboxMessageStatus.Completed;
                message.ProcessedAt = DateTime.UtcNow;
                message.ErrorMessage = null;

                _logger.LogInformation(
                    "Successfully processed route verification message {MessageId}, Event: {EventType}",
                    message.Id,
                    message.EventType);
            }
            else
            {
                await HandleFailureAsync(message);
            }

            await outboxRepository.UpdateAsync(message);
            await outboxRepository.SaveChangesAsync();
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error processing route verification message {MessageId}", message.Id);

            message.ErrorMessage = ex.Message;
            await HandleFailureAsync(message);
            await outboxRepository.UpdateAsync(message);
            await outboxRepository.SaveChangesAsync();
        }
    }

    private async Task<bool> SendToRouteVerifierAsync(
        OutboxMessage message,
        HttpClient httpClient,
        CancellationToken cancellationToken)
    {
        try
        {
            HttpResponseMessage? response = null;

            switch (message.EventType)
            {
                case OutboxEventType.RouteVerificationRequested:
                    var content = new StringContent(message.Payload, System.Text.Encoding.UTF8, "application/json");
                    response = await httpClient.PostAsync("/v1/routes", content, cancellationToken);
                    break;

                case OutboxEventType.RouteVerificationRemovalRequested:
                    var removePayload = JsonSerializer.Deserialize<JsonElement>(message.Payload, _jsonOptions);
                    var routeId = removePayload.GetProperty("id").GetString();
                    if (!string.IsNullOrEmpty(routeId))
                    {
                        response = await httpClient.DeleteAsync($"/v1/routes/{routeId}", cancellationToken);
                    }
                    break;

                default:
                    _logger.LogWarning("Unknown route verification event type: {EventType}", message.EventType);
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
                    "Failed to send route verification message {MessageId} to route-verifier. Status: {StatusCode}, Error: {Error}",
                    message.Id,
                    response.StatusCode,
                    errorContent);
            }

            return false;
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error sending route verification message {MessageId} to route-verifier", message.Id);
            message.ErrorMessage = ex.Message;
            return false;
        }
    }

    private async Task ProcessRouteStatusMessageAsync(
        OutboxMessage message,
        IOutboxRepository outboxRepository,
        IHttpClientFactory httpClientFactory,
        CancellationToken cancellationToken)
    {
        try
        {
            message.Status = OutboxMessageStatus.Processing;
            await outboxRepository.UpdateAsync(message);
            await outboxRepository.SaveChangesAsync();

            var httpClient = httpClientFactory.CreateClient("ClickRouterApi");

            var success = await SendRouteStatusToClickRouterAsync(message, httpClient, cancellationToken);

            if (success)
            {
                message.Status = OutboxMessageStatus.Completed;
                message.ProcessedAt = DateTime.UtcNow;
                _logger.LogInformation(
                    "Route status update message {MessageId} processed successfully",
                    message.Id);
            }
            else
            {
                await HandleFailureAsync(message);
            }

            await outboxRepository.UpdateAsync(message);
            await outboxRepository.SaveChangesAsync();
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error processing route status message {MessageId}", message.Id);
            message.ErrorMessage = ex.Message;
            await HandleFailureAsync(message);
            await outboxRepository.UpdateAsync(message);
            await outboxRepository.SaveChangesAsync();
        }
    }

    private async Task<bool> SendRouteStatusToClickRouterAsync(
        OutboxMessage message,
        HttpClient httpClient,
        CancellationToken cancellationToken)
    {
        try
        {
            var payload = JsonSerializer.Deserialize<JsonElement>(message.Payload, _jsonOptions);
            var routeId = payload.GetProperty("route_id").GetString();
            var status = payload.GetProperty("status").GetString();
            var blockedReason = payload.TryGetProperty("blocked_reason", out var reasonProp)
                ? reasonProp.GetString()
                : null;

            if (string.IsNullOrEmpty(routeId))
            {
                _logger.LogWarning("Route status message {MessageId} has no route_id", message.Id);
                return false;
            }

            // Build the PATCH payload for click-router-api
            // The PATCH endpoint expects: { "status": { "type": "Active" } }
            // or { "status": { "type": "Blocked", "reason": "..." } }
            object statusPayload;
            bool isBlocked = status?.StartsWith("Blocked") == true;

            if (isBlocked)
            {
                // Extract reason from status string "Blocked: reason" or use blockedReason
                var reason = blockedReason;
                if (string.IsNullOrEmpty(reason) && status != null && status.Contains(":"))
                {
                    reason = status.Substring(status.IndexOf(':') + 1).Trim();
                }
                reason ??= "Unknown";

                statusPayload = new
                {
                    status = new
                    {
                        type = "Blocked",
                        reason = reason
                    }
                };
            }
            else
            {
                statusPayload = new
                {
                    status = new
                    {
                        type = "Active"
                    }
                };
            }

            var content = new StringContent(
                JsonSerializer.Serialize(statusPayload),
                System.Text.Encoding.UTF8,
                "application/json");

            // Use PATCH endpoint for status-only updates
            var request = new HttpRequestMessage(HttpMethod.Patch, $"/v1/routes/{routeId}")
            {
                Content = content
            };
            var response = await httpClient.SendAsync(request, cancellationToken);

            if (response.IsSuccessStatusCode)
            {
                _logger.LogInformation(
                    "Successfully updated route {RouteId} status to {Status} in click-router",
                    routeId,
                    status);
                return true;
            }

            var errorContent = await response.Content.ReadAsStringAsync(cancellationToken);
            _logger.LogWarning(
                "Failed to update route {RouteId} status in click-router. Status: {StatusCode}, Error: {Error}",
                routeId,
                response.StatusCode,
                errorContent);

            // Don't retry on 404 - route might not exist in click-router yet
            if (response.StatusCode == System.Net.HttpStatusCode.NotFound)
            {
                _logger.LogWarning(
                    "Route {RouteId} not found in click-router, marking as completed",
                    routeId);
                return true;
            }

            message.ErrorMessage = $"HTTP {response.StatusCode}: {errorContent}";
            return false;
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error sending route status update to click-router");
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

/// <summary>
/// DTO for sending user settings to Rust API with snake_case JSON properties
/// </summary>
internal class RustApiUserSettingsDto
{
    [JsonPropertyName("email")]
    public string Email { get; set; } = string.Empty;

    [JsonPropertyName("status")]
    public string Status { get; set; } = string.Empty;

    [JsonPropertyName("debug")]
    public bool Debug { get; set; }

    [JsonPropertyName("overflow")]
    public bool Overflow { get; set; }

    [JsonPropertyName("skip_tracking")]
    public List<string> SkipTracking { get; set; } = new();

    [JsonPropertyName("allowed_request_params")]
    public List<string> AllowedRequestParams { get; set; } = new();

    [JsonPropertyName("allowed_destination_params")]
    public List<string> AllowedDestinationParams { get; set; } = new();

    public static RustApiUserSettingsDto FromUserSettings(UserSettings settings)
    {
        return new RustApiUserSettingsDto
        {
            Email = settings.Email,
            Status = settings.Status,
            Debug = settings.Debug,
            Overflow = settings.Overflow,
            SkipTracking = settings.SkipTracking,
            AllowedRequestParams = settings.AllowedRequestParams,
            AllowedDestinationParams = settings.AllowedDestinationParams
        };
    }
}

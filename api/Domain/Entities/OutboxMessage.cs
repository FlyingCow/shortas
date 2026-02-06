namespace ShortasProxyApi.Domain.Entities;

/// <summary>
/// Outbox message for eventual consistency with click-router-api
/// </summary>
public class OutboxMessage
{
    public Guid Id { get; set; } = Guid.NewGuid();
    public string EventType { get; set; } = string.Empty;
    public string AggregateId { get; set; } = string.Empty;
    public string Payload { get; set; } = "{}";
    public DateTime CreatedAt { get; set; } = DateTime.UtcNow;
    public DateTime? ProcessedAt { get; set; }
    public string Status { get; set; } = OutboxMessageStatus.Pending;
    public int RetryCount { get; set; } = 0;
    public int MaxRetries { get; set; } = 5;
    public string? ErrorMessage { get; set; }
    public DateTime? NextRetryAt { get; set; }
}

public static class OutboxMessageStatus
{
    public const string Pending = "Pending";
    public const string Processing = "Processing";
    public const string Completed = "Completed";
    public const string Failed = "Failed";
}

public static class OutboxEventType
{
    // Route events
    public const string RouteCreated = "RouteCreated";
    public const string RouteUpdated = "RouteUpdated";
    public const string RouteDeleted = "RouteDeleted";
    public const string RouteBulkCreated = "RouteBulkCreated";
    public const string RouteBulkUpdated = "RouteBulkUpdated";
    public const string RouteBulkDeleted = "RouteBulkDeleted";

    // Certificate events
    public const string CertificateCreated = "CertificateCreated";
    public const string CertificateUpdated = "CertificateUpdated";
    public const string CertificateDeleted = "CertificateDeleted";

    // UserSettings events
    public const string UserSettingsCreated = "UserSettingsCreated";
    public const string UserSettingsUpdated = "UserSettingsUpdated";
    public const string UserSettingsDeleted = "UserSettingsDeleted";

    // Route search index events (Elasticsearch)
    public const string RouteSearchIndex = "RouteSearchIndex";
    public const string RouteSearchDelete = "RouteSearchDelete";
    public const string RouteSearchBulkIndex = "RouteSearchBulkIndex";
    public const string RouteSearchBulkDelete = "RouteSearchBulkDelete";
}

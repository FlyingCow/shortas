namespace ShortasProxyApi.Application.DTOs;

/// <summary>
/// DTO for click stream data matching the non-nullable ClickHouse schema.
/// Uses default values instead of nulls: '_unknown' for strings, epoch for DateTime, 0 for numbers.
/// </summary>
public class ClickStreamDto
{
    /// <summary>
    /// Constant representing unknown/missing data
    /// </summary>
    public const string Unknown = "_unknown";

    /// <summary>
    /// Epoch timestamp representing unknown/missing session data
    /// </summary>
    public static readonly DateTime EpochDateTime = new DateTime(1970, 1, 1, 0, 0, 0, DateTimeKind.Utc);

    public string Id { get; set; } = string.Empty;
    public string OwnerId { get; set; } = string.Empty;
    public string CreatorId { get; set; } = string.Empty;
    public string RouteId { get; set; } = string.Empty;

    /// <summary>Route name (enriched from local data)</summary>
    public string? RouteName { get; set; }

    /// <summary>Route domain name (enriched from local data)</summary>
    public string? RouteDomainName { get; set; }

    public string WorkspaceId { get; set; } = string.Empty;
    public DateTime Created { get; set; }
    public string Dest { get; set; } = string.Empty;
    public string Ip { get; set; } = string.Empty;

    /// <summary>Geographic continent (defaults to "_unknown")</summary>
    public string Continent { get; set; } = Unknown;

    /// <summary>Geographic country (defaults to "_unknown")</summary>
    public string Country { get; set; } = Unknown;

    /// <summary>Geographic location (defaults to "_unknown")</summary>
    public string Location { get; set; } = Unknown;

    /// <summary>Operating system family (defaults to "_unknown")</summary>
    public string OsFamily { get; set; } = Unknown;

    /// <summary>Operating system version (defaults to "_unknown")</summary>
    public string OsVersion { get; set; } = Unknown;

    /// <summary>User agent family (defaults to "_unknown")</summary>
    public string UserAgentFamily { get; set; } = Unknown;

    /// <summary>User agent version (defaults to "_unknown")</summary>
    public string UserAgentVersion { get; set; } = Unknown;

    /// <summary>Device brand (defaults to "_unknown")</summary>
    public string DeviceBrand { get; set; } = Unknown;

    /// <summary>Device family (defaults to "_unknown")</summary>
    public string DeviceFamily { get; set; } = Unknown;

    /// <summary>Device model (defaults to "_unknown")</summary>
    public string DeviceModel { get; set; } = Unknown;

    /// <summary>First session timestamp (defaults to epoch: 1970-01-01)</summary>
    public DateTime SessionFirst { get; set; } = EpochDateTime;

    /// <summary>Number of clicks in session (defaults to 0)</summary>
    public long SessionClicks { get; set; } = 0;

    public bool IsUnique { get; set; }
    public bool IsBot { get; set; }

    /// <summary>
    /// Check if a string field contains unknown/missing data
    /// </summary>
    public static bool IsUnknown(string? value) => string.IsNullOrEmpty(value) || value == Unknown;

    /// <summary>
    /// Check if this item has valid session data
    /// </summary>
    public bool HasSession() => SessionFirst != EpochDateTime;

    /// <summary>
    /// Check if this item has valid geographic data
    /// </summary>
    public bool HasGeoData() => !IsUnknown(Country);

    /// <summary>
    /// Check if this item has valid device data
    /// </summary>
    public bool HasDeviceData() => !IsUnknown(DeviceFamily);

    /// <summary>
    /// Check if this item has valid user agent data
    /// </summary>
    public bool HasUserAgentData() => !IsUnknown(UserAgentFamily);
}


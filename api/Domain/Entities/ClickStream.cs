namespace ShortasProxyApi.Domain.Entities;

public class ClickStream
{
    public Guid Id { get; set; } = Guid.NewGuid();
    public string ExternalId { get; set; } = string.Empty;
    public string OwnerId { get; set; } = string.Empty;
    public string CreatorId { get; set; } = string.Empty;
    public string RouteId { get; set; } = string.Empty;
    public string WorkspaceId { get; set; } = string.Empty;
    public DateTime Created { get; set; }
    public string Dest { get; set; } = string.Empty;
    public string Ip { get; set; } = string.Empty;
    public string? Continent { get; set; }
    public string? Country { get; set; }
    public string? Location { get; set; }
    public string? OsFamily { get; set; }
    public string? OsVersion { get; set; }
    public string? UserAgentFamily { get; set; }
    public string? UserAgentVersion { get; set; }
    public string? DeviceBrand { get; set; }
    public string? DeviceFamily { get; set; }
    public string? DeviceModel { get; set; }
    public DateTime? SessionFirst { get; set; }
    public long? SessionClicks { get; set; }
    public bool IsUnique { get; set; }
    public bool IsBot { get; set; }
}

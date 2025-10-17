using ShortasProxyApi.Domain.Entities;
using RouteEntity = ShortasProxyApi.Domain.Entities.Route;

namespace ShortasProxyApi.Infrastructure.HttpClients;

/// <summary>
/// DTO for sending Route data to Click Router API
/// Excludes navigation properties and internal database fields
/// </summary>
public class ClickRouterRouteDto
{
    public string Switch { get; set; } = string.Empty;
    public string Link { get; set; } = string.Empty;
    public string? Dest { get; set; }
    public string DestFormat { get; set; } = "Http";
    public int? Code { get; set; }
    public long? Ttl { get; set; }
    public string Status { get; set; } = "Active";
    public string Terminal { get; set; } = "External";
    public RoutingPolicy? Policy { get; set; }
    public ClickRouterRoutePropertiesDto? Properties { get; set; }

    /// <summary>
    /// Maps a Route entity to ClickRouterRouteDto for API communication
    /// </summary>
    public static ClickRouterRouteDto FromEntity(RouteEntity route)
    {
        return new ClickRouterRouteDto
        {
            Switch = route.Switch,
            Link = route.Link,
            Dest = route.Dest,
            DestFormat = route.DestFormat,
            Code = route.Code,
            Ttl = route.Ttl,
            Status = route.Status,
            Terminal = route.Terminal,
            Policy = route.Policy,
            Properties = route.Properties != null
                ? ClickRouterRoutePropertiesDto.FromEntity(route.Properties)
                : null
        };
    }
}

/// <summary>
/// DTO for Route Properties to send to Click Router API
/// </summary>
public class ClickRouterRoutePropertiesDto
{
    public string? RouteId { get; set; }
    public string? DomainId { get; set; }
    public string? OwnerId { get; set; }
    public string? CreatorId { get; set; }
    public string? WorkspaceId { get; set; }
    public List<string>? Scripts { get; set; }
    public List<string>? Tags { get; set; }
    public Dictionary<string, object>? Custom { get; set; }
    public Dictionary<string, object>? Native { get; set; }
    public Dictionary<string, object>? Bundling { get; set; }
    public bool Opengraph { get; set; }
    public bool AllowDebug { get; set; }

    /// <summary>
    /// Maps RouteProperties entity to ClickRouterRoutePropertiesDto
    /// </summary>
    public static ClickRouterRoutePropertiesDto FromEntity(RouteProperties properties)
    {
        return new ClickRouterRoutePropertiesDto
        {
            RouteId = properties.RouteId,
            DomainId = properties.DomainId,
            OwnerId = properties.OwnerId,
            CreatorId = properties.CreatorId,
            WorkspaceId = properties.WorkspaceId,
            Scripts = properties.Scripts,
            Tags = properties.Tags,
            Custom = properties.Custom,
            Native = properties.Native,
            Bundling = properties.Bundling,
            Opengraph = properties.Opengraph,
            AllowDebug = properties.AllowDebug
        };
    }
}

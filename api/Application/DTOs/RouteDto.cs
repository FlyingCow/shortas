namespace ShortasProxyApi.Application.DTOs;

using ShortasProxyApi.Domain.Entities;

/// <summary>
/// DTO for conditional route destination - pairs a condition with a destination URL
/// </summary>
public class ConditionDestinationDto
{
    public string Dest { get; set; } = string.Empty;
    public Expression Condition { get; set; } = new();
}

public class RouteDto
{
    public string? Id { get; set; }  // Internal ID for route operations
    public string Switch { get; set; } = string.Empty;
    public string Link { get; set; } = string.Empty;
    public string? Dest { get; set; }  // Nullable - matches click-router Option<String>
    public string DestFormat { get; set; } = "Http";  // Default value
    public int? Code { get; set; }  // Nullable - matches click-router Option<u16>
    public long? Ttl { get; set; }  // Nullable long - matches click-router Option<u128>
    public string Status { get; set; } = "Active";  // Default value
    public string Terminal { get; set; } = "External";  // Default value
    public RoutingPolicy? Policy { get; set; }  // Routing policy (Basic, Conditional, etc.)
    public RoutePropertiesDto? Properties { get; set; }

    // Domain relationship
    public Guid? DomainId { get; set; }
    public DomainDto? Domain { get; set; }

    // Conditional routes - used for master/child pattern
    // When conditions are provided, the API creates child routes for each condition
    public List<ConditionDestinationDto>? Conditions { get; set; }
}

public class RoutePropertiesDto
{
    public string? RouteId { get; set; }  // Nullable - matches click-router Option<String>
    public string? DomainId { get; set; }  // Nullable
    public string? OwnerId { get; set; }  // Nullable
    public string? CreatorId { get; set; }  // NEW - matches click-router
    public string? WorkspaceId { get; set; }  // NEW - matches click-router
    public List<string>? Scripts { get; set; }  // Nullable - empty list becomes null
    public List<string>? Tags { get; set; }  // Nullable - empty list becomes null
    public Dictionary<string, object>? Custom { get; set; }  // Nullable
    public Dictionary<string, object>? Native { get; set; }  // NEW - matches click-router
    public Dictionary<string, object>? Bundling { get; set; }  // NEW - matches click-router
    public bool Opengraph { get; set; }
    public bool AllowDebug { get; set; }
}


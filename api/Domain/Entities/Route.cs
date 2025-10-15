using System.Text.Json;

namespace ShortasProxyApi.Domain.Entities;

public class Route
{
    public Guid Id { get; set; } = Guid.NewGuid();
    public string Switch { get; set; } = string.Empty;
    public string Link { get; set; } = string.Empty;
    public string? Dest { get; set; }  // Nullable - matches click-router Option<String>
    public string DestFormat { get; set; } = "Http";  // Default value
    public int? Code { get; set; }  // Nullable - matches click-router Option<u16>
    public long? Ttl { get; set; }  // Nullable long - matches click-router Option<u128>
    public string Status { get; set; } = "Active";  // Default value
    public string Terminal { get; set; } = "External";  // Default value
    public string PolicyJson { get; set; } = "\"Basic\"";  // JSON string for routing policy
    public RouteProperties Properties { get; set; } = new();  // Required, not nullable

    // Computed property for Policy deserialization
    public RoutingPolicy Policy
    {
        get => string.IsNullOrEmpty(PolicyJson) || PolicyJson == "\"Basic\""
            ? new BasicPolicy()
            : JsonSerializer.Deserialize<RoutingPolicy>(PolicyJson) ?? new BasicPolicy();
        set => PolicyJson = JsonSerializer.Serialize(value);
    }
}

public class RouteProperties
{
    public Guid Id { get; set; } = Guid.NewGuid();
    public string? RouteId { get; set; }  // Nullable - matches click-router Option<String>
    public string? DomainId { get; set; }  // Nullable
    public string? OwnerId { get; set; }  // Nullable
    public string? CreatorId { get; set; }  // NEW - matches click-router
    public string? WorkspaceId { get; set; }  // NEW - matches click-router
    public string ScriptsJson { get; set; } = "[]";
    public string TagsJson { get; set; } = "[]";
    public string CustomJson { get; set; } = "{}";
    public string NativeJson { get; set; } = "{}";  // NEW - matches click-router
    public string BundlingJson { get; set; } = "{}";  // NEW - matches click-router
    public bool Opengraph { get; set; }
    public bool AllowDebug { get; set; }

    public List<string>? Scripts
    {
        get => string.IsNullOrEmpty(ScriptsJson) || ScriptsJson == "[]"
            ? null
            : JsonSerializer.Deserialize<List<string>>(ScriptsJson);
        set => ScriptsJson = value == null || value.Count == 0
            ? "[]"
            : JsonSerializer.Serialize(value);
    }

    public List<string>? Tags
    {
        get => string.IsNullOrEmpty(TagsJson) || TagsJson == "[]"
            ? null
            : JsonSerializer.Deserialize<List<string>>(TagsJson);
        set => TagsJson = value == null || value.Count == 0
            ? "[]"
            : JsonSerializer.Serialize(value);
    }

    public Dictionary<string, object>? Custom
    {
        get => string.IsNullOrEmpty(CustomJson) || CustomJson == "{}"
            ? null
            : JsonSerializer.Deserialize<Dictionary<string, object>>(CustomJson);
        set => CustomJson = value == null || value.Count == 0
            ? "{}"
            : JsonSerializer.Serialize(value);
    }

    public Dictionary<string, object>? Native
    {
        get => string.IsNullOrEmpty(NativeJson) || NativeJson == "{}"
            ? null
            : JsonSerializer.Deserialize<Dictionary<string, object>>(NativeJson);
        set => NativeJson = value == null || value.Count == 0
            ? "{}"
            : JsonSerializer.Serialize(value);
    }

    public Dictionary<string, object>? Bundling
    {
        get => string.IsNullOrEmpty(BundlingJson) || BundlingJson == "{}"
            ? null
            : JsonSerializer.Deserialize<Dictionary<string, object>>(BundlingJson);
        set => BundlingJson = value == null || value.Count == 0
            ? "{}"
            : JsonSerializer.Serialize(value);
    }
}

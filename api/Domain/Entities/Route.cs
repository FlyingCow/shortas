using System.Text.Json;

namespace ShortasProxyApi.Domain.Entities;

public class Route
{
    public Guid Id { get; set; } = Guid.NewGuid();
    public string Switch { get; set; } = string.Empty;
    public string Link { get; set; } = string.Empty;
    public string Dest { get; set; } = string.Empty;
    public string DestFormat { get; set; } = string.Empty;
    public int Code { get; set; }
    public int Ttl { get; set; }
    public string Status { get; set; } = string.Empty;
    public string Terminal { get; set; } = string.Empty;
    public RouteProperties? Properties { get; set; }
}

public class RouteProperties
{
    public Guid Id { get; set; } = Guid.NewGuid();
    public string RouteId { get; set; } = string.Empty;
    public string DomainId { get; set; } = string.Empty;
    public string OwnerId { get; set; } = string.Empty;
    public string ScriptsJson { get; set; } = "[]";
    public string TagsJson { get; set; } = "[]";
    public string CustomJson { get; set; } = "{}";
    public bool Opengraph { get; set; }
    public bool AllowDebug { get; set; }

    public List<string> Scripts
    {
        get => JsonSerializer.Deserialize<List<string>>(ScriptsJson) ?? new();
        set => ScriptsJson = JsonSerializer.Serialize(value);
    }

    public List<string> Tags
    {
        get => JsonSerializer.Deserialize<List<string>>(TagsJson) ?? new();
        set => TagsJson = JsonSerializer.Serialize(value);
    }

    public Dictionary<string, object> Custom
    {
        get => JsonSerializer.Deserialize<Dictionary<string, object>>(CustomJson) ?? new();
        set => CustomJson = JsonSerializer.Serialize(value);
    }
}

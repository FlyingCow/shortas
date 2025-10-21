using System.Text.Json;

namespace ShortasProxyApi.Domain.Entities;

public class UserSettings
{
    public Guid Id { get; set; } = Guid.NewGuid();
    public string Email { get; set; } = string.Empty;
    public string Status { get; set; } = string.Empty;
    public bool Debug { get; set; }
    public bool Overflow { get; set; }
    public string SkipTrackingJson { get; set; } = "[]";
    public string AllowedRequestParamsJson { get; set; } = "[]";
    public string AllowedDestinationParamsJson { get; set; } = "[]";  

    public List<string> SkipTracking
    {
        get => JsonSerializer.Deserialize<List<string>>(SkipTrackingJson) ?? new();
        set => SkipTrackingJson = JsonSerializer.Serialize(value);
    }

    public List<string> AllowedRequestParams
    {
        get => JsonSerializer.Deserialize<List<string>>(AllowedRequestParamsJson) ?? new();
        set => AllowedRequestParamsJson = JsonSerializer.Serialize(value);
    }

    public List<string> AllowedDestinationParams
    {
        get => JsonSerializer.Deserialize<List<string>>(AllowedDestinationParamsJson) ?? new();
        set => AllowedDestinationParamsJson = JsonSerializer.Serialize(value);
    }
}

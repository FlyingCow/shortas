namespace ShortasProxyApi.Application.DTOs;

public class UserSettingsDto
{
    public string Email { get; set; } = string.Empty;
    public string Status { get; set; } = string.Empty;
    public bool Debug { get; set; }
    public bool Overflow { get; set; }
    public List<string> SkipTracking { get; set; } = new();
    public List<string> AllowedRequestParams { get; set; } = new();
    public List<string> AllowedDestinationParams { get; set; } = new();
}


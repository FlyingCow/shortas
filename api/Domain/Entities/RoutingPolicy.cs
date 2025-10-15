using System.Text.Json.Serialization;

namespace ShortasProxyApi.Domain.Entities;

/// <summary>
/// Routing policy for a route
/// Matches click-router RoutingPolicy enum
/// </summary>
[JsonConverter(typeof(RoutingPolicyConverter))]
public abstract class RoutingPolicy
{
    public abstract string PolicyType { get; }
}

/// <summary>
/// Basic routing - simple redirect to dest
/// </summary>
public class BasicPolicy : RoutingPolicy
{
    public override string PolicyType => "Basic";
}

/// <summary>
/// Conditional routing - redirect based on conditions
/// </summary>
public class ConditionalPolicy : RoutingPolicy
{
    public override string PolicyType => "Conditional";

    [JsonPropertyName("conditions")]
    public List<ConditionalRouting> Conditions { get; set; } = new();
}

/// <summary>
/// Challenge routing - show a challenge page before redirecting
/// </summary>
public class ChallengePolicy : RoutingPolicy
{
    public override string PolicyType => "Challenge";

    [JsonPropertyName("challenge")]
    public ChallengeRouting? Challenge { get; set; }
}

/// <summary>
/// File routing - serve a file instead of redirecting
/// </summary>
public class FilePolicy : RoutingPolicy
{
    public override string PolicyType => "File";

    [JsonPropertyName("file")]
    public FileRouting? File { get; set; }
}

/// <summary>
/// Mirroring routing - mirror the destination website
/// </summary>
public class MirroringPolicy : RoutingPolicy
{
    public override string PolicyType => "Mirroring";
}

/// <summary>
/// Conditional routing entry
/// </summary>
public class ConditionalRouting
{
    [JsonPropertyName("key")]
    public string Key { get; set; } = string.Empty;

    [JsonPropertyName("condition")]
    public Expression Condition { get; set; } = new();
}

/// <summary>
/// Challenge routing configuration
/// </summary>
public class ChallengeRouting
{
    [JsonPropertyName("type")]
    public string? Type { get; set; }

    [JsonPropertyName("title")]
    public string? Title { get; set; }

    [JsonPropertyName("message")]
    public string? Message { get; set; }
}

/// <summary>
/// File routing configuration
/// </summary>
public class FileRouting
{
    [JsonPropertyName("path")]
    public string? Path { get; set; }

    [JsonPropertyName("mime_type")]
    public string? MimeType { get; set; }
}

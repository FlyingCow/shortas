using System.Text.Json;
using System.Text.Json.Serialization;

namespace ShortasProxyApi.Domain.Common;

/// <summary>
/// Centralized JSON serialization configuration.
/// Use JsonConfig.Default for all serialization/deserialization operations.
/// </summary>
public static class JsonConfig
{
    /// <summary>
    /// Default JSON serializer options used throughout the application.
    /// - CamelCase property naming for API compatibility
    /// - Case-insensitive property matching for flexibility
    /// - Null values ignored in output to reduce payload size
    /// </summary>
    public static readonly JsonSerializerOptions Default = new()
    {
        PropertyNamingPolicy = JsonNamingPolicy.CamelCase,
        PropertyNameCaseInsensitive = true,
        DefaultIgnoreCondition = JsonIgnoreCondition.WhenWritingNull
    };

    /// <summary>
    /// Indented JSON options for debugging/logging purposes.
    /// Same settings as Default but with human-readable formatting.
    /// </summary>
    public static readonly JsonSerializerOptions Indented = new()
    {
        PropertyNamingPolicy = JsonNamingPolicy.CamelCase,
        PropertyNameCaseInsensitive = true,
        DefaultIgnoreCondition = JsonIgnoreCondition.WhenWritingNull,
        WriteIndented = true
    };
}

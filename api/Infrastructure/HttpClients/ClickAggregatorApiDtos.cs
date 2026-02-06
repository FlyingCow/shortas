using System.Text.Json.Serialization;
using ShortasProxyApi.Application.DTOs;

namespace ShortasProxyApi.Infrastructure.HttpClients;

/// <summary>
/// DTO for deserializing click stream data from the Click Aggregator API (Rust)
/// This DTO maps snake_case JSON fields to C# properties
/// </summary>
public class ClickAggregatorApiClickStreamDto
{
    [JsonPropertyName("id")]
    public string Id { get; set; } = string.Empty;

    [JsonPropertyName("owner_id")]
    public string OwnerId { get; set; } = string.Empty;

    [JsonPropertyName("creator_id")]
    public string CreatorId { get; set; } = string.Empty;

    [JsonPropertyName("route_id")]
    public string RouteId { get; set; } = string.Empty;

    [JsonPropertyName("workspace_id")]
    public string WorkspaceId { get; set; } = string.Empty;

    [JsonPropertyName("created")]
    public DateTime Created { get; set; }

    [JsonPropertyName("dest")]
    public string Dest { get; set; } = string.Empty;

    [JsonPropertyName("ip")]
    public string Ip { get; set; } = string.Empty;

    [JsonPropertyName("continent")]
    public string Continent { get; set; } = ClickStreamDto.Unknown;

    [JsonPropertyName("country")]
    public string Country { get; set; } = ClickStreamDto.Unknown;

    [JsonPropertyName("location")]
    public string Location { get; set; } = ClickStreamDto.Unknown;

    [JsonPropertyName("os_family")]
    public string OsFamily { get; set; } = ClickStreamDto.Unknown;

    [JsonPropertyName("os_version")]
    public string OsVersion { get; set; } = ClickStreamDto.Unknown;

    [JsonPropertyName("user_agent_family")]
    public string UserAgentFamily { get; set; } = ClickStreamDto.Unknown;

    [JsonPropertyName("user_agent_version")]
    public string UserAgentVersion { get; set; } = ClickStreamDto.Unknown;

    [JsonPropertyName("device_brand")]
    public string DeviceBrand { get; set; } = ClickStreamDto.Unknown;

    [JsonPropertyName("device_family")]
    public string DeviceFamily { get; set; } = ClickStreamDto.Unknown;

    [JsonPropertyName("device_model")]
    public string DeviceModel { get; set; } = ClickStreamDto.Unknown;

    [JsonPropertyName("session_first")]
    public DateTime SessionFirst { get; set; } = ClickStreamDto.EpochDateTime;

    [JsonPropertyName("session_clicks")]
    public long SessionClicks { get; set; } = 0;

    [JsonPropertyName("is_unique")]
    public bool IsUnique { get; set; }

    [JsonPropertyName("is_bot")]
    public bool IsBot { get; set; }

    /// <summary>
    /// Maps API DTO to Application DTO
    /// </summary>
    public static ClickStreamDto ToDto(ClickAggregatorApiClickStreamDto dto)
    {
        return new ClickStreamDto
        {
            Id = dto.Id,
            OwnerId = dto.OwnerId,
            CreatorId = dto.CreatorId,
            RouteId = dto.RouteId,
            WorkspaceId = dto.WorkspaceId,
            Created = dto.Created,
            Dest = dto.Dest,
            Ip = dto.Ip,
            Continent = dto.Continent,
            Country = dto.Country,
            Location = dto.Location,
            OsFamily = dto.OsFamily,
            OsVersion = dto.OsVersion,
            UserAgentFamily = dto.UserAgentFamily,
            UserAgentVersion = dto.UserAgentVersion,
            DeviceBrand = dto.DeviceBrand,
            DeviceFamily = dto.DeviceFamily,
            DeviceModel = dto.DeviceModel,
            SessionFirst = dto.SessionFirst,
            SessionClicks = dto.SessionClicks,
            IsUnique = dto.IsUnique,
            IsBot = dto.IsBot
        };
    }
}

/// <summary>
/// Response wrapper for paginated click stream data from the API
/// </summary>
public class ClickAggregatorApiClickStreamResponse
{
    [JsonPropertyName("items")]
    public List<ClickAggregatorApiClickStreamDto> Items { get; set; } = new();

    [JsonPropertyName("total")]
    public long Total { get; set; }

    [JsonPropertyName("offset")]
    public int Offset { get; set; }

    [JsonPropertyName("limit")]
    public int Limit { get; set; }

    [JsonPropertyName("has_more")]
    public bool HasMore { get; set; }
}

// ==================== Statistics DTOs ====================

/// <summary>
/// Daily click statistics
/// </summary>
public class DailyStatsDto
{
    [JsonPropertyName("date")]
    public string Date { get; set; } = string.Empty;

    [JsonPropertyName("total_clicks")]
    public long TotalClicks { get; set; }

    [JsonPropertyName("unique_clicks")]
    public long UniqueClicks { get; set; }

    [JsonPropertyName("bot_clicks")]
    public long BotClicks { get; set; }

    [JsonPropertyName("human_clicks")]
    public long HumanClicks { get; set; }

    [JsonPropertyName("unique_ips")]
    public long UniqueIps { get; set; }
}

/// <summary>
/// Hourly click statistics
/// </summary>
public class HourlyStatsDto
{
    [JsonPropertyName("hour")]
    public DateTime Hour { get; set; }

    [JsonPropertyName("total_clicks")]
    public long TotalClicks { get; set; }

    [JsonPropertyName("unique_clicks")]
    public long UniqueClicks { get; set; }

    [JsonPropertyName("bot_clicks")]
    public long BotClicks { get; set; }

    [JsonPropertyName("human_clicks")]
    public long HumanClicks { get; set; }

    [JsonPropertyName("unique_ips")]
    public long UniqueIps { get; set; }
}

/// <summary>
/// Geographic statistics
/// </summary>
public class GeographicStatsDto
{
    [JsonPropertyName("continent")]
    public string? Continent { get; set; }

    [JsonPropertyName("country")]
    public string Country { get; set; } = string.Empty;

    [JsonPropertyName("location")]
    public string? Location { get; set; }

    [JsonPropertyName("total_clicks")]
    public long TotalClicks { get; set; }

    [JsonPropertyName("unique_clicks")]
    public long UniqueClicks { get; set; }

    [JsonPropertyName("unique_ips")]
    public long UniqueIps { get; set; }
}

/// <summary>
/// Device statistics
/// </summary>
public class DeviceStatsDto
{
    [JsonPropertyName("device_family")]
    public string DeviceFamily { get; set; } = string.Empty;

    [JsonPropertyName("os_family")]
    public string OsFamily { get; set; } = string.Empty;

    [JsonPropertyName("total_clicks")]
    public long TotalClicks { get; set; }

    [JsonPropertyName("unique_clicks")]
    public long UniqueClicks { get; set; }
}

/// <summary>
/// Browser statistics
/// </summary>
public class BrowserStatsDto
{
    [JsonPropertyName("user_agent_family")]
    public string UserAgentFamily { get; set; } = string.Empty;

    [JsonPropertyName("user_agent_version")]
    public string? UserAgentVersion { get; set; }

    [JsonPropertyName("total_clicks")]
    public long TotalClicks { get; set; }

    [JsonPropertyName("unique_clicks")]
    public long UniqueClicks { get; set; }
}

/// <summary>
/// Route performance statistics
/// </summary>
public class RoutePerformanceDto
{
    [JsonPropertyName("route_id")]
    public string RouteId { get; set; } = string.Empty;

    /// <summary>Route name (enriched from local data)</summary>
    [JsonPropertyName("route_name")]
    public string? RouteName { get; set; }

    /// <summary>Route domain name (enriched from local data)</summary>
    [JsonPropertyName("route_domain_name")]
    public string? RouteDomainName { get; set; }

    [JsonPropertyName("total_clicks")]
    public long TotalClicks { get; set; }

    [JsonPropertyName("unique_visitors")]
    public long UniqueVisitors { get; set; }

    [JsonPropertyName("bot_clicks")]
    public long BotClicks { get; set; }

    [JsonPropertyName("human_clicks")]
    public long HumanClicks { get; set; }

    [JsonPropertyName("countries_reached")]
    public long CountriesReached { get; set; }

    [JsonPropertyName("device_types")]
    public long DeviceTypes { get; set; }
}

/// <summary>
/// Top destination statistics
/// </summary>
public class TopDestinationDto
{
    [JsonPropertyName("dest")]
    public string Dest { get; set; } = string.Empty;

    [JsonPropertyName("total_clicks")]
    public long TotalClicks { get; set; }

    [JsonPropertyName("unique_visitors")]
    public long UniqueVisitors { get; set; }
}

/// <summary>
/// Traffic type (bot vs human) statistics
/// </summary>
public class TrafficTypeStatsDto
{
    [JsonPropertyName("is_bot")]
    public bool IsBot { get; set; }

    [JsonPropertyName("total_clicks")]
    public long TotalClicks { get; set; }

    [JsonPropertyName("unique_ips")]
    public long UniqueIps { get; set; }
}

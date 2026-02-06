using ShortasProxyApi.Domain.Common;

namespace ShortasProxyApi.Domain.Interfaces;

/// <summary>
/// Search document representing a route in the search index
/// </summary>
public class RouteSearchDocument
{
    public string Id { get; set; } = string.Empty;
    public string Link { get; set; } = string.Empty;
    public string Switch { get; set; } = string.Empty;
    public string? Dest { get; set; }
    public string? DomainName { get; set; }
    public string Status { get; set; } = "Active";
    public string? OwnerId { get; set; }
    public string? WorkspaceId { get; set; }
}

/// <summary>
/// Service for managing route search index (Elasticsearch)
/// </summary>
public interface IRouteSearchService
{
    /// <summary>Ensure the search index exists with proper mappings</summary>
    Task EnsureIndexAsync();

    /// <summary>Index a single route document (create or update)</summary>
    Task<Result> IndexRouteAsync(RouteSearchDocument document);

    /// <summary>Index multiple route documents (create or update)</summary>
    Task<Result> IndexRoutesAsync(List<RouteSearchDocument> documents);

    /// <summary>Delete a single route from the index</summary>
    Task<Result> DeleteRouteAsync(string routeId);

    /// <summary>Delete multiple routes from the index</summary>
    Task<Result> DeleteRoutesAsync(List<string> routeIds);

    /// <summary>Search routes by query across link, domain name, and destination</summary>
    Task<Result<(List<RouteSearchDocument> Results, long TotalCount)>> SearchAsync(
        string query,
        string? ownerId = null,
        string? workspaceId = null,
        int page = 1,
        int pageSize = 20);
}

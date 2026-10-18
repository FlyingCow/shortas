using ShortasProxyApi.Domain.Entities;
using ShortasProxyApi.Domain.Common;

namespace ShortasProxyApi.Domain.Interfaces;

/// <summary>
/// Write operations for routes. Implemented by both EfRouteService (database)
/// and RouteService (HTTP client to click-router-api).
/// </summary>
public interface IRouteCommandService
{
    /// <summary>Create a new route</summary>
    Task<Result<Entities.Route>> CreateRouteAsync(Entities.Route route);

    /// <summary>Update route by internal ID</summary>
    Task<Result<Entities.Route>> UpdateRouteByIdAsync(Guid id, string userId, Entities.Route route);

    /// <summary>Update route by domain/path (legacy)</summary>
    Task<Result<Entities.Route>> UpdateRouteAsync(string domain, string path, string userId, Entities.Route route);

    /// <summary>Delete route by internal ID</summary>
    Task<Result> DeleteRouteByIdAsync(Guid id, string userId);

    /// <summary>Delete route by domain/path (legacy)</summary>
    Task<Result> DeleteRouteAsync(string domain, string path, string userId, string? switchParam = null);

    /// <summary>Bulk create routes</summary>
    Task<Result<List<Entities.Route>>> BulkCreateRoutesAsync(List<Entities.Route> routes);

    /// <summary>Bulk update routes</summary>
    Task<Result<List<Entities.Route>>> BulkUpdateRoutesAsync(string userId, List<Entities.Route> routes);

    /// <summary>Bulk delete routes by IDs</summary>
    Task<Result> BulkDeleteRoutesAsync(string userId, List<string> routeIds);
}

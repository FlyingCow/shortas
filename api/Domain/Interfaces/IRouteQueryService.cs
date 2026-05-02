using ShortasProxyApi.Domain.Entities;
using ShortasProxyApi.Domain.Common;

namespace ShortasProxyApi.Domain.Interfaces;

/// <summary>
/// Read operations for routes. Only implemented by EfRouteService (database).
/// The HTTP client service (RouteService) cannot perform read operations
/// because click-router-api uses domain/path identifiers, not internal IDs.
/// </summary>
public interface IRouteQueryService
{
    /// <summary>Get route by internal ID</summary>
    Task<Result<Entities.Route?>> GetRouteByIdAsync(Guid id, string userId);

    /// <summary>Get route by domain/path (legacy)</summary>
    Task<Result<Entities.Route?>> GetRouteAsync(string domain, string path, string userId, string? switchParam = null);

    /// <summary>List routes with pagination and filtering</summary>
    Task<Result<(List<Entities.Route> Routes, int TotalCount)>> ListRoutesAsync(
        int page = 1,
        int pageSize = 20,
        string? search = null,
        string? status = null,
        string? ownerId = null,
        string? workspaceId = null);
}

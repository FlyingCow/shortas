using ShortasProxyApi.Domain.Entities;
using ShortasProxyApi.Domain.Common;

namespace ShortasProxyApi.Domain.Interfaces;

public interface IRouteService
{
    // ID-based methods (primary)
    Task<Result<Entities.Route?>> GetRouteByIdAsync(Guid id, string userId);
    Task<Result<Entities.Route>> UpdateRouteByIdAsync(Guid id, string userId, Entities.Route route);
    Task<Result> DeleteRouteByIdAsync(Guid id, string userId);

    // Legacy domain/path methods (kept for backward compatibility)
    Task<Result<Entities.Route?>> GetRouteAsync(string domain, string path, string userId, string? switchParam = null);
    Task<Result<Entities.Route>> UpdateRouteAsync(string domain, string path, string userId, Entities.Route route);
    Task<Result> DeleteRouteAsync(string domain, string path, string userId);

    // Common methods
    Task<Result<Entities.Route>> CreateRouteAsync(Entities.Route route);
    Task<Result<List<Entities.Route>>> BulkCreateRoutesAsync(List<Entities.Route> routes);
    Task<Result<List<Entities.Route>>> BulkUpdateRoutesAsync(string userId, List<Entities.Route> routes);
    Task<Result> BulkDeleteRoutesAsync(string userId, List<string> routeIds);
    Task<Result<(List<Entities.Route> Routes, int TotalCount)>> ListRoutesAsync(
        int page = 1,
        int pageSize = 20,
        string? search = null,
        string? status = null,
        string? ownerId = null);
}
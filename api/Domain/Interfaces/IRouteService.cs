using ShortasProxyApi.Domain.Entities;
using ShortasProxyApi.Domain.Common;

namespace ShortasProxyApi.Domain.Interfaces;

/// <summary>
/// Combined route service interface for backwards compatibility.
/// Prefer using IRouteCommandService or IRouteQueryService directly:
/// - IRouteCommandService: Write operations (create, update, delete)
/// - IRouteQueryService: Read operations (get, list)
///
/// EfRouteService implements all operations (IRouteService).
/// RouteService (HTTP client) only implements IRouteCommandService.
/// </summary>
public interface IRouteService : IRouteCommandService, IRouteQueryService
{
    // All methods are inherited from IRouteCommandService and IRouteQueryService.
    // This interface exists for backwards compatibility with existing code
    // that injects IRouteService directly.
}
namespace ShortasProxyApi.Domain.Entities;

/// <summary>
/// Maintains a cached count of routes per domain.
/// Used by the slash tag generator to avoid expensive COUNT queries.
/// Updated atomically on route create/delete.
/// </summary>
public class DomainRouteCount
{
    public Guid DomainId { get; set; }
    public int RouteCount { get; set; }
}

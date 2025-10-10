using ShortasProxyApi.Domain.Entities;
using ShortasProxyApi.Domain.Common;

namespace ShortasProxyApi.Domain.Interfaces;

public interface IClickStreamService
{
    Task<Result<List<Entities.ClickStream>>> GetClickStreamAsync(string? routeId = null, DateTime? startDate = null, DateTime? endDate = null);
    Task<Result<Dictionary<string, object>>> GetClickStreamStatsAsync(string? routeId = null, DateTime? startDate = null, DateTime? endDate = null);
}
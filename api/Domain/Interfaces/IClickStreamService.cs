using ShortasProxyApi.Domain.Common;
using ShortasProxyApi.Application.DTOs;

namespace ShortasProxyApi.Domain.Interfaces;

public interface IClickStreamService
{
    Task<Result<List<ClickStreamDto>>> GetClickStreamAsync(string? routeId = null, DateTime? startDate = null, DateTime? endDate = null);
    Task<Result<Dictionary<string, object>>> GetClickStreamStatsAsync(string? routeId = null, DateTime? startDate = null, DateTime? endDate = null);
}
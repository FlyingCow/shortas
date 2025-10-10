using ShortasProxyApi.Domain.Interfaces;
using ShortasProxyApi.Application.Services;
using ShortasProxyApi.Domain.Common;

namespace ShortasProxyApi.Infrastructure.HttpClients;

public class ClickAggregatorApiClient : IClickStreamService
{
    private readonly ClickStreamService _clickStreamService;

    public ClickAggregatorApiClient(ClickStreamService clickStreamService)
    {
        _clickStreamService = clickStreamService;
    }

    public Task<Result<List<Domain.Entities.ClickStream>>> GetClickStreamAsync(string? routeId = null, DateTime? startDate = null, DateTime? endDate = null)
        => _clickStreamService.GetClickStreamAsync(routeId, startDate, endDate);

    public Task<Result<Dictionary<string, object>>> GetClickStreamStatsAsync(string? routeId = null, DateTime? startDate = null, DateTime? endDate = null)
        => _clickStreamService.GetClickStreamStatsAsync(routeId, startDate, endDate);
}
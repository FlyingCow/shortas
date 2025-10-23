using ShortasProxyApi.Domain.Interfaces;
using ShortasProxyApi.Domain.Common;
using ShortasProxyApi.Application.DTOs;
using ShortasProxyApi.Infrastructure.HttpClients;

namespace ShortasProxyApi.Infrastructure.Services;

/// <summary>
/// Service implementation that uses ClickAggregatorApiClient for HTTP communication.
/// This service implements the IClickStreamService interface and delegates to the HTTP client.
/// </summary>
public class ClickAggregatorApiService : IClickStreamService
{
    private readonly ClickAggregatorApiClient _httpClient;
    private readonly ILogger<ClickAggregatorApiService> _logger;

    public ClickAggregatorApiService(ClickAggregatorApiClient httpClient, ILogger<ClickAggregatorApiService> logger)
    {
        _httpClient = httpClient;
        _logger = logger;
    }

    #region IClickStreamService Implementation

    public async Task<Result<List<ClickStreamDto>>> GetClickStreamAsync(string? routeId = null, DateTime? startDate = null, DateTime? endDate = null)
    {
        _logger.LogDebug("Getting click stream data - startDate {StartDate}, endDate {EndDate}, routeId {RouteId}",
            startDate, endDate, routeId);

        // Call the HTTP client which now returns properly typed List<ClickStreamDto>
        var result = await _httpClient.GetClickStreamAsync(startDate, endDate, routeId, null, offset: 0, limit: 100);

        if (result.IsFailure)
        {
            return Result<List<ClickStreamDto>>.Failure(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        return Result<List<ClickStreamDto>>.Success(result.Value);
    }

    public async Task<Result<Dictionary<string, object>>> GetClickStreamStatsAsync(string? routeId = null, DateTime? startDate = null, DateTime? endDate = null)
    {
        _logger.LogDebug("Getting click stream stats - startDate {StartDate}, endDate {EndDate}, routeId {RouteId}",
            startDate, endDate, routeId);

        // Get the clickstream data and calculate stats from it
        var clickStreamResult = await GetClickStreamAsync(routeId, startDate, endDate);

        if (clickStreamResult.IsFailure)
        {
            return Result<Dictionary<string, object>>.Failure(clickStreamResult.ErrorCode ?? "UNKNOWN_ERROR", clickStreamResult.Error);
        }

        var clickStreams = clickStreamResult.Value;

        var stats = new Dictionary<string, object>
        {
            ["total_clicks"] = clickStreams.Count,
            ["unique_clicks"] = clickStreams.Count(c => c.IsUnique),
            ["bot_clicks"] = clickStreams.Count(c => c.IsBot),
            ["countries"] = clickStreams
                .Where(c => !ClickStreamDto.IsUnknown(c.Country))
                .GroupBy(c => c.Country)
                .Select(g => new { country = g.Key, count = g.Count() })
                .OrderByDescending(x => x.count)
                .Take(10)
                .ToList(),
            ["devices"] = clickStreams
                .Where(c => !ClickStreamDto.IsUnknown(c.DeviceFamily))
                .GroupBy(c => c.DeviceFamily)
                .Select(g => new { device = g.Key, count = g.Count() })
                .OrderByDescending(x => x.count)
                .Take(10)
                .ToList(),
            ["browsers"] = clickStreams
                .Where(c => !ClickStreamDto.IsUnknown(c.UserAgentFamily))
                .GroupBy(c => c.UserAgentFamily)
                .Select(g => new { browser = g.Key, count = g.Count() })
                .OrderByDescending(x => x.count)
                .Take(10)
                .ToList(),
            ["os"] = clickStreams
                .Where(c => !ClickStreamDto.IsUnknown(c.OsFamily))
                .GroupBy(c => c.OsFamily)
                .Select(g => new { os = g.Key, count = g.Count() })
                .OrderByDescending(x => x.count)
                .Take(10)
                .ToList()
        };

        _logger.LogInformation("Calculated stats for {TotalClicks} clicks", clickStreams.Count);
        return Result<Dictionary<string, object>>.Success(stats);
    }

    #endregion

    #region Additional Methods (not part of interface but available for future use)

    public async Task<Result<object>> GetClickStreamOverviewAsync(
        DateTime? startDate = null,
        DateTime? endDate = null,
        string? ownerId = null)
    {
        _logger.LogDebug("Getting click stream overview - startDate {StartDate}, endDate {EndDate}, ownerId {OwnerId}", 
            startDate, endDate, ownerId);
        return await _httpClient.GetClickStreamOverviewAsync(startDate, endDate, ownerId);
    }

    public async Task<Result<object>> GetRouteAnalyticsAsync(
        string routeId,
        DateTime? startDate = null,
        DateTime? endDate = null,
        string? ownerId = null)
    {
        _logger.LogDebug("Getting route analytics for route {RouteId} - startDate {StartDate}, endDate {EndDate}, ownerId {OwnerId}", 
            routeId, startDate, endDate, ownerId);
        return await _httpClient.GetRouteAnalyticsAsync(routeId, startDate, endDate, ownerId);
    }

    public async Task<Result<object>> GetGeographicAnalyticsAsync(
        DateTime? startDate = null,
        DateTime? endDate = null,
        string? ownerId = null,
        string? groupBy = "country")
    {
        _logger.LogDebug("Getting geographic analytics - startDate {StartDate}, endDate {EndDate}, ownerId {OwnerId}, groupBy {GroupBy}", 
            startDate, endDate, ownerId, groupBy);
        return await _httpClient.GetGeographicAnalyticsAsync(startDate, endDate, ownerId, groupBy);
    }

    public async Task<Result<object>> GetDeviceAnalyticsAsync(
        DateTime? startDate = null,
        DateTime? endDate = null,
        string? ownerId = null,
        string? groupBy = "device_family")
    {
        _logger.LogDebug("Getting device analytics - startDate {StartDate}, endDate {EndDate}, ownerId {OwnerId}, groupBy {GroupBy}", 
            startDate, endDate, ownerId, groupBy);
        return await _httpClient.GetDeviceAnalyticsAsync(startDate, endDate, ownerId, groupBy);
    }

    public async Task<Result<object>> GetBrowserAnalyticsAsync(
        DateTime? startDate = null,
        DateTime? endDate = null,
        string? ownerId = null,
        string? groupBy = "user_agent_family")
    {
        _logger.LogDebug("Getting browser analytics - startDate {StartDate}, endDate {EndDate}, ownerId {OwnerId}, groupBy {GroupBy}", 
            startDate, endDate, ownerId, groupBy);
        return await _httpClient.GetBrowserAnalyticsAsync(startDate, endDate, ownerId, groupBy);
    }

    public async Task<Result<object>> GetTimeSeriesAnalyticsAsync(
        DateTime? startDate = null,
        DateTime? endDate = null,
        string? ownerId = null,
        string? interval = "hour")
    {
        _logger.LogDebug("Getting time series analytics - startDate {StartDate}, endDate {EndDate}, ownerId {OwnerId}, interval {Interval}",
            startDate, endDate, ownerId, interval);
        return await _httpClient.GetTimeSeriesAnalyticsAsync(startDate, endDate, ownerId, interval);
    }

    #endregion

    #region Materialized View Statistics Methods

    public async Task<Result<List<DailyStatsDto>>> GetDailyStatsAsync(string? ownerId = null, string? routeId = null, string? fromDate = null, string? toDate = null)
    {
        _logger.LogDebug("Getting daily stats - ownerId {OwnerId}, routeId {RouteId}, fromDate {FromDate}, toDate {ToDate}",
            ownerId, routeId, fromDate, toDate);
        return await _httpClient.GetDailyStatsAsync(ownerId, routeId, fromDate, toDate);
    }

    public async Task<Result<List<HourlyStatsDto>>> GetHourlyStatsAsync(string? ownerId = null, string? routeId = null, string? fromHour = null, string? toHour = null)
    {
        _logger.LogDebug("Getting hourly stats - ownerId {OwnerId}, routeId {RouteId}, fromHour {FromHour}, toHour {ToHour}",
            ownerId, routeId, fromHour, toHour);
        return await _httpClient.GetHourlyStatsAsync(ownerId, routeId, fromHour, toHour);
    }

    public async Task<Result<List<GeographicStatsDto>>> GetGeographicStatsAsync(string? ownerId = null, string? routeId = null, string? fromDate = null, string? toDate = null)
    {
        _logger.LogDebug("Getting geographic stats - ownerId {OwnerId}, routeId {RouteId}, fromDate {FromDate}, toDate {ToDate}",
            ownerId, routeId, fromDate, toDate);
        return await _httpClient.GetGeographicStatsAsync(ownerId, routeId, fromDate, toDate);
    }

    public async Task<Result<List<DeviceStatsDto>>> GetDeviceStatsAsync(string? ownerId = null, string? routeId = null, string? fromDate = null, string? toDate = null)
    {
        _logger.LogDebug("Getting device stats - ownerId {OwnerId}, routeId {RouteId}, fromDate {FromDate}, toDate {ToDate}",
            ownerId, routeId, fromDate, toDate);
        return await _httpClient.GetDeviceStatsAsync(ownerId, routeId, fromDate, toDate);
    }

    public async Task<Result<List<BrowserStatsDto>>> GetBrowserStatsAsync(string? ownerId = null, string? routeId = null, string? fromDate = null, string? toDate = null)
    {
        _logger.LogDebug("Getting browser stats - ownerId {OwnerId}, routeId {RouteId}, fromDate {FromDate}, toDate {ToDate}",
            ownerId, routeId, fromDate, toDate);
        return await _httpClient.GetBrowserStatsAsync(ownerId, routeId, fromDate, toDate);
    }

    public async Task<Result<List<RoutePerformanceDto>>> GetRoutePerformanceAsync(string? ownerId = null, string? fromDate = null, string? toDate = null, int? limit = null)
    {
        _logger.LogDebug("Getting route performance - ownerId {OwnerId}, fromDate {FromDate}, toDate {ToDate}, limit {Limit}",
            ownerId, fromDate, toDate, limit);
        return await _httpClient.GetRoutePerformanceAsync(ownerId, fromDate, toDate, limit);
    }

    public async Task<Result<List<TopDestinationDto>>> GetTopDestinationsAsync(string? ownerId = null, string? routeId = null, string? fromDate = null, string? toDate = null, int? limit = null)
    {
        _logger.LogDebug("Getting top destinations - ownerId {OwnerId}, routeId {RouteId}, fromDate {FromDate}, toDate {ToDate}, limit {Limit}",
            ownerId, routeId, fromDate, toDate, limit);
        return await _httpClient.GetTopDestinationsAsync(ownerId, routeId, fromDate, toDate, limit);
    }

    public async Task<Result<List<TrafficTypeStatsDto>>> GetTrafficTypeStatsAsync(string? ownerId = null, string? routeId = null, string? fromHour = null, string? toHour = null)
    {
        _logger.LogDebug("Getting traffic type stats - ownerId {OwnerId}, routeId {RouteId}, fromHour {FromHour}, toHour {ToHour}",
            ownerId, routeId, fromHour, toHour);
        return await _httpClient.GetTrafficTypeStatsAsync(ownerId, routeId, fromHour, toHour);
    }

    #endregion
}

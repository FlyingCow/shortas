using ShortasProxyApi.Domain.Interfaces;
using ShortasProxyApi.Domain.Common;
using ShortasProxyApi.Domain.Entities;
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

    public async Task<Result<List<ClickStream>>> GetClickStreamAsync(string? routeId = null, DateTime? startDate = null, DateTime? endDate = null)
    {
        _logger.LogDebug("Getting click stream data - startDate {StartDate}, endDate {EndDate}, routeId {RouteId}", 
            startDate, endDate, routeId);
        
        var result = await _httpClient.GetClickStreamAsync(startDate, endDate, routeId, null, 1, 100);
        
        if (result.IsFailure)
        {
            return Result<List<ClickStream>>.Failure(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }
        
        // Convert the object result to List<ClickStream>
        // This is a simplified conversion - in a real implementation, you'd need proper deserialization
        var clickStreams = new List<ClickStream>();
        return Result<List<ClickStream>>.Success(clickStreams);
    }

    public async Task<Result<Dictionary<string, object>>> GetClickStreamStatsAsync(string? routeId = null, DateTime? startDate = null, DateTime? endDate = null)
    {
        _logger.LogDebug("Getting click stream stats - startDate {StartDate}, endDate {EndDate}, routeId {RouteId}", 
            startDate, endDate, routeId);
        
        var result = await _httpClient.GetClickStreamStatsAsync(startDate, endDate, null, null);
        
        if (result.IsFailure)
        {
            return Result<Dictionary<string, object>>.Failure(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }
        
        // Convert the object result to Dictionary<string, object>
        // This is a simplified conversion - in a real implementation, you'd need proper deserialization
        var stats = new Dictionary<string, object>();
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
}

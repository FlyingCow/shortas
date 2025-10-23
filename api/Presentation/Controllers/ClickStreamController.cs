using Microsoft.AspNetCore.Authorization;
using Microsoft.AspNetCore.Mvc;
using ShortasProxyApi.Application.DTOs;
using ShortasProxyApi.Domain.Interfaces;
using ShortasProxyApi.Domain.Common;
using ShortasProxyApi.Infrastructure.Services;
using ShortasProxyApi.Infrastructure.HttpClients;
using ShortasProxyApi.Presentation.Extensions;

namespace ShortasProxyApi.Presentation.Controllers;

[ApiController]
[Route("api/v1/clickstream")]
[Authorize]
public class ClickStreamController : ControllerBase
{
    private readonly ClickAggregatorApiService _clickStreamService;
    private readonly ILogger<ClickStreamController> _logger;

    public ClickStreamController(IClickStreamService clickStreamService, ILogger<ClickStreamController> logger)
    {
        // Cast to concrete type to access additional analytics methods
        _clickStreamService = (ClickAggregatorApiService)clickStreamService;
        _logger = logger;
    }

    /// <summary>
    /// Get click stream data
    /// </summary>
    /// <param name="routeId">Route ID (optional)</param>
    /// <param name="startDate">Start date (optional)</param>
    /// <param name="endDate">End date (optional)</param>
    /// <returns>Click stream data</returns>
    [HttpGet]
    public async Task<ActionResult<List<ClickStreamDto>>> GetClickStream(
        [FromQuery] string? routeId = null,
        [FromQuery] DateTime? startDate = null,
        [FromQuery] DateTime? endDate = null)
    {
        var result = await _clickStreamService.GetClickStreamAsync(routeId, startDate, endDate);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        return Ok(result.Value);
    }

    /// <summary>
    /// Get click stream data for a specific route
    /// </summary>
    /// <param name="routeId">Route ID</param>
    /// <param name="startDate">Start date (optional)</param>
    /// <param name="endDate">End date (optional)</param>
    /// <returns>Click stream data for the route</returns>
    [HttpGet("{routeId}")]
    public async Task<ActionResult<List<ClickStreamDto>>> GetClickStreamByRoute(
        string routeId,
        [FromQuery] DateTime? startDate = null,
        [FromQuery] DateTime? endDate = null)
    {
        var result = await _clickStreamService.GetClickStreamAsync(routeId, startDate, endDate);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        return Ok(result.Value);
    }

    /// <summary>
    /// Get click stream statistics
    /// </summary>
    /// <param name="routeId">Route ID (optional)</param>
    /// <param name="startDate">Start date (optional)</param>
    /// <param name="endDate">End date (optional)</param>
    /// <returns>Click stream statistics</returns>
    [HttpGet("stats")]
    public async Task<ActionResult<Dictionary<string, object>>> GetClickStreamStats(
        [FromQuery] string? routeId = null,
        [FromQuery] DateTime? startDate = null,
        [FromQuery] DateTime? endDate = null)
    {
        var result = await _clickStreamService.GetClickStreamStatsAsync(routeId, startDate, endDate);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        return Ok(result.Value);
    }

    /// <summary>
    /// Get click stream analytics overview
    /// </summary>
    /// <param name="startDate">Start date (optional)</param>
    /// <param name="endDate">End date (optional)</param>
    /// <param name="ownerId">Owner ID (optional)</param>
    /// <returns>Analytics overview</returns>
    [HttpGet("overview")]
    public async Task<ActionResult<object>> GetClickStreamOverview(
        [FromQuery] DateTime? startDate = null,
        [FromQuery] DateTime? endDate = null,
        [FromQuery] string? ownerId = null)
    {
        var result = await _clickStreamService.GetClickStreamOverviewAsync(startDate, endDate, ownerId);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        return Ok(result.Value);
    }

    /// <summary>
    /// Get geographic analytics
    /// </summary>
    /// <param name="startDate">Start date (optional)</param>
    /// <param name="endDate">End date (optional)</param>
    /// <param name="ownerId">Owner ID (optional)</param>
    /// <param name="groupBy">Group by field (default: country)</param>
    /// <returns>Geographic analytics</returns>
    [HttpGet("analytics/geographic")]
    public async Task<ActionResult<object>> GetGeographicAnalytics(
        [FromQuery] DateTime? startDate = null,
        [FromQuery] DateTime? endDate = null,
        [FromQuery] string? ownerId = null,
        [FromQuery] string? groupBy = "country")
    {
        var result = await _clickStreamService.GetGeographicAnalyticsAsync(startDate, endDate, ownerId, groupBy);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        return Ok(result.Value);
    }

    /// <summary>
    /// Get device analytics
    /// </summary>
    /// <param name="startDate">Start date (optional)</param>
    /// <param name="endDate">End date (optional)</param>
    /// <param name="ownerId">Owner ID (optional)</param>
    /// <param name="groupBy">Group by field (default: device_family)</param>
    /// <returns>Device analytics</returns>
    [HttpGet("analytics/device")]
    public async Task<ActionResult<object>> GetDeviceAnalytics(
        [FromQuery] DateTime? startDate = null,
        [FromQuery] DateTime? endDate = null,
        [FromQuery] string? ownerId = null,
        [FromQuery] string? groupBy = "device_family")
    {
        var result = await _clickStreamService.GetDeviceAnalyticsAsync(startDate, endDate, ownerId, groupBy);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        return Ok(result.Value);
    }

    /// <summary>
    /// Get browser analytics
    /// </summary>
    /// <param name="startDate">Start date (optional)</param>
    /// <param name="endDate">End date (optional)</param>
    /// <param name="ownerId">Owner ID (optional)</param>
    /// <param name="groupBy">Group by field (default: user_agent_family)</param>
    /// <returns>Browser analytics</returns>
    [HttpGet("analytics/browser")]
    public async Task<ActionResult<object>> GetBrowserAnalytics(
        [FromQuery] DateTime? startDate = null,
        [FromQuery] DateTime? endDate = null,
        [FromQuery] string? ownerId = null,
        [FromQuery] string? groupBy = "user_agent_family")
    {
        var result = await _clickStreamService.GetBrowserAnalyticsAsync(startDate, endDate, ownerId, groupBy);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        return Ok(result.Value);
    }

    /// <summary>
    /// Get time series analytics
    /// </summary>
    /// <param name="startDate">Start date (optional)</param>
    /// <param name="endDate">End date (optional)</param>
    /// <param name="ownerId">Owner ID (optional)</param>
    /// <param name="interval">Time interval (default: hour)</param>
    /// <returns>Time series analytics</returns>
    [HttpGet("analytics/timeseries")]
    public async Task<ActionResult<object>> GetTimeSeriesAnalytics(
        [FromQuery] DateTime? startDate = null,
        [FromQuery] DateTime? endDate = null,
        [FromQuery] string? ownerId = null,
        [FromQuery] string? interval = "hour")
    {
        var result = await _clickStreamService.GetTimeSeriesAnalyticsAsync(startDate, endDate, ownerId, interval);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        return Ok(result.Value);
    }

    /// <summary>
    /// Get route-specific analytics
    /// </summary>
    /// <param name="routeId">Route ID</param>
    /// <param name="startDate">Start date (optional)</param>
    /// <param name="endDate">End date (optional)</param>
    /// <param name="ownerId">Owner ID (optional)</param>
    /// <returns>Route analytics</returns>
    [HttpGet("analytics/route/{routeId}")]
    public async Task<ActionResult<object>> GetRouteAnalytics(
        string routeId,
        [FromQuery] DateTime? startDate = null,
        [FromQuery] DateTime? endDate = null,
        [FromQuery] string? ownerId = null)
    {
        var result = await _clickStreamService.GetRouteAnalyticsAsync(routeId, startDate, endDate, ownerId);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        return Ok(result.Value);
    }

    // ============== New Statistics Endpoints using Materialized Views ==============

    /// <summary>
    /// Get daily statistics from materialized views
    /// </summary>
    [HttpGet("stats/daily")]
    public async Task<ActionResult<List<DailyStatsDto>>> GetDailyStats(
        [FromQuery] string? routeId = null,
        [FromQuery] string? fromDate = null,
        [FromQuery] string? toDate = null)
    {
        var userId = this.GetUserId();
        var result = await _clickStreamService.GetDailyStatsAsync(userId, routeId, fromDate, toDate);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        return Ok(result.Value);
    }

    /// <summary>
    /// Get hourly statistics from materialized views
    /// </summary>
    [HttpGet("stats/hourly")]
    public async Task<ActionResult<List<HourlyStatsDto>>> GetHourlyStats(
        [FromQuery] string? routeId = null,
        [FromQuery] string? fromHour = null,
        [FromQuery] string? toHour = null)
    {
        var userId = this.GetUserId();
        var result = await _clickStreamService.GetHourlyStatsAsync(userId, routeId, fromHour, toHour);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        return Ok(result.Value);
    }

    /// <summary>
    /// Get geographic statistics from materialized views
    /// </summary>
    [HttpGet("stats/geographic")]
    public async Task<ActionResult<List<GeographicStatsDto>>> GetGeographicStats(
        [FromQuery] string? routeId = null,
        [FromQuery] string? fromDate = null,
        [FromQuery] string? toDate = null)
    {
        var userId = this.GetUserId();
        var result = await _clickStreamService.GetGeographicStatsAsync(userId, routeId, fromDate, toDate);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        return Ok(result.Value);
    }

    /// <summary>
    /// Get device statistics from materialized views
    /// </summary>
    [HttpGet("stats/devices")]
    public async Task<ActionResult<List<DeviceStatsDto>>> GetDeviceStats(
        [FromQuery] string? routeId = null,
        [FromQuery] string? fromDate = null,
        [FromQuery] string? toDate = null)
    {
        var userId = this.GetUserId();
        var result = await _clickStreamService.GetDeviceStatsAsync(userId, routeId, fromDate, toDate);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        return Ok(result.Value);
    }

    /// <summary>
    /// Get browser statistics from materialized views
    /// </summary>
    [HttpGet("stats/browsers")]
    public async Task<ActionResult<List<BrowserStatsDto>>> GetBrowserStats(
        [FromQuery] string? routeId = null,
        [FromQuery] string? fromDate = null,
        [FromQuery] string? toDate = null)
    {
        var userId = this.GetUserId();
        var result = await _clickStreamService.GetBrowserStatsAsync(userId, routeId, fromDate, toDate);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        return Ok(result.Value);
    }

    /// <summary>
    /// Get route performance statistics from materialized views
    /// </summary>
    [HttpGet("stats/route-performance")]
    public async Task<ActionResult<List<RoutePerformanceDto>>> GetRoutePerformance(
        [FromQuery] string? fromDate = null,
        [FromQuery] string? toDate = null,
        [FromQuery] int? limit = null)
    {
        var userId = this.GetUserId();
        var result = await _clickStreamService.GetRoutePerformanceAsync(userId, fromDate, toDate, limit);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        return Ok(result.Value);
    }

    /// <summary>
    /// Get top destinations from materialized views
    /// </summary>
    [HttpGet("stats/top-destinations")]
    public async Task<ActionResult<List<TopDestinationDto>>> GetTopDestinations(
        [FromQuery] string? routeId = null,
        [FromQuery] string? fromDate = null,
        [FromQuery] string? toDate = null,
        [FromQuery] int? limit = null)
    {
        var userId = this.GetUserId();
        var result = await _clickStreamService.GetTopDestinationsAsync(userId, routeId, fromDate, toDate, limit);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        return Ok(result.Value);
    }

    /// <summary>
    /// Get traffic type statistics (bot vs human) from materialized views
    /// </summary>
    [HttpGet("stats/traffic-types")]
    public async Task<ActionResult<List<TrafficTypeStatsDto>>> GetTrafficTypeStats(
        [FromQuery] string? routeId = null,
        [FromQuery] string? fromHour = null,
        [FromQuery] string? toHour = null)
    {
        var userId = this.GetUserId();
        var result = await _clickStreamService.GetTrafficTypeStatsAsync(userId, routeId, fromHour, toHour);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        return Ok(result.Value);
    }

    private ActionResult HandleError(string errorCode, string errorMessage)
    {
        return errorCode switch
        {
            "REQUIRED_FIELD" => BadRequest(new { error = errorCode, message = errorMessage }),
            "VALIDATION_ERROR" => BadRequest(new { error = errorCode, message = errorMessage }),
            "UNAUTHORIZED" => Unauthorized(new { error = errorCode, message = errorMessage }),
            "FORBIDDEN" => Forbid(),
            "NOT_FOUND" => NotFound(new { error = errorCode, message = errorMessage }),
            "CONFLICT" => Conflict(new { error = errorCode, message = errorMessage }),
            "BUSINESS_RULE_VIOLATION" => UnprocessableEntity(new { error = errorCode, message = errorMessage }),
            "RATE_LIMIT_EXCEEDED" => StatusCode(429, new { error = errorCode, message = errorMessage }),
            "BURST_LIMIT_EXCEEDED" => StatusCode(429, new { error = errorCode, message = errorMessage }),
            "TIMEOUT" => StatusCode(408, new { error = errorCode, message = errorMessage }),
            "CIRCUIT_BREAKER_OPEN" => StatusCode(503, new { error = errorCode, message = errorMessage }),
            "EXTERNAL_SERVICE_ERROR" => StatusCode(502, new { error = errorCode, message = errorMessage }),
            "NETWORK_ERROR" => StatusCode(502, new { error = errorCode, message = errorMessage }),
            "INTERNAL_ERROR" => StatusCode(500, new { error = errorCode, message = errorMessage }),
            _ => StatusCode(500, new { error = "UNKNOWN_ERROR", message = "An unknown error occurred" })
        };
    }
}
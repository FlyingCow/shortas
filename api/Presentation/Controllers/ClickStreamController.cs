using Microsoft.AspNetCore.Authorization;
using Microsoft.AspNetCore.Mvc;
using ShortasProxyApi.Application.DTOs;
using ShortasProxyApi.Domain.Entities;
using ShortasProxyApi.Domain.Interfaces;
using ShortasProxyApi.Domain.Common;

namespace ShortasProxyApi.Presentation.Controllers;

[ApiController]
[Route("api/v1/clickstream")]
[Authorize]
public class ClickStreamController : ControllerBase
{
    private readonly IClickStreamService _clickStreamService;
    private readonly ILogger<ClickStreamController> _logger;

    public ClickStreamController(IClickStreamService clickStreamService, ILogger<ClickStreamController> logger)
    {
        _clickStreamService = clickStreamService;
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

        var clickStreamDtos = result.Value.Select(MapToDto).ToList();
        return Ok(clickStreamDtos);
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

        var clickStreamDtos = result.Value.Select(MapToDto).ToList();
        return Ok(clickStreamDtos);
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

    private static ClickStreamDto MapToDto(ClickStream clickStream)
    {
        return new ClickStreamDto
        {
            Id = clickStream.ExternalId,
            OwnerId = clickStream.OwnerId,
            CreatorId = clickStream.CreatorId,
            RouteId = clickStream.RouteId,
            WorkspaceId = clickStream.WorkspaceId,
            Created = clickStream.Created,
            Dest = clickStream.Dest,
            Ip = clickStream.Ip,
            Continent = clickStream.Continent,
            Country = clickStream.Country,
            Location = clickStream.Location,
            OsFamily = clickStream.OsFamily,
            OsVersion = clickStream.OsVersion,
            UserAgentFamily = clickStream.UserAgentFamily,
            UserAgentVersion = clickStream.UserAgentVersion,
            DeviceBrand = clickStream.DeviceBrand,
            DeviceFamily = clickStream.DeviceFamily,
            DeviceModel = clickStream.DeviceModel,
            SessionFirst = clickStream.SessionFirst,
            SessionClicks = clickStream.SessionClicks,
            IsUnique = clickStream.IsUnique,
            IsBot = clickStream.IsBot
        };
    }
}
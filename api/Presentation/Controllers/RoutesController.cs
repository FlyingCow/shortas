using Microsoft.AspNetCore.Authorization;
using Microsoft.AspNetCore.Mvc;
using ShortasProxyApi.Application.DTOs;
using ShortasProxyApi.Domain.Entities;
using ShortasProxyApi.Domain.Interfaces;
using ShortasProxyApi.Domain.Common;
using ShortasProxyApi.Presentation.Extensions;
using System.Text.Json;

namespace ShortasProxyApi.Presentation.Controllers;

[ApiController]
[Route("api/v1/routes")]
[Authorize]
public class RoutesController : ControllerBase
{
    private readonly IRouteService _routeService;
    private readonly ILogger<RoutesController> _logger;
    private readonly JsonSerializerOptions _jsonOptions;

    public RoutesController(IRouteService routeService, ILogger<RoutesController> logger)
    {
        _routeService = routeService;
        _logger = logger;
        _jsonOptions = new JsonSerializerOptions
        {
            PropertyNamingPolicy = JsonNamingPolicy.CamelCase,
            PropertyNameCaseInsensitive = true
        };
    }

    /// <summary>
    /// List all routes with pagination and filtering
    /// </summary>
    /// <param name="page">Page number (default: 1)</param>
    /// <param name="pageSize">Page size (default: 20)</param>
    /// <param name="search">Search term for link, dest, or switch</param>
    /// <param name="status">Filter by status</param>
    /// <param name="workspaceId">Filter by workspace ID</param>
    /// <returns>Paginated list of routes</returns>
    [HttpGet]
    public async Task<ActionResult<object>> ListRoutes(
        [FromQuery] int page = 1,
        [FromQuery] int pageSize = 20,
        [FromQuery] string? search = null,
        [FromQuery] string? status = null,
        [FromQuery] string? workspaceId = null)
    {
        var userId = this.GetUserId();
        var result = await _routeService.ListRoutesAsync(page, pageSize, search, status, userId, workspaceId);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        var (routes, totalCount) = result.Value;
        var routeDtos = routes.Select(MapToDto).ToList();

        return Ok(new
        {
            data = routeDtos,
            pagination = new
            {
                page,
                pageSize,
                totalCount,
                totalPages = (int)Math.Ceiling(totalCount / (double)pageSize)
            }
        });
    }

    /// <summary>
    /// Get route information by ID
    /// </summary>
    /// <param name="id">Route ID</param>
    /// <returns>Route information</returns>
    [HttpGet("{id}")]
    public async Task<ActionResult<RouteDto>> GetRoute(string id)
    {
        if (!Guid.TryParse(id, out var routeId))
        {
            return BadRequest(new { error = "VALIDATION_ERROR", message = "Invalid route ID format" });
        }

        var userId = this.GetUserId();
        var result = await _routeService.GetRouteByIdAsync(routeId, userId);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        if (result.Value == null)
            return NotFound();

        var routeDto = MapToDto(result.Value);
        return Ok(routeDto);
    }

    /// <summary>
    /// Create a new route
    /// </summary>
    /// <param name="routeDto">Route data</param>
    /// <returns>Created route</returns>
    [HttpPost]
    public async Task<ActionResult<RouteDto>> CreateRoute([FromBody] RouteDto routeDto)
    {
        var userId = this.GetUserId();
        var route = MapFromDto(routeDto);

        // Ensure Properties exists and set OwnerId
        if (route.Properties == null)
        {
            route.Properties = new Domain.Entities.RouteProperties();
        }
        route.Properties.OwnerId = userId;
        route.Properties.CreatorId = userId;

        // Validate that WorkspaceId is provided (mandatory field)
        if (string.IsNullOrWhiteSpace(route.Properties.WorkspaceId))
        {
            return BadRequest(new { error = "VALIDATION_ERROR", message = "Workspace is required when creating a route" });
        }

        var result = await _routeService.CreateRouteAsync(route);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        var createdRouteDto = MapToDto(result.Value);
        return CreatedAtAction(nameof(GetRoute), new { id = result.Value.Id.ToString() }, createdRouteDto);
    }

    /// <summary>
    /// Update an existing route by ID
    /// </summary>
    /// <param name="id">Route ID</param>
    /// <param name="routeDto">Updated route data</param>
    /// <returns>Updated route</returns>
    [HttpPut("{id}")]
    public async Task<ActionResult<RouteDto>> UpdateRoute(string id, [FromBody] RouteDto routeDto)
    {
        if (!Guid.TryParse(id, out var routeId))
        {
            return BadRequest(new { error = "VALIDATION_ERROR", message = "Invalid route ID format" });
        }

        var userId = this.GetUserId();

        // Fetch existing route to validate workspace immutability
        var existingRouteResult = await _routeService.GetRouteByIdAsync(routeId, userId);
        if (existingRouteResult.IsFailure)
        {
            return HandleError(existingRouteResult.ErrorCode ?? "UNKNOWN_ERROR", existingRouteResult.Error);
        }

        if (existingRouteResult.Value == null)
        {
            return NotFound(new { error = "NOT_FOUND", message = "Route not found" });
        }

        // Validate that WorkspaceId has not been changed
        var existingWorkspaceId = existingRouteResult.Value.Properties?.WorkspaceId;
        var newWorkspaceId = routeDto.Properties?.WorkspaceId;

        if (!string.IsNullOrWhiteSpace(existingWorkspaceId) && existingWorkspaceId != newWorkspaceId)
        {
            return BadRequest(new { error = "VALIDATION_ERROR", message = "Workspace cannot be changed after route creation" });
        }

        var route = MapFromDto(routeDto);
        var result = await _routeService.UpdateRouteByIdAsync(routeId, userId, route);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        var updatedRouteDto = MapToDto(result.Value);
        return Ok(updatedRouteDto);
    }

    /// <summary>
    /// Delete a route by ID
    /// </summary>
    /// <param name="id">Route ID</param>
    /// <returns>No content</returns>
    [HttpDelete("{id}")]
    public async Task<IActionResult> DeleteRoute(string id)
    {
        if (!Guid.TryParse(id, out var routeId))
        {
            return BadRequest(new { error = "VALIDATION_ERROR", message = "Invalid route ID format" });
        }

        var userId = this.GetUserId();
        var result = await _routeService.DeleteRouteByIdAsync(routeId, userId);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        return NoContent();
    }

    /// <summary>
    /// Bulk create routes
    /// </summary>
    /// <param name="routesDto">List of routes to create</param>
    /// <returns>Created routes</returns>
    [HttpPost("bulk")]
    public async Task<ActionResult<List<RouteDto>>> BulkCreateRoutes([FromBody] List<RouteDto> routesDto)
    {
        var userId = this.GetUserId();
        var routes = routesDto.Select(dto =>
        {
            var route = MapFromDto(dto);
            // Ensure Properties exists and set OwnerId
            if (route.Properties == null)
            {
                route.Properties = new Domain.Entities.RouteProperties();
            }
            route.Properties.OwnerId = userId;
            route.Properties.CreatorId = userId;
            return route;
        }).ToList();

        // Validate that all routes have WorkspaceId
        var routesWithoutWorkspace = routes.Where(r => string.IsNullOrWhiteSpace(r.Properties?.WorkspaceId)).ToList();
        if (routesWithoutWorkspace.Any())
        {
            return BadRequest(new { error = "VALIDATION_ERROR", message = "All routes must have a workspace specified" });
        }

        var result = await _routeService.BulkCreateRoutesAsync(routes);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        var createdRoutesDto = result.Value.Select(MapToDto).ToList();
        return Ok(createdRoutesDto);
    }

    /// <summary>
    /// Bulk update routes
    /// </summary>
    /// <param name="routesDto">List of routes to update</param>
    /// <returns>Updated routes</returns>
    [HttpPut("bulk")]
    public async Task<ActionResult<List<RouteDto>>> BulkUpdateRoutes([FromBody] List<RouteDto> routesDto)
    {
        var userId = this.GetUserId();
        var routes = routesDto.Select(MapFromDto).ToList();
        var result = await _routeService.BulkUpdateRoutesAsync(userId, routes);
        
        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        var updatedRoutesDto = result.Value.Select(MapToDto).ToList();
        return Ok(updatedRoutesDto);
    }

    /// <summary>
    /// Bulk delete routes
    /// </summary>
    /// <param name="routeIds">List of route IDs to delete</param>
    /// <returns>No content</returns>
    [HttpDelete("bulk")]
    public async Task<IActionResult> BulkDeleteRoutes([FromBody] List<string> routeIds)
    {
        var userId = this.GetUserId();
        var result = await _routeService.BulkDeleteRoutesAsync(userId, routeIds);
        
        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        return NoContent();
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

    private static RouteDto MapToDto(Domain.Entities.Route route)
    {
        return new RouteDto
        {
            Id = route.Id.ToString(),  // Include internal ID
            Switch = route.Switch,
            Link = route.Link,
            Dest = route.Dest,
            DestFormat = route.DestFormat,
            Code = route.Code,
            Ttl = route.Ttl,
            Status = route.Status,
            Terminal = route.Terminal,
            Policy = route.Policy,  // Include routing policy
            DomainId = route.DomainId,
            Domain = route.Domain != null ? new DomainDto
            {
                Id = route.Domain.Id,
                Name = route.Domain.Name
            } : null,
            Properties = route.Properties != null ? new RoutePropertiesDto
            {
                RouteId = route.Properties.RouteId,
                DomainId = route.Properties.DomainId,
                OwnerId = route.Properties.OwnerId,
                CreatorId = route.Properties.CreatorId,
                WorkspaceId = route.Properties.WorkspaceId,
                Scripts = route.Properties.Scripts,
                Tags = route.Properties.Tags,
                Custom = route.Properties.Custom,
                Native = route.Properties.Native,
                Bundling = route.Properties.Bundling,
                Opengraph = route.Properties.Opengraph,
                AllowDebug = route.Properties.AllowDebug
            } : null
        };
    }

    private static Domain.Entities.Route MapFromDto(RouteDto routeDto)
    {
        var route = new Domain.Entities.Route
        {
            Switch = routeDto.Switch,
            Link = routeDto.Link,
            Dest = routeDto.Dest,
            DestFormat = routeDto.DestFormat,
            Code = routeDto.Code,
            Ttl = routeDto.Ttl,
            Status = routeDto.Status,
            Terminal = routeDto.Terminal,
            DomainId = routeDto.DomainId
        };

        // Set policy if provided, otherwise default to Basic
        if (routeDto.Policy != null)
        {
            route.Policy = routeDto.Policy;
        }

        // Properties is required, always initialize with new fields
        if (routeDto.Properties != null)
        {
            route.Properties.RouteId = route.Id.ToString();
            route.Properties.DomainId = routeDto.Properties.DomainId;
            route.Properties.OwnerId = routeDto.Properties.OwnerId;
            route.Properties.CreatorId = routeDto.Properties.CreatorId;
            route.Properties.WorkspaceId = routeDto.Properties.WorkspaceId;
            route.Properties.Scripts = routeDto.Properties.Scripts;
            route.Properties.Tags = routeDto.Properties.Tags;
            route.Properties.Custom = routeDto.Properties.Custom;
            route.Properties.Native = routeDto.Properties.Native;
            route.Properties.Bundling = routeDto.Properties.Bundling;
            route.Properties.Opengraph = routeDto.Properties.Opengraph;
            route.Properties.AllowDebug = routeDto.Properties.AllowDebug;
        }

        return route;
    }
}
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
    private readonly IRouteSearchService _routeSearchService;
    private readonly ISlashTagGenerator _slashTagGenerator;
    private readonly ILogger<RoutesController> _logger;
    private readonly JsonSerializerOptions _jsonOptions;

    public RoutesController(IRouteService routeService, IRouteSearchService routeSearchService, ISlashTagGenerator slashTagGenerator, ILogger<RoutesController> logger)
    {
        _routeService = routeService;
        _routeSearchService = routeSearchService;
        _slashTagGenerator = slashTagGenerator;
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

        var route = result.Value;
        var routeDto = MapToDto(route);

        // If this is a master route with conditional policy, populate conditions from child routes
        if (route.Switch == "main" && route.Policy is ConditionalPolicy conditionalPolicy)
        {
            routeDto.Conditions = await GetConditionsFromChildRoutes(route.Link, conditionalPolicy, userId);
        }

        return Ok(routeDto);
    }

    /// <summary>
    /// Get conditions by loading destinations from child routes
    /// </summary>
    private async Task<List<ConditionDestinationDto>> GetConditionsFromChildRoutes(
        string link,
        ConditionalPolicy conditionalPolicy,
        string userId)
    {
        var conditions = new List<ConditionDestinationDto>();

        // Get all routes with the same link
        var listResult = await _routeService.ListRoutesAsync(
            page: 1,
            pageSize: 1000,
            search: link,
            ownerId: userId);

        if (!listResult.IsSuccess)
        {
            return conditions;
        }

        var childRoutes = listResult.Value.Routes
            .Where(r => r.Link == link && r.Switch != "main")
            .ToDictionary(r => r.Switch, r => r);

        // Match conditions from policy with child route destinations
        foreach (var routing in conditionalPolicy.Conditions)
        {
            var dest = "";
            if (childRoutes.TryGetValue(routing.Key, out var childRoute))
            {
                dest = childRoute.Dest ?? "";
            }

            conditions.Add(new ConditionDestinationDto
            {
                Dest = dest,
                Condition = routing.Condition
            });
        }

        return conditions;
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

        // Check if conditions are provided - use route family pattern
        var hasConditions = routeDto.Conditions != null && routeDto.Conditions.Count > 0;

        if (hasConditions)
        {
            return await CreateRouteWithConditions(routeDto, userId);
        }

        // No conditions - create single Basic route
        var route = MapFromDto(routeDto);
        route.Switch = "main";  // Always set switch to 'main'
        route.Policy = new BasicPolicy();

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
    /// Create a route with conditions (master + child routes pattern)
    /// </summary>
    private async Task<ActionResult<RouteDto>> CreateRouteWithConditions(RouteDto routeDto, string userId)
    {
        var conditions = routeDto.Conditions!;
        var routesToCreate = new List<Domain.Entities.Route>();

        // Create master route with Conditional policy
        var masterRoute = MapFromDto(routeDto);
        masterRoute.Switch = "main";
        masterRoute.Policy = GenerateConditionalPolicy(conditions);

        // Ensure Properties exists and set OwnerId
        if (masterRoute.Properties == null)
        {
            masterRoute.Properties = new Domain.Entities.RouteProperties();
        }
        masterRoute.Properties.OwnerId = userId;
        masterRoute.Properties.CreatorId = userId;

        // Validate that WorkspaceId is provided (mandatory field)
        if (string.IsNullOrWhiteSpace(masterRoute.Properties.WorkspaceId))
        {
            return BadRequest(new { error = "VALIDATION_ERROR", message = "Workspace is required when creating a route" });
        }

        routesToCreate.Add(masterRoute);

        // Create child routes for each condition
        for (int i = 0; i < conditions.Count; i++)
        {
            var condition = conditions[i];
            var childRoute = new Domain.Entities.Route
            {
                Id = Guid.NewGuid(),
                Switch = GenerateChildSwitch(i),
                Link = routeDto.Link,
                Dest = condition.Dest,
                DestFormat = routeDto.DestFormat,
                Code = routeDto.Code,
                Ttl = routeDto.Ttl,
                Status = routeDto.Status,
                Terminal = routeDto.Terminal,
                Policy = new BasicPolicy(),
                DomainId = routeDto.DomainId,
                Properties = new Domain.Entities.RouteProperties
                {
                    DomainId = masterRoute.Properties?.DomainId,
                    OwnerId = userId,
                    CreatorId = userId,
                    WorkspaceId = masterRoute.Properties?.WorkspaceId,
                    Scripts = masterRoute.Properties?.Scripts,
                    Tags = masterRoute.Properties?.Tags,
                    Custom = masterRoute.Properties?.Custom,
                    Native = masterRoute.Properties?.Native,
                    Bundling = masterRoute.Properties?.Bundling,
                    Opengraph = masterRoute.Properties?.Opengraph ?? false,
                    AllowDebug = masterRoute.Properties?.AllowDebug ?? false
                }
            };
            routesToCreate.Add(childRoute);
        }

        var result = await _routeService.BulkCreateRoutesAsync(routesToCreate);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        // Return the master route
        var masterResult = result.Value.FirstOrDefault(r => r.Switch == "main");
        if (masterResult == null)
        {
            return StatusCode(500, new { error = "INTERNAL_ERROR", message = "Failed to retrieve master route after creation" });
        }

        var createdRouteDto = MapToDto(masterResult);
        // Include conditions in the response
        createdRouteDto.Conditions = conditions;
        return CreatedAtAction(nameof(GetRoute), new { id = masterResult.Id.ToString() }, createdRouteDto);
    }

    /// <summary>
    /// Generate switch name for child route (1-indexed: cond-1, cond-2, etc.)
    /// </summary>
    private static string GenerateChildSwitch(int index) => $"cond-{index + 1}";

    /// <summary>
    /// Generate conditional policy from conditions list
    /// </summary>
    private static ConditionalPolicy GenerateConditionalPolicy(List<ConditionDestinationDto> conditions)
    {
        var policy = new ConditionalPolicy();
        for (int i = 0; i < conditions.Count; i++)
        {
            policy.Conditions.Add(new ConditionalRouting
            {
                Key = GenerateChildSwitch(i),
                Condition = conditions[i].Condition
            });
        }
        return policy;
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

        var existingRoute = existingRouteResult.Value;

        // Validate that Link has not been changed (immutable after creation)
        var existingLink = existingRoute.Link;
        if (!string.IsNullOrWhiteSpace(routeDto.Link) && routeDto.Link != existingLink)
        {
            return BadRequest(new { error = "VALIDATION_ERROR", message = "Route link cannot be changed after creation" });
        }

        // Validate that Domain has not been changed (immutable after creation)
        var existingDomainId = existingRoute.DomainId;
        if (routeDto.DomainId.HasValue && existingDomainId.HasValue && routeDto.DomainId.Value != existingDomainId.Value)
        {
            return BadRequest(new { error = "VALIDATION_ERROR", message = "Route domain cannot be changed after creation" });
        }

        // Validate that WorkspaceId has not been changed
        var existingWorkspaceId = existingRoute.Properties?.WorkspaceId;
        var newWorkspaceId = routeDto.Properties?.WorkspaceId;

        if (!string.IsNullOrWhiteSpace(existingWorkspaceId) && existingWorkspaceId != newWorkspaceId)
        {
            return BadRequest(new { error = "VALIDATION_ERROR", message = "Workspace cannot be changed after route creation" });
        }

        // Only master routes can be updated with conditions
        if (existingRoute.Switch != "main")
        {
            return BadRequest(new { error = "VALIDATION_ERROR", message = "Only master routes (switch='main') can be updated. Update the master route instead." });
        }

        // Handle conditions update
        var hasConditions = routeDto.Conditions != null && routeDto.Conditions.Count > 0;

        if (hasConditions)
        {
            return await UpdateRouteWithConditions(routeId, routeDto, existingRoute, userId);
        }

        // No conditions - update as single Basic route
        var route = MapFromDto(routeDto);
        route.Switch = "main";
        route.Policy = new BasicPolicy();

        var result = await _routeService.UpdateRouteByIdAsync(routeId, userId, route);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        // Delete any existing child routes since we're now a Basic route
        await DeleteChildRoutes(existingLink, userId);

        var updatedRouteDto = MapToDto(result.Value);
        return Ok(updatedRouteDto);
    }

    /// <summary>
    /// Update a route with conditions (master + child routes pattern)
    /// </summary>
    private async Task<ActionResult<RouteDto>> UpdateRouteWithConditions(
        Guid routeId,
        RouteDto routeDto,
        Domain.Entities.Route existingRoute,
        string userId)
    {
        var conditions = routeDto.Conditions!;
        var existingLink = existingRoute.Link;

        // First, delete all existing child routes
        await DeleteChildRoutes(existingLink, userId);

        // Update master route with new conditional policy
        var masterRoute = MapFromDto(routeDto);
        masterRoute.Switch = "main";
        masterRoute.Policy = GenerateConditionalPolicy(conditions);

        var masterResult = await _routeService.UpdateRouteByIdAsync(routeId, userId, masterRoute);

        if (masterResult.IsFailure)
        {
            return HandleError(masterResult.ErrorCode ?? "UNKNOWN_ERROR", masterResult.Error);
        }

        // Create new child routes
        var childRoutes = new List<Domain.Entities.Route>();
        for (int i = 0; i < conditions.Count; i++)
        {
            var condition = conditions[i];
            var childRoute = new Domain.Entities.Route
            {
                Id = Guid.NewGuid(),
                Switch = GenerateChildSwitch(i),
                Link = existingLink,
                Dest = condition.Dest,
                DestFormat = existingRoute.DestFormat,
                Code = existingRoute.Code,
                Ttl = existingRoute.Ttl,
                Status = existingRoute.Status,
                Terminal = existingRoute.Terminal,
                Policy = new BasicPolicy(),
                DomainId = existingRoute.DomainId,
                Properties = new Domain.Entities.RouteProperties
                {
                    DomainId = existingRoute.Properties?.DomainId,
                    OwnerId = userId,
                    CreatorId = userId,
                    WorkspaceId = existingRoute.Properties?.WorkspaceId,
                    Scripts = existingRoute.Properties?.Scripts,
                    Tags = existingRoute.Properties?.Tags,
                    Custom = existingRoute.Properties?.Custom,
                    Native = existingRoute.Properties?.Native,
                    Bundling = existingRoute.Properties?.Bundling,
                    Opengraph = existingRoute.Properties?.Opengraph ?? false,
                    AllowDebug = existingRoute.Properties?.AllowDebug ?? false
                }
            };
            childRoutes.Add(childRoute);
        }

        if (childRoutes.Count > 0)
        {
            var bulkResult = await _routeService.BulkCreateRoutesAsync(childRoutes);
            if (bulkResult.IsFailure)
            {
                return HandleError(bulkResult.ErrorCode ?? "UNKNOWN_ERROR", bulkResult.Error);
            }
        }

        var updatedRouteDto = MapToDto(masterResult.Value);
        updatedRouteDto.Conditions = conditions;
        return Ok(updatedRouteDto);
    }

    /// <summary>
    /// Delete all child routes for a given link
    /// </summary>
    private async Task DeleteChildRoutes(string link, string userId)
    {
        // Get all routes with the same link that are not the master route
        var listResult = await _routeService.ListRoutesAsync(
            page: 1,
            pageSize: 1000,
            search: link,
            ownerId: userId);

        if (listResult.IsSuccess)
        {
            var childRouteIds = listResult.Value.Routes
                .Where(r => r.Link == link && r.Switch != "main")
                .Select(r => r.Id.ToString())
                .ToList();

            if (childRouteIds.Count > 0)
            {
                await _routeService.BulkDeleteRoutesAsync(userId, childRouteIds);
            }
        }
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

        // Fetch the route to check if it's a master route
        var existingRouteResult = await _routeService.GetRouteByIdAsync(routeId, userId);
        if (existingRouteResult.IsFailure)
        {
            return HandleError(existingRouteResult.ErrorCode ?? "UNKNOWN_ERROR", existingRouteResult.Error);
        }

        if (existingRouteResult.Value == null)
        {
            return NotFound(new { error = "NOT_FOUND", message = "Route not found" });
        }

        var existingRoute = existingRouteResult.Value;

        // If this is a master route, delete all child routes first
        if (existingRoute.Switch == "main")
        {
            await DeleteChildRoutes(existingRoute.Link, userId);
        }

        // Delete the route
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

    /// <summary>
    /// Suggest a unique slash tag (short link path) for a given domain.
    /// Uses a probabilistic approach starting with the shortest possible length (3 chars).
    /// </summary>
    /// <param name="domainId">The domain ID to generate a unique tag for</param>
    /// <returns>A suggested unique slash tag</returns>
    [HttpGet("suggest-link")]
    public async Task<ActionResult<object>> SuggestLink([FromQuery] Guid domainId)
    {
        if (domainId == Guid.Empty)
        {
            return BadRequest(new { error = "VALIDATION_ERROR", message = "domainId is required" });
        }

        var result = await _slashTagGenerator.GenerateAsync(domainId);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        return Ok(new { link = result.Value });
    }

    /// <summary>
    /// Full-text search routes by link, domain name, or destination URL
    /// </summary>
    /// <param name="q">Search query</param>
    /// <param name="page">Page number (default: 1)</param>
    /// <param name="pageSize">Page size (default: 20)</param>
    /// <param name="workspaceId">Filter by workspace ID (optional)</param>
    /// <returns>Search results with pagination</returns>
    [HttpGet("search")]
    public async Task<ActionResult<object>> SearchRoutes(
        [FromQuery] string q,
        [FromQuery] int page = 1,
        [FromQuery] int pageSize = 20,
        [FromQuery] string? workspaceId = null)
    {
        if (string.IsNullOrWhiteSpace(q))
        {
            return BadRequest(new { error = "VALIDATION_ERROR", message = "Search query 'q' is required" });
        }

        var userId = this.GetUserId();
        var result = await _routeSearchService.SearchAsync(q, userId, workspaceId, page, pageSize);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        var (results, totalCount) = result.Value;

        return Ok(new
        {
            data = results,
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
    /// Reindex all routes in the search index.
    /// Useful after initial Elasticsearch deployment or index corruption.
    /// </summary>
    [HttpPost("search/reindex")]
    public async Task<ActionResult<object>> ReindexRoutes()
    {
        var userId = this.GetUserId();

        // Fetch all routes for the user from the database
        var listResult = await _routeService.ListRoutesAsync(page: 1, pageSize: 10000, ownerId: userId);

        if (listResult.IsFailure)
        {
            return HandleError(listResult.ErrorCode ?? "UNKNOWN_ERROR", listResult.Error);
        }

        var (routes, totalCount) = listResult.Value;

        var searchDocuments = routes.Select(r => new RouteSearchDocument
        {
            Id = r.Id.ToString(),
            Link = r.Link,
            Switch = r.Switch,
            Dest = r.Dest,
            DomainName = r.Domain?.Name,
            Status = r.Status,
            OwnerId = r.Properties?.OwnerId,
            WorkspaceId = r.Properties?.WorkspaceId,
        }).ToList();

        var indexResult = await _routeSearchService.IndexRoutesAsync(searchDocuments);

        if (indexResult.IsFailure)
        {
            return HandleError(indexResult.ErrorCode ?? "UNKNOWN_ERROR", indexResult.Error);
        }

        return Ok(new { message = $"Reindexed {searchDocuments.Count} routes", count = searchDocuments.Count });
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
                Name = route.Domain.Name,
                OwnerId = route.Domain.OwnerId,
                VerificationStatus = route.Domain.VerificationStatus.ToString(),
                VerificationReason = route.Domain.VerificationReason,
                LastVerificationCheck = route.Domain.LastVerificationCheck,
                NextVerificationCheck = route.Domain.NextVerificationCheck
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
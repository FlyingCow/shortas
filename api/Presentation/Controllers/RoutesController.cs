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
    private readonly IObjectStorageService _objectStorageService;
    private readonly ILogger<RoutesController> _logger;
    private readonly JsonSerializerOptions _jsonOptions;

    public RoutesController(
        IRouteService routeService,
        IRouteSearchService routeSearchService,
        ISlashTagGenerator slashTagGenerator,
        IObjectStorageService objectStorageService,
        ILogger<RoutesController> logger)
    {
        _routeService = routeService;
        _routeSearchService = routeSearchService;
        _slashTagGenerator = slashTagGenerator;
        _objectStorageService = objectStorageService;
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

        // If this is a conditional route, extract conditions from the policy
        // (dest is stored inline in ConditionalRouting since management DB stores one route)
        if (route.Switch == "main" && route.Policy is ConditionalPolicy conditionalPolicy)
        {
            routeDto.Conditions = conditionalPolicy.Conditions.Select(c => new ConditionDestinationDto
            {
                Dest = c.Dest ?? "",
                Condition = c.Condition
            }).ToList();
        }

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
    /// Create a route with conditions.
    /// Stores ONE route in management DB with ConditionalPolicy (conditions+dests in policy JSON).
    /// The service layer builds the route family (master + children) for click-router.
    /// </summary>
    private async Task<ActionResult<RouteDto>> CreateRouteWithConditions(RouteDto routeDto, string userId)
    {
        var conditions = routeDto.Conditions!;

        // Create a single route with Conditional policy (includes dest per condition)
        var route = MapFromDto(routeDto);
        route.Switch = "main";
        route.Policy = GenerateConditionalPolicy(conditions);

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

        // CreateRouteAsync saves ONE route to DB, and the service handles
        // building the route family for click-router propagation.
        var result = await _routeService.CreateRouteAsync(route);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        var createdRouteDto = MapToDto(result.Value);
        createdRouteDto.Conditions = conditions;
        return CreatedAtAction(nameof(GetRoute), new { id = result.Value.Id.ToString() }, createdRouteDto);
    }

    /// <summary>
    /// Generate switch name for child route (1-indexed: cond-1, cond-2, etc.)
    /// </summary>
    private static string GenerateChildSwitch(int index) => $"cond-{index + 1}";

    /// <summary>
    /// Generate conditional policy from conditions list (includes dest for management DB storage)
    /// </summary>
    private static ConditionalPolicy GenerateConditionalPolicy(List<ConditionDestinationDto> conditions)
    {
        var policy = new ConditionalPolicy();
        for (int i = 0; i < conditions.Count; i++)
        {
            policy.Conditions.Add(new ConditionalRouting
            {
                Key = GenerateChildSwitch(i),
                Condition = conditions[i].Condition,
                Dest = conditions[i].Dest
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

        // UpdateRouteByIdAsync handles click-router propagation.
        // If the route was previously conditional, the service will clean up
        // old child routes on click-router.
        var result = await _routeService.UpdateRouteByIdAsync(routeId, userId, route);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        var updatedRouteDto = MapToDto(result.Value);
        return Ok(updatedRouteDto);
    }

    /// <summary>
    /// Update a route with conditions.
    /// Updates ONE route in management DB with new ConditionalPolicy (conditions+dests in policy JSON).
    /// The service layer rebuilds the route family on click-router.
    /// </summary>
    private async Task<ActionResult<RouteDto>> UpdateRouteWithConditions(
        Guid routeId,
        RouteDto routeDto,
        Domain.Entities.Route existingRoute,
        string userId)
    {
        var conditions = routeDto.Conditions!;

        // Update the single route with new conditional policy (includes dest per condition)
        var route = MapFromDto(routeDto);
        route.Switch = "main";
        route.Policy = GenerateConditionalPolicy(conditions);

        // UpdateRouteByIdAsync updates ONE route in DB, and the service handles
        // rebuilding the route family for click-router propagation.
        var result = await _routeService.UpdateRouteByIdAsync(routeId, userId, route);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        var updatedRouteDto = MapToDto(result.Value);
        updatedRouteDto.Conditions = conditions;
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

        // Delete the route (service handles click-router family cleanup for conditional routes)
        var result = await _routeService.DeleteRouteByIdAsync(routeId, userId);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        return NoContent();
    }

    /// <summary>
    /// Unblock a route that was blocked by Safe Browsing verification.
    /// Sets the route status back to "Active" for manual review scenarios.
    /// </summary>
    /// <param name="id">Route ID</param>
    /// <returns>Updated route</returns>
    [HttpPost("{id}/unblock")]
    public async Task<ActionResult<RouteDto>> UnblockRoute(string id)
    {
        if (!Guid.TryParse(id, out var routeId))
        {
            return BadRequest(new { error = "VALIDATION_ERROR", message = "Invalid route ID format" });
        }

        var userId = this.GetUserId();

        // Fetch existing route
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

        // Check if route is actually blocked
        if (!existingRoute.Status.StartsWith("Blocked", StringComparison.OrdinalIgnoreCase))
        {
            return BadRequest(new { error = "VALIDATION_ERROR", message = "Route is not blocked" });
        }

        // Update status to Active
        existingRoute.Status = "Active";

        var result = await _routeService.UpdateRouteByIdAsync(routeId, userId, existingRoute);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        _logger.LogInformation("Route {RouteId} unblocked by user {UserId}", routeId, userId);

        var updatedRouteDto = MapToDto(result.Value);
        return Ok(updatedRouteDto);
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

    /// <summary>
    /// Get QR code settings for a route
    /// </summary>
    /// <param name="id">Route ID</param>
    /// <returns>QR code settings</returns>
    [HttpGet("{id}/qr/settings")]
    public async Task<ActionResult<QrCodeSettingsDto>> GetQrSettings(string id)
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
        {
            return NotFound(new { error = "NOT_FOUND", message = "Route not found" });
        }

        var route = result.Value;
        var settings = new QrCodeSettingsDto();

        // Extract qrSettings from route.Properties.Custom if present
        if (route.Properties?.Custom != null &&
            route.Properties.Custom.TryGetValue("qrSettings", out var qrSettingsObj) &&
            qrSettingsObj != null)
        {
            try
            {
                var qrSettingsJson = JsonSerializer.Serialize(qrSettingsObj, _jsonOptions);
                var parsed = JsonSerializer.Deserialize<QrCodeSettingsDto>(qrSettingsJson, _jsonOptions);
                if (parsed != null)
                {
                    settings = parsed;
                }
            }
            catch (Exception ex)
            {
                _logger.LogWarning(ex, "Failed to parse QR settings for route {RouteId}", id);
            }
        }

        return Ok(settings);
    }

    /// <summary>
    /// Update QR code settings for a route
    /// </summary>
    /// <param name="id">Route ID</param>
    /// <param name="settings">QR code settings</param>
    /// <returns>Updated QR code settings</returns>
    [HttpPut("{id}/qr/settings")]
    public async Task<ActionResult<QrCodeSettingsDto>> UpdateQrSettings(string id, [FromBody] QrCodeSettingsDto settings)
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
        {
            return NotFound(new { error = "NOT_FOUND", message = "Route not found" });
        }

        var route = result.Value;

        // Initialize Properties if null
        if (route.Properties == null)
        {
            route.Properties = new Domain.Entities.RouteProperties();
        }

        // Store qrSettings in Custom (get, modify, set back due to property getter behavior)
        var custom = route.Properties.Custom ?? new Dictionary<string, object>();
        custom["qrSettings"] = settings;
        route.Properties.Custom = custom;

        // Update the route
        var updateResult = await _routeService.UpdateRouteByIdAsync(routeId, userId, route);

        if (updateResult.IsFailure)
        {
            return HandleError(updateResult.ErrorCode ?? "UNKNOWN_ERROR", updateResult.Error);
        }

        return Ok(settings);
    }

    /// <summary>
    /// Get a presigned URL for uploading the QR code SVG
    /// </summary>
    /// <param name="id">Route ID</param>
    /// <returns>Presigned upload URL</returns>
    [HttpPost("{id}/qr/upload-url")]
    public async Task<ActionResult<PresignedUrlResponseDto>> GetQrUploadUrl(string id)
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
        {
            return NotFound(new { error = "NOT_FOUND", message = "Route not found" });
        }

        var route = result.Value;
        var ownerId = route.Properties?.OwnerId ?? userId;
        var key = $"{ownerId}/{routeId}/qr.svg";

        var presignedUrl = await _objectStorageService.GeneratePresignedPutUrlAsync(key, "image/svg+xml", 15);

        return Ok(new PresignedUrlResponseDto
        {
            Url = presignedUrl,
            Key = key
        });
    }

    /// <summary>
    /// Get a presigned URL for uploading the QR code center logo
    /// </summary>
    /// <param name="id">Route ID</param>
    /// <param name="request">Upload request with content type</param>
    /// <returns>Presigned upload URL</returns>
    [HttpPost("{id}/qr/logo-upload-url")]
    public async Task<ActionResult<PresignedUrlResponseDto>> GetQrLogoUploadUrl(string id, [FromBody] PresignedUploadRequestDto? request)
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
        {
            return NotFound(new { error = "NOT_FOUND", message = "Route not found" });
        }

        var route = result.Value;
        var ownerId = route.Properties?.OwnerId ?? userId;
        var contentType = request?.ContentType ?? "image/png";

        // Determine file extension from content type
        var extension = contentType switch
        {
            "image/png" => "png",
            "image/jpeg" => "jpg",
            "image/jpg" => "jpg",
            "image/gif" => "gif",
            "image/webp" => "webp",
            "image/svg+xml" => "svg",
            _ => "png"
        };

        var key = $"{ownerId}/{routeId}/qr-logo.{extension}";

        var presignedUrl = await _objectStorageService.GeneratePresignedPutUrlAsync(key, contentType, 15);

        return Ok(new PresignedUrlResponseDto
        {
            Url = presignedUrl,
            Key = key
        });
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
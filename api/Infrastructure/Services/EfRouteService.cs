using Microsoft.EntityFrameworkCore;
using System.Text.Json;
using ShortasProxyApi.Domain.Common;
using ShortasProxyApi.Domain.Entities;
using ShortasProxyApi.Domain.Interfaces;
using ShortasProxyApi.Infrastructure.Data;
using ShortasProxyApi.Infrastructure.HttpClients;
using ShortasProxyApi.Application.Extensions;
using RouteEntity = ShortasProxyApi.Domain.Entities.Route;
using RouteSearchDoc = ShortasProxyApi.Domain.Interfaces.RouteSearchDocument;

namespace ShortasProxyApi.Infrastructure.Services;

public class EfRouteService : IRouteService
{
    private readonly ApplicationDbContext _context;
    private readonly ClickRouterApiClient _clickRouterApiClient;
    private readonly ILogger<EfRouteService> _logger;
    private readonly JsonSerializerOptions _jsonOptions;

    public EfRouteService(
        ApplicationDbContext context,
        ClickRouterApiClient clickRouterApiClient,
        ILogger<EfRouteService> logger)
    {
        _context = context;
        _clickRouterApiClient = clickRouterApiClient;
        _logger = logger;
        _jsonOptions = new JsonSerializerOptions
        {
            PropertyNamingPolicy = JsonNamingPolicy.CamelCase
        };
    }

    public async Task<Result<RouteEntity?>> GetRouteByIdAsync(Guid id, string userId)
    {
        try
        {
            if (string.IsNullOrWhiteSpace(userId))
                return Result<RouteEntity?>.Failure(Error.Required("userId"));

            var route = await _context.Routes
                .Include(r => r.Properties)
                .Include(r => r.Domain)
                .FirstOrDefaultAsync(r => r.Id == id &&
                                        r.Properties != null &&
                                        r.Properties.OwnerId == userId);

            if (route == null)
                return Result<RouteEntity?>.Failure(Error.NotFound("Route", id.ToString()));

            return Result<RouteEntity?>.Success(route);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error getting route by ID: {RouteId}", id);
            return Result<RouteEntity?>.Failure(Error.Internal("Failed to get route", ex.Message));
        }
    }

    public async Task<Result<RouteEntity?>> GetRouteAsync(string domain, string path, string userId, string? switchParam = null)
    {
        try
        {
            if (string.IsNullOrWhiteSpace(domain))
                return Result<RouteEntity?>.Failure(Error.Required("domain"));

            if (string.IsNullOrWhiteSpace(path))
                return Result<RouteEntity?>.Failure(Error.Required("path"));

            if (string.IsNullOrWhiteSpace(userId))
                return Result<RouteEntity?>.Failure(Error.Required("userId"));

            // Build link patterns to search for (both URL-encoded and literal formats)
            var encodedLink = $"{domain}%2F";
            var literalLink = $"{domain}/";

            var query = _context.Routes
                .Include(r => r.Properties)
                .Include(r => r.Domain)
                .Where(r => (r.Link.StartsWith(encodedLink) || r.Link.StartsWith(literalLink)) &&
                           r.Properties != null &&
                           r.Properties.OwnerId == userId);

            // Filter by switch if provided
            if (!string.IsNullOrWhiteSpace(switchParam))
            {
                query = query.Where(r => r.Switch == switchParam);
            }

            var route = await query.FirstOrDefaultAsync();

            return Result<RouteEntity?>.Success(route);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error getting route for domain: {Domain}, path: {Path}", domain, path);
            return Result<RouteEntity?>.Failure(Error.Internal("Failed to get route", ex.Message));
        }
    }

    public async Task<Result<RouteEntity>> CreateRouteAsync(RouteEntity route)
    {
        using var transaction = await _context.Database.BeginTransactionAsync();

        try
        {
            var validationResult = route.Validate();
            if (!validationResult.IsValid)
            {
                var errors = string.Join(", ", validationResult.Errors.Select(e => $"{e.FieldName}: {e.Message}"));
                return Result<RouteEntity>.Failure(Error.Validation("Route validation failed", errors));
            }

            // Validate domain is mandatory
            if (!route.DomainId.HasValue)
            {
                return Result<RouteEntity>.Failure(Error.Required("Domain is required for route creation"));
            }

            // Check if route already exists (unique by Link + Switch combination)
            var existing = await _context.Routes
                .FirstOrDefaultAsync(r => r.Link == route.Link && r.Switch == route.Switch);

            if (existing != null)
                return Result<RouteEntity>.Failure(Error.Conflict("Route with this link and switch already exists"));

            // Load domain from database and validate ownership
            route.Domain = await _context.RouteDomains
                .FirstOrDefaultAsync(d => d.Id == route.DomainId.Value);

            if (route.Domain == null)
            {
                return Result<RouteEntity>.Failure(Error.NotFound("Domain", route.DomainId.Value.ToString()));
            }

            // Verify domain belongs to current user
            if (route.Properties != null && !string.IsNullOrWhiteSpace(route.Properties.OwnerId))
            {
                if (route.Domain.OwnerId != route.Properties.OwnerId)
                {
                    return Result<RouteEntity>.Failure(Error.Forbidden("Domain does not belong to user"));
                }
            }

            // Add route to database
            await _context.Routes.AddAsync(route);
            await _context.SaveChangesAsync();

            // Update cached route count for the domain
            await IncrementRouteCountAsync(route.DomainId!.Value, 1);

            // Reload route with domain to ensure navigation property is populated
            var savedRoute = await _context.Routes
                .Include(r => r.Domain)
                .Include(r => r.Properties)
                .FirstOrDefaultAsync(r => r.Id == route.Id);

            if (savedRoute == null)
            {
                await transaction.RollbackAsync();
                return Result<RouteEntity>.Failure(Error.Internal("Failed to retrieve saved route"));
            }

            // Propagate to click-router API synchronously
            // Click-router-api handles route family creation atomically for conditional routes
            var apiDto = savedRoute.ToDto();
            var domainName = savedRoute.Domain?.Name ?? "";
            var apiResult = await _clickRouterApiClient.CreateRouteAsync(apiDto, domainName);

            if (apiResult.IsFailure)
            {
                await transaction.RollbackAsync();
                _logger.LogError("Failed to create route in click-router API: {Error}", apiResult.Error);
                return Result<RouteEntity>.Failure(apiResult.ErrorCode ?? "EXTERNAL_SERVICE_ERROR",
                    $"Failed to create route in click-router API: {apiResult.Error}");
            }

            // Enqueue search index update via outbox
            await EnqueueSearchIndexAsync(savedRoute);

            // Enqueue route verification via outbox
            await EnqueueRouteVerificationAsync(savedRoute);
            await _context.SaveChangesAsync();

            await transaction.CommitAsync();

            _logger.LogInformation("Route created: {RouteId}, Link: {Link}", route.Id, route.Link);

            return Result<RouteEntity>.Success(route);
        }
        catch (Exception ex)
        {
            await transaction.RollbackAsync();
            _logger.LogError(ex, "Error creating route");
            return Result<RouteEntity>.Failure(Error.Internal("Failed to create route", ex.Message));
        }
    }

    public async Task<Result<RouteEntity>> UpdateRouteByIdAsync(Guid id, string userId, RouteEntity route)
    {
        using var transaction = await _context.Database.BeginTransactionAsync();

        try
        {
            if (string.IsNullOrWhiteSpace(userId))
                return Result<RouteEntity>.Failure(Error.Required("userId"));

            var validationResult = route.Validate();
            if (!validationResult.IsValid)
            {
                var errors = string.Join(", ", validationResult.Errors.Select(e => $"{e.FieldName}: {e.Message}"));
                return Result<RouteEntity>.Failure(Error.Validation("Route validation failed", errors));
            }

            // Find existing route
            var existingRoute = await _context.Routes
                .Include(r => r.Properties)
                .Include(r => r.Domain)
                .FirstOrDefaultAsync(r => r.Id == id);

            if (existingRoute == null)
                return Result<RouteEntity>.Failure(Error.NotFound("Route", id.ToString()));

            // Verify ownership
            if (existingRoute.Properties == null || existingRoute.Properties.OwnerId != userId)
                return Result<RouteEntity>.Failure(Error.Forbidden());

            // Update route properties (Link, DomainId are immutable after creation)
            existingRoute.Switch = route.Switch;
            existingRoute.Dest = route.Dest;
            existingRoute.DestFormat = route.DestFormat;
            existingRoute.Code = route.Code;
            existingRoute.Ttl = route.Ttl;
            existingRoute.Status = route.Status;
            existingRoute.Terminal = route.Terminal;
            existingRoute.Policy = route.Policy;

            if (route.Properties != null)
            {
                if (existingRoute.Properties != null)
                {
                    existingRoute.Properties.RouteId = route.Properties.RouteId;
                    existingRoute.Properties.DomainId = route.Properties.DomainId;
                    existingRoute.Properties.CreatorId = route.Properties.CreatorId;
                    existingRoute.Properties.WorkspaceId = route.Properties.WorkspaceId;
                    existingRoute.Properties.Scripts = route.Properties.Scripts;
                    existingRoute.Properties.Tags = route.Properties.Tags;
                    existingRoute.Properties.Custom = route.Properties.Custom;
                    existingRoute.Properties.Native = route.Properties.Native;
                    existingRoute.Properties.Bundling = route.Properties.Bundling;
                    existingRoute.Properties.Opengraph = route.Properties.Opengraph;
                    existingRoute.Properties.AllowDebug = route.Properties.AllowDebug;
                }
                else
                {
                    existingRoute.Properties = route.Properties;
                }
            }

            // Apply updates to the single management DB route
            _context.Routes.Update(existingRoute);
            await _context.SaveChangesAsync();

            // Propagate to click-router API synchronously
            // Click-router-api handles route family creation/update atomically for conditional routes
            var apiDto = existingRoute.ToDto();
            var apiResult = await _clickRouterApiClient.UpdateRouteByIdAsync(id, userId, apiDto);

            if (apiResult.IsFailure)
            {
                await transaction.RollbackAsync();
                _logger.LogError("Failed to update route in click-router API: {Error}", apiResult.Error);
                return Result<RouteEntity>.Failure(apiResult.ErrorCode ?? "EXTERNAL_SERVICE_ERROR",
                    $"Failed to update route in click-router API: {apiResult.Error}");
            }

            // Enqueue search index update via outbox
            await EnqueueSearchIndexAsync(existingRoute);

            // Enqueue route verification via outbox (destination may have changed)
            await EnqueueRouteVerificationAsync(existingRoute);
            await _context.SaveChangesAsync();

            await transaction.CommitAsync();

            _logger.LogInformation("Route updated by ID: {RouteId}, Link: {Link}", existingRoute.Id, existingRoute.Link);

            return Result<RouteEntity>.Success(existingRoute);
        }
        catch (Exception ex)
        {
            await transaction.RollbackAsync();
            _logger.LogError(ex, "Error updating route by ID: {RouteId}", id);
            return Result<RouteEntity>.Failure(Error.Internal("Failed to update route", ex.Message));
        }
    }

    public async Task<Result<RouteEntity>> UpdateRouteAsync(string domain, string path, string userId, RouteEntity route)
    {
        using var transaction = await _context.Database.BeginTransactionAsync();

        try
        {
            if (string.IsNullOrWhiteSpace(domain))
                return Result<RouteEntity>.Failure(Error.Required("domain"));

            if (string.IsNullOrWhiteSpace(path))
                return Result<RouteEntity>.Failure(Error.Required("path"));

            if (string.IsNullOrWhiteSpace(userId))
                return Result<RouteEntity>.Failure(Error.Required("userId"));

            var validationResult = route.Validate();
            if (!validationResult.IsValid)
            {
                var errors = string.Join(", ", validationResult.Errors.Select(e => $"{e.FieldName}: {e.Message}"));
                return Result<RouteEntity>.Failure(Error.Validation("Route validation failed", errors));
            }

            // Find existing route
            var linkPattern = $"{domain}/{path}";
            var existingRoute = await _context.Routes
                .Include(r => r.Properties)
                .Include(r => r.Domain)
                .FirstOrDefaultAsync(r => r.Link.Contains(linkPattern));

            if (existingRoute == null)
                return Result<RouteEntity>.Failure(Error.NotFound("Route", $"{domain}/{path}"));

            // Verify ownership
            if (existingRoute.Properties == null || existingRoute.Properties.OwnerId != userId)
                return Result<RouteEntity>.Failure(Error.Forbidden());

            // Update route properties
            existingRoute.Switch = route.Switch;
            existingRoute.Dest = route.Dest;
            existingRoute.DestFormat = route.DestFormat;
            existingRoute.Code = route.Code;
            existingRoute.Ttl = route.Ttl;
            existingRoute.Status = route.Status;
            existingRoute.Terminal = route.Terminal;

            if (route.Properties != null)
            {
                if (existingRoute.Properties != null)
                {
                    existingRoute.Properties.DomainId = route.Properties.DomainId;
                    existingRoute.Properties.OwnerId = route.Properties.OwnerId;
                    existingRoute.Properties.CreatorId = route.Properties.CreatorId;
                    existingRoute.Properties.WorkspaceId = route.Properties.WorkspaceId;
                    existingRoute.Properties.Scripts = route.Properties.Scripts;
                    existingRoute.Properties.Tags = route.Properties.Tags;
                    existingRoute.Properties.Custom = route.Properties.Custom;
                    existingRoute.Properties.Native = route.Properties.Native;
                    existingRoute.Properties.Bundling = route.Properties.Bundling;
                    existingRoute.Properties.Opengraph = route.Properties.Opengraph;
                    existingRoute.Properties.AllowDebug = route.Properties.AllowDebug;
                }
                else
                {
                    existingRoute.Properties = route.Properties;
                }
            }

            _context.Routes.Update(existingRoute);
            await _context.SaveChangesAsync();

            // Propagate to click-router API synchronously
            var apiDto = existingRoute.ToDto();
            var apiResult = await _clickRouterApiClient.UpdateRouteAsync(domain, path, userId, apiDto);

            if (apiResult.IsFailure)
            {
                await transaction.RollbackAsync();
                _logger.LogError("Failed to update route in click-router API: {Error}", apiResult.Error);
                return Result<RouteEntity>.Failure(apiResult.ErrorCode ?? "EXTERNAL_SERVICE_ERROR",
                    $"Failed to update route in click-router API: {apiResult.Error}");
            }

            // Enqueue search index update via outbox
            await EnqueueSearchIndexAsync(existingRoute);

            // Enqueue route verification via outbox (destination may have changed)
            await EnqueueRouteVerificationAsync(existingRoute);
            await _context.SaveChangesAsync();

            await transaction.CommitAsync();

            _logger.LogInformation("Route updated: {RouteId}, Link: {Link}", existingRoute.Id, existingRoute.Link);

            return Result<RouteEntity>.Success(existingRoute);
        }
        catch (Exception ex)
        {
            await transaction.RollbackAsync();
            _logger.LogError(ex, "Error updating route for domain: {Domain}, path: {Path}", domain, path);
            return Result<RouteEntity>.Failure(Error.Internal("Failed to update route", ex.Message));
        }
    }

    public async Task<Result> DeleteRouteByIdAsync(Guid id, string userId)
    {
        using var transaction = await _context.Database.BeginTransactionAsync();

        try
        {
            if (string.IsNullOrWhiteSpace(userId))
                return Result.Failure(Error.Required("userId"));

            // Find existing route
            var existingRoute = await _context.Routes
                .Include(r => r.Properties)
                .Include(r => r.Domain)
                .FirstOrDefaultAsync(r => r.Id == id);

            if (existingRoute == null)
                return Result.Failure(Error.NotFound("Route", id.ToString()));

            // Verify ownership
            if (existingRoute.Properties == null || existingRoute.Properties.OwnerId != userId)
                return Result.Failure(Error.Forbidden());

            // Delete route
            _context.Routes.Remove(existingRoute);
            await _context.SaveChangesAsync();

            // Update cached route count for the domain
            if (existingRoute.DomainId.HasValue)
            {
                await IncrementRouteCountAsync(existingRoute.DomainId.Value, -1);
            }

            // Propagate to click-router API synchronously
            // Click-router-api handles route family deletion atomically for conditional routes
            var apiResult = await _clickRouterApiClient.DeleteRouteByIdAsync(id, userId);

            if (apiResult.IsFailure)
            {
                await transaction.RollbackAsync();
                _logger.LogError("Failed to delete route in click-router API: {Error}", apiResult.Error);
                return Result.Failure(apiResult.ErrorCode ?? "EXTERNAL_SERVICE_ERROR",
                    $"Failed to delete route in click-router API: {apiResult.Error}");
            }

            // Enqueue search index delete via outbox
            await EnqueueSearchDeleteAsync(existingRoute);

            // Enqueue route verification removal via outbox
            await EnqueueRouteVerificationRemovalAsync(existingRoute);
            await _context.SaveChangesAsync();

            await transaction.CommitAsync();

            _logger.LogInformation("Route deleted by ID: {RouteId}, Link: {Link}", existingRoute.Id, existingRoute.Link);

            return Result.Success();
        }
        catch (Exception ex)
        {
            await transaction.RollbackAsync();
            _logger.LogError(ex, "Error deleting route by ID: {RouteId}", id);
            return Result.Failure(Error.Internal("Failed to delete route", ex.Message));
        }
    }

    public async Task<Result> DeleteRouteAsync(string domain, string path, string userId, string? switchParam = null)
    {
        using var transaction = await _context.Database.BeginTransactionAsync();

        try
        {
            if (string.IsNullOrWhiteSpace(domain))
                return Result.Failure(Error.Required("domain"));

            if (string.IsNullOrWhiteSpace(path))
                return Result.Failure(Error.Required("path"));

            if (string.IsNullOrWhiteSpace(userId))
                return Result.Failure(Error.Required("userId"));

            // Find existing route
            var linkPattern = $"{domain}/{path}";
            var existingRoute = await _context.Routes
                .Include(r => r.Properties)
                .Include(r => r.Domain)
                .FirstOrDefaultAsync(r => r.Link.Contains(linkPattern));

            if (existingRoute == null)
                return Result.Failure(Error.NotFound("Route", $"{domain}/{path}"));

            // Verify ownership
            if (existingRoute.Properties == null || existingRoute.Properties.OwnerId != userId)
                return Result.Failure(Error.Forbidden());

            // Delete route
            _context.Routes.Remove(existingRoute);
            await _context.SaveChangesAsync();

            // Update cached route count for the domain
            if (existingRoute.DomainId.HasValue)
            {
                await IncrementRouteCountAsync(existingRoute.DomainId.Value, -1);
            }

            // Propagate to click-router API synchronously
            var apiResult = await _clickRouterApiClient.DeleteRouteAsync(domain, path, userId);

            if (apiResult.IsFailure)
            {
                await transaction.RollbackAsync();
                _logger.LogError("Failed to delete route in click-router API: {Error}", apiResult.Error);
                return Result.Failure(apiResult.ErrorCode ?? "EXTERNAL_SERVICE_ERROR",
                    $"Failed to delete route in click-router API: {apiResult.Error}");
            }

            // Enqueue search index delete via outbox
            await EnqueueSearchDeleteAsync(existingRoute);

            // Enqueue route verification removal via outbox
            await EnqueueRouteVerificationRemovalAsync(existingRoute);
            await _context.SaveChangesAsync();

            await transaction.CommitAsync();

            _logger.LogInformation("Route deleted: {RouteId}, Link: {Link}", existingRoute.Id, existingRoute.Link);

            return Result.Success();
        }
        catch (Exception ex)
        {
            await transaction.RollbackAsync();
            _logger.LogError(ex, "Error deleting route for domain: {Domain}, path: {Path}", domain, path);
            return Result.Failure(Error.Internal("Failed to delete route", ex.Message));
        }
    }

    public async Task<Result<List<RouteEntity>>> BulkCreateRoutesAsync(List<RouteEntity> routes)
    {
        using var transaction = await _context.Database.BeginTransactionAsync();

        try
        {
            if (routes == null || !routes.Any())
                return Result<List<RouteEntity>>.Failure(Error.Validation("Routes list cannot be empty"));

            // Validate all routes
            foreach (var route in routes)
            {
                var validationResult = route.Validate();
                if (!validationResult.IsValid)
                {
                    var errors = string.Join(", ", validationResult.Errors.Select(e => $"{e.FieldName}: {e.Message}"));
                    return Result<List<RouteEntity>>.Failure(Error.Validation("Route validation failed", errors));
                }

                // Validate domain is mandatory for each route
                if (!route.DomainId.HasValue)
                {
                    return Result<List<RouteEntity>>.Failure(Error.Required("Domain is required for all routes"));
                }
            }

            // Load domains for all routes
            var domainIds = routes
                .Select(r => r.DomainId!.Value)
                .Distinct()
                .ToList();

            var domains = await _context.RouteDomains
                .Where(d => domainIds.Contains(d.Id))
                .ToListAsync();

            // Validate all domains exist and assign them to routes
            foreach (var route in routes)
            {
                route.Domain = domains.FirstOrDefault(d => d.Id == route.DomainId!.Value);
                if (route.Domain == null)
                {
                    return Result<List<RouteEntity>>.Failure(Error.NotFound("Domain", route.DomainId!.Value.ToString()));
                }

                // Verify domain belongs to the route owner
                if (route.Properties != null && !string.IsNullOrWhiteSpace(route.Properties.OwnerId))
                {
                    if (route.Domain.OwnerId != route.Properties.OwnerId)
                    {
                        return Result<List<RouteEntity>>.Failure(Error.Forbidden($"Domain {route.Domain.Name} does not belong to user"));
                    }
                }
            }

            // Add routes to database
            await _context.Routes.AddRangeAsync(routes);
            await _context.SaveChangesAsync();

            // Update cached route counts per domain
            var countsByDomain = routes
                .Where(r => r.DomainId.HasValue)
                .GroupBy(r => r.DomainId!.Value)
                .Select(g => new { DomainId = g.Key, Count = g.Count() });
            foreach (var group in countsByDomain)
            {
                await IncrementRouteCountAsync(group.DomainId, group.Count);
            }

            // Reload routes with domains to ensure navigation properties are populated
            var routeIds = routes.Select(r => r.Id).ToList();
            var savedRoutes = await _context.Routes
                .Include(r => r.Domain)
                .Include(r => r.Properties)
                .Where(r => routeIds.Contains(r.Id))
                .ToListAsync();

            // Propagate to click-router API synchronously
            var apiDtos = savedRoutes.Select(r => r.ToDto()).ToList();
            var apiResult = await _clickRouterApiClient.BulkCreateRoutesAsync(apiDtos);

            if (apiResult.IsFailure)
            {
                await transaction.RollbackAsync();
                _logger.LogError("Failed to bulk create routes in click-router API: {Error}", apiResult.Error);
                return Result<List<RouteEntity>>.Failure(apiResult.ErrorCode ?? "EXTERNAL_SERVICE_ERROR",
                    $"Failed to bulk create routes in click-router API: {apiResult.Error}");
            }

            // Enqueue search index update via outbox
            await EnqueueSearchBulkIndexAsync(savedRoutes);

            // Enqueue route verification via outbox
            await EnqueueBulkRouteVerificationAsync(savedRoutes);
            await _context.SaveChangesAsync();

            await transaction.CommitAsync();

            _logger.LogInformation("Bulk created {Count} routes", savedRoutes.Count);

            return Result<List<RouteEntity>>.Success(savedRoutes);
        }
        catch (Exception ex)
        {
            await transaction.RollbackAsync();
            _logger.LogError(ex, "Error bulk creating routes");
            return Result<List<RouteEntity>>.Failure(Error.Internal("Failed to bulk create routes", ex.Message));
        }
    }

    public async Task<Result<List<RouteEntity>>> BulkUpdateRoutesAsync(string userId, List<RouteEntity> routes)
    {
        using var transaction = await _context.Database.BeginTransactionAsync();

        try
        {
            if (string.IsNullOrWhiteSpace(userId))
                return Result<List<RouteEntity>>.Failure(Error.Required("userId"));

            if (routes == null || !routes.Any())
                return Result<List<RouteEntity>>.Failure(Error.Validation("Routes list cannot be empty"));

            // Validate all routes
            foreach (var route in routes)
            {
                var validationResult = route.Validate();
                if (!validationResult.IsValid)
                {
                    var errors = string.Join(", ", validationResult.Errors.Select(e => $"{e.FieldName}: {e.Message}"));
                    return Result<List<RouteEntity>>.Failure(Error.Validation("Route validation failed", errors));
                }

                // Validate domain is mandatory for each route
                if (!route.DomainId.HasValue)
                {
                    return Result<List<RouteEntity>>.Failure(Error.Required("Domain is required for all routes"));
                }
            }

            // Verify ownership of all routes
            var routeIds = routes.Select(r => r.Id).ToList();
            var existingRoutes = await _context.Routes
                .Include(r => r.Properties)
                .Include(r => r.Domain)
                .Where(r => routeIds.Contains(r.Id))
                .ToListAsync();

            if (existingRoutes.Any(r => r.Properties == null || r.Properties.OwnerId != userId))
                return Result<List<RouteEntity>>.Failure(Error.Forbidden());

            // Load all domains for validation
            var domainIds = routes
                .Select(r => r.DomainId!.Value)
                .Distinct()
                .ToList();

            var domains = await _context.RouteDomains
                .Where(d => domainIds.Contains(d.Id))
                .ToListAsync();

            var domainLookup = domains.ToDictionary(d => d.Id);

            // Build lookup of existing routes to preserve immutable fields
            var existingRouteLookup = existingRoutes.ToDictionary(r => r.Id);

            // Validate all domains exist and belong to user, then assign to routes
            foreach (var route in routes)
            {
                if (!domainLookup.ContainsKey(route.DomainId!.Value))
                {
                    return Result<List<RouteEntity>>.Failure(Error.NotFound("Domain", route.DomainId.Value.ToString()));
                }

                route.Domain = domainLookup[route.DomainId.Value];

                // Verify domain belongs to current user
                if (route.Domain.OwnerId != userId)
                {
                    return Result<List<RouteEntity>>.Failure(Error.Forbidden($"Domain {route.Domain.Name} does not belong to user"));
                }

                // Link and DomainId are immutable after creation — preserve existing values
                if (existingRouteLookup.TryGetValue(route.Id, out var existing))
                {
                    route.Link = existing.Link;
                    route.DomainId = existing.DomainId;
                    route.Domain = existing.Domain;
                }
            }

            // Update routes in database
            _context.Routes.UpdateRange(routes);
            await _context.SaveChangesAsync();

            // Propagate to click-router API synchronously
            var apiDtos = routes.Select(r => r.ToDto()).ToList();
            var apiResult = await _clickRouterApiClient.BulkUpdateRoutesAsync(userId, apiDtos);

            if (apiResult.IsFailure)
            {
                await transaction.RollbackAsync();
                _logger.LogError("Failed to bulk update routes in click-router API: {Error}", apiResult.Error);
                return Result<List<RouteEntity>>.Failure(apiResult.ErrorCode ?? "EXTERNAL_SERVICE_ERROR",
                    $"Failed to bulk update routes in click-router API: {apiResult.Error}");
            }

            // Enqueue search index update via outbox
            await EnqueueSearchBulkIndexAsync(routes);

            // Enqueue route verification via outbox (destinations may have changed)
            await EnqueueBulkRouteVerificationAsync(routes);
            await _context.SaveChangesAsync();

            await transaction.CommitAsync();

            _logger.LogInformation("Bulk updated {Count} routes", routes.Count);

            return Result<List<RouteEntity>>.Success(routes);
        }
        catch (Exception ex)
        {
            await transaction.RollbackAsync();
            _logger.LogError(ex, "Error bulk updating routes");
            return Result<List<RouteEntity>>.Failure(Error.Internal("Failed to bulk update routes", ex.Message));
        }
    }

    public async Task<Result> BulkDeleteRoutesAsync(string userId, List<string> routeIds)
    {
        using var transaction = await _context.Database.BeginTransactionAsync();

        try
        {
            if (string.IsNullOrWhiteSpace(userId))
                return Result.Failure(Error.Required("userId"));

            if (routeIds == null || !routeIds.Any())
                return Result.Failure(Error.Validation("Route IDs list cannot be empty"));

            // Parse route IDs to GUIDs
            var guids = routeIds.Select(id => Guid.TryParse(id, out var guid) ? guid : Guid.Empty)
                               .Where(g => g != Guid.Empty)
                               .ToList();

            if (!guids.Any())
                return Result.Failure(Error.Validation("No valid route IDs provided"));

            // Find routes to delete
            var routesToDelete = await _context.Routes
                .Include(r => r.Properties)
                .Include(r => r.Domain)
                .Where(r => guids.Contains(r.Id))
                .ToListAsync();

            if (!routesToDelete.Any())
                return Result.Failure(Error.NotFound("Routes", "None of the specified routes were found"));

            // Verify ownership of all routes
            if (routesToDelete.Any(r => r.Properties == null || r.Properties.OwnerId != userId))
                return Result.Failure(Error.Forbidden());

            // Delete routes
            _context.Routes.RemoveRange(routesToDelete);
            await _context.SaveChangesAsync();

            // Update cached route counts per domain
            var countsByDomain = routesToDelete
                .Where(r => r.DomainId.HasValue)
                .GroupBy(r => r.DomainId!.Value)
                .Select(g => new { DomainId = g.Key, Count = g.Count() });
            foreach (var group in countsByDomain)
            {
                await IncrementRouteCountAsync(group.DomainId, -group.Count);
            }

            // Propagate to click-router API synchronously
            var apiResult = await _clickRouterApiClient.BulkDeleteRoutesAsync(userId, routeIds);

            if (apiResult.IsFailure)
            {
                await transaction.RollbackAsync();
                _logger.LogError("Failed to bulk delete routes in click-router API: {Error}", apiResult.Error);
                return Result.Failure(apiResult.ErrorCode ?? "EXTERNAL_SERVICE_ERROR",
                    $"Failed to bulk delete routes in click-router API: {apiResult.Error}");
            }

            // Enqueue search index delete via outbox
            await EnqueueSearchBulkDeleteAsync(routesToDelete);

            // Enqueue route verification removal via outbox
            await EnqueueBulkRouteVerificationRemovalAsync(routesToDelete);
            await _context.SaveChangesAsync();

            await transaction.CommitAsync();

            _logger.LogInformation("Bulk deleted {Count} routes", routesToDelete.Count);

            return Result.Success();
        }
        catch (Exception ex)
        {
            await transaction.RollbackAsync();
            _logger.LogError(ex, "Error bulk deleting routes");
            return Result.Failure(Error.Internal("Failed to bulk delete routes", ex.Message));
        }
    }

    public async Task<Result<(List<RouteEntity> Routes, int TotalCount)>> ListRoutesAsync(
        int page = 1,
        int pageSize = 20,
        string? search = null,
        string? status = null,
        string? ownerId = null,
        string? workspaceId = null)
    {
        try
        {
            var query = _context.Routes
                .Include(r => r.Properties)
                .Include(r => r.Domain)
                .AsQueryable();

            // Apply filters
            if (!string.IsNullOrWhiteSpace(search))
            {
                query = query.Where(r => r.Link.Contains(search) ||
                                       (r.Dest != null && r.Dest.Contains(search)) ||
                                       r.Switch.Contains(search));
            }

            if (!string.IsNullOrWhiteSpace(status))
            {
                query = query.Where(r => r.Status == status);
            }

            if (!string.IsNullOrWhiteSpace(ownerId) && ownerId != "all")
            {
                query = query.Where(r => r.Properties != null && r.Properties.OwnerId == ownerId);
            }

            if (!string.IsNullOrWhiteSpace(workspaceId) && workspaceId != "all")
            {
                query = query.Where(r => r.Properties != null && r.Properties.WorkspaceId == workspaceId);
            }

            // Get total count
            var totalCount = await query.CountAsync();

            // Apply pagination
            var routes = await query
                .OrderByDescending(r => r.Id)
                .Skip((page - 1) * pageSize)
                .Take(pageSize)
                .ToListAsync();

            return Result<(List<RouteEntity> Routes, int TotalCount)>.Success((routes, totalCount));
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error listing routes");
            return Result<(List<RouteEntity> Routes, int TotalCount)>.Failure(
                Error.Internal("Failed to list routes", ex.Message));
        }
    }

    /// <summary>
    /// Extracts switch, domain, and path from a route object for API calls
    /// </summary>
    private (string switchParam, string domain, string path) ExtractRouteIdentifiers(RouteEntity route)
    {
        var switchParam = string.IsNullOrEmpty(route.Switch) ? "main" : route.Switch;

        // The Link field contains "domain/path" or just "domain"
        var parts = route.Link.Split('/', 2);

        if (parts.Length >= 2)
        {
            return (switchParam, parts[0], parts[1]);
        }
        else if (parts.Length == 1)
        {
            // If there's no path, use the link as domain and "/" as path
            return (switchParam, parts[0], "/");
        }

        // Fallback - this shouldn't happen with valid data
        _logger.LogWarning("Unable to extract domain/path from route link: {Link}", route.Link);
        return (switchParam, string.Empty, string.Empty);
    }

    #region Route Count Helpers

    /// <summary>
    /// Atomically increment (or decrement) the cached route count for a domain.
    /// Uses upsert semantics: creates the row if it doesn't exist yet.
    /// </summary>
    private async Task IncrementRouteCountAsync(Guid domainId, int delta)
    {
        var countRow = await _context.DomainRouteCounts
            .FirstOrDefaultAsync(c => c.DomainId == domainId);

        if (countRow == null)
        {
            countRow = new DomainRouteCount
            {
                DomainId = domainId,
                RouteCount = Math.Max(0, delta)
            };
            await _context.DomainRouteCounts.AddAsync(countRow);
        }
        else
        {
            countRow.RouteCount = Math.Max(0, countRow.RouteCount + delta);
        }

        await _context.SaveChangesAsync();
    }

    #endregion


    #region Search Index Outbox Helpers

    private static RouteSearchDoc ToSearchDocument(RouteEntity route)
    {
        return new RouteSearchDoc
        {
            Id = route.Id.ToString(),
            Link = route.Link,
            Switch = route.Switch,
            Dest = route.Dest,
            DomainName = route.Domain?.Name,
            Status = route.Status,
            OwnerId = route.Properties?.OwnerId,
            WorkspaceId = route.Properties?.WorkspaceId,
        };
    }

    private async Task EnqueueSearchIndexAsync(RouteEntity route)
    {
        var doc = ToSearchDocument(route);
        await _context.OutboxMessages.AddAsync(new OutboxMessage
        {
            EventType = OutboxEventType.RouteSearchIndex,
            AggregateId = route.Id.ToString(),
            Payload = JsonSerializer.Serialize(doc, _jsonOptions)
        });
    }

    private async Task EnqueueSearchDeleteAsync(RouteEntity route)
    {
        await _context.OutboxMessages.AddAsync(new OutboxMessage
        {
            EventType = OutboxEventType.RouteSearchDelete,
            AggregateId = route.Id.ToString(),
            Payload = JsonSerializer.Serialize(new { id = route.Id.ToString() }, _jsonOptions)
        });
    }

    private async Task EnqueueSearchBulkIndexAsync(List<RouteEntity> routes)
    {
        var docs = routes.Select(ToSearchDocument).ToList();
        await _context.OutboxMessages.AddAsync(new OutboxMessage
        {
            EventType = OutboxEventType.RouteSearchBulkIndex,
            AggregateId = string.Join(",", routes.Select(r => r.Id)),
            Payload = JsonSerializer.Serialize(docs, _jsonOptions)
        });
    }

    private async Task EnqueueSearchBulkDeleteAsync(List<RouteEntity> routes)
    {
        var ids = routes.Select(r => r.Id.ToString()).ToList();
        await _context.OutboxMessages.AddAsync(new OutboxMessage
        {
            EventType = OutboxEventType.RouteSearchBulkDelete,
            AggregateId = string.Join(",", ids),
            Payload = JsonSerializer.Serialize(ids, _jsonOptions)
        });
    }

    #endregion

    #region Route Verification Outbox Helpers

    /// <summary>
    /// Collects all destinations from a route (main dest + conditional policy dests).
    /// </summary>
    private static List<string> CollectDestinations(RouteEntity route)
    {
        var destinations = new List<string>();

        // Add main destination
        if (!string.IsNullOrWhiteSpace(route.Dest))
        {
            destinations.Add(route.Dest);
        }

        // Add destinations from conditional policy
        if (route.Policy is ConditionalPolicy conditionalPolicy)
        {
            foreach (var condition in conditionalPolicy.Conditions)
            {
                if (!string.IsNullOrWhiteSpace(condition.Dest))
                {
                    destinations.Add(condition.Dest);
                }
            }
        }

        return destinations.Distinct().ToList();
    }

    private async Task EnqueueRouteVerificationAsync(RouteEntity route)
    {
        var destinations = CollectDestinations(route);

        // Skip if no destinations to verify
        if (destinations.Count == 0)
        {
            return;
        }

        var payload = new
        {
            id = route.Id.ToString(),
            link = route.Link,
            destinations = destinations,
            owner_id = route.Properties?.OwnerId,
            workspace_id = route.Properties?.WorkspaceId
        };

        await _context.OutboxMessages.AddAsync(new OutboxMessage
        {
            EventType = OutboxEventType.RouteVerificationRequested,
            AggregateId = route.Id.ToString(),
            Payload = JsonSerializer.Serialize(payload, _jsonOptions)
        });
    }

    private async Task EnqueueRouteVerificationRemovalAsync(RouteEntity route)
    {
        var payload = new
        {
            id = route.Id.ToString(),
            link = route.Link
        };

        await _context.OutboxMessages.AddAsync(new OutboxMessage
        {
            EventType = OutboxEventType.RouteVerificationRemovalRequested,
            AggregateId = route.Id.ToString(),
            Payload = JsonSerializer.Serialize(payload, _jsonOptions)
        });
    }

    private async Task EnqueueBulkRouteVerificationAsync(List<RouteEntity> routes)
    {
        foreach (var route in routes)
        {
            await EnqueueRouteVerificationAsync(route);
        }
    }

    private async Task EnqueueBulkRouteVerificationRemovalAsync(List<RouteEntity> routes)
    {
        foreach (var route in routes)
        {
            await EnqueueRouteVerificationRemovalAsync(route);
        }
    }

    #endregion
}

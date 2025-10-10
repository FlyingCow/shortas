using Microsoft.EntityFrameworkCore;
using System.Text.Json;
using ShortasProxyApi.Domain.Common;
using ShortasProxyApi.Domain.Entities;
using ShortasProxyApi.Domain.Interfaces;
using ShortasProxyApi.Infrastructure.Data;
using RouteEntity = ShortasProxyApi.Domain.Entities.Route;

namespace ShortasProxyApi.Infrastructure.Services;

public class EfRouteService : IRouteService
{
    private readonly ApplicationDbContext _context;
    private readonly IOutboxRepository _outboxRepository;
    private readonly ILogger<EfRouteService> _logger;
    private readonly JsonSerializerOptions _jsonOptions;

    public EfRouteService(
        ApplicationDbContext context,
        IOutboxRepository outboxRepository,
        ILogger<EfRouteService> logger)
    {
        _context = context;
        _outboxRepository = outboxRepository;
        _logger = logger;
        _jsonOptions = new JsonSerializerOptions
        {
            PropertyNamingPolicy = JsonNamingPolicy.CamelCase
        };
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

            // Build the link pattern to search for
            var linkPattern = $"{domain}/{path}";

            var route = await _context.Routes
                .Include(r => r.Properties)
                .FirstOrDefaultAsync(r => r.Link.Contains(linkPattern) &&
                                        r.Properties != null &&
                                        r.Properties.OwnerId == userId);

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

            // Check if route already exists
            var existing = await _context.Routes
                .FirstOrDefaultAsync(r => r.Link == route.Link);

            if (existing != null)
                return Result<RouteEntity>.Failure(Error.Conflict("Route with this link already exists"));

            // Add route to database
            await _context.Routes.AddAsync(route);
            await _context.SaveChangesAsync();

            // Create outbox message for eventual consistency with click-router-api
            var outboxMessage = new OutboxMessage
            {
                EventType = OutboxEventType.RouteCreated,
                AggregateId = route.Id.ToString(),
                Payload = JsonSerializer.Serialize(route, _jsonOptions),
                CreatedAt = DateTime.UtcNow,
                Status = OutboxMessageStatus.Pending
            };

            await _outboxRepository.AddAsync(outboxMessage);
            await _outboxRepository.SaveChangesAsync();

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
                    existingRoute.Properties.Scripts = route.Properties.Scripts;
                    existingRoute.Properties.Tags = route.Properties.Tags;
                    existingRoute.Properties.Custom = route.Properties.Custom;
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

            // Create outbox message
            var outboxMessage = new OutboxMessage
            {
                EventType = OutboxEventType.RouteUpdated,
                AggregateId = existingRoute.Id.ToString(),
                Payload = JsonSerializer.Serialize(existingRoute, _jsonOptions),
                CreatedAt = DateTime.UtcNow,
                Status = OutboxMessageStatus.Pending
            };

            await _outboxRepository.AddAsync(outboxMessage);
            await _outboxRepository.SaveChangesAsync();

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

    public async Task<Result> DeleteRouteAsync(string domain, string path, string userId)
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
                .FirstOrDefaultAsync(r => r.Link.Contains(linkPattern));

            if (existingRoute == null)
                return Result.Failure(Error.NotFound("Route", $"{domain}/{path}"));

            // Verify ownership
            if (existingRoute.Properties == null || existingRoute.Properties.OwnerId != userId)
                return Result.Failure(Error.Forbidden());

            // Delete route
            _context.Routes.Remove(existingRoute);
            await _context.SaveChangesAsync();

            // Create outbox message
            var outboxMessage = new OutboxMessage
            {
                EventType = OutboxEventType.RouteDeleted,
                AggregateId = existingRoute.Id.ToString(),
                Payload = JsonSerializer.Serialize(new { Domain = domain, Path = path, RouteId = existingRoute.Id }, _jsonOptions),
                CreatedAt = DateTime.UtcNow,
                Status = OutboxMessageStatus.Pending
            };

            await _outboxRepository.AddAsync(outboxMessage);
            await _outboxRepository.SaveChangesAsync();

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
            }

            // Add routes to database
            await _context.Routes.AddRangeAsync(routes);
            await _context.SaveChangesAsync();

            // Create outbox message for bulk operation
            var outboxMessage = new OutboxMessage
            {
                EventType = OutboxEventType.RouteBulkCreated,
                AggregateId = Guid.NewGuid().ToString(),
                Payload = JsonSerializer.Serialize(routes, _jsonOptions),
                CreatedAt = DateTime.UtcNow,
                Status = OutboxMessageStatus.Pending
            };

            await _outboxRepository.AddAsync(outboxMessage);
            await _outboxRepository.SaveChangesAsync();

            await transaction.CommitAsync();

            _logger.LogInformation("Bulk created {Count} routes", routes.Count);

            return Result<List<RouteEntity>>.Success(routes);
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
            }

            // Verify ownership of all routes
            var routeIds = routes.Select(r => r.Id).ToList();
            var existingRoutes = await _context.Routes
                .Include(r => r.Properties)
                .Where(r => routeIds.Contains(r.Id))
                .ToListAsync();

            if (existingRoutes.Any(r => r.Properties == null || r.Properties.OwnerId != userId))
                return Result<List<RouteEntity>>.Failure(Error.Forbidden());

            // Update routes in database
            _context.Routes.UpdateRange(routes);
            await _context.SaveChangesAsync();

            // Create outbox message for bulk operation
            var outboxMessage = new OutboxMessage
            {
                EventType = OutboxEventType.RouteBulkUpdated,
                AggregateId = Guid.NewGuid().ToString(),
                Payload = JsonSerializer.Serialize(routes, _jsonOptions),
                CreatedAt = DateTime.UtcNow,
                Status = OutboxMessageStatus.Pending
            };

            await _outboxRepository.AddAsync(outboxMessage);
            await _outboxRepository.SaveChangesAsync();

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

            // Create outbox message for bulk operation
            var outboxMessage = new OutboxMessage
            {
                EventType = OutboxEventType.RouteBulkDeleted,
                AggregateId = Guid.NewGuid().ToString(),
                Payload = JsonSerializer.Serialize(new { RouteIds = routeIds }, _jsonOptions),
                CreatedAt = DateTime.UtcNow,
                Status = OutboxMessageStatus.Pending
            };

            await _outboxRepository.AddAsync(outboxMessage);
            await _outboxRepository.SaveChangesAsync();

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
        string? ownerId = null)
    {
        try
        {
            var query = _context.Routes
                .Include(r => r.Properties)
                .AsQueryable();

            // Apply filters
            if (!string.IsNullOrWhiteSpace(search))
            {
                query = query.Where(r => r.Link.Contains(search) ||
                                       r.Dest.Contains(search) ||
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
}

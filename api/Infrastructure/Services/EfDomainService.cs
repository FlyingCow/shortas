using Microsoft.EntityFrameworkCore;
using ShortasProxyApi.Domain.Common;
using ShortasProxyApi.Domain.Entities;
using ShortasProxyApi.Domain.Interfaces;
using ShortasProxyApi.Infrastructure.Data;

namespace ShortasProxyApi.Infrastructure.Services;

public class EfDomainService : IDomainService
{
    private readonly ApplicationDbContext _context;
    private readonly IRouteService _routeService;
    private readonly ILogger<EfDomainService> _logger;

    private const string IndexSwitch = "index";
    private const string NotFoundSwitch = "404";

    public EfDomainService(
        ApplicationDbContext context,
        IRouteService routeService,
        ILogger<EfDomainService> logger)
    {
        _context = context;
        _routeService = routeService;
        _logger = logger;
    }

    public async Task<Result<RouteDomain?>> GetDomainByIdAsync(Guid id, string userId)
    {
        try
        {
            if (string.IsNullOrWhiteSpace(userId))
                return Result<RouteDomain?>.Failure(Error.Required("userId"));

            var domain = await _context.RouteDomains
                .FirstOrDefaultAsync(d => d.Id == id && d.OwnerId == userId);

            if (domain == null)
                return Result<RouteDomain?>.Failure(Error.NotFound("Domain", id.ToString()));

            return Result<RouteDomain?>.Success(domain);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error getting domain by ID: {DomainId}", id);
            return Result<RouteDomain?>.Failure(Error.Internal("Failed to get domain", ex.Message));
        }
    }

    public async Task<Result<RouteDomain?>> GetDomainByNameAsync(string name, string userId)
    {
        try
        {
            if (string.IsNullOrWhiteSpace(name))
                return Result<RouteDomain?>.Failure(Error.Required("name"));

            if (string.IsNullOrWhiteSpace(userId))
                return Result<RouteDomain?>.Failure(Error.Required("userId"));

            var normalizedName = name.ToLowerInvariant();
            var domain = await _context.RouteDomains
                .FirstOrDefaultAsync(d => d.Name == normalizedName && d.OwnerId == userId);

            if (domain == null)
                return Result<RouteDomain?>.Failure(Error.NotFound("Domain", name));

            return Result<RouteDomain?>.Success(domain);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error getting domain by name: {DomainName}", name);
            return Result<RouteDomain?>.Failure(Error.Internal("Failed to get domain", ex.Message));
        }
    }

    public async Task<Result<RouteDomain>> CreateDomainAsync(RouteDomain domain, string userId)
    {
        try
        {
            if (string.IsNullOrWhiteSpace(domain.Name))
                return Result<RouteDomain>.Failure(Error.Required("name"));

            if (string.IsNullOrWhiteSpace(userId))
                return Result<RouteDomain>.Failure(Error.Required("userId"));

            // Set the owner
            domain.OwnerId = userId;

            // Check if domain already exists for this user
            var normalizedName = domain.Name.ToLowerInvariant();
            var existing = await _context.RouteDomains
                .FirstOrDefaultAsync(d => d.Name == normalizedName && d.OwnerId == userId);

            if (existing != null)
                return Result<RouteDomain>.Failure(Error.Conflict("Domain with this name already exists"));

            // Add domain to database
            await _context.RouteDomains.AddAsync(domain);

            // Create outbox message for domain verification
            var verificationPayload = System.Text.Json.JsonSerializer.Serialize(new
            {
                id = domain.Id.ToString(),
                name = domain.Name,
                owner_id = domain.OwnerId
            });

            var outboxMessage = new OutboxMessage
            {
                EventType = OutboxEventType.DomainVerificationRequested,
                AggregateId = domain.Id.ToString(),
                Payload = verificationPayload
            };

            await _context.OutboxMessages.AddAsync(outboxMessage);
            await _context.SaveChangesAsync();

            _logger.LogInformation("Domain created: {DomainId}, Name: {DomainName}, OwnerId: {OwnerId}", domain.Id, domain.Name, domain.OwnerId);

            return Result<RouteDomain>.Success(domain);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error creating domain");
            return Result<RouteDomain>.Failure(Error.Internal("Failed to create domain", ex.Message));
        }
    }

    public async Task<Result<RouteDomain>> UpdateDomainAsync(Guid id, RouteDomain domain, string userId)
    {
        try
        {
            if (string.IsNullOrWhiteSpace(domain.Name))
                return Result<RouteDomain>.Failure(Error.Required("name"));

            if (string.IsNullOrWhiteSpace(userId))
                return Result<RouteDomain>.Failure(Error.Required("userId"));

            // Find existing domain
            var existingDomain = await _context.RouteDomains
                .FirstOrDefaultAsync(d => d.Id == id);

            if (existingDomain == null)
                return Result<RouteDomain>.Failure(Error.NotFound("Domain", id.ToString()));

            // Verify ownership
            if (existingDomain.OwnerId != userId)
                return Result<RouteDomain>.Failure(Error.Forbidden());

            // Check if another domain with the same name exists for this user
            var normalizedName = domain.Name.ToLowerInvariant();
            var duplicate = await _context.RouteDomains
                .FirstOrDefaultAsync(d => d.Name == normalizedName && d.Id != id && d.OwnerId == userId);

            if (duplicate != null)
                return Result<RouteDomain>.Failure(Error.Conflict("Domain with this name already exists"));

            // Update domain properties
            existingDomain.Name = domain.Name;

            _context.RouteDomains.Update(existingDomain);
            await _context.SaveChangesAsync();

            _logger.LogInformation("Domain updated: {DomainId}, Name: {DomainName}, OwnerId: {OwnerId}", existingDomain.Id, existingDomain.Name, existingDomain.OwnerId);

            return Result<RouteDomain>.Success(existingDomain);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error updating domain: {DomainId}", id);
            return Result<RouteDomain>.Failure(Error.Internal("Failed to update domain", ex.Message));
        }
    }

    public async Task<Result> DeleteDomainAsync(Guid id, string userId)
    {
        try
        {
            if (string.IsNullOrWhiteSpace(userId))
                return Result.Failure(Error.Required("userId"));

            // Find existing domain
            var existingDomain = await _context.RouteDomains
                .Include(d => d.Routes)
                .FirstOrDefaultAsync(d => d.Id == id);

            if (existingDomain == null)
                return Result.Failure(Error.NotFound("Domain", id.ToString()));

            // Verify ownership
            if (existingDomain.OwnerId != userId)
                return Result.Failure(Error.Forbidden());

            // Check if domain has routes
            if (existingDomain.Routes.Any())
                return Result.Failure(Error.Conflict("Cannot delete domain with existing routes"));

            // Create outbox message for domain removal
            var removalPayload = System.Text.Json.JsonSerializer.Serialize(new
            {
                id = existingDomain.Id.ToString()
            });

            var outboxMessage = new OutboxMessage
            {
                EventType = OutboxEventType.DomainRemovalRequested,
                AggregateId = existingDomain.Id.ToString(),
                Payload = removalPayload
            };

            await _context.OutboxMessages.AddAsync(outboxMessage);

            // Delete domain
            _context.RouteDomains.Remove(existingDomain);
            await _context.SaveChangesAsync();

            _logger.LogInformation("Domain deleted: {DomainId}, Name: {DomainName}, OwnerId: {OwnerId}", existingDomain.Id, existingDomain.Name, existingDomain.OwnerId);

            return Result.Success();
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error deleting domain: {DomainId}", id);
            return Result.Failure(Error.Internal("Failed to delete domain", ex.Message));
        }
    }

    public async Task<Result<(List<RouteDomain> Domains, int TotalCount)>> ListDomainsAsync(
        string userId,
        int page = 1,
        int pageSize = 20,
        string? search = null)
    {
        try
        {
            if (string.IsNullOrWhiteSpace(userId))
                return Result<(List<RouteDomain> Domains, int TotalCount)>.Failure(Error.Required("userId"));

            var query = _context.RouteDomains
                .Where(d => d.OwnerId == userId)
                .AsQueryable();

            // Apply search filter
            if (!string.IsNullOrWhiteSpace(search))
            {
                var searchLower = search.ToLowerInvariant();
                query = query.Where(d => d.Name.Contains(searchLower));
            }

            // Get total count
            var totalCount = await query.CountAsync();

            // Apply pagination
            var domains = await query
                .OrderBy(d => d.Name)
                .Skip((page - 1) * pageSize)
                .Take(pageSize)
                .ToListAsync();

            return Result<(List<RouteDomain> Domains, int TotalCount)>.Success((domains, totalCount));
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error listing domains");
            return Result<(List<RouteDomain> Domains, int TotalCount)>.Failure(
                Error.Internal("Failed to list domains", ex.Message));
        }
    }

    public async Task<Result<RouteDomain>> UpdateCustomPagesAsync(Guid id, string userId, string? customIndexUrl, string? customNotFoundUrl)
    {
        try
        {
            if (string.IsNullOrWhiteSpace(userId))
                return Result<RouteDomain>.Failure(Error.Required("userId"));

            // Find existing domain
            var existingDomain = await _context.RouteDomains
                .FirstOrDefaultAsync(d => d.Id == id);

            if (existingDomain == null)
                return Result<RouteDomain>.Failure(Error.NotFound("Domain", id.ToString()));

            // Verify ownership
            if (existingDomain.OwnerId != userId)
                return Result<RouteDomain>.Failure(Error.Forbidden());

            var previousIndexUrl = existingDomain.CustomIndexUrl;
            var previousNotFoundUrl = existingDomain.CustomNotFoundUrl;
            var domainName = existingDomain.Name;
            var link = $"{domainName}%2F";

            // Propagate routes to downstream API first
            var indexPropagationResult = await PropagateCustomPageRoute(
                id, domainName, IndexSwitch, link, userId, previousIndexUrl, customIndexUrl);
            if (indexPropagationResult.IsFailure)
            {
                return Result<RouteDomain>.Failure(indexPropagationResult.ErrorCode ?? "PROPAGATION_ERROR",
                    $"Failed to propagate index route: {indexPropagationResult.Error}");
            }

            var notFoundPropagationResult = await PropagateCustomPageRoute(
                id, domainName, NotFoundSwitch, link, userId, previousNotFoundUrl, customNotFoundUrl);
            if (notFoundPropagationResult.IsFailure)
            {
                // Try to rollback the index route change
                await PropagateCustomPageRoute(id, domainName, IndexSwitch, link, userId, customIndexUrl, previousIndexUrl);
                return Result<RouteDomain>.Failure(notFoundPropagationResult.ErrorCode ?? "PROPAGATION_ERROR",
                    $"Failed to propagate 404 route: {notFoundPropagationResult.Error}");
            }

            // Update custom page URLs on domain entity
            existingDomain.CustomIndexUrl = customIndexUrl;
            existingDomain.CustomNotFoundUrl = customNotFoundUrl;

            _context.RouteDomains.Update(existingDomain);
            await _context.SaveChangesAsync();

            _logger.LogInformation("Domain custom pages updated: {DomainId}, CustomIndexUrl: {CustomIndexUrl}, CustomNotFoundUrl: {CustomNotFoundUrl}",
                existingDomain.Id, customIndexUrl, customNotFoundUrl);

            return Result<RouteDomain>.Success(existingDomain);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error updating domain custom pages: {DomainId}", id);
            return Result<RouteDomain>.Failure(Error.Internal("Failed to update domain custom pages", ex.Message));
        }
    }

    private async Task<Result> PropagateCustomPageRoute(Guid domainId, string domainName, string switchValue, string link, string userId, string? previousUrl, string? newUrl)
    {
        // If URL hasn't changed, no need to propagate
        if (previousUrl == newUrl)
        {
            return Result.Success();
        }

        try
        {
            if (!string.IsNullOrEmpty(newUrl))
            {
                // Create or update route in downstream API
                var routeId = Guid.NewGuid();
                var route = new Domain.Entities.Route
                {
                    Id = routeId,
                    DomainId = domainId,
                    Domain = new RouteDomain { Name = domainName },
                    Switch = switchValue,
                    Link = link,
                    Dest = newUrl,
                    DestFormat = "Http",
                    Code = 302,
                    Ttl = 0,
                    Status = "Active",
                    Terminal = "External",
                    Properties = new RouteProperties
                    {
                        RouteId = routeId.ToString(),
                        DomainId = domainName,
                        OwnerId = userId,
                        Tags = new List<string> { $"custom-page:{switchValue}" }
                    }
                };

                if (!string.IsNullOrEmpty(previousUrl))
                {
                    // Update existing route
                    var updateResult = await _routeService.UpdateRouteAsync(domainName, "/", userId, route);
                    if (updateResult.IsFailure)
                    {
                        return Result.Failure(updateResult.ErrorCode ?? "UPDATE_FAILED", updateResult.Error);
                    }
                }
                else
                {
                    // Create new route
                    var createResult = await _routeService.CreateRouteAsync(route);
                    if (createResult.IsFailure)
                    {
                        return Result.Failure(createResult.ErrorCode ?? "CREATE_FAILED", createResult.Error);
                    }
                }
            }
            else if (!string.IsNullOrEmpty(previousUrl))
            {
                // Delete route from downstream API
                var deleteResult = await _routeService.DeleteRouteAsync(domainName, "/", userId, switchValue);
                if (deleteResult.IsFailure)
                {
                    return Result.Failure(deleteResult.ErrorCode ?? "DELETE_FAILED", deleteResult.Error);
                }
            }

            return Result.Success();
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error propagating custom page route: {DomainName}, {Switch}", domainName, switchValue);
            return Result.Failure(Error.Internal("Failed to propagate route", ex.Message));
        }
    }
}

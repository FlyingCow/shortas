using Microsoft.EntityFrameworkCore;
using ShortasProxyApi.Domain.Common;
using ShortasProxyApi.Domain.Entities;
using ShortasProxyApi.Domain.Interfaces;
using ShortasProxyApi.Infrastructure.Data;

namespace ShortasProxyApi.Infrastructure.Services;

public class EfDomainService : IDomainService
{
    private readonly ApplicationDbContext _context;
    private readonly ILogger<EfDomainService> _logger;

    public EfDomainService(
        ApplicationDbContext context,
        ILogger<EfDomainService> logger)
    {
        _context = context;
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
}

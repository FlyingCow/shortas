using Microsoft.EntityFrameworkCore;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;
using Microsoft.Extensions.Logging;
using ShortasProxyApi.Domain.Entities;
using ShortasProxyApi.Infrastructure.Data;

namespace ShortasProxyApi.Infrastructure.BackgroundServices;

/// <summary>
/// Seeds shared/common domains on application startup.
/// Shared domains can be used by any authenticated user to create routes.
/// </summary>
public class SharedDomainSeeder : IHostedService
{
    private readonly IServiceProvider _serviceProvider;
    private readonly IConfiguration _configuration;
    private readonly ILogger<SharedDomainSeeder> _logger;

    /// <summary>
    /// Special owner ID for system-owned shared domains
    /// </summary>
    public const string SystemOwnerId = "__system__";

    public SharedDomainSeeder(
        IServiceProvider serviceProvider,
        IConfiguration configuration,
        ILogger<SharedDomainSeeder> logger)
    {
        _serviceProvider = serviceProvider;
        _configuration = configuration;
        _logger = logger;
    }

    public async Task StartAsync(CancellationToken cancellationToken)
    {
        _logger.LogInformation("SharedDomainSeeder starting");

        var sharedDomainNames = _configuration.GetSection("SharedDomains:Names").Get<List<string>>() ?? new List<string>();

        if (sharedDomainNames.Count == 0)
        {
            _logger.LogInformation("No shared domains configured");
            return;
        }

        using var scope = _serviceProvider.CreateScope();
        var context = scope.ServiceProvider.GetRequiredService<ApplicationDbContext>();

        foreach (var domainName in sharedDomainNames)
        {
            var normalizedName = domainName.ToLowerInvariant();

            var existingDomain = await context.RouteDomains
                .FirstOrDefaultAsync(d => d.Name == normalizedName, cancellationToken);

            if (existingDomain != null)
            {
                if (!existingDomain.IsShared)
                {
                    _logger.LogWarning(
                        "Domain '{DomainName}' already exists but is not marked as shared. Skipping.",
                        normalizedName);
                }
                else
                {
                    _logger.LogDebug("Shared domain '{DomainName}' already exists", normalizedName);
                }
                continue;
            }

            var sharedDomain = new RouteDomain
            {
                Id = Guid.NewGuid(),
                Name = normalizedName,
                OwnerId = SystemOwnerId,
                IsShared = true,
                VerificationStatus = DomainVerificationStatus.Verified,
                VerificationReason = "shared_domain"
            };

            await context.RouteDomains.AddAsync(sharedDomain, cancellationToken);
            _logger.LogInformation("Created shared domain: {DomainName} (ID: {DomainId})", normalizedName, sharedDomain.Id);
        }

        await context.SaveChangesAsync(cancellationToken);
        _logger.LogInformation("SharedDomainSeeder completed");
    }

    public Task StopAsync(CancellationToken cancellationToken)
    {
        return Task.CompletedTask;
    }
}

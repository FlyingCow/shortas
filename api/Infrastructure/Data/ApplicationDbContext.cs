using Microsoft.EntityFrameworkCore;
using ShortasProxyApi.Domain.Entities;
using ShortasProxyApi.Infrastructure.Data.Configurations;
using RouteEntity = ShortasProxyApi.Domain.Entities.Route;

namespace ShortasProxyApi.Infrastructure.Data;

public class ApplicationDbContext : DbContext
{
    public ApplicationDbContext(DbContextOptions<ApplicationDbContext> options) : base(options)
    {
    }

    public DbSet<Certificate> Certificates { get; set; }
    public DbSet<RouteDomain> RouteDomains { get; set; }
    public DbSet<RouteEntity> Routes { get; set; }
    public DbSet<RouteProperties> RouteProperties { get; set; }
    public DbSet<UserSettings> UserSettings { get; set; }
    public DbSet<OutboxMessage> OutboxMessages { get; set; }
    public DbSet<Workspace> Workspaces { get; set; }
    public DbSet<UserWorkspace> UserWorkspaces { get; set; }

    protected override void OnModelCreating(ModelBuilder modelBuilder)
    {
        base.OnModelCreating(modelBuilder);

        // Apply all entity configurations from the Configurations folder
        modelBuilder.ApplyConfiguration(new CertificateConfiguration());
        modelBuilder.ApplyConfiguration(new RouteDomainConfiguration());
        modelBuilder.ApplyConfiguration(new RouteConfiguration());
        modelBuilder.ApplyConfiguration(new RoutePropertiesConfiguration());
        modelBuilder.ApplyConfiguration(new UserSettingsConfiguration());
        modelBuilder.ApplyConfiguration(new OutboxMessageConfiguration());
        modelBuilder.ApplyConfiguration(new WorkspaceConfiguration());
        modelBuilder.ApplyConfiguration(new UserWorkspaceConfiguration());
    }
}


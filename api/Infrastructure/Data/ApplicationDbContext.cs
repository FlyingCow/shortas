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
    public DbSet<RouteEntity> Routes { get; set; }
    public DbSet<RouteProperties> RouteProperties { get; set; }
    public DbSet<UserSettings> UserSettings { get; set; }
    public DbSet<OutboxMessage> OutboxMessages { get; set; }

    protected override void OnModelCreating(ModelBuilder modelBuilder)
    {
        base.OnModelCreating(modelBuilder);

        // Apply all entity configurations from the Configurations folder
        modelBuilder.ApplyConfiguration(new CertificateConfiguration());
        modelBuilder.ApplyConfiguration(new RouteConfiguration());
        modelBuilder.ApplyConfiguration(new RoutePropertiesConfiguration());
        modelBuilder.ApplyConfiguration(new UserSettingsConfiguration());
        modelBuilder.ApplyConfiguration(new OutboxMessageConfiguration());
    }
}


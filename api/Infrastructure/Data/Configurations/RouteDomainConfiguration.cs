using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.Metadata.Builders;
using ShortasProxyApi.Domain.Entities;

namespace ShortasProxyApi.Infrastructure.Data.Configurations;

public class RouteDomainConfiguration : IEntityTypeConfiguration<RouteDomain>
{
    public void Configure(EntityTypeBuilder<RouteDomain> builder)
    {
        builder.ToTable("Domains");

        builder.HasKey(d => d.Id);

        builder.Property(d => d.Id)
            .ValueGeneratedOnAdd();

        builder.Property(d => d.Name)
            .IsRequired()
            .HasMaxLength(255);

        builder.Property(d => d.OwnerId)
            .IsRequired()
            .HasMaxLength(255);

        // Add unique index on Name per Owner (each user can have unique domain names)
        builder.HasIndex(d => new { d.OwnerId, d.Name })
            .IsUnique();

        // Add index on OwnerId for performance
        builder.HasIndex(d => d.OwnerId);
    }
}

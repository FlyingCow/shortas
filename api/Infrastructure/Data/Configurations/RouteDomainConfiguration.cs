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

        // Verification status fields
        builder.Property(d => d.VerificationStatus)
            .IsRequired()
            .HasDefaultValue(DomainVerificationStatus.Pending);

        builder.Property(d => d.VerificationReason)
            .IsRequired()
            .HasMaxLength(255)
            .HasDefaultValue("not_checked");

        builder.Property(d => d.LastVerificationCheck);
        builder.Property(d => d.NextVerificationCheck);

        // IsShared flag for shared/common domains
        builder.Property(d => d.IsShared)
            .IsRequired()
            .HasDefaultValue(false);

        // Domain names must be globally unique (shared domains use special owner ID)
        builder.HasIndex(d => d.Name)
            .IsUnique();

        // Add index on OwnerId for performance
        builder.HasIndex(d => d.OwnerId);

        // Add index on VerificationStatus for filtering
        builder.HasIndex(d => d.VerificationStatus);
    }
}

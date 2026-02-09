using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.Metadata.Builders;
using ShortasProxyApi.Domain.Entities;

namespace ShortasProxyApi.Infrastructure.Data.Configurations;

public class DomainRouteCountConfiguration : IEntityTypeConfiguration<DomainRouteCount>
{
    public void Configure(EntityTypeBuilder<DomainRouteCount> builder)
    {
        builder.ToTable("DomainRouteCounts");

        builder.HasKey(d => d.DomainId);

        builder.Property(d => d.DomainId)
            .ValueGeneratedNever();

        builder.Property(d => d.RouteCount)
            .IsRequired()
            .HasDefaultValue(0);

        // Foreign key to Domains table
        builder.HasOne<RouteDomain>()
            .WithOne()
            .HasForeignKey<DomainRouteCount>(d => d.DomainId)
            .OnDelete(DeleteBehavior.Cascade);
    }
}

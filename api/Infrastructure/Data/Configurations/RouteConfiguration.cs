using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.Metadata.Builders;
using RouteEntity = ShortasProxyApi.Domain.Entities.Route;

namespace ShortasProxyApi.Infrastructure.Data.Configurations;

public class RouteConfiguration : IEntityTypeConfiguration<RouteEntity>
{
    public void Configure(EntityTypeBuilder<RouteEntity> builder)
    {
        builder.ToTable("Routes");

        builder.HasKey(r => r.Id);

        builder.Property(r => r.Id)
            .ValueGeneratedOnAdd();

        builder.Property(r => r.Switch)
            .IsRequired()
            .HasMaxLength(255);

        builder.Property(r => r.Link)
            .IsRequired();

        builder.Property(r => r.Dest);  // Nullable - matches click-router Option<String>

        builder.Property(r => r.DestFormat)
            .IsRequired()
            .HasDefaultValue("Http");

        builder.Property(r => r.Code);  // Nullable - matches click-router Option<u16>

        builder.Property(r => r.Ttl);  // Nullable - matches click-router Option<u128>

        builder.Property(r => r.Status)
            .IsRequired()
            .HasDefaultValue("Active");

        builder.Property(r => r.Terminal)
            .IsRequired()
            .HasDefaultValue("External");

        builder.Property(r => r.PolicyJson)
            .HasColumnType("jsonb")
            .HasDefaultValue("\"Basic\"");

        // Ignore computed properties
        builder.Ignore(r => r.Policy);

        // Add indexes for performance
        builder.HasIndex(r => r.Link);
        builder.HasIndex(r => r.Status);
    }
}

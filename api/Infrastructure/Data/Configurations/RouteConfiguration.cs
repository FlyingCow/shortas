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

        builder.Property(r => r.Dest)
            .IsRequired();

        builder.Property(r => r.DestFormat)
            .IsRequired();

        builder.Property(r => r.Code)
            .IsRequired();

        builder.Property(r => r.Ttl)
            .IsRequired();

        builder.Property(r => r.Status)
            .IsRequired();

        builder.Property(r => r.Terminal)
            .IsRequired();

        // Add indexes for performance
        builder.HasIndex(r => r.Link);
        builder.HasIndex(r => r.Status);
    }
}

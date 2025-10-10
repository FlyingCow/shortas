using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.Metadata.Builders;
using ShortasProxyApi.Domain.Entities;

namespace ShortasProxyApi.Infrastructure.Data.Configurations;

public class RoutePropertiesConfiguration : IEntityTypeConfiguration<RouteProperties>
{
    public void Configure(EntityTypeBuilder<RouteProperties> builder)
    {
        builder.ToTable("RouteProperties");

        builder.HasKey(rp => rp.Id);

        builder.Property(rp => rp.Id)
            .ValueGeneratedOnAdd();

        builder.Property(rp => rp.RouteId)
            .IsRequired()
            .HasMaxLength(255);

        builder.Property(rp => rp.DomainId)
            .IsRequired();

        builder.Property(rp => rp.OwnerId)
            .IsRequired();

        builder.Property(rp => rp.ScriptsJson)
            .HasColumnType("jsonb")
            .HasDefaultValue("[]");

        builder.Property(rp => rp.TagsJson)
            .HasColumnType("jsonb")
            .HasDefaultValue("[]");

        builder.Property(rp => rp.CustomJson)
            .HasColumnType("jsonb")
            .HasDefaultValue("{}");

        builder.Property(rp => rp.Opengraph)
            .IsRequired();

        builder.Property(rp => rp.AllowDebug)
            .IsRequired();

        // Ignore computed properties
        builder.Ignore(rp => rp.Scripts);
        builder.Ignore(rp => rp.Tags);
        builder.Ignore(rp => rp.Custom);

        // Add indexes for performance
        builder.HasIndex(rp => rp.OwnerId);
        builder.HasIndex(rp => rp.RouteId);
    }
}

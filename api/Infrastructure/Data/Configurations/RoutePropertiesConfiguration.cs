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

        // Nullable fields to match click-router Option<String>
        builder.Property(rp => rp.RouteId)
            .HasMaxLength(255);

        builder.Property(rp => rp.DomainId)
            .HasMaxLength(255);

        builder.Property(rp => rp.OwnerId)
            .HasMaxLength(255);

        builder.Property(rp => rp.CreatorId)
            .HasMaxLength(255);

        builder.Property(rp => rp.WorkspaceId)
            .HasMaxLength(255);

        // JSON fields
        builder.Property(rp => rp.ScriptsJson)
            .HasColumnType("jsonb")
            .HasDefaultValue("[]");

        builder.Property(rp => rp.TagsJson)
            .HasColumnType("jsonb")
            .HasDefaultValue("[]");

        builder.Property(rp => rp.CustomJson)
            .HasColumnType("jsonb")
            .HasDefaultValue("{}");

        builder.Property(rp => rp.NativeJson)
            .HasColumnType("jsonb")
            .HasDefaultValue("{}");

        builder.Property(rp => rp.BundlingJson)
            .HasColumnType("jsonb")
            .HasDefaultValue("{}");

        builder.Property(rp => rp.Opengraph)
            .IsRequired()
            .HasDefaultValue(false);

        builder.Property(rp => rp.AllowDebug)
            .IsRequired()
            .HasDefaultValue(false);

        // Ignore computed properties
        builder.Ignore(rp => rp.Scripts);
        builder.Ignore(rp => rp.Tags);
        builder.Ignore(rp => rp.Custom);
        builder.Ignore(rp => rp.Native);
        builder.Ignore(rp => rp.Bundling);

        // Add indexes for performance
        builder.HasIndex(rp => rp.OwnerId);
        builder.HasIndex(rp => rp.RouteId);
        builder.HasIndex(rp => rp.WorkspaceId);
    }
}

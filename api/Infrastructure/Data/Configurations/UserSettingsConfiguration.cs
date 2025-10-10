using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.Metadata.Builders;
using ShortasProxyApi.Domain.Entities;

namespace ShortasProxyApi.Infrastructure.Data.Configurations;

public class UserSettingsConfiguration : IEntityTypeConfiguration<UserSettings>
{
    public void Configure(EntityTypeBuilder<UserSettings> builder)
    {
        builder.ToTable("UserSettings");

        builder.HasKey(us => us.Id);

        builder.Property(us => us.Id)
            .ValueGeneratedOnAdd();

        builder.Property(us => us.Email)
            .IsRequired()
            .HasMaxLength(255);

        builder.Property(us => us.Status)
            .IsRequired();

        builder.Property(us => us.Debug)
            .IsRequired();

        builder.Property(us => us.Overflow)
            .IsRequired();

        builder.Property(us => us.SkipTrackingJson)
            .HasColumnType("jsonb")
            .HasDefaultValue("[]");

        builder.Property(us => us.AllowedRequestParamsJson)
            .HasColumnType("jsonb")
            .HasDefaultValue("[]");

        builder.Property(us => us.AllowedDestinationParamsJson)
            .HasColumnType("jsonb")
            .HasDefaultValue("[]");

        // Ignore computed properties
        builder.Ignore(us => us.SkipTracking);
        builder.Ignore(us => us.AllowedRequestParams);
        builder.Ignore(us => us.AllowedDestinationParams);

        // Add index for performance
        builder.HasIndex(us => us.Email)
            .IsUnique();
    }
}

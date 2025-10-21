using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.Metadata.Builders;
using ShortasProxyApi.Domain.Entities;

namespace ShortasProxyApi.Infrastructure.Data.Configurations;

public class UserWorkspaceConfiguration : IEntityTypeConfiguration<UserWorkspace>
{
    public void Configure(EntityTypeBuilder<UserWorkspace> builder)
    {
        builder.ToTable("UserWorkspaces");

        builder.HasKey(uw => uw.Id);

        builder.Property(uw => uw.Id)
            .ValueGeneratedOnAdd();

        builder.Property(uw => uw.UserId)
            .IsRequired()
            .HasMaxLength(255);

        builder.Property(uw => uw.WorkspaceId)
            .IsRequired();

        builder.Property(uw => uw.Role)
            .IsRequired()
            .HasMaxLength(50)
            .HasDefaultValue("Member");

        builder.Property(uw => uw.JoinedAt)
            .IsRequired();

        // Add indexes for performance
        builder.HasIndex(uw => uw.UserId);
        builder.HasIndex(uw => uw.WorkspaceId);
        builder.HasIndex(uw => new { uw.UserId, uw.WorkspaceId })
            .IsUnique();
    }
}

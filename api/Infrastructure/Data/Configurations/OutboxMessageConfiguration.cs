using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.Metadata.Builders;
using ShortasProxyApi.Domain.Entities;

namespace ShortasProxyApi.Infrastructure.Data.Configurations;

public class OutboxMessageConfiguration : IEntityTypeConfiguration<OutboxMessage>
{
    public void Configure(EntityTypeBuilder<OutboxMessage> builder)
    {
        builder.ToTable("OutboxMessages");

        builder.HasKey(om => om.Id);

        builder.Property(om => om.Id)
            .ValueGeneratedOnAdd();

        builder.Property(om => om.EventType)
            .IsRequired()
            .HasMaxLength(50);

        builder.Property(om => om.AggregateId)
            .IsRequired()
            .HasMaxLength(255);

        builder.Property(om => om.Payload)
            .IsRequired()
            .HasColumnType("jsonb")
            .HasDefaultValue("{}");

        builder.Property(om => om.CreatedAt)
            .IsRequired();

        builder.Property(om => om.ProcessedAt)
            .IsRequired(false);

        builder.Property(om => om.Status)
            .IsRequired()
            .HasMaxLength(20)
            .HasDefaultValue(OutboxMessageStatus.Pending);

        builder.Property(om => om.RetryCount)
            .IsRequired()
            .HasDefaultValue(0);

        builder.Property(om => om.MaxRetries)
            .IsRequired()
            .HasDefaultValue(5);

        builder.Property(om => om.ErrorMessage)
            .HasMaxLength(2000)
            .IsRequired(false);

        builder.Property(om => om.NextRetryAt)
            .IsRequired(false);

        // Add indexes for performance (critical for outbox polling)
        builder.HasIndex(om => om.Status);
        builder.HasIndex(om => new { om.Status, om.NextRetryAt });
        builder.HasIndex(om => om.CreatedAt);
    }
}

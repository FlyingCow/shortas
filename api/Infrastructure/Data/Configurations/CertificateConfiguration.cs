using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.Metadata.Builders;
using ShortasProxyApi.Domain.Entities;

namespace ShortasProxyApi.Infrastructure.Data.Configurations;

public class CertificateConfiguration : IEntityTypeConfiguration<Certificate>
{
    public void Configure(EntityTypeBuilder<Certificate> builder)
    {
        builder.ToTable("Certificates");

        builder.HasKey(c => c.Id);

        builder.Property(c => c.Id)
            .ValueGeneratedOnAdd();

        builder.Property(c => c.Key)
            .IsRequired()
            .HasMaxLength(255);

        builder.Property(c => c.Cert)
            .IsRequired();

        builder.Property(c => c.OcspResp)
            .IsRequired(false);

        builder.Property(c => c.OwnerId)
            .IsRequired()
            .HasMaxLength(255);

        builder.Property(c => c.DomainId)
            .IsRequired();

        // Configure relationship with Domain
        builder.HasOne(c => c.Domain)
            .WithMany(d => d.Certificates)
            .HasForeignKey(c => c.DomainId)
            .OnDelete(DeleteBehavior.Restrict);

        // Add indexes for performance
        builder.HasIndex(c => c.Key);
        builder.HasIndex(c => c.OwnerId);
        builder.HasIndex(c => c.DomainId);
        builder.HasIndex(c => new { c.OwnerId, c.DomainId });
    }
}

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

        // Add index for performance
        builder.HasIndex(c => c.Key);
    }
}

using Microsoft.EntityFrameworkCore;
using System.Text.Json;
using ShortasProxyApi.Domain.Common;
using ShortasProxyApi.Domain.Entities;
using ShortasProxyApi.Domain.Interfaces;
using ShortasProxyApi.Infrastructure.Data;

namespace ShortasProxyApi.Infrastructure.Services;

public class EfCertificateService : ICertificateService
{
    private readonly ApplicationDbContext _context;
    private readonly IOutboxRepository _outboxRepository;
    private readonly ILogger<EfCertificateService> _logger;
    private readonly JsonSerializerOptions _jsonOptions;

    public EfCertificateService(
        ApplicationDbContext context,
        IOutboxRepository outboxRepository,
        ILogger<EfCertificateService> logger)
    {
        _context = context;
        _outboxRepository = outboxRepository;
        _logger = logger;
        _jsonOptions = new JsonSerializerOptions
        {
            PropertyNamingPolicy = JsonNamingPolicy.CamelCase
        };
    }

    public async Task<Result<Certificate?>> GetCertificateAsync(Guid domainId, string userId)
    {
        try
        {
            if (string.IsNullOrWhiteSpace(userId))
                return Result<Certificate?>.Failure(Error.Required("userId"));

            var certificate = await _context.Certificates
                .Include(c => c.Domain)
                .FirstOrDefaultAsync(c => c.DomainId == domainId && c.OwnerId == userId);

            if (certificate == null)
                return Result<Certificate?>.Failure(Error.NotFound("Certificate", domainId.ToString()));

            return Result<Certificate?>.Success(certificate);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error getting certificate for domain: {DomainId}", domainId);
            return Result<Certificate?>.Failure(Error.Internal("Failed to get certificate", ex.Message));
        }
    }

    public async Task<Result<Certificate>> CreateCertificateAsync(Certificate certificate, string userId)
    {
        using var transaction = await _context.Database.BeginTransactionAsync();

        try
        {
            if (string.IsNullOrWhiteSpace(userId))
                return Result<Certificate>.Failure(Error.Required("userId"));

            if (certificate == null)
                return Result<Certificate>.Failure(Error.Required("certificate"));

            if (string.IsNullOrWhiteSpace(certificate.Key))
                return Result<Certificate>.Failure(Error.Required("certificate.Key"));

            // Verify the user owns the domain
            var domain = await _context.RouteDomains
                .FirstOrDefaultAsync(d => d.Id == certificate.DomainId);

            if (domain == null)
                return Result<Certificate>.Failure(Error.NotFound("Domain", certificate.DomainId.ToString()));

            if (domain.OwnerId != userId)
                return Result<Certificate>.Failure(Error.Forbidden());

            // Check if certificate already exists for this domain
            var existing = await _context.Certificates
                .FirstOrDefaultAsync(c => c.DomainId == certificate.DomainId);

            if (existing != null)
                return Result<Certificate>.Failure(Error.Conflict("Certificate for this domain already exists"));

            // Set the owner
            certificate.OwnerId = userId;

            // Add certificate to database
            await _context.Certificates.AddAsync(certificate);
            await _context.SaveChangesAsync();

            // Create outbox message for eventual consistency with click-router-api
            var outboxMessage = new OutboxMessage
            {
                EventType = OutboxEventType.CertificateCreated,
                AggregateId = certificate.Id.ToString(),
                Payload = JsonSerializer.Serialize(certificate, _jsonOptions),
                CreatedAt = DateTime.UtcNow,
                Status = OutboxMessageStatus.Pending
            };

            await _outboxRepository.AddAsync(outboxMessage);
            await _outboxRepository.SaveChangesAsync();

            await transaction.CommitAsync();

            _logger.LogInformation("Certificate created: {CertificateId}, DomainId: {DomainId}, OwnerId: {OwnerId}",
                certificate.Id, certificate.DomainId, certificate.OwnerId);

            return Result<Certificate>.Success(certificate);
        }
        catch (Exception ex)
        {
            await transaction.RollbackAsync();
            _logger.LogError(ex, "Error creating certificate for domain: {DomainId}", certificate?.DomainId);
            return Result<Certificate>.Failure(Error.Internal("Failed to create certificate", ex.Message));
        }
    }

    public async Task<Result<Certificate>> UpdateCertificateAsync(Guid id, Certificate certificate, string userId)
    {
        using var transaction = await _context.Database.BeginTransactionAsync();

        try
        {
            if (string.IsNullOrWhiteSpace(userId))
                return Result<Certificate>.Failure(Error.Required("userId"));

            if (certificate == null)
                return Result<Certificate>.Failure(Error.Required("certificate"));

            // Find existing certificate
            var existingCertificate = await _context.Certificates
                .FirstOrDefaultAsync(c => c.Id == id);

            if (existingCertificate == null)
                return Result<Certificate>.Failure(Error.NotFound("Certificate", id.ToString()));

            // Verify ownership
            if (existingCertificate.OwnerId != userId)
                return Result<Certificate>.Failure(Error.Forbidden());

            // Update certificate properties
            existingCertificate.Key = certificate.Key;
            existingCertificate.Cert = certificate.Cert;
            existingCertificate.OcspResp = certificate.OcspResp;

            _context.Certificates.Update(existingCertificate);
            await _context.SaveChangesAsync();

            // Create outbox message
            var outboxMessage = new OutboxMessage
            {
                EventType = OutboxEventType.CertificateUpdated,
                AggregateId = existingCertificate.Id.ToString(),
                Payload = JsonSerializer.Serialize(existingCertificate, _jsonOptions),
                CreatedAt = DateTime.UtcNow,
                Status = OutboxMessageStatus.Pending
            };

            await _outboxRepository.AddAsync(outboxMessage);
            await _outboxRepository.SaveChangesAsync();

            await transaction.CommitAsync();

            _logger.LogInformation("Certificate updated: {CertificateId}, OwnerId: {OwnerId}",
                existingCertificate.Id, existingCertificate.OwnerId);

            return Result<Certificate>.Success(existingCertificate);
        }
        catch (Exception ex)
        {
            await transaction.RollbackAsync();
            _logger.LogError(ex, "Error updating certificate: {CertificateId}", id);
            return Result<Certificate>.Failure(Error.Internal("Failed to update certificate", ex.Message));
        }
    }

    public async Task<Result> DeleteCertificateAsync(Guid id, string userId)
    {
        using var transaction = await _context.Database.BeginTransactionAsync();

        try
        {
            if (string.IsNullOrWhiteSpace(userId))
                return Result.Failure(Error.Required("userId"));

            // Find existing certificate
            var certificate = await _context.Certificates
                .FirstOrDefaultAsync(c => c.Id == id);

            if (certificate == null)
                return Result.Failure(Error.NotFound("Certificate", id.ToString()));

            // Verify ownership
            if (certificate.OwnerId != userId)
                return Result.Failure(Error.Forbidden());

            // Delete certificate
            _context.Certificates.Remove(certificate);
            await _context.SaveChangesAsync();

            // Create outbox message
            var outboxMessage = new OutboxMessage
            {
                EventType = OutboxEventType.CertificateDeleted,
                AggregateId = certificate.Id.ToString(),
                Payload = JsonSerializer.Serialize(new { CertificateId = certificate.Id, DomainId = certificate.DomainId }, _jsonOptions),
                CreatedAt = DateTime.UtcNow,
                Status = OutboxMessageStatus.Pending
            };

            await _outboxRepository.AddAsync(outboxMessage);
            await _outboxRepository.SaveChangesAsync();

            await transaction.CommitAsync();

            _logger.LogInformation("Certificate deleted: {CertificateId}, OwnerId: {OwnerId}",
                certificate.Id, certificate.OwnerId);

            return Result.Success();
        }
        catch (Exception ex)
        {
            await transaction.RollbackAsync();
            _logger.LogError(ex, "Error deleting certificate: {CertificateId}", id);
            return Result.Failure(Error.Internal("Failed to delete certificate", ex.Message));
        }
    }

    public async Task<Result<(List<Certificate> Certificates, int TotalCount)>> ListCertificatesAsync(
        string userId,
        int page = 1,
        int pageSize = 20,
        string? search = null,
        Guid? domainId = null)
    {
        try
        {
            if (string.IsNullOrWhiteSpace(userId))
                return Result<(List<Certificate> Certificates, int TotalCount)>.Failure(Error.Required("userId"));

            var query = _context.Certificates
                .Include(c => c.Domain)
                .Where(c => c.OwnerId == userId)
                .AsQueryable();

            // Apply domain filter
            if (domainId.HasValue)
            {
                query = query.Where(c => c.DomainId == domainId.Value);
            }

            // Apply search filter
            if (!string.IsNullOrWhiteSpace(search))
            {
                query = query.Where(c => c.Key.Contains(search));
            }

            // Get total count
            var totalCount = await query.CountAsync();

            // Apply pagination
            var certificates = await query
                .OrderBy(c => c.Key)
                .Skip((page - 1) * pageSize)
                .Take(pageSize)
                .ToListAsync();

            return Result<(List<Certificate> Certificates, int TotalCount)>.Success((certificates, totalCount));
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error listing certificates");
            return Result<(List<Certificate> Certificates, int TotalCount)>.Failure(
                Error.Internal("Failed to list certificates", ex.Message));
        }
    }
}

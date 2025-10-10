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

    public async Task<Result<Certificate?>> GetCertificateAsync(string domain)
    {
        try
        {
            if (string.IsNullOrWhiteSpace(domain))
                return Result<Certificate?>.Failure(Error.Required("domain"));

            var certificate = await _context.Certificates
                .FirstOrDefaultAsync(c => c.Key == domain);

            return Result<Certificate?>.Success(certificate);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error getting certificate for domain: {Domain}", domain);
            return Result<Certificate?>.Failure(Error.Internal("Failed to get certificate", ex.Message));
        }
    }

    public async Task<Result<Certificate>> CreateCertificateAsync(string domain, Certificate certificate)
    {
        using var transaction = await _context.Database.BeginTransactionAsync();

        try
        {
            if (string.IsNullOrWhiteSpace(domain))
                return Result<Certificate>.Failure(Error.Required("domain"));

            if (certificate == null)
                return Result<Certificate>.Failure(Error.Required("certificate"));

            // Check if certificate already exists
            var existing = await _context.Certificates
                .FirstOrDefaultAsync(c => c.Key == domain);

            if (existing != null)
                return Result<Certificate>.Failure(Error.Conflict("Certificate for this domain already exists"));

            // Set the domain key
            certificate.Key = domain;

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

            _logger.LogInformation("Certificate created: {CertificateId}, Domain: {Domain}", certificate.Id, domain);

            return Result<Certificate>.Success(certificate);
        }
        catch (Exception ex)
        {
            await transaction.RollbackAsync();
            _logger.LogError(ex, "Error creating certificate for domain: {Domain}", domain);
            return Result<Certificate>.Failure(Error.Internal("Failed to create certificate", ex.Message));
        }
    }

    public async Task<Result<Certificate>> UpdateCertificateAsync(string domain, Certificate certificate)
    {
        using var transaction = await _context.Database.BeginTransactionAsync();

        try
        {
            if (string.IsNullOrWhiteSpace(domain))
                return Result<Certificate>.Failure(Error.Required("domain"));

            if (certificate == null)
                return Result<Certificate>.Failure(Error.Required("certificate"));

            // Find existing certificate
            var existingCertificate = await _context.Certificates
                .FirstOrDefaultAsync(c => c.Key == domain);

            if (existingCertificate == null)
                return Result<Certificate>.Failure(Error.NotFound("Certificate", domain));

            // Update certificate properties
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

            _logger.LogInformation("Certificate updated: {CertificateId}, Domain: {Domain}", existingCertificate.Id, domain);

            return Result<Certificate>.Success(existingCertificate);
        }
        catch (Exception ex)
        {
            await transaction.RollbackAsync();
            _logger.LogError(ex, "Error updating certificate for domain: {Domain}", domain);
            return Result<Certificate>.Failure(Error.Internal("Failed to update certificate", ex.Message));
        }
    }

    public async Task<Result> DeleteCertificateAsync(string domain)
    {
        using var transaction = await _context.Database.BeginTransactionAsync();

        try
        {
            if (string.IsNullOrWhiteSpace(domain))
                return Result.Failure(Error.Required("domain"));

            // Find existing certificate
            var certificate = await _context.Certificates
                .FirstOrDefaultAsync(c => c.Key == domain);

            if (certificate == null)
                return Result.Failure(Error.NotFound("Certificate", domain));

            // Delete certificate
            _context.Certificates.Remove(certificate);
            await _context.SaveChangesAsync();

            // Create outbox message
            var outboxMessage = new OutboxMessage
            {
                EventType = OutboxEventType.CertificateDeleted,
                AggregateId = certificate.Id.ToString(),
                Payload = JsonSerializer.Serialize(new { Domain = domain, CertificateId = certificate.Id }, _jsonOptions),
                CreatedAt = DateTime.UtcNow,
                Status = OutboxMessageStatus.Pending
            };

            await _outboxRepository.AddAsync(outboxMessage);
            await _outboxRepository.SaveChangesAsync();

            await transaction.CommitAsync();

            _logger.LogInformation("Certificate deleted: {CertificateId}, Domain: {Domain}", certificate.Id, domain);

            return Result.Success();
        }
        catch (Exception ex)
        {
            await transaction.RollbackAsync();
            _logger.LogError(ex, "Error deleting certificate for domain: {Domain}", domain);
            return Result.Failure(Error.Internal("Failed to delete certificate", ex.Message));
        }
    }

    public async Task<Result<(List<Certificate> Certificates, int TotalCount)>> ListCertificatesAsync(
        int page = 1,
        int pageSize = 20,
        string? search = null)
    {
        try
        {
            var query = _context.Certificates.AsQueryable();

            // Apply search filter
            if (!string.IsNullOrWhiteSpace(search))
            {
                query = query.Where(c => c.Key.Contains(search));
            }

            // Get total count
            var totalCount = await query.CountAsync();

            // Apply pagination
            var certificates = await query
                .OrderByDescending(c => c.Id)
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

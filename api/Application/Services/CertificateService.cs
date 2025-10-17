using ShortasProxyApi.Domain.Entities;
using ShortasProxyApi.Domain.Interfaces;
using ShortasProxyApi.Domain.Common;
using System.Text.Json;

namespace ShortasProxyApi.Application.Services;

public class CertificateService : ICertificateService
{
    private readonly HttpClient _httpClient;
    private readonly ILogger<CertificateService> _logger;
    private readonly JsonSerializerOptions _jsonOptions;

    public CertificateService(HttpClient httpClient, ILogger<CertificateService> logger)
    {
        _httpClient = httpClient;
        _logger = logger;
        _jsonOptions = new JsonSerializerOptions
        {
            PropertyNamingPolicy = JsonNamingPolicy.CamelCase,
            PropertyNameCaseInsensitive = true
        };
    }

    public async Task<Result<Domain.Entities.Certificate?>> GetCertificateAsync(Guid domainId, string userId)
    {
        // This method is not implemented in the HTTP client proxy service
        // The external API uses domain name strings, not internal domain IDs
        // It should only be called when using the EF-based service
        throw new NotImplementedException("GetCertificateAsync with domainId is only available in EF-based service");
    }

    public async Task<Result<Domain.Entities.Certificate>> CreateCertificateAsync(Domain.Entities.Certificate certificate, string userId)
    {
        // This method is not implemented in the HTTP client proxy service
        // The external API uses domain name strings, not internal domain IDs
        // It should only be called when using the EF-based service
        throw new NotImplementedException("CreateCertificateAsync is only available in EF-based service");
    }

    public async Task<Result<Domain.Entities.Certificate>> UpdateCertificateAsync(Guid id, Domain.Entities.Certificate certificate, string userId)
    {
        // This method is not implemented in the HTTP client proxy service
        // The external API uses domain name strings, not internal domain IDs
        // It should only be called when using the EF-based service
        throw new NotImplementedException("UpdateCertificateAsync is only available in EF-based service");
    }

    public async Task<Result> DeleteCertificateAsync(Guid id, string userId)
    {
        // This method is not implemented in the HTTP client proxy service
        // The external API uses domain name strings, not internal domain IDs
        // It should only be called when using the EF-based service
        throw new NotImplementedException("DeleteCertificateAsync is only available in EF-based service");
    }

    public Task<Result<(List<Domain.Entities.Certificate> Certificates, int TotalCount)>> ListCertificatesAsync(
        string userId,
        int page = 1,
        int pageSize = 20,
        string? search = null,
        Guid? domainId = null)
    {
        // This method is not implemented in the HTTP client proxy service
        // It should only be called when using the EF-based service
        throw new NotImplementedException("ListCertificatesAsync is only available in EF-based service");
    }
}
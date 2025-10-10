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

    public async Task<Result<Domain.Entities.Certificate?>> GetCertificateAsync(string domain)
    {
        try
        {
            if (string.IsNullOrWhiteSpace(domain))
                return Result<Domain.Entities.Certificate?>.Failure(Error.Required("domain"));

            var response = await _httpClient.GetAsync($"/v1/certificates/{domain}");
            
            if (response.IsSuccessStatusCode)
            {
                var content = await response.Content.ReadAsStringAsync();
                var certificate = JsonSerializer.Deserialize<Domain.Entities.Certificate>(content, _jsonOptions);
                return Result<Domain.Entities.Certificate?>.Success(certificate);
            }
            
            if (response.StatusCode == System.Net.HttpStatusCode.NotFound)
                return Result<Domain.Entities.Certificate?>.Success(null);
                
            if (response.StatusCode == System.Net.HttpStatusCode.Unauthorized)
                return Result<Domain.Entities.Certificate?>.Failure(Error.Unauthorized());

            if (response.StatusCode == System.Net.HttpStatusCode.Forbidden)
                return Result<Domain.Entities.Certificate?>.Failure(Error.Forbidden());

            return Result<Domain.Entities.Certificate?>.Failure(
                Error.ExternalService("ClickRouterApi", $"HTTP {response.StatusCode}: {response.ReasonPhrase}"));
        }
        catch (HttpRequestException ex)
        {
            _logger.LogError(ex, "HTTP error getting certificate for domain: {Domain}", domain);
            return Result<Domain.Entities.Certificate?>.Failure(Error.Network(ex.Message));
        }
        catch (TaskCanceledException ex) when (ex.InnerException is TimeoutException)
        {
            _logger.LogError(ex, "Timeout getting certificate for domain: {Domain}", domain);
            return Result<Domain.Entities.Certificate?>.Failure(Error.Timeout("ClickRouterApi", 30));
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Unexpected error getting certificate for domain: {Domain}", domain);
            return Result<Domain.Entities.Certificate?>.Failure(Error.Internal("An unexpected error occurred", ex.Message));
        }
    }

    public async Task<Result<Domain.Entities.Certificate>> CreateCertificateAsync(string domain, Domain.Entities.Certificate certificate)
    {
        try
        {
            if (string.IsNullOrWhiteSpace(domain))
                return Result<Domain.Entities.Certificate>.Failure(Error.Required("domain"));

            var validationResult = certificate.Validate();
            if (!validationResult.IsValid)
            {
                var errors = string.Join(", ", validationResult.Errors.Select(e => $"{e.FieldName}: {e.Message}"));
                return Result<Domain.Entities.Certificate>.Failure(Error.Validation("Certificate validation failed", errors));
            }

            var json = JsonSerializer.Serialize(certificate, _jsonOptions);
            var content = new StringContent(json, System.Text.Encoding.UTF8, "application/json");
            
            var response = await _httpClient.PostAsync($"/v1/certificates/{domain}", content);
            
            if (response.IsSuccessStatusCode)
            {
                var responseContent = await response.Content.ReadAsStringAsync();
                var createdCertificate = JsonSerializer.Deserialize<Domain.Entities.Certificate>(responseContent, _jsonOptions) ?? certificate;
                return Result<Domain.Entities.Certificate>.Success(createdCertificate);
            }

            if (response.StatusCode == System.Net.HttpStatusCode.Unauthorized)
                return Result<Domain.Entities.Certificate>.Failure(Error.Unauthorized());

            if (response.StatusCode == System.Net.HttpStatusCode.Forbidden)
                return Result<Domain.Entities.Certificate>.Failure(Error.Forbidden());

            if (response.StatusCode == System.Net.HttpStatusCode.Conflict)
                return Result<Domain.Entities.Certificate>.Failure(Error.Conflict("Certificate already exists for this domain"));

            return Result<Domain.Entities.Certificate>.Failure(
                Error.ExternalService("ClickRouterApi", $"HTTP {response.StatusCode}: {response.ReasonPhrase}"));
        }
        catch (HttpRequestException ex)
        {
            _logger.LogError(ex, "HTTP error creating certificate for domain: {Domain}", domain);
            return Result<Domain.Entities.Certificate>.Failure(Error.Network(ex.Message));
        }
        catch (TaskCanceledException ex) when (ex.InnerException is TimeoutException)
        {
            _logger.LogError(ex, "Timeout creating certificate for domain: {Domain}", domain);
            return Result<Domain.Entities.Certificate>.Failure(Error.Timeout("ClickRouterApi", 30));
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Unexpected error creating certificate for domain: {Domain}", domain);
            return Result<Domain.Entities.Certificate>.Failure(Error.Internal("An unexpected error occurred", ex.Message));
        }
    }

    public async Task<Result<Domain.Entities.Certificate>> UpdateCertificateAsync(string domain, Domain.Entities.Certificate certificate)
    {
        try
        {
            if (string.IsNullOrWhiteSpace(domain))
                return Result<Domain.Entities.Certificate>.Failure(Error.Required("domain"));

            var validationResult = certificate.Validate();
            if (!validationResult.IsValid)
            {
                var errors = string.Join(", ", validationResult.Errors.Select(e => $"{e.FieldName}: {e.Message}"));
                return Result<Domain.Entities.Certificate>.Failure(Error.Validation("Certificate validation failed", errors));
            }

            var json = JsonSerializer.Serialize(certificate, _jsonOptions);
            var content = new StringContent(json, System.Text.Encoding.UTF8, "application/json");
            
            var response = await _httpClient.PutAsync($"/v1/certificates/{domain}", content);
            
            if (response.IsSuccessStatusCode)
            {
                var responseContent = await response.Content.ReadAsStringAsync();
                var updatedCertificate = JsonSerializer.Deserialize<Domain.Entities.Certificate>(responseContent, _jsonOptions) ?? certificate;
                return Result<Domain.Entities.Certificate>.Success(updatedCertificate);
            }

            if (response.StatusCode == System.Net.HttpStatusCode.NotFound)
                return Result<Domain.Entities.Certificate>.Failure(Error.NotFound("Certificate", domain));

            if (response.StatusCode == System.Net.HttpStatusCode.Unauthorized)
                return Result<Domain.Entities.Certificate>.Failure(Error.Unauthorized());

            if (response.StatusCode == System.Net.HttpStatusCode.Forbidden)
                return Result<Domain.Entities.Certificate>.Failure(Error.Forbidden());

            return Result<Domain.Entities.Certificate>.Failure(
                Error.ExternalService("ClickRouterApi", $"HTTP {response.StatusCode}: {response.ReasonPhrase}"));
        }
        catch (HttpRequestException ex)
        {
            _logger.LogError(ex, "HTTP error updating certificate for domain: {Domain}", domain);
            return Result<Domain.Entities.Certificate>.Failure(Error.Network(ex.Message));
        }
        catch (TaskCanceledException ex) when (ex.InnerException is TimeoutException)
        {
            _logger.LogError(ex, "Timeout updating certificate for domain: {Domain}", domain);
            return Result<Domain.Entities.Certificate>.Failure(Error.Timeout("ClickRouterApi", 30));
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Unexpected error updating certificate for domain: {Domain}", domain);
            return Result<Domain.Entities.Certificate>.Failure(Error.Internal("An unexpected error occurred", ex.Message));
        }
    }

    public async Task<Result> DeleteCertificateAsync(string domain)
    {
        try
        {
            if (string.IsNullOrWhiteSpace(domain))
                return Result.Failure(Error.Required("domain"));

            var response = await _httpClient.DeleteAsync($"/v1/certificates/{domain}");
            
            if (response.IsSuccessStatusCode)
                return Result.Success();

            if (response.StatusCode == System.Net.HttpStatusCode.NotFound)
                return Result.Failure(Error.NotFound("Certificate", domain));

            if (response.StatusCode == System.Net.HttpStatusCode.Unauthorized)
                return Result.Failure(Error.Unauthorized());

            if (response.StatusCode == System.Net.HttpStatusCode.Forbidden)
                return Result.Failure(Error.Forbidden());

            return Result.Failure(
                Error.ExternalService("ClickRouterApi", $"HTTP {response.StatusCode}: {response.ReasonPhrase}"));
        }
        catch (HttpRequestException ex)
        {
            _logger.LogError(ex, "HTTP error deleting certificate for domain: {Domain}", domain);
            return Result.Failure(Error.Network(ex.Message));
        }
        catch (TaskCanceledException ex) when (ex.InnerException is TimeoutException)
        {
            _logger.LogError(ex, "Timeout deleting certificate for domain: {Domain}", domain);
            return Result.Failure(Error.Timeout("ClickRouterApi", 30));
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Unexpected error deleting certificate for domain: {Domain}", domain);
            return Result.Failure(Error.Internal("An unexpected error occurred", ex.Message));
        }
    }

    public Task<Result<(List<Domain.Entities.Certificate> Certificates, int TotalCount)>> ListCertificatesAsync(
        int page = 1,
        int pageSize = 20,
        string? search = null)
    {
        // This method is not implemented in the HTTP client proxy service
        // It should only be called when using the EF-based service
        throw new NotImplementedException("ListCertificatesAsync is only available in EF-based service");
    }
}
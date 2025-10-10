using ShortasProxyApi.Domain.Entities;
using ShortasProxyApi.Domain.Common;

namespace ShortasProxyApi.Domain.Interfaces;

public interface ICertificateService
{
    Task<Result<Entities.Certificate?>> GetCertificateAsync(string domain);
    Task<Result<Entities.Certificate>> CreateCertificateAsync(string domain, Entities.Certificate certificate);
    Task<Result<Entities.Certificate>> UpdateCertificateAsync(string domain, Entities.Certificate certificate);
    Task<Result> DeleteCertificateAsync(string domain);
    Task<Result<(List<Entities.Certificate> Certificates, int TotalCount)>> ListCertificatesAsync(
        int page = 1,
        int pageSize = 20,
        string? search = null);
}
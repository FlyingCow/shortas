using ShortasProxyApi.Domain.Entities;
using ShortasProxyApi.Domain.Common;

namespace ShortasProxyApi.Domain.Interfaces;

public interface ICertificateService
{
    Task<Result<Entities.Certificate?>> GetCertificateAsync(Guid domainId, string userId);
    Task<Result<Entities.Certificate>> CreateCertificateAsync(Entities.Certificate certificate, string userId);
    Task<Result<Entities.Certificate>> UpdateCertificateAsync(Guid id, Entities.Certificate certificate, string userId);
    Task<Result> DeleteCertificateAsync(Guid id, string userId);
    Task<Result<(List<Entities.Certificate> Certificates, int TotalCount)>> ListCertificatesAsync(
        string userId,
        int page = 1,
        int pageSize = 20,
        string? search = null,
        Guid? domainId = null);
}
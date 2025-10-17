using ShortasProxyApi.Domain.Entities;
using ShortasProxyApi.Domain.Common;

namespace ShortasProxyApi.Domain.Interfaces;

public interface IDomainService
{
    Task<Result<RouteDomain?>> GetDomainByIdAsync(Guid id, string userId);
    Task<Result<RouteDomain?>> GetDomainByNameAsync(string name, string userId);
    Task<Result<RouteDomain>> CreateDomainAsync(RouteDomain domain, string userId);
    Task<Result<RouteDomain>> UpdateDomainAsync(Guid id, RouteDomain domain, string userId);
    Task<Result> DeleteDomainAsync(Guid id, string userId);
    Task<Result<(List<RouteDomain> Domains, int TotalCount)>> ListDomainsAsync(
        string userId,
        int page = 1,
        int pageSize = 20,
        string? search = null);
}

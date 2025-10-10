using ShortasProxyApi.Domain.Interfaces;
using ShortasProxyApi.Application.Services;
using ShortasProxyApi.Domain.Common;

namespace ShortasProxyApi.Infrastructure.HttpClients;

public class ClickRouterApiClient : IRouteService, ICertificateService, IUserSettingsService
{
    private readonly RouteService _routeService;
    private readonly CertificateService _certificateService;
    private readonly UserSettingsService _userSettingsService;

    public ClickRouterApiClient(RouteService routeService, CertificateService certificateService, UserSettingsService userSettingsService)
    {
        _routeService = routeService;
        _certificateService = certificateService;
        _userSettingsService = userSettingsService;
    }

    // Route Service Methods
    public Task<Result<Domain.Entities.Route?>> GetRouteAsync(string domain, string path, string userId, string? switchParam = null)
        => _routeService.GetRouteAsync(domain, path, userId, switchParam);

    public Task<Result<Domain.Entities.Route>> CreateRouteAsync(Domain.Entities.Route route)
        => _routeService.CreateRouteAsync(route);

    public Task<Result<Domain.Entities.Route>> UpdateRouteAsync(string domain, string path, string userId, Domain.Entities.Route route)
        => _routeService.UpdateRouteAsync(domain, path, userId, route);

    public Task<Result> DeleteRouteAsync(string domain, string path, string userId)
        => _routeService.DeleteRouteAsync(domain, path, userId);

    public Task<Result<List<Domain.Entities.Route>>> BulkCreateRoutesAsync(List<Domain.Entities.Route> routes)
        => _routeService.BulkCreateRoutesAsync(routes);

    public Task<Result<List<Domain.Entities.Route>>> BulkUpdateRoutesAsync(string userId, List<Domain.Entities.Route> routes)
        => _routeService.BulkUpdateRoutesAsync(userId, routes);

    public Task<Result> BulkDeleteRoutesAsync(string userId, List<string> routeIds)
        => _routeService.BulkDeleteRoutesAsync(userId, routeIds);

    public Task<Result<(List<Domain.Entities.Route> Routes, int TotalCount)>> ListRoutesAsync(
        int page = 1,
        int pageSize = 20,
        string? search = null,
        string? status = null,
        string? ownerId = null)
        => _routeService.ListRoutesAsync(page, pageSize, search, status, ownerId);

    // Certificate Service Methods
    public Task<Result<Domain.Entities.Certificate?>> GetCertificateAsync(string domain)
        => _certificateService.GetCertificateAsync(domain);

    public Task<Result<Domain.Entities.Certificate>> CreateCertificateAsync(string domain, Domain.Entities.Certificate certificate)
        => _certificateService.CreateCertificateAsync(domain, certificate);

    public Task<Result<Domain.Entities.Certificate>> UpdateCertificateAsync(string domain, Domain.Entities.Certificate certificate)
        => _certificateService.UpdateCertificateAsync(domain, certificate);

    public Task<Result> DeleteCertificateAsync(string domain)
        => _certificateService.DeleteCertificateAsync(domain);

    public Task<Result<(List<Domain.Entities.Certificate> Certificates, int TotalCount)>> ListCertificatesAsync(
        int page = 1,
        int pageSize = 20,
        string? search = null)
        => _certificateService.ListCertificatesAsync(page, pageSize, search);

    // User Settings Service Methods
    public Task<Result<Domain.Entities.UserSettings?>> GetUserSettingsAsync(string userId)
        => _userSettingsService.GetUserSettingsAsync(userId);

    public Task<Result<Domain.Entities.UserSettings>> CreateUserSettingsAsync(string userId, Domain.Entities.UserSettings settings)
        => _userSettingsService.CreateUserSettingsAsync(userId, settings);

    public Task<Result<Domain.Entities.UserSettings>> UpdateUserSettingsAsync(string userId, Domain.Entities.UserSettings settings)
        => _userSettingsService.UpdateUserSettingsAsync(userId, settings);

    public Task<Result> DeleteUserSettingsAsync(string userId)
        => _userSettingsService.DeleteUserSettingsAsync(userId);
}
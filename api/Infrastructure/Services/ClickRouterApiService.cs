using ShortasProxyApi.Domain.Entities;
using ShortasProxyApi.Domain.Interfaces;
using ShortasProxyApi.Domain.Common;
using ShortasProxyApi.Infrastructure.HttpClients;
using Route = ShortasProxyApi.Domain.Entities.Route;

namespace ShortasProxyApi.Infrastructure.Services;

/// <summary>
/// Service implementation that uses ClickRouterApiClient for HTTP communication.
/// This service implements the domain service interfaces and delegates to the HTTP client.
/// </summary>
public class ClickRouterApiService : IRouteService, ICertificateService, IUserSettingsService
{
    private readonly ClickRouterApiClient _httpClient;
    private readonly ILogger<ClickRouterApiService> _logger;

    public ClickRouterApiService(ClickRouterApiClient httpClient, ILogger<ClickRouterApiService> logger)
    {
        _httpClient = httpClient;
        _logger = logger;
    }

    #region IRouteService Implementation

    public async Task<Result<Route?>> GetRouteByIdAsync(Guid id, string userId)
    {
        _logger.LogDebug("Getting route by ID {RouteId} for user {UserId}", id, userId);
        return await _httpClient.GetRouteByIdAsync(id, userId);
    }

    public async Task<Result<Route>> UpdateRouteByIdAsync(Guid id, string userId, Route route)
    {
        _logger.LogDebug("Updating route by ID {RouteId} for user {UserId}", id, userId);
        return await _httpClient.UpdateRouteByIdAsync(id, userId, route);
    }

    public async Task<Result> DeleteRouteByIdAsync(Guid id, string userId)
    {
        _logger.LogDebug("Deleting route by ID {RouteId} for user {UserId}", id, userId);
        return await _httpClient.DeleteRouteByIdAsync(id, userId);
    }

    public async Task<Result<Route?>> GetRouteAsync(string domain, string path, string userId, string? switchParam = null)
    {
        _logger.LogDebug("Getting route {Domain}/{Path} for user {UserId}", domain, path, userId);
        return await _httpClient.GetRouteAsync(domain, path, userId, switchParam);
    }

    public async Task<Result<Route>> CreateRouteAsync(Route route)
    {
        _logger.LogDebug("Creating route {Link}", route.Link);
        return await _httpClient.CreateRouteAsync(route);
    }

    public async Task<Result<Route>> UpdateRouteAsync(string domain, string path, string userId, Route route)
    {
        _logger.LogDebug("Updating route {Domain}/{Path} for user {UserId}", domain, path, userId);
        return await _httpClient.UpdateRouteAsync(domain, path, userId, route);
    }

    public async Task<Result> DeleteRouteAsync(string domain, string path, string userId, string? switchParam = null)
    {
        _logger.LogDebug("Deleting route {Domain}/{Path} for user {UserId}", domain, path, userId);
        return await _httpClient.DeleteRouteAsync(domain, path, userId);
    }

    public async Task<Result<List<Route>>> BulkCreateRoutesAsync(List<Route> routes)
    {
        _logger.LogDebug("Bulk creating {Count} routes", routes.Count);
        return await _httpClient.BulkCreateRoutesAsync(routes);
    }

    public async Task<Result<List<Route>>> BulkUpdateRoutesAsync(string userId, List<Route> routes)
    {
        _logger.LogDebug("Bulk updating {Count} routes for user {UserId}", routes.Count, userId);
        return await _httpClient.BulkUpdateRoutesAsync(userId, routes);
    }

    public async Task<Result> BulkDeleteRoutesAsync(string userId, List<string> routeIds)
    {
        _logger.LogDebug("Bulk deleting {Count} routes for user {UserId}", routeIds.Count, userId);
        return await _httpClient.BulkDeleteRoutesAsync(userId, routeIds);
    }

    public async Task<Result<(List<Route> Routes, int TotalCount)>> ListRoutesAsync(
        int page = 1,
        int pageSize = 20,
        string? search = null,
        string? status = null,
        string? ownerId = null)
    {
        _logger.LogDebug("Listing routes - page {Page}, pageSize {PageSize}, search {Search}, status {Status}, ownerId {OwnerId}", 
            page, pageSize, search, status, ownerId);
        return await _httpClient.ListRoutesAsync(page, pageSize, search, status, ownerId);
    }

    #endregion

    #region ICertificateService Implementation

    public Task<Result<Certificate?>> GetCertificateAsync(Guid domainId, string userId)
    {
        // This method is not implemented in the HTTP client proxy service
        // The external API uses domain name strings, not internal domain IDs
        // It should only be called when using the EF-based service
        throw new NotImplementedException("GetCertificateAsync with domainId is only available in EF-based service");
    }

    public Task<Result<Certificate>> CreateCertificateAsync(Certificate certificate, string userId)
    {
        // This method is not implemented in the HTTP client proxy service
        // The external API uses domain name strings, not internal domain IDs
        // It should only be called when using the EF-based service
        throw new NotImplementedException("CreateCertificateAsync is only available in EF-based service");
    }

    public Task<Result<Certificate>> UpdateCertificateAsync(Guid id, Certificate certificate, string userId)
    {
        // This method is not implemented in the HTTP client proxy service
        // The external API uses domain name strings, not internal domain IDs
        // It should only be called when using the EF-based service
        throw new NotImplementedException("UpdateCertificateAsync is only available in EF-based service");
    }

    public Task<Result> DeleteCertificateAsync(Guid id, string userId)
    {
        // This method is not implemented in the HTTP client proxy service
        // The external API uses domain name strings, not internal domain IDs
        // It should only be called when using the EF-based service
        throw new NotImplementedException("DeleteCertificateAsync is only available in EF-based service");
    }

    public Task<Result<(List<Certificate> Certificates, int TotalCount)>> ListCertificatesAsync(
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

    #endregion

    #region IUserSettingsService Implementation

    public async Task<Result<UserSettings?>> GetUserSettingsAsync(string userId)
    {
        _logger.LogDebug("Getting user settings for user {UserId}", userId);
        return await _httpClient.GetUserSettingsAsync(userId);
    }

    public async Task<Result<UserSettings>> CreateUserSettingsAsync(string userId, UserSettings settings)
    {
        _logger.LogDebug("Creating user settings for user {UserId}", userId);
        return await _httpClient.CreateUserSettingsAsync(userId, settings);
    }

    public async Task<Result<UserSettings>> UpdateUserSettingsAsync(string userId, UserSettings settings)
    {
        _logger.LogDebug("Updating user settings for user {UserId}", userId);
        return await _httpClient.UpdateUserSettingsAsync(userId, settings);
    }

    public async Task<Result> DeleteUserSettingsAsync(string userId)
    {
        _logger.LogDebug("Deleting user settings for user {UserId}", userId);
        return await _httpClient.DeleteUserSettingsAsync(userId);
    }

    #endregion
}

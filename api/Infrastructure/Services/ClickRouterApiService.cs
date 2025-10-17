using ShortasProxyApi.Domain.Entities;
using ShortasProxyApi.Domain.Interfaces;
using ShortasProxyApi.Domain.Common;
using ShortasProxyApi.Infrastructure.HttpClients;
using ShortasProxyApi.Application.Extensions;
using Route = ShortasProxyApi.Domain.Entities.Route;

namespace ShortasProxyApi.Infrastructure.Services;

/// <summary>
/// Service implementation that uses ClickRouterApiClient for HTTP communication.
/// This service implements the domain service interfaces and converts between entities and DTOs.
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
        var result = await _httpClient.GetRouteByIdAsync(id, userId);
        return result.IsSuccess && result.Value != null
            ? Result<Route?>.Success(result.Value.ToEntity())
            : Result<Route?>.Failure(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
    }

    public async Task<Result<Route>> UpdateRouteByIdAsync(Guid id, string userId, Route route)
    {
        _logger.LogDebug("Updating route by ID {RouteId} for user {UserId}", id, userId);
        var dto = route.ToDto();
        var result = await _httpClient.UpdateRouteByIdAsync(id, userId, dto);
        return result.IsSuccess
            ? Result<Route>.Success(result.Value.ToEntity())
            : Result<Route>.Failure(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
    }

    public async Task<Result> DeleteRouteByIdAsync(Guid id, string userId)
    {
        _logger.LogDebug("Deleting route by ID {RouteId} for user {UserId}", id, userId);
        return await _httpClient.DeleteRouteByIdAsync(id, userId);
    }

    public async Task<Result<Route?>> GetRouteAsync(string domain, string path, string userId, string? switchParam = null)
    {
        _logger.LogDebug("Getting route {Domain}/{Path} for user {UserId}", domain, path, userId);
        var result = await _httpClient.GetRouteAsync(domain, path, userId, switchParam);
        return result.IsSuccess && result.Value != null
            ? Result<Route?>.Success(result.Value.ToEntity())
            : Result<Route?>.Failure(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
    }

    public async Task<Result<Route>> CreateRouteAsync(Route route)
    {
        _logger.LogDebug("Creating route {Link}", route.Link);
        var dto = route.ToDto();
        var domain = route.Domain?.Name ?? "";

        if (string.IsNullOrEmpty(domain))
        {
            return Result<Route>.Failure("VALIDATION_ERROR", "Domain is required for route creation");
        }

        var result = await _httpClient.CreateRouteAsync(dto, domain);
        return result.IsSuccess
            ? Result<Route>.Success(result.Value.ToEntity())
            : Result<Route>.Failure(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
    }

    public async Task<Result<Route>> UpdateRouteAsync(string domain, string path, string userId, Route route)
    {
        _logger.LogDebug("Updating route {Domain}/{Path} for user {UserId}", domain, path, userId);
        var dto = route.ToDto();
        var result = await _httpClient.UpdateRouteAsync(domain, path, userId, dto);
        return result.IsSuccess
            ? Result<Route>.Success(result.Value.ToEntity())
            : Result<Route>.Failure(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
    }

    public async Task<Result> DeleteRouteAsync(string domain, string path, string userId, string? switchParam = null)
    {
        _logger.LogDebug("Deleting route {Domain}/{Path} for user {UserId}", domain, path, userId);
        return await _httpClient.DeleteRouteAsync(domain, path, userId);
    }

    public async Task<Result<List<Route>>> BulkCreateRoutesAsync(List<Route> routes)
    {
        _logger.LogDebug("Bulk creating {Count} routes", routes.Count);
        var dtos = routes.Select(r => r.ToDto()).ToList();
        var result = await _httpClient.BulkCreateRoutesAsync(dtos);
        return result.IsSuccess
            ? Result<List<Route>>.Success(result.Value.Select(dto => dto.ToEntity()).ToList())
            : Result<List<Route>>.Failure(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
    }

    public async Task<Result<List<Route>>> BulkUpdateRoutesAsync(string userId, List<Route> routes)
    {
        _logger.LogDebug("Bulk updating {Count} routes for user {UserId}", routes.Count, userId);
        var dtos = routes.Select(r => r.ToDto()).ToList();
        var result = await _httpClient.BulkUpdateRoutesAsync(userId, dtos);
        return result.IsSuccess
            ? Result<List<Route>>.Success(result.Value.Select(dto => dto.ToEntity()).ToList())
            : Result<List<Route>>.Failure(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
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
        var result = await _httpClient.ListRoutesAsync(page, pageSize, search, status, ownerId);
        return result.IsSuccess
            ? Result<(List<Route>, int)>.Success((result.Value.Routes.Select(dto => dto.ToEntity()).ToList(), result.Value.TotalCount))
            : Result<(List<Route>, int)>.Failure(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
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
        var result = await _httpClient.GetUserSettingsAsync(userId);
        return result.IsSuccess && result.Value != null
            ? Result<UserSettings?>.Success(result.Value.ToEntity())
            : Result<UserSettings?>.Failure(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
    }

    public async Task<Result<UserSettings>> CreateUserSettingsAsync(string userId, UserSettings settings)
    {
        _logger.LogDebug("Creating user settings for user {UserId}", userId);
        var dto = settings.ToDto();
        var result = await _httpClient.CreateUserSettingsAsync(userId, dto);
        return result.IsSuccess
            ? Result<UserSettings>.Success(result.Value.ToEntity())
            : Result<UserSettings>.Failure(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
    }

    public async Task<Result<UserSettings>> UpdateUserSettingsAsync(string userId, UserSettings settings)
    {
        _logger.LogDebug("Updating user settings for user {UserId}", userId);
        var dto = settings.ToDto();
        var result = await _httpClient.UpdateUserSettingsAsync(userId, dto);
        return result.IsSuccess
            ? Result<UserSettings>.Success(result.Value.ToEntity())
            : Result<UserSettings>.Failure(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
    }

    public async Task<Result> DeleteUserSettingsAsync(string userId)
    {
        _logger.LogDebug("Deleting user settings for user {UserId}", userId);
        return await _httpClient.DeleteUserSettingsAsync(userId);
    }

    #endregion
}

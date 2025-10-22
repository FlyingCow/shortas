using ShortasProxyApi.Application.DTOs;
using ShortasProxyApi.Domain.Common;
using System.Text;
using System.Text.Json;

namespace ShortasProxyApi.Infrastructure.HttpClients;

/// <summary>
/// HTTP client for communicating with the Click Router API.
/// This client uses DTOs for all communication and is independent of domain entities.
/// </summary>
public class ClickRouterApiClient
{
    private readonly HttpClient _httpClient;
    private readonly ILogger<ClickRouterApiClient> _logger;
    private readonly JsonSerializerOptions _jsonOptions;

    public ClickRouterApiClient(HttpClient httpClient, ILogger<ClickRouterApiClient> logger)
    {
        _httpClient = httpClient;
        _logger = logger;
        _jsonOptions = new JsonSerializerOptions
        {
            PropertyNamingPolicy = JsonNamingPolicy.CamelCase,
            PropertyNameCaseInsensitive = true
        };
    }

    #region Route Operations

    /// <summary>
    /// Get route by ID from Click Router API
    /// NOTE: This endpoint is not available in the Rust Click Router API.
    /// Use GetRouteAsync with domain/path instead.
    /// </summary>
    [Obsolete("The Click Router API does not support getting routes by ID. Use GetRouteAsync instead.")]
    public Task<Result<RouteDto?>> GetRouteByIdAsync(Guid id, string userId)
    {
        _logger.LogWarning("GetRouteByIdAsync is not supported by Click Router API. Use GetRouteAsync with domain/path instead.");
        return Task.FromResult(Result<RouteDto?>.Failure("NOT_SUPPORTED", "Getting routes by ID is not supported. Use domain/path instead."));
    }

    /// <summary>
    /// Create route via Click Router API
    /// </summary>
    public async Task<Result<RouteDto>> CreateRouteAsync(RouteDto route, string domain)
    {
        try
        {
            // Map to ClickRouter DTO for API communication
            var apiDto = ClickRouterRouteDto.FromDto(route);
            var json = JsonSerializer.Serialize(apiDto, _jsonOptions);
            var content = new StringContent(json, Encoding.UTF8, "application/json");

            var switchValue = route.Switch ?? "main";
            var link = route.Link;

            if (string.IsNullOrWhiteSpace(domain))
            {
                _logger.LogError("Cannot create route: domain name is missing");
                return Result<RouteDto>.Failure("VALIDATION_ERROR", "Domain name is required for route creation");
            }

            var response = await _httpClient.PostAsync($"/v1/routes/{switchValue}/{domain}/{link}", content);
            return await HandleResponse<RouteDto, ClickRouterRouteDto>(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to create route");
            return Result<RouteDto>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Router API");
        }
    }

    /// <summary>
    /// Update route by ID via Click Router API
    /// NOTE: This endpoint is not available in the Rust Click Router API.
    /// Use UpdateRouteAsync with domain/path instead.
    /// </summary>
    [Obsolete("The Click Router API does not support updating routes by ID. Use UpdateRouteAsync instead.")]
    public Task<Result<RouteDto>> UpdateRouteByIdAsync(Guid id, string userId, RouteDto route)
    {
        _logger.LogWarning("UpdateRouteByIdAsync is not supported by Click Router API. Use UpdateRouteAsync with domain/path instead.");
        return Task.FromResult(Result<RouteDto>.Failure("NOT_SUPPORTED", "Updating routes by ID is not supported. Use domain/path instead."));
    }

    /// <summary>
    /// Delete route by ID via Click Router API
    /// NOTE: This endpoint is not available in the Rust Click Router API.
    /// Use DeleteRouteAsync with domain/path instead.
    /// </summary>
    [Obsolete("The Click Router API does not support deleting routes by ID. Use DeleteRouteAsync instead.")]
    public Task<Result> DeleteRouteByIdAsync(Guid id, string userId)
    {
        _logger.LogWarning("DeleteRouteByIdAsync is not supported by Click Router API. Use DeleteRouteAsync with domain/path instead.");
        return Task.FromResult(Result.Failure("NOT_SUPPORTED", "Deleting routes by ID is not supported. Use domain/path instead."));
    }

    /// <summary>
    /// Get route by domain and path from Click Router API
    /// </summary>
    public async Task<Result<RouteDto?>> GetRouteAsync(string domain, string path, string userId, string? switchParam = null)
    {
        try
        {
            var switchValue = switchParam ?? "main";
            var url = $"/v1/routes/{switchValue}/{domain}/{path}";

            var response = await _httpClient.GetAsync(url);
            return await HandleResponse<RouteDto, ClickRouterRouteDto>(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to get route {Switch}/{Domain}/{Path}", switchParam ?? "main", domain, path);
            return Result<RouteDto?>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Router API");
        }
    }

    /// <summary>
    /// Update route by domain and path via Click Router API
    /// </summary>
    public async Task<Result<RouteDto>> UpdateRouteAsync(string domain, string path, string userId, RouteDto route)
    {
        try
        {
            // Map to ClickRouter DTO for API communication
            var apiDto = ClickRouterRouteDto.FromDto(route);
            var json = JsonSerializer.Serialize(apiDto, _jsonOptions);
            var content = new StringContent(json, Encoding.UTF8, "application/json");

            var switchValue = route.Switch ?? "main";
            var response = await _httpClient.PutAsync($"/v1/routes/{switchValue}/{domain}/{path}", content);
            return await HandleResponse<RouteDto, ClickRouterRouteDto>(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to update route {Switch}/{Domain}/{Path}", route.Switch ?? "main", domain, path);
            return Result<RouteDto>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Router API");
        }
    }

    /// <summary>
    /// Delete route by domain and path via Click Router API
    /// </summary>
    public async Task<Result> DeleteRouteAsync(string domain, string path, string userId)
    {
        try
        {
            var switchValue = "main";
            var response = await _httpClient.DeleteAsync($"/v1/routes/{switchValue}/{domain}/{path}");
            return await HandleResponse(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to delete route {Switch}/{Domain}/{Path}", "main", domain, path);
            return Result.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Router API");
        }
    }

    /// <summary>
    /// Bulk create routes via Click Router API
    /// </summary>
    public async Task<Result<List<RouteDto>>> BulkCreateRoutesAsync(List<RouteDto> routes)
    {
        try
        {
            // Map to ClickRouter DTOs for API communication
            var apiDtos = routes.Select(ClickRouterRouteDto.FromDto).ToList();
            var json = JsonSerializer.Serialize(apiDtos, _jsonOptions);
            var content = new StringContent(json, Encoding.UTF8, "application/json");

            var response = await _httpClient.PostAsync("/v1/routes/bulk", content);
            return await HandleListResponse<RouteDto, ClickRouterRouteDto>(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to bulk create routes");
            return Result<List<RouteDto>>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Router API");
        }
    }

    /// <summary>
    /// Bulk update routes via Click Router API
    /// </summary>
    public async Task<Result<List<RouteDto>>> BulkUpdateRoutesAsync(string userId, List<RouteDto> routes)
    {
        try
        {
            // Map to ClickRouter DTOs for API communication
            var apiDtos = routes.Select(ClickRouterRouteDto.FromDto).ToList();
            var json = JsonSerializer.Serialize(apiDtos, _jsonOptions);
            var content = new StringContent(json, Encoding.UTF8, "application/json");

            var response = await _httpClient.PutAsync("/v1/routes/bulk", content);
            return await HandleListResponse<RouteDto, ClickRouterRouteDto>(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to bulk update routes");
            return Result<List<RouteDto>>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Router API");
        }
    }

    /// <summary>
    /// Bulk delete routes via Click Router API
    /// </summary>
    public async Task<Result> BulkDeleteRoutesAsync(string userId, List<string> routeIds)
    {
        try
        {
            var json = JsonSerializer.Serialize(routeIds, _jsonOptions);
            var content = new StringContent(json, Encoding.UTF8, "application/json");

            var request = new HttpRequestMessage(HttpMethod.Delete, "/v1/routes/bulk")
            {
                Content = content
            };

            var response = await _httpClient.SendAsync(request);
            return await HandleResponse(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to bulk delete routes");
            return Result.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Router API");
        }
    }

    /// <summary>
    /// List routes with pagination via Click Router API
    /// </summary>
    public async Task<Result<(List<RouteDto> Routes, int TotalCount)>> ListRoutesAsync(
        int page = 1,
        int pageSize = 20,
        string? search = null,
        string? status = null,
        string? ownerId = null,
        string? workspaceId = null)
    {
        try
        {
            var queryParams = new List<string>
            {
                $"page={page}",
                $"pageSize={pageSize}"
            };

            if (!string.IsNullOrEmpty(search))
                queryParams.Add($"search={Uri.EscapeDataString(search)}");
            if (!string.IsNullOrEmpty(status))
                queryParams.Add($"status={Uri.EscapeDataString(status)}");
            if (!string.IsNullOrEmpty(ownerId))
                queryParams.Add($"ownerId={Uri.EscapeDataString(ownerId)}");
            if (!string.IsNullOrEmpty(workspaceId))
                queryParams.Add($"workspaceId={Uri.EscapeDataString(workspaceId)}");

            var queryString = string.Join("&", queryParams);
            var requestUrl = $"/v1/routes?{queryString}";
            var response = await _httpClient.GetAsync(requestUrl);

            if (!response.IsSuccessStatusCode)
            {
                var errorContent = await response.Content.ReadAsStringAsync();
                _logger.LogError("Failed to list routes: {StatusCode} - {Content}", response.StatusCode, errorContent);
                return Result<(List<RouteDto>, int)>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Router API");
            }

            var content = await response.Content.ReadAsStringAsync();
            var jsonDoc = JsonDocument.Parse(content);
            var root = jsonDoc.RootElement;

            // Parse the response structure
            var dataElement = root.GetProperty("data");
            var apiDtos = JsonSerializer.Deserialize<List<ClickRouterRouteDto>>(dataElement.GetRawText(), _jsonOptions);
            var routes = apiDtos?.Select(dto => dto.ToDto()).ToList() ?? new List<RouteDto>();

            var totalCount = root.GetProperty("pagination").GetProperty("totalCount").GetInt32();

            return Result<(List<RouteDto>, int)>.Success((routes, totalCount));
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to list routes");
            return Result<(List<RouteDto>, int)>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Router API");
        }
    }

    #endregion

    #region Certificate Operations

    /// <summary>
    /// Get certificate by domain from Click Router API
    /// </summary>
    public async Task<Result<CertificateDto?>> GetCertificateAsync(string domain)
    {
        try
        {
            var response = await _httpClient.GetAsync($"/v1/certificates/{domain}");
            return await HandleResponse<CertificateDto, ClickRouterCertificateDto>(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to get certificate for domain {Domain}", domain);
            return Result<CertificateDto?>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Router API");
        }
    }

    /// <summary>
    /// Create certificate via Click Router API
    /// </summary>
    public async Task<Result<CertificateDto>> CreateCertificateAsync(string domain, CertificateDto certificate)
    {
        try
        {
            var apiDto = ClickRouterCertificateDto.FromDto(certificate);
            var json = JsonSerializer.Serialize(apiDto, _jsonOptions);
            var content = new StringContent(json, Encoding.UTF8, "application/json");

            var response = await _httpClient.PostAsync($"/v1/certificates/{domain}", content);
            return await HandleResponse<CertificateDto, ClickRouterCertificateDto>(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to create certificate for domain {Domain}", domain);
            return Result<CertificateDto>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Router API");
        }
    }

    /// <summary>
    /// Update certificate via Click Router API
    /// </summary>
    public async Task<Result<CertificateDto>> UpdateCertificateAsync(string domain, CertificateDto certificate)
    {
        try
        {
            var apiDto = ClickRouterCertificateDto.FromDto(certificate);
            var json = JsonSerializer.Serialize(apiDto, _jsonOptions);
            var content = new StringContent(json, Encoding.UTF8, "application/json");

            var response = await _httpClient.PutAsync($"/v1/certificates/{domain}", content);
            return await HandleResponse<CertificateDto, ClickRouterCertificateDto>(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to update certificate for domain {Domain}", domain);
            return Result<CertificateDto>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Router API");
        }
    }

    /// <summary>
    /// Delete certificate via Click Router API
    /// </summary>
    public async Task<Result> DeleteCertificateAsync(string domain)
    {
        try
        {
            var response = await _httpClient.DeleteAsync($"/v1/certificates/{domain}");
            return await HandleResponse(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to delete certificate for domain {Domain}", domain);
            return Result.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Router API");
        }
    }

    /// <summary>
    /// List certificates with pagination via Click Router API
    /// NOTE: This endpoint may not be available in the Rust Click Router API.
    /// </summary>
    public async Task<Result<(List<CertificateDto> Certificates, int TotalCount)>> ListCertificatesAsync(
        int page = 1,
        int pageSize = 20,
        string? search = null)
    {
        try
        {
            var queryParams = new List<string>
            {
                $"page={page}",
                $"pageSize={pageSize}"
            };

            if (!string.IsNullOrEmpty(search))
                queryParams.Add($"search={Uri.EscapeDataString(search)}");

            var queryString = string.Join("&", queryParams);
            var response = await _httpClient.GetAsync($"/v1/certificates?{queryString}");

            if (!response.IsSuccessStatusCode)
            {
                var errorContent = await response.Content.ReadAsStringAsync();
                _logger.LogError("Failed to list certificates: {StatusCode} - {Content}", response.StatusCode, errorContent);
                return Result<(List<CertificateDto>, int)>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Router API");
            }

            var content = await response.Content.ReadAsStringAsync();
            var jsonDoc = JsonDocument.Parse(content);
            var root = jsonDoc.RootElement;

            // Parse the response structure
            var dataElement = root.GetProperty("data");
            var apiDtos = JsonSerializer.Deserialize<List<ClickRouterCertificateDto>>(dataElement.GetRawText(), _jsonOptions);
            var certificates = apiDtos?.Select(dto => dto.ToDto()).ToList() ?? new List<CertificateDto>();

            var totalCount = root.GetProperty("pagination").GetProperty("totalCount").GetInt32();

            return Result<(List<CertificateDto>, int)>.Success((certificates, totalCount));
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to list certificates");
            return Result<(List<CertificateDto>, int)>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Router API");
        }
    }

    #endregion

    #region User Settings Operations

    /// <summary>
    /// Get user settings from Click Router API
    /// </summary>
    public async Task<Result<UserSettingsDto?>> GetUserSettingsAsync(string userId)
    {
        try
        {
            var response = await _httpClient.GetAsync($"/v1/user-settings/{userId}");

            // Special case: 404 means user settings don't exist yet, which is a valid state
            if (response.StatusCode == System.Net.HttpStatusCode.NotFound)
            {
                _logger.LogInformation("User settings not found for user {UserId} - this is expected for new users", userId);
                return Result<UserSettingsDto?>.Success(null);
            }

            return await HandleResponse<UserSettingsDto, ClickRouterUserSettingsDto>(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to get user settings for user {UserId}", userId);
            return Result<UserSettingsDto?>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Router API");
        }
    }

    /// <summary>
    /// Create user settings via Click Router API
    /// </summary>
    public async Task<Result<UserSettingsDto>> CreateUserSettingsAsync(string userId, UserSettingsDto settings)
    {
        try
        {
            var apiDto = ClickRouterUserSettingsDto.FromDto(settings);
            var json = JsonSerializer.Serialize(apiDto, _jsonOptions);
            var content = new StringContent(json, Encoding.UTF8, "application/json");

            var response = await _httpClient.PostAsync($"/v1/user-settings/{userId}", content);
            return await HandleResponse<UserSettingsDto, ClickRouterUserSettingsDto>(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to create user settings for user {UserId}", userId);
            return Result<UserSettingsDto>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Router API");
        }
    }

    /// <summary>
    /// Update user settings via Click Router API
    /// </summary>
    public async Task<Result<UserSettingsDto>> UpdateUserSettingsAsync(string userId, UserSettingsDto settings)
    {
        try
        {
            var apiDto = ClickRouterUserSettingsDto.FromDto(settings);
            var json = JsonSerializer.Serialize(apiDto, _jsonOptions);
            var content = new StringContent(json, Encoding.UTF8, "application/json");

            var response = await _httpClient.PutAsync($"/v1/user-settings/{userId}", content);
            return await HandleResponse<UserSettingsDto, ClickRouterUserSettingsDto>(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to update user settings for user {UserId}", userId);
            return Result<UserSettingsDto>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Router API");
        }
    }

    /// <summary>
    /// Delete user settings via Click Router API
    /// </summary>
    public async Task<Result> DeleteUserSettingsAsync(string userId)
    {
        try
        {
            var response = await _httpClient.DeleteAsync($"/v1/user-settings/{userId}");
            return await HandleResponse(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to delete user settings for user {UserId}", userId);
            return Result.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Router API");
        }
    }

    #endregion

    #region Private Helper Methods

    /// <summary>
    /// Handle response and map from ClickRouter API DTO to Application DTO
    /// </summary>
    private async Task<Result<TAppDto>> HandleResponse<TAppDto, TApiDto>(HttpResponseMessage response)
        where TApiDto : class
    {
        if (response.IsSuccessStatusCode)
        {
            var content = await response.Content.ReadAsStringAsync();
            if (string.IsNullOrEmpty(content))
            {
                return Result<TAppDto>.Success(default(TAppDto)!);
            }

            var apiDto = JsonSerializer.Deserialize<TApiDto>(content, _jsonOptions);
            if (apiDto == null)
            {
                return Result<TAppDto>.Success(default(TAppDto)!);
            }

            // Map API DTO to Application DTO
            var appDto = MapToApplicationDto<TAppDto, TApiDto>(apiDto);
            return Result<TAppDto>.Success(appDto);
        }

        return await HandleErrorResponse<TAppDto>(response);
    }

    /// <summary>
    /// Handle list response and map from ClickRouter API DTOs to Application DTOs
    /// </summary>
    private async Task<Result<List<TAppDto>>> HandleListResponse<TAppDto, TApiDto>(HttpResponseMessage response)
        where TApiDto : class
    {
        if (response.IsSuccessStatusCode)
        {
            var content = await response.Content.ReadAsStringAsync();
            if (string.IsNullOrEmpty(content))
            {
                return Result<List<TAppDto>>.Success(new List<TAppDto>());
            }

            var apiDtos = JsonSerializer.Deserialize<List<TApiDto>>(content, _jsonOptions);
            if (apiDtos == null)
            {
                return Result<List<TAppDto>>.Success(new List<TAppDto>());
            }

            // Map API DTOs to Application DTOs
            var appDtos = apiDtos.Select(apiDto => MapToApplicationDto<TAppDto, TApiDto>(apiDto)).ToList();
            return Result<List<TAppDto>>.Success(appDtos);
        }

        return await HandleErrorResponse<List<TAppDto>>(response);
    }

    /// <summary>
    /// Map ClickRouter API DTO to Application DTO
    /// </summary>
    private TAppDto MapToApplicationDto<TAppDto, TApiDto>(TApiDto apiDto)
    {
        object result = apiDto switch
        {
            ClickRouterRouteDto routeDto => routeDto.ToDto(),
            ClickRouterCertificateDto certDto => certDto.ToDto(),
            ClickRouterUserSettingsDto settingsDto => settingsDto.ToDto(),
            _ => throw new NotSupportedException($"Mapping from {typeof(TApiDto).Name} to {typeof(TAppDto).Name} is not supported")
        };

        return (TAppDto)result;
    }

    private async Task<Result<T>> HandleErrorResponse<T>(HttpResponseMessage response)
    {
        var errorContent = await response.Content.ReadAsStringAsync();
        _logger.LogError("HTTP request failed: {StatusCode} - {Content}", response.StatusCode, errorContent);

        return response.StatusCode switch
        {
            System.Net.HttpStatusCode.BadRequest => Result<T>.Failure("VALIDATION_ERROR", "Invalid request data"),
            System.Net.HttpStatusCode.Unauthorized => Result<T>.Failure("UNAUTHORIZED", "Authentication required"),
            System.Net.HttpStatusCode.Forbidden => Result<T>.Failure("FORBIDDEN", "Access denied"),
            System.Net.HttpStatusCode.NotFound => Result<T>.Failure("NOT_FOUND", "Resource not found"),
            System.Net.HttpStatusCode.Conflict => Result<T>.Failure("CONFLICT", "Resource conflict"),
            System.Net.HttpStatusCode.RequestTimeout => Result<T>.Failure("TIMEOUT", "Request timeout"),
            System.Net.HttpStatusCode.TooManyRequests => Result<T>.Failure("RATE_LIMIT_EXCEEDED", "Rate limit exceeded"),
            System.Net.HttpStatusCode.ServiceUnavailable => Result<T>.Failure("CIRCUIT_BREAKER_OPEN", "Service unavailable"),
            _ => Result<T>.Failure("EXTERNAL_SERVICE_ERROR", "External service error")
        };
    }

    private async Task<Result> HandleResponse(HttpResponseMessage response)
    {
        if (response.IsSuccessStatusCode)
        {
            return Result.Success();
        }

        var errorContent = await response.Content.ReadAsStringAsync();
        _logger.LogError("HTTP request failed: {StatusCode} - {Content}", response.StatusCode, errorContent);

        return response.StatusCode switch
        {
            System.Net.HttpStatusCode.BadRequest => Result.Failure("VALIDATION_ERROR", "Invalid request data"),
            System.Net.HttpStatusCode.Unauthorized => Result.Failure("UNAUTHORIZED", "Authentication required"),
            System.Net.HttpStatusCode.Forbidden => Result.Failure("FORBIDDEN", "Access denied"),
            System.Net.HttpStatusCode.NotFound => Result.Failure("NOT_FOUND", "Resource not found"),
            System.Net.HttpStatusCode.Conflict => Result.Failure("CONFLICT", "Resource conflict"),
            System.Net.HttpStatusCode.RequestTimeout => Result.Failure("TIMEOUT", "Request timeout"),
            System.Net.HttpStatusCode.TooManyRequests => Result.Failure("RATE_LIMIT_EXCEEDED", "Rate limit exceeded"),
            System.Net.HttpStatusCode.ServiceUnavailable => Result.Failure("CIRCUIT_BREAKER_OPEN", "Service unavailable"),
            _ => Result.Failure("EXTERNAL_SERVICE_ERROR", "External service error")
        };
    }

    #endregion
}

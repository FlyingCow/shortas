using ShortasProxyApi.Domain.Entities;
using ShortasProxyApi.Domain.Common;
using System.Text;
using System.Text.Json;
using Route = ShortasProxyApi.Domain.Entities.Route;

namespace ShortasProxyApi.Infrastructure.HttpClients;

/// <summary>
/// HTTP client for communicating with the Click Router API.
/// This client is independent of service interfaces and focuses purely on HTTP communication.
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
    /// </summary>
    public async Task<Result<Route?>> GetRouteByIdAsync(Guid id, string userId)
    {
        try
        {
            var response = await _httpClient.GetAsync($"/v1/routes/{id}?userId={userId}");
            return await HandleResponse<Route>(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to get route by ID {RouteId}", id);
            return Result<Route?>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Router API");
        }
    }

    /// <summary>
    /// Create route via Click Router API
    /// </summary>
    public async Task<Result<Route>> CreateRouteAsync(Route route)
    {
        try
        {
            var json = JsonSerializer.Serialize(route, _jsonOptions);
            var content = new StringContent(json, Encoding.UTF8, "application/json");
            
            var response = await _httpClient.PostAsync("/v1/routes", content);
            return await HandleResponse<Route>(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to create route");
            return Result<Route>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Router API");
        }
    }

    /// <summary>
    /// Update route by ID via Click Router API
    /// </summary>
    public async Task<Result<Route>> UpdateRouteByIdAsync(Guid id, string userId, Route route)
    {
        try
        {
            var json = JsonSerializer.Serialize(route, _jsonOptions);
            var content = new StringContent(json, Encoding.UTF8, "application/json");
            
            var response = await _httpClient.PutAsync($"/v1/routes/{id}?userId={userId}", content);
            return await HandleResponse<Route>(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to update route {RouteId}", id);
            return Result<Route>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Router API");
        }
    }

    /// <summary>
    /// Delete route by ID via Click Router API
    /// </summary>
    public async Task<Result> DeleteRouteByIdAsync(Guid id, string userId)
    {
        try
        {
            var response = await _httpClient.DeleteAsync($"/v1/routes/{id}?userId={userId}");
            return await HandleResponse(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to delete route {RouteId}", id);
            return Result.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Router API");
        }
    }

    /// <summary>
    /// Get route by domain and path from Click Router API
    /// </summary>
    public async Task<Result<Route?>> GetRouteAsync(string domain, string path, string userId, string? switchParam = null)
    {
        try
        {
            var url = $"/v1/routes/{domain}/{path}?userId={userId}";
            if (!string.IsNullOrEmpty(switchParam))
            {
                url += $"&switch={switchParam}";
            }
            
            var response = await _httpClient.GetAsync(url);
            return await HandleResponse<Route>(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to get route {Domain}/{Path}", domain, path);
            return Result<Route?>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Router API");
        }
    }

    /// <summary>
    /// Update route by domain and path via Click Router API
    /// </summary>
    public async Task<Result<Route>> UpdateRouteAsync(string domain, string path, string userId, Route route)
    {
        try
        {
            var json = JsonSerializer.Serialize(route, _jsonOptions);
            var content = new StringContent(json, Encoding.UTF8, "application/json");
            
            var response = await _httpClient.PutAsync($"/v1/routes/{domain}/{path}?userId={userId}", content);
            return await HandleResponse<Route>(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to update route {Domain}/{Path}", domain, path);
            return Result<Route>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Router API");
        }
    }

    /// <summary>
    /// Delete route by domain and path via Click Router API
    /// </summary>
    public async Task<Result> DeleteRouteAsync(string domain, string path, string userId)
    {
        try
        {
            var response = await _httpClient.DeleteAsync($"/v1/routes/{domain}/{path}?userId={userId}");
            return await HandleResponse(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to delete route {Domain}/{Path}", domain, path);
            return Result.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Router API");
        }
    }

    /// <summary>
    /// Bulk create routes via Click Router API
    /// </summary>
    public async Task<Result<List<Route>>> BulkCreateRoutesAsync(List<Route> routes)
    {
        try
        {
            var json = JsonSerializer.Serialize(routes, _jsonOptions);
            var content = new StringContent(json, Encoding.UTF8, "application/json");
            
            var response = await _httpClient.PostAsync("/v1/routes/bulk", content);
            return await HandleResponse<List<Route>>(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to bulk create routes");
            return Result<List<Route>>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Router API");
        }
    }

    /// <summary>
    /// Bulk update routes via Click Router API
    /// </summary>
    public async Task<Result<List<Route>>> BulkUpdateRoutesAsync(string userId, List<Route> routes)
    {
        try
        {
            var json = JsonSerializer.Serialize(routes, _jsonOptions);
            var content = new StringContent(json, Encoding.UTF8, "application/json");
            
            var response = await _httpClient.PutAsync($"/v1/routes/bulk?userId={userId}", content);
            return await HandleResponse<List<Route>>(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to bulk update routes");
            return Result<List<Route>>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Router API");
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
            
            var request = new HttpRequestMessage(HttpMethod.Delete, $"/v1/routes/bulk?userId={userId}")
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
    public async Task<Result<(List<Route> Routes, int TotalCount)>> ListRoutesAsync(
        int page = 1,
        int pageSize = 20,
        string? search = null,
        string? status = null,
        string? ownerId = null)
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
            
            var queryString = string.Join("&", queryParams);
            var requestUrl = $"/v1/routes?{queryString}";
            var response = await _httpClient.GetAsync(requestUrl);
            
            if (!response.IsSuccessStatusCode)
            {
                var errorContent = await response.Content.ReadAsStringAsync();
                _logger.LogError("Failed to list routes: {StatusCode} - {Content}", response.StatusCode, errorContent);
                return Result<(List<Route>, int)>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Router API");
            }
            
            var content = await response.Content.ReadAsStringAsync();
            var result = JsonSerializer.Deserialize<dynamic>(content, _jsonOptions);
            
            // Parse the response structure
            var routes = JsonSerializer.Deserialize<List<Route>>(result.GetProperty("data").GetRawText(), _jsonOptions);
            var totalCount = result.GetProperty("pagination").GetProperty("totalCount").GetInt32();
            
            return Result<(List<Route>, int)>.Success((routes ?? new List<Route>(), totalCount));
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to list routes");
            return Result<(List<Route>, int)>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Router API");
        }
    }

    #endregion

    #region Certificate Operations

    /// <summary>
    /// Get certificate by domain from Click Router API
    /// </summary>
    public async Task<Result<Certificate?>> GetCertificateAsync(string domain)
    {
        try
        {
            var response = await _httpClient.GetAsync($"/v1/certificates/{domain}");
            return await HandleResponse<Certificate>(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to get certificate for domain {Domain}", domain);
            return Result<Certificate?>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Router API");
        }
    }

    /// <summary>
    /// Create certificate via Click Router API
    /// </summary>
    public async Task<Result<Certificate>> CreateCertificateAsync(string domain, Certificate certificate)
    {
        try
        {
            var json = JsonSerializer.Serialize(certificate, _jsonOptions);
            var content = new StringContent(json, Encoding.UTF8, "application/json");
            
            var response = await _httpClient.PostAsync($"/v1/certificates/{domain}", content);
            return await HandleResponse<Certificate>(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to create certificate for domain {Domain}", domain);
            return Result<Certificate>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Router API");
        }
    }

    /// <summary>
    /// Update certificate via Click Router API
    /// </summary>
    public async Task<Result<Certificate>> UpdateCertificateAsync(string domain, Certificate certificate)
    {
        try
        {
            var json = JsonSerializer.Serialize(certificate, _jsonOptions);
            var content = new StringContent(json, Encoding.UTF8, "application/json");
            
            var response = await _httpClient.PutAsync($"/v1/certificates/{domain}", content);
            return await HandleResponse<Certificate>(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to update certificate for domain {Domain}", domain);
            return Result<Certificate>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Router API");
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
    /// </summary>
    public async Task<Result<(List<Certificate> Certificates, int TotalCount)>> ListCertificatesAsync(
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
                return Result<(List<Certificate>, int)>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Router API");
            }
            
            var content = await response.Content.ReadAsStringAsync();
            var result = JsonSerializer.Deserialize<dynamic>(content, _jsonOptions);
            
            // Parse the response structure
            var certificates = JsonSerializer.Deserialize<List<Certificate>>(result.GetProperty("data").GetRawText(), _jsonOptions);
            var totalCount = result.GetProperty("pagination").GetProperty("totalCount").GetInt32();
            
            return Result<(List<Certificate>, int)>.Success((certificates ?? new List<Certificate>(), totalCount));
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to list certificates");
            return Result<(List<Certificate>, int)>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Router API");
        }
    }

    #endregion

    #region User Settings Operations

    /// <summary>
    /// Get user settings from Click Router API
    /// </summary>
    public async Task<Result<UserSettings?>> GetUserSettingsAsync(string userId)
    {
        try
        {
            var response = await _httpClient.GetAsync($"/v1/user-settings/{userId}");
            return await HandleResponse<UserSettings>(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to get user settings for user {UserId}", userId);
            return Result<UserSettings?>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Router API");
        }
    }

    /// <summary>
    /// Create user settings via Click Router API
    /// </summary>
    public async Task<Result<UserSettings>> CreateUserSettingsAsync(string userId, UserSettings settings)
    {
        try
        {
            var json = JsonSerializer.Serialize(settings, _jsonOptions);
            var content = new StringContent(json, Encoding.UTF8, "application/json");
            
            var response = await _httpClient.PostAsync($"/v1/user-settings/{userId}", content);
            return await HandleResponse<UserSettings>(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to create user settings for user {UserId}", userId);
            return Result<UserSettings>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Router API");
        }
    }

    /// <summary>
    /// Update user settings via Click Router API
    /// </summary>
    public async Task<Result<UserSettings>> UpdateUserSettingsAsync(string userId, UserSettings settings)
    {
        try
        {
            var json = JsonSerializer.Serialize(settings, _jsonOptions);
            var content = new StringContent(json, Encoding.UTF8, "application/json");
            
            var response = await _httpClient.PutAsync($"/v1/user-settings/{userId}", content);
            return await HandleResponse<UserSettings>(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to update user settings for user {UserId}", userId);
            return Result<UserSettings>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Router API");
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

    private async Task<Result<T>> HandleResponse<T>(HttpResponseMessage response)
    {
        if (response.IsSuccessStatusCode)
        {
            var content = await response.Content.ReadAsStringAsync();
            if (string.IsNullOrEmpty(content))
            {
                return Result<T>.Success(default(T)!);
            }
            
            var result = JsonSerializer.Deserialize<T>(content, _jsonOptions);
            return Result<T>.Success(result!);
        }
        
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
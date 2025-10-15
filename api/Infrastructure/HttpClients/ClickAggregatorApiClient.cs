using ShortasProxyApi.Domain.Entities;
using ShortasProxyApi.Domain.Common;
using System.Text;
using System.Text.Json;

namespace ShortasProxyApi.Infrastructure.HttpClients;

/// <summary>
/// HTTP client for communicating with the Click Aggregator API.
/// This client is independent of service interfaces and focuses purely on HTTP communication.
/// </summary>
public class ClickAggregatorApiClient
{
    private readonly HttpClient _httpClient;
    private readonly ILogger<ClickAggregatorApiClient> _logger;
    private readonly JsonSerializerOptions _jsonOptions;

    public ClickAggregatorApiClient(HttpClient httpClient, ILogger<ClickAggregatorApiClient> logger)
    {
        _httpClient = httpClient;
        _logger = logger;
        _jsonOptions = new JsonSerializerOptions
        {
            PropertyNamingPolicy = JsonNamingPolicy.CamelCase,
            PropertyNameCaseInsensitive = true
        };
    }

    #region Click Stream Operations

    /// <summary>
    /// Get click stream data from Click Aggregator API
    /// </summary>
    public async Task<Result<object>> GetClickStreamAsync(
        DateTime? startDate = null,
        DateTime? endDate = null,
        string? routeId = null,
        string? ownerId = null,
        int page = 1,
        int pageSize = 100)
    {
        try
        {
            var queryParams = new List<string>
            {
                $"page={page}",
                $"pageSize={pageSize}"
            };
            
            if (startDate.HasValue)
                queryParams.Add($"start={startDate.Value:yyyy-MM-ddTHH:mm:ssZ}");
            if (endDate.HasValue)
                queryParams.Add($"end={endDate.Value:yyyy-MM-ddTHH:mm:ssZ}");
            if (!string.IsNullOrEmpty(routeId))
                queryParams.Add($"routeId={Uri.EscapeDataString(routeId)}");
            if (!string.IsNullOrEmpty(ownerId))
                queryParams.Add($"ownerId={Uri.EscapeDataString(ownerId)}");
            
            var queryString = string.Join("&", queryParams);
            var response = await _httpClient.GetAsync($"/v1/clickstream?{queryString}");
            
            return await HandleResponse<object>(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to get click stream data");
            return Result<object>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Aggregator API");
        }
    }

    /// <summary>
    /// Get click stream analytics overview from Click Aggregator API
    /// </summary>
    public async Task<Result<object>> GetClickStreamOverviewAsync(
        DateTime? startDate = null,
        DateTime? endDate = null,
        string? ownerId = null)
    {
        try
        {
            var queryParams = new List<string>();
            
            if (startDate.HasValue)
                queryParams.Add($"start={startDate.Value:yyyy-MM-ddTHH:mm:ssZ}");
            if (endDate.HasValue)
                queryParams.Add($"end={endDate.Value:yyyy-MM-ddTHH:mm:ssZ}");
            if (!string.IsNullOrEmpty(ownerId))
                queryParams.Add($"ownerId={Uri.EscapeDataString(ownerId)}");
            
            var queryString = queryParams.Count > 0 ? "?" + string.Join("&", queryParams) : "";
            var response = await _httpClient.GetAsync($"/v1/clickstream/overview{queryString}");
            
            return await HandleResponse<object>(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to get click stream overview");
            return Result<object>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Aggregator API");
        }
    }

    /// <summary>
    /// Get click stream statistics from Click Aggregator API
    /// </summary>
    public async Task<Result<object>> GetClickStreamStatsAsync(
        DateTime? startDate = null,
        DateTime? endDate = null,
        string? ownerId = null,
        string? groupBy = null)
    {
        try
        {
            var queryParams = new List<string>();
            
            if (startDate.HasValue)
                queryParams.Add($"start={startDate.Value:yyyy-MM-ddTHH:mm:ssZ}");
            if (endDate.HasValue)
                queryParams.Add($"end={endDate.Value:yyyy-MM-ddTHH:mm:ssZ}");
            if (!string.IsNullOrEmpty(ownerId))
                queryParams.Add($"ownerId={Uri.EscapeDataString(ownerId)}");
            if (!string.IsNullOrEmpty(groupBy))
                queryParams.Add($"groupBy={Uri.EscapeDataString(groupBy)}");
            
            var queryString = queryParams.Count > 0 ? "?" + string.Join("&", queryParams) : "";
            var response = await _httpClient.GetAsync($"/v1/clickstream/stats{queryString}");
            
            return await HandleResponse<object>(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to get click stream statistics");
            return Result<object>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Aggregator API");
        }
    }

    /// <summary>
    /// Get route-specific analytics from Click Aggregator API
    /// </summary>
    public async Task<Result<object>> GetRouteAnalyticsAsync(
        string routeId,
        DateTime? startDate = null,
        DateTime? endDate = null,
        string? ownerId = null)
    {
        try
        {
            var queryParams = new List<string>();
            
            if (startDate.HasValue)
                queryParams.Add($"start={startDate.Value:yyyy-MM-ddTHH:mm:ssZ}");
            if (endDate.HasValue)
                queryParams.Add($"end={endDate.Value:yyyy-MM-ddTHH:mm:ssZ}");
            if (!string.IsNullOrEmpty(ownerId))
                queryParams.Add($"ownerId={Uri.EscapeDataString(ownerId)}");
            
            var queryString = queryParams.Count > 0 ? "?" + string.Join("&", queryParams) : "";
            var response = await _httpClient.GetAsync($"/v1/clickstream/{routeId}{queryString}");
            
            return await HandleResponse<object>(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to get route analytics for route {RouteId}", routeId);
            return Result<object>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Aggregator API");
        }
    }

    /// <summary>
    /// Get geographic analytics from Click Aggregator API
    /// </summary>
    public async Task<Result<object>> GetGeographicAnalyticsAsync(
        DateTime? startDate = null,
        DateTime? endDate = null,
        string? ownerId = null,
        string? groupBy = "country")
    {
        try
        {
            var queryParams = new List<string>
            {
                $"groupBy={Uri.EscapeDataString(groupBy)}"
            };
            
            if (startDate.HasValue)
                queryParams.Add($"start={startDate.Value:yyyy-MM-ddTHH:mm:ssZ}");
            if (endDate.HasValue)
                queryParams.Add($"end={endDate.Value:yyyy-MM-ddTHH:mm:ssZ}");
            if (!string.IsNullOrEmpty(ownerId))
                queryParams.Add($"ownerId={Uri.EscapeDataString(ownerId)}");
            
            var queryString = string.Join("&", queryParams);
            var response = await _httpClient.GetAsync($"/v1/analytics/geographic?{queryString}");
            
            return await HandleResponse<object>(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to get geographic analytics");
            return Result<object>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Aggregator API");
        }
    }

    /// <summary>
    /// Get device analytics from Click Aggregator API
    /// </summary>
    public async Task<Result<object>> GetDeviceAnalyticsAsync(
        DateTime? startDate = null,
        DateTime? endDate = null,
        string? ownerId = null,
        string? groupBy = "device_family")
    {
        try
        {
            var queryParams = new List<string>
            {
                $"groupBy={Uri.EscapeDataString(groupBy)}"
            };
            
            if (startDate.HasValue)
                queryParams.Add($"start={startDate.Value:yyyy-MM-ddTHH:mm:ssZ}");
            if (endDate.HasValue)
                queryParams.Add($"end={endDate.Value:yyyy-MM-ddTHH:mm:ssZ}");
            if (!string.IsNullOrEmpty(ownerId))
                queryParams.Add($"ownerId={Uri.EscapeDataString(ownerId)}");
            
            var queryString = string.Join("&", queryParams);
            var response = await _httpClient.GetAsync($"/v1/analytics/device?{queryString}");
            
            return await HandleResponse<object>(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to get device analytics");
            return Result<object>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Aggregator API");
        }
    }

    /// <summary>
    /// Get browser analytics from Click Aggregator API
    /// </summary>
    public async Task<Result<object>> GetBrowserAnalyticsAsync(
        DateTime? startDate = null,
        DateTime? endDate = null,
        string? ownerId = null,
        string? groupBy = "user_agent_family")
    {
        try
        {
            var queryParams = new List<string>
            {
                $"groupBy={Uri.EscapeDataString(groupBy)}"
            };
            
            if (startDate.HasValue)
                queryParams.Add($"start={startDate.Value:yyyy-MM-ddTHH:mm:ssZ}");
            if (endDate.HasValue)
                queryParams.Add($"end={endDate.Value:yyyy-MM-ddTHH:mm:ssZ}");
            if (!string.IsNullOrEmpty(ownerId))
                queryParams.Add($"ownerId={Uri.EscapeDataString(ownerId)}");
            
            var queryString = string.Join("&", queryParams);
            var response = await _httpClient.GetAsync($"/v1/analytics/browser?{queryString}");
            
            return await HandleResponse<object>(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to get browser analytics");
            return Result<object>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Aggregator API");
        }
    }

    /// <summary>
    /// Get time series analytics from Click Aggregator API
    /// </summary>
    public async Task<Result<object>> GetTimeSeriesAnalyticsAsync(
        DateTime? startDate = null,
        DateTime? endDate = null,
        string? ownerId = null,
        string? interval = "hour")
    {
        try
        {
            var queryParams = new List<string>
            {
                $"interval={Uri.EscapeDataString(interval)}"
            };
            
            if (startDate.HasValue)
                queryParams.Add($"start={startDate.Value:yyyy-MM-ddTHH:mm:ssZ}");
            if (endDate.HasValue)
                queryParams.Add($"end={endDate.Value:yyyy-MM-ddTHH:mm:ssZ}");
            if (!string.IsNullOrEmpty(ownerId))
                queryParams.Add($"ownerId={Uri.EscapeDataString(ownerId)}");
            
            var queryString = string.Join("&", queryParams);
            var response = await _httpClient.GetAsync($"/v1/analytics/timeseries?{queryString}");
            
            return await HandleResponse<object>(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to get time series analytics");
            return Result<object>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Aggregator API");
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

    #endregion
}
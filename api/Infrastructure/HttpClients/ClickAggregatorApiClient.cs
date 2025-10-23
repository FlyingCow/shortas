using ShortasProxyApi.Domain.Common;
using ShortasProxyApi.Application.DTOs;
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
            // PropertyNamingPolicy not needed since we use JsonPropertyName attributes in DTOs
            PropertyNameCaseInsensitive = true
        };
    }

    #region Click Stream Operations

    /// <summary>
    /// Get click stream data from Click Aggregator API
    /// </summary>
    public async Task<Result<List<ClickStreamDto>>> GetClickStreamAsync(
        DateTime? startDate = null,
        DateTime? endDate = null,
        string? routeId = null,
        string? ownerId = null,
        int offset = 0,
        int limit = 100)
    {
        try
        {
            var queryParams = new List<string>();

            // Use snake_case parameter names to match Rust API
            if (!string.IsNullOrEmpty(routeId))
                queryParams.Add($"route_id={Uri.EscapeDataString(routeId)}");
            if (!string.IsNullOrEmpty(ownerId))
                queryParams.Add($"owner_id={Uri.EscapeDataString(ownerId)}");
            if (startDate.HasValue)
                queryParams.Add($"created_from={startDate.Value:yyyy-MM-ddTHH:mm:ssZ}");
            if (endDate.HasValue)
                queryParams.Add($"created_to={endDate.Value:yyyy-MM-ddTHH:mm:ssZ}");
            queryParams.Add($"limit={limit}");
            queryParams.Add($"offset={offset}");

            var queryString = string.Join("&", queryParams);
            _logger.LogDebug("Requesting clickstream: /v1/clickstream?{QueryString}", queryString);

            var response = await _httpClient.GetAsync($"/v1/clickstream?{queryString}");

            if (response.IsSuccessStatusCode)
            {
                var content = await response.Content.ReadAsStringAsync();
                _logger.LogDebug("Received clickstream response: {Length} bytes", content.Length);

                // Deserialize to API DTO
                var apiResponse = JsonSerializer.Deserialize<ClickAggregatorApiClickStreamResponse>(content, _jsonOptions);
                if (apiResponse == null)
                {
                    _logger.LogWarning("Failed to deserialize clickstream response");
                    return Result<List<ClickStreamDto>>.Success(new List<ClickStreamDto>());
                }

                _logger.LogInformation("Retrieved {Count} clickstream items out of {Total} total",
                    apiResponse.Items.Count, apiResponse.Total);

                // Map API DTOs to Application DTOs
                var dtos = apiResponse.Items.Select(ClickAggregatorApiClickStreamDto.ToDto).ToList();

                return Result<List<ClickStreamDto>>.Success(dtos);
            }

            return await HandleErrorResponse<List<ClickStreamDto>>(response);
        }
        catch (HttpRequestException ex)
        {
            _logger.LogError(ex, "HTTP error getting click stream data");
            return Result<List<ClickStreamDto>>.Failure("NETWORK_ERROR", "Network error communicating with Click Aggregator API");
        }
        catch (JsonException ex)
        {
            _logger.LogError(ex, "JSON deserialization error getting click stream data");
            return Result<List<ClickStreamDto>>.Failure("INTERNAL_ERROR", "Failed to parse clickstream response");
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to get click stream data");
            return Result<List<ClickStreamDto>>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to communicate with Click Aggregator API");
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

        return await HandleErrorResponse<T>(response);
    }

    #endregion

    #region Statistics Operations

    /// <summary>
    /// Get daily statistics
    /// </summary>
    public async Task<Result<List<DailyStatsDto>>> GetDailyStatsAsync(
        string? ownerId = null,
        string? routeId = null,
        string? fromDate = null,
        string? toDate = null)
    {
        try
        {
            var queryParams = new List<string>();
            if (!string.IsNullOrEmpty(ownerId))
                queryParams.Add($"owner_id={Uri.EscapeDataString(ownerId)}");
            if (!string.IsNullOrEmpty(routeId))
                queryParams.Add($"route_id={Uri.EscapeDataString(routeId)}");
            if (!string.IsNullOrEmpty(fromDate))
                queryParams.Add($"from_date={fromDate}");
            if (!string.IsNullOrEmpty(toDate))
                queryParams.Add($"to_date={toDate}");

            var queryString = queryParams.Count > 0 ? "?" + string.Join("&", queryParams) : "";
            var response = await _httpClient.GetAsync($"/v1/stats/daily{queryString}");

            if (response.IsSuccessStatusCode)
            {
                var content = await response.Content.ReadAsStringAsync();
                var stats = JsonSerializer.Deserialize<List<DailyStatsDto>>(content, _jsonOptions);
                return Result<List<DailyStatsDto>>.Success(stats ?? new List<DailyStatsDto>());
            }

            return await HandleErrorResponse<List<DailyStatsDto>>(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to get daily stats");
            return Result<List<DailyStatsDto>>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to get daily statistics");
        }
    }

    /// <summary>
    /// Get hourly statistics
    /// </summary>
    public async Task<Result<List<HourlyStatsDto>>> GetHourlyStatsAsync(
        string? ownerId = null,
        string? routeId = null,
        string? fromHour = null,
        string? toHour = null)
    {
        try
        {
            var queryParams = new List<string>();
            if (!string.IsNullOrEmpty(ownerId))
                queryParams.Add($"owner_id={Uri.EscapeDataString(ownerId)}");
            if (!string.IsNullOrEmpty(routeId))
                queryParams.Add($"route_id={Uri.EscapeDataString(routeId)}");
            if (!string.IsNullOrEmpty(fromHour))
                queryParams.Add($"from_hour={fromHour}");
            if (!string.IsNullOrEmpty(toHour))
                queryParams.Add($"to_hour={toHour}");

            var queryString = queryParams.Count > 0 ? "?" + string.Join("&", queryParams) : "";
            var response = await _httpClient.GetAsync($"/v1/stats/hourly{queryString}");

            if (response.IsSuccessStatusCode)
            {
                var content = await response.Content.ReadAsStringAsync();
                var stats = JsonSerializer.Deserialize<List<HourlyStatsDto>>(content, _jsonOptions);
                return Result<List<HourlyStatsDto>>.Success(stats ?? new List<HourlyStatsDto>());
            }

            return await HandleErrorResponse<List<HourlyStatsDto>>(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to get hourly stats");
            return Result<List<HourlyStatsDto>>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to get hourly statistics");
        }
    }

    /// <summary>
    /// Get geographic statistics
    /// </summary>
    public async Task<Result<List<GeographicStatsDto>>> GetGeographicStatsAsync(
        string? ownerId = null,
        string? routeId = null,
        string? fromDate = null,
        string? toDate = null)
    {
        try
        {
            var queryParams = new List<string>();
            if (!string.IsNullOrEmpty(ownerId))
                queryParams.Add($"owner_id={Uri.EscapeDataString(ownerId)}");
            if (!string.IsNullOrEmpty(routeId))
                queryParams.Add($"route_id={Uri.EscapeDataString(routeId)}");
            if (!string.IsNullOrEmpty(fromDate))
                queryParams.Add($"from_date={fromDate}");
            if (!string.IsNullOrEmpty(toDate))
                queryParams.Add($"to_date={toDate}");

            var queryString = queryParams.Count > 0 ? "?" + string.Join("&", queryParams) : "";
            var response = await _httpClient.GetAsync($"/v1/stats/geographic{queryString}");

            if (response.IsSuccessStatusCode)
            {
                var content = await response.Content.ReadAsStringAsync();
                var stats = JsonSerializer.Deserialize<List<GeographicStatsDto>>(content, _jsonOptions);
                return Result<List<GeographicStatsDto>>.Success(stats ?? new List<GeographicStatsDto>());
            }

            return await HandleErrorResponse<List<GeographicStatsDto>>(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to get geographic stats");
            return Result<List<GeographicStatsDto>>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to get geographic statistics");
        }
    }

    /// <summary>
    /// Get device statistics
    /// </summary>
    public async Task<Result<List<DeviceStatsDto>>> GetDeviceStatsAsync(
        string? ownerId = null,
        string? routeId = null,
        string? fromDate = null,
        string? toDate = null)
    {
        try
        {
            var queryParams = new List<string>();
            if (!string.IsNullOrEmpty(ownerId))
                queryParams.Add($"owner_id={Uri.EscapeDataString(ownerId)}");
            if (!string.IsNullOrEmpty(routeId))
                queryParams.Add($"route_id={Uri.EscapeDataString(routeId)}");
            if (!string.IsNullOrEmpty(fromDate))
                queryParams.Add($"from_date={fromDate}");
            if (!string.IsNullOrEmpty(toDate))
                queryParams.Add($"to_date={toDate}");

            var queryString = queryParams.Count > 0 ? "?" + string.Join("&", queryParams) : "";
            var response = await _httpClient.GetAsync($"/v1/stats/devices{queryString}");

            if (response.IsSuccessStatusCode)
            {
                var content = await response.Content.ReadAsStringAsync();
                var stats = JsonSerializer.Deserialize<List<DeviceStatsDto>>(content, _jsonOptions);
                return Result<List<DeviceStatsDto>>.Success(stats ?? new List<DeviceStatsDto>());
            }

            return await HandleErrorResponse<List<DeviceStatsDto>>(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to get device stats");
            return Result<List<DeviceStatsDto>>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to get device statistics");
        }
    }

    /// <summary>
    /// Get browser statistics
    /// </summary>
    public async Task<Result<List<BrowserStatsDto>>> GetBrowserStatsAsync(
        string? ownerId = null,
        string? routeId = null,
        string? fromDate = null,
        string? toDate = null)
    {
        try
        {
            var queryParams = new List<string>();
            if (!string.IsNullOrEmpty(ownerId))
                queryParams.Add($"owner_id={Uri.EscapeDataString(ownerId)}");
            if (!string.IsNullOrEmpty(routeId))
                queryParams.Add($"route_id={Uri.EscapeDataString(routeId)}");
            if (!string.IsNullOrEmpty(fromDate))
                queryParams.Add($"from_date={fromDate}");
            if (!string.IsNullOrEmpty(toDate))
                queryParams.Add($"to_date={toDate}");

            var queryString = queryParams.Count > 0 ? "?" + string.Join("&", queryParams) : "";
            var response = await _httpClient.GetAsync($"/v1/stats/browsers{queryString}");

            if (response.IsSuccessStatusCode)
            {
                var content = await response.Content.ReadAsStringAsync();
                var stats = JsonSerializer.Deserialize<List<BrowserStatsDto>>(content, _jsonOptions);
                return Result<List<BrowserStatsDto>>.Success(stats ?? new List<BrowserStatsDto>());
            }

            return await HandleErrorResponse<List<BrowserStatsDto>>(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to get browser stats");
            return Result<List<BrowserStatsDto>>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to get browser statistics");
        }
    }

    /// <summary>
    /// Get route performance statistics
    /// </summary>
    public async Task<Result<List<RoutePerformanceDto>>> GetRoutePerformanceAsync(
        string? ownerId = null,
        string? fromDate = null,
        string? toDate = null,
        int? limit = null)
    {
        try
        {
            var queryParams = new List<string>();
            if (!string.IsNullOrEmpty(ownerId))
                queryParams.Add($"owner_id={Uri.EscapeDataString(ownerId)}");
            if (!string.IsNullOrEmpty(fromDate))
                queryParams.Add($"from_date={fromDate}");
            if (!string.IsNullOrEmpty(toDate))
                queryParams.Add($"to_date={toDate}");
            if (limit.HasValue)
                queryParams.Add($"limit={limit.Value}");

            var queryString = queryParams.Count > 0 ? "?" + string.Join("&", queryParams) : "";
            var response = await _httpClient.GetAsync($"/v1/stats/route-performance{queryString}");

            if (response.IsSuccessStatusCode)
            {
                var content = await response.Content.ReadAsStringAsync();
                var stats = JsonSerializer.Deserialize<List<RoutePerformanceDto>>(content, _jsonOptions);
                return Result<List<RoutePerformanceDto>>.Success(stats ?? new List<RoutePerformanceDto>());
            }

            return await HandleErrorResponse<List<RoutePerformanceDto>>(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to get route performance");
            return Result<List<RoutePerformanceDto>>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to get route performance");
        }
    }

    /// <summary>
    /// Get top destinations
    /// </summary>
    public async Task<Result<List<TopDestinationDto>>> GetTopDestinationsAsync(
        string? ownerId = null,
        string? routeId = null,
        string? fromDate = null,
        string? toDate = null,
        int? limit = null)
    {
        try
        {
            var queryParams = new List<string>();
            if (!string.IsNullOrEmpty(ownerId))
                queryParams.Add($"owner_id={Uri.EscapeDataString(ownerId)}");
            if (!string.IsNullOrEmpty(routeId))
                queryParams.Add($"route_id={Uri.EscapeDataString(routeId)}");
            if (!string.IsNullOrEmpty(fromDate))
                queryParams.Add($"from_date={fromDate}");
            if (!string.IsNullOrEmpty(toDate))
                queryParams.Add($"to_date={toDate}");
            if (limit.HasValue)
                queryParams.Add($"limit={limit.Value}");

            var queryString = queryParams.Count > 0 ? "?" + string.Join("&", queryParams) : "";
            var response = await _httpClient.GetAsync($"/v1/stats/top-destinations{queryString}");

            if (response.IsSuccessStatusCode)
            {
                var content = await response.Content.ReadAsStringAsync();
                var stats = JsonSerializer.Deserialize<List<TopDestinationDto>>(content, _jsonOptions);
                return Result<List<TopDestinationDto>>.Success(stats ?? new List<TopDestinationDto>());
            }

            return await HandleErrorResponse<List<TopDestinationDto>>(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to get top destinations");
            return Result<List<TopDestinationDto>>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to get top destinations");
        }
    }

    /// <summary>
    /// Get traffic type statistics (bot vs human)
    /// </summary>
    public async Task<Result<List<TrafficTypeStatsDto>>> GetTrafficTypeStatsAsync(
        string? ownerId = null,
        string? routeId = null,
        string? fromHour = null,
        string? toHour = null)
    {
        try
        {
            var queryParams = new List<string>();
            if (!string.IsNullOrEmpty(ownerId))
                queryParams.Add($"owner_id={Uri.EscapeDataString(ownerId)}");
            if (!string.IsNullOrEmpty(routeId))
                queryParams.Add($"route_id={Uri.EscapeDataString(routeId)}");
            if (!string.IsNullOrEmpty(fromHour))
                queryParams.Add($"from_hour={fromHour}");
            if (!string.IsNullOrEmpty(toHour))
                queryParams.Add($"to_hour={toHour}");

            var queryString = queryParams.Count > 0 ? "?" + string.Join("&", queryParams) : "";
            var response = await _httpClient.GetAsync($"/v1/stats/traffic-types{queryString}");

            if (response.IsSuccessStatusCode)
            {
                var content = await response.Content.ReadAsStringAsync();
                var stats = JsonSerializer.Deserialize<List<TrafficTypeStatsDto>>(content, _jsonOptions);
                return Result<List<TrafficTypeStatsDto>>.Success(stats ?? new List<TrafficTypeStatsDto>());
            }

            return await HandleErrorResponse<List<TrafficTypeStatsDto>>(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to get traffic type stats");
            return Result<List<TrafficTypeStatsDto>>.Failure("EXTERNAL_SERVICE_ERROR", "Failed to get traffic type statistics");
        }
    }

    #endregion

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
}
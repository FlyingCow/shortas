using ShortasProxyApi.Domain.Entities;
using ShortasProxyApi.Domain.Interfaces;
using ShortasProxyApi.Domain.Common;
using System.Text.Json;

namespace ShortasProxyApi.Application.Services;

public class ClickStreamService : IClickStreamService
{
    private readonly HttpClient _httpClient;
    private readonly ILogger<ClickStreamService> _logger;
    private readonly JsonSerializerOptions _jsonOptions;

    public ClickStreamService(HttpClient httpClient, ILogger<ClickStreamService> logger)
    {
        _httpClient = httpClient;
        _logger = logger;
        _jsonOptions = new JsonSerializerOptions
        {
            PropertyNamingPolicy = JsonNamingPolicy.CamelCase,
            PropertyNameCaseInsensitive = true
        };
    }

    public async Task<Result<List<Domain.Entities.ClickStream>>> GetClickStreamAsync(string? routeId = null, DateTime? startDate = null, DateTime? endDate = null)
    {
        try
        {
            var queryParams = new List<string>();
            
            if (!string.IsNullOrEmpty(routeId))
                queryParams.Add($"routeId={Uri.EscapeDataString(routeId)}");
            if (startDate.HasValue)
                queryParams.Add($"startDate={startDate.Value:yyyy-MM-ddTHH:mm:ssZ}");
            if (endDate.HasValue)
                queryParams.Add($"endDate={endDate.Value:yyyy-MM-ddTHH:mm:ssZ}");
            
            var queryString = queryParams.Count > 0 ? "?" + string.Join("&", queryParams) : "";
            var url = routeId != null ? $"/v1/clickstream/{routeId}{queryString}" : $"/v1/clickstream{queryString}";
            
            var response = await _httpClient.GetAsync(url);
            
            if (response.IsSuccessStatusCode)
            {
                var content = await response.Content.ReadAsStringAsync();
                var clickStreams = JsonSerializer.Deserialize<List<Domain.Entities.ClickStream>>(content, _jsonOptions) ?? new List<Domain.Entities.ClickStream>();
                return Result<List<Domain.Entities.ClickStream>>.Success(clickStreams);
            }

            if (response.StatusCode == System.Net.HttpStatusCode.Unauthorized)
                return Result<List<Domain.Entities.ClickStream>>.Failure(Error.Unauthorized());

            if (response.StatusCode == System.Net.HttpStatusCode.Forbidden)
                return Result<List<Domain.Entities.ClickStream>>.Failure(Error.Forbidden());

            return Result<List<Domain.Entities.ClickStream>>.Failure(
                Error.ExternalService("ClickAggregatorApi", $"HTTP {response.StatusCode}: {response.ReasonPhrase}"));
        }
        catch (HttpRequestException ex)
        {
            _logger.LogError(ex, "HTTP error getting click stream data");
            return Result<List<Domain.Entities.ClickStream>>.Failure(Error.Network(ex.Message));
        }
        catch (TaskCanceledException ex) when (ex.InnerException is TimeoutException)
        {
            _logger.LogError(ex, "Timeout getting click stream data");
            return Result<List<Domain.Entities.ClickStream>>.Failure(Error.Timeout("ClickAggregatorApi", 30));
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Unexpected error getting click stream data");
            return Result<List<Domain.Entities.ClickStream>>.Failure(Error.Internal("An unexpected error occurred", ex.Message));
        }
    }

    public async Task<Result<Dictionary<string, object>>> GetClickStreamStatsAsync(string? routeId = null, DateTime? startDate = null, DateTime? endDate = null)
    {
        try
        {
            var queryParams = new List<string>();
            
            if (!string.IsNullOrEmpty(routeId))
                queryParams.Add($"routeId={Uri.EscapeDataString(routeId)}");
            if (startDate.HasValue)
                queryParams.Add($"startDate={startDate.Value:yyyy-MM-ddTHH:mm:ssZ}");
            if (endDate.HasValue)
                queryParams.Add($"endDate={endDate.Value:yyyy-MM-ddTHH:mm:ssZ}");
            
            var queryString = queryParams.Count > 0 ? "?" + string.Join("&", queryParams) : "";
            var url = $"/v1/clickstream/stats{queryString}";
            
            var response = await _httpClient.GetAsync(url);
            
            if (response.IsSuccessStatusCode)
            {
                var content = await response.Content.ReadAsStringAsync();
                var stats = JsonSerializer.Deserialize<Dictionary<string, object>>(content, _jsonOptions) ?? new Dictionary<string, object>();
                return Result<Dictionary<string, object>>.Success(stats);
            }

            if (response.StatusCode == System.Net.HttpStatusCode.Unauthorized)
                return Result<Dictionary<string, object>>.Failure(Error.Unauthorized());

            if (response.StatusCode == System.Net.HttpStatusCode.Forbidden)
                return Result<Dictionary<string, object>>.Failure(Error.Forbidden());

            return Result<Dictionary<string, object>>.Failure(
                Error.ExternalService("ClickAggregatorApi", $"HTTP {response.StatusCode}: {response.ReasonPhrase}"));
        }
        catch (HttpRequestException ex)
        {
            _logger.LogError(ex, "HTTP error getting click stream stats");
            return Result<Dictionary<string, object>>.Failure(Error.Network(ex.Message));
        }
        catch (TaskCanceledException ex) when (ex.InnerException is TimeoutException)
        {
            _logger.LogError(ex, "Timeout getting click stream stats");
            return Result<Dictionary<string, object>>.Failure(Error.Timeout("ClickAggregatorApi", 30));
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Unexpected error getting click stream stats");
            return Result<Dictionary<string, object>>.Failure(Error.Internal("An unexpected error occurred", ex.Message));
        }
    }
}
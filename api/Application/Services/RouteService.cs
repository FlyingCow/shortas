using ShortasProxyApi.Application.DTOs;
using ShortasProxyApi.Domain.Entities;
using ShortasProxyApi.Domain.Interfaces;
using ShortasProxyApi.Domain.Common;
using System.Text.Json;

namespace ShortasProxyApi.Application.Services;

public class RouteService : IRouteService
{
    private readonly HttpClient _httpClient;
    private readonly ILogger<RouteService> _logger;
    private readonly JsonSerializerOptions _jsonOptions;

    public RouteService(HttpClient httpClient, ILogger<RouteService> logger)
    {
        _httpClient = httpClient;
        _logger = logger;
        _jsonOptions = new JsonSerializerOptions
        {
            PropertyNamingPolicy = JsonNamingPolicy.CamelCase,
            PropertyNameCaseInsensitive = true
        };
    }

    public Task<Result<Domain.Entities.Route?>> GetRouteByIdAsync(Guid id, string userId)
    {
        // This HTTP client calls an external API that uses domain/path, not IDs
        // This method should not be used with this service - use EfRouteService instead
        return Task.FromResult(Result<Domain.Entities.Route?>.Failure(
            Error.Internal("GetRouteByIdAsync not supported in HTTP client service. Use EfRouteService or call GetRouteAsync with domain/path.")));
    }

    public async Task<Result<Domain.Entities.Route?>> GetRouteAsync(string domain, string path, string userId, string? switchParam = null)
    {
        try
        {
            if (string.IsNullOrWhiteSpace(domain))
                return Result<Domain.Entities.Route?>.Failure(Error.Required("domain"));

            if (string.IsNullOrWhiteSpace(path))
                return Result<Domain.Entities.Route?>.Failure(Error.Required("path"));

            var url = switchParam != null 
                ? $"/v1/routes/{switchParam}/{domain}/{path}"
                : $"/v1/routes/{domain}/{path}";
            
            var response = await _httpClient.GetAsync(url);
            
            if (response.IsSuccessStatusCode)
            {
                var content = await response.Content.ReadAsStringAsync();
                var route = JsonSerializer.Deserialize<Domain.Entities.Route>(content, _jsonOptions);
                return Result<Domain.Entities.Route?>.Success(route);
            }
            
            if (response.StatusCode == System.Net.HttpStatusCode.NotFound)
                return Result<Domain.Entities.Route?>.Success(null);
                
            if (response.StatusCode == System.Net.HttpStatusCode.Unauthorized)
                return Result<Domain.Entities.Route?>.Failure(Error.Unauthorized());

            if (response.StatusCode == System.Net.HttpStatusCode.Forbidden)
                return Result<Domain.Entities.Route?>.Failure(Error.Forbidden());

            return Result<Domain.Entities.Route?>.Failure(
                Error.ExternalService("ClickRouterApi", $"HTTP {response.StatusCode}: {response.ReasonPhrase}"));
        }
        catch (HttpRequestException ex)
        {
            _logger.LogError(ex, "HTTP error getting route for domain: {Domain}, path: {Path}", domain, path);
            return Result<Domain.Entities.Route?>.Failure(Error.Network(ex.Message));
        }
        catch (TaskCanceledException ex) when (ex.InnerException is TimeoutException)
        {
            _logger.LogError(ex, "Timeout getting route for domain: {Domain}, path: {Path}", domain, path);
            return Result<Domain.Entities.Route?>.Failure(Error.Timeout("ClickRouterApi", 30));
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Unexpected error getting route for domain: {Domain}, path: {Path}", domain, path);
            return Result<Domain.Entities.Route?>.Failure(Error.Internal("An unexpected error occurred", ex.Message));
        }
    }

    public async Task<Result<Domain.Entities.Route>> CreateRouteAsync(Domain.Entities.Route route)
    {
        try
        {
            var validationResult = route.Validate();
            if (!validationResult.IsValid)
            {
                var errors = string.Join(", ", validationResult.Errors.Select(e => $"{e.FieldName}: {e.Message}"));
                return Result<Domain.Entities.Route>.Failure(Error.Validation("Route validation failed", errors));
            }

            var json = JsonSerializer.Serialize(route, _jsonOptions);
            var content = new StringContent(json, System.Text.Encoding.UTF8, "application/json");
            
            var response = await _httpClient.PostAsync("/v1/routes", content);
            
            if (response.IsSuccessStatusCode)
            {
                var responseContent = await response.Content.ReadAsStringAsync();
                var createdRoute = JsonSerializer.Deserialize<Domain.Entities.Route>(responseContent, _jsonOptions) ?? route;
                return Result<Domain.Entities.Route>.Success(createdRoute);
            }

            if (response.StatusCode == System.Net.HttpStatusCode.Unauthorized)
                return Result<Domain.Entities.Route>.Failure(Error.Unauthorized());

            if (response.StatusCode == System.Net.HttpStatusCode.Forbidden)
                return Result<Domain.Entities.Route>.Failure(Error.Forbidden());

            if (response.StatusCode == System.Net.HttpStatusCode.Conflict)
                return Result<Domain.Entities.Route>.Failure(Error.Conflict("Route already exists"));

            return Result<Domain.Entities.Route>.Failure(
                Error.ExternalService("ClickRouterApi", $"HTTP {response.StatusCode}: {response.ReasonPhrase}"));
        }
        catch (HttpRequestException ex)
        {
            _logger.LogError(ex, "HTTP error creating route");
            return Result<Domain.Entities.Route>.Failure(Error.Network(ex.Message));
        }
        catch (TaskCanceledException ex) when (ex.InnerException is TimeoutException)
        {
            _logger.LogError(ex, "Timeout creating route");
            return Result<Domain.Entities.Route>.Failure(Error.Timeout("ClickRouterApi", 30));
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Unexpected error creating route");
            return Result<Domain.Entities.Route>.Failure(Error.Internal("An unexpected error occurred", ex.Message));
        }
    }

    public Task<Result<Domain.Entities.Route>> UpdateRouteByIdAsync(Guid id, string userId, Domain.Entities.Route route)
    {
        // This HTTP client calls an external API that uses domain/switch/path, not IDs
        // Adapter: Map fields to external API format
        // - domain → route.Properties.DomainId
        // - path → route.Link
        // - switch → route.Switch

        if (route.Properties == null || string.IsNullOrWhiteSpace(route.Properties.DomainId))
        {
            return Task.FromResult(Result<Domain.Entities.Route>.Failure(
                Error.Required("route.Properties.DomainId is required to update via external API")));
        }

        if (string.IsNullOrWhiteSpace(route.Link))
        {
            return Task.FromResult(Result<Domain.Entities.Route>.Failure(
                Error.Required("route.Link is required to update via external API")));
        }

        var domain = route.Properties.DomainId;
        var path = route.Link;

        return UpdateRouteAsync(domain, path, userId, route);
    }

    public async Task<Result<Domain.Entities.Route>> UpdateRouteAsync(string domain, string path, string userId, Domain.Entities.Route route)
    {
        try
        {
            if (string.IsNullOrWhiteSpace(domain))
                return Result<Domain.Entities.Route>.Failure(Error.Required("domain"));

            if (string.IsNullOrWhiteSpace(path))
                return Result<Domain.Entities.Route>.Failure(Error.Required("path"));

            var validationResult = route.Validate();
            if (!validationResult.IsValid)
            {
                var errors = string.Join(", ", validationResult.Errors.Select(e => $"{e.FieldName}: {e.Message}"));
                return Result<Domain.Entities.Route>.Failure(Error.Validation("Route validation failed", errors));
            }

            var json = JsonSerializer.Serialize(route, _jsonOptions);
            var content = new StringContent(json, System.Text.Encoding.UTF8, "application/json");
            
            var response = await _httpClient.PutAsync($"/v1/routes/{domain}/{path}", content);
            
            if (response.IsSuccessStatusCode)
            {
                var responseContent = await response.Content.ReadAsStringAsync();
                var updatedRoute = JsonSerializer.Deserialize<Domain.Entities.Route>(responseContent, _jsonOptions) ?? route;
                return Result<Domain.Entities.Route>.Success(updatedRoute);
            }

            if (response.StatusCode == System.Net.HttpStatusCode.NotFound)
                return Result<Domain.Entities.Route>.Failure(Error.NotFound("Route", $"{domain}/{path}"));

            if (response.StatusCode == System.Net.HttpStatusCode.Unauthorized)
                return Result<Domain.Entities.Route>.Failure(Error.Unauthorized());

            if (response.StatusCode == System.Net.HttpStatusCode.Forbidden)
                return Result<Domain.Entities.Route>.Failure(Error.Forbidden());

            return Result<Domain.Entities.Route>.Failure(
                Error.ExternalService("ClickRouterApi", $"HTTP {response.StatusCode}: {response.ReasonPhrase}"));
        }
        catch (HttpRequestException ex)
        {
            _logger.LogError(ex, "HTTP error updating route for domain: {Domain}, path: {Path}", domain, path);
            return Result<Domain.Entities.Route>.Failure(Error.Network(ex.Message));
        }
        catch (TaskCanceledException ex) when (ex.InnerException is TimeoutException)
        {
            _logger.LogError(ex, "Timeout updating route for domain: {Domain}, path: {Path}", domain, path);
            return Result<Domain.Entities.Route>.Failure(Error.Timeout("ClickRouterApi", 30));
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Unexpected error updating route for domain: {Domain}, path: {Path}", domain, path);
            return Result<Domain.Entities.Route>.Failure(Error.Internal("An unexpected error occurred", ex.Message));
        }
    }

    public Task<Result> DeleteRouteByIdAsync(Guid id, string userId)
    {
        // This HTTP client calls an external API that uses domain/path, not IDs
        // Adapter: First fetch the route to get domain/path, then delete
        // Note: This requires two API calls, which is inefficient
        // For production, consider using EfRouteService for ID-based operations

        _logger.LogWarning("DeleteRouteByIdAsync in HTTP client requires fetching route first. Consider using EfRouteService for better performance.");

        return Task.FromResult(Result.Failure(Error.Internal(
            "DeleteRouteByIdAsync not efficiently supported in HTTP client service. " +
            "Use EfRouteService or call DeleteRouteAsync with domain/path.")));
    }

    public async Task<Result> DeleteRouteAsync(string domain, string path, string userId)
    {
        try
        {
            if (string.IsNullOrWhiteSpace(domain))
                return Result.Failure(Error.Required("domain"));

            if (string.IsNullOrWhiteSpace(path))
                return Result.Failure(Error.Required("path"));

            var response = await _httpClient.DeleteAsync($"/v1/routes/{domain}/{path}");
            
            if (response.IsSuccessStatusCode)
                return Result.Success();

            if (response.StatusCode == System.Net.HttpStatusCode.NotFound)
                return Result.Failure(Error.NotFound("Route", $"{domain}/{path}"));

            if (response.StatusCode == System.Net.HttpStatusCode.Unauthorized)
                return Result.Failure(Error.Unauthorized());

            if (response.StatusCode == System.Net.HttpStatusCode.Forbidden)
                return Result.Failure(Error.Forbidden());

            return Result.Failure(
                Error.ExternalService("ClickRouterApi", $"HTTP {response.StatusCode}: {response.ReasonPhrase}"));
        }
        catch (HttpRequestException ex)
        {
            _logger.LogError(ex, "HTTP error deleting route for domain: {Domain}, path: {Path}", domain, path);
            return Result.Failure(Error.Network(ex.Message));
        }
        catch (TaskCanceledException ex) when (ex.InnerException is TimeoutException)
        {
            _logger.LogError(ex, "Timeout deleting route for domain: {Domain}, path: {Path}", domain, path);
            return Result.Failure(Error.Timeout("ClickRouterApi", 30));
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Unexpected error deleting route for domain: {Domain}, path: {Path}", domain, path);
            return Result.Failure(Error.Internal("An unexpected error occurred", ex.Message));
        }
    }

    public async Task<Result<List<Domain.Entities.Route>>> BulkCreateRoutesAsync(List<Domain.Entities.Route> routes)
    {
        try
        {
            if (routes == null || !routes.Any())
                return Result<List<Domain.Entities.Route>>.Failure(Error.Validation("Routes list cannot be empty"));

            foreach (var route in routes)
            {
                var validationResult = route.Validate();
                if (!validationResult.IsValid)
                {
                    var errors = string.Join(", ", validationResult.Errors.Select(e => $"{e.FieldName}: {e.Message}"));
                    return Result<List<Domain.Entities.Route>>.Failure(Error.Validation("Route validation failed", errors));
                }
            }

            var json = JsonSerializer.Serialize(routes, _jsonOptions);
            var content = new StringContent(json, System.Text.Encoding.UTF8, "application/json");
            
            var response = await _httpClient.PostAsync("/v1/routes/bulk", content);
            
            if (response.IsSuccessStatusCode)
            {
                var responseContent = await response.Content.ReadAsStringAsync();
                var createdRoutes = JsonSerializer.Deserialize<List<Domain.Entities.Route>>(responseContent, _jsonOptions) ?? routes;
                return Result<List<Domain.Entities.Route>>.Success(createdRoutes);
            }

            if (response.StatusCode == System.Net.HttpStatusCode.Unauthorized)
                return Result<List<Domain.Entities.Route>>.Failure(Error.Unauthorized());

            if (response.StatusCode == System.Net.HttpStatusCode.Forbidden)
                return Result<List<Domain.Entities.Route>>.Failure(Error.Forbidden());

            return Result<List<Domain.Entities.Route>>.Failure(
                Error.ExternalService("ClickRouterApi", $"HTTP {response.StatusCode}: {response.ReasonPhrase}"));
        }
        catch (HttpRequestException ex)
        {
            _logger.LogError(ex, "HTTP error bulk creating routes");
            return Result<List<Domain.Entities.Route>>.Failure(Error.Network(ex.Message));
        }
        catch (TaskCanceledException ex) when (ex.InnerException is TimeoutException)
        {
            _logger.LogError(ex, "Timeout bulk creating routes");
            return Result<List<Domain.Entities.Route>>.Failure(Error.Timeout("ClickRouterApi", 30));
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Unexpected error bulk creating routes");
            return Result<List<Domain.Entities.Route>>.Failure(Error.Internal("An unexpected error occurred", ex.Message));
        }
    }

    public async Task<Result<List<Domain.Entities.Route>>> BulkUpdateRoutesAsync(string userId, List<Domain.Entities.Route> routes)
    {
        try
        {
            if (routes == null || !routes.Any())
                return Result<List<Domain.Entities.Route>>.Failure(Error.Validation("Routes list cannot be empty"));

            foreach (var route in routes)
            {
                var validationResult = route.Validate();
                if (!validationResult.IsValid)
                {
                    var errors = string.Join(", ", validationResult.Errors.Select(e => $"{e.FieldName}: {e.Message}"));
                    return Result<List<Domain.Entities.Route>>.Failure(Error.Validation("Route validation failed", errors));
                }
            }

            var json = JsonSerializer.Serialize(routes, _jsonOptions);
            var content = new StringContent(json, System.Text.Encoding.UTF8, "application/json");
            
            var response = await _httpClient.PutAsync("/v1/routes/bulk", content);
            
            if (response.IsSuccessStatusCode)
            {
                var responseContent = await response.Content.ReadAsStringAsync();
                var updatedRoutes = JsonSerializer.Deserialize<List<Domain.Entities.Route>>(responseContent, _jsonOptions) ?? routes;
                return Result<List<Domain.Entities.Route>>.Success(updatedRoutes);
            }

            if (response.StatusCode == System.Net.HttpStatusCode.Unauthorized)
                return Result<List<Domain.Entities.Route>>.Failure(Error.Unauthorized());

            if (response.StatusCode == System.Net.HttpStatusCode.Forbidden)
                return Result<List<Domain.Entities.Route>>.Failure(Error.Forbidden());

            return Result<List<Domain.Entities.Route>>.Failure(
                Error.ExternalService("ClickRouterApi", $"HTTP {response.StatusCode}: {response.ReasonPhrase}"));
        }
        catch (HttpRequestException ex)
        {
            _logger.LogError(ex, "HTTP error bulk updating routes");
            return Result<List<Domain.Entities.Route>>.Failure(Error.Network(ex.Message));
        }
        catch (TaskCanceledException ex) when (ex.InnerException is TimeoutException)
        {
            _logger.LogError(ex, "Timeout bulk updating routes");
            return Result<List<Domain.Entities.Route>>.Failure(Error.Timeout("ClickRouterApi", 30));
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Unexpected error bulk updating routes");
            return Result<List<Domain.Entities.Route>>.Failure(Error.Internal("An unexpected error occurred", ex.Message));
        }
    }

    public async Task<Result> BulkDeleteRoutesAsync(string userId, List<string> routeIds)
    {
        try
        {
            if (routeIds == null || !routeIds.Any())
                return Result.Failure(Error.Validation("Route IDs list cannot be empty"));

            var json = JsonSerializer.Serialize(routeIds, _jsonOptions);
            var content = new StringContent(json, System.Text.Encoding.UTF8, "application/json");

            // DELETE with body requires HttpRequestMessage
            var request = new HttpRequestMessage(HttpMethod.Delete, "/v1/routes/bulk")
            {
                Content = content
            };

            var response = await _httpClient.SendAsync(request);

            if (response.IsSuccessStatusCode)
                return Result.Success();

            if (response.StatusCode == System.Net.HttpStatusCode.Unauthorized)
                return Result.Failure(Error.Unauthorized());

            if (response.StatusCode == System.Net.HttpStatusCode.Forbidden)
                return Result.Failure(Error.Forbidden());

            return Result.Failure(
                Error.ExternalService("ClickRouterApi", $"HTTP {response.StatusCode}: {response.ReasonPhrase}"));
        }
        catch (HttpRequestException ex)
        {
            _logger.LogError(ex, "HTTP error bulk deleting routes");
            return Result.Failure(Error.Network(ex.Message));
        }
        catch (TaskCanceledException ex) when (ex.InnerException is TimeoutException)
        {
            _logger.LogError(ex, "Timeout bulk deleting routes");
            return Result.Failure(Error.Timeout("ClickRouterApi", 30));
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Unexpected error bulk deleting routes");
            return Result.Failure(Error.Internal("An unexpected error occurred", ex.Message));
        }
    }

    public Task<Result<(List<Domain.Entities.Route> Routes, int TotalCount)>> ListRoutesAsync(
        int page = 1,
        int pageSize = 20,
        string? search = null,
        string? status = null,
        string? ownerId = null)
    {
        // This method is not implemented in the HTTP client proxy service
        // It should only be called when using the EF-based service
        throw new NotImplementedException("ListRoutesAsync is only available in EF-based service");
    }
}
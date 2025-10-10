using ShortasProxyApi.Domain.Entities;
using ShortasProxyApi.Domain.Interfaces;
using ShortasProxyApi.Domain.Common;
using System.Text.Json;

namespace ShortasProxyApi.Application.Services;

public class UserSettingsService : IUserSettingsService
{
    private readonly HttpClient _httpClient;
    private readonly ILogger<UserSettingsService> _logger;
    private readonly JsonSerializerOptions _jsonOptions;

    public UserSettingsService(HttpClient httpClient, ILogger<UserSettingsService> logger)
    {
        _httpClient = httpClient;
        _logger = logger;
        _jsonOptions = new JsonSerializerOptions
        {
            PropertyNamingPolicy = JsonNamingPolicy.CamelCase,
            PropertyNameCaseInsensitive = true
        };
    }

    public async Task<Result<Domain.Entities.UserSettings?>> GetUserSettingsAsync(string userId)
    {
        try
        {
            if (string.IsNullOrWhiteSpace(userId))
                return Result<Domain.Entities.UserSettings?>.Failure(Error.Required("userId"));

            var response = await _httpClient.GetAsync($"/v1/user-settings/{userId}");
            
            if (response.IsSuccessStatusCode)
            {
                var content = await response.Content.ReadAsStringAsync();
                var userSettings = JsonSerializer.Deserialize<Domain.Entities.UserSettings>(content, _jsonOptions);
                return Result<Domain.Entities.UserSettings?>.Success(userSettings);
            }
            
            if (response.StatusCode == System.Net.HttpStatusCode.NotFound)
                return Result<Domain.Entities.UserSettings?>.Success(null);
                
            if (response.StatusCode == System.Net.HttpStatusCode.Unauthorized)
                return Result<Domain.Entities.UserSettings?>.Failure(Error.Unauthorized());

            if (response.StatusCode == System.Net.HttpStatusCode.Forbidden)
                return Result<Domain.Entities.UserSettings?>.Failure(Error.Forbidden());

            return Result<Domain.Entities.UserSettings?>.Failure(
                Error.ExternalService("ClickRouterApi", $"HTTP {response.StatusCode}: {response.ReasonPhrase}"));
        }
        catch (HttpRequestException ex)
        {
            _logger.LogError(ex, "HTTP error getting user settings for user: {UserId}", userId);
            return Result<Domain.Entities.UserSettings?>.Failure(Error.Network(ex.Message));
        }
        catch (TaskCanceledException ex) when (ex.InnerException is TimeoutException)
        {
            _logger.LogError(ex, "Timeout getting user settings for user: {UserId}", userId);
            return Result<Domain.Entities.UserSettings?>.Failure(Error.Timeout("ClickRouterApi", 30));
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Unexpected error getting user settings for user: {UserId}", userId);
            return Result<Domain.Entities.UserSettings?>.Failure(Error.Internal("An unexpected error occurred", ex.Message));
        }
    }

    public async Task<Result<Domain.Entities.UserSettings>> CreateUserSettingsAsync(string userId, Domain.Entities.UserSettings settings)
    {
        try
        {
            if (string.IsNullOrWhiteSpace(userId))
                return Result<Domain.Entities.UserSettings>.Failure(Error.Required("userId"));

            var validationResult = settings.Validate();
            if (!validationResult.IsValid)
            {
                var errors = string.Join(", ", validationResult.Errors.Select(e => $"{e.FieldName}: {e.Message}"));
                return Result<Domain.Entities.UserSettings>.Failure(Error.Validation("User settings validation failed", errors));
            }

            var json = JsonSerializer.Serialize(settings, _jsonOptions);
            var content = new StringContent(json, System.Text.Encoding.UTF8, "application/json");
            
            var response = await _httpClient.PostAsync($"/v1/user-settings/{userId}", content);
            
            if (response.IsSuccessStatusCode)
            {
                var responseContent = await response.Content.ReadAsStringAsync();
                var createdSettings = JsonSerializer.Deserialize<Domain.Entities.UserSettings>(responseContent, _jsonOptions) ?? settings;
                return Result<Domain.Entities.UserSettings>.Success(createdSettings);
            }

            if (response.StatusCode == System.Net.HttpStatusCode.Unauthorized)
                return Result<Domain.Entities.UserSettings>.Failure(Error.Unauthorized());

            if (response.StatusCode == System.Net.HttpStatusCode.Forbidden)
                return Result<Domain.Entities.UserSettings>.Failure(Error.Forbidden());

            if (response.StatusCode == System.Net.HttpStatusCode.Conflict)
                return Result<Domain.Entities.UserSettings>.Failure(Error.Conflict("User settings already exist for this user"));

            return Result<Domain.Entities.UserSettings>.Failure(
                Error.ExternalService("ClickRouterApi", $"HTTP {response.StatusCode}: {response.ReasonPhrase}"));
        }
        catch (HttpRequestException ex)
        {
            _logger.LogError(ex, "HTTP error creating user settings for user: {UserId}", userId);
            return Result<Domain.Entities.UserSettings>.Failure(Error.Network(ex.Message));
        }
        catch (TaskCanceledException ex) when (ex.InnerException is TimeoutException)
        {
            _logger.LogError(ex, "Timeout creating user settings for user: {UserId}", userId);
            return Result<Domain.Entities.UserSettings>.Failure(Error.Timeout("ClickRouterApi", 30));
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Unexpected error creating user settings for user: {UserId}", userId);
            return Result<Domain.Entities.UserSettings>.Failure(Error.Internal("An unexpected error occurred", ex.Message));
        }
    }

    public async Task<Result<Domain.Entities.UserSettings>> UpdateUserSettingsAsync(string userId, Domain.Entities.UserSettings settings)
    {
        try
        {
            if (string.IsNullOrWhiteSpace(userId))
                return Result<Domain.Entities.UserSettings>.Failure(Error.Required("userId"));

            var validationResult = settings.Validate();
            if (!validationResult.IsValid)
            {
                var errors = string.Join(", ", validationResult.Errors.Select(e => $"{e.FieldName}: {e.Message}"));
                return Result<Domain.Entities.UserSettings>.Failure(Error.Validation("User settings validation failed", errors));
            }

            var json = JsonSerializer.Serialize(settings, _jsonOptions);
            var content = new StringContent(json, System.Text.Encoding.UTF8, "application/json");
            
            var response = await _httpClient.PutAsync($"/v1/user-settings/{userId}", content);
            
            if (response.IsSuccessStatusCode)
            {
                var responseContent = await response.Content.ReadAsStringAsync();
                var updatedSettings = JsonSerializer.Deserialize<Domain.Entities.UserSettings>(responseContent, _jsonOptions) ?? settings;
                return Result<Domain.Entities.UserSettings>.Success(updatedSettings);
            }

            if (response.StatusCode == System.Net.HttpStatusCode.NotFound)
                return Result<Domain.Entities.UserSettings>.Failure(Error.NotFound("User settings", userId));

            if (response.StatusCode == System.Net.HttpStatusCode.Unauthorized)
                return Result<Domain.Entities.UserSettings>.Failure(Error.Unauthorized());

            if (response.StatusCode == System.Net.HttpStatusCode.Forbidden)
                return Result<Domain.Entities.UserSettings>.Failure(Error.Forbidden());

            return Result<Domain.Entities.UserSettings>.Failure(
                Error.ExternalService("ClickRouterApi", $"HTTP {response.StatusCode}: {response.ReasonPhrase}"));
        }
        catch (HttpRequestException ex)
        {
            _logger.LogError(ex, "HTTP error updating user settings for user: {UserId}", userId);
            return Result<Domain.Entities.UserSettings>.Failure(Error.Network(ex.Message));
        }
        catch (TaskCanceledException ex) when (ex.InnerException is TimeoutException)
        {
            _logger.LogError(ex, "Timeout updating user settings for user: {UserId}", userId);
            return Result<Domain.Entities.UserSettings>.Failure(Error.Timeout("ClickRouterApi", 30));
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Unexpected error updating user settings for user: {UserId}", userId);
            return Result<Domain.Entities.UserSettings>.Failure(Error.Internal("An unexpected error occurred", ex.Message));
        }
    }

    public async Task<Result> DeleteUserSettingsAsync(string userId)
    {
        try
        {
            if (string.IsNullOrWhiteSpace(userId))
                return Result.Failure(Error.Required("userId"));

            var response = await _httpClient.DeleteAsync($"/v1/user-settings/{userId}");
            
            if (response.IsSuccessStatusCode)
                return Result.Success();

            if (response.StatusCode == System.Net.HttpStatusCode.NotFound)
                return Result.Failure(Error.NotFound("User settings", userId));

            if (response.StatusCode == System.Net.HttpStatusCode.Unauthorized)
                return Result.Failure(Error.Unauthorized());

            if (response.StatusCode == System.Net.HttpStatusCode.Forbidden)
                return Result.Failure(Error.Forbidden());

            return Result.Failure(
                Error.ExternalService("ClickRouterApi", $"HTTP {response.StatusCode}: {response.ReasonPhrase}"));
        }
        catch (HttpRequestException ex)
        {
            _logger.LogError(ex, "HTTP error deleting user settings for user: {UserId}", userId);
            return Result.Failure(Error.Network(ex.Message));
        }
        catch (TaskCanceledException ex) when (ex.InnerException is TimeoutException)
        {
            _logger.LogError(ex, "Timeout deleting user settings for user: {UserId}", userId);
            return Result.Failure(Error.Timeout("ClickRouterApi", 30));
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Unexpected error deleting user settings for user: {UserId}", userId);
            return Result.Failure(Error.Internal("An unexpected error occurred", ex.Message));
        }
    }
}
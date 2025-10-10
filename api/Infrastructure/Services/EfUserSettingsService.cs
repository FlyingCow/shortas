using Microsoft.EntityFrameworkCore;
using System.Text.Json;
using ShortasProxyApi.Domain.Common;
using ShortasProxyApi.Domain.Entities;
using ShortasProxyApi.Domain.Interfaces;
using ShortasProxyApi.Infrastructure.Data;

namespace ShortasProxyApi.Infrastructure.Services;

public class EfUserSettingsService : IUserSettingsService
{
    private readonly ApplicationDbContext _context;
    private readonly IOutboxRepository _outboxRepository;
    private readonly ILogger<EfUserSettingsService> _logger;
    private readonly JsonSerializerOptions _jsonOptions;

    public EfUserSettingsService(
        ApplicationDbContext context,
        IOutboxRepository outboxRepository,
        ILogger<EfUserSettingsService> logger)
    {
        _context = context;
        _outboxRepository = outboxRepository;
        _logger = logger;
        _jsonOptions = new JsonSerializerOptions
        {
            PropertyNamingPolicy = JsonNamingPolicy.CamelCase
        };
    }

    public async Task<Result<UserSettings?>> GetUserSettingsAsync(string userId)
    {
        try
        {
            if (string.IsNullOrWhiteSpace(userId))
                return Result<UserSettings?>.Failure(Error.Required("userId"));

            var userSettings = await _context.UserSettings
                .FirstOrDefaultAsync(u => u.Email == userId);

            return Result<UserSettings?>.Success(userSettings);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error getting user settings for user: {UserId}", userId);
            return Result<UserSettings?>.Failure(Error.Internal("Failed to get user settings", ex.Message));
        }
    }

    public async Task<Result<UserSettings>> CreateUserSettingsAsync(string userId, UserSettings settings)
    {
        using var transaction = await _context.Database.BeginTransactionAsync();

        try
        {
            if (string.IsNullOrWhiteSpace(userId))
                return Result<UserSettings>.Failure(Error.Required("userId"));

            if (settings == null)
                return Result<UserSettings>.Failure(Error.Required("settings"));

            // Check if settings already exist
            var existing = await _context.UserSettings
                .FirstOrDefaultAsync(u => u.Email == userId);

            if (existing != null)
                return Result<UserSettings>.Failure(Error.Conflict("User settings already exist for this user"));

            // Set the user ID
            settings.Email = userId;

            // Add settings to database
            await _context.UserSettings.AddAsync(settings);
            await _context.SaveChangesAsync();

            // Create outbox message for eventual consistency with click-router-api
            var outboxMessage = new OutboxMessage
            {
                EventType = OutboxEventType.UserSettingsCreated,
                AggregateId = settings.Id.ToString(),
                Payload = JsonSerializer.Serialize(settings, _jsonOptions),
                CreatedAt = DateTime.UtcNow,
                Status = OutboxMessageStatus.Pending
            };

            await _outboxRepository.AddAsync(outboxMessage);
            await _outboxRepository.SaveChangesAsync();

            await transaction.CommitAsync();

            _logger.LogInformation("User settings created: {SettingsId}, UserId: {UserId}", settings.Id, userId);

            return Result<UserSettings>.Success(settings);
        }
        catch (Exception ex)
        {
            await transaction.RollbackAsync();
            _logger.LogError(ex, "Error creating user settings for user: {UserId}", userId);
            return Result<UserSettings>.Failure(Error.Internal("Failed to create user settings", ex.Message));
        }
    }

    public async Task<Result<UserSettings>> UpdateUserSettingsAsync(string userId, UserSettings settings)
    {
        using var transaction = await _context.Database.BeginTransactionAsync();

        try
        {
            if (string.IsNullOrWhiteSpace(userId))
                return Result<UserSettings>.Failure(Error.Required("userId"));

            if (settings == null)
                return Result<UserSettings>.Failure(Error.Required("settings"));

            // Find existing settings
            var existingSettings = await _context.UserSettings
                .FirstOrDefaultAsync(u => u.Email == userId);

            if (existingSettings == null)
                return Result<UserSettings>.Failure(Error.NotFound("UserSettings", userId));

            // Update settings properties
            existingSettings.Status = settings.Status;
            existingSettings.Debug = settings.Debug;
            existingSettings.Overflow = settings.Overflow;
            existingSettings.SkipTracking = settings.SkipTracking;
            existingSettings.AllowedRequestParams = settings.AllowedRequestParams;
            existingSettings.AllowedDestinationParams = settings.AllowedDestinationParams;

            _context.UserSettings.Update(existingSettings);
            await _context.SaveChangesAsync();

            // Create outbox message
            var outboxMessage = new OutboxMessage
            {
                EventType = OutboxEventType.UserSettingsUpdated,
                AggregateId = existingSettings.Id.ToString(),
                Payload = JsonSerializer.Serialize(existingSettings, _jsonOptions),
                CreatedAt = DateTime.UtcNow,
                Status = OutboxMessageStatus.Pending
            };

            await _outboxRepository.AddAsync(outboxMessage);
            await _outboxRepository.SaveChangesAsync();

            await transaction.CommitAsync();

            _logger.LogInformation("User settings updated: {SettingsId}, UserId: {UserId}", existingSettings.Id, userId);

            return Result<UserSettings>.Success(existingSettings);
        }
        catch (Exception ex)
        {
            await transaction.RollbackAsync();
            _logger.LogError(ex, "Error updating user settings for user: {UserId}", userId);
            return Result<UserSettings>.Failure(Error.Internal("Failed to update user settings", ex.Message));
        }
    }

    public async Task<Result> DeleteUserSettingsAsync(string userId)
    {
        using var transaction = await _context.Database.BeginTransactionAsync();

        try
        {
            if (string.IsNullOrWhiteSpace(userId))
                return Result.Failure(Error.Required("userId"));

            // Find existing settings
            var userSettings = await _context.UserSettings
                .FirstOrDefaultAsync(u => u.Email == userId);

            if (userSettings == null)
                return Result.Failure(Error.NotFound("UserSettings", userId));

            // Delete settings
            _context.UserSettings.Remove(userSettings);
            await _context.SaveChangesAsync();

            // Create outbox message
            var outboxMessage = new OutboxMessage
            {
                EventType = OutboxEventType.UserSettingsDeleted,
                AggregateId = userSettings.Id.ToString(),
                Payload = JsonSerializer.Serialize(new { UserId = userId, SettingsId = userSettings.Id }, _jsonOptions),
                CreatedAt = DateTime.UtcNow,
                Status = OutboxMessageStatus.Pending
            };

            await _outboxRepository.AddAsync(outboxMessage);
            await _outboxRepository.SaveChangesAsync();

            await transaction.CommitAsync();

            _logger.LogInformation("User settings deleted: {SettingsId}, UserId: {UserId}", userSettings.Id, userId);

            return Result.Success();
        }
        catch (Exception ex)
        {
            await transaction.RollbackAsync();
            _logger.LogError(ex, "Error deleting user settings for user: {UserId}", userId);
            return Result.Failure(Error.Internal("Failed to delete user settings", ex.Message));
        }
    }
}

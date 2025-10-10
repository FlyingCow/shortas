using ShortasProxyApi.Domain.Entities;
using ShortasProxyApi.Domain.Common;

namespace ShortasProxyApi.Domain.Interfaces;

public interface IUserSettingsService
{
    Task<Result<Entities.UserSettings?>> GetUserSettingsAsync(string userId);
    Task<Result<Entities.UserSettings>> CreateUserSettingsAsync(string userId, Entities.UserSettings settings);
    Task<Result<Entities.UserSettings>> UpdateUserSettingsAsync(string userId, Entities.UserSettings settings);
    Task<Result> DeleteUserSettingsAsync(string userId);
}
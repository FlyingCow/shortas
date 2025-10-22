using ShortasProxyApi.Domain.Entities;
using ShortasProxyApi.Domain.Common;

namespace ShortasProxyApi.Domain.Interfaces;

public interface IWorkspaceService
{
    Task<Result<Workspace>> CreateWorkspaceAsync(string name, string description, string userId, string type = "User");
    Task<Result<Workspace?>> GetWorkspaceByIdAsync(Guid id, string userId);
    Task<Result<List<Workspace>>> ListUserWorkspacesAsync(string userId);
    Task<Result<Workspace>> UpdateWorkspaceAsync(Guid id, string userId, string? name, string? description);
    Task<Result> DeleteWorkspaceAsync(Guid id, string userId);
    Task<Result> AddUserToWorkspaceAsync(Guid workspaceId, string userId, string targetUserId, string role = "Member");
    Task<Result> RemoveUserFromWorkspaceAsync(Guid workspaceId, string userId, string targetUserId);
    Task<Result> UpdateUserRoleAsync(Guid workspaceId, string userId, string targetUserId, string newRole);
    Task<Result<List<UserWorkspace>>> GetWorkspaceMembersAsync(Guid workspaceId, string userId);
}

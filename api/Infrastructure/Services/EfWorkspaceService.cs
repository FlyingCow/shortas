using Microsoft.EntityFrameworkCore;
using ShortasProxyApi.Domain.Common;
using ShortasProxyApi.Domain.Entities;
using ShortasProxyApi.Domain.Interfaces;
using ShortasProxyApi.Infrastructure.Data;

namespace ShortasProxyApi.Infrastructure.Services;

public class EfWorkspaceService : IWorkspaceService
{
    private readonly ApplicationDbContext _context;
    private readonly ILogger<EfWorkspaceService> _logger;

    public EfWorkspaceService(
        ApplicationDbContext context,
        ILogger<EfWorkspaceService> logger)
    {
        _context = context;
        _logger = logger;
    }

    public async Task<Result<Workspace>> CreateWorkspaceAsync(string name, string description, string userId, string type = "User")
    {
        try
        {
            if (string.IsNullOrWhiteSpace(name))
                return Result<Workspace>.Failure(Error.Required("name"));

            if (string.IsNullOrWhiteSpace(userId))
                return Result<Workspace>.Failure(Error.Required("userId"));

            // Validate type
            if (type != "System" && type != "User")
                return Result<Workspace>.Failure(Error.Validation("Type must be either 'System' or 'User'"));

            var workspace = new Workspace
            {
                Name = name,
                Description = description ?? string.Empty,
                Type = type,
                CreatedAt = DateTime.UtcNow,
                UpdatedAt = DateTime.UtcNow
            };

            _context.Workspaces.Add(workspace);

            // Add the creator as Owner
            var userWorkspace = new UserWorkspace
            {
                UserId = userId,
                WorkspaceId = workspace.Id,
                Role = "Owner",
                JoinedAt = DateTime.UtcNow
            };

            _context.UserWorkspaces.Add(userWorkspace);

            await _context.SaveChangesAsync();

            return Result<Workspace>.Success(workspace);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error creating workspace: {Name}", name);
            return Result<Workspace>.Failure(Error.Internal("Failed to create workspace", ex.Message));
        }
    }

    public async Task<Result<Workspace?>> GetWorkspaceByIdAsync(Guid id, string userId)
    {
        try
        {
            if (string.IsNullOrWhiteSpace(userId))
                return Result<Workspace?>.Failure(Error.Required("userId"));

            var workspace = await _context.Workspaces
                .Include(w => w.UserWorkspaces)
                .FirstOrDefaultAsync(w => w.Id == id &&
                                        w.UserWorkspaces.Any(uw => uw.UserId == userId));

            if (workspace == null)
                return Result<Workspace?>.Failure(Error.NotFound("Workspace", id.ToString()));

            return Result<Workspace?>.Success(workspace);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error getting workspace by ID: {WorkspaceId}", id);
            return Result<Workspace?>.Failure(Error.Internal("Failed to get workspace", ex.Message));
        }
    }

    public async Task<Result<List<Workspace>>> ListUserWorkspacesAsync(string userId)
    {
        try
        {
            if (string.IsNullOrWhiteSpace(userId))
                return Result<List<Workspace>>.Failure(Error.Required("userId"));

            var workspaces = await _context.Workspaces
                .Include(w => w.UserWorkspaces)
                .Where(w => w.UserWorkspaces.Any(uw => uw.UserId == userId))
                .OrderByDescending(w => w.UpdatedAt)
                .ToListAsync();

            return Result<List<Workspace>>.Success(workspaces);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error listing workspaces for user: {UserId}", userId);
            return Result<List<Workspace>>.Failure(Error.Internal("Failed to list workspaces", ex.Message));
        }
    }

    public async Task<Result<Workspace>> UpdateWorkspaceAsync(Guid id, string userId, string? name, string? description)
    {
        try
        {
            if (string.IsNullOrWhiteSpace(userId))
                return Result<Workspace>.Failure(Error.Required("userId"));

            var workspace = await _context.Workspaces
                .Include(w => w.UserWorkspaces)
                .FirstOrDefaultAsync(w => w.Id == id);

            if (workspace == null)
                return Result<Workspace>.Failure(Error.NotFound("Workspace", id.ToString()));

            // Check if workspace is a system workspace
            if (workspace.IsSystem)
                return Result<Workspace>.Failure(Error.Forbidden("System workspaces cannot be modified"));

            // Check if user is Owner or Admin
            var userWorkspace = workspace.UserWorkspaces.FirstOrDefault(uw => uw.UserId == userId);
            if (userWorkspace == null || (userWorkspace.Role != "Owner" && userWorkspace.Role != "Admin"))
                return Result<Workspace>.Failure(Error.Forbidden("Only workspace owners and admins can update workspace"));

            if (!string.IsNullOrWhiteSpace(name))
                workspace.Name = name;

            if (description != null)
                workspace.Description = description;

            workspace.UpdatedAt = DateTime.UtcNow;

            await _context.SaveChangesAsync();

            return Result<Workspace>.Success(workspace);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error updating workspace: {WorkspaceId}", id);
            return Result<Workspace>.Failure(Error.Internal("Failed to update workspace", ex.Message));
        }
    }

    public async Task<Result> DeleteWorkspaceAsync(Guid id, string userId)
    {
        try
        {
            if (string.IsNullOrWhiteSpace(userId))
                return Result.Failure(Error.Required("userId"));

            var workspace = await _context.Workspaces
                .Include(w => w.UserWorkspaces)
                .FirstOrDefaultAsync(w => w.Id == id);

            if (workspace == null)
                return Result.Failure(Error.NotFound("Workspace", id.ToString()));

            // Check if workspace is a system workspace
            if (workspace.IsSystem)
                return Result.Failure(Error.Forbidden("System workspaces cannot be deleted"));

            // Check if user is Owner
            var userWorkspace = workspace.UserWorkspaces.FirstOrDefault(uw => uw.UserId == userId);
            if (userWorkspace == null || userWorkspace.Role != "Owner")
                return Result.Failure(Error.Forbidden("Only workspace owner can delete workspace"));

            _context.Workspaces.Remove(workspace);
            await _context.SaveChangesAsync();

            return Result.Success();
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error deleting workspace: {WorkspaceId}", id);
            return Result.Failure(Error.Internal("Failed to delete workspace", ex.Message));
        }
    }

    public async Task<Result> AddUserToWorkspaceAsync(Guid workspaceId, string userId, string targetUserId, string role = "Member")
    {
        try
        {
            if (string.IsNullOrWhiteSpace(userId))
                return Result.Failure(Error.Required("userId"));

            if (string.IsNullOrWhiteSpace(targetUserId))
                return Result.Failure(Error.Required("targetUserId"));

            var workspace = await _context.Workspaces
                .Include(w => w.UserWorkspaces)
                .FirstOrDefaultAsync(w => w.Id == workspaceId);

            if (workspace == null)
                return Result.Failure(Error.NotFound("Workspace", workspaceId.ToString()));

            // Check if requesting user is Owner or Admin
            var requestingUserWorkspace = workspace.UserWorkspaces.FirstOrDefault(uw => uw.UserId == userId);
            if (requestingUserWorkspace == null || (requestingUserWorkspace.Role != "Owner" && requestingUserWorkspace.Role != "Admin"))
                return Result.Failure(Error.Forbidden("Only workspace owners and admins can add users"));

            // Check if target user is already a member
            if (workspace.UserWorkspaces.Any(uw => uw.UserId == targetUserId))
                return Result.Failure(Error.Conflict("User is already a member of this workspace"));

            var userWorkspace = new UserWorkspace
            {
                UserId = targetUserId,
                WorkspaceId = workspaceId,
                Role = role,
                JoinedAt = DateTime.UtcNow
            };

            _context.UserWorkspaces.Add(userWorkspace);
            await _context.SaveChangesAsync();

            return Result.Success();
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error adding user to workspace: {WorkspaceId}", workspaceId);
            return Result.Failure(Error.Internal("Failed to add user to workspace", ex.Message));
        }
    }

    public async Task<Result> RemoveUserFromWorkspaceAsync(Guid workspaceId, string userId, string targetUserId)
    {
        try
        {
            if (string.IsNullOrWhiteSpace(userId))
                return Result.Failure(Error.Required("userId"));

            if (string.IsNullOrWhiteSpace(targetUserId))
                return Result.Failure(Error.Required("targetUserId"));

            var workspace = await _context.Workspaces
                .Include(w => w.UserWorkspaces)
                .FirstOrDefaultAsync(w => w.Id == workspaceId);

            if (workspace == null)
                return Result.Failure(Error.NotFound("Workspace", workspaceId.ToString()));

            // Check if requesting user is Owner or Admin (or removing themselves)
            var requestingUserWorkspace = workspace.UserWorkspaces.FirstOrDefault(uw => uw.UserId == userId);
            if (requestingUserWorkspace == null)
                return Result.Failure(Error.Forbidden("You are not a member of this workspace"));

            if (userId != targetUserId && requestingUserWorkspace.Role != "Owner" && requestingUserWorkspace.Role != "Admin")
                return Result.Failure(Error.Forbidden("Only workspace owners and admins can remove other users"));

            var targetUserWorkspace = workspace.UserWorkspaces.FirstOrDefault(uw => uw.UserId == targetUserId);
            if (targetUserWorkspace == null)
                return Result.Failure(Error.NotFound("User membership", targetUserId));

            // Prevent removing the last owner
            if (targetUserWorkspace.Role == "Owner" && workspace.UserWorkspaces.Count(uw => uw.Role == "Owner") <= 1)
                return Result.Failure(Error.Conflict("Cannot remove the last owner of the workspace"));

            _context.UserWorkspaces.Remove(targetUserWorkspace);
            await _context.SaveChangesAsync();

            return Result.Success();
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error removing user from workspace: {WorkspaceId}", workspaceId);
            return Result.Failure(Error.Internal("Failed to remove user from workspace", ex.Message));
        }
    }

    public async Task<Result> UpdateUserRoleAsync(Guid workspaceId, string userId, string targetUserId, string newRole)
    {
        try
        {
            if (string.IsNullOrWhiteSpace(userId))
                return Result.Failure(Error.Required("userId"));

            if (string.IsNullOrWhiteSpace(targetUserId))
                return Result.Failure(Error.Required("targetUserId"));

            if (string.IsNullOrWhiteSpace(newRole))
                return Result.Failure(Error.Required("newRole"));

            var workspace = await _context.Workspaces
                .Include(w => w.UserWorkspaces)
                .FirstOrDefaultAsync(w => w.Id == workspaceId);

            if (workspace == null)
                return Result.Failure(Error.NotFound("Workspace", workspaceId.ToString()));

            // Check if requesting user is Owner
            var requestingUserWorkspace = workspace.UserWorkspaces.FirstOrDefault(uw => uw.UserId == userId);
            if (requestingUserWorkspace == null || requestingUserWorkspace.Role != "Owner")
                return Result.Failure(Error.Forbidden("Only workspace owner can change user roles"));

            var targetUserWorkspace = workspace.UserWorkspaces.FirstOrDefault(uw => uw.UserId == targetUserId);
            if (targetUserWorkspace == null)
                return Result.Failure(Error.NotFound("User membership", targetUserId));

            // Prevent demoting the last owner
            if (targetUserWorkspace.Role == "Owner" && newRole != "Owner" &&
                workspace.UserWorkspaces.Count(uw => uw.Role == "Owner") <= 1)
                return Result.Failure(Error.Conflict("Cannot demote the last owner of the workspace"));

            targetUserWorkspace.Role = newRole;
            await _context.SaveChangesAsync();

            return Result.Success();
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error updating user role in workspace: {WorkspaceId}", workspaceId);
            return Result.Failure(Error.Internal("Failed to update user role", ex.Message));
        }
    }

    public async Task<Result<List<UserWorkspace>>> GetWorkspaceMembersAsync(Guid workspaceId, string userId)
    {
        try
        {
            if (string.IsNullOrWhiteSpace(userId))
                return Result<List<UserWorkspace>>.Failure(Error.Required("userId"));

            var workspace = await _context.Workspaces
                .Include(w => w.UserWorkspaces)
                .FirstOrDefaultAsync(w => w.Id == workspaceId);

            if (workspace == null)
                return Result<List<UserWorkspace>>.Failure(Error.NotFound("Workspace", workspaceId.ToString()));

            // Check if requesting user is a member
            if (!workspace.UserWorkspaces.Any(uw => uw.UserId == userId))
                return Result<List<UserWorkspace>>.Failure(Error.Forbidden("You are not a member of this workspace"));

            return Result<List<UserWorkspace>>.Success(workspace.UserWorkspaces.ToList());
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error getting workspace members: {WorkspaceId}", workspaceId);
            return Result<List<UserWorkspace>>.Failure(Error.Internal("Failed to get workspace members", ex.Message));
        }
    }
}

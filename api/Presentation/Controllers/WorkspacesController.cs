using Microsoft.AspNetCore.Authorization;
using Microsoft.AspNetCore.Mvc;
using ShortasProxyApi.Application.DTOs;
using ShortasProxyApi.Domain.Interfaces;
using ShortasProxyApi.Presentation.Extensions;

namespace ShortasProxyApi.Presentation.Controllers;

[ApiController]
[Route("api/v1/workspaces")]
[Authorize]
public class WorkspacesController : ControllerBase
{
    private readonly IWorkspaceService _workspaceService;
    private readonly ILogger<WorkspacesController> _logger;

    public WorkspacesController(IWorkspaceService workspaceService, ILogger<WorkspacesController> logger)
    {
        _workspaceService = workspaceService;
        _logger = logger;
    }

    /// <summary>
    /// List all workspaces for the current user
    /// </summary>
    /// <returns>List of workspaces</returns>
    [HttpGet]
    public async Task<ActionResult<List<WorkspaceDto>>> ListWorkspaces()
    {
        var userId = this.GetUserId();
        var result = await _workspaceService.ListUserWorkspacesAsync(userId);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        var workspaceDtos = result.Value.Select(w => MapToDto(w, userId)).ToList();
        return Ok(workspaceDtos);
    }

    /// <summary>
    /// Get workspace by ID
    /// </summary>
    /// <param name="id">Workspace ID</param>
    /// <returns>Workspace information</returns>
    [HttpGet("{id}")]
    public async Task<ActionResult<WorkspaceDto>> GetWorkspace(Guid id)
    {
        var userId = this.GetUserId();
        var result = await _workspaceService.GetWorkspaceByIdAsync(id, userId);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        if (result.Value == null)
            return NotFound();

        var workspaceDto = MapToDto(result.Value, userId);
        return Ok(workspaceDto);
    }

    /// <summary>
    /// Create a new workspace
    /// </summary>
    /// <param name="createDto">Workspace data</param>
    /// <returns>Created workspace</returns>
    [HttpPost]
    public async Task<ActionResult<WorkspaceDto>> CreateWorkspace([FromBody] CreateWorkspaceDto createDto)
    {
        var userId = this.GetUserId();
        var result = await _workspaceService.CreateWorkspaceAsync(createDto.Name, createDto.Description, userId);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        var workspaceDto = MapToDto(result.Value, userId);
        return CreatedAtAction(nameof(GetWorkspace), new { id = result.Value.Id }, workspaceDto);
    }

    /// <summary>
    /// Update an existing workspace
    /// </summary>
    /// <param name="id">Workspace ID</param>
    /// <param name="updateDto">Updated workspace data</param>
    /// <returns>Updated workspace</returns>
    [HttpPut("{id}")]
    public async Task<ActionResult<WorkspaceDto>> UpdateWorkspace(Guid id, [FromBody] UpdateWorkspaceDto updateDto)
    {
        var userId = this.GetUserId();
        var result = await _workspaceService.UpdateWorkspaceAsync(id, userId, updateDto.Name, updateDto.Description);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        var workspaceDto = MapToDto(result.Value, userId);
        return Ok(workspaceDto);
    }

    /// <summary>
    /// Delete a workspace
    /// </summary>
    /// <param name="id">Workspace ID</param>
    /// <returns>No content on success</returns>
    [HttpDelete("{id}")]
    public async Task<ActionResult> DeleteWorkspace(Guid id)
    {
        var userId = this.GetUserId();
        var result = await _workspaceService.DeleteWorkspaceAsync(id, userId);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        return NoContent();
    }

    /// <summary>
    /// Get workspace members
    /// </summary>
    /// <param name="id">Workspace ID</param>
    /// <returns>List of workspace members</returns>
    [HttpGet("{id}/members")]
    public async Task<ActionResult<List<UserWorkspaceDto>>> GetWorkspaceMembers(Guid id)
    {
        var userId = this.GetUserId();
        var result = await _workspaceService.GetWorkspaceMembersAsync(id, userId);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        var memberDtos = result.Value.Select(MapMemberToDto).ToList();
        return Ok(memberDtos);
    }

    /// <summary>
    /// Add a user to a workspace
    /// </summary>
    /// <param name="id">Workspace ID</param>
    /// <param name="request">User to add</param>
    /// <returns>No content on success</returns>
    [HttpPost("{id}/members")]
    public async Task<ActionResult> AddUserToWorkspace(Guid id, [FromBody] AddUserToWorkspaceRequest request)
    {
        var userId = this.GetUserId();
        var result = await _workspaceService.AddUserToWorkspaceAsync(id, userId, request.UserId, request.Role);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        return NoContent();
    }

    /// <summary>
    /// Remove a user from a workspace
    /// </summary>
    /// <param name="id">Workspace ID</param>
    /// <param name="memberId">User ID to remove</param>
    /// <returns>No content on success</returns>
    [HttpDelete("{id}/members/{memberId}")]
    public async Task<ActionResult> RemoveUserFromWorkspace(Guid id, string memberId)
    {
        var userId = this.GetUserId();
        var result = await _workspaceService.RemoveUserFromWorkspaceAsync(id, userId, memberId);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        return NoContent();
    }

    /// <summary>
    /// Update a user's role in a workspace
    /// </summary>
    /// <param name="id">Workspace ID</param>
    /// <param name="memberId">User ID</param>
    /// <param name="request">New role</param>
    /// <returns>No content on success</returns>
    [HttpPut("{id}/members/{memberId}")]
    public async Task<ActionResult> UpdateUserRole(Guid id, string memberId, [FromBody] UpdateUserRoleRequest request)
    {
        var userId = this.GetUserId();
        var result = await _workspaceService.UpdateUserRoleAsync(id, userId, memberId, request.Role);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        return NoContent();
    }

    private ActionResult HandleError(string errorCode, string errorMessage)
    {
        return errorCode switch
        {
            "REQUIRED_FIELD" => BadRequest(new { error = errorCode, message = errorMessage }),
            "VALIDATION_ERROR" => BadRequest(new { error = errorCode, message = errorMessage }),
            "UNAUTHORIZED" => Unauthorized(new { error = errorCode, message = errorMessage }),
            "FORBIDDEN" => Forbid(),
            "NOT_FOUND" => NotFound(new { error = errorCode, message = errorMessage }),
            "CONFLICT" => Conflict(new { error = errorCode, message = errorMessage }),
            "BUSINESS_RULE_VIOLATION" => UnprocessableEntity(new { error = errorCode, message = errorMessage }),
            "INTERNAL_ERROR" => StatusCode(500, new { error = errorCode, message = errorMessage }),
            _ => StatusCode(500, new { error = "UNKNOWN_ERROR", message = "An unknown error occurred" })
        };
    }

    private static WorkspaceDto MapToDto(Domain.Entities.Workspace workspace, string userId)
    {
        var userWorkspace = workspace.UserWorkspaces.FirstOrDefault(uw => uw.UserId == userId);

        return new WorkspaceDto
        {
            Id = workspace.Id,
            Name = workspace.Name,
            Description = workspace.Description,
            Type = workspace.Type,
            CreatedAt = workspace.CreatedAt,
            UpdatedAt = workspace.UpdatedAt,
            UserRole = userWorkspace?.Role
        };
    }

    private static UserWorkspaceDto MapMemberToDto(Domain.Entities.UserWorkspace userWorkspace)
    {
        return new UserWorkspaceDto
        {
            Id = userWorkspace.Id,
            UserId = userWorkspace.UserId,
            WorkspaceId = userWorkspace.WorkspaceId,
            Role = userWorkspace.Role,
            JoinedAt = userWorkspace.JoinedAt
        };
    }
}

public class AddUserToWorkspaceRequest
{
    public string UserId { get; set; } = string.Empty;
    public string Role { get; set; } = "Member";
}

public class UpdateUserRoleRequest
{
    public string Role { get; set; } = "Member";
}

using Microsoft.AspNetCore.Authorization;
using Microsoft.AspNetCore.Mvc;
using ShortasProxyApi.Application.DTOs;
using ShortasProxyApi.Domain.Interfaces;
using ShortasProxyApi.Domain.Entities;
using ShortasProxyApi.Presentation.Extensions;

namespace ShortasProxyApi.Presentation.Controllers;

[ApiController]
[Route("api/v1/user")]
[Authorize]
public class UserController : ControllerBase
{
    private readonly IWorkspaceService _workspaceService;
    private readonly IUserSettingsService _userSettingsService;
    private readonly ILogger<UserController> _logger;

    public UserController(
        IWorkspaceService workspaceService, 
        IUserSettingsService userSettingsService,
        ILogger<UserController> logger)
    {
        _workspaceService = workspaceService;
        _userSettingsService = userSettingsService;
        _logger = logger;
    }

    /// <summary>
    /// Initialize default workspace and user settings for a new user
    /// </summary>
    /// <returns>Initialization result with workspace and user settings</returns>
    [HttpPost("initialize")]
    public async Task<ActionResult<InitializationResponse>> Initialize()
    {
        var userId = this.GetUserId();
        var email = this.GetUserEmail();
        
        _logger.LogInformation("Initializing user {UserId}", userId);

        WorkspaceDto? workspaceDto = null;
        UserSettingsDto? userSettingsDto = null;

        // Check if user already has workspaces
        var existingWorkspaces = await _workspaceService.ListUserWorkspacesAsync(userId);
        if (existingWorkspaces.IsSuccess && existingWorkspaces.Value.Any())
        {
            // User already has a workspace
            var firstWorkspace = existingWorkspaces.Value.First();
            workspaceDto = MapWorkspaceToDto(firstWorkspace, userId);
            _logger.LogInformation("User {UserId} already has workspace {WorkspaceId}", userId, firstWorkspace.Id);
        }
        else
        {
            // Create default workspace
            var workspaceResult = await _workspaceService.CreateWorkspaceAsync(
                "My Workspace",
                "Default workspace for organizing your routes",
                userId
            );

            if (workspaceResult.IsFailure)
            {
                _logger.LogError("Failed to create workspace for user {UserId}: {Error}", userId, workspaceResult.Error);
                return HandleError(workspaceResult.ErrorCode ?? "UNKNOWN_ERROR", workspaceResult.Error);
            }

            workspaceDto = MapWorkspaceToDto(workspaceResult.Value, userId);
            _logger.LogInformation("Created default workspace {WorkspaceId} for user {UserId}", workspaceResult.Value.Id, userId);
        }

        // Check if user already has settings
        var existingSettings = await _userSettingsService.GetUserSettingsAsync(userId);
        if (existingSettings.IsSuccess && existingSettings.Value != null)
        {
            // User already has settings
            userSettingsDto = MapSettingsToDto(existingSettings.Value);
            _logger.LogInformation("User {UserId} already has settings", userId);
        }
        else
        {
            // Create default user settings
            var defaultSettings = new UserSettings
            {
                Email = email,
                Status = "Active",
                Debug = false,
                Overflow = false,
                SkipTracking = new List<string>(),
                AllowedRequestParams = new List<string>(),
                AllowedDestinationParams = new List<string>()
            };

            var settingsResult = await _userSettingsService.CreateUserSettingsAsync(userId, defaultSettings);

            if (settingsResult.IsFailure)
            {
                // Log error but don't fail the entire initialization if workspace was created
                _logger.LogError("Failed to create user settings for user {UserId}: {Error}", userId, settingsResult.Error);
                
                // If workspace was also not created, return error
                if (workspaceDto == null || existingWorkspaces.IsSuccess && !existingWorkspaces.Value.Any())
                {
                    return HandleError(settingsResult.ErrorCode ?? "UNKNOWN_ERROR", settingsResult.Error);
                }
            }
            else
            {
                userSettingsDto = MapSettingsToDto(settingsResult.Value);
                _logger.LogInformation("Created default settings for user {UserId}", userId);
            }
        }

        var response = new InitializationResponse
        {
            Workspace = workspaceDto,
            UserSettings = userSettingsDto,
            Message = "User initialization completed successfully"
        };

        return Ok(response);
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

    private static WorkspaceDto MapWorkspaceToDto(Workspace workspace, string userId)
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

    private static UserSettingsDto MapSettingsToDto(UserSettings settings)
    {
        return new UserSettingsDto
        {
            Email = settings.Email,
            Status = settings.Status,
            Debug = settings.Debug,
            Overflow = settings.Overflow,
            SkipTracking = settings.SkipTracking,
            AllowedRequestParams = settings.AllowedRequestParams,
            AllowedDestinationParams = settings.AllowedDestinationParams
        };
    }
}

public class InitializationResponse
{
    public WorkspaceDto? Workspace { get; set; }
    public UserSettingsDto? UserSettings { get; set; }
    public string Message { get; set; } = string.Empty;
}


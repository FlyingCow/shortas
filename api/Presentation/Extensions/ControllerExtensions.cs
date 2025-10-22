using Microsoft.AspNetCore.Mvc;
using System.Security.Claims;

namespace ShortasProxyApi.Presentation.Extensions;

public static class ControllerExtensions
{
    /// <summary>
    /// Gets the current user's ID from JWT claims
    /// </summary>
    /// <param name="controller">The controller instance</param>
    /// <returns>User ID from JWT token claims</returns>
    public static string GetUserId(this ControllerBase controller)
    {
        // Try standard JWT claim types first
        var userId = controller.User.FindFirst(ClaimTypes.NameIdentifier)?.Value
                  ?? controller.User.FindFirst("sub")?.Value  // Standard OIDC subject claim
                  ?? controller.User.FindFirst("preferred_username")?.Value  // Keycloak preferred username
                  ?? controller.User.FindFirst("email")?.Value;  // Fallback to email

        if (string.IsNullOrWhiteSpace(userId))
        {
            throw new UnauthorizedAccessException("User ID not found in token claims");
        }

        return userId;
    }
    
    /// <summary>
    /// Gets the current user's ID from JWT claims
    /// </summary>
    /// <param name="controller">The controller instance</param>
    /// <returns>User ID from JWT token claims</returns>
    public static string GetUserEmail(this ControllerBase controller)
    {
        // Try standard JWT claim types first
        var userEmail = controller.User.FindFirst(ClaimTypes.Email)?.Value
                     ?? controller.User.FindFirst("email")?.Value;  // Fallback to email

        if (string.IsNullOrWhiteSpace(userEmail))
        {
            throw new UnauthorizedAccessException("User Email not found in token claims");
        }

        return userEmail;
    }

    /// <summary>
    /// Tries to get the current user's ID from JWT claims
    /// </summary>
    /// <param name="controller">The controller instance</param>
    /// <param name="userId">Output user ID if found</param>
    /// <returns>True if user ID was found, false otherwise</returns>
    public static bool TryGetUserId(this ControllerBase controller, out string userId)
    {
        userId = controller.User.FindFirst(ClaimTypes.NameIdentifier)?.Value
              ?? controller.User.FindFirst("sub")?.Value
              ?? controller.User.FindFirst("preferred_username")?.Value
              ?? controller.User.FindFirst("email")?.Value
              ?? string.Empty;

        return !string.IsNullOrWhiteSpace(userId);
    }
}

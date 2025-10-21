namespace ShortasProxyApi.Application.DTOs;

public class WorkspaceDto
{
    public Guid Id { get; set; }
    public string Name { get; set; } = string.Empty;
    public string Description { get; set; } = string.Empty;
    public string Type { get; set; } = "User";  // "System" or "User"
    public DateTime CreatedAt { get; set; }
    public DateTime UpdatedAt { get; set; }
    public string? UserRole { get; set; }  // Role of the current user in this workspace
    public List<UserWorkspaceDto>? Members { get; set; }  // Optional, for detailed view
    public bool IsSystem => Type == "System";
}

public class UserWorkspaceDto
{
    public Guid Id { get; set; }
    public string UserId { get; set; } = string.Empty;
    public Guid WorkspaceId { get; set; }
    public string Role { get; set; } = "Member";
    public DateTime JoinedAt { get; set; }
}

public class CreateWorkspaceDto
{
    public string Name { get; set; } = string.Empty;
    public string Description { get; set; } = string.Empty;
}

public class UpdateWorkspaceDto
{
    public string? Name { get; set; }
    public string? Description { get; set; }
}

namespace ShortasProxyApi.Domain.Entities;

public class Workspace
{
    public Guid Id { get; set; } = Guid.NewGuid();
    public string Name { get; set; } = string.Empty;
    public string Description { get; set; } = string.Empty;
    public string Type { get; set; } = "User";  // "System" or "User"
    public DateTime CreatedAt { get; set; } = DateTime.UtcNow;
    public DateTime UpdatedAt { get; set; } = DateTime.UtcNow;

    // Navigation property for user associations
    public ICollection<UserWorkspace> UserWorkspaces { get; set; } = new List<UserWorkspace>();

    public bool IsSystem => Type == "System";
}

public class UserWorkspace
{
    public Guid Id { get; set; } = Guid.NewGuid();
    public string UserId { get; set; } = string.Empty;  // Keycloak user ID from JWT
    public Guid WorkspaceId { get; set; }
    public string Role { get; set; } = "Member";  // Owner, Admin, Member
    public DateTime JoinedAt { get; set; } = DateTime.UtcNow;

    // Navigation property
    public Workspace Workspace { get; set; } = null!;
}

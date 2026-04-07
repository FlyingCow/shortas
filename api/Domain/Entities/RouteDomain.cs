namespace ShortasProxyApi.Domain.Entities;

public class RouteDomain
{
    public Guid Id { get; set; } = Guid.NewGuid();

    private string _name = string.Empty;
    public string Name
    {
        get => _name;
        set => _name = value?.ToLowerInvariant() ?? string.Empty;
    }

    public string OwnerId { get; set; } = string.Empty;

    // Shared domain flag - shared domains can be used by any user
    public bool IsShared { get; set; } = false;

    // Verification status
    public DomainVerificationStatus VerificationStatus { get; set; } = DomainVerificationStatus.Pending;
    public string VerificationReason { get; set; } = "not_checked";
    public DateTime? LastVerificationCheck { get; set; }
    public DateTime? NextVerificationCheck { get; set; }

    // Custom pages
    public string? CustomIndexUrl { get; set; }
    public string? CustomNotFoundUrl { get; set; }

    // Navigation properties
    public ICollection<Route> Routes { get; set; } = new List<Route>();
    public ICollection<Certificate> Certificates { get; set; } = new List<Certificate>();
}

public enum DomainVerificationStatus
{
    Pending,
    Verified,
    Failed
}

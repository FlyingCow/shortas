namespace ShortasProxyApi.Domain.Entities;

public class Certificate
{
    public Guid Id { get; set; } = Guid.NewGuid();
    public string Key { get; set; } = string.Empty;
    public string Cert { get; set; } = string.Empty;
    public string? OcspResp { get; set; }

    public string OwnerId { get; set; } = string.Empty;

    // Domain relationship
    public Guid DomainId { get; set; }  // Foreign key (required)
    public RouteDomain Domain { get; set; } = null!;  // Navigation property (required)
}

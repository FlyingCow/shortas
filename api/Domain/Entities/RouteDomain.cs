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

    // Navigation properties
    public ICollection<Route> Routes { get; set; } = new List<Route>();
    public ICollection<Certificate> Certificates { get; set; } = new List<Certificate>();
}

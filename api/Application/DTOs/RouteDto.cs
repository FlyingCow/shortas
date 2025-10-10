namespace ShortasProxyApi.Application.DTOs;

public class RouteDto
{
    public string Switch { get; set; } = string.Empty;
    public string Link { get; set; } = string.Empty;
    public string Dest { get; set; } = string.Empty;
    public string DestFormat { get; set; } = string.Empty;
    public int Code { get; set; }
    public int Ttl { get; set; }
    public string Status { get; set; } = string.Empty;
    public string Terminal { get; set; } = string.Empty;
    public RoutePropertiesDto? Properties { get; set; }
}

public class RoutePropertiesDto
{
    public string RouteId { get; set; } = string.Empty;
    public string DomainId { get; set; } = string.Empty;
    public string OwnerId { get; set; } = string.Empty;
    public List<string> Scripts { get; set; } = new();
    public List<string> Tags { get; set; } = new();
    public Dictionary<string, object> Custom { get; set; } = new();
    public bool Opengraph { get; set; }
    public bool AllowDebug { get; set; }
}


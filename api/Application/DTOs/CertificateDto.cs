namespace ShortasProxyApi.Application.DTOs;

public class CertificateDto
{
    public Guid Id { get; set; }
    public string Key { get; set; } = string.Empty;
    public string Cert { get; set; } = string.Empty;
    public string? OcspResp { get; set; }
    public string OwnerId { get; set; } = string.Empty;
    public Guid DomainId { get; set; }
    public DomainDto? Domain { get; set; }
}


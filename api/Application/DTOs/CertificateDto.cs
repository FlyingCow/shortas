namespace ShortasProxyApi.Application.DTOs;

public class CertificateDto
{
    public string Key { get; set; } = string.Empty;
    public string Cert { get; set; } = string.Empty;
    public string? OcspResp { get; set; }
}


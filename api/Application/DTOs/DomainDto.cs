using ShortasProxyApi.Domain.Entities;

namespace ShortasProxyApi.Application.DTOs;

public class DomainDto
{
    public Guid Id { get; set; }
    public string Name { get; set; } = string.Empty;
    public string OwnerId { get; set; } = string.Empty;
    public string VerificationStatus { get; set; } = "Pending";
    public string VerificationReason { get; set; } = "not_checked";
    public DateTime? LastVerificationCheck { get; set; }
    public DateTime? NextVerificationCheck { get; set; }
}

public class CreateDomainDto
{
    public string Name { get; set; } = string.Empty;
}

public class UpdateDomainDto
{
    public string Name { get; set; } = string.Empty;
}

public class DnsConfigDto
{
    public string TxtRecordName { get; set; } = string.Empty;
    public List<string> AllowedIpv4 { get; set; } = new();
    public List<string> AllowedIpv6 { get; set; } = new();
}

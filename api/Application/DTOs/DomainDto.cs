namespace ShortasProxyApi.Application.DTOs;

public class DomainDto
{
    public Guid Id { get; set; }
    public string Name { get; set; } = string.Empty;
    public string OwnerId { get; set; } = string.Empty;
}

public class CreateDomainDto
{
    public string Name { get; set; } = string.Empty;
}

public class UpdateDomainDto
{
    public string Name { get; set; } = string.Empty;
}

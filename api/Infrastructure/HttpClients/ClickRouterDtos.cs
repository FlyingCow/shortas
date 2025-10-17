using ShortasProxyApi.Application.DTOs;

namespace ShortasProxyApi.Infrastructure.HttpClients;

/// <summary>
/// DTOs for communicating with the Click Router API.
/// These DTOs exclude navigation properties and internal database fields (like Guid Ids).
/// </summary>

#region Route DTOs

/// <summary>
/// DTO for sending/receiving Route data to/from Click Router API
/// </summary>
public class ClickRouterRouteDto
{
    public string Switch { get; set; } = string.Empty;
    public string Link { get; set; } = string.Empty;
    public string? Dest { get; set; }
    public string DestFormat { get; set; } = "Http";
    public int? Code { get; set; }
    public long? Ttl { get; set; }
    public string Status { get; set; } = "Active";
    public string Terminal { get; set; } = "External";
    public Domain.Entities.RoutingPolicy? Policy { get; set; }
    public ClickRouterRoutePropertiesDto? Properties { get; set; }

    /// <summary>
    /// Maps Application DTO to ClickRouter API DTO
    /// </summary>
    public static ClickRouterRouteDto FromDto(RouteDto dto)
    {
        return new ClickRouterRouteDto
        {
            Switch = dto.Switch,
            Link = dto.Link,
            Dest = dto.Dest,
            DestFormat = dto.DestFormat,
            Code = dto.Code,
            Ttl = dto.Ttl,
            Status = dto.Status,
            Terminal = dto.Terminal,
            Policy = dto.Policy,
            Properties = dto.Properties != null
                ? ClickRouterRoutePropertiesDto.FromDto(dto.Properties)
                : null
        };
    }

    /// <summary>
    /// Maps ClickRouter API DTO to Application DTO
    /// </summary>
    public RouteDto ToDto()
    {
        return new RouteDto
        {
            Switch = Switch,
            Link = Link,
            Dest = Dest,
            DestFormat = DestFormat,
            Code = Code,
            Ttl = Ttl,
            Status = Status,
            Terminal = Terminal,
            Policy = Policy,
            Properties = Properties?.ToDto()
        };
    }
}

/// <summary>
/// DTO for Route Properties to send/receive from Click Router API
/// </summary>
public class ClickRouterRoutePropertiesDto
{
    public string? RouteId { get; set; }
    public string? DomainId { get; set; }
    public string? OwnerId { get; set; }
    public string? CreatorId { get; set; }
    public string? WorkspaceId { get; set; }
    public List<string>? Scripts { get; set; }
    public List<string>? Tags { get; set; }
    public Dictionary<string, object>? Custom { get; set; }
    public Dictionary<string, object>? Native { get; set; }
    public Dictionary<string, object>? Bundling { get; set; }
    public bool Opengraph { get; set; }
    public bool AllowDebug { get; set; }

    /// <summary>
    /// Maps Application DTO to ClickRouter API DTO
    /// </summary>
    public static ClickRouterRoutePropertiesDto FromDto(RoutePropertiesDto dto)
    {
        return new ClickRouterRoutePropertiesDto
        {
            RouteId = dto.RouteId,
            DomainId = dto.DomainId,
            OwnerId = dto.OwnerId,
            CreatorId = dto.CreatorId,
            WorkspaceId = dto.WorkspaceId,
            Scripts = dto.Scripts,
            Tags = dto.Tags,
            Custom = dto.Custom,
            Native = dto.Native,
            Bundling = dto.Bundling,
            Opengraph = dto.Opengraph,
            AllowDebug = dto.AllowDebug
        };
    }

    /// <summary>
    /// Maps ClickRouter API DTO to Application DTO
    /// </summary>
    public RoutePropertiesDto ToDto()
    {
        return new RoutePropertiesDto
        {
            RouteId = RouteId,
            DomainId = DomainId,
            OwnerId = OwnerId,
            CreatorId = CreatorId,
            WorkspaceId = WorkspaceId,
            Scripts = Scripts,
            Tags = Tags,
            Custom = Custom,
            Native = Native,
            Bundling = Bundling,
            Opengraph = Opengraph,
            AllowDebug = AllowDebug
        };
    }
}

#endregion

#region Certificate DTOs

/// <summary>
/// DTO for sending/receiving Certificate data to/from Click Router API
/// Excludes database-specific fields like Guid Id
/// </summary>
public class ClickRouterCertificateDto
{
    public string Key { get; set; } = string.Empty;
    public string Cert { get; set; } = string.Empty;
    public string? OcspResp { get; set; }
    public string OwnerId { get; set; } = string.Empty;

    /// <summary>
    /// Maps Application DTO to ClickRouter API DTO
    /// </summary>
    public static ClickRouterCertificateDto FromDto(CertificateDto dto)
    {
        return new ClickRouterCertificateDto
        {
            Key = dto.Key,
            Cert = dto.Cert,
            OcspResp = dto.OcspResp,
            OwnerId = dto.OwnerId
        };
    }

    /// <summary>
    /// Maps ClickRouter API DTO to Application DTO
    /// </summary>
    public CertificateDto ToDto()
    {
        return new CertificateDto
        {
            Key = Key,
            Cert = Cert,
            OcspResp = OcspResp,
            OwnerId = OwnerId
        };
    }
}

#endregion

#region UserSettings DTOs

/// <summary>
/// DTO for sending/receiving UserSettings data to/from Click Router API
/// Excludes database-specific fields like Guid Id
/// </summary>
public class ClickRouterUserSettingsDto
{
    public string Email { get; set; } = string.Empty;
    public string Status { get; set; } = string.Empty;
    public bool Debug { get; set; }
    public bool Overflow { get; set; }
    public List<string> SkipTracking { get; set; } = new();
    public List<string> AllowedRequestParams { get; set; } = new();
    public List<string> AllowedDestinationParams { get; set; } = new();

    /// <summary>
    /// Maps Application DTO to ClickRouter API DTO
    /// </summary>
    public static ClickRouterUserSettingsDto FromDto(UserSettingsDto dto)
    {
        return new ClickRouterUserSettingsDto
        {
            Email = dto.Email,
            Status = dto.Status,
            Debug = dto.Debug,
            Overflow = dto.Overflow,
            SkipTracking = dto.SkipTracking,
            AllowedRequestParams = dto.AllowedRequestParams,
            AllowedDestinationParams = dto.AllowedDestinationParams
        };
    }

    /// <summary>
    /// Maps ClickRouter API DTO to Application DTO
    /// </summary>
    public UserSettingsDto ToDto()
    {
        return new UserSettingsDto
        {
            Email = Email,
            Status = Status,
            Debug = Debug,
            Overflow = Overflow,
            SkipTracking = SkipTracking,
            AllowedRequestParams = AllowedRequestParams,
            AllowedDestinationParams = AllowedDestinationParams
        };
    }
}

#endregion

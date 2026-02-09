using ShortasProxyApi.Application.DTOs;
using ShortasProxyApi.Domain.Entities;
using RouteEntity = ShortasProxyApi.Domain.Entities.Route;

namespace ShortasProxyApi.Application.Extensions;

/// <summary>
/// Extension methods for mapping between entities and DTOs
/// </summary>
public static class MappingExtensions
{
    #region Route Mappings

    public static RouteDto ToDto(this RouteEntity entity)
    {
        return new RouteDto
        {
            Id = entity.Id.ToString(),
            Switch = entity.Switch,
            Link = entity.Link,
            Dest = entity.Dest,
            DestFormat = entity.DestFormat,
            Code = entity.Code,
            Ttl = entity.Ttl,
            Status = entity.Status,
            Terminal = entity.Terminal,
            Policy = entity.Policy,
            Properties = entity.Properties?.ToDto(),
            DomainId = entity.DomainId,
            Domain = entity.Domain?.ToDto()
        };
    }

    public static RouteEntity ToEntity(this RouteDto dto)
    {
        return new RouteEntity
        {
            Id = string.IsNullOrEmpty(dto.Id) ? Guid.NewGuid() : Guid.Parse(dto.Id),
            Switch = dto.Switch,
            Link = dto.Link,
            Dest = dto.Dest,
            DestFormat = dto.DestFormat,
            Code = dto.Code,
            Ttl = dto.Ttl,
            Status = dto.Status,
            Terminal = dto.Terminal,
            Policy = dto.Policy ?? new BasicPolicy(),
            Properties = dto.Properties?.ToEntity() ?? new RouteProperties(),
            DomainId = dto.DomainId
        };
    }

    public static RoutePropertiesDto ToDto(this RouteProperties entity )
    {
        return new RoutePropertiesDto
        {
            RouteId = entity.RouteId,
            DomainId = entity.DomainId,
            OwnerId = entity.OwnerId,
            CreatorId = entity.CreatorId,
            WorkspaceId = entity.WorkspaceId,
            Scripts = entity.Scripts,
            Tags = entity.Tags,
            Custom = entity.Custom,
            Native = entity.Native,
            Bundling = entity.Bundling,
            Opengraph = entity.Opengraph,
            AllowDebug = entity.AllowDebug
        };
    }

    public static RouteProperties ToEntity(this RoutePropertiesDto dto)
    {
        return new RouteProperties
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

    #endregion

    #region Certificate Mappings

    public static CertificateDto ToDto(this Certificate entity)
    {
        return new CertificateDto
        {
            Id = entity.Id,
            Key = entity.Key,
            Cert = entity.Cert,
            OcspResp = entity.OcspResp,
            OwnerId = entity.OwnerId,
            DomainId = entity.DomainId,
            Domain = entity.Domain?.ToDto()
        };
    }

    public static Certificate ToEntity(this CertificateDto dto)
    {
        return new Certificate
        {
            Id = dto.Id,
            Key = dto.Key,
            Cert = dto.Cert,
            OcspResp = dto.OcspResp,
            OwnerId = dto.OwnerId,
            DomainId = dto.DomainId
        };
    }

    #endregion

    #region UserSettings Mappings

    public static UserSettingsDto ToDto(this UserSettings entity)
    {
        return new UserSettingsDto
        {
            Email = entity.Email,
            Status = entity.Status,
            Debug = entity.Debug,
            Overflow = entity.Overflow,
            SkipTracking = entity.SkipTracking,
            AllowedRequestParams = entity.AllowedRequestParams,
            AllowedDestinationParams = entity.AllowedDestinationParams
        };
    }

    public static UserSettings ToEntity(this UserSettingsDto dto)
    {
        return new UserSettings
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

    #endregion

    #region Domain Mappings

    public static DomainDto? ToDto(this RouteDomain? entity)
    {
        if (entity == null) return null;

        return new DomainDto
        {
            Id = entity.Id,
            Name = entity.Name,
            OwnerId = entity.OwnerId,
            VerificationStatus = entity.VerificationStatus.ToString(),
            VerificationReason = entity.VerificationReason,
            LastVerificationCheck = entity.LastVerificationCheck,
            NextVerificationCheck = entity.NextVerificationCheck
        };
    }

    #endregion
}

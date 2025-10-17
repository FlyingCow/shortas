using Microsoft.AspNetCore.Authorization;
using Microsoft.AspNetCore.Mvc;
using ShortasProxyApi.Application.DTOs;
using ShortasProxyApi.Domain.Entities;
using ShortasProxyApi.Domain.Interfaces;
using ShortasProxyApi.Domain.Common;
using ShortasProxyApi.Presentation.Extensions;

namespace ShortasProxyApi.Presentation.Controllers;

[ApiController]
[Route("api/v1/certificates")]
[Authorize]
public class CertificatesController : ControllerBase
{
    private readonly ICertificateService _certificateService;
    private readonly ILogger<CertificatesController> _logger;

    public CertificatesController(ICertificateService certificateService, ILogger<CertificatesController> logger)
    {
        _certificateService = certificateService;
        _logger = logger;
    }

    /// <summary>
    /// List all certificates with pagination and filtering
    /// </summary>
    /// <param name="page">Page number (default: 1)</param>
    /// <param name="pageSize">Page size (default: 20)</param>
    /// <param name="search">Search term for domain/key</param>
    /// <param name="domainId">Optional domain ID filter</param>
    /// <returns>Paginated list of certificates</returns>
    [HttpGet]
    public async Task<ActionResult<object>> ListCertificates(
        [FromQuery] int page = 1,
        [FromQuery] int pageSize = 20,
        [FromQuery] string? search = null,
        [FromQuery] Guid? domainId = null)
    {
        var userId = this.GetUserId();
        var result = await _certificateService.ListCertificatesAsync(userId, page, pageSize, search, domainId);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        var (certificates, totalCount) = result.Value;
        var certificateDtos = certificates.Select(MapToDto).ToList();

        return Ok(new
        {
            data = certificateDtos,
            pagination = new
            {
                page,
                pageSize,
                totalCount,
                totalPages = (int)Math.Ceiling(totalCount / (double)pageSize)
            }
        });
    }

    /// <summary>
    /// Get certificate information by domain ID
    /// </summary>
    /// <param name="domainId">Domain ID</param>
    /// <returns>Certificate information</returns>
    [HttpGet("by-domain/{domainId}")]
    public async Task<ActionResult<CertificateDto>> GetCertificate(Guid domainId)
    {
        var userId = this.GetUserId();
        var result = await _certificateService.GetCertificateAsync(domainId, userId);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        if (result.Value == null)
            return NotFound();

        var certificateDto = MapToDto(result.Value);
        return Ok(certificateDto);
    }

    /// <summary>
    /// Create a new certificate
    /// </summary>
    /// <param name="certificateDto">Certificate data</param>
    /// <returns>Created certificate</returns>
    [HttpPost]
    public async Task<ActionResult<CertificateDto>> CreateCertificate([FromBody] CertificateDto certificateDto)
    {
        var userId = this.GetUserId();
        var certificate = MapFromDto(certificateDto);
        var result = await _certificateService.CreateCertificateAsync(certificate, userId);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        var createdCertificateDto = MapToDto(result.Value);
        return CreatedAtAction(nameof(GetCertificate), new { domainId = result.Value.DomainId }, createdCertificateDto);
    }

    /// <summary>
    /// Update an existing certificate
    /// </summary>
    /// <param name="id">Certificate ID</param>
    /// <param name="certificateDto">Updated certificate data</param>
    /// <returns>Updated certificate</returns>
    [HttpPut("{id}")]
    public async Task<ActionResult<CertificateDto>> UpdateCertificate(Guid id, [FromBody] CertificateDto certificateDto)
    {
        var userId = this.GetUserId();
        var certificate = MapFromDto(certificateDto);
        var result = await _certificateService.UpdateCertificateAsync(id, certificate, userId);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        var updatedCertificateDto = MapToDto(result.Value);
        return Ok(updatedCertificateDto);
    }

    /// <summary>
    /// Delete a certificate
    /// </summary>
    /// <param name="id">Certificate ID</param>
    /// <returns>No content</returns>
    [HttpDelete("{id}")]
    public async Task<IActionResult> DeleteCertificate(Guid id)
    {
        var userId = this.GetUserId();
        var result = await _certificateService.DeleteCertificateAsync(id, userId);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        return NoContent();
    }

    private ActionResult HandleError(string errorCode, string errorMessage)
    {
        return errorCode switch
        {
            "REQUIRED_FIELD" => BadRequest(new { error = errorCode, message = errorMessage }),
            "VALIDATION_ERROR" => BadRequest(new { error = errorCode, message = errorMessage }),
            "UNAUTHORIZED" => Unauthorized(new { error = errorCode, message = errorMessage }),
            "FORBIDDEN" => Forbid(),
            "NOT_FOUND" => NotFound(new { error = errorCode, message = errorMessage }),
            "CONFLICT" => Conflict(new { error = errorCode, message = errorMessage }),
            "BUSINESS_RULE_VIOLATION" => UnprocessableEntity(new { error = errorCode, message = errorMessage }),
            "RATE_LIMIT_EXCEEDED" => StatusCode(429, new { error = errorCode, message = errorMessage }),
            "BURST_LIMIT_EXCEEDED" => StatusCode(429, new { error = errorCode, message = errorMessage }),
            "TIMEOUT" => StatusCode(408, new { error = errorCode, message = errorMessage }),
            "CIRCUIT_BREAKER_OPEN" => StatusCode(503, new { error = errorCode, message = errorMessage }),
            "EXTERNAL_SERVICE_ERROR" => StatusCode(502, new { error = errorCode, message = errorMessage }),
            "NETWORK_ERROR" => StatusCode(502, new { error = errorCode, message = errorMessage }),
            "INTERNAL_ERROR" => StatusCode(500, new { error = errorCode, message = errorMessage }),
            _ => StatusCode(500, new { error = "UNKNOWN_ERROR", message = "An unknown error occurred" })
        };
    }

    private static CertificateDto MapToDto(Certificate certificate)
    {
        return new CertificateDto
        {
            Id = certificate.Id,
            Key = certificate.Key,
            Cert = certificate.Cert,
            OcspResp = certificate.OcspResp,
            OwnerId = certificate.OwnerId,
            DomainId = certificate.DomainId,
            Domain = certificate.Domain != null ? new DomainDto
            {
                Id = certificate.Domain.Id,
                Name = certificate.Domain.Name,
                OwnerId = certificate.Domain.OwnerId
            } : null
        };
    }

    private static Certificate MapFromDto(CertificateDto certificateDto)
    {
        return new Certificate
        {
            Id = certificateDto.Id,
            Key = certificateDto.Key,
            Cert = certificateDto.Cert,
            OcspResp = certificateDto.OcspResp,
            DomainId = certificateDto.DomainId
            // OwnerId is set by the service layer from userId
        };
    }
}
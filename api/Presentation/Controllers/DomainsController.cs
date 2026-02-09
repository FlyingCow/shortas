using Microsoft.AspNetCore.Authorization;
using Microsoft.AspNetCore.Mvc;
using ShortasProxyApi.Application.DTOs;
using ShortasProxyApi.Domain.Entities;
using ShortasProxyApi.Domain.Interfaces;
using ShortasProxyApi.Presentation.Extensions;

namespace ShortasProxyApi.Presentation.Controllers;

[ApiController]
[Route("api/v1/domains")]
[Authorize]
public class DomainsController : ControllerBase
{
    private readonly IDomainService _domainService;
    private readonly ILogger<DomainsController> _logger;
    private readonly IConfiguration _configuration;

    public DomainsController(IDomainService domainService, ILogger<DomainsController> logger, IConfiguration configuration)
    {
        _domainService = domainService;
        _logger = logger;
        _configuration = configuration;
    }

    /// <summary>
    /// List all domains with pagination and filtering
    /// </summary>
    /// <param name="page">Page number (default: 1)</param>
    /// <param name="pageSize">Page size (default: 20)</param>
    /// <param name="search">Search term for domain name</param>
    /// <returns>Paginated list of domains</returns>
    [HttpGet]
    public async Task<ActionResult<object>> ListDomains(
        [FromQuery] int page = 1,
        [FromQuery] int pageSize = 20,
        [FromQuery] string? search = null)
    {
        var userId = this.GetUserId();
        var result = await _domainService.ListDomainsAsync(userId, page, pageSize, search);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        var (domains, totalCount) = result.Value;
        var domainDtos = domains.Select(MapToDto).ToList();

        return Ok(new
        {
            data = domainDtos,
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
    /// Get DNS configuration for domain verification
    /// </summary>
    /// <returns>DNS configuration including TXT record name and allowed IPs</returns>
    [HttpGet("dns-config")]
    public ActionResult<DnsConfigDto> GetDnsConfig()
    {
        var config = new DnsConfigDto
        {
            TxtRecordName = _configuration["DomainVerification:TxtRecordName"] ?? "_shortas-domain-challenge",
            AllowedIpv4 = _configuration.GetSection("DomainVerification:AllowedIpv4").Get<List<string>>() ?? new List<string> { "203.0.113.10" },
            AllowedIpv6 = _configuration.GetSection("DomainVerification:AllowedIpv6").Get<List<string>>() ?? new List<string>()
        };
        return Ok(config);
    }

    /// <summary>
    /// Get domain information by ID
    /// </summary>
    /// <param name="id">Domain ID</param>
    /// <returns>Domain information</returns>
    [HttpGet("{id}")]
    public async Task<ActionResult<DomainDto>> GetDomain(string id)
    {
        if (!Guid.TryParse(id, out var domainId))
        {
            return BadRequest(new { error = "VALIDATION_ERROR", message = "Invalid domain ID format" });
        }

        var userId = this.GetUserId();
        var result = await _domainService.GetDomainByIdAsync(domainId, userId);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        if (result.Value == null)
            return NotFound();

        var domainDto = MapToDto(result.Value);
        return Ok(domainDto);
    }

    /// <summary>
    /// Get domain information by name
    /// </summary>
    /// <param name="name">Domain name</param>
    /// <returns>Domain information</returns>
    [HttpGet("by-name/{name}")]
    public async Task<ActionResult<DomainDto>> GetDomainByName(string name)
    {
        var userId = this.GetUserId();
        var result = await _domainService.GetDomainByNameAsync(name, userId);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        if (result.Value == null)
            return NotFound();

        var domainDto = MapToDto(result.Value);
        return Ok(domainDto);
    }

    /// <summary>
    /// Create a new domain
    /// </summary>
    /// <param name="createDto">Domain data</param>
    /// <returns>Created domain</returns>
    [HttpPost]
    public async Task<ActionResult<DomainDto>> CreateDomain([FromBody] CreateDomainDto createDto)
    {
        var userId = this.GetUserId();
        var domain = new RouteDomain
        {
            Name = createDto.Name
        };

        var result = await _domainService.CreateDomainAsync(domain, userId);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        var domainDto = MapToDto(result.Value);
        return CreatedAtAction(nameof(GetDomain), new { id = result.Value.Id.ToString() }, domainDto);
    }

    /// <summary>
    /// Update an existing domain by ID
    /// </summary>
    /// <param name="id">Domain ID</param>
    /// <param name="updateDto">Updated domain data</param>
    /// <returns>Updated domain</returns>
    [HttpPut("{id}")]
    public async Task<ActionResult<DomainDto>> UpdateDomain(string id, [FromBody] UpdateDomainDto updateDto)
    {
        if (!Guid.TryParse(id, out var domainId))
        {
            return BadRequest(new { error = "VALIDATION_ERROR", message = "Invalid domain ID format" });
        }

        var userId = this.GetUserId();
        var domain = new RouteDomain
        {
            Name = updateDto.Name
        };

        var result = await _domainService.UpdateDomainAsync(domainId, domain, userId);

        if (result.IsFailure)
        {
            return HandleError(result.ErrorCode ?? "UNKNOWN_ERROR", result.Error);
        }

        var domainDto = MapToDto(result.Value);
        return Ok(domainDto);
    }

    /// <summary>
    /// Delete a domain by ID
    /// </summary>
    /// <param name="id">Domain ID</param>
    /// <returns>No content</returns>
    [HttpDelete("{id}")]
    public async Task<IActionResult> DeleteDomain(string id)
    {
        if (!Guid.TryParse(id, out var domainId))
        {
            return BadRequest(new { error = "VALIDATION_ERROR", message = "Invalid domain ID format" });
        }

        var userId = this.GetUserId();
        var result = await _domainService.DeleteDomainAsync(domainId, userId);

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
            "INTERNAL_ERROR" => StatusCode(500, new { error = errorCode, message = errorMessage }),
            _ => StatusCode(500, new { error = "UNKNOWN_ERROR", message = "An unknown error occurred" })
        };
    }

    private static DomainDto MapToDto(RouteDomain domain)
    {
        return new DomainDto
        {
            Id = domain.Id,
            Name = domain.Name,
            OwnerId = domain.OwnerId,
            VerificationStatus = domain.VerificationStatus.ToString(),
            VerificationReason = domain.VerificationReason,
            LastVerificationCheck = domain.LastVerificationCheck,
            NextVerificationCheck = domain.NextVerificationCheck
        };
    }
}

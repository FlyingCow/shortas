using Microsoft.AspNetCore.Mvc;

namespace ShortasProxyApi.Presentation.Controllers;

[ApiController]
[Route("api/health")]
public class HealthController : ControllerBase
{
    private readonly ILogger<HealthController> _logger;

    public HealthController(ILogger<HealthController> logger)
    {
        _logger = logger;
    }

    /// <summary>
    /// Health check endpoint
    /// </summary>
    /// <returns>Health status</returns>
    [HttpGet]
    public IActionResult GetHealth()
    {
        return Ok(new
        {
            status = "healthy",
            timestamp = DateTime.UtcNow,
            version = "1.0.0",
            service = "Shortas Proxy API"
        });
    }

    /// <summary>
    /// Readiness check endpoint
    /// </summary>
    /// <returns>Readiness status</returns>
    [HttpGet("ready")]
    public IActionResult GetReadiness()
    {
        // Add any readiness checks here (database connectivity, external services, etc.)
        return Ok(new
        {
            status = "ready",
            timestamp = DateTime.UtcNow
        });
    }

    /// <summary>
    /// Liveness check endpoint
    /// </summary>
    /// <returns>Liveness status</returns>
    [HttpGet("live")]
    public IActionResult GetLiveness()
    {
        return Ok(new
        {
            status = "alive",
            timestamp = DateTime.UtcNow
        });
    }
}


using System.Collections.Concurrent;

namespace ShortasProxyApi.Infrastructure.Security;

public class RateLimitingMiddleware
{
    private readonly RequestDelegate _next;
    private readonly IConfiguration _configuration;
    private readonly ConcurrentDictionary<string, RateLimitInfo> _rateLimitStore = new();
    private readonly int _requestsPerMinute;
    private readonly int _burstLimit;

    public RateLimitingMiddleware(RequestDelegate next, IConfiguration configuration)
    {
        _next = next;
        _configuration = configuration;
        _requestsPerMinute = _configuration.GetValue<int>("RateLimiting:RequestsPerMinute", 100);
        _burstLimit = _configuration.GetValue<int>("RateLimiting:BurstLimit", 20);
    }

    public async Task InvokeAsync(HttpContext context)
    {
        var clientIp = GetClientIpAddress(context);
        var now = DateTime.UtcNow;
        
        var rateLimitInfo = _rateLimitStore.GetOrAdd(clientIp, _ => new RateLimitInfo());
        
        // Clean old requests
        rateLimitInfo.Requests.RemoveAll(r => r < now.AddMinutes(-1));
        
        // Check rate limit
        if (rateLimitInfo.Requests.Count >= _requestsPerMinute)
        {
            context.Response.StatusCode = 429;
            context.Response.Headers["Retry-After"] = "60";
            await context.Response.WriteAsync("Rate limit exceeded. Please try again later.");
            return;
        }
        
        // Check burst limit
        var recentRequests = rateLimitInfo.Requests.Count(r => r > now.AddSeconds(-10));
        if (recentRequests >= _burstLimit)
        {
            context.Response.StatusCode = 429;
            context.Response.Headers["Retry-After"] = "10";
            await context.Response.WriteAsync("Burst limit exceeded. Please slow down your requests.");
            return;
        }
        
        // Add current request
        rateLimitInfo.Requests.Add(now);
        
        await _next(context);
    }

    private static string GetClientIpAddress(HttpContext context)
    {
        var xForwardedFor = context.Request.Headers["X-Forwarded-For"].FirstOrDefault();
        if (!string.IsNullOrEmpty(xForwardedFor))
        {
            return xForwardedFor.Split(',')[0].Trim();
        }
        
        var xRealIp = context.Request.Headers["X-Real-IP"].FirstOrDefault();
        if (!string.IsNullOrEmpty(xRealIp))
        {
            return xRealIp;
        }
        
        return context.Connection.RemoteIpAddress?.ToString() ?? "unknown";
    }

    private class RateLimitInfo
    {
        public List<DateTime> Requests { get; } = new();
    }
}

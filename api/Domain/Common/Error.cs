namespace ShortasProxyApi.Domain.Common;

/// <summary>
/// Represents an error that can occur in the application
/// </summary>
public class Error
{
    public string Code { get; }
    public string Message { get; }
    public string? Details { get; }

    private Error(string code, string message, string? details = null)
    {
        Code = code;
        Message = message;
        Details = details;
    }

    public static Error None => new(string.Empty, string.Empty);

    // Validation errors
    public static Error Validation(string message, string? details = null) => 
        new("VALIDATION_ERROR", message, details);

    public static Error Required(string fieldName) => 
        new("REQUIRED_FIELD", $"The field '{fieldName}' is required");

    public static Error InvalidFormat(string fieldName, string expectedFormat) => 
        new("INVALID_FORMAT", $"The field '{fieldName}' has an invalid format. Expected: {expectedFormat}");

    // Authentication errors
    public static Error Unauthorized(string message = "Authentication required") => 
        new("UNAUTHORIZED", message);

    public static Error Forbidden(string message = "Access denied") => 
        new("FORBIDDEN", message);

    public static Error InvalidToken(string message = "Invalid or expired token") => 
        new("INVALID_TOKEN", message);

    // Business logic errors
    public static Error NotFound(string resource, string identifier) => 
        new("NOT_FOUND", $"{resource} with identifier '{identifier}' was not found");

    public static Error Conflict(string message, string? details = null) => 
        new("CONFLICT", message, details);

    public static Error BusinessRule(string message, string? details = null) => 
        new("BUSINESS_RULE_VIOLATION", message, details);

    // External service errors
    public static Error ExternalService(string serviceName, string message) => 
        new("EXTERNAL_SERVICE_ERROR", $"Error calling {serviceName}: {message}");

    public static Error Timeout(string serviceName, int timeoutSeconds) => 
        new("TIMEOUT", $"Request to {serviceName} timed out after {timeoutSeconds} seconds");

    public static Error CircuitBreakerOpen(string serviceName) => 
        new("CIRCUIT_BREAKER_OPEN", $"Circuit breaker for {serviceName} is open");

    // Rate limiting errors
    public static Error RateLimitExceeded(string message = "Rate limit exceeded") => 
        new("RATE_LIMIT_EXCEEDED", message);

    public static Error BurstLimitExceeded(string message = "Burst limit exceeded") => 
        new("BURST_LIMIT_EXCEEDED", message);

    // System errors
    public static Error Internal(string message, string? details = null) => 
        new("INTERNAL_ERROR", message, details);

    public static Error Database(string message, string? details = null) => 
        new("DATABASE_ERROR", message, details);

    public static Error Network(string message, string? details = null) => 
        new("NETWORK_ERROR", message, details);

    public override string ToString() => string.IsNullOrEmpty(Details) ? Message : $"{Message} - {Details}";
}


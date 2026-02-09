using ShortasProxyApi.Domain.Common;

namespace ShortasProxyApi.Domain.Interfaces;

/// <summary>
/// Generates unique short slash tags (link paths) for routes within a domain.
/// Uses a probabilistic approach to minimize database hits.
/// </summary>
public interface ISlashTagGenerator
{
    /// <summary>
    /// Generate a unique slash tag for the given domain.
    /// Starts with the shortest possible length (3 chars) and grows as needed.
    /// </summary>
    /// <param name="domainId">The domain to generate a unique tag for</param>
    /// <returns>A unique slash tag string</returns>
    Task<Result<string>> GenerateAsync(Guid domainId);
}

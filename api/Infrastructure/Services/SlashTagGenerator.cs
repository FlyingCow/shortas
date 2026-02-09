using System.Security.Cryptography;
using System.Text;
using Microsoft.EntityFrameworkCore;
using ShortasProxyApi.Domain.Common;
using ShortasProxyApi.Domain.Interfaces;
using ShortasProxyApi.Infrastructure.Data;

namespace ShortasProxyApi.Infrastructure.Services;

/// <summary>
/// Probabilistic slash tag generator that creates unique short link paths per domain.
///
/// Algorithm:
/// 1. Read the cached route count from DomainRouteCounts table (cheap PK lookup).
/// 2. Determine the optimal tag length: find the smallest L (starting at 3) where
///    the fill ratio (existingCount / alphabetSize^L) is below a threshold.
///    This ensures high probability that a random candidate is available.
/// 3. Generate a batch of random candidates at that length.
/// 4. Check the batch against the database in a single query (1 DB query).
/// 5. Return the first candidate that doesn't collide.
/// 6. If all candidates collide (extremely unlikely), retry with increased length.
///
/// Total: 2 cheap database queries in the common case (PK lookup + batch IN check).
/// </summary>
public class SlashTagGenerator : ISlashTagGenerator
{
    private const string Alphabet = "abcdefghijklmnopqrstuvwxyz0123456789";
    private const int MinLength = 3;
    private const int MaxLength = 10;
    private const int BatchSize = 10;
    private const double FillThreshold = 0.3; // grow length when >30% of the space is used
    private const int MaxRetries = 3;

    private readonly ApplicationDbContext _context;
    private readonly ILogger<SlashTagGenerator> _logger;

    public SlashTagGenerator(ApplicationDbContext context, ILogger<SlashTagGenerator> logger)
    {
        _context = context;
        _logger = logger;
    }

    public async Task<Result<string>> GenerateAsync(Guid domainId)
    {
        try
        {
            // Validate domain exists
            var domainExists = await _context.RouteDomains.AnyAsync(d => d.Id == domainId);
            if (!domainExists)
            {
                return Result<string>.Failure(Error.NotFound("Domain", domainId.ToString()));
            }

            // Step 1: Read cached route count for this domain (cheap single-row PK lookup)
            var countRow = await _context.DomainRouteCounts
                .FirstOrDefaultAsync(c => c.DomainId == domainId);
            var existingCount = countRow?.RouteCount ?? 0;

            // Step 2: Determine optimal length
            var length = CalculateOptimalLength(existingCount);

            // Step 3-6: Generate and verify with retries
            for (var retry = 0; retry < MaxRetries; retry++)
            {
                var candidates = GenerateCandidates(length, BatchSize);

                // Step 4: Batch-check which candidates already exist (1 DB query)
                var existingLinks = await _context.Routes
                    .Where(r => r.DomainId == domainId && candidates.Contains(r.Link))
                    .Select(r => r.Link)
                    .ToListAsync();

                var existingSet = new HashSet<string>(existingLinks);

                // Step 5: Return first available candidate
                var available = candidates.FirstOrDefault(c => !existingSet.Contains(c));
                if (available != null)
                {
                    _logger.LogDebug(
                        "Generated slash tag '{Tag}' for domain {DomainId} (length={Length}, retry={Retry})",
                        available, domainId, length, retry);
                    return Result<string>.Success(available);
                }

                // All candidates collided — increase length and retry
                _logger.LogWarning(
                    "All {BatchSize} candidates collided for domain {DomainId} at length {Length}, retrying with length {NextLength}",
                    BatchSize, domainId, length, length + 1);
                length = Math.Min(length + 1, MaxLength);
            }

            return Result<string>.Failure(
                Error.Internal("Failed to generate a unique slash tag after maximum retries"));
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error generating slash tag for domain {DomainId}", domainId);
            return Result<string>.Failure(Error.Internal("Failed to generate slash tag", ex.Message));
        }
    }

    /// <summary>
    /// Find the smallest length L (>= MinLength) where the fill ratio is below the threshold.
    /// Fill ratio = existingCount / alphabetSize^L
    /// </summary>
    private static int CalculateOptimalLength(int existingCount)
    {
        var alphabetSize = Alphabet.Length; // 36

        for (var length = MinLength; length <= MaxLength; length++)
        {
            var totalSpace = Math.Pow(alphabetSize, length);
            var fillRatio = existingCount / totalSpace;

            if (fillRatio < FillThreshold)
            {
                return length;
            }
        }

        return MaxLength;
    }

    /// <summary>
    /// Generate a batch of cryptographically random strings from the alphabet.
    /// </summary>
    private static List<string> GenerateCandidates(int length, int count)
    {
        var candidates = new List<string>(count);
        var alphabetLength = Alphabet.Length;

        for (var i = 0; i < count; i++)
        {
            var sb = new StringBuilder(length);
            for (var j = 0; j < length; j++)
            {
                var index = RandomNumberGenerator.GetInt32(alphabetLength);
                sb.Append(Alphabet[index]);
            }
            candidates.Add(sb.ToString());
        }

        return candidates;
    }
}

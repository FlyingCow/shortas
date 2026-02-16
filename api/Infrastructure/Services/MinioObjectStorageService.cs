using Amazon.S3;
using Amazon.S3.Model;
using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.Logging;
using ShortasProxyApi.Domain.Interfaces;

namespace ShortasProxyApi.Infrastructure.Services;

public class MinioObjectStorageService : IObjectStorageService
{
    private readonly IAmazonS3 _s3Client;
    private readonly ILogger<MinioObjectStorageService> _logger;
    private readonly string _bucketName;
    private readonly string _internalEndpoint;
    private readonly string _publicEndpoint;

    public MinioObjectStorageService(
        IAmazonS3 s3Client,
        IConfiguration configuration,
        ILogger<MinioObjectStorageService> logger)
    {
        _s3Client = s3Client;
        _logger = logger;
        _bucketName = configuration["S3:BucketName"] ?? "route-images";
        _internalEndpoint = configuration["S3:Endpoint"] ?? "http://localhost:9000";
        _publicEndpoint = configuration["S3:PublicEndpoint"] ?? _internalEndpoint;
    }

    /// <summary>
    /// Replaces the internal endpoint with the public endpoint in presigned URLs
    /// so they can be accessed from the browser.
    /// </summary>
    private string MakeUrlPublic(string url)
    {
        if (_internalEndpoint == _publicEndpoint)
            return url;

        // Extract host:port from internal endpoint (e.g., "shortas-minio:9000" from "http://shortas-minio:9000")
        var internalUri = new Uri(_internalEndpoint);
        var internalHostPort = internalUri.Host + (internalUri.IsDefaultPort ? "" : $":{internalUri.Port}");

        // Extract host:port from public endpoint
        var publicUri = new Uri(_publicEndpoint);
        var publicHostPort = publicUri.Host + (publicUri.IsDefaultPort ? "" : $":{publicUri.Port}");

        // Replace internal host with public host (handles both http and https)
        var result = url
            .Replace($"https://{internalHostPort}", $"{publicUri.Scheme}://{publicHostPort}")
            .Replace($"http://{internalHostPort}", $"{publicUri.Scheme}://{publicHostPort}");

        return result;
    }

    public async Task<string> GeneratePresignedPutUrlAsync(string key, string contentType, int expirationMinutes = 15)
    {
        try
        {
            var request = new GetPreSignedUrlRequest
            {
                BucketName = _bucketName,
                Key = key,
                Verb = HttpVerb.PUT,
                Expires = DateTime.UtcNow.AddMinutes(expirationMinutes),
                ContentType = contentType
            };

            var url = await _s3Client.GetPreSignedURLAsync(request);
            var publicUrl = MakeUrlPublic(url);
            _logger.LogDebug("Generated presigned PUT URL for key {Key}: {Url}", key, publicUrl);
            return publicUrl;
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to generate presigned PUT URL for key {Key}", key);
            throw;
        }
    }

    public async Task<string> GeneratePresignedGetUrlAsync(string key, int expirationMinutes = 60)
    {
        try
        {
            var request = new GetPreSignedUrlRequest
            {
                BucketName = _bucketName,
                Key = key,
                Verb = HttpVerb.GET,
                Expires = DateTime.UtcNow.AddMinutes(expirationMinutes)
            };

            var url = await _s3Client.GetPreSignedURLAsync(request);
            var publicUrl = MakeUrlPublic(url);
            _logger.LogDebug("Generated presigned GET URL for key {Key}: {Url}", key, publicUrl);
            return publicUrl;
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to generate presigned GET URL for key {Key}", key);
            throw;
        }
    }

    public async Task<bool> ObjectExistsAsync(string key)
    {
        try
        {
            var request = new GetObjectMetadataRequest
            {
                BucketName = _bucketName,
                Key = key
            };

            await _s3Client.GetObjectMetadataAsync(request);
            return true;
        }
        catch (AmazonS3Exception ex) when (ex.StatusCode == System.Net.HttpStatusCode.NotFound)
        {
            return false;
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to check if object exists for key {Key}", key);
            throw;
        }
    }

    public async Task DeleteObjectAsync(string key)
    {
        try
        {
            var request = new DeleteObjectRequest
            {
                BucketName = _bucketName,
                Key = key
            };

            await _s3Client.DeleteObjectAsync(request);
            _logger.LogDebug("Deleted object with key {Key}", key);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to delete object with key {Key}", key);
            throw;
        }
    }

    public string GetPublicUrl(string key)
    {
        // For publicly accessible bucket, return direct URL
        return $"{_publicEndpoint.TrimEnd('/')}/{_bucketName}/{key}";
    }
}

namespace ShortasProxyApi.Domain.Interfaces;

public interface IObjectStorageService
{
    /// <summary>
    /// Generates a presigned PUT URL for uploading an object.
    /// </summary>
    /// <param name="key">The object key (path) in the bucket</param>
    /// <param name="contentType">The content type of the object</param>
    /// <param name="expirationMinutes">URL expiration time in minutes</param>
    /// <returns>The presigned URL for uploading</returns>
    Task<string> GeneratePresignedPutUrlAsync(string key, string contentType, int expirationMinutes = 15);

    /// <summary>
    /// Generates a presigned GET URL for downloading an object.
    /// </summary>
    /// <param name="key">The object key (path) in the bucket</param>
    /// <param name="expirationMinutes">URL expiration time in minutes</param>
    /// <returns>The presigned URL for downloading</returns>
    Task<string> GeneratePresignedGetUrlAsync(string key, int expirationMinutes = 60);

    /// <summary>
    /// Checks if an object exists in the bucket.
    /// </summary>
    /// <param name="key">The object key (path) in the bucket</param>
    /// <returns>True if the object exists</returns>
    Task<bool> ObjectExistsAsync(string key);

    /// <summary>
    /// Deletes an object from the bucket.
    /// </summary>
    /// <param name="key">The object key (path) in the bucket</param>
    Task DeleteObjectAsync(string key);

    /// <summary>
    /// Gets the public URL for an object (if bucket is publicly accessible).
    /// </summary>
    /// <param name="key">The object key (path) in the bucket</param>
    /// <returns>The public URL</returns>
    string GetPublicUrl(string key);
}

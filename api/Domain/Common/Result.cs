namespace ShortasProxyApi.Domain.Common;

/// <summary>
/// Represents the result of an operation that can either succeed or fail
/// </summary>
/// <typeparam name="T">The type of the value returned on success</typeparam>
public class Result<T>
{
    public bool IsSuccess { get; }
    public bool IsFailure => !IsSuccess;
    public T Value { get; }
    public string Error { get; }
    public string? ErrorCode { get; }

    private Result(bool isSuccess, T value, string error, string? errorCode = null)
    {
        IsSuccess = isSuccess;
        Value = value;
        Error = error;
        ErrorCode = errorCode;
    }

    public static Result<T> Success(T value) => new(true, value, string.Empty);
    
    public static Result<T> Failure(string error, string? errorCode = null) => new(false, default!, error, errorCode);
    
    public static Result<T> Failure(Error error) => new(false, default!, error.Message, error.Code);

    public static implicit operator Result<T>(T value) => Success(value);
}

/// <summary>
/// Represents the result of an operation that can either succeed or fail (without a value)
/// </summary>
public class Result
{
    public bool IsSuccess { get; }
    public bool IsFailure => !IsSuccess;
    public string Error { get; }
    public string? ErrorCode { get; }

    private Result(bool isSuccess, string error, string? errorCode = null)
    {
        IsSuccess = isSuccess;
        Error = error;
        ErrorCode = errorCode;
    }

    public static Result Success() => new(true, string.Empty);
    
    public static Result Failure(string error, string? errorCode = null) => new(false, error, errorCode);
    
    public static Result Failure(Error error) => new(false, error.Message, error.Code);

    public static implicit operator Result(bool success) => success ? Success() : Failure("Operation failed");
}

/// <summary>
/// Represents the result of an operation that can either succeed or fail with a list of errors
/// </summary>
/// <typeparam name="T">The type of the value returned on success</typeparam>
public class Result<T, TError>
{
    public bool IsSuccess { get; }
    public bool IsFailure => !IsSuccess;
    public T Value { get; }
    public TError Error { get; }

    private Result(bool isSuccess, T value, TError error)
    {
        IsSuccess = isSuccess;
        Value = value;
        Error = error;
    }

    public static Result<T, TError> Success(T value) => new(true, value, default!);
    
    public static Result<T, TError> Failure(TError error) => new(false, default!, error);

    public static implicit operator Result<T, TError>(T value) => Success(value);
}

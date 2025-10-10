using System.ComponentModel.DataAnnotations;

namespace ShortasProxyApi.Domain.Common;

/// <summary>
/// Represents the result of a validation operation
/// </summary>
public class ValidationResult
{
    public bool IsValid { get; }
    public List<ValidationError> Errors { get; }

    private ValidationResult(bool isValid, List<ValidationError> errors)
    {
        IsValid = isValid;
        Errors = errors;
    }

    public static ValidationResult Success() => new(true, new List<ValidationError>());
    
    public static ValidationResult Failure(List<ValidationError> errors) => new(false, errors);
    
    public static ValidationResult Failure(ValidationError error) => new(false, new List<ValidationError> { error });
    
    public static ValidationResult Failure(string fieldName, string message) => 
        new(false, new List<ValidationError> { new ValidationError(fieldName, message) });

    public static implicit operator bool(ValidationResult result) => result.IsValid;
}

/// <summary>
/// Represents a validation error for a specific field
/// </summary>
public class ValidationError
{
    public string FieldName { get; }
    public string Message { get; }
    public object? AttemptedValue { get; }

    public ValidationError(string fieldName, string message, object? attemptedValue = null)
    {
        FieldName = fieldName;
        Message = message;
        AttemptedValue = attemptedValue;
    }
}

/// <summary>
/// Extension methods for validation
/// </summary>
public static class ValidationExtensions
{
    public static ValidationResult Validate<T>(this T obj) where T : class
    {
        var context = new ValidationContext(obj);
        var results = new List<System.ComponentModel.DataAnnotations.ValidationResult>();
        var isValid = Validator.TryValidateObject(obj, context, results, true);

        if (isValid)
            return ValidationResult.Success();

        var errors = results.Select(r => new ValidationError(
            r.MemberNames.FirstOrDefault() ?? "Unknown",
            r.ErrorMessage ?? "Validation failed",
            null
        )).ToList();

        return ValidationResult.Failure(errors);
    }

    public static ValidationResult ValidateProperty<T>(this T obj, string propertyName) where T : class
    {
        var context = new ValidationContext(obj) { MemberName = propertyName };
        var results = new List<System.ComponentModel.DataAnnotations.ValidationResult>();
        var property = typeof(T).GetProperty(propertyName);
        
        if (property == null)
            return ValidationResult.Failure(propertyName, "Property not found");

        var value = property.GetValue(obj);
        var isValid = Validator.TryValidateProperty(value, context, results);

        if (isValid)
            return ValidationResult.Success();

        var errors = results.Select(r => new ValidationError(
            propertyName,
            r.ErrorMessage ?? "Validation failed",
            value
        )).ToList();

        return ValidationResult.Failure(errors);
    }
}


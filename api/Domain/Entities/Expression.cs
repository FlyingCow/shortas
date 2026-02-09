using System.Text.Json.Serialization;

namespace ShortasProxyApi.Domain.Entities;

/// <summary>
/// Expression for conditional routing
/// Matches click-router Expression structure
/// </summary>
public class Expression
{
    [JsonPropertyName("default_operator")]
    [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    public DefaultOperator? DefaultOperator { get; set; }

    [JsonPropertyName("ua")]
    [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    public UACondition? UA { get; set; }

    [JsonPropertyName("os")]
    [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    public OSCondition? OS { get; set; }

    [JsonPropertyName("device")]
    [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    public DeviceCondition? Device { get; set; }

    [JsonPropertyName("lang")]
    [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    public LangCondition? Lang { get; set; }

    [JsonPropertyName("country")]
    [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    public CountryCondition? Country { get; set; }

    [JsonPropertyName("date")]
    [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    public DateCondition? Date { get; set; }

    [JsonPropertyName("rnd")]
    [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    public RNDCondition? RND { get; set; }

    [JsonPropertyName("day_of_week")]
    [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    public DayOfWeekCondition? DayOfWeek { get; set; }

    [JsonPropertyName("day_of_month")]
    [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    public DayOfMonthCondition? DayOfMonth { get; set; }

    [JsonPropertyName("month")]
    [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    public MonthCondition? Month { get; set; }

    [JsonPropertyName("and")]
    [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    public List<Expression>? And { get; set; }

    [JsonPropertyName("or")]
    [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    public List<Expression>? Or { get; set; }
}

[JsonConverter(typeof(JsonStringEnumConverter))]
public enum DefaultOperator
{
    And,
    Or
}

// String-based conditions (UA, OS, Device, Lang, Country)
public abstract class StringCondition
{
    [JsonPropertyName("eq")]
    [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    public string? EQ { get; set; }

    [JsonPropertyName("starts")]
    [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    public string? Starts { get; set; }

    [JsonPropertyName("ends")]
    [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    public string? Ends { get; set; }

    [JsonPropertyName("in")]
    [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    public List<string>? IN { get; set; }
}

public class UACondition : StringCondition { }
public class OSCondition : StringCondition { }
public class DeviceCondition : StringCondition { }
public class LangCondition : StringCondition { }
public class CountryCondition : StringCondition { }

// Date condition
public class DateCondition
{
    [JsonPropertyName("eq")]
    [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    public string? EQ { get; set; }

    [JsonPropertyName("gt")]
    [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    public string? GT { get; set; }

    [JsonPropertyName("lt")]
    [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    public string? LT { get; set; }

    [JsonPropertyName("in")]
    [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    public List<string>? IN { get; set; }
}

// Numeric conditions (DayOfMonth, DayOfWeek, Month, RND)
public abstract class NumericCondition
{
    [JsonPropertyName("eq")]
    [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    public int? EQ { get; set; }

    [JsonPropertyName("gt")]
    [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    public int? GT { get; set; }

    [JsonPropertyName("lt")]
    [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    public int? LT { get; set; }

    [JsonPropertyName("in")]
    [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    public List<int>? IN { get; set; }
}

public class DayOfMonthCondition : NumericCondition { }
public class DayOfWeekCondition : NumericCondition { }
public class MonthCondition : NumericCondition { }
public class RNDCondition : NumericCondition { }

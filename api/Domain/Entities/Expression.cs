using System.Text.Json.Serialization;

namespace ShortasProxyApi.Domain.Entities;

/// <summary>
/// Expression for conditional routing
/// Matches click-router Expression structure
/// </summary>
public class Expression
{
    [JsonPropertyName("default_operator")]
    public DefaultOperator? DefaultOperator { get; set; }

    [JsonPropertyName("ua")]
    public UACondition? UA { get; set; }

    [JsonPropertyName("os")]
    public OSCondition? OS { get; set; }

    [JsonPropertyName("device")]
    public DeviceCondition? Device { get; set; }

    [JsonPropertyName("lang")]
    public LangCondition? Lang { get; set; }

    [JsonPropertyName("country")]
    public CountryCondition? Country { get; set; }

    [JsonPropertyName("date")]
    public DateCondition? Date { get; set; }

    [JsonPropertyName("rnd")]
    public RNDCondition? RND { get; set; }

    [JsonPropertyName("day_of_week")]
    public DayOfWeekCondition? DayOfWeek { get; set; }

    [JsonPropertyName("day_of_month")]
    public DayOfMonthCondition? DayOfMonth { get; set; }

    [JsonPropertyName("month")]
    public MonthCondition? Month { get; set; }

    [JsonPropertyName("and")]
    public List<Expression>? And { get; set; }

    [JsonPropertyName("or")]
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
    public string? EQ { get; set; }

    [JsonPropertyName("starts")]
    public string? Starts { get; set; }

    [JsonPropertyName("ends")]
    public string? Ends { get; set; }

    [JsonPropertyName("in")]
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
    public string? EQ { get; set; }

    [JsonPropertyName("gt")]
    public string? GT { get; set; }

    [JsonPropertyName("lt")]
    public string? LT { get; set; }

    [JsonPropertyName("in")]
    public List<string>? IN { get; set; }
}

// Numeric conditions (DayOfMonth, DayOfWeek, Month, RND)
public abstract class NumericCondition
{
    [JsonPropertyName("eq")]
    public int? EQ { get; set; }

    [JsonPropertyName("gt")]
    public int? GT { get; set; }

    [JsonPropertyName("lt")]
    public int? LT { get; set; }

    [JsonPropertyName("in")]
    public List<int>? IN { get; set; }
}

public class DayOfMonthCondition : NumericCondition { }
public class DayOfWeekCondition : NumericCondition { }
public class MonthCondition : NumericCondition { }
public class RNDCondition : NumericCondition { }

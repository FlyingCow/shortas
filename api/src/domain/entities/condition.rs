//! Condition types for conditional routing expressions.

use serde::{Deserialize, Serialize};

/// A recursive condition expression that can be combined with AND/OR operators.
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Condition {
    #[serde(default)]
    #[serde(alias = "default_operator", alias = "DEFAULT_OPERATOR")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_operator: Option<DefaultOperator>,

    #[serde(default)]
    #[serde(alias = "ua", alias = "UA")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ua: Option<StringCondition>,

    #[serde(default)]
    #[serde(alias = "os", alias = "OS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<StringCondition>,

    #[serde(default)]
    #[serde(alias = "device", alias = "DEVICE")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<StringCondition>,

    #[serde(default)]
    #[serde(alias = "lang", alias = "LANG")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<StringCondition>,

    #[serde(default)]
    #[serde(alias = "country", alias = "COUNTRY")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<StringCondition>,

    #[serde(default)]
    #[serde(alias = "date", alias = "DATE")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<StringCondition>,

    #[serde(default)]
    #[serde(alias = "rnd", alias = "RND")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rnd: Option<NumericCondition>,

    #[serde(default)]
    #[serde(alias = "day_of_week", alias = "DAY_OF_WEEK")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<NumericCondition>,

    #[serde(default)]
    #[serde(alias = "day_of_month", alias = "DAY_OF_MONTH")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_month: Option<NumericCondition>,

    #[serde(default)]
    #[serde(alias = "month", alias = "MONTH")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<NumericCondition>,

    #[serde(default)]
    #[serde(alias = "and", alias = "AND")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and: Option<Vec<Box<Condition>>>,

    #[serde(default)]
    #[serde(alias = "or", alias = "OR")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or: Option<Vec<Box<Condition>>>,
}

/// Default operator for combining conditions.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DefaultOperator {
    #[serde(alias = "and", alias = "AND")]
    And,
    #[serde(alias = "or", alias = "OR")]
    Or,
}

/// String-based condition operators.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum StringCondition {
    #[serde(alias = "eq", alias = "EQ")]
    Eq(String),
    #[serde(alias = "starts", alias = "STARTS")]
    Starts(String),
    #[serde(alias = "ends", alias = "ENDS")]
    Ends(String),
    #[serde(rename = "in", alias = "IN")]
    In(Vec<String>),
}

/// Numeric condition operators.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum NumericCondition {
    #[serde(alias = "eq", alias = "EQ")]
    Eq(i32),
    #[serde(alias = "gt", alias = "GT")]
    Gt(i32),
    #[serde(alias = "lt", alias = "LT")]
    Lt(i32),
    #[serde(rename = "in", alias = "IN")]
    In(Vec<i32>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_condition_defaults_all_none() {
        let condition: Condition = serde_json::from_str("{}").unwrap();
        assert_eq!(condition, Condition::default());
        assert!(condition.ua.is_none());
        assert!(condition.and.is_none());
        assert!(condition.or.is_none());
    }

    #[test]
    fn test_condition_serialization_roundtrip() {
        let condition = Condition {
            ua: Some(StringCondition::Eq("Chrome".to_string())),
            ..Default::default()
        };
        let json = serde_json::to_string(&condition).unwrap();
        assert!(json.contains("Chrome"));
        let parsed: Condition = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, condition);
    }

    #[test]
    fn test_deeply_nested_and_or_condition() {
        let json = r#"{
            "or": [
                { "and": [ { "ua": { "eq": "Chrome" } }, { "os": { "eq": "Windows" } } ] },
                { "and": [ { "ua": { "eq": "Safari" } }, { "os": { "eq": "macOS" } } ] }
            ]
        }"#;
        let parsed: Condition = serde_json::from_str(json).unwrap();
        let or_branches = parsed.or.expect("expected or branches");
        assert_eq!(or_branches.len(), 2);

        let first_and = or_branches[0].and.as_ref().expect("expected and branch");
        assert_eq!(first_and.len(), 2);
        assert_eq!(first_and[0].ua, Some(StringCondition::Eq("Chrome".to_string())));
        assert_eq!(first_and[1].os, Some(StringCondition::Eq("Windows".to_string())));
    }

    #[test]
    fn test_field_name_aliases_are_case_insensitive() {
        let lower: Condition = serde_json::from_str(r#"{"ua": {"eq": "Chrome"}}"#).unwrap();
        let upper: Condition = serde_json::from_str(r#"{"UA": {"eq": "Chrome"}}"#).unwrap();
        assert_eq!(lower, upper);

        let lower_op: Condition = serde_json::from_str(r#"{"default_operator": "and"}"#).unwrap();
        let upper_op: Condition = serde_json::from_str(r#"{"DEFAULT_OPERATOR": "AND"}"#).unwrap();
        assert_eq!(lower_op, upper_op);
        assert_eq!(lower_op.default_operator, Some(DefaultOperator::And));
    }

    #[test]
    fn test_string_condition_variants() {
        assert_eq!(
            serde_json::from_str::<StringCondition>(r#"{"eq": "Chrome"}"#).unwrap(),
            StringCondition::Eq("Chrome".to_string())
        );
        assert_eq!(
            serde_json::from_str::<StringCondition>(r#"{"starts": "Chr"}"#).unwrap(),
            StringCondition::Starts("Chr".to_string())
        );
        assert_eq!(
            serde_json::from_str::<StringCondition>(r#"{"ends": "me"}"#).unwrap(),
            StringCondition::Ends("me".to_string())
        );
        assert_eq!(
            serde_json::from_str::<StringCondition>(r#"{"in": ["Chrome", "Firefox"]}"#).unwrap(),
            StringCondition::In(vec!["Chrome".to_string(), "Firefox".to_string()])
        );
    }

    #[test]
    fn test_numeric_condition_variants() {
        assert_eq!(
            serde_json::from_str::<NumericCondition>(r#"{"eq": 5}"#).unwrap(),
            NumericCondition::Eq(5)
        );
        assert_eq!(
            serde_json::from_str::<NumericCondition>(r#"{"gt": 1}"#).unwrap(),
            NumericCondition::Gt(1)
        );
        assert_eq!(
            serde_json::from_str::<NumericCondition>(r#"{"lt": 10}"#).unwrap(),
            NumericCondition::Lt(10)
        );
        assert_eq!(
            serde_json::from_str::<NumericCondition>(r#"{"in": [1, 2, 3]}"#).unwrap(),
            NumericCondition::In(vec![1, 2, 3])
        );
    }

    #[test]
    fn test_condition_roundtrip_skips_none_fields() {
        let condition = Condition {
            rnd: Some(NumericCondition::Gt(50)),
            ..Default::default()
        };
        let json = serde_json::to_string(&condition).unwrap();
        assert!(!json.contains("\"ua\""));
        assert!(!json.contains("\"and\""));
        assert!(json.contains("rnd"));

        let parsed: Condition = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, condition);
    }
}

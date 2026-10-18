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

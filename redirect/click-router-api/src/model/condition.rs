use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Condition {
    
    #[serde(default)]
    #[serde(alias="default_operator", alias="DEFAULT_OPERATOR")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_operator: Option<DefaultOperator>,

    #[serde(default)]
    #[serde(alias="ua", alias="UA")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ua: Option<UA>,

    #[serde(default)]
    #[serde(alias="os", alias="OS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<OS>,

    #[serde(default)]
    #[serde(alias="date", alias="DATE")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Date>,
    //Query: Query,

    #[serde(default)]
    #[serde(alias="rnd", alias="RND")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rnd: Option<RND>,

    #[serde(default)]
    #[serde(alias="day_of_week", alias="DAY_OF_WEEK")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<DayOfWeek>,

    #[serde(default)]
    #[serde(alias="day_of_month", alias="DAY_OF_MONTH")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_month: Option<DayOfMonth>,

    #[serde(default)]
    #[serde(alias="month", alias="MONTH")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<Month>,

    #[serde(default)]
    #[serde(alias="and", alias="AND")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and: Option<Vec<Box<Condition>>>,

    #[serde(default)]
    #[serde(alias="or", alias="OR")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or: Option<Vec<Box<Condition>>>
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DefaultOperator {
    #[serde(alias="and", alias="AND")]
    And,
    #[serde(alias="or", alias="OR")]
    Or,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum UA {
    #[serde(alias="eq", alias="EQ")]
    EQ(String),
    #[serde(alias="starts", alias="STARTS")]
    Starts(String),
    #[serde(alias="ends", alias="ENDS")]
    Ends(String),
    #[serde(alias="in", alias="IN")]
    IN(Vec<String>)
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum OS {
    #[serde(alias="eq", alias="EQ")]
    EQ(String),
    #[serde(alias="starts", alias="STARTS")]
    Starts(String),
    #[serde(alias="ends", alias="ENDS")]
    Ends(String),
    #[serde(alias="in", alias="IN")]
    IN(Vec<String>)
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Date {
    #[serde(alias="eq", alias="EQ")]
    EQ(String),
    #[serde(alias="in", alias="IN")]
    IN(Vec<String>)
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DayOfMonth {
    #[serde(alias="eq", alias="EQ")]
    EQ(u32),
    #[serde(alias="gt", alias="GT")]
    GT(u32),
    #[serde(alias="lt", alias="LT")]
    LT(u32),
    #[serde(alias="in", alias="IN")]
    IN(Vec<u32>)
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DayOfWeek {
    #[serde(alias="eq", alias="EQ")]
    EQ(u32),
    #[serde(alias="gt", alias="GT")]
    GT(u32),
    #[serde(alias="lt", alias="LT")]
    LT(u32),
    #[serde(alias="in", alias="IN")]
    IN(Vec<u32>)
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Month {
    #[serde(alias="eq", alias="EQ")]
    EQ(u32),
    #[serde(alias="gt", alias="GT")]
    GT(u32),
    #[serde(alias="lt", alias="LT")]
    LT(u32),
    #[serde(alias="in", alias="IN")]
    IN(Vec<u32>)
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum RND {
    #[serde(alias="eq", alias="EQ")]
    EQ(u32),
    #[serde(alias="gt", alias="GT")]
    GT(u32),
    #[serde(alias="lt", alias="LT")]
    LT(u32),
    #[serde(alias="in", alias="IN")]
    IN(Vec<u32>)
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
            ua: Some(UA::EQ("Chrome".to_string())),
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
        assert_eq!(first_and[0].ua, Some(UA::EQ("Chrome".to_string())));
        assert_eq!(first_and[1].os, Some(OS::EQ("Windows".to_string())));
    }

    #[test]
    fn test_field_name_aliases_are_case_insensitive() {
        let lower: Condition = serde_json::from_str(r#"{"ua": {"eq": "Chrome"}}"#).unwrap();
        let upper: Condition = serde_json::from_str(r#"{"UA": {"eq": "Chrome"}}"#).unwrap();
        assert_eq!(lower, upper);

        let lower_op: Condition =
            serde_json::from_str(r#"{"default_operator": "And"}"#).unwrap();
        let upper_op: Condition =
            serde_json::from_str(r#"{"DEFAULT_OPERATOR": "And"}"#).unwrap();
        assert_eq!(lower_op, upper_op);
        assert_eq!(lower_op.default_operator, Some(DefaultOperator::And));
    }

    #[test]
    fn test_default_operator_variant_aliases() {
        assert_eq!(
            serde_json::from_str::<DefaultOperator>("\"And\"").unwrap(),
            DefaultOperator::And
        );
        assert_eq!(
            serde_json::from_str::<DefaultOperator>("\"and\"").unwrap(),
            DefaultOperator::And
        );
        assert_eq!(
            serde_json::from_str::<DefaultOperator>("\"AND\"").unwrap(),
            DefaultOperator::And
        );
        assert_eq!(
            serde_json::from_str::<DefaultOperator>("\"Or\"").unwrap(),
            DefaultOperator::Or
        );
        assert_eq!(
            serde_json::from_str::<DefaultOperator>("\"or\"").unwrap(),
            DefaultOperator::Or
        );
        assert_eq!(
            serde_json::from_str::<DefaultOperator>("\"OR\"").unwrap(),
            DefaultOperator::Or
        );
    }

    #[test]
    fn test_ua_and_os_string_variants() {
        assert_eq!(
            serde_json::from_str::<UA>(r#"{"eq": "Chrome"}"#).unwrap(),
            UA::EQ("Chrome".to_string())
        );
        assert_eq!(
            serde_json::from_str::<UA>(r#"{"EQ": "Chrome"}"#).unwrap(),
            UA::EQ("Chrome".to_string())
        );
        assert_eq!(
            serde_json::from_str::<UA>(r#"{"starts": "Chr"}"#).unwrap(),
            UA::Starts("Chr".to_string())
        );
        assert_eq!(
            serde_json::from_str::<UA>(r#"{"ends": "me"}"#).unwrap(),
            UA::Ends("me".to_string())
        );
        assert_eq!(
            serde_json::from_str::<UA>(r#"{"in": ["Chrome", "Firefox"]}"#).unwrap(),
            UA::IN(vec!["Chrome".to_string(), "Firefox".to_string()])
        );
        assert_eq!(
            serde_json::from_str::<OS>(r#"{"eq": "Windows"}"#).unwrap(),
            OS::EQ("Windows".to_string())
        );
    }

    #[test]
    fn test_date_variants() {
        assert_eq!(
            serde_json::from_str::<Date>(r#"{"eq": "2024-01-01"}"#).unwrap(),
            Date::EQ("2024-01-01".to_string())
        );
        assert_eq!(
            serde_json::from_str::<Date>(r#"{"in": ["2024-01-01", "2024-02-01"]}"#).unwrap(),
            Date::IN(vec!["2024-01-01".to_string(), "2024-02-01".to_string()])
        );
    }

    #[test]
    fn test_numeric_like_variants() {
        assert_eq!(
            serde_json::from_str::<RND>(r#"{"eq": 5}"#).unwrap(),
            RND::EQ(5)
        );
        assert_eq!(
            serde_json::from_str::<RND>(r#"{"gt": 1}"#).unwrap(),
            RND::GT(1)
        );
        assert_eq!(
            serde_json::from_str::<RND>(r#"{"lt": 10}"#).unwrap(),
            RND::LT(10)
        );
        assert_eq!(
            serde_json::from_str::<RND>(r#"{"in": [1, 2, 3]}"#).unwrap(),
            RND::IN(vec![1, 2, 3])
        );

        assert_eq!(
            serde_json::from_str::<DayOfWeek>(r#"{"eq": 1}"#).unwrap(),
            DayOfWeek::EQ(1)
        );
        assert_eq!(
            serde_json::from_str::<DayOfMonth>(r#"{"gt": 15}"#).unwrap(),
            DayOfMonth::GT(15)
        );
        assert_eq!(
            serde_json::from_str::<Month>(r#"{"lt": 6}"#).unwrap(),
            Month::LT(6)
        );
    }

    #[test]
    fn test_condition_roundtrip_skips_none_fields() {
        let condition = Condition {
            rnd: Some(RND::GT(50)),
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
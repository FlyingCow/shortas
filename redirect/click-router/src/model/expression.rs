use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Expression{
    
    #[serde(alias="default_operator", alias="DEFAULT_OPERATOR")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_operator: Option<DefaultOperator>,

    #[serde(alias="ua", alias="UA")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ua: Option<UA>,

    #[serde(alias="os", alias="OS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<OS>,

    #[serde(alias="date", alias="DATE")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Date>,
    //Query: Query,

    #[serde(alias="rnd", alias="RND")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rnd: Option<RND>,

    #[serde(alias="day_of_week", alias="DAY_OF_WEEK")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<DayOfWeek>,

    #[serde(alias="day_of_month", alias="DAY_OF_MONTH")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_month: Option<DayOfMonth>,

    #[serde(alias="month", alias="MONTH")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<Month>,

    #[serde(alias="and", alias="AND")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and: Option<Vec<Box<Expression>>>,

    #[serde(alias="or", alias="OR")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or: Option<Vec<Box<Expression>>>
}

impl Default for Expression{
    fn default() -> Self {
        Self { 
            default_operator: Default::default(),
            ua: Default::default(),
            os: Default::default(),
            date: Default::default(),
            rnd: Default::default(),
            day_of_week: Default::default(),
            day_of_month: Default::default(),
            month: Default::default(),
            and: Default::default(),
            or: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DefaultOperator {
    #[serde(alias="and", alias="AND")]
    And,
    #[serde(alias="or", alias="OR")]
    Or,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Date {
    #[serde(alias="eq", alias="EQ")]
    EQ(String),
    #[serde(alias="in", alias="IN")]
    IN(Vec<String>)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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
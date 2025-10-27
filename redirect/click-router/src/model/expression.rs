use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Expression {
    #[serde(alias = "default_operator", alias = "DEFAULT_OPERATOR")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_operator: Option<DefaultOperator>,

    #[serde(alias = "ua", alias = "UA")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ua: Option<UA>,

    #[serde(alias = "os", alias = "OS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<OS>,

    #[serde(alias = "device", alias = "DEVICE")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Device>,

    #[serde(alias = "lang", alias = "LANG")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<Lang>,

    #[serde(alias = "country", alias = "COUNTRY")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Country>,

    #[serde(alias = "date", alias = "DATE")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Date>,
    
    #[serde(alias = "rnd", alias = "RND")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rnd: Option<RND>,

    #[serde(alias = "day_of_week", alias = "DAY_OF_WEEK")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<DayOfWeek>,

    #[serde(alias = "day_of_month", alias = "DAY_OF_MONTH")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_month: Option<DayOfMonth>,

    #[serde(alias = "month", alias = "MONTH")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<Month>,

    #[serde(alias = "and", alias = "AND")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and: Option<Vec<Box<Expression>>>,

    #[serde(alias = "or", alias = "OR")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or: Option<Vec<Box<Expression>>>,
}

impl Default for Expression {
    fn default() -> Self {
        Self {
            default_operator: Default::default(),
            lang: Default::default(),
            ua: Default::default(),
            os: Default::default(),
            device: Default::default(),
            country: Default::default(),
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

impl Expression {
    ///
    /// Checks if current expression or subsequential expressions need device to be preloaded.
    ///
    pub fn needs_device(&self) -> bool {
        let curent = self.device.is_some();
        let and = self.and.is_some()
            && self
                .and
                .as_ref()
                .unwrap()
                .iter()
                .any(|item| item.needs_device());
        let or = self.or.is_some()
            && self
                .or
                .as_ref()
                .unwrap()
                .iter()
                .any(|item| item.needs_device());

        curent || and || or
    }

    ///
    /// Checks if current expression or subsequential expressions need os to be preloaded.
    ///
    pub fn needs_os(&self) -> bool {
        let curent = self.os.is_some();
        let and = self.and.is_some()
            && self
                .and
                .as_ref()
                .unwrap()
                .iter()
                .any(|item| item.needs_os());
        let or = self.or.is_some()
            && self
                .or
                .as_ref()
                .unwrap()
                .iter()
                .any(|item| item.needs_os());

        curent || and || or
    }

    ///
    /// Checks if current expression or subsequential expressions need browser to be preloaded.
    ///
    pub fn needs_ua(&self) -> bool {
        let curent = self.ua.is_some();
        let and = self.and.is_some()
            && self
                .and
                .as_ref()
                .unwrap()
                .iter()
                .any(|item| item.needs_ua());
        let or = self.or.is_some()
            && self
                .or
                .as_ref()
                .unwrap()
                .iter()
                .any(|item| item.needs_ua());

        curent || and || or
    }

    ///
    /// Checks if current expression or subsequential expressions need country to be preloaded.
    ///
    pub fn needs_country(&self) -> bool {
        let curent = self.country.is_some();
        let and = self.and.is_some()
            && self
                .and
                .as_ref()
                .unwrap()
                .iter()
                .any(|item| item.needs_country());
        let or = self.or.is_some()
            && self
                .or
                .as_ref()
                .unwrap()
                .iter()
                .any(|item| item.needs_country());

        curent || and || or
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DefaultOperator {
    #[serde(alias = "and", alias = "AND")]
    And,
    #[serde(alias = "or", alias = "OR")]
    Or,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Lang {
    #[serde(alias = "eq", alias = "EQ")]
    EQ(String),
    #[serde(alias = "in", alias = "IN")]
    IN(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UA {
    #[serde(alias = "eq", alias = "EQ")]
    EQ(String),
    #[serde(alias = "starts", alias = "STARTS")]
    Starts(String),
    #[serde(alias = "ends", alias = "ENDS")]
    Ends(String),
    #[serde(alias = "in", alias = "IN")]
    IN(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OS {
    #[serde(alias = "eq", alias = "EQ")]
    EQ(String),
    #[serde(alias = "starts", alias = "STARTS")]
    Starts(String),
    #[serde(alias = "ends", alias = "ENDS")]
    Ends(String),
    #[serde(alias = "in", alias = "IN")]
    IN(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Device {
    #[serde(alias = "eq", alias = "EQ")]
    EQ(String),
    #[serde(alias = "starts", alias = "STARTS")]
    Starts(String),
    #[serde(alias = "ends", alias = "ENDS")]
    Ends(String),
    #[serde(alias = "in", alias = "IN")]
    IN(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Country {
    #[serde(alias = "eq", alias = "EQ")]
    EQ(String),
    #[serde(alias = "starts", alias = "STARTS")]
    Starts(String),
    #[serde(alias = "ends", alias = "ENDS")]
    Ends(String),
    #[serde(alias = "in", alias = "IN")]
    IN(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Date {
    #[serde(alias = "eq", alias = "EQ")]
    EQ(String),
    #[serde(alias = "gt", alias = "GT")]
    GT(String),
    #[serde(alias = "lt", alias = "LT")]
    LT(String),
    #[serde(alias = "in", alias = "IN")]
    IN(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DayOfMonth {
    #[serde(alias = "eq", alias = "EQ")]
    EQ(u32),
    #[serde(alias = "gt", alias = "GT")]
    GT(u32),
    #[serde(alias = "lt", alias = "LT")]
    LT(u32),
    #[serde(alias = "in", alias = "IN")]
    IN(Vec<u32>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DayOfWeek {
    #[serde(alias = "eq", alias = "EQ")]
    EQ(u32),
    #[serde(alias = "gt", alias = "GT")]
    GT(u32),
    #[serde(alias = "lt", alias = "LT")]
    LT(u32),
    #[serde(alias = "in", alias = "IN")]
    IN(Vec<u32>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Month {
    #[serde(alias = "eq", alias = "EQ")]
    EQ(u32),
    #[serde(alias = "gt", alias = "GT")]
    GT(u32),
    #[serde(alias = "lt", alias = "LT")]
    LT(u32),
    #[serde(alias = "in", alias = "IN")]
    IN(Vec<u32>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RND {
    #[serde(alias = "eq", alias = "EQ")]
    EQ(u32),
    #[serde(alias = "gt", alias = "GT")]
    GT(u32),
    #[serde(alias = "lt", alias = "LT")]
    LT(u32),
    #[serde(alias = "in", alias = "IN")]
    IN(Vec<u32>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_default_expression() {
        let expr = Expression::default();

        assert!(expr.default_operator.is_none());
        assert!(expr.ua.is_none());
        assert!(expr.os.is_none());
        assert!(expr.device.is_none());
        assert!(expr.lang.is_none());
        assert!(expr.country.is_none());
        assert!(expr.date.is_none());
        assert!(expr.rnd.is_none());
        assert!(expr.day_of_week.is_none());
        assert!(expr.day_of_month.is_none());
        assert!(expr.month.is_none());
        assert!(expr.and.is_none());
        assert!(expr.or.is_none());
    }

    #[test]
    fn should_detect_needs_device() {
        let mut expr = Expression::default();
        assert!(!expr.needs_device());

        expr.device = Some(Device::EQ("mobile".to_string()));
        assert!(expr.needs_device());
    }

    #[test]
    fn should_detect_needs_device_in_nested_and() {
        let inner_expr = Expression {
            device: Some(Device::EQ("mobile".to_string())),
            ..Default::default()
        };

        let expr = Expression {
            and: Some(vec![Box::new(inner_expr)]),
            ..Default::default()
        };

        assert!(expr.needs_device());
    }

    #[test]
    fn should_detect_needs_device_in_nested_or() {
        let inner_expr = Expression {
            device: Some(Device::EQ("tablet".to_string())),
            ..Default::default()
        };

        let expr = Expression {
            or: Some(vec![Box::new(inner_expr)]),
            ..Default::default()
        };

        assert!(expr.needs_device());
    }

    #[test]
    fn should_detect_needs_os() {
        let mut expr = Expression::default();
        assert!(!expr.needs_os());

        expr.os = Some(OS::EQ("windows".to_string()));
        assert!(expr.needs_os());
    }

    #[test]
    fn should_detect_needs_os_in_nested_expressions() {
        let inner_expr = Expression {
            os: Some(OS::IN(vec!["macos".to_string(), "linux".to_string()])),
            ..Default::default()
        };

        let expr = Expression {
            and: Some(vec![Box::new(inner_expr)]),
            ..Default::default()
        };

        assert!(expr.needs_os());
    }

    #[test]
    fn should_detect_needs_ua() {
        let mut expr = Expression::default();
        assert!(!expr.needs_ua());

        expr.ua = Some(UA::Starts("Mozilla".to_string()));
        assert!(expr.needs_ua());
    }

    #[test]
    fn should_detect_needs_ua_in_nested_expressions() {
        let inner_expr = Expression {
            ua: Some(UA::Ends("Safari".to_string())),
            ..Default::default()
        };

        let expr = Expression {
            or: Some(vec![Box::new(inner_expr)]),
            ..Default::default()
        };

        assert!(expr.needs_ua());
    }

    #[test]
    fn should_detect_needs_country() {
        let mut expr = Expression::default();
        assert!(!expr.needs_country());

        expr.country = Some(Country::EQ("US".to_string()));
        assert!(expr.needs_country());
    }

    #[test]
    fn should_detect_needs_country_in_nested_expressions() {
        let inner_expr = Expression {
            country: Some(Country::IN(vec!["US".to_string(), "CA".to_string()])),
            ..Default::default()
        };

        let expr = Expression {
            and: Some(vec![Box::new(inner_expr)]),
            ..Default::default()
        };

        assert!(expr.needs_country());
    }

    #[test]
    fn should_serialize_and_deserialize_expression() {
        let expr = Expression {
            default_operator: Some(DefaultOperator::And),
            ua: Some(UA::EQ("Chrome".to_string())),
            os: Some(OS::EQ("Windows".to_string())),
            device: Some(Device::EQ("desktop".to_string())),
            country: Some(Country::EQ("US".to_string())),
            ..Default::default()
        };

        let json = serde_json::to_string(&expr).unwrap();
        let deserialized: Expression = serde_json::from_str(&json).unwrap();

        assert!(deserialized.default_operator.is_some());
        assert!(deserialized.ua.is_some());
        assert!(deserialized.os.is_some());
        assert!(deserialized.device.is_some());
        assert!(deserialized.country.is_some());
    }

    #[test]
    fn should_handle_ua_variants() {
        let eq = UA::EQ("Chrome".to_string());
        assert!(matches!(eq, UA::EQ(_)));

        let starts = UA::Starts("Mozilla".to_string());
        assert!(matches!(starts, UA::Starts(_)));

        let ends = UA::Ends("Safari".to_string());
        assert!(matches!(ends, UA::Ends(_)));

        let in_list = UA::IN(vec!["Chrome".to_string(), "Firefox".to_string()]);
        assert!(matches!(in_list, UA::IN(_)));
    }

    #[test]
    fn should_handle_os_variants() {
        let eq = OS::EQ("Windows".to_string());
        assert!(matches!(eq, OS::EQ(_)));

        let starts = OS::Starts("Mac".to_string());
        assert!(matches!(starts, OS::Starts(_)));

        let ends = OS::Ends("Linux".to_string());
        assert!(matches!(ends, OS::Ends(_)));

        let in_list = OS::IN(vec!["Windows".to_string(), "macOS".to_string()]);
        assert!(matches!(in_list, OS::IN(_)));
    }

    #[test]
    fn should_handle_device_variants() {
        let eq = Device::EQ("mobile".to_string());
        assert!(matches!(eq, Device::EQ(_)));

        let starts = Device::Starts("iPhone".to_string());
        assert!(matches!(starts, Device::Starts(_)));

        let ends = Device::Ends("Plus".to_string());
        assert!(matches!(ends, Device::Ends(_)));

        let in_list = Device::IN(vec!["mobile".to_string(), "tablet".to_string()]);
        assert!(matches!(in_list, Device::IN(_)));
    }

    #[test]
    fn should_handle_country_variants() {
        let eq = Country::EQ("US".to_string());
        assert!(matches!(eq, Country::EQ(_)));

        let starts = Country::Starts("U".to_string());
        assert!(matches!(starts, Country::Starts(_)));

        let ends = Country::Ends("S".to_string());
        assert!(matches!(ends, Country::Ends(_)));

        let in_list = Country::IN(vec!["US".to_string(), "CA".to_string(), "UK".to_string()]);
        assert!(matches!(in_list, Country::IN(_)));
    }

    #[test]
    fn should_handle_date_variants() {
        let eq = Date::EQ("2024-01-01".to_string());
        assert!(matches!(eq, Date::EQ(_)));

        let gt = Date::GT("2024-01-01".to_string());
        assert!(matches!(gt, Date::GT(_)));

        let lt = Date::LT("2024-12-31".to_string());
        assert!(matches!(lt, Date::LT(_)));

        let in_list = Date::IN(vec!["2024-01-01".to_string(), "2024-12-25".to_string()]);
        assert!(matches!(in_list, Date::IN(_)));
    }

    #[test]
    fn should_handle_day_of_month_variants() {
        let eq = DayOfMonth::EQ(15);
        assert!(matches!(eq, DayOfMonth::EQ(15)));

        let gt = DayOfMonth::GT(20);
        assert!(matches!(gt, DayOfMonth::GT(20)));

        let lt = DayOfMonth::LT(10);
        assert!(matches!(lt, DayOfMonth::LT(10)));

        let in_list = DayOfMonth::IN(vec![1, 15, 31]);
        assert!(matches!(in_list, DayOfMonth::IN(_)));
    }

    #[test]
    fn should_handle_day_of_week_variants() {
        let eq = DayOfWeek::EQ(1);
        assert!(matches!(eq, DayOfWeek::EQ(1)));

        let gt = DayOfWeek::GT(3);
        assert!(matches!(gt, DayOfWeek::GT(3)));

        let lt = DayOfWeek::LT(5);
        assert!(matches!(lt, DayOfWeek::LT(5)));

        let in_list = DayOfWeek::IN(vec![1, 2, 5, 6, 7]);
        assert!(matches!(in_list, DayOfWeek::IN(_)));
    }

    #[test]
    fn should_handle_month_variants() {
        let eq = Month::EQ(6);
        assert!(matches!(eq, Month::EQ(6)));

        let gt = Month::GT(3);
        assert!(matches!(gt, Month::GT(3)));

        let lt = Month::LT(9);
        assert!(matches!(lt, Month::LT(9)));

        let in_list = Month::IN(vec![1, 6, 12]);
        assert!(matches!(in_list, Month::IN(_)));
    }

    #[test]
    fn should_handle_rnd_variants() {
        let eq = RND::EQ(50);
        assert!(matches!(eq, RND::EQ(50)));

        let gt = RND::GT(75);
        assert!(matches!(gt, RND::GT(75)));

        let lt = RND::LT(25);
        assert!(matches!(lt, RND::LT(25)));

        let in_list = RND::IN(vec![10, 20, 30, 40]);
        assert!(matches!(in_list, RND::IN(_)));
    }

    #[test]
    fn should_handle_lang_variants() {
        let eq = Lang::EQ("en".to_string());
        assert!(matches!(eq, Lang::EQ(_)));

        let in_list = Lang::IN(vec!["en".to_string(), "fr".to_string(), "de".to_string()]);
        assert!(matches!(in_list, Lang::IN(_)));
    }

    #[test]
    fn should_handle_default_operator_variants() {
        let and = DefaultOperator::And;
        assert!(matches!(and, DefaultOperator::And));

        let or = DefaultOperator::Or;
        assert!(matches!(or, DefaultOperator::Or));
    }

    #[test]
    fn should_clone_expression() {
        let expr = Expression {
            ua: Some(UA::EQ("Chrome".to_string())),
            os: Some(OS::EQ("Windows".to_string())),
            ..Default::default()
        };

        let cloned = expr.clone();

        assert!(cloned.ua.is_some());
        assert!(cloned.os.is_some());
    }

    #[test]
    fn should_handle_complex_nested_expressions() {
        let inner1 = Expression {
            device: Some(Device::EQ("mobile".to_string())),
            ..Default::default()
        };

        let inner2 = Expression {
            os: Some(OS::IN(vec!["iOS".to_string(), "Android".to_string()])),
            ..Default::default()
        };

        let expr = Expression {
            and: Some(vec![Box::new(inner1), Box::new(inner2)]),
            ..Default::default()
        };

        assert!(expr.needs_device());
        assert!(expr.needs_os());
        assert!(!expr.needs_ua());
        assert!(!expr.needs_country());
    }
}

use chrono::{DateTime, prelude::*, Utc};
use serde::{Serialize, Deserialize};
use uaparser::Parser;
use woothee::parser::Parser as Woothee;
use uaparser::UserAgentParser;
use fast_uaparser::{Device, OperatingSystem, UserAgent};
use stopwatch::{Stopwatch};

#[derive(Serialize, Deserialize, Debug)]
struct Expression {
    
    #[serde(alias="default_operator", alias="DEFAULT_OPERATOR")]
    #[serde(skip_serializing_if = "Option::is_none")]
    default_operator: Option<DefaultOperator>,

    #[serde(alias="ua", alias="UA")]
    #[serde(skip_serializing_if = "Option::is_none")]
    ua: Option<UA>,

    #[serde(alias="os", alias="OS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    os: Option<OS>,

    #[serde(alias="date", alias="DATE")]
    #[serde(skip_serializing_if = "Option::is_none")]
    date: Option<Date>,
    //Query: Query,

    #[serde(alias="rnd", alias="RND")]
    #[serde(skip_serializing_if = "Option::is_none")]
    rnd: Option<RND>,

    #[serde(alias="day_of_week", alias="DAY_OF_WEEK")]
    #[serde(skip_serializing_if = "Option::is_none")]
    day_of_week: Option<DayOfWeek>,

    #[serde(alias="day_of_month", alias="DAY_OF_MONTH")]
    #[serde(skip_serializing_if = "Option::is_none")]
    day_of_month: Option<DayOfMonth>,

    #[serde(alias="month", alias="MONTH")]
    #[serde(skip_serializing_if = "Option::is_none")]
    month: Option<Month>,

    #[serde(alias="and", alias="AND")]
    #[serde(skip_serializing_if = "Option::is_none")]
    and: Option<Vec<Box<Expression>>>,

    #[serde(alias="or", alias="OR")]
    #[serde(skip_serializing_if = "Option::is_none")]
    or: Option<Vec<Box<Expression>>>
}

impl Default for Expression {
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

#[derive(Serialize, Deserialize, Debug)]
enum DefaultOperator {
    #[serde(alias="and", alias="AND")]
    And,
    #[serde(alias="or", alias="OR")]
    Or,
}

#[derive(Serialize, Deserialize, Debug)]
enum UA {
    #[serde(alias="eq", alias="EQ")]
    EQ(String),
    #[serde(alias="starts", alias="STARTS")]
    Starts(String),
    #[serde(alias="ends", alias="ENDS")]
    Ends(String),
    #[serde(alias="in", alias="IN")]
    IN(Vec<String>)
}

#[derive(Serialize, Deserialize, Debug)]
enum OS {
    #[serde(alias="eq", alias="EQ")]
    EQ(String),
    #[serde(alias="starts", alias="STARTS")]
    Starts(String),
    #[serde(alias="ends", alias="ENDS")]
    Ends(String),
    #[serde(alias="in", alias="IN")]
    IN(Vec<String>)
}

#[derive(Serialize, Deserialize, Debug)]
enum Date {
    #[serde(alias="eq", alias="EQ")]
    EQ(String),
    #[serde(alias="in", alias="IN")]
    IN(Vec<String>)
}

#[derive(Serialize, Deserialize, Debug)]
enum DayOfMonth {
    #[serde(alias="eq", alias="EQ")]
    EQ(u32),
    #[serde(alias="gt", alias="GT")]
    GT(u32),
    #[serde(alias="lt", alias="LT")]
    LT(u32),
    #[serde(alias="in", alias="IN")]
    IN(Vec<u32>)
}

#[derive(Serialize, Deserialize, Debug)]
enum DayOfWeek {
    #[serde(alias="eq", alias="EQ")]
    EQ(u32),
    #[serde(alias="gt", alias="GT")]
    GT(u32),
    #[serde(alias="lt", alias="LT")]
    LT(u32),
    #[serde(alias="in", alias="IN")]
    IN(Vec<u32>)
}

#[derive(Serialize, Deserialize, Debug)]
enum Month {
    #[serde(alias="eq", alias="EQ")]
    EQ(u32),
    #[serde(alias="gt", alias="GT")]
    GT(u32),
    #[serde(alias="lt", alias="LT")]
    LT(u32),
    #[serde(alias="in", alias="IN")]
    IN(Vec<u32>)
}

#[derive(Serialize, Deserialize, Debug)]
enum RND {
    #[serde(alias="eq", alias="EQ")]
    EQ(u32),
    #[serde(alias="gt", alias="GT")]
    GT(u32),
    #[serde(alias="lt", alias="LT")]
    LT(u32),
    #[serde(alias="in", alias="IN")]
    IN(Vec<u32>)
}
struct ExpressionContext { }

impl ExpressionContext {
    fn get_os(&self) -> Result<String, std::io::Error> {
        // let parser = Woothee::new();
        // let result = parser.parse("com.apple.invitation-registration [Watch OS,10.1.1,21S71,Watch7,4]");
        
        // println!("{:?}", result);

        Ok("Windows".into())
    }
    fn get_ua(&self) -> Result<String, std::io::Error> {

        Ok("Chrome".into())
    }
    fn get_day_of_month(&self) -> Result<u32, std::io::Error> {
        let now = Utc::now();
        Ok(now.day())
    }
    fn get_day_of_week(&self) -> Result<u32, std::io::Error> {
        let now = Utc::now();
        Ok(now.weekday().num_days_from_sunday())
    }
    fn get_month(&self) -> Result<u32, std::io::Error> {
        let now = Utc::now();
        Ok(now.month())
    }
    fn get_date(&self) -> Result<DateTime<Utc>, std::io::Error> {
        let now = Utc::now();
        Ok(now)
    }
}

impl Expression {
    fn eval(&self, context: &ExpressionContext) -> bool {
        let mut result = Vec::new();

        if self.ua.is_some(){
            result.push(self.eval_ua(&context));
        };

        if self.os.is_some(){
            result.push(self.eval_os(&context));
        };

        if self.day_of_month.is_some(){
            result.push(self.eval_day_of_month(&context));
        };

        if self.month.is_some(){
            result.push(self.eval_month(&context));
        };

        if self.day_of_week.is_some(){
            result.push(self.eval_day_of_week(&context));
        };

        if self.or.is_some(){
            result.push(self.eval_or(&context));
        };

        if self.and.is_some(){
            result.push(self.eval_and(&context));
        };


        match &self.default_operator {
            //and by default
            None => result.iter().all(|&i| i),
            Some(op) => {
                match op {
                    DefaultOperator::And => result.iter().all(|&i| i),
                    DefaultOperator::Or => result.iter().any(|&i| i),
                }
            }
        }
    }

    fn eval_ua(&self, context: &ExpressionContext) -> bool {
        let ua_result = context.get_ua();

        match ua_result {
            Ok(ua) => {
                match self.ua.as_ref().unwrap() {
                    UA::EQ(str) => ua.eq_ignore_ascii_case(str),
                    UA::Ends(str) => ua.to_lowercase().ends_with(str),
                    UA::Starts(str) => ua.to_lowercase().starts_with(str),
                    UA::IN(array) => array.iter().any(|i| ua.eq_ignore_ascii_case(&i)),
                }
            },
            Err(_) => false,
        }
    }

    fn eval_os(&self, context: &ExpressionContext) -> bool {

        let os_result = context.get_os();

        match os_result {
            Ok(os) => {
                match self.os.as_ref().unwrap() {
                    OS::EQ(str) => os.eq_ignore_ascii_case(str),
                    OS::Ends(str) => os.to_lowercase().ends_with(str),
                    OS::Starts(str) => os.to_lowercase().starts_with(str),
                    OS::IN(array) => array.iter().any(|i| os.eq_ignore_ascii_case(&i)),
                }
            },
            Err(_) => false,
        }
    }

    // fn eval_date(&self, context: ExpressionContext) -> bool {

    // }

    fn eval_day_of_month(&self, context: &ExpressionContext) -> bool {

        let day_of_month_result = context.get_day_of_month();

        match day_of_month_result {
            Ok(day_of_month) => {
                match self.day_of_month.as_ref().unwrap() {
                    DayOfMonth::EQ(day) => *day == day_of_month,
                    DayOfMonth::GT(day) => *day > day_of_month,
                    DayOfMonth::LT(day) => *day < day_of_month,
                    DayOfMonth::IN(days) => days.iter().any(|&i| day_of_month == i),
                }
            },
            Err(_) => false,
        }
    }

    fn eval_day_of_week(&self, context: &ExpressionContext) -> bool {
        let day_of_week_result = context.get_day_of_week();

        match day_of_week_result {
            Ok(day_of_week) => {
                match &self.day_of_month.as_ref().unwrap() {
                    DayOfMonth::EQ(day) => *day == day_of_week,
                    DayOfMonth::GT(day) => *day > day_of_week,
                    DayOfMonth::LT(day) => *day < day_of_week,
                    DayOfMonth::IN(days) => days.iter().any(|&i| day_of_week == i),
                }
            },
            Err(_) => false,
        }
    }

    fn eval_month(&self, context: &ExpressionContext) -> bool {
        let month_result = context.get_month();
        
        match month_result {
            Ok(month) => {
                match self.month.as_ref().unwrap() {
                    Month::EQ(day) => *day == month,
                    Month::GT(day) => *day > month,
                    Month::LT(day) => *day < month,
                    Month::IN(days) => days.iter().any(|&i| month == i),
                }
            },
            Err(_) => false,
        }
    }

    fn eval_and(&self, context: &ExpressionContext) -> bool {
        self.and.as_ref().unwrap().iter().all(|i| i.eval(&context))
    }

    fn eval_or(&self, context: &ExpressionContext) -> bool {
        self.and.as_ref().unwrap().iter().any(|i| i.eval(&context))
    }
}

fn main() {

    for i in 0..100 {
    let sw = Stopwatch::start_new();// Pay initialisation costs
    let parser = Woothee::new();
    let result = parser.parse("Mozilla/5.0 (iPad; CPU iPhone OS 15_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148 Version/15.0 Safari/605.1.15 AlohaBrowser/3.2.6");

    println!("Thing took {}ms", sw.elapsed_ms());
    }

    let sw = Stopwatch::start_new();// Pay initialisation costs
    fast_uaparser::init().unwrap();
    println!("Thing took {}ms", sw.elapsed_ms());
    let sw = Stopwatch::start_new();
    let ua: UserAgent =
    "Mozilla/5.0 (X11; Linux i686; rv:70.0) Gecko/20100101 Firefox/70.0"
        .parse()
        .unwrap();
    println!("Thing took {}ms", sw.elapsed_ms());











    let expression = Expression {

        ua: Some(UA::IN(vec!["Edge".into(), "Chrome".into(), "Firefox".into()])),
        day_of_month: Some(DayOfMonth::IN(vec![7, 14, 30, 26])),
        and: Some(vec![Box::new(Expression{
            os: Some(OS::EQ("Windows".into())),
            ..Default::default()
        })]),
        ..Default::default()

    };

    let serialized = serde_json::to_string(&expression).unwrap().to_uppercase();

    println!("serialized = {}", serialized);

    let eval = expression.eval(&ExpressionContext{});

    println!("eval = {}", eval);

    let deserialized: Expression = serde_json::from_str(&serialized).unwrap();

    println!("deserialized = {:?}", deserialized);

    let eval = deserialized.eval(&ExpressionContext{});

    println!("eval = {}", eval);
}

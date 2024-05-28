use anyhow::Result;
use chrono::{prelude::*, DateTime, Utc};
use rand::{rngs::ThreadRng, Rng};

use crate::{
    core::{
        base_flow_router::FlowRouterContext,
        base_location_detector::Country,
        base_user_agent_detector::{Device, UserAgent, OS},
        InitOnce,
    },
    flow_router::{
        base_expression_evaluator::BaseExpressionEvaluator, base_language_extractor::Language,
    },
    model::expression::{
        Country as CountryExpr, Date as DateExpr, DayOfMonth, DayOfWeek, DefaultOperator, Device as DeviceExpr, Expression, Lang as LangExpr, OS as OSExpr, RND, UA as UAExpr
    },
};

const DATE_FORMAT: &'static str = "%Y%m%d";

#[derive(Clone)]
pub struct DefaultExpressionEvaluator {
    rng: ThreadRng
}

impl DefaultExpressionEvaluator {
    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng()
        }
    }
    fn eval_country(
        &self,
        client_country: &InitOnce<Option<Country>>,
        country: CountryExpr,
    ) -> bool {
        if let Some(client_country) = &client_country.clone().get_value() {
            let result = match country {
                CountryExpr::EQ(str) => client_country.iso_code.eq_ignore_ascii_case(&str),
                CountryExpr::Ends(str) => client_country.iso_code.to_lowercase().ends_with(&str),
                CountryExpr::Starts(str) => {
                    client_country.iso_code.to_lowercase().starts_with(&str)
                }
                CountryExpr::IN(array) => array
                    .iter()
                    .any(|i| client_country.iso_code.eq_ignore_ascii_case(&i)),
            };

            return result;
        }

        false
    }

    fn eval_lang(&self, client_langs: &Option<Vec<Language>>, lang: LangExpr) -> bool {
        if let Some(client_langs) = &client_langs.clone() {
            if let Some(top_lang) = client_langs.first() {
                let top_lang = &top_lang.name[..2];

                let result = match lang {
                    LangExpr::EQ(str) => top_lang.eq_ignore_ascii_case(&str),
                    LangExpr::IN(array) => array.iter().any(|i| top_lang.eq_ignore_ascii_case(&i)),
                };

                return result;
            }
        }

        false
    }

    fn eval_ua(&self, client_ua: &InitOnce<Option<UserAgent>>, ua: UAExpr) -> bool {
        if let Some(client_ua) = &client_ua.clone().get_value() {
            let result = match ua {
                UAExpr::EQ(str) => client_ua.family.eq_ignore_ascii_case(&str),
                UAExpr::Ends(str) => client_ua.family.to_lowercase().ends_with(&str),
                UAExpr::Starts(str) => client_ua.family.to_lowercase().starts_with(&str),
                UAExpr::IN(array) => array
                    .iter()
                    .any(|i| client_ua.family.eq_ignore_ascii_case(&i)),
            };

            return result;
        }

        false
    }

    fn eval_os(&self, client_os: &InitOnce<Option<OS>>, os: OSExpr) -> bool {
        if let Some(client_os) = &client_os.clone().get_value() {
            let result = match os {
                OSExpr::EQ(str) => client_os.family.eq_ignore_ascii_case(&str),
                OSExpr::Ends(str) => client_os.family.to_lowercase().ends_with(&str),
                OSExpr::Starts(str) => client_os.family.to_lowercase().starts_with(&str),
                OSExpr::IN(array) => array
                    .iter()
                    .any(|i| client_os.family.eq_ignore_ascii_case(&i)),
            };

            return result;
        }

        false
    }

    fn eval_device(&self, client_device: &InitOnce<Option<Device>>, device: DeviceExpr) -> bool {
        if let Some(client_device) = &client_device.clone().get_value() {
            let result = match device {
                DeviceExpr::EQ(str) => client_device.family.eq_ignore_ascii_case(&str),
                DeviceExpr::Ends(str) => client_device.family.to_lowercase().ends_with(&str),
                DeviceExpr::Starts(str) => client_device.family.to_lowercase().starts_with(&str),
                DeviceExpr::IN(array) => array
                    .iter()
                    .any(|i| client_device.family.eq_ignore_ascii_case(&i)),
            };

            return result;
        }

        false
    }

    fn eval_rnd(&self, rng: &mut ThreadRng, rnd: RND) -> bool {
        let rng = rng.gen_range(0..100);

        let result = match rnd {
            RND::EQ(num) => num == rng,
            RND::GT(num) => num > rng,
            RND::LT(num) => num < rng,
            RND::IN(nums) => nums.iter().any(|&i| rng == i),
        };

        return result;
    }

    fn eval_day_of_month(&self, date_time: &DateTime<Utc>, day_of_month: DayOfMonth) -> bool {
        let request_day_of_month = date_time.day();

        let result = match day_of_month {
            DayOfMonth::EQ(day) => day == request_day_of_month,
            DayOfMonth::GT(day) => day > request_day_of_month,
            DayOfMonth::LT(day) => day < request_day_of_month,
            DayOfMonth::IN(days) => days.iter().any(|&i| request_day_of_month == i),
        };

        return result;
    }

    fn eval_day_of_week(&self, date_time: &DateTime<Utc>, day_of_week: DayOfWeek) -> bool {
        let request_day_of_week = date_time.weekday().num_days_from_sunday();

        let result = match day_of_week {
            DayOfWeek::EQ(day) => day == request_day_of_week,
            DayOfWeek::GT(day) => day > request_day_of_week,
            DayOfWeek::LT(day) => day < request_day_of_week,
            DayOfWeek::IN(days) => days.iter().any(|&i| request_day_of_week == i),
        };

        return result;
    }

    fn eval_date(&self, date_time: &DateTime<Utc>, date: DateExpr) -> bool {
        let request_date = date_time.date_naive();

        let result = match date {
            DateExpr::EQ(date) => {
                let parse_result = NaiveDate::parse_from_str(&date, DATE_FORMAT);

                if let Ok(parse_result) = parse_result {
                    return parse_result == request_date;
                }

                false
            }

            DateExpr::GT(date) => {
                let parse_result = NaiveDate::parse_from_str(&date, DATE_FORMAT);

                if let Ok(parse_result) = parse_result {
                    return parse_result >= request_date;
                }

                false
            }

            DateExpr::LT(date) => {
                let parse_result = NaiveDate::parse_from_str(&date, DATE_FORMAT);

                if let Ok(parse_result) = parse_result {
                    return parse_result <= request_date;
                }

                false
            }

            DateExpr::IN(dates) => dates.iter().any(|date| {
                let parse_result = NaiveDate::parse_from_str(&date, DATE_FORMAT);

                if let Ok(parse_result) = parse_result {
                    return parse_result == request_date;
                }

                false
            }),
        };

        return result;
    }

    fn eval_expression(&self, router_context: &FlowRouterContext, expr: &Expression) -> bool {
        let mut result = Vec::new();
/* 
        if let Some(country) = expr.country {
            result.push(self.eval_country(&router_context.client_country, country));
        };

        if let Some(lang) = expr.lang {
            result.push(self.eval_lang(&router_context.client_langs, lang));
        };

        if let Some(ua) = expr.ua {
            result.push(self.eval_ua(&router_context.client_ua, ua));
        };

        if let Some(os) = expr.os {
            result.push(self.eval_os(&router_context.client_os, os));
        };

        if let Some(dev) = expr.device {
            result.push(self.eval_device(&router_context.client_device, dev));
        };

        if let Some(rnd) = expr.rnd {
            result.push(self.eval_rnd(&mut self.rng, rnd));
        };

        if let Some(day) = expr.day_of_month {
            result.push(self.eval_day_of_month(&router_context.utc, day));
        };

        if let Some(day) = expr.day_of_week {
            result.push(self.eval_day_of_week(&router_context.utc, day));
        };

        if let Some(date) = expr.date {
            result.push(self.eval_date(&router_context.utc, date));
        };

        */

        match &expr.default_operator {
            //and by default
            None => result.iter().all(|&i| i),
            Some(op) => match op {
                DefaultOperator::And => result.iter().all(|&i| i),
                DefaultOperator::Or => result.iter().any(|&i| i),
            },
        }
    }
}

impl BaseExpressionEvaluator for DefaultExpressionEvaluator {
    fn eval(&self, router_context: &FlowRouterContext, expr: &Expression) -> Result<bool> {
        Ok(self.eval_expression(router_context, expr))
    }
}

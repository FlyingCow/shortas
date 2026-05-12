//! Message contracts for inter-service communication.
//!
//! This module provides conversions between internal model types and
//! shortas-common types used for communication between services.

use crate::model::route as internal;
use shortas_common as common;

/// Convert internal Route to shortas-common Route (message contract)
impl From<&internal::Route> for common::Route {
    fn from(route: &internal::Route) -> Self {
        common::Route {
            id: uuid::Uuid::new_v4(), // Generate new ID for message
            switch: route.switch.clone(),
            link: route.link.clone(),
            dest: route.dest.clone(),
            dest_format: match route.dest_format {
                internal::DestinationFormat::Http => common::DestinationFormat::Http,
                internal::DestinationFormat::Native => common::DestinationFormat::Native,
            },
            code: route.code,
            ttl: route.ttl,
            status: match &route.status {
                internal::RouteStatus::Active => common::RouteStatus::Active,
                internal::RouteStatus::Blocked(reason) => {
                    common::RouteStatus::Blocked(match reason {
                        internal::BlockedReason::Resoned(msg) => {
                            common::BlockedReason::Reasoned(msg.clone())
                        }
                        internal::BlockedReason::Unknown => common::BlockedReason::Unknown,
                    })
                }
            },
            terminal: match route.terminal {
                internal::RoutingTerminal::External => common::RoutingTerminal::External,
                internal::RoutingTerminal::Internal => common::RoutingTerminal::Internal,
                internal::RoutingTerminal::Middleware => common::RoutingTerminal::Middleware,
            },
            policy: (&route.policy).into(),
            properties: (&route.properties).into(),
            domain_id: None,
        }
    }
}

impl From<internal::Route> for common::Route {
    fn from(route: internal::Route) -> Self {
        (&route).into()
    }
}

/// Convert internal RoutingPolicy to shortas-common RoutingPolicy
impl From<&internal::RoutingPolicy> for common::RoutingPolicy {
    fn from(policy: &internal::RoutingPolicy) -> Self {
        match policy {
            internal::RoutingPolicy::Basic => common::RoutingPolicy::Basic,
            internal::RoutingPolicy::Conditional(conditions) => {
                common::RoutingPolicy::Conditional {
                    conditions: conditions.iter().map(|c| c.into()).collect(),
                }
            }
            internal::RoutingPolicy::Challenge(challenge) => common::RoutingPolicy::Challenge {
                challenge: Some(common::ChallengeRouting {
                    key: challenge.key.clone(),
                    source: challenge.source.clone(),
                    challenge_type: challenge.challenge_type.clone(),
                }),
            },
            internal::RoutingPolicy::File(file) => common::RoutingPolicy::File {
                file: Some(common::FileRouting {
                    content_type: file.content_type.clone(),
                }),
            },
            internal::RoutingPolicy::Mirroring => common::RoutingPolicy::Mirroring,
            internal::RoutingPolicy::Unknown => common::RoutingPolicy::Unknown,
        }
    }
}

/// Convert internal ConditionalRouting to shortas-common ConditionalRouting
impl From<&internal::ConditionalRouting> for common::ConditionalRouting {
    fn from(cond: &internal::ConditionalRouting) -> Self {
        common::ConditionalRouting {
            key: cond.key.clone(),
            condition: (&cond.condition).into(),
            dest: cond.dest.clone(),
        }
    }
}

/// Convert internal Condition to shortas-common Condition
impl From<&crate::model::condition::Condition> for common::Condition {
    fn from(cond: &crate::model::condition::Condition) -> Self {
        common::Condition {
            default_operator: cond.default_operator.as_ref().map(|op| match op {
                crate::model::condition::DefaultOperator::And => common::DefaultOperator::And,
                crate::model::condition::DefaultOperator::Or => common::DefaultOperator::Or,
            }),
            ua: cond.ua.as_ref().map(|v| convert_string_condition(v)),
            os: cond.os.as_ref().map(|v| convert_string_condition(v)),
            device: None, // Internal model doesn't have device
            lang: None,   // Internal model doesn't have lang
            country: None, // Internal model doesn't have country
            date: cond.date.as_ref().map(|v| convert_date_condition(v)),
            rnd: cond.rnd.as_ref().map(|v| convert_numeric_condition(v)),
            day_of_week: cond.day_of_week.as_ref().map(|v| convert_numeric_condition(v)),
            day_of_month: cond.day_of_month.as_ref().map(|v| convert_numeric_condition(v)),
            month: cond.month.as_ref().map(|v| convert_numeric_condition(v)),
            and: cond.and.as_ref().map(|v| v.iter().map(|c| Box::new(c.as_ref().into())).collect()),
            or: cond.or.as_ref().map(|v| v.iter().map(|c| Box::new(c.as_ref().into())).collect()),
        }
    }
}

fn convert_string_condition<T: StringConditionLike>(cond: &T) -> common::StringCondition {
    cond.to_common()
}

fn convert_date_condition(cond: &crate::model::condition::Date) -> common::StringCondition {
    match cond {
        crate::model::condition::Date::EQ(v) => common::StringCondition::Eq(v.clone()),
        crate::model::condition::Date::IN(v) => common::StringCondition::In(v.clone()),
    }
}

fn convert_numeric_condition<T: NumericConditionLike>(cond: &T) -> common::NumericCondition {
    cond.to_common()
}

trait StringConditionLike {
    fn to_common(&self) -> common::StringCondition;
}

impl StringConditionLike for crate::model::condition::UA {
    fn to_common(&self) -> common::StringCondition {
        match self {
            crate::model::condition::UA::EQ(v) => common::StringCondition::Eq(v.clone()),
            crate::model::condition::UA::Starts(v) => common::StringCondition::Starts(v.clone()),
            crate::model::condition::UA::Ends(v) => common::StringCondition::Ends(v.clone()),
            crate::model::condition::UA::IN(v) => common::StringCondition::In(v.clone()),
        }
    }
}

impl StringConditionLike for crate::model::condition::OS {
    fn to_common(&self) -> common::StringCondition {
        match self {
            crate::model::condition::OS::EQ(v) => common::StringCondition::Eq(v.clone()),
            crate::model::condition::OS::Starts(v) => common::StringCondition::Starts(v.clone()),
            crate::model::condition::OS::Ends(v) => common::StringCondition::Ends(v.clone()),
            crate::model::condition::OS::IN(v) => common::StringCondition::In(v.clone()),
        }
    }
}

trait NumericConditionLike {
    fn to_common(&self) -> common::NumericCondition;
}

impl NumericConditionLike for crate::model::condition::RND {
    fn to_common(&self) -> common::NumericCondition {
        match self {
            crate::model::condition::RND::EQ(v) => common::NumericCondition::Eq(*v as i32),
            crate::model::condition::RND::GT(v) => common::NumericCondition::Gt(*v as i32),
            crate::model::condition::RND::LT(v) => common::NumericCondition::Lt(*v as i32),
            crate::model::condition::RND::IN(v) => {
                common::NumericCondition::In(v.iter().map(|x| *x as i32).collect())
            }
        }
    }
}

impl NumericConditionLike for crate::model::condition::DayOfWeek {
    fn to_common(&self) -> common::NumericCondition {
        match self {
            crate::model::condition::DayOfWeek::EQ(v) => common::NumericCondition::Eq(*v as i32),
            crate::model::condition::DayOfWeek::GT(v) => common::NumericCondition::Gt(*v as i32),
            crate::model::condition::DayOfWeek::LT(v) => common::NumericCondition::Lt(*v as i32),
            crate::model::condition::DayOfWeek::IN(v) => {
                common::NumericCondition::In(v.iter().map(|x| *x as i32).collect())
            }
        }
    }
}

impl NumericConditionLike for crate::model::condition::DayOfMonth {
    fn to_common(&self) -> common::NumericCondition {
        match self {
            crate::model::condition::DayOfMonth::EQ(v) => common::NumericCondition::Eq(*v as i32),
            crate::model::condition::DayOfMonth::GT(v) => common::NumericCondition::Gt(*v as i32),
            crate::model::condition::DayOfMonth::LT(v) => common::NumericCondition::Lt(*v as i32),
            crate::model::condition::DayOfMonth::IN(v) => {
                common::NumericCondition::In(v.iter().map(|x| *x as i32).collect())
            }
        }
    }
}

impl NumericConditionLike for crate::model::condition::Month {
    fn to_common(&self) -> common::NumericCondition {
        match self {
            crate::model::condition::Month::EQ(v) => common::NumericCondition::Eq(*v as i32),
            crate::model::condition::Month::GT(v) => common::NumericCondition::Gt(*v as i32),
            crate::model::condition::Month::LT(v) => common::NumericCondition::Lt(*v as i32),
            crate::model::condition::Month::IN(v) => {
                common::NumericCondition::In(v.iter().map(|x| *x as i32).collect())
            }
        }
    }
}

/// Convert internal RouteProperties to shortas-common RouteProperties
impl From<&internal::RouteProperties> for common::RouteProperties {
    fn from(props: &internal::RouteProperties) -> Self {
        common::RouteProperties {
            route_id: props.route_id.clone(),
            domain_id: props.domain_id.clone(),
            owner_id: props.owner_id.clone(),
            creator_id: props.creator_id.clone(),
            workspace_id: props.workspace_id.clone(),
            scripts: props.scripts.clone(),
            tags: props.tags.clone(),
            custom: props.custom.clone(),
            native: props.native.clone(),
            bundling: props.bundling.clone(),
            qr_settings: None,
            opengraph: props.opengraph,
            allow_debug: props.allow_debug,
        }
    }
}

// === Reverse conversions (shortas-common -> internal) ===

/// Convert shortas-common Route to internal Route
impl From<&common::Route> for internal::Route {
    fn from(route: &common::Route) -> Self {
        internal::Route {
            switch: route.switch.clone(),
            link: route.link.clone(),
            dest: route.dest.clone(),
            dest_format: match route.dest_format {
                common::DestinationFormat::Http => internal::DestinationFormat::Http,
                common::DestinationFormat::Native => internal::DestinationFormat::Native,
            },
            code: route.code,
            ttl: route.ttl,
            status: match &route.status {
                common::RouteStatus::Active => internal::RouteStatus::Active,
                common::RouteStatus::Blocked(reason) => {
                    internal::RouteStatus::Blocked(match reason {
                        common::BlockedReason::Reasoned(msg) => {
                            internal::BlockedReason::Resoned(msg.clone())
                        }
                        common::BlockedReason::Unknown => internal::BlockedReason::Unknown,
                    })
                }
            },
            terminal: match route.terminal {
                common::RoutingTerminal::External => internal::RoutingTerminal::External,
                common::RoutingTerminal::Internal => internal::RoutingTerminal::Internal,
                common::RoutingTerminal::Middleware => internal::RoutingTerminal::Middleware,
            },
            policy: (&route.policy).into(),
            properties: (&route.properties).into(),
        }
    }
}

impl From<common::Route> for internal::Route {
    fn from(route: common::Route) -> Self {
        (&route).into()
    }
}

/// Convert shortas-common RoutingPolicy to internal RoutingPolicy
impl From<&common::RoutingPolicy> for internal::RoutingPolicy {
    fn from(policy: &common::RoutingPolicy) -> Self {
        match policy {
            common::RoutingPolicy::Basic => internal::RoutingPolicy::Basic,
            common::RoutingPolicy::Conditional { conditions } => {
                internal::RoutingPolicy::Conditional(
                    conditions.iter().map(|c| c.into()).collect(),
                )
            }
            common::RoutingPolicy::Challenge { challenge } => {
                if let Some(ch) = challenge {
                    internal::RoutingPolicy::Challenge(internal::ChallengeRouting {
                        key: ch.key.clone(),
                        source: ch.source.clone(),
                        challenge_type: ch.challenge_type.clone(),
                    })
                } else {
                    internal::RoutingPolicy::Unknown
                }
            }
            common::RoutingPolicy::File { file } => {
                if let Some(f) = file {
                    internal::RoutingPolicy::File(internal::FileRouting {
                        content_type: f.content_type.clone(),
                    })
                } else {
                    internal::RoutingPolicy::Unknown
                }
            }
            common::RoutingPolicy::Mirroring => internal::RoutingPolicy::Mirroring,
            common::RoutingPolicy::Unknown => internal::RoutingPolicy::Unknown,
        }
    }
}

/// Convert shortas-common ConditionalRouting to internal ConditionalRouting
impl From<&common::ConditionalRouting> for internal::ConditionalRouting {
    fn from(cond: &common::ConditionalRouting) -> Self {
        internal::ConditionalRouting {
            key: cond.key.clone(),
            condition: (&cond.condition).into(),
            dest: cond.dest.clone(),
        }
    }
}

/// Convert shortas-common Condition to internal Condition
impl From<&common::Condition> for crate::model::condition::Condition {
    fn from(cond: &common::Condition) -> Self {
        crate::model::condition::Condition {
            default_operator: cond.default_operator.as_ref().map(|op| match op {
                common::DefaultOperator::And => crate::model::condition::DefaultOperator::And,
                common::DefaultOperator::Or => crate::model::condition::DefaultOperator::Or,
            }),
            ua: cond.ua.as_ref().map(|v| convert_to_internal_ua(v)),
            os: cond.os.as_ref().map(|v| convert_to_internal_os(v)),
            date: cond.date.as_ref().map(|v| convert_to_internal_date(v)),
            rnd: cond.rnd.as_ref().map(|v| convert_to_internal_rnd(v)),
            day_of_week: cond.day_of_week.as_ref().map(|v| convert_to_internal_day_of_week(v)),
            day_of_month: cond.day_of_month.as_ref().map(|v| convert_to_internal_day_of_month(v)),
            month: cond.month.as_ref().map(|v| convert_to_internal_month(v)),
            and: cond.and.as_ref().map(|v| v.iter().map(|c| Box::new(c.as_ref().into())).collect()),
            or: cond.or.as_ref().map(|v| v.iter().map(|c| Box::new(c.as_ref().into())).collect()),
        }
    }
}

fn convert_to_internal_ua(cond: &common::StringCondition) -> crate::model::condition::UA {
    match cond {
        common::StringCondition::Eq(v) => crate::model::condition::UA::EQ(v.clone()),
        common::StringCondition::Starts(v) => crate::model::condition::UA::Starts(v.clone()),
        common::StringCondition::Ends(v) => crate::model::condition::UA::Ends(v.clone()),
        common::StringCondition::In(v) => crate::model::condition::UA::IN(v.clone()),
    }
}

fn convert_to_internal_os(cond: &common::StringCondition) -> crate::model::condition::OS {
    match cond {
        common::StringCondition::Eq(v) => crate::model::condition::OS::EQ(v.clone()),
        common::StringCondition::Starts(v) => crate::model::condition::OS::Starts(v.clone()),
        common::StringCondition::Ends(v) => crate::model::condition::OS::Ends(v.clone()),
        common::StringCondition::In(v) => crate::model::condition::OS::IN(v.clone()),
    }
}

fn convert_to_internal_date(cond: &common::StringCondition) -> crate::model::condition::Date {
    match cond {
        common::StringCondition::Eq(v) => crate::model::condition::Date::EQ(v.clone()),
        common::StringCondition::In(v) => crate::model::condition::Date::IN(v.clone()),
        // Date doesn't support Starts/Ends, fallback to Eq
        common::StringCondition::Starts(v) => crate::model::condition::Date::EQ(v.clone()),
        common::StringCondition::Ends(v) => crate::model::condition::Date::EQ(v.clone()),
    }
}

fn convert_to_internal_rnd(cond: &common::NumericCondition) -> crate::model::condition::RND {
    match cond {
        common::NumericCondition::Eq(v) => crate::model::condition::RND::EQ(*v as u32),
        common::NumericCondition::Gt(v) => crate::model::condition::RND::GT(*v as u32),
        common::NumericCondition::Lt(v) => crate::model::condition::RND::LT(*v as u32),
        common::NumericCondition::In(v) => {
            crate::model::condition::RND::IN(v.iter().map(|x| *x as u32).collect())
        }
    }
}

fn convert_to_internal_day_of_week(cond: &common::NumericCondition) -> crate::model::condition::DayOfWeek {
    match cond {
        common::NumericCondition::Eq(v) => crate::model::condition::DayOfWeek::EQ(*v as u32),
        common::NumericCondition::Gt(v) => crate::model::condition::DayOfWeek::GT(*v as u32),
        common::NumericCondition::Lt(v) => crate::model::condition::DayOfWeek::LT(*v as u32),
        common::NumericCondition::In(v) => {
            crate::model::condition::DayOfWeek::IN(v.iter().map(|x| *x as u32).collect())
        }
    }
}

fn convert_to_internal_day_of_month(cond: &common::NumericCondition) -> crate::model::condition::DayOfMonth {
    match cond {
        common::NumericCondition::Eq(v) => crate::model::condition::DayOfMonth::EQ(*v as u32),
        common::NumericCondition::Gt(v) => crate::model::condition::DayOfMonth::GT(*v as u32),
        common::NumericCondition::Lt(v) => crate::model::condition::DayOfMonth::LT(*v as u32),
        common::NumericCondition::In(v) => {
            crate::model::condition::DayOfMonth::IN(v.iter().map(|x| *x as u32).collect())
        }
    }
}

fn convert_to_internal_month(cond: &common::NumericCondition) -> crate::model::condition::Month {
    match cond {
        common::NumericCondition::Eq(v) => crate::model::condition::Month::EQ(*v as u32),
        common::NumericCondition::Gt(v) => crate::model::condition::Month::GT(*v as u32),
        common::NumericCondition::Lt(v) => crate::model::condition::Month::LT(*v as u32),
        common::NumericCondition::In(v) => {
            crate::model::condition::Month::IN(v.iter().map(|x| *x as u32).collect())
        }
    }
}

/// Convert shortas-common RouteProperties to internal RouteProperties
impl From<&common::RouteProperties> for internal::RouteProperties {
    fn from(props: &common::RouteProperties) -> Self {
        internal::RouteProperties {
            route_id: props.route_id.clone(),
            domain_id: props.domain_id.clone(),
            owner_id: props.owner_id.clone(),
            creator_id: props.creator_id.clone(),
            workspace_id: props.workspace_id.clone(),
            scripts: props.scripts.clone(),
            tags: props.tags.clone(),
            custom: props.custom.clone(),
            native: props.native.clone(),
            bundling: props.bundling.clone(),
            opengraph: props.opengraph,
            allow_debug: props.allow_debug,
        }
    }
}

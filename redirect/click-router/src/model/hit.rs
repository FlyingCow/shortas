use std::net::IpAddr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::Route;
use crate::core::trace::HitTrace;
use crate::core::{ConversionEvent, ConversionFunnelStep};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Click<'a> {
    pub dest: Option<&'a str>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Event<'a> {
    pub click: &'a str,
}

#[derive(Clone, Debug, Serialize)]
pub enum HitData<'a> {
    Click(&'a Click<'a>),
    Event(&'a Event<'a>),
    Conversion(ConversionEvent),
    FunnelStep(ConversionFunnelStep),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HitRoute {
    pub id: Option<String>,
    pub owner_id: Option<String>,
    pub creator_id: Option<String>,
    pub workspace_id: Option<String>,
}

impl HitRoute {
    pub fn from_route(route: &Option<std::sync::Arc<Route>>) -> Option<Self> {
        if route.is_none() {
            return None;
        }

        let route = route.as_ref().unwrap();

        Some(Self {
            id: route.properties.route_id.clone(),
            owner_id: route.properties.owner_id.clone(),
            creator_id: route.properties.creator_id.clone(),
            workspace_id: route.properties.workspace_id.clone(),
        })
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Hit<'a> {
    pub id: &'a str,
    pub data: HitData<'a>,
    pub route: Option<HitRoute>,
    pub user_agent: Option<&'a str>,
    pub ip: Option<IpAddr>,
    pub utc: DateTime<Utc>,
    /// Optional trace data for debug routes (only present when allow_debug=true)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace: Option<HitTrace>,
}

impl<'a> Click<'a> {
    pub fn new(dest: &'a str) -> Self {
        Click { dest: Some(dest) }
    }
}

impl<'a> Event<'a> {
    pub fn new(click: &'a str) -> Self {
        Event { click }
    }
}

impl<'a> Hit<'a> {
    pub fn click(
        id: &'a str,
        utc: DateTime<Utc>,
        user_agent: Option<&'a str>,
        ip: Option<IpAddr>,
        click: &'a Click,
        route: Option<HitRoute>,
    ) -> Self {
        Self {
            id,
            utc,
            user_agent,
            ip,
            route,
            data: HitData::Click(&click),
            trace: None,
        }
    }

    pub fn click_with_trace(
        id: &'a str,
        utc: DateTime<Utc>,
        user_agent: Option<&'a str>,
        ip: Option<IpAddr>,
        click: &'a Click,
        route: Option<HitRoute>,
        trace: Option<HitTrace>,
    ) -> Self {
        Self {
            id,
            utc,
            user_agent,
            ip,
            route,
            data: HitData::Click(&click),
            trace,
        }
    }

    pub fn event(
        id: &'a str,
        utc: DateTime<Utc>,
        user_agent: Option<&'a str>,
        ip: Option<IpAddr>,
        event: &'a Event,
        route: Option<HitRoute>,
    ) -> Self {
        Self {
            id,
            utc,
            user_agent,
            ip,
            route,
            data: HitData::Event(&event),
            trace: None,
        }
    }

    pub fn conversion(
        id: &'a str,
        utc: DateTime<Utc>,
        user_agent: Option<&'a str>,
        ip: Option<IpAddr>,
        conversion: ConversionEvent,
        route: Option<HitRoute>,
    ) -> Self {
        Self {
            id,
            utc,
            user_agent,
            ip,
            route,
            data: HitData::Conversion(conversion),
            trace: None,
        }
    }

    pub fn funnel_step(
        id: &'a str,
        utc: DateTime<Utc>,
        user_agent: Option<&'a str>,
        ip: Option<IpAddr>,
        funnel_step: ConversionFunnelStep,
        route: Option<HitRoute>,
    ) -> Self {
        Self {
            id,
            utc,
            user_agent,
            ip,
            route,
            data: HitData::FunnelStep(funnel_step),
            trace: None,
        }
    }
}

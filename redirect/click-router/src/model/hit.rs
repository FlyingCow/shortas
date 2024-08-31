use std::net::IpAddr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::Route;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Click {
    pub dest: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    pub click: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum HitData {
    Click(Click),
    Event(Event),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HitRoute {
    pub id: Option<String>,
    pub owner_id: Option<String>,
    pub creator_id: Option<String>,
    pub workspace_id: Option<String>,
}

impl HitRoute {
    pub fn from_route(route: &Option<Route>) -> Option<Self> {
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Hit {
    pub id: String,
    pub data: HitData,
    pub route: Option<HitRoute>,
    pub user_agent: Option<String>,
    pub ip: Option<IpAddr>,
    pub utc: DateTime<Utc>,
}

impl Click {
    pub fn new(dest: String) -> Self {
        Click { dest: Some(dest) }
    }
}

impl Event {
    pub fn new(click: String) -> Self {
        Event { click }
    }
}

impl Hit {
    pub fn click(
        id: String,
        utc: DateTime<Utc>,
        user_agent: Option<String>,
        ip: Option<IpAddr>,
        click: Click,
        route: Option<HitRoute>,
    ) -> Self {
        Self {
            id,
            utc,
            user_agent,
            ip,
            route,
            data: HitData::Click(click),
        }
    }

    pub fn event(
        id: String,
        utc: DateTime<Utc>,
        user_agent: Option<String>,
        ip: Option<IpAddr>,
        event: Event,
        route: Option<HitRoute>,
    ) -> Self {
        Self {
            id,
            utc,
            user_agent,
            ip,
            route,
            data: HitData::Event(event),
        }
    }
}

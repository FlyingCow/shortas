use std::net::IpAddr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};


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
pub struct Hit {
    pub id: String,
    pub data: HitData,
    pub user_agent: Option<String>,
    pub ip: Option<IpAddr>,
    pub utc: DateTime<Utc>,
}

impl Click {
    pub fn new(dest: String) -> Self {
        Click {
            dest: Some(dest)
        }
    }
}

impl Event {
    pub fn new(click: String) -> Self {
        Event {
            click
        }
    }
}

impl Hit {
    pub fn click(
        id: String,
        utc: DateTime<Utc>,
        user_agent: Option<String>,
        ip: Option<IpAddr>,
        click: Click
    ) -> Self {
        Self {
            id,
            utc,
            user_agent,
            ip,
            data: HitData::Click(click),
        }
    }

    pub fn event(
        id: String,
        utc: DateTime<Utc>,
        user_agent: Option<String>,
        ip: Option<IpAddr>,
        event: Event
    ) -> Self {
        Self {
            id,
            utc,
            user_agent,
            ip,
            data: HitData::Event(event),
        }
    }
}

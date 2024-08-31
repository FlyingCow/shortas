use std::{net::IpAddr, sync::Arc};

use redis::Commands;
use redis::cluster::ClusterClient;
use tracing::info;

use crate::core::session_detect::{BaseSessionDetector, Session};

#[derive(Clone)]
pub struct RedisSessionDetector {
    redis_client: ClusterClient
}

impl RedisSessionDetector {
    pub fn new(initial_nodes: Vec<&str>) -> Self {
        info!("  redis -> {}", "");

        let client = ClusterClient::new(initial_nodes).unwrap();

        Self {
            redis_client: client,
        }
    }
}

impl BaseSessionDetector for RedisSessionDetector {
    fn detect_session(&self, route_id: &str, ip_addr: &IpAddr) -> Option<Session> {

        None
    }
}

use std::{borrow::Cow, net::IpAddr};

use dyn_clone::{clone_trait_object, DynClone};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct Country{
    pub iso_code: String, 
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct Location {
    pub country: Option<Country>
}

impl Default for Location {
    fn default() -> Self {
        Self {
            country: None,
        }
    }
}

pub trait BaseLocationDetector: DynClone {
    fn detect_country(&self, ip_addr: &IpAddr) -> Option<Country>;
}

clone_trait_object!(BaseLocationDetector);
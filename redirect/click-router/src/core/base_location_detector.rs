use std::{borrow::Cow, net::IpAddr};

use dyn_clone::{clone_trait_object, DynClone};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct Country<'a>{
    pub name: Cow<'a, str>,
    pub iso_code: Cow<'a, str>, 
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct Location<'a> {
    pub country: Option<Country<'a>>
}

impl<'a> Default for Location<'a> {
    fn default() -> Self {
        Self {
            country: None,
        }
    }
}

pub trait BaseLocationDetector: DynClone {
    fn detect_location<'a>(&self, ip_addr: &'a IpAddr) -> Location<'a>;
}

clone_trait_object!(BaseLocationDetector);
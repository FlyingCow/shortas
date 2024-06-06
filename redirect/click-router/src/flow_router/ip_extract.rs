use std::net::IpAddr;

use dyn_clone::{clone_trait_object, DynClone};

use crate::core::flow_router::RequestData;

#[derive(Clone, Debug)]
pub struct IPInfo {
    pub address: IpAddr,
}

pub trait BaseIPExtractor: DynClone {
    fn detect(&self, context: &RequestData) -> Option<IPInfo>;
}

clone_trait_object!(BaseIPExtractor);

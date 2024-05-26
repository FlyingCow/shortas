use std::net::IpAddr;

use dyn_clone::{clone_trait_object, DynClone};

use crate::core::base_flow_router::FlowRouterContext;

#[derive(Clone, Debug)]
pub struct IPInfo {
    pub address: IpAddr,
}

pub trait BaseIPExtractor: DynClone {
    fn detect(&self, context: &FlowRouterContext) -> Option<IPInfo>;
}

clone_trait_object!(BaseIPExtractor);

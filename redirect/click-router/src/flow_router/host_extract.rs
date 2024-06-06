use dyn_clone::{clone_trait_object, DynClone};

use crate::core::flow_router::RequestData;


#[derive(Clone, Debug)]
pub struct HostInfo{
    pub host: String,
    pub port: u16
}

pub trait BaseHostExtractor: DynClone{
    fn detect(&self, request: &RequestData) -> Option<HostInfo>;
}

clone_trait_object!(BaseHostExtractor);
use crate::core::flow_router::RequestData;
use dyn_clone::{clone_trait_object, DynClone};

#[derive(Clone, Debug)]
pub struct ProtoInfo {
    pub proto: String,
    pub ssl_on: bool,
}

pub trait BaseProtocolExtractor: DynClone {
    fn detect(&self, request: &RequestData, debug: bool) -> Option<ProtoInfo>;
}

clone_trait_object!(BaseProtocolExtractor);

use dyn_clone::{clone_trait_object, DynClone};

use crate::core::flow_router::RequestData;


pub trait BaseUserAgentStringExtractor: DynClone{
    fn detect(&self, request: &RequestData, debug: bool) -> Option<String>;
}

clone_trait_object!(BaseUserAgentStringExtractor);
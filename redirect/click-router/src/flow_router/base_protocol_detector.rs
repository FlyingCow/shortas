use dyn_clone::{clone_trait_object, DynClone};
use http::Request;


#[derive(Clone, Debug)]
pub struct ProtoInfo{
    pub proto: String,
    pub ssl_on: bool
}

pub trait BaseProtocolDetector: DynClone{
    fn detect(&self, request: &Request<()>) -> Option<ProtoInfo>;
}

clone_trait_object!(BaseProtocolDetector);
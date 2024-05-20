use dyn_clone::{clone_trait_object, DynClone};
use http::Request;


#[derive(Clone, Debug)]
pub struct HostInfo{
    pub host: String,
    pub port: u16
}

pub trait BaseHostDetector: DynClone{
    fn detect(&self, request: &Request<()>) -> Option<HostInfo>;
}

clone_trait_object!(BaseHostDetector);
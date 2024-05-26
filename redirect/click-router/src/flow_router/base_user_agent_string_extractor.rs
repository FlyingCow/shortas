use dyn_clone::{clone_trait_object, DynClone};
use http::Request;


pub trait BaseUserAgentStringExtractor: DynClone{
    fn detect(&self, request: &Request<()>) -> Option<String>;
}

clone_trait_object!(BaseUserAgentStringExtractor);
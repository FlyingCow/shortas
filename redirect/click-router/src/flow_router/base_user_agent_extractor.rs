use dyn_clone::{clone_trait_object, DynClone};
use http::Request;


pub trait BaseUserAgentExtractor: DynClone{
    fn detect(&self, request: &Request<()>) -> Option<String>;
}

clone_trait_object!(BaseUserAgentExtractor);
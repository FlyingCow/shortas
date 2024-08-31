use dyn_clone::{clone_trait_object, DynClone};
use crate::core::flow_router::RequestData;

#[derive(Clone, Debug)]
pub struct Language {
    pub name: String,
    pub quality: f32,
}

impl Language{
    pub fn new(lang: String, q: f32) -> Self{
        Self { name: lang, quality: q }
    } 
}

pub trait BaseLanguageExtractor: DynClone{
    fn detect(&self, request: &RequestData, debug: bool) -> Option<Vec<Language>>;
}

clone_trait_object!(BaseLanguageExtractor);
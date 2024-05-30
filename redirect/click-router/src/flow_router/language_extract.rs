use dyn_clone::{clone_trait_object, DynClone};
use http::Request;

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
    fn detect(&self, request: &Request<()>) -> Option<Vec<Language>>;
}

clone_trait_object!(BaseLanguageExtractor);
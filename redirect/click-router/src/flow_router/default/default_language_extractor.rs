use crate::flow_router::base_language_extractor::{BaseLanguageExtractor, Language};
use accept_language::parse_with_quality;
use http::Request;

static ACCEPT_LANGUAGE_HEADER: &str = "Accept-Language";

#[derive(Clone)]
pub struct DefaultLanguageExtractor {}

impl DefaultLanguageExtractor {
    pub fn new() -> Self {
        Self {}
    }
}

fn detect_from_headers(request: &http::Request<()>) -> Option<Vec<Language>> {
    if let Some(accept_language_header) = *&request.headers().get(ACCEPT_LANGUAGE_HEADER) {
        let languages = accept_language_header.to_str();

        if languages.is_err() {
            return None;
        }

        let languages = parse_with_quality(languages.unwrap());

        let languages = languages
            .iter()
            .map(|f| Language::new(f.0.to_string(), f.1))
            .collect();

        return Some(languages);
    }

    None
}

impl BaseLanguageExtractor for DefaultLanguageExtractor {
    fn detect(&self, request: &Request<()>) -> Option<Vec<Language>> {
        let header = detect_from_headers(&request);

        if header.is_none() {
            return None;
        }

        header
    }
}

use crate::{
    core::flow_router::RequestData,
    flow_router::language_extract::{BaseLanguageExtractor, Language},
};
use accept_language::parse_with_quality;

static ACCEPT_LANGUAGE_HEADER: &str = "Accept-Language";

#[derive(Clone)]
pub struct DefaultLanguageExtractor {}

impl DefaultLanguageExtractor {
    pub fn new() -> Self {
        Self {}
    }
}

fn detect_from_headers(request: &RequestData) -> Option<Vec<Language>> {
    if let Some(accept_language_header) = *&request.headers.get(ACCEPT_LANGUAGE_HEADER) {
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
    fn detect(&self, request: &RequestData) -> Option<Vec<Language>> {
        let header = detect_from_headers(&request);

        if header.is_none() {
            return None;
        }

        header
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_extract_from_accept_language_header_when_present() {
        let mut request = RequestData {
            ..Default::default()
        };

        request
            .headers
            .insert(ACCEPT_LANGUAGE_HEADER, "en-US,en;q=0.5".parse().unwrap());

        let result = DefaultLanguageExtractor::new().detect(&request);

        assert!(result.is_some());
        let languages = result.unwrap();
        assert_eq!(languages.len(), 2);
        assert_eq!(languages[0].name, "en-US");
        assert_eq!(languages[0].quality, 1.0);

        assert_eq!(languages[1].name, "en");
        assert_eq!(languages[1].quality, 0.5);
    }
}

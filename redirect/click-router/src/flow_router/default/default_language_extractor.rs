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


#[cfg(test)]
mod tests {
    use http::Request;

    use super::*;

    #[test]
    fn should_extract_from_accept_language_header_when_present() {
        let mut builder = Request::builder();

        builder = builder.header(ACCEPT_LANGUAGE_HEADER, "en-US,en;q=0.5");

        let result = DefaultLanguageExtractor::new().detect(&builder.body(()).unwrap());

        assert!(result.is_some());
        let languages = result.unwrap();
        assert_eq!(languages.len(), 2);
        assert_eq!(languages[0].name, "en-US");
        assert_eq!(languages[0].quality, 1.0);

        assert_eq!(languages[1].name, "en");
        assert_eq!(languages[1].quality, 0.5);
    }
}
use crate::adapters::RequestType;

use accept_language::parse_with_quality;

use super::flow_router::Request;

#[derive(Clone, Debug)]
pub struct Language {
    pub name: String,
    pub quality: f32,
}

impl Language {
    pub fn new(lang: String, q: f32) -> Self {
        Self {
            name: lang,
            quality: q,
        }
    }
}

const DEBUG_LANGS_PARAM: &'static str = "x_debug_langs";
const ACCEPT_LANGUAGE_HEADER: &str = "Accept-Language";

#[derive(Clone)]
pub struct LanguageExtractor {}

impl LanguageExtractor {
    pub fn new() -> Self {
        Self {}
    }
}

fn get_debug(request: &RequestType) -> Option<String> {
    let queries = request.get_queries();

    let param_value = queries.get(DEBUG_LANGS_PARAM);

    if param_value.is_some() {
        return param_value.cloned();
    }

    let header_value = request.get_headers().get(DEBUG_LANGS_PARAM).cloned();

    if let Some(header) = header_value {
        return Some(header.to_str().unwrap_or_default().to_string());
    }

    None
}

fn detect_from_headers(request: &RequestType) -> Option<Vec<Language>> {
    if let Some(accept_language_header) = *&request.get_headers().get(ACCEPT_LANGUAGE_HEADER) {
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

impl LanguageExtractor {
    pub fn detect(&self, request: &RequestType, debug: bool) -> Option<Vec<Language>> {
        if debug {
            if let Some(debug_lngs) = get_debug(&request) {
                let debug_lngs = parse_with_quality(debug_lngs.as_str());

                let languages = debug_lngs
                    .iter()
                    .map(|f| Language::new(f.0.to_string(), f.1))
                    .collect();

                return Some(languages);
            }
        }

        let header = detect_from_headers(request);

        if header.is_none() {
            return None;
        }

        header
    }
}

#[cfg(test)]
mod tests {

    use crate::core::flow_router::RequestData;

    use super::*;

    #[test]
    fn should_extract_from_accept_language_header_when_present() {
        let mut request_data = RequestData {
            ..Default::default()
        };

        request_data
            .headers
            .insert(ACCEPT_LANGUAGE_HEADER, "en-US,en;q=0.5".parse().unwrap());

        let request = RequestType::Test(request_data);

        let result = LanguageExtractor::new().detect(&request, false);

        assert!(result.is_some());
        let languages = result.unwrap();
        assert_eq!(languages.len(), 2);
        assert_eq!(languages[0].name, "en-US");
        assert_eq!(languages[0].quality, 1.0);

        assert_eq!(languages[1].name, "en");
        assert_eq!(languages[1].quality, 0.5);
    }

    #[test]
    fn should_extract_from_debug_language_header_when_present() {
        let mut request_data = RequestData {
            ..Default::default()
        };

        request_data
            .headers
            .insert(ACCEPT_LANGUAGE_HEADER, "en-US,en;q=0.5".parse().unwrap());

        request_data
            .headers
            .insert(DEBUG_LANGS_PARAM, "en-UK,en;q=0.5".parse().unwrap());

        let request = RequestType::Test(request_data);

        let result = LanguageExtractor::new().detect(&request, true);

        assert!(result.is_some());
        let languages = result.unwrap();
        assert_eq!(languages.len(), 2);
        assert_eq!(languages[0].name, "en-UK");
        assert_eq!(languages[0].quality, 1.0);

        assert_eq!(languages[1].name, "en");
        assert_eq!(languages[1].quality, 0.5);
    }
}

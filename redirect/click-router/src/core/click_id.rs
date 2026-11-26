//! Adds the click ID to destinations, so sites can report conversions.

use url::{form_urlencoded, Url};

pub const CLICK_ID_PARAM: &str = "shortas_cid";

/// Returns `destination` with the click ID as its only `shortas_cid`, or `None`
/// when it can't be added safely. Everything else stays exactly as written:
/// re-encoding could break a destination that checks its own URL.
pub fn add_click_id(destination: &str, click_id: &str) -> Option<String> {
    // Parsers strip or rewrite these, so the result could mean something else.
    if destination
        .chars()
        .any(|c| c.is_ascii_control() || c.is_whitespace() || c == '\\')
    {
        return None;
    }
    let url = Url::parse(destination).ok()?;
    if !matches!(url.scheme(), "http" | "https") || !url.has_host() {
        return None;
    }

    let (rest, fragment) = match destination.split_once('#') {
        Some((rest, fragment)) => (rest, Some(fragment)),
        None => (destination, None),
    };
    let (base, query) = match rest.split_once('?') {
        Some((base, query)) => (base, Some(query)),
        None => (rest, None),
    };

    let mut result = String::with_capacity(destination.len() + click_id.len() + 14);
    result.push_str(base);
    result.push('?');
    let kept = query
        .unwrap_or_default()
        .split('&')
        .filter(|pair| !is_click_id(pair));
    for pair in kept {
        result.push_str(pair);
        if !result.ends_with('?') {
            result.push('&');
        }
    }
    result.push_str(CLICK_ID_PARAM);
    result.push('=');
    result.push_str(click_id);
    if let Some(fragment) = fragment {
        result.push('#');
        result.push_str(fragment);
    }
    Some(result)
}

fn is_click_id(pair: &str) -> bool {
    form_urlencoded::parse(pair.as_bytes())
        .next()
        .is_some_and(|(name, _)| name == CLICK_ID_PARAM)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_the_id_and_keeps_the_rest() {
        for (destination, expected) in [
            ("https://a.com", "https://a.com?shortas_cid=ID"),
            ("https://a.com/", "https://a.com/?shortas_cid=ID"),
            ("https://a.com/p?x=1&x=2#top", "https://a.com/p?x=1&x=2&shortas_cid=ID#top"),
            ("http://a.com/p#a?b=c#d", "http://a.com/p?shortas_cid=ID#a?b=c#d"),
            ("https://a.com/?", "https://a.com/?shortas_cid=ID"),
            ("https://a.com/?#", "https://a.com/?shortas_cid=ID#"),
            ("https://a.com/?flag&e=", "https://a.com/?flag&e=&shortas_cid=ID"),
            ("https://a.com/?u=http://b.com/?c=d", "https://a.com/?u=http://b.com/?c=d&shortas_cid=ID"),
            // Encoding is not touched.
            ("https://a.com/%7eA/%2F?q=a+b%20c&s=%e4", "https://a.com/%7eA/%2F?q=a+b%20c&s=%e4&shortas_cid=ID"),
            ("HTTPS://A.com:443/ü?ü=1", "HTTPS://A.com:443/ü?ü=1&shortas_cid=ID"),
        ] {
            assert_eq!(add_click_id(destination, "ID").as_deref(), Some(expected));
        }
    }

    #[test]
    fn replaces_an_existing_id() {
        for (destination, expected) in [
            ("https://a.com/?shortas_cid=old", "https://a.com/?shortas_cid=ID"),
            ("https://a.com/?shortas_cid=1&x=1&shortas_cid=2#f", "https://a.com/?x=1&shortas_cid=ID#f"),
            ("https://a.com/?shortas%5Fcid=old&shortas_cid", "https://a.com/?shortas_cid=ID"),
            ("https://a.com/?my_shortas_cid=1", "https://a.com/?my_shortas_cid=1&shortas_cid=ID"),
        ] {
            assert_eq!(add_click_id(destination, "ID").as_deref(), Some(expected));
        }
    }

    #[test]
    fn refuses_what_it_cannot_change_safely() {
        for destination in [
            "", "a.com/p", "/p", "//a.com", "myapp://open?x=1", "mailto:a@b.c", "javascript:alert(1)",
            "https://", "https://a b.com", "https://a.com/a b", " https://a.com", "https://a.com/\r\nX: y",
            "https:\\\\a.com", "https://a.com:99999",
        ] {
            assert_eq!(add_click_id(destination, "ID"), None, "{destination:?}");
        }
    }
}

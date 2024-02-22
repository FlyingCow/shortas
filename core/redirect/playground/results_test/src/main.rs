use http::{StatusCode, Uri};

enum RedirectType {
    Permanent,
    Temporary,
}

enum FlowRouterResult {
    Empty(StatusCode),
    Json(String, StatusCode),
    PlainText(String, StatusCode),
    Proxied(Uri, StatusCode),
    Redirect(Uri, RedirectType),
    Retargeting(Uri, Vec<Uri>),
    Error
}

fn main() {
    println!("Hello, world!");
}

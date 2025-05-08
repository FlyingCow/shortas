use multimap::MultiMap;
use salvo::Request as SalvoInternalRequest;

use crate::core::flow_router::Request;

pub struct SalvoRequest<'a> {
    request: &'a SalvoInternalRequest,
}

impl<'a> SalvoRequest<'a> {
    pub fn new(request: &'a SalvoInternalRequest) -> Self {
        Self { request }
    }
}

impl<'a> Request for SalvoRequest<'a> {
    fn get_uri(&self) -> &http::Uri {
        &self.request.uri()
    }

    fn get_headers(&self) -> &http::HeaderMap {
        &self.request.headers()
    }

    fn get_method(&self) -> &http::Method {
        &self.request.method()
    }

    fn get_scheme(&self) -> &http::uri::Scheme {
        &self.request.scheme()
    }

    fn get_remote_addr(&self) -> Option<std::net::SocketAddr> {
        self.request.remote_addr().clone().into_std()
    }

    fn get_params(&self) -> &indexmap::IndexMap<String, String> {
        &self.request.params()
    }

    fn get_queries(&self) -> &MultiMap<String, String> {
        &self.request.queries()
    }
}

use cookie::Cookie;
use cookie::CookieJar;
use http::header::IntoHeaderName;
use http::HeaderValue;
use multimap::MultiMap;
use salvo::Request as SalvoInternalRequest;
use salvo::Response as SalvoInternalResponse;

use crate::core::flow_router::Request;
use crate::core::flow_router::Response;

pub struct SalvoRequest<'a> {
    request: &'a SalvoInternalRequest,
}

impl<'a> SalvoRequest<'a> {
    pub fn new(request: &'a SalvoInternalRequest) -> Self {
        Self { request }
    }
}

impl<'a> Request for SalvoRequest<'a> {
    fn uri(&self) -> &http::Uri {
        &self.request.uri()
    }

    fn headers(&self) -> &http::HeaderMap {
        &self.request.headers()
    }

    fn method(&self) -> &http::Method {
        &self.request.method()
    }

    fn scheme(&self) -> &http::uri::Scheme {
        &self.request.scheme()
    }

    fn remote_addr(&self) -> Option<std::net::SocketAddr> {
        self.request.remote_addr().clone().into_std()
    }

    fn params(&self) -> &indexmap::IndexMap<String, String> {
        &self.request.params()
    }

    fn queries(&self) -> &MultiMap<String, String> {
        &self.request.queries()
    }

    fn cookies(&self) -> &CookieJar {
        &self.request.cookies()
    }

    /// Get `Cookie` from cookies.
    fn cookie<T>(&self, name: T) -> Option<&Cookie<'static>>
    where
        T: AsRef<str>,
    {
        self.request.cookie(name)
    }
}

pub struct SalvoResponse<'a> {
    request: &'a mut SalvoInternalResponse,
}

impl<'a> SalvoResponse<'a> {
    pub fn new(request: &'a mut SalvoInternalResponse) -> Self {
        Self { request }
    }
}

impl<'a> Response for SalvoResponse<'a> {
    fn add_header<N, V>(&mut self, name: N, value: V, overwrite: bool) -> anyhow::Result<()>
    where
        N: IntoHeaderName,
        V: TryInto<HeaderValue>,
    {
        let _ = &mut self.request.add_header(name, value, overwrite)?;
        Ok(())
    }
    /// Get cookies reference.
    #[inline]
    fn cookies(&self) -> &CookieJar {
        &self.request.cookies()
    }
    /// Get mutable cookies reference.
    #[inline]
    fn cookies_mut(&mut self) -> &mut CookieJar {
        self.request.cookies_mut()
    }
    /// Helper function for get cookie.
    #[inline]
    fn cookie<T>(&self, name: T) -> Option<&Cookie<'static>>
    where
        T: AsRef<str>,
    {
        self.request.cookie(name)
    }
    /// Helper function for add cookie.
    #[inline]
    fn add_cookie(&mut self, cookie: Cookie<'static>) {
        self.request.add_cookie(cookie);
    }
}

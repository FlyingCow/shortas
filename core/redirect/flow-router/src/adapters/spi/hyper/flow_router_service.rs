use futures_util::FutureExt;
use http_body_util::{combinators::BoxBody, BodyExt, Full};

use hyper::body::{Bytes, Incoming};
use hyper::service::Service;
use hyper::{Request, Response};

use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crate::core::base_flow_router::{BaseFlowRouter, FlowRouterResult};
use crate::core::PerRequestData;

#[derive(Clone, Debug)]
pub struct FlowRouterService<Base>
where
    Base: BaseFlowRouter + Send + Sync + Clone + 'static,
{
    flow_router: Box<Base>,
}

impl<Base> FlowRouterService<Base>
where
    Base: BaseFlowRouter + Send + Sync + Clone,
{
    pub fn new(flow_router: Base) -> Self {
        FlowRouterService { flow_router: Box::new(flow_router) }
    }
}

impl<Base> Service<Request<Incoming>> for FlowRouterService<Base>
where
    Base: BaseFlowRouter + Send + Sync + Clone,
{
    type Response = Response<BoxBody<Bytes, Infallible>>;
    type Error = Infallible;
    type Future =
        Pin<Box<dyn Future<Output = std::result::Result<Self::Response, Self::Error>> + Send>>;


    fn call(&self, req: Request<Incoming>) -> Self::Future {


        let mut request_builder = Request::builder()
            .method(req.method())
            .uri(req.uri());


        for (header_name, header_value) in req.headers().iter() {
            request_builder = request_builder.header(header_name.as_str(), header_value);
        }
            

        let request = request_builder.body(())
            .unwrap();

        let data = PerRequestData {
            tls_info: None,
            request: request,
        };

        let flow_router = self.flow_router.clone();

        let fut = async move {
            
            let route = flow_router.handle(data).await.unwrap();

            let result = match route {
                
                FlowRouterResult::Redirect(uri, redirect_type) => 
                    Response::new(Full::new(Bytes::from(uri.to_string())).boxed()),
                _ => Response::new(Full::new(Bytes::from(String::from("Error1!"))).boxed())
            };

            Ok(result)
        };

        fut.boxed()
    }
}

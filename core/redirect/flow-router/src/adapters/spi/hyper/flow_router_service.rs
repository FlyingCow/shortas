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

#[derive(Copy, Clone, Debug)]
pub struct FlowRouterService<Base>
where
    Base: BaseFlowRouter<Incoming> + Send + Sync + Clone + 'static,
{
    flow_router: Base,
}

impl<Base> FlowRouterService<Base>
where
    Base: BaseFlowRouter<Incoming> + Send + Sync + Clone + 'static,
{
    pub fn new(flow_router: Base) -> Self {
        FlowRouterService { flow_router }
    }
}

impl<Base> Service<Request<Incoming>> for FlowRouterService<Base>
where
    Base: BaseFlowRouter<Incoming> + Send + Sync + Clone + 'static,
{
    type Response = Response<BoxBody<Bytes, Infallible>>;
    type Error = Infallible;
    type Future =
        Pin<Box<dyn Future<Output = std::result::Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<Incoming>) -> Self::Future {
        let data = PerRequestData {
            tls_info: None,
            request: req,
        };

        let flow_router = Arc::new(self.flow_router.clone());

        let fut = async move {
            
            let route = flow_router.handle(data).await.unwrap();

            let result = match route {
                
                FlowRouterResult::Redirect(uri, redirect_type) => 
                    Response::new(Full::new(Bytes::from(uri.to_string())).boxed()),
                _ => Response::new(Full::new(Bytes::from(String::from("Error1!"))).boxed())
            };

            Ok(result)
        };

        Box::pin(fut)
    }
}

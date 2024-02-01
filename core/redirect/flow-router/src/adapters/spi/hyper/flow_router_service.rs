use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
use hyper::service::Service;
use hyper::{Request, Response};
use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crate::core::base_flow_router::BaseFlowRouter;
use crate::core::PerRequestData;

#[derive(Copy, Clone, Debug)]
pub struct FlowRouterService<Base>
where
    Base: BaseFlowRouter<Incoming> + Send + Sync + Clone,
{
    flow_router: Base,
}

impl<Base> FlowRouterService<Base>
where
    Base: BaseFlowRouter<Incoming> + Send + Sync + Clone,
{
    pub fn new(flow_router: Base) -> Self {
        FlowRouterService { flow_router }
    }
}

impl<Base> Service<Request<Incoming>> for FlowRouterService<Base>
where
    Base: BaseFlowRouter<Incoming> + Send + Sync + Clone,
{
    type Response = Response<Full<Bytes>>;
    type Error = Infallible;
    type Future =
        Pin<Box<dyn Future<Output = std::result::Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<Incoming>) -> Self::Future {
        let data = PerRequestData {
            tls_info: None,
            request: req,
        };
        let fut = self.flow_router.handle(data);

        fut
    }
}

use std::{convert::Infallible, future::Future, pin::Pin, sync::Arc};

use http::{Request, Response};
use http_body_util::{combinators::BoxBody, BodyExt, Full};
use hyper::{body::{Bytes, Incoming}, service::Service};

use crate::core::{flow_router::{FlowRouterResult, PerRequestData}, BaseFlowRouter};

#[derive(Clone)]
pub struct FlowRouterService
{
    flow_router: Arc<Box<dyn BaseFlowRouter>>,
}


impl FlowRouterService
{
    pub fn new(flow_router: Box<dyn BaseFlowRouter>) -> Self {
        FlowRouterService { flow_router: Arc::new(flow_router) }
    }
}


impl Service<Request<Incoming>> for FlowRouterService
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
            local_addr: None,
            remote_addr: None,
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
use crate::core::{base_flow_router::Result, BaseFlowRouter, PerRequestData};

use std::{future::Future, pin::Pin};

use bytes::Bytes;
use http::Response;
use http_body_util::Full;

#[derive(Copy, Clone, Debug)]
pub struct FlowRouter {}

impl<Req> BaseFlowRouter<Req> for FlowRouter
where
    Req: Send + Sync,
{
    fn handle(
        &self,
        req: PerRequestData<Req>,
    ) -> Pin<Box<dyn Future<Output = Result<Response<Full<Bytes>>>> + Send>> {

        //let request = "Test".as_bytes();
        let request = req.request.uri().to_string();

        let result = Ok(Response::new(Full::new(
            request.into(),
        )));

        let fut = async { result };

        Box::pin(fut)
    }
}

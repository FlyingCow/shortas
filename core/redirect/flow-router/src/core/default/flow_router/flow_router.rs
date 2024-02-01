use crate::core::{base_flow_router::Result, BaseFlowRouter, BaseRoutesManager, PerRequestData};
use std::{future::Future, pin::Pin, sync::Arc};

use bytes::Bytes;
use http::{Request, Response};
use http_body_util::Full;

#[derive(Clone, Debug)]
pub struct FlowRouter<RM>
where
    RM: BaseRoutesManager + Send + Sync + Clone + 'static,
{
    routes_manager: Arc<RM>,
}

impl<RM> FlowRouter<RM>
where
    RM: BaseRoutesManager + Send + Sync + Clone,
{
    pub fn new(routes_manager: RM) -> Self {
        Self {
            routes_manager: Arc::new(routes_manager),
        }
    }
}

fn parse_domain_and_path<Req>(request: Request<Req>) -> (String, String)
where
    Req: Send + Sync,
{
    let path = &request.uri().path()[1..];

    let domain = match request.headers().get("Host") {
        Some(host) => host
            .to_str()
            .unwrap_or_default()
            .split(":")
            .next()
            .unwrap_or_default(),
        None => "",
    };

    (domain.to_ascii_lowercase(), path.to_ascii_lowercase())
}

impl<Req, RM> BaseFlowRouter<Req> for FlowRouter<RM>
where
    Req: Send + Sync,
    RM: BaseRoutesManager + Send + Sync + Clone,
{
    fn handle(
        &self,
        req: PerRequestData<Req>,
    ) -> Pin<Box<dyn Future<Output = Result<Response<Full<Bytes>>>> + Send>> {
        let (domain, path) = parse_domain_and_path(req.request);

        let routes_manager = Arc::new(self.routes_manager.clone());

        let fut = async move {
            let route = routes_manager
                .get_route("main", &domain, &path)
                .await
                .unwrap();

            let result = match route {
                Some(route) => {
                    let destination = route
                        .dest
                        .unwrap_or(String::from("Link destination is empty!"));

                    Response::new(Full::new(Bytes::from(destination)))
                }
                None => Response::new(Full::new(Bytes::from(String::from("Link Not found!")))),
            };

            Ok(result)
        };

        Box::pin(fut)
    }
}

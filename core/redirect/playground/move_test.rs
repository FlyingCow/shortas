use std::{future::Future, pin::Pin, process::Output, sync::Arc};

use http::{Request, Response};

#[tokio::main]
async fn main() {
    let router = FlowRouter {
        routes_manager: Arc::new(RoutesManager {}),
    };

    let request = Request::builder()
        .method("GET")
        .uri("http://localhost/api/")
        .body("body")
        .unwrap();

    let result = router.handle(request).await;

    println!("{}", result.unwrap().body());
}

struct Route {
    dest: String,
}

struct RoutesManager {}

impl RoutesManager {
    pub async fn get_route(&self, dest: String) -> Result<Option<Route>, std::io::Error> {
        Ok(Some(Route { dest }))
    }
}

struct FlowRouter {
    routes_manager: Arc<RoutesManager>,
}

impl FlowRouter {
    pub fn handle<Req>(
        &self,
        req: Request<Req>,
    ) -> Pin<Box<dyn Future<Output = Result<Response<String>, std::io::Error>> + Send>>
    where
        Req: Send + Sync,
    {
        let request = req.uri().host().unwrap().to_ascii_lowercase();
        let man = self.routes_manager.clone();


        let fut = async move {
            let route_result = man
                .get_route(request)
                .await
                .unwrap()
                .unwrap();

            let result = Ok(Response::new(route_result.dest));

            result
        };

        Box::pin(fut)
    }
}

// impl FlowRouterService {
//     pub fn Execute(
//         req: Request<String>,
//     ) -> Box<Pin<dyn Future<Output = Result<Response<String>, std::io::Error>>>> {
//         async {}
//     }
// }

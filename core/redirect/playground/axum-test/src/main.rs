use std::fmt;
use axum::{routing::get, http::Uri, Router};
use tokio::net::TcpStream;
use std::task::{Context, Poll};

use tower::{
    limit::ConcurrencyLimitLayer, make::Shared, Layer, Service, ServiceBuilder, ServiceExt,
};

// This service implements the Log behavior
pub struct LogService<S> {
    target: &'static str,
    service: S,
}

impl<S, Request> Service<Request> for LogService<S>
where
    S: Service<Request>,
    Request: fmt::Debug,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        // Insert log statement here or other functionality
        println!("request = {:?}, target = {:?}", request, self.target);
        self.service.call(request)
    }
}


struct FlowRouter {
    router: Router
}

impl FlowRouter {
    fn new(router: Router) -> Self {
        Self {
            router: router.route("/:link", get(|uri: Uri| async move {
                println!("{}", uri);
            }))
        }
    }
}



#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    let svc = ServiceBuilder::new()
    // Maximum of two connections at a time
    .layer(ConcurrencyLimitLayer::new(2))
    // .layer_fn(|service| Logger::new(service, "layer_fn".to_string()))
    // .layer(LoggerLayer)
    // .layer_fn(|service| Waiter::new(service, Duration::from_secs(3)))
    .service(Responder::new());

    
    // A factory for creating services from the ServiceBuilder service
    let mut factory_svc = Shared::new(svc);

        // Create a Logger<Logger<Responder>> service
        let mut svc = factory_svc.call(()).await?;

    loop {
        let (stream, _) = listener.accept().await?;

        // The ConcurrencyLimit service waits until there is available capacity, so call
        // ServiceExt::<Request>::ready to block until ready.
        // https://docs.rs/tower/0.4.13/tower/trait.ServiceExt.html#method.ready
        tokio::spawn(ServiceExt::<TcpStream>::ready(&mut svc).await?.call(stream));
    }
}
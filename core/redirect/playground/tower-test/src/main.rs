use axum::{response::Html, routing::get};
use tower_http::trace::TraceLayer;
use tower_http::timeout::TimeoutLayer;

use std::time::Duration;
use axum::{
    extract::{rejection::JsonRejection, FromRequest, MatchedPath, Request, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Router,
};
use tower::{Service, ServiceExt};
use hyper::body::Incoming;
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "tower-test=debug,tower_http=debug".into()),
        )
    .with(tracing_subscriber::fmt::layer())
    .init();


    // build our application with a route
    let app = Router::new().route("/", get(handler))
    .layer((
        TraceLayer::new_for_http()
            // Create our own span for the request and include the matched path. The matched
            // path is useful for figuring out which handler the request was routed to.
            .make_span_with(|req: &Request| {
                let method = req.method();
                let uri = req.uri();

                // axum automatically adds this extension.
                let matched_path = req
                    .extensions()
                    .get::<MatchedPath>()
                    .map(|matched_path| matched_path.as_str());

                tracing::debug_span!("request", %method, %uri, matched_path)
            })
            // By default `TraceLayer` will log 5xx responses but we're doing our specific
            // logging of errors so disable that
            .on_failure(()),
            // Graceful shutdown will wait for outstanding requests to complete. Add a timeout so
            // requests don't hang forever.
            TimeoutLayer::new(Duration::from_secs(10))
    ));


    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    

            // Continuously accept new connections.
    loop {
        // In this example we discard the remote address. See `fn serve_with_connect_info` for how
        // to expose that.
        let (socket, _remote_addr) = listener.accept().await.unwrap();

        // We don't need to call `poll_ready` because `Router` is always ready.
        let tower_service = app.clone();

        // Spawn a task to handle the connection. That way we can multiple connections
        // concurrently.
        tokio::spawn(async move {
            // Hyper has its own `AsyncRead` and `AsyncWrite` traits and doesn't use tokio.
            // `TokioIo` converts between them.
            let socket = TokioIo::new(socket);

            // Hyper also has its own `Service` trait and doesn't use tower. We can use
            // `hyper::service::service_fn` to create a hyper `Service` that calls our app through
            // `tower::Service::call`.
            let hyper_service = hyper::service::service_fn(move |request: Request<Incoming>| {
                // We have to clone `tower_service` because hyper's `Service` uses `&self` whereas
                // tower's `Service` requires `&mut self`.
                //
                // We don't need to call `poll_ready` since `Router` is always ready.
                tower_service.clone().call(request)
            });

            // `server::conn::auto::Builder` supports both http1 and http2.
            //
            // `TokioExecutor` tells hyper to use `tokio::spawn` to spawn tasks.
            if let Err(err) = server::conn::auto::Builder::new(TokioExecutor::new())
                // `serve_connection_with_upgrades` is required for websockets. If you don't need
                // that you can use `serve_connection` instead.
                .serve_connection_with_upgrades(socket, hyper_service)
                .await
            {
                eprintln!("failed to serve connection: {err:#}");
            }
        });
    }

}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
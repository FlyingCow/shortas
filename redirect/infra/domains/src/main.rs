use salvo::prelude::*;
#[handler]
async fn all(_req: &mut Request, res: &mut Response) {
    res.render(Text::Html(format!(
        "<html>
            <body>
            Not Found
                <a href=\"/todos\">{}</a>
            </body>
        </html>",
        "general"
    )));
}

#[handler]
async fn index(req: &mut Request, res: &mut Response) {
    let domain = req.param::<String>("domain");

    if domain.is_some() {
        res.render(Text::Html(format!(
            "<html>
            <body>
            root
                <a href=\"/todos\">{}</a>
            </body>
        </html>",
            domain.unwrap()
        )));
    }
}

#[handler]
async fn not_found(req: &mut Request, res: &mut Response) {
    let domain = req.param::<String>("domain").unwrap();
    let path = req.param::<String>("path").unwrap_or("".to_string());

    res.render(Text::Html(format!(
        "<html>
            <body>
            not_found
                <a href=\"/todos\">{}:{}</a>
            </body>
        </html>",
        domain, path
    )));
}

fn route() -> Router {
    Router::new()
        .push(Router::with_path("404/{domain}/{**path}").get(not_found))
        .push(Router::with_path("index/{domain}").get(index))
        .push(Router::with_path("{**}").get(all))
}

#[tokio::main]
async fn main() {
    // Initialize logging subsystem
    tracing_subscriber::fmt().init();

    // Bind server to port 5800
    let acceptor = TcpListener::new("0.0.0.0:5801").bind().await;

    let router = route();

    // Print router structure for debugging
    println!("{:?}", router);

    let service = Service::new(router).hoop(Logger::new());

    // Start serving requests
    Server::new(acceptor).serve(service).await;
}

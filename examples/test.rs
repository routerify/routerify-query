use hyper::{Body, Request, Response, Server};
use routerify::{Router, RouterService};
use routerify_query::{query_parser, RequestQueryExt};
use std::{convert::Infallible, net::SocketAddr};

async fn home_handler(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let q = req.query("bookname");
    dbg!(q);

    Ok(Response::new(Body::from("Home page")))
}

fn router() -> Router<Body, Infallible> {
    Router::builder()
        .middleware(query_parser())
        .get("/", home_handler)
        .build()
        .unwrap()
}

#[tokio::main]
async fn main() {
    let router = router();

    let service = RouterService::new(router);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

    let server = Server::bind(&addr).serve(service);

    println!("App is running on: {}", addr);
    if let Err(err) = server.await {
        eprintln!("Server error: {}", err);
    }
}

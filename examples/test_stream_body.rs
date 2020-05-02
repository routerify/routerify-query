use hyper::{Body as HyperBody, Request, Response, Server};
use routerify::{Router, RouterService};
use routerify_query::{query_parser, RequestQueryExt};
use std::{convert::Infallible, net::SocketAddr};
use stream_body::StreamBody;

async fn home_handler(req: Request<HyperBody>) -> Result<Response<StreamBody>, Infallible> {
    dbg!(req.queries());
    dbg!(req.query("name"));
    dbg!(req.query("bookName"));

    Ok(Response::new(StreamBody::from("Home page")))
}

fn router() -> Router<StreamBody, Infallible> {
    Router::builder()
        .middleware(query_parser())
        .get("/", home_handler)
        .options(
            "/*",
            |_req| async move { Ok(Response::new(StreamBody::from("Options"))) },
        )
        .any(|_req| async move { Ok(Response::new(StreamBody::from("Not Found"))) })
        .err_handler(|err| async move { Response::new(StreamBody::from(format!("Error: {}", err))) })
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

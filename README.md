[![Github Actions Status](https://github.com/routerify/routerify-query/workflows/Test/badge.svg)](https://github.com/routerify/routerify-query/actions)
[![crates.io](https://img.shields.io/crates/v/routerify-query.svg)](https://crates.io/crates/routerify-query)
[![Documentation](https://docs.rs/routerify-query/badge.svg)](https://docs.rs/routerify-query)
[![MIT](https://img.shields.io/crates/l/routerify-query.svg)](./LICENSE)

# routerify-query

A [`Routerify`](https://github.com/routerify/routerify) middleware which parses the request query string and populates in the `req` object.

[Docs](https://docs.rs/routerify-query)

## Install

Add this to your `Cargo.toml`:

```toml
[dependencies]
routerify = "3"
routerify-query = "3" 
```
 
## Example

```rust
use hyper::{Body, Request, Response, Server};
use routerify::{Router, RouterService};
// Import the query_parser function and the RequestQueryExt trait.
use routerify_query::{query_parser, RequestQueryExt};
use std::{convert::Infallible, net::SocketAddr};

// A handler for "/" page. Visit: "/?username=Alice&bookname=HarryPotter" to see query values.
async fn home_handler(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    // Access the query values.
    let user_name = req.query("username").unwrap();
    let book_name = req.query("bookname").unwrap();

    Ok(Response::new(Body::from(format!(
        "User: {}, Book: {}",
        user_name, book_name
    ))))
}

// Create a router.
fn router() -> Router<Body, Infallible> {
    Router::builder()
        // Attach the query_parser middleware.
        .middleware(query_parser())
        .get("/", home_handler)
        .build()
        .unwrap()
}

#[tokio::main]
async fn main() {
    let router = router();

    // Create a Service from the router above to handle incoming requests.
    let service = RouterService::new(router).unwrap();

    // The address on which the server will be listening.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

    // Create a server by passing the created service to `.serve` method.
    let server = Server::bind(&addr).serve(service);

    println!("App is running on: {}", addr);
    if let Err(err) = server.await {
        eprintln!("Server error: {}", err);
    }
}
```

## Contributing

Your PRs and suggestions are always welcome.

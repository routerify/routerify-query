//! A [`Routerify`](https://github.com/routerify/routerify) middleware which parses the request query string and populates in the `req` object.
//!
//! # Examples
//!
//! ```no_run
//! use hyper::{Body, Request, Response, Server};
//! use routerify::{Router, RouterService};
//! // Import the query_parser function and the RequestQueryExt trait.
//! use routerify_query::{query_parser, RequestQueryExt};
//! use std::{convert::Infallible, net::SocketAddr};
//!
//! // A handler for "/" page. Visit: "/?username=Alice&bookname=HarryPotter" to see query values.
//! async fn home_handler(req: Request<Body>) -> Result<Response<Body>, Infallible> {
//!     // Access the query values.
//!     let user_name = req.query("username").unwrap();
//!     let book_name = req.query("bookname").unwrap();
//!
//!     Ok(Response::new(Body::from(format!(
//!         "User: {}, Book: {}",
//!         user_name, book_name
//!     ))))
//! }
//!
//! // Create a router.
//! fn router() -> Router<Body, Infallible> {
//!     Router::builder()
//!         // Attach the query_parser middleware.
//!         .middleware(query_parser())
//!         .get("/", home_handler)
//!         .build()
//!         .unwrap()
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     let router = router();
//!
//!     // Create a Service from the router above to handle incoming requests.
//!     let service = RouterService::new(router).unwrap();
//!
//!     // The address on which the server will be listening.
//!     let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
//!
//!     // Create a server by passing the created service to `.serve` method.
//!     let server = Server::bind(&addr).serve(service);
//!
//!     println!("App is running on: {}", addr);
//!     if let Err(err) = server.await {
//!         eprintln!("Server error: {}", err);
//!     }
//! }
//! ```
//!

use hyper::{body::HttpBody, Request};
use routerify::Middleware;
use std::collections::HashMap;
use url::form_urlencoded;

pub use ext::RequestQueryExt;

mod ext;

#[derive(Debug, Clone)]
pub(crate) struct Query(pub HashMap<String, String>);

/// Parses the request query string and populates in the `req` object.
///
/// # Examples
///
/// ```
/// use hyper::{Body, Request, Response, Server};
/// use routerify::{Router, RouterService};
/// // Import the query_parser function and the RequestQueryExt trait.
/// use routerify_query::{query_parser, RequestQueryExt};
/// use std::{convert::Infallible, net::SocketAddr};
///
/// // A handler for "/" page. Visit: "/?username=Alice&bookname=HarryPotter" to see query values.
/// async fn home_handler(req: Request<Body>) -> Result<Response<Body>, Infallible> {
///     // Access the query values.
///     let user_name = req.query("username").unwrap();
///     let book_name = req.query("bookname").unwrap();
///
///     Ok(Response::new(Body::from(format!(
///         "User: {}, Book: {}",
///         user_name, book_name
///     ))))
/// }
///
///
/// # fn run() -> Router<Body, Infallible> {
/// // Create a router.
/// Router::builder()
///   // Attach the query_parser middleware.
///   .middleware(query_parser())
///   .get("/", home_handler)
///   .build()
///   .unwrap()
/// }
/// # run();
/// ```
pub fn query_parser<B, E>() -> Middleware<B, E>
where
    B: HttpBody + Send + Sync + Unpin + 'static,
    E: std::error::Error + Send + Sync + Unpin + 'static,
{
    Middleware::pre(query_parser_middleware_handler::<E>)
}

async fn query_parser_middleware_handler<E>(mut req: Request<hyper::Body>) -> Result<Request<hyper::Body>, E>
where
    E: std::error::Error + Send + Sync + Unpin + 'static,
{
    let mut q = Query(HashMap::new());

    if let Some(query_str) = req.uri().query() {
        q = Query(form_urlencoded::parse(query_str.as_bytes()).into_owned().collect());
    }

    req.extensions_mut().insert(q);

    Ok(req)
}

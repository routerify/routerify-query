use crate::Query;
use hyper::Request;
use std::collections::HashMap;

/// An extension trait which extends the [`hyper::Request`](https://docs.rs/hyper/0.13.5/hyper/struct.Request.html) type with some helpful methods to
/// access query values from `req` object.
pub trait RequestQueryExt {
    /// It returns the parsed queries in a [HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html).
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
    ///     let queries = req.queries();
    ///
    ///     let user_name = queries.get("username").unwrap();
    ///     let book_name = queries.get("bookname").unwrap();
    ///
    ///     Ok(Response::new(Body::from(format!(
    ///         "User: {}, Book: {}",
    ///         user_name, book_name
    ///     ))))
    /// }
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
    fn queries(&self) -> &HashMap<String, String>;

    /// It returns the query value by a query name.
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
    fn query<P: Into<String>>(&self, query_name: P) -> Option<&String>;
}

impl RequestQueryExt for Request<hyper::Body> {
    fn queries(&self) -> &HashMap<String, String> {
        self.extensions()
            .get::<Query>()
            .map(|q| &q.0)
            .expect("Routerify-Query: No parsed queries added to the request object while processing request. Make sure the `query_parser` middleware is attached properly.")
    }

    fn query<P: Into<String>>(&self, query_name: P) -> Option<&String> {
        self.queries().get(&query_name.into())
    }
}

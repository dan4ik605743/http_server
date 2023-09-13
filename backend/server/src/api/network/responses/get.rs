use hyper::Response;
use std::convert::Infallible;
use tower_http::services::fs::ServeFileSystemResponseBody;

pub type GetResponse = Result<Response<ServeFileSystemResponseBody>, Infallible>;

pub mod tools;

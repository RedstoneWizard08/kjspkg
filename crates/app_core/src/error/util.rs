use axum::{body::Body, response::Response};
use std::fmt::Display;

pub trait HasCode {
    fn code(&self) -> u16;
}

pub trait AxumError: Display
where
    Self: Sized + HasCode,
{
    fn as_response(self) -> Response {
        match Response::builder()
            .status(self.code())
            .body(Body::new(format!("{}", self)))
        {
            Ok(it) => it,
            Err(err) => Response::new(Body::new(format!("Could not create a response: {}", err))),
        }
    }
}

impl<T: Display + HasCode> AxumError for T {}

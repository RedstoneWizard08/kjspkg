// TODO for the entire crate: Don't use anyhow's Error type, use a more descriptive one.

#[macro_use]
extern crate serde;

pub mod apis;
pub mod models;

pub use apis::ApiClient;

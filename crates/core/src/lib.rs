#[macro_use]
extern crate async_trait;

#[macro_use]
extern crate async_recursion;

pub mod models;

pub mod r#impl;

pub mod utils;

pub mod auth;

pub mod traits;

pub mod middleware;

pub use utils::{
    environment,
    result::{Error, Result},
};

pub use r#impl::*;

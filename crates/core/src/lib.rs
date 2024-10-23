#[macro_use]
extern crate async_trait;

pub mod database;

pub mod models;

pub mod r#impl;

pub mod utils;

pub mod traits;

pub use utils::result::{Error, Result};

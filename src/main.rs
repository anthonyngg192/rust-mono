pub mod modules;

pub mod db;

pub mod utils;

pub mod routes;

#[actix_web::main]
async fn main() {
    println!("Hello, world!");
}

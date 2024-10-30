use actix_web::{middleware, web, App, HttpServer};
use core::database::DatabaseInfo;
use env_logger;
use routes::auth::{login::get_products, testing::create_user};
use std::env;
// #[macro_use]
// extern crate serde_json;

pub mod routes;
pub mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    env::set_var("RUST_LOG", "actix_web=debug,actix_server=debug"); // You can adjust this
    env_logger::init();

    let db = DatabaseInfo::Auto.connect().await.unwrap();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()).clone())
            .service(get_products)
            .service(create_user)
            .wrap(middleware::Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

use actix_web::{post, web, Error, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Part {
    id: Option<i64>,
    part_type: Option<String>,
    name: Option<String>,
}


#[post("get_products")]
pub async fn get_products(_query: web::Json<Option<Part>>) -> Result<HttpResponse, Error> {
    print!("hellp");
    Ok(HttpResponse::Ok().finish())
}

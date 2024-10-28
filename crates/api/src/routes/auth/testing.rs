use core::{database::Database, models::users::user::User};

use actix_web::{get, post, web::Data, HttpResponse, Responder};
use chrono::Utc;
use mongodb::bson::oid::ObjectId;

#[get("auth/testing")]
pub async fn testing_fn(db: Data<Database>) -> HttpResponse {
    match db.get_all().await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_err) => HttpResponse::NotFound().into(),
    }
}

#[post("auth/user")]
pub async fn testing_create_user(db: Data<Database>) -> impl Responder {
    let object_id = ObjectId::new();
    let now = Utc::now();
    let timestamp_millis = now.timestamp_millis();
    let user = User {
        id: Some(object_id.to_string()),
        email: "tony@gmail.com".to_string(),
        password: "1231231231".to_string(),
        name: "tony".to_string(),
        code: "TONY".to_string(),
        password_updated_at: timestamp_millis.clone(),
        status: core::models::users::user::Status::Active,
        blacklist: vec![],
    };
    let res = db.create_user(&user).await;
    match res {
        Ok(_) => {
            println!("Success")
        }
        Err(_) => {
            println!("Bi Loi")
        }
    }
    "success"
}

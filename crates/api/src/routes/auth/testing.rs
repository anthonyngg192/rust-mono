use core::{
    database::Database,
    models::users::user::{CreateNewUser, NewUserPayload},
};

use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};

#[post("/user/new")]
pub async fn create_user(db: Data<Database>, payload: Json<NewUserPayload>) -> HttpResponse {
    let code = db.generate_user_code().await.unwrap();

    let res = db
        .create_user(
            &CreateNewUser {
                email: payload.email.to_string(),
                name: payload.name.to_string(),
            },
            &payload.password,
            &code,
        )
        .await;
    match res {
        Ok(_) => HttpResponse::Ok().json(true),
        Err(_) => HttpResponse::BadRequest().body("Email already exited"),
    }
}

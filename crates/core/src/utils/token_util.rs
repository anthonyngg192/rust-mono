use actix_web::{http::header::HeaderValue, web::Data};
use jsonwebtoken::{DecodingKey, TokenData, Validation};

use crate::{
    database::Database,
    models::users::user::{User, UserToken},
    Error,
};

use super::environment::JWT_SECRET_KEY;

pub fn decode_token(token: String) -> Result<TokenData<UserToken>, Error> {
    let result = jsonwebtoken::decode::<UserToken>(
        &token,
        &DecodingKey::from_base64_secret(&JWT_SECRET_KEY.as_ref())
            .expect("Failed to create DecodingKey"),
        &Validation::default(),
    );
    match result {
        Ok(data_result) => Ok(data_result),
        Err(_err) => Err(Error::BadRequest),
    }
}

pub async fn verify_token(
    token_data: &TokenData<UserToken>,
    db: Data<Database>,
) -> Result<User, String> {
    if db.validate_user_token(&token_data.claims).await {
        Ok(token_data.claims.user.clone())
    } else {
        Err("Invalid Token".to_string())
    }
}

pub fn is_auth_header_valid(authen_header: &HeaderValue) -> bool {
    if let Ok(authen_str) = authen_header.to_str() {
        return authen_str.starts_with("bearer") || authen_str.starts_with("Bearer");
    }

    return false;
}

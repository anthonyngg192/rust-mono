use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::ValidationErrors;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, Display)]
pub enum Error {
    #[display(fmt = "Not Found")]
    NotFound,

    #[display(fmt = "Bad Request")]
    BadRequest,

    #[display(fmt = "Internal Server")]
    InternalServer,

    #[display(fmt = "Failed validation")]
    FailedValidation {
        #[serde(skip_serializing, skip_deserializing)]
        error: ValidationErrors,
    },

    #[display(fmt = "Database Error")]
    DatabaseError {
        operation: &'static str,
        with: &'static str,
    },

    #[display(fmt = "Email already exited")]
    EmailAlreadyExisted,
}

impl Error {
    pub fn from_invalid<T>(validation_error: ValidationErrors) -> Result<T> {
        Err(Error::FailedValidation {
            error: validation_error,
        })
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::BadRequest => HttpResponse::BadRequest().finish(),
            Error::NotFound => HttpResponse::NotFound().finish(),
            Error::FailedValidation { error: _ } => HttpResponse::BadRequest().finish(),
            Error::EmailAlreadyExisted => HttpResponse::BadRequest().json("Email already exited"),
            Error::DatabaseError {
                operation: _,
                with: _,
            } => HttpResponse::BadGateway().finish(),
            Error::InternalServer => HttpResponse::InternalServerError().finish(),
        }
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }
}

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::ValidationErrors;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub enum Error {
    NotFound,
    BadRequest,
    FailedValidation {
        #[serde(skip_serializing, skip_deserializing)]
        error: ValidationErrors,
    },
    DatabaseError {
        operation: &'static str,
        with: &'static str,
    },
}

impl Error {
    pub fn from_invalid<T>(validation_error: ValidationErrors) -> Result<T> {
        Err(Error::FailedValidation {
            error: validation_error,
        })
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

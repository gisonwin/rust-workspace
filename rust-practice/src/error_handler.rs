use std::fmt;
use std::fmt::{format, Formatter, Write};
use diesel::result;
use serde_json::json;

#[derive(Debug, Deserialize)]
pub struct CustomError {
    pub error_status_code: u16,
    pub error_message: String,
}

impl CustomError {
    pub fn new(error_status_code: u16, error_message: String) -> CustomError {
        CustomError {
            error_status_code,
            error_message,
        }
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.error_message.as_str())
    }
}

impl From<DieselError> for CustomError {
    fn from(error: DieselError) -> CustomError {
        match error {
            DieselError::DatabaseError(_, err) => CustomError::new(409, err.message().to_string()),
            DieselError::NotFound => CustomError(404,"custom not found".to_string()),
            err =>CustomError::new(500,format!("unknown diesel error:{}",err))
        }
    }
}

impl ResponseError for CustomError{

}
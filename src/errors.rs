use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum ServiceError {

    #[display(fmt = "JWKSFetchError")]
    JWKSFetchError,
}
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {           
            ServiceError::JWKSFetchError => {
                HttpResponse::InternalServerError().json("Could not fetch JWKS")
            }
        }
    }
}

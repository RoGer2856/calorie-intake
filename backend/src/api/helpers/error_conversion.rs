use super::ErrorMessage;
use crate::hyper_helpers::response::DeserializeJsonRequestError;
use crate::hyper_helpers::ErrorResponse;
use crate::services::*;
use crate::utils::*;

impl From<DeserializeJsonRequestError> for ErrorResponse {
    fn from(e: DeserializeJsonRequestError) -> Self {
        let msg = match e {
            DeserializeJsonRequestError::Hyper(_) => "Cannot read bytes from stream",
            DeserializeJsonRequestError::Utf8(_) => "Cannot decode UTF8 data",
            DeserializeJsonRequestError::SerdeJson(_) => "Invalid JSON object",
        };

        let msg = ErrorMessage {
            reason: msg.to_string(),
        };

        msg.into()
    }
}

impl From<DietAuthorizationError> for ErrorResponse {
    fn from(e: DietAuthorizationError) -> Self {
        let status = match e {
            DietAuthorizationError::StdIoError(_) => hyper::StatusCode::INTERNAL_SERVER_ERROR,
            DietAuthorizationError::TomlDeserializeError(_) => {
                hyper::StatusCode::INTERNAL_SERVER_ERROR
            }
            DietAuthorizationError::JwtManagerError(JwtError::InvalidSecretSize) => {
                hyper::StatusCode::INTERNAL_SERVER_ERROR
            }
            DietAuthorizationError::JwtManagerError(JwtError::JwtMacError) => {
                hyper::StatusCode::INTERNAL_SERVER_ERROR
            }
            DietAuthorizationError::JwtManagerError(JwtError::InvalidJwtReceived) => {
                hyper::StatusCode::BAD_REQUEST
            }
            DietAuthorizationError::InvalidRoleStringError(_) => hyper::StatusCode::BAD_REQUEST,
            DietAuthorizationError::InvalidJwtReceivedError => hyper::StatusCode::BAD_REQUEST,
        };

        if status == hyper::StatusCode::BAD_REQUEST {
            let msg = ErrorMessage {
                reason: "Invalid access token".to_string(),
            };

            msg.into()
        } else {
            ErrorResponse::from_status_code(status)
        }
    }
}

impl From<FoodStorageError> for ErrorResponse {
    fn from(e: FoodStorageError) -> Self {
        match e {
            FoodStorageError::ItemNotFound => {
                let msg = ErrorMessage {
                    reason: "Item not found".to_string(),
                };

                msg.to_response(hyper::StatusCode::NOT_FOUND)
            }
            FoodStorageError::InternalError => {
                ErrorResponse::from_status_code(hyper::StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}

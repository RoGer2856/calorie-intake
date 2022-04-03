#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ErrorMessage<T> {
    pub reason: T,
}

impl<T> ErrorMessage<T>
where
    T: serde::Serialize,
{
    pub fn to_response(self, status: hyper::StatusCode) -> crate::hyper_helpers::ErrorResponse {
        let response = match crate::hyper_helpers::create_json_response(status, &self) {
            Ok(response) => response,
            Err(_) => crate::hyper_helpers::response_from_status_code(
                hyper::StatusCode::INTERNAL_SERVER_ERROR,
            ),
        };

        crate::hyper_helpers::ErrorResponse(response)
    }
}

impl<T> From<ErrorMessage<T>> for crate::hyper_helpers::ErrorResponse
where
    T: serde::Serialize,
{
    fn from(e: ErrorMessage<T>) -> Self {
        e.to_response(hyper::StatusCode::BAD_REQUEST)
    }
}

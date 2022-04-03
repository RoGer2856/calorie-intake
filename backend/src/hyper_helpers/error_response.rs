pub struct ErrorResponse(pub hyper::Response<hyper::Body>);

impl ErrorResponse {
    pub fn from_status_code(status_code: hyper::StatusCode) -> Self {
        Self(crate::hyper_helpers::response_from_status_code(status_code))
    }
}

pub fn response_ok() -> hyper::Response<hyper::Body> {
    let mut response = hyper::Response::default();
    *response.status_mut() = hyper::StatusCode::OK;
    response
}

pub fn response_from_status_code(status_code: hyper::StatusCode) -> hyper::Response<hyper::Body> {
    let mut response = hyper::Response::default();
    *response.status_mut() = status_code;
    response
}

pub fn empty_request_from_method(
    method: hyper::Method,
    uri: hyper::Uri,
) -> hyper::Request<hyper::Body> {
    let mut request = hyper::Request::default();
    *request.method_mut() = method;
    *request.uri_mut() = uri;
    request
}

#[derive(Debug)]
pub enum DeserializeJsonResponseError {
    Hyper(hyper::Error),
    Utf8(std::str::Utf8Error),
    SerdeJson(serde_json::Error),
}

impl From<hyper::Error> for DeserializeJsonResponseError {
    fn from(e: hyper::Error) -> Self {
        Self::Hyper(e)
    }
}

impl From<std::str::Utf8Error> for DeserializeJsonResponseError {
    fn from(e: std::str::Utf8Error) -> Self {
        Self::Utf8(e)
    }
}

impl From<serde_json::Error> for DeserializeJsonResponseError {
    fn from(e: serde_json::Error) -> Self {
        Self::SerdeJson(e)
    }
}

#[derive(Debug)]
pub enum DeserializeJsonRequestError {
    Hyper(hyper::Error),
    Utf8(std::str::Utf8Error),
    SerdeJson(serde_json::Error),
}

impl From<hyper::Error> for DeserializeJsonRequestError {
    fn from(e: hyper::Error) -> Self {
        Self::Hyper(e)
    }
}

impl From<std::str::Utf8Error> for DeserializeJsonRequestError {
    fn from(e: std::str::Utf8Error) -> Self {
        Self::Utf8(e)
    }
}

impl From<serde_json::Error> for DeserializeJsonRequestError {
    fn from(e: serde_json::Error) -> Self {
        Self::SerdeJson(e)
    }
}

#[derive(Debug)]
pub enum SerializeJsonResponseError {
    SerdeJson(serde_json::Error),
}

impl From<serde_json::Error> for SerializeJsonResponseError {
    fn from(e: serde_json::Error) -> Self {
        Self::SerdeJson(e)
    }
}

impl From<SerializeJsonResponseError> for crate::hyper_helpers::ErrorResponse {
    fn from(_: SerializeJsonResponseError) -> Self {
        crate::hyper_helpers::ErrorResponse::from_status_code(
            hyper::StatusCode::INTERNAL_SERVER_ERROR,
        )
    }
}

pub struct Deserializer {
    body_buffer: Vec<u8>,
}

impl Deserializer {
    pub fn new() -> Self {
        Self {
            body_buffer: Vec::new(),
        }
    }

    pub async fn read_response_as_json<'a, T: serde::Deserialize<'a>>(
        &'a mut self,
        response: hyper::Response<hyper::Body>,
    ) -> Result<T, DeserializeJsonResponseError> {
        let bytes = hyper::body::to_bytes(response.into_body()).await?;
        self.body_buffer = bytes.into_iter().collect();
        let data = std::str::from_utf8(&self.body_buffer)?;
        Ok(serde_json::from_str(data)?)
    }

    pub async fn read_request_as_json<'a, T: serde::Deserialize<'a>>(
        &'a mut self,
        request: hyper::Request<hyper::Body>,
    ) -> Result<T, DeserializeJsonRequestError> {
        let bytes = hyper::body::to_bytes(request.into_body()).await?;
        self.body_buffer = bytes.into_iter().collect();
        let data = std::str::from_utf8(&self.body_buffer)?;
        Ok(serde_json::from_str(data)?)
    }
}

pub fn create_json_response<T: serde::Serialize>(
    status_code: hyper::StatusCode,
    data: &T,
) -> Result<hyper::Response<hyper::Body>, SerializeJsonResponseError> {
    let response = hyper::Response::builder()
        .status(status_code)
        .body(hyper::Body::from(serde_json::to_string(data)?))
        .unwrap();

    Ok(response)
}

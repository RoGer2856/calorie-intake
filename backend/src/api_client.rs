use crate::api::food::messages::*;

pub struct ApiClient {
    base_url: String,
    deserializer: crate::hyper_helpers::response::Deserializer,
}

#[derive(Debug)]
pub enum ApiClientError {
    Hyper(hyper::Error),
    SerdeJson(serde_json::Error),
    InvalidUri(http::uri::InvalidUri),
    Http(http::Error),
    Deserialization(crate::hyper_helpers::response::DeserializeJsonResponseError),
}

impl From<hyper::Error> for ApiClientError {
    fn from(e: hyper::Error) -> Self {
        Self::Hyper(e)
    }
}

impl From<serde_json::Error> for ApiClientError {
    fn from(e: serde_json::Error) -> Self {
        Self::SerdeJson(e)
    }
}

impl From<http::uri::InvalidUri> for ApiClientError {
    fn from(e: http::uri::InvalidUri) -> Self {
        Self::InvalidUri(e)
    }
}

impl From<http::Error> for ApiClientError {
    fn from(e: http::Error) -> Self {
        Self::Http(e)
    }
}

impl From<crate::hyper_helpers::response::DeserializeJsonResponseError> for ApiClientError {
    fn from(e: crate::hyper_helpers::response::DeserializeJsonResponseError) -> Self {
        Self::Deserialization(e)
    }
}

pub struct StructResponse<T> {
    pub status: hyper::StatusCode,
    pub headers: hyper::HeaderMap,
    pub object: T,
}

impl ApiClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            deserializer: crate::hyper_helpers::response::Deserializer::new(),
        }
    }

    pub async fn get_status(
        &mut self,
    ) -> Result<StructResponse<crate::hyper_helpers::EmptyMessage>, ApiClientError> {
        self.json_request::<crate::hyper_helpers::EmptyMessage, crate::hyper_helpers::EmptyMessage>(
            hyper::Method::GET,
            &crate::hyper_helpers::EmptyMessage,
            "/status",
        )
        .await
    }

    pub async fn add_food(
        &mut self,
        access_token: &str,
        food_request: &AddFoodRequest,
    ) -> Result<StructResponse<AddFoodResponse>, ApiClientError> {
        self.json_request::<AddFoodRequest, AddFoodResponse>(
            hyper::Method::POST,
            &food_request,
            &("/food?access_token=".to_string() + &access_token),
        )
        .await
    }

    pub async fn get_food_list(
        &mut self,
        access_token: &str,
    ) -> Result<StructResponse<GetFoodListResponse>, ApiClientError> {
        self.json_request::<crate::hyper_helpers::EmptyMessage, GetFoodListResponse>(
            hyper::Method::GET,
            &crate::hyper_helpers::EmptyMessage,
            &("/food?access_token=".to_string() + &access_token),
        )
        .await
    }

    async fn json_request<'a, T: serde::Serialize, R: serde::Deserialize<'a>>(
        &'a mut self,
        method: hyper::Method,
        data: &T,
        request_url: &str,
    ) -> Result<StructResponse<R>, ApiClientError> {
        let client = hyper::client::Client::new();

        let request = if method == hyper::Method::GET || method == hyper::Method::HEAD {
            hyper::Request::builder()
                .method(method)
                .uri(self.create_uri(request_url)?)
                .body(hyper::Body::empty())?
        } else {
            hyper::Request::builder()
                .method(method)
                .uri(self.create_uri(request_url)?)
                .body(hyper::Body::from(serde_json::to_string(data)?))?
        };

        let response = client.request(request).await?;

        Ok(StructResponse {
            status: response.status(),
            headers: response.headers().clone(),
            object: self
                .deserializer
                .read_response_as_json::<R>(response)
                .await?,
        })
    }

    fn create_uri(&mut self, request_url: &str) -> Result<hyper::Uri, http::uri::InvalidUri> {
        (self.base_url.clone() + request_url).parse()
    }
}

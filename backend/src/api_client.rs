use crate::api::food::messages::*;
use crate::api::userinfo::messages::*;

pub struct ApiClient {
    base_url: String,
    deserializer: crate::hyper_helpers::Deserializer,
}

#[derive(Debug)]
pub enum ApiClientError {
    Hyper(hyper::Error),
    SerdeJson(serde_json::Error),
    InvalidUri(http::uri::InvalidUri),
    Http(http::Error),
    Deserialization(crate::hyper_helpers::DeserializeJsonResponseError),
}

#[derive(Debug)]
pub enum RequestError {
    ApiClientError(ApiClientError),
    ClientOrServerError(StructResponse<crate::api::helpers::ErrorMessage<String>>),
}

impl From<hyper::Error> for RequestError {
    fn from(e: hyper::Error) -> Self {
        Self::ApiClientError(ApiClientError::Hyper(e))
    }
}

impl From<serde_json::Error> for RequestError {
    fn from(e: serde_json::Error) -> Self {
        Self::ApiClientError(ApiClientError::SerdeJson(e))
    }
}

impl From<http::uri::InvalidUri> for RequestError {
    fn from(e: http::uri::InvalidUri) -> Self {
        Self::ApiClientError(ApiClientError::InvalidUri(e))
    }
}

impl From<http::Error> for RequestError {
    fn from(e: http::Error) -> Self {
        Self::ApiClientError(ApiClientError::Http(e))
    }
}

impl From<crate::hyper_helpers::DeserializeJsonResponseError> for RequestError {
    fn from(e: crate::hyper_helpers::DeserializeJsonResponseError) -> Self {
        Self::ApiClientError(ApiClientError::Deserialization(e))
    }
}

#[derive(Debug)]
pub struct StructResponse<T> {
    pub status: hyper::StatusCode,
    pub headers: hyper::HeaderMap,
    pub object: T,
}

#[derive(Debug)]
pub struct NoBodyResponse {
    pub status: hyper::StatusCode,
    pub headers: hyper::HeaderMap,
}

impl ApiClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            deserializer: crate::hyper_helpers::Deserializer::new(),
        }
    }

    pub async fn get_status(&mut self) -> Result<NoBodyResponse, RequestError> {
        self.empty_request_with_no_response_body(hyper::Method::GET, "/status")
            .await
    }

    pub async fn add_food(
        &mut self,
        access_token: &str,
        food_request: &AddFoodRequest,
    ) -> Result<StructResponse<AddFoodResponse>, RequestError> {
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
    ) -> Result<StructResponse<GetFoodListResponse>, RequestError> {
        self.empty_request_with_json_response::<GetFoodListResponse>(
            hyper::Method::GET,
            &("/food?access_token=".to_string() + &access_token),
        )
        .await
    }

    pub async fn get_all_user_food_list(
        &mut self,
        access_token: &str,
    ) -> Result<StructResponse<GetFoodListResponse>, RequestError> {
        self.empty_request_with_json_response::<GetFoodListResponse>(
            hyper::Method::GET,
            &("/food/all?access_token=".to_string() + &access_token),
        )
        .await
    }

    pub async fn get_food_report(
        &mut self,
        access_token: &str,
    ) -> Result<StructResponse<GetFoodReportResponse>, RequestError> {
        self.empty_request_with_json_response::<GetFoodReportResponse>(
            hyper::Method::GET,
            &("/food/report?access_token=".to_string() + &access_token),
        )
        .await
    }

    pub async fn get_food_by_id(
        &mut self,
        access_token: &str,
        id: &str,
    ) -> Result<StructResponse<GetFoodByIdResponse>, RequestError> {
        self.empty_request_with_json_response::<GetFoodByIdResponse>(
            hyper::Method::GET,
            &("/food/".to_string() + id + "?access_token=" + &access_token),
        )
        .await
    }

    pub async fn update_food_by_id(
        &mut self,
        access_token: &str,
        id: &str,
        food: &UpdateFoodRequest,
    ) -> Result<NoBodyResponse, RequestError> {
        self.json_request_with_no_response_body(
            hyper::Method::PUT,
            food,
            &("/food/".to_string() + id + "?access_token=" + &access_token),
        )
        .await
    }

    pub async fn delete_food_by_id(
        &mut self,
        access_token: &str,
        id: &str,
    ) -> Result<NoBodyResponse, RequestError> {
        self.empty_request_with_no_response_body(
            hyper::Method::DELETE,
            &("/food/".to_string() + id + "?access_token=" + &access_token),
        )
        .await
    }

    pub async fn get_userinfo(
        &mut self,
        access_token: &str,
    ) -> Result<StructResponse<GetUserInfoResponse>, RequestError> {
        self.empty_request_with_json_response::<GetUserInfoResponse>(
            hyper::Method::GET,
            &("/userinfo?access_token=".to_string() + &access_token),
        )
        .await
    }

    async fn empty_request_with_no_response_body(
        &mut self,
        method: hyper::Method,
        request_url: &str,
    ) -> Result<NoBodyResponse, RequestError> {
        let client = hyper::client::Client::new();

        let request = crate::hyper_helpers::empty_request_from_method(
            method,
            self.create_uri(request_url).unwrap(),
        );

        let response = client.request(request).await?;

        if response.status() == hyper::StatusCode::OK {
            Ok(NoBodyResponse {
                status: response.status(),
                headers: response.headers().clone(),
            })
        } else {
            Err(RequestError::ClientOrServerError(StructResponse {
                status: response.status(),
                headers: response.headers().clone(),
                object: self
                    .deserializer
                    .read_response_as_json::<crate::api::helpers::ErrorMessage<String>>(response)
                    .await?,
            }))
        }
    }

    async fn json_request_with_no_response_body<T: serde::Serialize>(
        &mut self,
        method: hyper::Method,
        data: &T,
        request_url: &str,
    ) -> Result<NoBodyResponse, RequestError> {
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

        if response.status() == hyper::StatusCode::OK {
            Ok(NoBodyResponse {
                status: response.status(),
                headers: response.headers().clone(),
            })
        } else {
            Err(RequestError::ClientOrServerError(StructResponse {
                status: response.status(),
                headers: response.headers().clone(),
                object: self
                    .deserializer
                    .read_response_as_json::<crate::api::helpers::ErrorMessage<String>>(response)
                    .await?,
            }))
        }
    }

    async fn empty_request_with_json_response<'a, R: serde::Deserialize<'a>>(
        &'a mut self,
        method: hyper::Method,
        request_url: &str,
    ) -> Result<StructResponse<R>, RequestError> {
        let client = hyper::client::Client::new();

        let request = crate::hyper_helpers::empty_request_from_method(
            method,
            self.create_uri(request_url).unwrap(),
        );

        let response = client.request(request).await?;

        if response.status() == hyper::StatusCode::OK {
            Ok(StructResponse {
                status: response.status(),
                headers: response.headers().clone(),
                object: self
                    .deserializer
                    .read_response_as_json::<R>(response)
                    .await?,
            })
        } else {
            Err(RequestError::ClientOrServerError(StructResponse {
                status: response.status(),
                headers: response.headers().clone(),
                object: self
                    .deserializer
                    .read_response_as_json::<crate::api::helpers::ErrorMessage<String>>(response)
                    .await?,
            }))
        }
    }

    async fn json_request<'a, T: serde::Serialize, R: serde::Deserialize<'a>>(
        &'a mut self,
        method: hyper::Method,
        data: &T,
        request_url: &str,
    ) -> Result<StructResponse<R>, RequestError> {
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

        if response.status() == hyper::StatusCode::OK {
            Ok(StructResponse {
                status: response.status(),
                headers: response.headers().clone(),
                object: self
                    .deserializer
                    .read_response_as_json::<R>(response)
                    .await?,
            })
        } else {
            Err(RequestError::ClientOrServerError(StructResponse {
                status: response.status(),
                headers: response.headers().clone(),
                object: self
                    .deserializer
                    .read_response_as_json::<crate::api::helpers::ErrorMessage<String>>(response)
                    .await?,
            }))
        }
    }

    fn create_uri(&mut self, request_url: &str) -> Result<hyper::Uri, http::uri::InvalidUri> {
        (self.base_url.clone() + request_url).parse()
    }
}

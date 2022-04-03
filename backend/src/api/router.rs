use crate::utils::LogError;
use lazy_static::lazy_static;
use regex::*;

pub enum RoutingError {
    InvalidCaptureGroupId,
}

impl From<RoutingError> for crate::hyper_helpers::ErrorResponse {
    fn from(e: RoutingError) -> Self {
        match e {
            RoutingError::InvalidCaptureGroupId => {
                crate::hyper_helpers::ErrorResponse::from_status_code(
                    hyper::StatusCode::INTERNAL_SERVER_ERROR,
                )
            }
        }
    }
}

pub struct RoutingItem {
    pub method: hyper::Method,
    pub regex: Regex,
}

impl RoutingItem {
    pub fn new(method: hyper::Method, regex: &str) -> Self {
        Self {
            method,
            regex: Regex::new(&("^".to_string() + regex + "$"))
                .log_error(|| {
                    log::error!(
                        "Could not create routing table, because a regex could not be compiled"
                    )
                })
                .unwrap(),
        }
    }

    fn match_request<'a>(&self, method: &hyper::Method, path: &'a str) -> Option<Captures<'a>> {
        if *method == self.method {
            self.regex.captures(path)
        } else {
            None
        }
    }
}

trait LogRoutingError<T> {
    fn log_routing_error(self) -> Self;
}

impl<T> LogRoutingError<T> for Result<T, RoutingError> {
    fn log_routing_error(self) -> Self {
        match &self {
            Ok(_) => (),
            Err(RoutingError::InvalidCaptureGroupId) => {
                log::error!("Invalid capture group id in routing")
            }
        }
        self
    }
}

pub async fn router(
    req: hyper::Request<hyper::Body>,
    app_context: crate::AppContext,
) -> Result<hyper::Response<hyper::Body>, crate::hyper_helpers::ErrorResponse> {
    lazy_static! {
        static ref GET_STATUS: RoutingItem = RoutingItem::new(hyper::Method::GET, "/status");
        static ref GET_USERINFO: RoutingItem = RoutingItem::new(hyper::Method::GET, "/userinfo");
        static ref GET_FOOD_LIST: RoutingItem = RoutingItem::new(hyper::Method::GET, "/food");
        static ref POST_FOOD: RoutingItem = RoutingItem::new(hyper::Method::POST, "/food");
        static ref GET_FOOD: RoutingItem = RoutingItem::new(
            hyper::Method::GET,
            "/food/(food-[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12})"
        );
    }

    log::debug!("Request = {} {}", req.method(), req.uri().path());

    let path = req.uri().path().to_lowercase();
    if let Some(_captures) = GET_STATUS.match_request(&req.method(), &path) {
        crate::api::status(req, app_context).await
    } else if let Some(_captures) = GET_USERINFO.match_request(&req.method(), &path) {
        crate::api::userinfo::get_userinfo(req, app_context).await
    } else if let Some(_captures) = GET_FOOD_LIST.match_request(&req.method(), &path) {
        crate::api::food::get_food_list(req, app_context).await
    } else if let Some(_captures) = POST_FOOD.match_request(&req.method(), &path) {
        crate::api::food::add_food(req, app_context).await
    } else if let Some(captures) = GET_FOOD.match_request(&req.method(), &path) {
        let food_id = captures
            .get(1)
            .ok_or(RoutingError::InvalidCaptureGroupId)
            .log_routing_error()?
            .as_str();
        crate::api::food::get_food(req, app_context, food_id.to_string()).await
    } else {
        Err(crate::hyper_helpers::ErrorResponse::from_status_code(
            hyper::StatusCode::NOT_FOUND,
        ))
    }
}

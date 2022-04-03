use crate::utils::LogError;
use lazy_static::lazy_static;
use regex::*;

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
                    log::error!("Could create routing table, because a regex could not be compiled")
                })
                .unwrap(),
        }
    }

    fn matchRequest<'a>(&self, method: &hyper::Method, path: &'a str) -> Option<Captures<'a>> {
        if *method == self.method {
            self.regex.captures(path)
        } else {
            None
        }
    }
}

pub async fn router(
    req: hyper::Request<hyper::Body>,
    app_context: crate::AppContext,
) -> Result<hyper::Response<hyper::Body>, crate::hyper_helpers::ErrorResponse> {
    lazy_static! {
        static ref GET_STATUS: RoutingItem = RoutingItem::new(hyper::Method::GET, "/status");
        static ref GET_USERINFO: RoutingItem = RoutingItem::new(hyper::Method::GET, "/userinfo");
        static ref GET_FOOD: RoutingItem = RoutingItem::new(hyper::Method::GET, "/food");
        static ref POST_FOOD: RoutingItem = RoutingItem::new(hyper::Method::POST, "/food");
    }

    log::debug!("Request = {} {}", req.method(), req.uri().path());

    let path = req.uri().path().to_lowercase();
    if let Some(_captures) = GET_STATUS.matchRequest(&req.method(), &path) {
        crate::api::status(req, app_context).await
    } else if let Some(_captures) = GET_USERINFO.matchRequest(&req.method(), &path) {
        crate::api::userinfo::get_userinfo(req, app_context).await
    } else if let Some(_captures) = GET_FOOD.matchRequest(&req.method(), &path) {
        crate::api::food::get_food_list(req, app_context).await
    } else if let Some(_captures) = POST_FOOD.matchRequest(&req.method(), &path) {
        crate::api::food::add_food(req, app_context).await
    } else {
        Err(crate::hyper_helpers::ErrorResponse::from_status_code(
            hyper::StatusCode::NOT_FOUND,
        ))
    }
}

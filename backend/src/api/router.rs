pub async fn router(
    req: hyper::Request<hyper::Body>,
    app_context: crate::AppContext,
) -> Result<hyper::Response<hyper::Body>, crate::hyper_helpers::ErrorResponse> {
    log::debug!("Request = {} {}", req.method(), req.uri().path());

    match (req.method(), req.uri().path().to_lowercase().as_str()) {
        (&hyper::Method::GET, "/status") => crate::api::status(req, app_context).await,
        (&hyper::Method::GET, "/userinfo") => {
            crate::api::userinfo::get_userinfo(req, app_context).await
        }
        (&hyper::Method::GET, "/food") => crate::api::food::get_food_list(req, app_context).await,
        (&hyper::Method::POST, "/food") => crate::api::food::add_food(req, app_context).await,

        // 404 Not Found
        _ => Err(crate::hyper_helpers::ErrorResponse::from_status_code(
            hyper::StatusCode::NOT_FOUND,
        )),
    }
}

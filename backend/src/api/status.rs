pub async fn status(
    _req: hyper::Request<hyper::Body>,
    _app_context: crate::AppContext,
) -> Result<hyper::Response<hyper::Body>, crate::hyper_helpers::ErrorResponse> {
    Ok(crate::hyper_helpers::response::create_json_response(
        hyper::StatusCode::OK,
        &crate::hyper_helpers::EmptyMessage,
    )?)
}

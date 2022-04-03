pub mod messages {
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct GetUserInfoResponse {
        pub username: String,
        pub role: String,
        pub max_calories_per_day: u16,
    }
}

pub async fn get_userinfo(
    req: hyper::Request<hyper::Body>,
    app_context: crate::AppContext,
) -> Result<hyper::Response<hyper::Body>, crate::hyper_helpers::ErrorResponse> {
    let access_token =
        crate::api::helpers::get_access_token_from_query_params(req.uri().query().unwrap_or(""))?;

    let authz_info = app_context
        .authorization
        .lock()
        .await
        .verify_jwt(&access_token)?;

    let resp_msg = messages::GetUserInfoResponse {
        username: authz_info.username,
        role: authz_info.role.to_string(),
        max_calories_per_day: authz_info.max_calories_per_day,
    };

    Ok(crate::hyper_helpers::create_json_response(
        hyper::StatusCode::OK,
        &resp_msg,
    )?)
}

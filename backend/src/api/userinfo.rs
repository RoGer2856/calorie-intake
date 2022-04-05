use crate::services::RoleType;

pub mod messages {
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct GetUserInfoResponse {
        pub username: String,
        pub role: String,
        pub max_calories_per_day: u16,
    }

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct GetUserListResponse {
        pub users: Vec<String>,
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

pub async fn get_user_list(
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

    match authz_info.role {
        RoleType::Admin => {
            let mut resp_msg = messages::GetUserListResponse { users: Vec::new() };

            let food_storage = app_context.food_storage.lock().await;

            for (username, _user_food_storage) in food_storage.user_storages_iter() {
                resp_msg.users.push(username.clone());
            }

            Ok(crate::hyper_helpers::create_json_response(
                hyper::StatusCode::OK,
                &resp_msg,
            )?)
        }
        RoleType::RegularUser => Err(crate::hyper_helpers::ErrorResponse(
            crate::hyper_helpers::response_from_status_code(hyper::StatusCode::UNAUTHORIZED),
        )),
    }
}

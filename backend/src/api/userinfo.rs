use crate::services::RoleType;

pub mod messages {
    pub type GetUserInfoResponse = crate::services::UserInfo;

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct GetUserListResponse {
        pub users: Vec<GetUserInfoResponse>,
    }
}

pub async fn get_userinfo(
    _req: hyper::Request<hyper::Body>,
    _app_context: crate::AppContext,
    authz_info: crate::services::AuthorizationInfo,
) -> Result<hyper::Response<hyper::Body>, crate::hyper_helpers::ErrorResponse> {
    let resp_msg = messages::GetUserInfoResponse::from(authz_info);

    Ok(crate::hyper_helpers::create_json_response(
        hyper::StatusCode::OK,
        &resp_msg,
    )?)
}

pub async fn get_user_list(
    _req: hyper::Request<hyper::Body>,
    app_context: crate::AppContext,
    authz_info: crate::services::AuthorizationInfo,
) -> Result<hyper::Response<hyper::Body>, crate::hyper_helpers::ErrorResponse> {
    match authz_info.role {
        RoleType::Admin => {
            let mut userinfo_storage = app_context.userinfo_storage.lock().await;

            let resp_msg = messages::GetUserListResponse {
                users: userinfo_storage
                    .iter_userinfo()?
                    .map(|userinfo| messages::GetUserInfoResponse::from(userinfo))
                    .collect(),
            };

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

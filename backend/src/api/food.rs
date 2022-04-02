pub mod messages {
    pub type Food = crate::services::PartialFood;

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct AddFoodRequest {
        pub access_token: String,
        pub food: Food,
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct AddFoodResponse {
        pub id: String,
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct GetFoodListRequest {
        pub access_token: String,
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct GetFoodListResponse {
        pub foods: Vec<Food>,
    }
}

pub async fn add_food(
    req: hyper::Request<hyper::Body>,
    app_context: crate::AppContext,
) -> Result<hyper::Response<hyper::Body>, crate::hyper_helpers::ErrorResponse> {
    let mut deserializer = crate::hyper_helpers::response::Deserializer::new();
    let payload = deserializer
        .read_request_as_json::<messages::AddFoodRequest>(req)
        .await?;

    let authz_info = app_context
        .authorization
        .lock()
        .await
        .verify_jwt(&payload.access_token)?;

    let food_storage = app_context
        .food_storage
        .lock()
        .await
        .get_food_storage_for_user(authz_info.username);
    let id = food_storage.lock().await.add_food(payload.food)?;

    let resp_msg = messages::AddFoodResponse { id: id.0 };

    Ok(crate::hyper_helpers::response::create_json_response(
        hyper::StatusCode::OK,
        &resp_msg,
    )?)
}

pub async fn get_food_list(
    _req: hyper::Request<hyper::Body>,
    _app_context: crate::AppContext,
) -> Result<hyper::Response<hyper::Body>, crate::hyper_helpers::ErrorResponse> {
    Ok(crate::hyper_helpers::response::create_json_response(
        hyper::StatusCode::OK,
        &crate::hyper_helpers::EmptyMessage::new(),
    )?)
}

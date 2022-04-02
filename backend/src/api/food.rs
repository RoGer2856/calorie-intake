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
    _req: hyper::Request<hyper::Body>,
    _app_context: crate::AppContext,
) -> Result<hyper::Response<hyper::Body>, crate::hyper_helpers::ErrorResponse> {
    Ok(crate::hyper_helpers::response::create_json_response(
        hyper::StatusCode::OK,
        &crate::hyper_helpers::EmptyMessage::new(),
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

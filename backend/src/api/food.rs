use crate::services::RoleType;

pub mod messages {
    pub type Food = crate::services::Food;

    pub type AddFoodRequest = crate::services::PartialFood;

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct AddFoodResponse {
        pub id: String,
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct GetFoodListResponse {
        pub foods: Vec<Food>,
    }

    pub type GetFoodByIdRequest = crate::hyper_helpers::EmptyMessage;
    pub type GetFoodByIdResponse = crate::services::Food;
}

pub async fn add_food(
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

    let mut deserializer = crate::hyper_helpers::response::Deserializer::new();
    let payload = deserializer
        .read_request_as_json::<messages::AddFoodRequest>(req)
        .await?;

    let food_storage = app_context
        .food_storage
        .lock()
        .await
        .get_food_storage_for_user(authz_info.username);
    let mut food_storage = food_storage.lock().await;

    let id = food_storage.add_food(payload)?;

    let resp_msg = messages::AddFoodResponse { id: id.0 };

    Ok(crate::hyper_helpers::response::create_json_response(
        hyper::StatusCode::OK,
        &resp_msg,
    )?)
}

pub async fn get_food_list(
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

    let food_storage = app_context
        .food_storage
        .lock()
        .await
        .get_food_storage_for_user(authz_info.username);
    let mut food_storage = food_storage.lock().await;

    let resp_msg = messages::GetFoodListResponse {
        foods: food_storage.iter_food()?.collect(),
    };

    Ok(crate::hyper_helpers::response::create_json_response(
        hyper::StatusCode::OK,
        &resp_msg,
    )?)
}

pub async fn get_food(
    req: hyper::Request<hyper::Body>,
    app_context: crate::AppContext,
    food_id: String,
) -> Result<hyper::Response<hyper::Body>, crate::hyper_helpers::ErrorResponse> {
    let access_token =
        crate::api::helpers::get_access_token_from_query_params(req.uri().query().unwrap_or(""))?;

    let authz_info = app_context
        .authorization
        .lock()
        .await
        .verify_jwt(&access_token)?;

    let mut food_storage = app_context.food_storage.lock().await;

    let food_id = &crate::services::FoodId(food_id);

    match authz_info.role {
        RoleType::Admin => {
            for (_username, user_food_storage) in food_storage.user_storages_iter() {
                if let Ok(food) = user_food_storage.lock().await.get_food(&food_id) {
                    return Ok(crate::hyper_helpers::response::create_json_response(
                        hyper::StatusCode::OK,
                        &food,
                    )?);
                }
            }

            Err(crate::services::FoodStorageError::ItemNotFound.into())
        }
        RoleType::RegularUser => {
            let food_storage = food_storage.get_food_storage_for_user(authz_info.username);
            let mut food_storage = food_storage.lock().await;
            Ok(crate::hyper_helpers::response::create_json_response(
                hyper::StatusCode::OK,
                &food_storage.get_food(&food_id)?,
            )?)
        }
    }
}

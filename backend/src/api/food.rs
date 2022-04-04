use crate::services::RoleType;
use crate::utils::LogError;
use num_bigint::*;
use num_traits::*;

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

    pub type GetFoodByIdResponse = crate::services::Food;

    pub type UpdateFoodRequest = crate::services::PartialFood;

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct GetFoodReportResponse {
        pub food_entries_added_last_week: u64,
        pub food_entries_added_week_before_last_week: u64,
        pub average_calories_consumed_last_week: u64,
    }
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

    let mut deserializer = crate::hyper_helpers::Deserializer::new();
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

    Ok(crate::hyper_helpers::create_json_response(
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

    let mut food_storage = app_context.food_storage.lock().await;

    let food_storage = food_storage.get_food_storage_for_user(authz_info.username);
    let mut food_storage = food_storage.lock().await;

    let resp = messages::GetFoodListResponse {
        foods: food_storage.iter_food()?.collect(),
    };

    Ok(crate::hyper_helpers::create_json_response(
        hyper::StatusCode::OK,
        &resp,
    )?)
}

pub async fn get_all_user_food_list(
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
            let food_storage = app_context.food_storage.lock().await;

            let mut foods = Vec::<crate::services::Food>::new();
            for (_username, user_food_storage) in food_storage.user_storages_iter() {
                let mut user_food_storage = user_food_storage.lock().await;
                foods.append(&mut user_food_storage.iter_food()?.collect());
            }

            Ok(crate::hyper_helpers::create_json_response(
                hyper::StatusCode::OK,
                &messages::GetFoodListResponse { foods },
            )?)
        }
        RoleType::RegularUser => Err(crate::hyper_helpers::ErrorResponse(
            crate::hyper_helpers::response_from_status_code(hyper::StatusCode::UNAUTHORIZED),
        )),
    }
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
                    return Ok(crate::hyper_helpers::create_json_response(
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
            Ok(crate::hyper_helpers::create_json_response(
                hyper::StatusCode::OK,
                &food_storage.get_food(&food_id)?,
            )?)
        }
    }
}

pub async fn update_food(
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

    match authz_info.role {
        RoleType::Admin => {
            let mut deserializer = crate::hyper_helpers::Deserializer::new();
            let payload = deserializer
                .read_request_as_json::<messages::UpdateFoodRequest>(req)
                .await?;

            let food_storage = app_context.food_storage.lock().await;

            let food_id = &crate::services::FoodId(food_id);

            for (_username, user_food_storage) in food_storage.user_storages_iter() {
                if let Ok(_) = user_food_storage
                    .lock()
                    .await
                    .update_food(&food_id, &payload)
                {
                    return Ok(crate::hyper_helpers::response_ok());
                }
            }

            Err(crate::services::FoodStorageError::ItemNotFound.into())
        }
        RoleType::RegularUser => Err(crate::hyper_helpers::ErrorResponse(
            crate::hyper_helpers::response_from_status_code(hyper::StatusCode::UNAUTHORIZED),
        )),
    }
}

pub async fn delete_food(
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

    match authz_info.role {
        RoleType::Admin => {
            let food_storage = app_context.food_storage.lock().await;

            let food_id = &crate::services::FoodId(food_id);

            for (_username, user_food_storage) in food_storage.user_storages_iter() {
                if let Ok(_food) = user_food_storage.lock().await.delete_food(&food_id) {
                    return Ok(crate::hyper_helpers::response_ok());
                }
            }

            Err(crate::services::FoodStorageError::ItemNotFound.into())
        }
        RoleType::RegularUser => Err(crate::hyper_helpers::ErrorResponse(
            crate::hyper_helpers::response_from_status_code(hyper::StatusCode::UNAUTHORIZED),
        )),
    }
}

pub async fn get_report(
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
            let datetime_now = chrono::Local::now();
            let datetime_1_week_before = datetime_now - chrono::Duration::days(7);
            let datetime_2_week_before = datetime_now - chrono::Duration::days(14);

            let food_storage = app_context.food_storage.lock().await;

            let mut user_count: u64 = 0;
            let mut calories_sum: BigUint = (0 as u64).into();
            let mut food_entries_added_last_week: u64 = 0;
            let mut food_entries_added_week_before_last_week: u64 = 0;

            for (_username, user_food_storage) in food_storage.user_storages_iter() {
                user_count += 1;

                let mut user_food_storage = user_food_storage.lock().await;
                for food in user_food_storage.iter_food()? {
                    if let Ok(food_datetime) = food.get_date_time() {
                        if food_datetime >= datetime_1_week_before {
                            food_entries_added_last_week += 1;
                            calories_sum += food.calories;
                        } else if food_datetime >= datetime_2_week_before {
                            food_entries_added_week_before_last_week += 1;
                        }
                    } else {
                        log::error!("Could not parse time");
                        return Err(crate::hyper_helpers::ErrorResponse(
                            crate::hyper_helpers::response_from_status_code(
                                hyper::StatusCode::INTERNAL_SERVER_ERROR,
                            ),
                        ));
                    }
                }
            }

            let average_calories_consumed_last_week = (calories_sum / user_count)
                .to_u64()
                .ok_or(crate::hyper_helpers::ErrorResponse(
                    crate::hyper_helpers::response_from_status_code(
                        hyper::StatusCode::INTERNAL_SERVER_ERROR,
                    ),
                ))
                .log_error(|| log::error!("Could not convert average calories to u64"))?;

            let resp_object = messages::GetFoodReportResponse {
                food_entries_added_last_week,
                food_entries_added_week_before_last_week,
                average_calories_consumed_last_week,
            };

            Ok(crate::hyper_helpers::create_json_response(
                hyper::StatusCode::OK,
                &resp_object,
            )?)
        }
        RoleType::RegularUser => Err(crate::hyper_helpers::ErrorResponse(
            crate::hyper_helpers::response_from_status_code(hyper::StatusCode::UNAUTHORIZED),
        )),
    }
}

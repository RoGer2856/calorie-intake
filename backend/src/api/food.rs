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
    authz_info: crate::services::AuthorizationInfo,
    username: String,
) -> Result<hyper::Response<hyper::Body>, crate::hyper_helpers::ErrorResponse> {
    match authz_info.role {
        RoleType::Admin => (),
        RoleType::RegularUser => {
            if username != authz_info.username {
                return Err(crate::hyper_helpers::ErrorResponse(
                    crate::hyper_helpers::response_from_status_code(
                        hyper::StatusCode::UNAUTHORIZED,
                    ),
                ));
            }
        }
    }

    let mut deserializer = crate::hyper_helpers::Deserializer::new();
    let payload = deserializer
        .read_request_as_json::<messages::AddFoodRequest>(req)
        .await?;

    let food_storage = app_context
        .food_storage
        .lock()
        .await
        .get_food_storage_for_user(username);
    let mut food_storage = food_storage.lock().await;

    let id = food_storage.add_food(payload)?;

    let resp_msg = messages::AddFoodResponse { id: id.0 };

    Ok(crate::hyper_helpers::create_json_response(
        hyper::StatusCode::OK,
        &resp_msg,
    )?)
}

pub async fn get_food_list(
    _req: hyper::Request<hyper::Body>,
    app_context: crate::AppContext,
    authz_info: crate::services::AuthorizationInfo,
    username: String,
) -> Result<hyper::Response<hyper::Body>, crate::hyper_helpers::ErrorResponse> {
    match authz_info.role {
        RoleType::Admin => (),
        RoleType::RegularUser => {
            if username != authz_info.username {
                return Err(crate::hyper_helpers::ErrorResponse(
                    crate::hyper_helpers::response_from_status_code(
                        hyper::StatusCode::UNAUTHORIZED,
                    ),
                ));
            }
        }
    }

    let mut food_storage = app_context.food_storage.lock().await;
    let food_storage = food_storage.get_food_storage_for_user(username);
    let mut food_storage = food_storage.lock().await;

    let resp = messages::GetFoodListResponse {
        foods: food_storage.iter_food()?.collect(),
    };

    Ok(crate::hyper_helpers::create_json_response(
        hyper::StatusCode::OK,
        &resp,
    )?)
}

pub async fn get_food(
    _req: hyper::Request<hyper::Body>,
    app_context: crate::AppContext,
    authz_info: crate::services::AuthorizationInfo,
    username: String,
    food_id: String,
) -> Result<hyper::Response<hyper::Body>, crate::hyper_helpers::ErrorResponse> {
    match authz_info.role {
        RoleType::Admin => (),
        RoleType::RegularUser => {
            if username != authz_info.username {
                return Err(crate::hyper_helpers::ErrorResponse(
                    crate::hyper_helpers::response_from_status_code(
                        hyper::StatusCode::UNAUTHORIZED,
                    ),
                ));
            }
        }
    }

    let mut food_storage = app_context.food_storage.lock().await;

    let food_id = &crate::services::FoodId(food_id);

    let food_storage = food_storage.get_food_storage_for_user(username);
    let mut food_storage = food_storage.lock().await;
    Ok(crate::hyper_helpers::create_json_response(
        hyper::StatusCode::OK,
        &food_storage.get_food(&food_id)?,
    )?)
}

pub async fn update_food(
    req: hyper::Request<hyper::Body>,
    app_context: crate::AppContext,
    authz_info: crate::services::AuthorizationInfo,
    username: String,
    food_id: String,
) -> Result<hyper::Response<hyper::Body>, crate::hyper_helpers::ErrorResponse> {
    match authz_info.role {
        RoleType::Admin => (),
        RoleType::RegularUser => {
            return Err(crate::hyper_helpers::ErrorResponse(
                crate::hyper_helpers::response_from_status_code(hyper::StatusCode::UNAUTHORIZED),
            ));
        }
    }

    let mut deserializer = crate::hyper_helpers::Deserializer::new();
    let payload = deserializer
        .read_request_as_json::<messages::UpdateFoodRequest>(req)
        .await?;

    let food_id = &crate::services::FoodId(food_id);

    let mut food_storage = app_context.food_storage.lock().await;
    let food_storage = food_storage.get_food_storage_for_user(username);
    let mut food_storage = food_storage.lock().await;

    if let Ok(_) = food_storage.update_food(&food_id, &payload) {
        return Ok(crate::hyper_helpers::response_ok());
    }

    Err(crate::services::FoodStorageError::ItemNotFound.into())
}

pub async fn delete_food(
    _req: hyper::Request<hyper::Body>,
    app_context: crate::AppContext,
    authz_info: crate::services::AuthorizationInfo,
    username: String,
    food_id: String,
) -> Result<hyper::Response<hyper::Body>, crate::hyper_helpers::ErrorResponse> {
    match authz_info.role {
        RoleType::Admin => (),
        RoleType::RegularUser => {
            return Err(crate::hyper_helpers::ErrorResponse(
                crate::hyper_helpers::response_from_status_code(hyper::StatusCode::UNAUTHORIZED),
            ));
        }
    }

    let food_id = &crate::services::FoodId(food_id);

    let mut food_storage = app_context.food_storage.lock().await;
    let food_storage = food_storage.get_food_storage_for_user(username);
    let mut food_storage = food_storage.lock().await;

    if let Ok(_food) = food_storage.delete_food(&food_id) {
        return Ok(crate::hyper_helpers::response_ok());
    }

    Err(crate::services::FoodStorageError::ItemNotFound.into())
}

pub async fn get_report(
    _req: hyper::Request<hyper::Body>,
    app_context: crate::AppContext,
    authz_info: crate::services::AuthorizationInfo,
) -> Result<hyper::Response<hyper::Body>, crate::hyper_helpers::ErrorResponse> {
    match authz_info.role {
        RoleType::Admin => {
            let datetime_now = crate::utils::time::current_day_start_local();
            let datetime_1_week_before = datetime_now - chrono::Duration::days(6);
            let datetime_2_week_before = datetime_1_week_before - chrono::Duration::days(7);

            let food_storage = app_context.food_storage.lock().await;

            let mut user_count: u64 = 0;
            let mut calories_sum: BigUint = (0 as u64).into();
            let mut food_entries_added_last_week: u64 = 0;
            let mut food_entries_added_week_before_last_week: u64 = 0;

            for (_username, user_food_storage) in food_storage.user_storages_iter() {
                user_count += 1;

                let mut user_food_storage = user_food_storage.lock().await;
                for food in user_food_storage.iter_food()? {
                    let food_datetime = food.get_date_time();
                    if *food_datetime >= datetime_1_week_before {
                        food_entries_added_last_week += 1;
                        calories_sum += food.calories;
                    } else if *food_datetime >= datetime_2_week_before {
                        food_entries_added_week_before_last_week += 1;
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

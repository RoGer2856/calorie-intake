use crate::functional_tests::test_utils::*;
use crate::services::*;

#[tokio::test]
#[serial_test::serial]
async fn multiple_user_foods() {
    let address = (crate::functional_tests::IPV6_LOCALHOST, 4000).into();
    crate::functional_tests::test_utils::run_test(
        address,
        crate::functional_tests::SECRETS_FILE_LOCATION.into(),
        async {
            let mut api_client =
                crate::api_client::ApiClient::new(&("http://".to_string() + &address.to_string()));

            let authorization =
                DietAuthorization::new(crate::functional_tests::SECRETS_FILE_LOCATION.into())
                    .unwrap();

            let access_token0 = authorization
                .create_jwt("john".into(), RoleType::RegularUser)
                .unwrap();

            let access_token1 = authorization
                .create_jwt("jane".into(), RoleType::RegularUser)
                .unwrap();

            let access_token_admin = authorization
                .create_jwt("admin".into(), RoleType::Admin)
                .unwrap();

            let foods = generate_example_foods();

            add_foods(&mut api_client, &access_token0.clone(), &foods).await;
            add_foods(&mut api_client, &access_token1.clone(), &foods).await;

            // check the list of foods for access_token_admin
            let resp = api_client.get_food_list(&access_token_admin).await.unwrap();
            assert_eq!(foods.len() * 2, resp.object.foods.len());
        },
    )
    .await;
}

#[tokio::test]
#[serial_test::serial]
async fn get_food_by_id() {
    let address = (crate::functional_tests::IPV6_LOCALHOST, 4000).into();
    crate::functional_tests::test_utils::run_test(
        address,
        crate::functional_tests::SECRETS_FILE_LOCATION.into(),
        async {
            let mut api_client =
                crate::api_client::ApiClient::new(&("http://".to_string() + &address.to_string()));

            let authorization =
                DietAuthorization::new(crate::functional_tests::SECRETS_FILE_LOCATION.into())
                    .unwrap();
            let access_token_john = authorization
                .create_jwt("john".into(), RoleType::RegularUser)
                .unwrap();
            let access_token_admin = authorization
                .create_jwt("admin".into(), RoleType::Admin)
                .unwrap();

            let foods = generate_example_foods();

            let ids = add_foods(&mut api_client, &access_token_john.clone(), &foods).await;

            for id in ids.iter() {
                let resp = api_client
                    .get_food_by_id(&access_token_admin, id)
                    .await
                    .unwrap();
                food_request_array_contains_food(&foods, &resp.object).unwrap();
            }
        },
    )
    .await;
}

#[tokio::test]
#[serial_test::serial]
async fn update_food() {
    let address = (crate::functional_tests::IPV6_LOCALHOST, 4000).into();
    crate::functional_tests::test_utils::run_test(
        address,
        crate::functional_tests::SECRETS_FILE_LOCATION.into(),
        async {
            let mut api_client =
                crate::api_client::ApiClient::new(&("http://".to_string() + &address.to_string()));

            let authorization =
                DietAuthorization::new(crate::functional_tests::SECRETS_FILE_LOCATION.into())
                    .unwrap();
            let access_token_john = authorization
                .create_jwt("john".into(), RoleType::RegularUser)
                .unwrap();
            let access_token_admin = authorization
                .create_jwt("admin".into(), RoleType::Admin)
                .unwrap();

            let foods = generate_example_foods();

            let ids = add_foods(&mut api_client, &access_token_john.clone(), &foods).await;

            let id_to_update = ids.get(0).unwrap();
            let updated_food = crate::api::food::messages::UpdateFoodById {
                id: None,
                name: Some("new name".into()),
                calories: Some(0),
                time: Some("new time".into()),
            };
            api_client
                .update_food_by_id(&access_token_admin, id_to_update, &updated_food)
                .await
                .unwrap();

            for id in ids.iter() {
                let resp = api_client
                    .get_food_by_id(&access_token_admin, &id)
                    .await
                    .unwrap();

                if *id == *id_to_update {
                    assert_eq!(
                        resp.object,
                        crate::services::Food::from_partial_food(
                            crate::services::FoodId(id_to_update.clone()),
                            updated_food.clone()
                        )
                        .unwrap()
                    );
                } else {
                    food_request_array_contains_food(&foods, &resp.object).unwrap();
                }
            }
        },
    )
    .await;
}

#[tokio::test]
#[serial_test::serial]
async fn delete_food_by_id() {
    let address = (crate::functional_tests::IPV6_LOCALHOST, 4000).into();
    crate::functional_tests::test_utils::run_test(
        address,
        crate::functional_tests::SECRETS_FILE_LOCATION.into(),
        async {
            let mut api_client =
                crate::api_client::ApiClient::new(&("http://".to_string() + &address.to_string()));

            let authorization =
                DietAuthorization::new(crate::functional_tests::SECRETS_FILE_LOCATION.into())
                    .unwrap();
            let access_token_john = authorization
                .create_jwt("john".into(), RoleType::RegularUser)
                .unwrap();
            let access_token_admin = authorization
                .create_jwt("admin".into(), RoleType::Admin)
                .unwrap();

            let foods = generate_example_foods();

            let ids = add_foods(&mut api_client, &access_token_john.clone(), &foods).await;

            let id_to_delete = ids.get(0).unwrap();
            api_client
                .delete_food_by_id(&access_token_admin, id_to_delete)
                .await
                .unwrap();

            let ret = api_client
                .get_food_by_id(&access_token_admin, id_to_delete)
                .await;
            if let Err(crate::api_client::RequestError::ClientOrServerError(e)) = ret {
                assert_eq!(e.status, hyper::StatusCode::NOT_FOUND);
            } else {
                assert!(false);
            }

            for id in ids.iter() {
                if *id != *id_to_delete {
                    let resp = api_client
                        .get_food_by_id(&access_token_admin, id)
                        .await
                        .unwrap();
                    food_request_array_contains_food(&foods, &resp.object).unwrap();
                }
            }
        },
    )
    .await;
}

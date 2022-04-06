use crate::functional_tests::test_utils::*;
use crate::services::*;

#[tokio::test]
#[serial_test::serial]
async fn no_food() {
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
            let access_token = authorization
                .create_jwt("john".into(), RoleType::RegularUser, 2100)
                .unwrap();

            let resp = api_client
                .get_food_list(&access_token, "john")
                .await
                .unwrap();

            assert_eq!(0, resp.object.foods.len());
        },
    )
    .await;
}

#[tokio::test]
#[serial_test::serial]
async fn get_multiple_foods() {
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
            let access_token = authorization
                .create_jwt("john".into(), RoleType::RegularUser, 2100)
                .unwrap();

            let foods = generate_example_foods();

            add_foods(&mut api_client, &access_token.clone(), "john", &foods).await;

            let resp = api_client
                .get_food_list(&access_token, "john")
                .await
                .unwrap();

            assert_eq!(foods.len(), resp.object.foods.len());
            check_food_array_equality(&foods, &resp.object.foods).unwrap();
        },
    )
    .await;
}

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
                .create_jwt("john".into(), RoleType::RegularUser, 2100)
                .unwrap();

            let access_token1 = authorization
                .create_jwt("jane".into(), RoleType::RegularUser, 2100)
                .unwrap();

            let foods = generate_example_foods();

            add_foods(&mut api_client, &access_token0.clone(), "john", &foods).await;
            add_foods(&mut api_client, &access_token1.clone(), "jane", &foods).await;

            // check the list of foods for access_token0
            {
                let resp = api_client
                    .get_food_list(&access_token0, "john")
                    .await
                    .unwrap();

                assert_eq!(foods.len(), resp.object.foods.len());
                check_food_array_equality(&foods, &resp.object.foods).unwrap();
            }

            // check the list of foods for access_token1
            {
                let resp = api_client
                    .get_food_list(&access_token1, "jane")
                    .await
                    .unwrap();

                assert_eq!(foods.len(), resp.object.foods.len());
                check_food_array_equality(&foods, &resp.object.foods).unwrap();
            }
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
            let access_token = authorization
                .create_jwt("john".into(), RoleType::RegularUser, 2100)
                .unwrap();

            let foods = generate_example_foods();

            let ids = add_foods(&mut api_client, &access_token.clone(), "john", &foods).await;

            for id in ids.iter() {
                let resp = api_client
                    .get_food_by_id(&access_token, "john", id)
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
            let access_token = authorization
                .create_jwt("john".into(), RoleType::RegularUser, 2100)
                .unwrap();

            let foods = generate_example_foods();

            let ids = add_foods(&mut api_client, &access_token.clone(), "john", &foods).await;

            let id_to_update = ids.get(0).unwrap();
            let updated_food = crate::api::food::messages::UpdateFoodRequest {
                id: None,
                name: Some("new name".into()),
                calories: Some(0),
                time: chrono::Local::now().into(),
            };
            assert!(api_client
                .update_food_by_id(&access_token, "john", id_to_update, &updated_food)
                .await
                .is_err());

            for id in ids.iter() {
                let resp = api_client
                    .get_food_by_id(&access_token, "john", id)
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
            let access_token = authorization
                .create_jwt("john".into(), RoleType::RegularUser, 2100)
                .unwrap();

            let foods = generate_example_foods();

            let ids = add_foods(&mut api_client, &access_token.clone(), "john", &foods).await;

            let id_to_delete = ids.get(0).unwrap();
            assert!(api_client
                .delete_food_by_id(&access_token, "john", id_to_delete)
                .await
                .is_err());

            for id in ids.iter() {
                let resp = api_client
                    .get_food_by_id(&access_token, "john", id)
                    .await
                    .unwrap();
                food_request_array_contains_food(&foods, &resp.object).unwrap();
            }
        },
    )
    .await;
}

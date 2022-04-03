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

            for id in ids {
                let resp = api_client
                    .get_food_by_id(&access_token_admin, &id)
                    .await
                    .unwrap();
                food_request_array_contains_food(&foods, &resp.object).unwrap();
            }
        },
    )
    .await;
}

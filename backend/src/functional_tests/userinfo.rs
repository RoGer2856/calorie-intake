use crate::functional_tests::test_utils::*;
use crate::services::*;

#[tokio::test]
#[serial_test::serial]
async fn get_regular_user_info() {
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

            let username = "john";
            let role = RoleType::RegularUser;
            let max_calories_per_day = 2100;

            let access_token = authorization
                .create_jwt(username.into(), role.clone(), max_calories_per_day)
                .unwrap();

            let resp = api_client.get_userinfo(&access_token).await.unwrap();

            assert_eq!(resp.object.username, username);
            assert_eq!(resp.object.role, role.to_string());
        },
    )
    .await;
}

#[tokio::test]
#[serial_test::serial]
async fn get_admin_user_info() {
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

            let username = "admin";
            let role = RoleType::Admin;
            let max_calories_per_day = 2100;

            let access_token = authorization
                .create_jwt(username.into(), role.clone(), max_calories_per_day)
                .unwrap();

            let resp = api_client.get_userinfo(&access_token).await.unwrap();

            assert_eq!(resp.object.username, username);
            assert_eq!(resp.object.role, role.to_string());
            assert_eq!(resp.object.max_calories_per_day, max_calories_per_day);
        },
    )
    .await;
}

#[tokio::test]
#[serial_test::serial]
async fn get_user_list_as_admin() {
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

            let access_token_admin = authorization
                .create_jwt("admin".into(), RoleType::Admin, 2100)
                .unwrap();

            let access_token_jane = authorization
                .create_jwt("jane".into(), RoleType::RegularUser, 2100)
                .unwrap();

            let access_token_john = authorization
                .create_jwt("john".into(), RoleType::RegularUser, 2100)
                .unwrap();

            add_foods(
                &mut api_client,
                &access_token_jane,
                &generate_example_foods(),
            )
            .await;
            add_foods(
                &mut api_client,
                &access_token_john,
                &generate_example_foods(),
            )
            .await;

            let resp = api_client.get_user_list(&access_token_admin).await.unwrap();

            assert_eq!(resp.object.users.len(), 2);
        },
    )
    .await;
}

#[tokio::test]
#[serial_test::serial]
async fn get_user_list_as_regular_user() {
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

            let access_token_jane = authorization
                .create_jwt("jane".into(), RoleType::RegularUser, 2100)
                .unwrap();

            let access_token_john = authorization
                .create_jwt("john".into(), RoleType::RegularUser, 2100)
                .unwrap();

            add_foods(
                &mut api_client,
                &access_token_jane,
                &generate_example_foods(),
            )
            .await;
            add_foods(
                &mut api_client,
                &access_token_john,
                &generate_example_foods(),
            )
            .await;

            assert!(api_client.get_user_list(&access_token_jane).await.is_err());
        },
    )
    .await;
}

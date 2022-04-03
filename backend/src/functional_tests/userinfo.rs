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

            let access_token = authorization
                .create_jwt(username.into(), role.clone(), 2100)
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
                .create_jwt(username.into(), role.clone(), 2100)
                .unwrap();

            let resp = api_client.get_userinfo(&access_token).await.unwrap();

            assert_eq!(resp.object.username, username);
            assert_eq!(resp.object.role, role.to_string());
            assert_eq!(resp.object.max_calories_per_day, max_calories_per_day);
        },
    )
    .await;
}

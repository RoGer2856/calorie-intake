#[tokio::test]
#[serial_test::serial]
async fn hello() {
    let address = (crate::functional_tests::IPV6_LOCALHOST, 4000).into();
    crate::functional_tests::test_utils::run_test(address, async {
        let mut api_client =
            crate::api_client::ApiClient::new(&("http://".to_string() + &address.to_string()));
        api_client.get_status().await.unwrap();
    })
    .await;
}

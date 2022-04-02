mod assert_food;
mod runtime_config;

pub use assert_food::*;
pub use runtime_config::*;

pub const TEST_TIMEOUT_MILLIS: u64 = 10000;

pub async fn run_test(
    listener_address: std::net::SocketAddr,
    secrets_file_location: std::path::PathBuf,
    test: impl std::future::Future<Output = ()>,
) {
    run_test_custom_timeout(
        listener_address,
        secrets_file_location,
        tokio::time::Duration::from_millis(TEST_TIMEOUT_MILLIS),
        test,
    )
    .await
}

pub async fn run_test_custom_timeout(
    listener_address: std::net::SocketAddr,
    secrets_file_location: std::path::PathBuf,
    timeout: tokio::time::Duration,
    test: impl std::future::Future<Output = ()>,
) {
    let _ = env_logger::builder()
        .parse_filters("debug")
        // .is_test(true)
        .try_init();

    let start_time = std::time::Instant::now();

    let config = Box::new(RuntimeConfig::new(
        listener_address.clone(),
        secrets_file_location,
    ));
    let application = crate::Application::spawn_start(config).await;
    assert!(wait_for_start(listener_address, timeout).await.is_ok());

    let end_time = std::time::Instant::now();
    let elapsed = end_time - start_time;
    let remaining_timeout = timeout - elapsed;

    tokio::select! {
        _ = async {
            test.await
        } => {
        }
        _ = tokio::time::sleep(remaining_timeout) => {
            assert!(false, "test timeout");
        }
    }

    application.halt().await;
}

async fn block_on_wait_for_start(address: std::net::SocketAddr) {
    let mut api_client =
        crate::api_client::ApiClient::new(&("http://".to_string() + &address.to_string()));
    while api_client.get_status().await.is_err() {}
}

pub async fn wait_for_start(
    address: std::net::SocketAddr,
    timeout: tokio::time::Duration,
) -> Result<(), ()> {
    tokio::select! {
        _ = block_on_wait_for_start(address) => {
            Ok(())
        }
        _ = tokio::time::sleep(timeout) => {
            Err(())
        }
    }
}

extern crate diet;

pub fn main() {
    env_logger::init();
    log::info!("Starting the application");

    let config = Box::new(diet::CommandLineArgsConfig::new());

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        let application = diet::Application::spawn_start(config).await;

        tokio::signal::ctrl_c().await.unwrap();

        application.halt().await;
    });
}

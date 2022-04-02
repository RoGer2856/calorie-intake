#[cfg(test)]
mod functional_tests;

pub mod api;
mod api_client;
mod app_context;
mod config;
pub mod hyper_helpers;
pub mod services;
pub mod utils;

pub use api_client::*;
pub use app_context::*;
pub use config::*;

pub struct Application {
    server: crate::hyper_helpers::Server,
}

impl Application {
    pub fn block_start(config: Box<dyn crate::Config>) {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        rt.block_on(async {
            let application = Self::spawn_start(config).await;

            tokio::signal::ctrl_c().await.unwrap();

            application.server.halt().await;
        });
    }

    pub async fn spawn_start(config: Box<dyn crate::Config>) -> Self {
        let listener_address = config.get_listener_address();
        let application_context = crate::AppContext::new(config);
        let server = crate::hyper_helpers::Server::spawn_start(
            listener_address,
            application_context,
            crate::api::router,
        )
        .await;
        Self { server }
    }

    pub async fn halt(self) {
        self.server.halt().await;
    }
}

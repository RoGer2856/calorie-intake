use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppContext {
    pub config: Arc<Mutex<Box<dyn crate::Config + Send>>>,
}

impl AppContext {
    pub fn new(config: Box<dyn crate::Config + Send>) -> Self {
        Self {
            config: Arc::new(Mutex::new(config)),
        }
    }
}

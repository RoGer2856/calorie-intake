use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppContext {
    pub config: Arc<Mutex<Box<dyn crate::Config>>>,
    pub authorization: Arc<Mutex<crate::services::DietAuthorization>>,
    pub food_storage: Arc<Mutex<Box<dyn crate::services::FoodStorage>>>,
}

impl AppContext {
    pub fn new(config: Box<dyn crate::Config + Send>) -> Self {
        let secrets_file_location = config.get_secrets_file_location().clone();
        Self {
            config: Arc::new(Mutex::new(config)),
            authorization: Arc::new(Mutex::new(
                crate::services::DietAuthorization::new(secrets_file_location).unwrap(),
            )),
            food_storage: Arc::new(Mutex::new(Box::new(
                crate::services::InMemoryFoodStorage::new(),
            ))),
        }
    }
}

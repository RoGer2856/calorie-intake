use std::sync::Arc;
use tokio::sync::Mutex;

use crate::services::*;

pub struct FoodStorageHelper {
    food_storage: Vec<(String, Arc<Mutex<Box<dyn FoodStorage>>>)>,
}

#[derive(Clone)]
pub struct AppContext {
    pub config: Arc<Mutex<Box<dyn crate::Config>>>,
    pub authorization: Arc<Mutex<DietAuthorization>>,
    pub food_storage: std::sync::Arc<Mutex<FoodStorageHelper>>,
}

impl AppContext {
    pub fn new(config: Box<dyn crate::Config>) -> Self {
        let secrets_file_location = config.get_secrets_file_location().clone();
        Self {
            config: Arc::new(Mutex::new(config)),
            authorization: Arc::new(Mutex::new(
                DietAuthorization::new(secrets_file_location).unwrap(),
            )),
            food_storage: std::sync::Arc::new(Mutex::new(FoodStorageHelper::new())),
        }
    }
    pub async fn get_food_storage_for_user(
        &mut self,
        username: String,
    ) -> Arc<Mutex<Box<dyn FoodStorage>>> {
        let mut food_storage = self.food_storage.lock().await;
        food_storage.get_food_storage_for_user(username)
    }
}

impl FoodStorageHelper {
    pub fn new() -> Self {
        Self {
            food_storage: Vec::new(),
        }
    }

    pub fn get_food_storage_for_user(
        &mut self,
        username: String,
    ) -> Arc<Mutex<Box<dyn FoodStorage>>> {
        if let Some(food_storage) = self.food_storage.iter().find_map(|(uname, storage)| {
            if *uname == username {
                Some(storage.clone())
            } else {
                None
            }
        }) {
            food_storage
        } else {
            let food_storage_for_user: Arc<Mutex<Box<dyn FoodStorage>>> =
                Arc::new(Mutex::new(Box::new(InMemoryFoodStorage::new())));

            self.food_storage
                .push((username, food_storage_for_user.clone()));

            food_storage_for_user
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn place_food_for_user() {
        let mut food_storage_helper = FoodStorageHelper::new();

        let partial_food0 = PartialFood {
            name: "Hamburger".into(),
            calorie: 600,
            time: "2022 March 2 8:0".into(),
        };

        // add food for john
        let id0 = {
            let food_storage = food_storage_helper.get_food_storage_for_user("john".into());
            let mut food_storage = food_storage.lock().await;
            food_storage.add_food(partial_food0.clone()).unwrap()
        };

        let partial_food1 = PartialFood {
            name: "Chicken".into(),
            calorie: 300,
            time: "2022 March 2 12:00".into(),
        };

        // add food for jane
        let id1 = {
            let food_storage = food_storage_helper.get_food_storage_for_user("jane".into());
            let mut food_storage = food_storage.lock().await;
            food_storage.add_food(partial_food1.clone()).unwrap()
        };

        {
            let food_storage = food_storage_helper.get_food_storage_for_user("john".into());
            let mut food_storage = food_storage.lock().await;
            // check that john's food is in john's storage
            assert_eq!(
                Food::from_partial_food(id0.clone(), partial_food0),
                *food_storage.get_food(&id0).unwrap()
            );
            // check that jane's food is not in john's storage
            assert!(food_storage.get_food(&id1).is_err());
        }
    }
}

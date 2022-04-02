#[derive(serde::Serialize, serde::Deserialize, PartialEq, Clone)]
pub struct PartialFood {
    pub name: String,
    pub calorie: i16,
    pub time: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FoodId(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct Food {
    pub id: FoodId,
    pub name: String,
    pub calorie: i16,
    pub time: String,
}

impl Food {
    pub fn from_partial_food(id: FoodId, partial_food: PartialFood) -> Self {
        Self {
            id,
            name: partial_food.name,
            calorie: partial_food.calorie,
            time: partial_food.time,
        }
    }
}

pub trait FoodStorage: Send {
    fn add_food(&mut self, partial_food: PartialFood) -> Result<FoodId, FoodStorageError>;
    fn get_food(&mut self, id: &FoodId) -> Result<&Food, FoodStorageError>;
}

#[derive(Debug, PartialEq)]
pub enum FoodStorageError {
    ItemNotFound,
}

pub struct InMemoryFoodStorage {
    foods: Vec<Food>,
}

impl InMemoryFoodStorage {
    pub fn new() -> Self {
        Self { foods: Vec::new() }
    }

    fn generate_id(&self) -> FoodId {
        loop {
            let new_id = "food-".to_string() + &uuid::Uuid::new_v4().to_hyphenated().to_string();
            if self.foods.iter().find(|item| item.id.0 == new_id).is_none() {
                return FoodId(new_id);
            }
        }
    }
}

impl FoodStorage for InMemoryFoodStorage {
    fn add_food(&mut self, partial_food: PartialFood) -> Result<FoodId, FoodStorageError> {
        let id = self.generate_id();
        let food = Food::from_partial_food(id.clone(), partial_food);
        self.foods.push(food);

        Ok(id)
    }

    fn get_food(&mut self, id: &FoodId) -> Result<&Food, FoodStorageError> {
        self.foods
            .iter()
            .find(|item| item.id == *id)
            .ok_or(FoodStorageError::ItemNotFound)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_food_by_id() {
        let partial_food0 = PartialFood {
            name: "Hamburger".into(),
            calorie: 600,
            time: "2022 March 2 8:0".into(),
        };

        let partial_food1 = PartialFood {
            name: "Chicken".into(),
            calorie: 300,
            time: "2022 March 2 12:00".into(),
        };

        let partial_food2 = PartialFood {
            name: "Scrambled eggs".into(),
            calorie: 400,
            time: "2022 March 2 18:00".into(),
        };

        let mut food_storage = InMemoryFoodStorage::new();
        let id0 = food_storage.add_food(partial_food0.clone()).unwrap();
        let id1 = food_storage.add_food(partial_food1.clone()).unwrap();
        let id2 = food_storage.add_food(partial_food2.clone()).unwrap();

        assert_eq!(
            Food::from_partial_food(id0.clone(), partial_food0),
            *food_storage.get_food(&id0).unwrap()
        );
        assert_eq!(
            Food::from_partial_food(id1.clone(), partial_food1),
            *food_storage.get_food(&id1).unwrap()
        );
        assert_eq!(
            Food::from_partial_food(id2.clone(), partial_food2),
            *food_storage.get_food(&id2).unwrap()
        );
    }

    #[test]
    fn missing_item() {
        let mut food_storage = InMemoryFoodStorage::new();
        assert_eq!(
            Err(FoodStorageError::ItemNotFound),
            food_storage.get_food(&FoodId("id".into()))
        )
    }
}

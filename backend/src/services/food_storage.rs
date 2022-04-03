#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Clone)]
pub struct PartialFood {
    pub name: String,
    pub calories: u16,
    pub time: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Clone)]
pub struct FoodId(pub String);

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Clone)]
pub struct Food {
    pub id: FoodId,
    pub name: String,
    pub calories: u16,
    pub time: String,
}

impl Food {
    pub fn from_partial_food(id: FoodId, partial_food: PartialFood) -> Self {
        Self {
            id,
            name: partial_food.name,
            calories: partial_food.calories,
            time: partial_food.time,
        }
    }
}

pub trait FoodStorage: Send {
    fn add_food(&mut self, partial_food: PartialFood) -> Result<FoodId, FoodStorageError>;
    fn get_food(&mut self, id: &FoodId) -> Result<&Food, FoodStorageError>;
    fn delete_food(&mut self, id: &FoodId) -> Result<(), FoodStorageError>;
    fn iter_food<'a>(&'a mut self)
        -> Result<Box<dyn Iterator<Item = Food> + 'a>, FoodStorageError>;
}

#[derive(Debug, PartialEq)]
pub enum FoodStorageError {
    InternalError,
    ItemNotFound,
}

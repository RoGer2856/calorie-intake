#[derive(Debug, PartialEq)]
pub enum FoodFromPartialError {
    ProvidedId,
    MissingField(String),
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Clone)]
pub struct PartialFood {
    pub id: Option<String>,
    pub name: Option<String>,
    pub calories: Option<u16>,
    pub time: Option<String>,
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
    pub fn update_from_partial_food(
        &mut self,
        partial_food: PartialFood,
    ) -> Result<(), FoodFromPartialError> {
        if let Some(_id) = partial_food.id {
            return Err(FoodFromPartialError::ProvidedId);
        }
        if let Some(name) = partial_food.name {
            self.name = name;
        }
        if let Some(calories) = partial_food.calories {
            self.calories = calories;
        }
        if let Some(time) = partial_food.time {
            self.time = time;
        }

        Ok(())
    }

    pub fn from_partial_food(
        id: FoodId,
        partial_food: PartialFood,
    ) -> Result<Self, FoodFromPartialError> {
        Ok(Self {
            id,
            name: partial_food
                .name
                .ok_or(FoodFromPartialError::MissingField("name".to_string()))?,
            calories: partial_food
                .calories
                .ok_or(FoodFromPartialError::MissingField("calories".to_string()))?,
            time: partial_food
                .time
                .ok_or(FoodFromPartialError::MissingField("time".to_string()))?,
        })
    }
}

pub trait FoodStorage: Send {
    fn add_food(&mut self, partial_food: PartialFood) -> Result<FoodId, FoodStorageError>;
    fn get_food(&mut self, id: &FoodId) -> Result<&Food, FoodStorageError>;
    fn update_food(
        &mut self,
        id: &FoodId,
        partial_food: PartialFood,
    ) -> Result<(), FoodStorageError>;
    fn delete_food(&mut self, id: &FoodId) -> Result<(), FoodStorageError>;
    fn iter_food<'a>(&'a mut self)
        -> Result<Box<dyn Iterator<Item = Food> + 'a>, FoodStorageError>;
}

#[derive(Debug, PartialEq)]
pub enum FoodStorageError {
    InternalError,
    ItemNotFound,
    FoodFromPartialError(FoodFromPartialError),
}

impl From<FoodFromPartialError> for FoodStorageError {
    fn from(e: FoodFromPartialError) -> Self {
        Self::FoodFromPartialError(e)
    }
}

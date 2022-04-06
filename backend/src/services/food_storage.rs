#[derive(Debug, PartialEq)]
pub enum FoodFromPartialError {
    ProvidedId,
    MissingField(String),
}

pub struct CouldNotParseTimeError(pub String);

impl From<crate::utils::time::DateTimeFromStrError> for CouldNotParseTimeError {
    fn from(e: crate::utils::time::DateTimeFromStrError) -> Self {
        CouldNotParseTimeError(e.0)
    }
}

impl std::fmt::Display for CouldNotParseTimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CouldNotParseTimeError on string {}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Time(pub chrono::DateTime<chrono::Local>);

impl Into<String> for Time {
    fn into(self) -> String {
        self.0.to_rfc3339()
    }
}

impl TryFrom<String> for Time {
    type Error = CouldNotParseTimeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(crate::utils::time::date_time_from_str(&value)?))
    }
}

impl From<chrono::DateTime<chrono::Local>> for Time {
    fn from(dt: chrono::DateTime<chrono::Local>) -> Self {
        Self(dt)
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(try_from = "Option<String>", into = "Option<String>")]
pub struct OptionTime(pub Option<chrono::DateTime<chrono::Local>>);

impl Into<Option<String>> for OptionTime {
    fn into(self) -> Option<String> {
        if let Some(dt) = self.0 {
            Some(dt.to_rfc3339())
        } else {
            None
        }
    }
}

impl TryFrom<Option<String>> for OptionTime {
    type Error = CouldNotParseTimeError;

    fn try_from(value: Option<String>) -> Result<Self, Self::Error> {
        if let Some(time_str) = value {
            Ok(Self(Some(crate::utils::time::date_time_from_str(
                &time_str,
            )?)))
        } else {
            Ok(Self(None))
        }
    }
}

impl From<chrono::DateTime<chrono::Local>> for OptionTime {
    fn from(dt: chrono::DateTime<chrono::Local>) -> Self {
        Self(Some(dt))
    }
}

impl From<Option<chrono::DateTime<chrono::Local>>> for OptionTime {
    fn from(dt: Option<chrono::DateTime<chrono::Local>>) -> Self {
        Self(dt)
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Clone)]
pub struct PartialFood {
    pub id: Option<String>,
    pub name: Option<String>,
    pub calories: Option<u16>,
    pub time: OptionTime,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Clone)]
pub struct FoodId(pub String);

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Clone)]
pub struct Food {
    pub id: FoodId,
    pub name: String,
    pub calories: u16,
    pub time: Time,
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
        if let Some(time) = partial_food.time.0 {
            self.time = time.into();
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
                .0
                .ok_or(FoodFromPartialError::MissingField("time".to_string()))?
                .into(),
        })
    }

    pub fn get_date_time(&self) -> &chrono::DateTime<chrono::Local> {
        &self.time.0
    }
}

pub trait FoodStorage: Send {
    fn add_food(&mut self, partial_food: PartialFood) -> Result<FoodId, FoodStorageError>;
    fn get_food(&mut self, id: &FoodId) -> Result<&Food, FoodStorageError>;
    fn update_food(
        &mut self,
        id: &FoodId,
        partial_food: &PartialFood,
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

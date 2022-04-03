use super::*;

pub struct InMemoryFoodStorage {
    foods: Vec<Food>,
}

pub struct InMemoryFoodStorageIterator<'a> {
    iter: std::slice::Iter<'a, Food>,
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
        self.foods.push(food?);

        Ok(id)
    }

    fn get_food(&mut self, id: &FoodId) -> Result<&Food, FoodStorageError> {
        self.foods
            .iter()
            .find(|item| item.id == *id)
            .ok_or(FoodStorageError::ItemNotFound)
    }

    fn update_food(
        &mut self,
        id: &FoodId,
        partial_food: &PartialFood,
    ) -> Result<(), FoodStorageError> {
        let food = self
            .foods
            .iter_mut()
            .find(|item| item.id == *id)
            .ok_or(FoodStorageError::ItemNotFound)?;

        food.update_from_partial_food(partial_food.clone())?;

        Ok(())
    }

    fn delete_food(&mut self, id: &FoodId) -> Result<(), FoodStorageError> {
        if let Some(position) = self.foods.iter().position(|item| item.id == *id) {
            self.foods.swap_remove(position);
            Ok(())
        } else {
            Err(FoodStorageError::ItemNotFound)
        }
    }

    fn iter_food<'a>(
        &'a mut self,
    ) -> Result<Box<dyn Iterator<Item = Food> + 'a>, FoodStorageError> {
        Ok(Box::new(InMemoryFoodStorageIterator::new(
            self.foods.iter(),
        )))
    }
}

impl<'a> InMemoryFoodStorageIterator<'a> {
    pub fn new(iter: std::slice::Iter<'a, Food>) -> Self {
        Self { iter }
    }
}

impl<'a> Iterator for InMemoryFoodStorageIterator<'a> {
    type Item = Food;

    fn next(&mut self) -> Option<Food> {
        self.iter.next().map(|food_ref| food_ref.clone())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_food_by_id() {
        let partial_food0 = PartialFood {
            id: None,
            name: Some("Hamburger".into()),
            calories: Some(600),
            time: Some("2022 March 2 8:0".into()),
        };

        let partial_food1 = PartialFood {
            id: None,
            name: Some("Chicken".into()),
            calories: Some(300),
            time: Some("2022 March 2 12:00".into()),
        };

        let partial_food2 = PartialFood {
            id: None,
            name: Some("Scrambled eggs".into()),
            calories: Some(400),
            time: Some("2022 March 2 18:00".into()),
        };

        let mut food_storage = InMemoryFoodStorage::new();
        let id0 = food_storage.add_food(partial_food0.clone()).unwrap();
        let id1 = food_storage.add_food(partial_food1.clone()).unwrap();
        let id2 = food_storage.add_food(partial_food2.clone()).unwrap();

        assert_eq!(
            Food::from_partial_food(id0.clone(), partial_food0).unwrap(),
            *food_storage.get_food(&id0).unwrap()
        );
        assert_eq!(
            Food::from_partial_food(id1.clone(), partial_food1).unwrap(),
            *food_storage.get_food(&id1).unwrap()
        );
        assert_eq!(
            Food::from_partial_food(id2.clone(), partial_food2).unwrap(),
            *food_storage.get_food(&id2).unwrap()
        );
    }

    #[test]
    fn update_food() {
        let partial_food0 = PartialFood {
            id: None,
            name: Some("Hamburger".into()),
            calories: Some(600),
            time: Some("2022 March 2 8:0".into()),
        };

        let partial_food1 = PartialFood {
            id: None,
            name: Some("Chicken".into()),
            calories: Some(300),
            time: Some("2022 March 2 12:00".into()),
        };

        let partial_food2 = PartialFood {
            id: None,
            name: Some("Scrambled eggs".into()),
            calories: Some(400),
            time: Some("2022 March 2 18:00".into()),
        };

        let mut food_storage = InMemoryFoodStorage::new();
        let id0 = food_storage.add_food(partial_food0.clone()).unwrap();
        let id1 = food_storage.add_food(partial_food1.clone()).unwrap();
        let id2 = food_storage.add_food(partial_food2.clone()).unwrap();

        let new_name = Some("new name".into());
        let new_calories = Some(0);
        let new_time = Some("new time".into());

        food_storage
            .update_food(
                &id0,
                &PartialFood {
                    id: None,
                    name: new_name.clone(),
                    calories: None,
                    time: None,
                },
            )
            .unwrap();

        food_storage
            .update_food(
                &id1,
                &PartialFood {
                    id: None,
                    name: None,
                    calories: new_calories,
                    time: None,
                },
            )
            .unwrap();

        food_storage
            .update_food(
                &id2,
                &PartialFood {
                    id: None,
                    name: None,
                    calories: None,
                    time: new_time.clone(),
                },
            )
            .unwrap();

        let food = food_storage.get_food(&id0).unwrap();
        assert_ne!(
            *food,
            Food::from_partial_food(id0.clone(), partial_food0.clone()).unwrap()
        );
        assert_eq!(
            *food,
            Food::from_partial_food(
                id0.clone(),
                PartialFood {
                    id: None,
                    name: new_name,
                    calories: partial_food0.calories,
                    time: partial_food0.time,
                }
            )
            .unwrap()
        );

        let food = food_storage.get_food(&id1).unwrap();
        assert_ne!(
            *food,
            Food::from_partial_food(id1.clone(), partial_food1.clone()).unwrap()
        );
        assert_eq!(
            *food,
            Food::from_partial_food(
                id1.clone(),
                PartialFood {
                    id: None,
                    name: partial_food1.name,
                    calories: new_calories,
                    time: partial_food1.time,
                }
            )
            .unwrap()
        );

        let food = food_storage.get_food(&id2).unwrap();
        assert_ne!(
            *food,
            Food::from_partial_food(id2.clone(), partial_food2.clone()).unwrap()
        );
        assert_eq!(
            *food,
            Food::from_partial_food(
                id2.clone(),
                PartialFood {
                    id: None,
                    name: partial_food2.name,
                    calories: partial_food2.calories,
                    time: new_time,
                }
            )
            .unwrap()
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

    #[test]
    fn delete_food_by_id() {
        let partial_food0 = PartialFood {
            id: None,
            name: Some("Hamburger".into()),
            calories: Some(600),
            time: Some("2022 March 2 8:0".into()),
        };

        let partial_food1 = PartialFood {
            id: None,
            name: Some("Chicken".into()),
            calories: Some(300),
            time: Some("2022 March 2 12:00".into()),
        };

        let partial_food2 = PartialFood {
            id: None,
            name: Some("Scrambled eggs".into()),
            calories: Some(400),
            time: Some("2022 March 2 18:00".into()),
        };

        let mut food_storage = InMemoryFoodStorage::new();
        let id0 = food_storage.add_food(partial_food0.clone()).unwrap();
        let id1 = food_storage.add_food(partial_food1.clone()).unwrap();
        let id2 = food_storage.add_food(partial_food2.clone()).unwrap();

        food_storage.delete_food(&id1).unwrap();

        assert_eq!(
            Food::from_partial_food(id0.clone(), partial_food0).unwrap(),
            *food_storage.get_food(&id0).unwrap()
        );
        assert!(food_storage.get_food(&id1).is_err());
        assert_eq!(
            Food::from_partial_food(id2.clone(), partial_food2).unwrap(),
            *food_storage.get_food(&id2).unwrap()
        );
    }
}

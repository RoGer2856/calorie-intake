use crate::api::food::messages::*;

pub fn food_request_array_contains_food(foods: &[AddFoodRequest], food: &Food) -> Result<(), ()> {
    if let Some(_) = foods
        .iter()
        .find(|item| Food::from_partial_food(food.id.clone(), (*item).clone()).unwrap() == *food)
    {
        Ok(())
    } else {
        Err(())
    }
}

pub fn food_array_contains_food(foods: &[Food], food: &AddFoodRequest) -> Result<(), ()> {
    if let Some(_) = foods
        .iter()
        .find(|item| **item == Food::from_partial_food(item.id.clone(), food.clone()).unwrap())
    {
        Ok(())
    } else {
        Err(())
    }
}

pub fn check_food_array_equality(expected: &[AddFoodRequest], received: &[Food]) -> Result<(), ()> {
    for food in expected.iter() {
        food_array_contains_food(received, food)?;
    }

    Ok(())
}

pub fn generate_example_foods() -> Vec<AddFoodRequest> {
    let mut ret = Vec::new();

    let random_value = rand::random::<u16>();

    ret.push(AddFoodRequest {
        id: None,
        name: Some("Hamburger".into()),
        calories: Some(600 + random_value),
        time: Some("2022 March 2 8:0".into()),
    });

    ret.push(AddFoodRequest {
        id: None,
        name: Some("Chicken".into()),
        calories: Some(300 + random_value),
        time: Some("2022 March 2 12:00".into()),
    });

    ret.push(AddFoodRequest {
        id: None,
        name: Some("Scrambled eggs".into()),
        calories: Some(400 + random_value),
        time: Some("2022 March 2 18:00".into()),
    });

    ret
}

pub async fn add_foods(
    api_client: &mut crate::ApiClient,
    access_token: &str,
    foods: &[AddFoodRequest],
) -> Vec<String> {
    let mut ids = Vec::new();
    for food in foods.iter() {
        ids.push(
            api_client
                .add_food(access_token, &food)
                .await
                .unwrap()
                .object
                .id,
        );
    }
    ids
}

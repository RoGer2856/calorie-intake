use crate::api::food::messages::*;

pub fn food_array_contains_food(foods: &[Food], food: &AddFoodRequest) -> bool {
    foods
        .iter()
        .find(|item| **item == Food::from_partial_food(item.id.clone(), food.clone()))
        .is_some()
}

pub fn check_food_array_equality(expected: &[AddFoodRequest], received: &[Food]) -> bool {
    for food in expected.iter() {
        if !food_array_contains_food(received, food) {
            return false;
        }
    }

    true
}

pub fn generate_example_foods() -> Vec<AddFoodRequest> {
    let mut ret = Vec::new();

    ret.push(AddFoodRequest {
        name: "Hamburger".into(),
        calories: 600,
        time: "2022 March 2 8:0".into(),
    });

    ret.push(AddFoodRequest {
        name: "Chicken".into(),
        calories: 300,
        time: "2022 March 2 12:00".into(),
    });

    ret.push(AddFoodRequest {
        name: "Scrambled eggs".into(),
        calories: 400,
        time: "2022 March 2 18:00".into(),
    });

    ret
}

pub async fn add_foods(
    api_client: &mut crate::ApiClient,
    access_token: &str,
    foods: &[AddFoodRequest],
) {
    for food in foods.iter() {
        api_client.add_food(access_token, &food).await.unwrap();
    }
}

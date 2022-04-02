use crate::api::food::messages::*;

pub fn food_array_contains_food(foods: &[Food], food: &Food) -> bool {
    foods.iter().find(|item| **item == *food).is_some()
}

pub fn check_food_array_equality(foods0: &[Food], foods1: &[Food]) -> bool {
    for food in foods0.iter() {
        if !food_array_contains_food(foods1, food) {
            return false;
        }
    }

    true
}

pub fn generate_example_foods() -> Vec<Food> {
    let mut ret = Vec::new();

    ret.push(Food {
        name: "Hamburger".into(),
        calorie: 600,
        time: "2022 March 2 8:0".into(),
    });

    ret.push(Food {
        name: "Chicken".into(),
        calorie: 300,
        time: "2022 March 2 12:00".into(),
    });

    ret.push(Food {
        name: "Scrambled eggs".into(),
        calorie: 400,
        time: "2022 March 2 18:00".into(),
    });

    ret
}

pub async fn add_foods(api_client: &mut crate::ApiClient, access_token: String, foods: &[Food]) {
    for food in foods.iter() {
        api_client
            .add_food(&AddFoodRequest {
                access_token: access_token.clone(),
                food: food.clone(),
            })
            .await
            .unwrap();
    }
}

use crate::api::food::messages::*;

pub fn food_array_contains_food(foods: &[Food], food: &PartialFood) -> bool {
    foods
        .iter()
        .find(|item| **item == Food::from_partial_food(item.id.clone(), food.clone()))
        .is_some()
}

pub fn check_food_array_equality(expected: &[PartialFood], received: &[Food]) -> bool {
    for food in expected.iter() {
        if !food_array_contains_food(received, food) {
            return false;
        }
    }

    true
}

pub fn generate_example_foods() -> Vec<PartialFood> {
    let mut ret = Vec::new();

    ret.push(PartialFood {
        name: "Hamburger".into(),
        calories: 600,
        time: "2022 March 2 8:0".into(),
    });

    ret.push(PartialFood {
        name: "Chicken".into(),
        calories: 300,
        time: "2022 March 2 12:00".into(),
    });

    ret.push(PartialFood {
        name: "Scrambled eggs".into(),
        calories: 400,
        time: "2022 March 2 18:00".into(),
    });

    ret
}

pub async fn add_foods(
    api_client: &mut crate::ApiClient,
    access_token: String,
    foods: &[PartialFood],
) {
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

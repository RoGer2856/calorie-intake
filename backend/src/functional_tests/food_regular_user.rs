use crate::api::food::messages::*;
use crate::services::*;

#[tokio::test]
#[serial_test::serial]
async fn no_food() {
    let address = (crate::functional_tests::IPV6_LOCALHOST, 4000).into();
    crate::functional_tests::test_utils::run_test(address, async {
        let mut api_client =
            crate::api_client::ApiClient::new(&("http://".to_string() + &address.to_string()));

        let authorization = DietAuthorization::new("config_file".into()).unwrap();
        let access_token = authorization
            .create_jwt("john".into(), RoleType::RegularUser)
            .unwrap();

        let resp = api_client
            .get_food_list(&GetFoodListRequest {
                access_token: access_token.clone(),
            })
            .await
            .unwrap();
        assert_eq!(0, resp.object.foods.len());
    })
    .await;
}

fn food_array_contains_food(foods: &[Food], food: &Food) -> bool {
    foods.iter().find(|item| **item == *food).is_some()
}

fn check_food_array_equality(foods0: &[Food], foods1: &[Food]) -> bool {
    for food in foods0.iter() {
        if !food_array_contains_food(foods1, food) {
            return false;
        }
    }

    true
}

fn generate_example_foods() -> Vec<Food> {
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

async fn add_foods(api_client: &mut crate::ApiClient, access_token: String, foods: &[Food]) {
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

#[tokio::test]
#[serial_test::serial]
async fn get_multiple_foods() {
    let address = (crate::functional_tests::IPV6_LOCALHOST, 4000).into();
    crate::functional_tests::test_utils::run_test(address, async {
        let mut api_client =
            crate::api_client::ApiClient::new(&("http://".to_string() + &address.to_string()));

        let authorization = DietAuthorization::new("config_file".into()).unwrap();
        let access_token = authorization
            .create_jwt("john".into(), RoleType::RegularUser)
            .unwrap();

        let foods = generate_example_foods();

        add_foods(&mut api_client, access_token.clone(), &foods).await;

        let resp = api_client
            .get_food_list(&GetFoodListRequest {
                access_token: access_token,
            })
            .await
            .unwrap();

        assert_eq!(foods.len(), resp.object.foods.len());
        assert!(check_food_array_equality(&foods, &resp.object.foods));
    })
    .await;
}

#[tokio::test]
#[serial_test::serial]
async fn multiple_user_foods() {
    let address = (crate::functional_tests::IPV6_LOCALHOST, 4000).into();
    crate::functional_tests::test_utils::run_test(address, async {
        let mut api_client =
            crate::api_client::ApiClient::new(&("http://".to_string() + &address.to_string()));

        let authorization = DietAuthorization::new("config_file".into()).unwrap();

        let access_token0 = authorization
            .create_jwt("john".into(), RoleType::RegularUser)
            .unwrap();

        let access_token1 = authorization
            .create_jwt("jane".into(), RoleType::RegularUser)
            .unwrap();

        let foods = generate_example_foods();

        add_foods(&mut api_client, access_token0.clone(), &foods).await;
        add_foods(&mut api_client, access_token1.clone(), &foods).await;

        // check the list of foods for access_token0
        {
            let resp = api_client
                .get_food_list(&GetFoodListRequest {
                    access_token: access_token0,
                })
                .await
                .unwrap();

            assert_eq!(foods.len(), resp.object.foods.len());
            assert!(check_food_array_equality(&foods, &resp.object.foods));
        }

        // check the list of foods for access_token1
        {
            let resp = api_client
                .get_food_list(&GetFoodListRequest {
                    access_token: access_token1,
                })
                .await
                .unwrap();

            assert_eq!(foods.len(), resp.object.foods.len());
            assert!(check_food_array_equality(&foods, &resp.object.foods));
        }
    })
    .await;
}

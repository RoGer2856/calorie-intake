extern crate diet;

use diet::api::food::messages::AddFoodRequest;
use diet::Config;

fn create_food_for_the_last_n_days(days: i64) -> Vec<AddFoodRequest> {
    let mut ret = Vec::new();

    let day_start = diet::utils::time::current_day_start_local();

    for i in 0..days {
        let mut calories_for_the_day = rand::random::<u16>() % 1000;
        calories_for_the_day += 1600;

        let day_start = day_start - chrono::Duration::days(days - i);

        let breakfast = AddFoodRequest {
            id: None,
            name: Some("scrambled eggs".into()),
            calories: Some(calories_for_the_day / 4 + 100),
            time: (day_start + chrono::Duration::hours(7)).into(),
        };

        let lunch = AddFoodRequest {
            id: None,
            name: Some("grilled chicken".into()),
            calories: Some(calories_for_the_day / 2),
            time: (day_start + chrono::Duration::hours(12)).into(),
        };

        let dinner = AddFoodRequest {
            id: None,
            name: Some("slice of pizza".into()),
            calories: Some(calories_for_the_day / 4 - 100),
            time: (day_start + chrono::Duration::hours(18)).into(),
        };

        ret.push(breakfast);
        ret.push(lunch);
        ret.push(dinner);
    }

    ret
}

pub async fn add_foods(
    api_client: &mut diet::ApiClient,
    access_token: String,
    username: &str,
    foods: &[AddFoodRequest],
) {
    for food in foods.iter() {
        api_client
            .add_food(&access_token, username, &food)
            .await
            .unwrap();
    }
}

pub fn main() {
    env_logger::init();
    log::info!("Starting the application");

    let config = Box::new(diet::CommandLineArgsConfig::new());
    let listener_address = config.get_listener_address();
    let secrets_file_location = config.get_secrets_file_location().clone();

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        let application = diet::Application::spawn_start(config).await;

        {
            // populating server with data
            let mut api_client =
                diet::ApiClient::new(&("http://".to_string() + &listener_address.to_string()));

            let authorization =
                diet::services::DietAuthorization::new(secrets_file_location).unwrap();

            let access_token_jane = authorization
                .create_jwt("jane".into(), diet::services::RoleType::RegularUser, 2100)
                .unwrap();
            let access_token_john = authorization
                .create_jwt("john".into(), diet::services::RoleType::RegularUser, 2100)
                .unwrap();
            let _access_token_admin = authorization
                .create_jwt("admin".into(), diet::services::RoleType::RegularUser, 2100)
                .unwrap();

            let foods = create_food_for_the_last_n_days(70);
            add_foods(&mut api_client, access_token_jane, "jane", &foods).await;

            let foods = create_food_for_the_last_n_days(2);
            add_foods(&mut api_client, access_token_john, "john", &foods).await;
        }

        tokio::signal::ctrl_c().await.unwrap();

        application.halt().await;
    });
}

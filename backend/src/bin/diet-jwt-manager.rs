extern crate diet;

pub fn main() {
    let matches = clap::Command::new("diet-jwt-manager")
        .version("0.1.0")
        .author("Gergő Róth <roth@bytifex.com>")
        .about("With this tool one can manage JWT tokens for the diet-app")
        .subcommand_required(true)
        .subcommand(
            clap::Command::new("create")
                .about("Creates a new JWT")
                .arg(
                    clap::Arg::new("secret-file")
                        .short('s')
                        .long("secret-file")
                        .value_name("FILE")
                        .help("Reads the secrets from the given config file (toml)")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    clap::Arg::new("role")
                        .short('r')
                        .long("role")
                        .value_name("ROLE")
                        .help("Sets the role for the JWT.")
                        .possible_value("admin")
                        .possible_value("regular_user")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    clap::Arg::new("username")
                        .short('u')
                        .long("username")
                        .value_name("USERNAME")
                        .help("Sets the username for the JWT")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    clap::Arg::new("max_calories")
                        .short('c')
                        .long("max_calories")
                        .value_name("MAX_CALORIES_PER_DAY")
                        .help("Sets the maximum calories for the user")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("create") {
        let secrets_file_location = matches.value_of("secret-file").unwrap();
        let role = matches.value_of("role").unwrap();
        let username = matches.value_of("username").unwrap();
        let max_calories_per_day = matches.value_of("max_calories").unwrap();

        let diet_authorization =
            diet::services::DietAuthorization::new(secrets_file_location.into()).unwrap();

        let jwt = diet_authorization
            .create_jwt(
                username.to_string(),
                diet::services::RoleType::try_from(role).unwrap(),
                max_calories_per_day.parse().unwrap(),
            )
            .unwrap();

        println!("{}", jwt);
    }
}

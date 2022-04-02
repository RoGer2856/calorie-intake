pub trait Config {
    fn get_listener_address(&self) -> std::net::SocketAddr;
    fn set_listener_address(&mut self, listener_address: std::net::SocketAddr);
    fn get_secrets_file_location(&self) -> &std::path::PathBuf;
}

pub struct CommandLineArgsConfig {
    listener_address: std::net::SocketAddr,
    secrets_file_location: std::path::PathBuf,
}

impl CommandLineArgsConfig {
    pub fn new() -> Self {
        let matches = clap::Command::new("cmap-backend")
            .version("0.1.0")
            .author("Gergő Róth <roth@bytifex.com>")
            .about("This is the backend server for CMap")
            .arg(
                clap::Arg::new("listener-address")
                    .short('l')
                    .long("listener-address")
                    .value_name("LISTENER-ADDRESS")
                    .help("Sets the address where the server accepts the incoming connections")
                    .required(false)
                    .default_value("[::1]:3001")
                    .takes_value(true),
            )
            .arg(
                clap::Arg::new("secret-file")
                    .short('s')
                    .long("secret-file")
                    .value_name("FILE")
                    .help("Reads the secrets from the given config file (toml)")
                    .required(true)
                    .takes_value(true),
            )
            .get_matches();

        let secrets_file_location = matches.value_of("secret-file").unwrap();

        let listener_address = matches.value_of("listener-address").unwrap().to_string();

        Self {
            listener_address: listener_address.parse().unwrap(),
            secrets_file_location: secrets_file_location.into(),
        }
    }
}

impl Config for CommandLineArgsConfig {
    fn get_listener_address(&self) -> std::net::SocketAddr {
        self.listener_address
    }

    fn set_listener_address(&mut self, listener_address: std::net::SocketAddr) {
        self.listener_address = listener_address;
    }

    fn get_secrets_file_location(&self) -> &std::path::PathBuf {
        &self.secrets_file_location
    }
}

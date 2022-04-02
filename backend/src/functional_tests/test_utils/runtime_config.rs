pub struct RuntimeConfig {
    listener_address: std::net::SocketAddr,
    secrets_file_location: std::path::PathBuf,
}

impl RuntimeConfig {
    pub fn new(
        listener_address: std::net::SocketAddr,
        secrets_file_location: std::path::PathBuf,
    ) -> Self {
        Self {
            listener_address,
            secrets_file_location,
        }
    }
}

impl crate::Config for RuntimeConfig {
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

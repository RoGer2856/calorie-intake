pub struct RuntimeConfig {
    listener_address: std::net::SocketAddr,
}

impl RuntimeConfig {
    pub fn new(listener_address: std::net::SocketAddr) -> Self {
        Self { listener_address }
    }
}

impl crate::Config for RuntimeConfig {
    fn get_listener_address(&self) -> std::net::SocketAddr {
        self.listener_address
    }

    fn set_listener_address(&mut self, listener_address: std::net::SocketAddr) {
        self.listener_address = listener_address;
    }
}

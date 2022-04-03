mod food_regular_user;
mod userinfo;

pub mod test_utils;

const IPV6_LOCALHOST: std::net::Ipv6Addr = std::net::Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1);
const SECRETS_FILE_LOCATION: &str = "test_resources/test_secrets.toml";

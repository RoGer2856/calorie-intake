use crate::utils::LogError;

use std::str::FromStr;

#[derive(serde::Serialize, serde::Deserialize)]
struct SecretConfig {
    secret_key: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum RoleType {
    Admin,
    RegularUser,
}

impl std::string::ToString for RoleType {
    fn to_string(&self) -> String {
        match self {
            RoleType::Admin => "admin".to_string(),
            RoleType::RegularUser => "regular_user".to_string(),
        }
    }
}

impl std::str::FromStr for RoleType {
    type Err = DietAuthorizationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "admin" => Ok(RoleType::Admin),
            "regular_user" => Ok(RoleType::RegularUser),
            _ => Err(DietAuthorizationError::InvalidRoleStringError(
                s.to_string(),
            )),
        }
    }
}

#[derive(Debug)]
pub enum DietAuthorizationError {
    StdIoError(std::io::Error),
    JwtManagerError(crate::utils::JwtError),
    TomlDeserializeError(toml::de::Error),
    InvalidRoleStringError(String),
    InvalidJwtReceivedError,
    ParseIntError,
}

impl From<std::io::Error> for DietAuthorizationError {
    fn from(e: std::io::Error) -> Self {
        Self::StdIoError(e)
    }
}

impl From<crate::utils::JwtError> for DietAuthorizationError {
    fn from(e: crate::utils::JwtError) -> Self {
        Self::JwtManagerError(e)
    }
}

impl From<toml::de::Error> for DietAuthorizationError {
    fn from(e: toml::de::Error) -> Self {
        Self::TomlDeserializeError(e)
    }
}
impl From<std::num::ParseIntError> for DietAuthorizationError {
    fn from(_: std::num::ParseIntError) -> Self {
        Self::ParseIntError
    }
}

pub struct DietAuthorization {
    jwt_manager: crate::utils::JwtManager,
}

pub struct AuthorizationInfo {
    pub username: String,
    pub role: RoleType,
    pub max_calories_per_day: u16,
}

impl DietAuthorization {
    pub fn new(secret_config_file: std::path::PathBuf) -> Result<Self, DietAuthorizationError> {
        let secret_config: SecretConfig = toml::from_str(
            &std::fs::read_to_string(&secret_config_file)
                .log_error(|| log::error!("Could not read secrets config file"))?,
        )
        .log_error(|| log::error!("Could not parse secrets config file"))?;

        Ok(Self {
            jwt_manager: crate::utils::JwtManager::new(secret_config.secret_key)
                .log_error(|| log::error!("Could not create JwtManager"))?,
        })
    }

    pub fn create_jwt(
        &self,
        username: String,
        role: RoleType,
        max_calories_per_day: u16,
    ) -> Result<String, DietAuthorizationError> {
        let mut claims = std::collections::BTreeMap::new();
        claims.insert("username".to_string(), username);
        claims.insert("role".to_string(), role.to_string());
        claims.insert(
            "max_calories_per_day".to_string(),
            max_calories_per_day.to_string(),
        );
        Ok(self
            .jwt_manager
            .create_token(&claims)
            .log_error(|| log::error!("Could not create jwt"))?)
    }

    pub fn verify_jwt(&self, jwt: &str) -> Result<AuthorizationInfo, DietAuthorizationError> {
        let claims = self.jwt_manager.get_verified_claims(jwt)?;
        let username = claims
            .get("username")
            .ok_or(DietAuthorizationError::InvalidJwtReceivedError)?;
        let role = claims
            .get("role")
            .ok_or(DietAuthorizationError::InvalidJwtReceivedError)?;
        let max_calories_per_day = claims
            .get("max_calories_per_day")
            .ok_or(DietAuthorizationError::InvalidJwtReceivedError)?;

        Ok(AuthorizationInfo {
            username: username.clone(),
            role: RoleType::from_str(&role)?,
            max_calories_per_day: max_calories_per_day.parse()?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_jwt(role: RoleType) {
        let authorization =
            DietAuthorization::new("test_resources/test_secrets.toml".into()).unwrap();

        let username = "username".to_string();
        let jwt = authorization
            .create_jwt(username.clone(), role.clone(), 2100)
            .unwrap();
        let authz_info = authorization.verify_jwt(&jwt).unwrap();
        assert_eq!(authz_info.role, role);
        assert_eq!(authz_info.username, username);
    }

    #[test]
    fn test_admin_jwt() {
        test_jwt(RoleType::Admin);
    }

    #[test]
    fn test_regular_user_jwt() {
        test_jwt(RoleType::RegularUser);
    }

    #[test]
    fn invalid_secret_config_file() {
        assert!(DietAuthorization::new("non existing file".into()).is_err());
        assert!(DietAuthorization::new("test_resources/invalid_secrets.toml".into()).is_err());
    }
}

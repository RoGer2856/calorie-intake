use crate::utils::LogError;

#[derive(serde::Serialize, serde::Deserialize)]
struct SecretConfig {
    secret_key: String,
}

#[derive(Debug)]
pub struct CouldNotParseRoleError(pub String);

impl std::fmt::Display for CouldNotParseRoleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CouldNotParseRoleError on string {}", self.0)
    }
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
#[serde(try_from = "String", into = "String")]
pub enum RoleType {
    Admin,
    RegularUser,
}

impl Into<String> for RoleType {
    fn into(self) -> String {
        match self {
            RoleType::Admin => "admin".into(),
            RoleType::RegularUser => "regular_user".into(),
        }
    }
}

impl TryFrom<String> for RoleType {
    type Error = CouldNotParseRoleError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "admin" => Ok(RoleType::Admin),
            "regular_user" => Ok(RoleType::RegularUser),
            _ => Err(CouldNotParseRoleError(value)),
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
    CouldNotParseRoleError(String),
}

impl From<CouldNotParseRoleError> for DietAuthorizationError {
    fn from(e: CouldNotParseRoleError) -> Self {
        match e {
            CouldNotParseRoleError(v) => DietAuthorizationError::CouldNotParseRoleError(v),
        }
    }
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

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Clone)]
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
        claims.insert("role".to_string(), role.into());
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
            role: RoleType::try_from(role.clone())?,
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

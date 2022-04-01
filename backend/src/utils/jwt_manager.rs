use crate::utils::LogError;

#[derive(Debug)]
pub enum JwtError {
    InvalidSecretSize,
    InvalidJwtReceived,
    JwtMacError,
}

impl From<crypto_common::InvalidLength> for JwtError {
    fn from(_e: crypto_common::InvalidLength) -> Self {
        JwtError::InvalidSecretSize
    }
}

impl From<jwt::Error> for JwtError {
    fn from(e: jwt::Error) -> Self {
        match e {
            jwt::Error::AlgorithmMismatch(_, _) => JwtError::InvalidJwtReceived,
            jwt::Error::Base64(_) => JwtError::InvalidJwtReceived,
            jwt::Error::Format => JwtError::InvalidJwtReceived,
            jwt::Error::InvalidSignature => JwtError::InvalidJwtReceived,
            jwt::Error::Json(_) => JwtError::InvalidJwtReceived,
            jwt::Error::NoClaimsComponent => JwtError::InvalidJwtReceived,
            jwt::Error::NoHeaderComponent => JwtError::InvalidJwtReceived,
            jwt::Error::NoKeyId => JwtError::InvalidJwtReceived,
            jwt::Error::NoKeyWithKeyId(_) => JwtError::InvalidJwtReceived,
            jwt::Error::NoSignatureComponent => JwtError::InvalidJwtReceived,
            jwt::Error::RustCryptoMac(_) => JwtError::JwtMacError,
            jwt::Error::RustCryptoMacKeyLength(_) => JwtError::JwtMacError,
            jwt::Error::TooManyComponents => JwtError::InvalidJwtReceived,
            jwt::Error::Utf8(_) => JwtError::InvalidJwtReceived,
        }
    }
}

pub struct JwtManager {
    secret_key: String,
}

impl JwtManager {
    pub fn new(secret_key: String) -> Result<Self, JwtError> {
        let ret = Self { secret_key };

        // this is to verify the setup in construction time
        let mut claims = std::collections::BTreeMap::new();
        claims.insert("key".to_string(), "value".to_string());
        let jwt = ret.create_token(&claims)?;
        ret.get_verified_claims(&jwt)?;

        Ok(ret)
    }

    fn create_token(
        &self,
        claims: &std::collections::BTreeMap<String, String>,
    ) -> Result<String, JwtError> {
        use hmac::Mac;
        use jwt::SignWithKey;

        let key: hmac::Hmac<sha2::Sha256> = hmac::Hmac::new_from_slice(self.secret_key.as_bytes())
            .log_error(|| log::error!("Invalid secret size when creating a jwt"))?;

        Ok(claims
            .sign_with_key(&key)
            .log_error(|| log::error!("Could not create jwt"))?)
    }

    fn get_verified_claims(
        &self,
        jwt: &str,
    ) -> Result<std::collections::BTreeMap<String, String>, JwtError> {
        use hmac::Mac;
        use jwt::VerifyWithKey;

        let key: hmac::Hmac<sha2::Sha256> = hmac::Hmac::new_from_slice(self.secret_key.as_bytes())
            .log_error(|| log::error!("Invalid secret size when creating a jwt"))?;

        let claims: std::collections::BTreeMap<String, String> = jwt
            .verify_with_key(&key)
            .log_error(|| log::error!("Could not verify jwt"))?;

        Ok(claims)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_claims() {
        let jwt_manager = JwtManager::new("this is my ultimate secret key".to_string()).unwrap();

        let claims = std::collections::BTreeMap::new();
        let jwt = jwt_manager.create_token(&claims).unwrap();
        jwt_manager.get_verified_claims(&jwt).unwrap();
    }

    #[test]
    fn cross_jwt_verification() {
        let jwt_manager0 = JwtManager::new("secret0".to_string()).unwrap();
        let jwt_manager1 = JwtManager::new("secret1".to_string()).unwrap();

        let claims = std::collections::BTreeMap::new();
        let jwt = jwt_manager0.create_token(&claims).unwrap();
        assert!(jwt_manager1.get_verified_claims(&jwt).is_err());
    }

    #[test]
    fn multiple_claims() {
        let jwt_manager = JwtManager::new("this is my ultimate secret key".to_string()).unwrap();

        let mut claims = std::collections::BTreeMap::new();
        for i in 0..20 {
            claims.insert(i.to_string(), i.to_string());
        }
        let jwt = jwt_manager.create_token(&claims).unwrap();

        let verified_claims = jwt_manager.get_verified_claims(&jwt).unwrap();
        assert_eq!(verified_claims.len(), claims.len());
        for claim in verified_claims.iter() {
            assert_eq!(claims.get(claim.0).unwrap(), claim.0);
        }
    }
}

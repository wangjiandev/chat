use std::ops::Deref;

use jwt_simple::prelude::*;

use crate::{AppError, User};

pub struct EncodingKey(Ed25519KeyPair);
pub struct DecodingKey(Ed25519PublicKey);

const JWT_DURATION: u64 = 60 * 60 * 24 * 7; // 7 days
const ISSUER: &str = "chat_server";
const AUDIENCE: &str = "chat_web";

impl EncodingKey {
    pub fn load(pem: &str) -> Result<Self, AppError> {
        Ok(Self(Ed25519KeyPair::from_pem(pem)?))
    }

    pub fn sign(&self, user: impl Into<User>) -> Result<String, AppError> {
        let claims = Claims::with_custom_claims(user.into(), Duration::from_secs(JWT_DURATION));
        let claims = claims.with_issuer(ISSUER).with_audience(AUDIENCE);
        Ok(self.0.sign(claims)?)
    }
}

impl Deref for EncodingKey {
    type Target = Ed25519KeyPair;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DecodingKey {
    pub fn load(pem: &str) -> Result<Self, AppError> {
        Ok(Self(Ed25519PublicKey::from_pem(pem)?))
    }

    pub fn verify(&self, token: &str) -> Result<User, AppError> {
        let options = VerificationOptions {
            allowed_audiences: Some(HashSet::from_strings(&[AUDIENCE])),
            allowed_issuers: Some(HashSet::from_strings(&[ISSUER])),
            ..Default::default()
        };
        let claims = self.0.verify_token::<User>(token, Some(options))?;
        Ok(claims.custom)
    }
}

impl Deref for DecodingKey {
    type Target = Ed25519PublicKey;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn jwt_sign_verify_should_work() -> Result<()> {
        let encoding_pem = include_str!("../../fixtures/encoding.pem");
        let decoding_pem = include_str!("../../fixtures/decoding.pem");
        let ek = EncodingKey::load(encoding_pem)?;
        let dk = DecodingKey::load(decoding_pem)?;

        let user_source = User::new(1, "test".to_string(), "test@test.com".to_string());
        let token = ek.sign(user_source.clone())?;
        let user = dk.verify(&token)?;
        assert_eq!(user, user_source);
        Ok(())
    }
}

use std::{sync::Arc, time::SystemTime};

use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header, Validation};
use rsa::{
    pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey},
    pkcs8::LineEnding,
    RsaPrivateKey, RsaPublicKey,
};
use serde::{Deserialize, Serialize};

use super::time;

const EXPIRATION_WITHIN_SEC: u64 = 60 * 5;
const ONE_HOUR_SEC: u64 = 60 * 60;
const RSA_BITS: usize = 2048;

#[derive(Debug, thiserror::Error)]
pub enum ErrorJwt {
    #[error("Failed to generate JWT service: {0}")]
    Generate(#[from] rsa::errors::Error),
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Claims {
    pub iat: u64,
    pub exp: u64,
    pub sub: u64,
    pub display_name: String,
    pub email: String,
}

impl Claims {
    pub fn new(sub: u64, display_name: impl Into<String>, email: impl Into<String>) -> Self {
        let now = time::now_unix().unwrap();
        let iat = now;
        let exp = now + ONE_HOUR_SEC;

        Self {
            iat,
            exp,
            sub,
            display_name: display_name.into(),
            email: email.into(),
        }
    }
}

pub struct Jwt {
    public_key: RsaPublicKey,
    decoding_key: DecodingKey,

    private_key: RsaPrivateKey,
    encoding_key: EncodingKey,

    validation: Validation,
}

impl std::fmt::Debug for Jwt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Jwt")
            .field("public_key", &self.public_key)
            .field("private_key", &self.private_key)
            .field("validation", &self.validation)
            .finish()
    }
}

impl Jwt {
    pub fn generate() -> Result<Self, ErrorJwt> {
        let mut rng = rand::thread_rng();
        let private_key = RsaPrivateKey::new(&mut rng, RSA_BITS)?;
        private_key.validate()?;
        Ok(Self::new(private_key))
    }

    pub fn new(private_key: RsaPrivateKey) -> Self {
        let private_pem = private_key
            .to_pkcs1_pem(LineEnding::LF)
            .expect("Failed to serialize private key as PEM");

        let public_key = RsaPublicKey::from(&private_key);
        let public_pem = public_key
            .to_pkcs1_pem(LineEnding::LF)
            .expect("Failed to serialize public key as PEM");

        let mut validation = Validation::new(jsonwebtoken::Algorithm::RS384);
        validation.validate_exp = true;
        validation.reject_tokens_expiring_in_less_than = EXPIRATION_WITHIN_SEC;

        Self {
            public_key,
            decoding_key: DecodingKey::from_rsa_pem(public_pem.as_bytes())
                .expect("Failed to create decoding key from public key"),
            private_key,
            encoding_key: EncodingKey::from_rsa_pem(private_pem.as_bytes())
                .expect("Failed to create encoding key from private key"),
            validation,
        }
    }

    pub fn sign(&self, claims: &Claims) -> Result<String, String> {
        let header = Header::new(jsonwebtoken::Algorithm::RS384);
        let token = encode(&header, &claims, &self.encoding_key).map_err(|e| e.to_string())?;
        Ok(token)
    }

    pub fn verify(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let result = jsonwebtoken::decode::<Claims>(token, &self.decoding_key, &self.validation);
        Ok(result?.claims)
    }
}

#[derive(Debug, Clone)]
pub struct JwtController {
    jwt: Arc<Jwt>,
}

impl JwtController {
    pub(crate) fn new() -> Result<Self, ErrorJwt> {
        Ok(Self {
            jwt: Arc::new(Jwt::generate()?),
        })
    }

    pub fn sign(&self, claims: &Claims) -> Result<String, String> {
        self.jwt.sign(claims)
    }

    pub fn verify(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        self.jwt.verify(token)
    }
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::service::jwt::Claims;

    use super::Jwt;

    use jsonwebtoken::errors::ErrorKind;
    use lazy_static::lazy_static;

    lazy_static! {
        /// A Lazy static instance to use the same key across all tests
        static ref JWT: Jwt = Jwt::generate().unwrap();
    }

    #[test]
    fn round_trip_a_token() {
        let claims = Claims::new(1, "Someone".to_string(), "someone@contoso.com".to_string());

        let token = JWT.sign(&claims).expect("Failed to sign token");
        let verified_claims = JWT.verify(&token).expect("Failed to verify token");
        assert_eq!(claims, verified_claims);
    }

    #[test]
    fn token_expiring_soon_is_invalid() {
        let now = SystemTime::now();
        let iat = now.duration_since(UNIX_EPOCH).unwrap().as_secs();
        let exp = iat + 10; // expires 10 seconds from now
        let claims = Claims {
            iat,
            exp,
            sub: 1,
            display_name: "Someone".to_string(),
            email: "someone@contoso.com".to_string(),
        };

        let token = JWT.sign(&claims).expect("Failed to sign token");
        let result = JWT
            .verify(&token)
            .expect_err("Token should be invalid and expiring soon!");
        assert_eq!(result.kind(), &ErrorKind::ExpiredSignature);
    }
}

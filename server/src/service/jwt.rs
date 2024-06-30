use std::time::SystemTime;

use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header};
use rsa::{
    pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey},
    pkcs8::LineEnding,
    RsaPrivateKey, RsaPublicKey,
};
use serde::{Deserialize, Serialize};

const ONE_MINUTE_SEC: u64 = 60;
const ONE_HOUR_SEC: u64 = 3600;
const RSA_BITS: usize = 2048;
const RSA_ALGORITHM: &str = "RS384";

pub struct Jwt {
    public_key: RsaPublicKey,
    decoding_key: DecodingKey,

    private_key: RsaPrivateKey,
    encoding_key: EncodingKey,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Claims {
    pub iat: u64,
    pub exp: u64,
    pub sub: String,
    pub display_name: String,
    pub email: String,
}

impl Claims {
    pub fn new(
        sub: impl Into<String>,
        display_name: impl Into<String>,
        email: impl Into<String>,
    ) -> Self {
        let now = SystemTime::now();
        let iat = now
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Failed to get system time! (Time went backwards?)")
            .as_secs();

        let exp = (now + std::time::Duration::from_secs(ONE_HOUR_SEC))
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Failed to get system time! (Time went backwards?)")
            .as_secs();

        Self {
            iat,
            exp,
            sub: sub.into(),
            display_name: display_name.into(),
            email: email.into(),
        }
    }
}

impl Jwt {
    pub fn generate() -> Result<Self, rsa::errors::Error> {
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

        Self {
            public_key,
            decoding_key: DecodingKey::from_rsa_pem(public_pem.as_bytes())
                .expect("Failed to create decoding key from public key"),
            private_key,
            encoding_key: EncodingKey::from_rsa_pem(private_pem.as_bytes())
                .expect("Failed to create encoding key from private key"),
        }
    }

    pub fn sign(&self, claims: &Claims) -> Result<String, String> {
        let header = Header::new(jsonwebtoken::Algorithm::RS384);
        let token = encode(&header, &claims, &self.encoding_key).map_err(|e| e.to_string())?;
        Ok(token)
    }

    pub fn verify(&self, token: &str) -> Result<Claims, String> {
        let decoding_key = &self.decoding_key;
        let result = jsonwebtoken::decode::<Claims>(
            token,
            decoding_key,
            &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS384),
        );
        match result {
            Ok(token_data) => Ok(token_data.claims),
            Err(err) => Err(err.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::service::jwt::Claims;

    use super::Jwt;

    use lazy_static::lazy_static;

    lazy_static! {
        /// A Lazy static instance to use the same key across all tests
        static ref JWT: Jwt = Jwt::generate().unwrap();
    }

    #[test]
    fn round_trip_a_token() {
        let claims = Claims::new(
            "someone".to_string(),
            "Someone".to_string(),
            "foobar@contoso.com".to_string(),
        );

        let token = JWT.sign(&claims).expect("Failed to sign token");

        let verified_claims = JWT.verify(&token).expect("Failed to verify token");

        assert_eq!(claims, verified_claims);
    }
}

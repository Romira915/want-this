use anyhow::{bail, Context, Ok, Result};
use jsonwebtoken::{
    jwk::{self, JwkSet},
    DecodingKey, Validation,
};
use serde::{Deserialize, Serialize};

const GOOGLE_OAUTH2_CERTS_URL: &'static str = "https://www.googleapis.com/oauth2/v3/certs";

#[derive(Debug, Serialize, Deserialize)]
pub struct GooglePayload {
    pub iss: String,
    pub nbf: u32,
    pub aud: String,
    pub sub: String,
    pub hd: Option<String>,
    pub email: String,
    pub email_verified: bool,
    pub name: String,
    pub picture: String,
    pub given_name: String,
    pub family_name: String,
    pub iat: u32,
    pub exp: u32,
    pub jti: String,
}

pub fn decode_google_jwt_with_jwkset(token: &str, public_keys: JwkSet) -> Result<GooglePayload> {
    let header = jsonwebtoken::decode_header(token)
        .with_context(|| format!("Failed to decode header {}", token))?;
    let kid = header.kid.unwrap();

    let decoded_token = if let Some(jwk) = public_keys.find(&kid) {
        match jwk.algorithm {
            jwk::AlgorithmParameters::RSA(ref rsa) => {
                let decoding_key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e)
                    .with_context(|| format!("Failed to DecodingKey {:?}", rsa))?;
                let mut validation =
                    Validation::new(jwk.common.algorithm.context("jwk algorithm is None")?);
                validation.validate_exp = true;
                let decoded_token =
                    jsonwebtoken::decode::<GooglePayload>(token, &decoding_key, &validation)
                        .with_context(|| format!("Failed to decode token {}", token))?;
                decoded_token.claims
            }
            _ => bail!("this should be a RSA"),
        }
    } else {
        bail!("Not Found publick key");
    };

    Ok(decoded_token)
}

pub async fn decode_google_jwt_with_jwturl(token: &str) -> Result<GooglePayload> {
    let resp = reqwest::get(GOOGLE_OAUTH2_CERTS_URL)
        .await
        .with_context(|| format!("Failed to request url: {}", GOOGLE_OAUTH2_CERTS_URL))?;
    let public_keys: jwk::JwkSet = resp.json().await.unwrap();

    decode_google_jwt_with_jwkset(token, public_keys)
}

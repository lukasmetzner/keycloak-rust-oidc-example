use std::error::Error;

use jsonwebtoken::{decode, Algorithm, DecodingKey, TokenData, Validation};
use serde::Deserialize;

use crate::{errors::AlgNotFoundError, oidc::Claims};

#[derive(Deserialize, Clone, Debug)]
struct Jwks {
    pub keys: Vec<Jwk>,
}

#[derive(Deserialize, Clone, Debug)]
struct Jwk {
    kid: String,
    kty: String,
    alg: String,
    n: String,
    e: String,
}

async fn get_rs256_public_key(issuer_url: &str) -> Result<Jwk, Box<dyn Error>> {
    let jwks_url = format!("{}/protocol/openid-connect/certs", issuer_url);
    let jwks: Jwks = reqwest::get(&jwks_url).await?.json().await?;
    Ok(jwks
        .keys
        .into_iter()
        .filter(|key| key.alg.eq("RS256"))
        .collect::<Vec<Jwk>>()
        .first()
        .ok_or(AlgNotFoundError::new("RS256"))?
        .clone())
}

fn get_rsa_decoding_key(jwk: &Jwk) -> Result<DecodingKey, Box<dyn Error>> {
    let n = String::from_utf8(jwk.n.as_bytes().to_vec())?;
    let e = String::from_utf8(jwk.e.as_bytes().to_vec())?;
    Ok(DecodingKey::from_rsa_components(&n, &e)?)
}

pub async fn verify_token(
    token: &str,
    issuer_url: &str,
    client_id: &str,
) -> Result<TokenData<Claims>, Box<dyn Error>> {
    let jwk = get_rs256_public_key(issuer_url).await?;
    let decoding_key = get_rsa_decoding_key(&jwk)?;
    let mut validation = Validation::new(Algorithm::RS256);
    // aud value
    validation.set_audience(&[client_id]);
    // iss value
    validation.set_issuer(&[issuer_url]);

    let token_data = decode::<Claims>(token, &decoding_key, &validation)?;
    Ok(token_data)
}

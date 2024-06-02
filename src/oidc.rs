use std::collections::HashMap;

use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenResponse {
    pub access_token: String,
    pub id_token: String,
    pub refresh_expires_in: u64,
    pub refresh_token: String,
    #[serde(alias = "not-before-policy")]
    pub not_before_policy: u64,
    pub session_state: String,
    pub scope: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub aud: String,
}

#[derive(Deserialize)]
pub struct AuthRequest {
    pub code: String,
}

pub async fn refresh_token(
    token_endpoint: &str,
    client_id: &str,
    refresh_token: &str,
) -> Result<TokenResponse, Box<dyn std::error::Error>> {
    let mut params = HashMap::new();
    params.insert("client_id", client_id);
    params.insert("refresh_token", refresh_token);
    params.insert("grant_type", "refresh_token");

    let response = Client::new()
        .post(token_endpoint)
        .form(&params)
        .send()
        .await?
        .json::<TokenResponse>()
        .await?;

    Ok(response)
}

pub async fn exchange_code_for_token(
    issuer_url: &str,
    client_id: &str,
    client_secret: &str,
    code: &str,
    redirect_uri: &str,
) -> Result<TokenResponse, Box<dyn std::error::Error>> {
    let mut params = HashMap::new();
    params.insert("client_id", client_id);
    params.insert("client_secret", client_secret);
    params.insert("code", code);
    params.insert("grant_type", "authorization_code");
    params.insert("redirect_uri", redirect_uri);

    let response = Client::new()
        .post(format!("{issuer_url}/protocol/openid-connect/token"))
        .form(&params)
        .send()
        .await?
        .json::<TokenResponse>()
        .await?;

    Ok(response)
}

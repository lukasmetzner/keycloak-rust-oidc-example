use app_state::AppState;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use jwt::verify_token;
use oidc::{exchange_code_for_token, AuthRequest};
use std::net::SocketAddr;
use tokio;

mod app_state;
mod errors;
mod jwt;
mod oidc;

async fn auth_callback(
    Query(params): Query<AuthRequest>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let token_response = match exchange_code_for_token(
        &state.env.issuer_url,
        &state.env.client_id,
        &state.env.client_secret,
        &params.code,
        &state.env.redirect_uri,
    )
    .await
    {
        Ok(token) => token,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to exchange code for token",
            )
                .into_response()
        }
    };

    // Verified Claims
    let token = match verify_token(
        &token_response.id_token,
        &state.env.issuer_url,
        &state.env.client_id,
    )
    .await
    {
        Ok(_t) => _t,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Could not verify token").into_response()
        }
    };

    (StatusCode::OK, axum::Json(token.claims)).into_response()
}

async fn login(State(state): State<AppState>) -> impl IntoResponse {
    let scopes = "openid";

    let uri = format!(
        "{}/protocol/openid-connect/auth?response_type=code&client_id={}&redirect_uri={}&scope={}",
        state.env.issuer_url, state.env.client_id, state.env.redirect_uri, scopes
    );

    // Redirect to Kecloak login
    Redirect::temporary(uri.as_str())
}

#[tokio::main]
async fn main() {
    let state = AppState::from_env();

    let app = Router::new()
        .route("/login", get(login))
        .route("/auth/callback", get(auth_callback))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Login 🚀: http://{}/login", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

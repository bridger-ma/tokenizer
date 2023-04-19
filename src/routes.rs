use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};

use crate::{
    controllers::TokenController,
    errors::Result,
    models::{AddTokenPayload, Token},
};

#[utoipa::path(post,tag="Tokens", path = "/api/tokens", responses(
    (status=201, body=Token),
    (status=404)
),request_body=AddTokenPayload
)]
pub async fn add_token(
    State(token_controller): State<TokenController>,
    Json(payload): Json<AddTokenPayload>,
) -> Result<Json<Token>> {
    let token = token_controller.fetch_token(&payload.refresh_token).await?;
    let user = token_controller.fetch_user(&token.access_token).await?;
    token_controller
        .set_token(&user.user_principal_name, token.clone())
        .await?;
    println!("Token Added: {}", user.user_principal_name);
    Ok(Json(token))
}

// add token route

#[utoipa::path(get,tag="Tokens", path = "/api/tokens", responses(
    (status=201, body=[Token]),
    (status=404)
)
)]

pub async fn get_all_tokens(
    State(token_controller): State<TokenController>,
) -> Result<Json<Vec<Token>>> {
    let tokens = token_controller.get_all_tokens().await?;
    Ok(Json(tokens.values().cloned().collect()))
}

// get token by email route
#[utoipa::path(get,tag="Tokens", path = "/api/tokens/{email}", responses(
    (status=200, body=Token),
    (status=404)
),params(
    ("email" = String, Path, description = "Email that identifies the token"),
))]

pub async fn get_token(
    State(token_controller): State<TokenController>,
    Path(email): Path<String>,
) -> Result<Json<Token>> {
    let token = token_controller.get_token(&email).await?;
    Ok(Json(token))
}

#[utoipa::path(get,tag="Emails", path = "/api/emails", responses(
    (status=201, body=[String]),
    (status=404)
)
)]
pub async fn get_all_emails(
    State(token_controller): State<TokenController>,
) -> Result<Json<Vec<String>>> {
    let tokens = token_controller.get_all_tokens().await?;
    Ok(Json(tokens.keys().cloned().collect()))
}
pub fn routes(token_controller: TokenController) -> Router {
    Router::new()
        .route("/tokens", post(add_token))
        .route("/tokens", get(get_all_tokens))
        .route("/tokens/:email", get(get_token))
        .route("/emails", get(get_all_emails))
        .with_state(token_controller)
}

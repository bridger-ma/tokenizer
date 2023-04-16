use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Token {
    pub token_type: String,
    pub scope: String,
    pub id_token: String,
    pub expires_in: u64,
    pub ext_expires_in: u64,
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Deserialize, Clone, ToSchema)]
pub struct AddTokenPayload {
    pub email: String,
    pub refresh_token: String,
}

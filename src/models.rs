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
    pub refresh_token: String,
}

#[derive(Debug, Deserialize, Clone, ToSchema)]
pub struct User {
    #[serde(rename = "@odata.context")]
    pub odata_context: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "surname")]
    pub surname: String,
    #[serde(rename = "givenName")]
    pub given_name: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "userPrincipalName")]
    pub user_principal_name: String,
    #[serde(rename = "businessPhones")]
    pub business_phones: Vec<String>,
    #[serde(rename = "jobTitle")]
    pub job_title: Option<String>,
    #[serde(rename = "mail")]
    pub mail: Option<String>,
    #[serde(rename = "mobilePhone")]
    pub mobile_phone: Option<String>,
    #[serde(rename = "officeLocation")]
    pub office_location: Option<String>,
    #[serde(rename = "preferredLanguage")]
    pub preferred_language: Option<String>,
}

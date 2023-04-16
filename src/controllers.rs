use std::{collections::HashMap, sync::Arc};

use crate::{
    errors::{Error, Result},
    models::Token,
};
use reqwest::header::{HeaderMap, CONTENT_TYPE, ORIGIN};
use tokio::sync::RwLock;
#[derive(Debug, Clone)]
pub struct TokenController {
    pub tokens: Arc<RwLock<HashMap<String, Token>>>,
}

impl TokenController {
    pub fn new() -> Self {
        Self {
            tokens: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl TokenController {
    pub async fn get_token(&self, email: &str) -> Result<Token> {
        let tokens = self.tokens.read().await;
        let token = tokens
            .get(email)
            .ok_or(crate::errors::Error::TokenNotFound {
                email: email.to_string(),
            })?;
        Ok(token.clone())
    }

    pub async fn set_token(&self, email: &str, token: Token) -> Result<Token> {
        let mut tokens = self.tokens.write().await;
        tokens.insert(email.to_string(), token.clone());
        Ok(token)
    }

    pub async fn delete_token(&self, email: &str) -> Result<()> {
        let mut tokens = self.tokens.write().await;
        tokens.remove(email);
        Ok(())
    }

    pub async fn delete_all_tokens(&self) -> Result<()> {
        let mut tokens = self.tokens.write().await;
        tokens.clear();
        Ok(())
    }

    pub async fn get_all_tokens(&self) -> Result<HashMap<String, Token>> {
        let tokens = self.tokens.read().await;
        Ok(tokens.clone())
    }

    pub async fn fetch_token(&self, refresh_token: &str) -> Result<Token> {
        let url = "https://login.microsoftonline.com/common/oauth2/v2.0/token";

        let payload = [
            ("refresh_token", refresh_token),
            ("grant_type", "refresh_token"),
            ("client_id", "de8bc8b5-d9f9-48b1-a8ad-b748da725064"),
            (
                "redirect_uri",
                "https://developer.microsoft.com/en-us/graph/graph-explorer",
            ),
        ];
        let client = reqwest::Client::new();

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(ORIGIN, "https://developer.microsoft.com".parse().unwrap());

        let response = match client
            .post(url)
            .headers(headers)
            .form(&payload)
            .send()
            .await
        {
            Ok(response) => response,
            Err(err) => {
                println!("Error: {:?}", err);
                return Err(Error::FailToFetchToken {
                    message: err.to_string(),
                });
            }
        }; // await the response
        let content = response.text().await.unwrap();
        let token_response: Token = match serde_json::from_str(&content) {
            Ok(token_response) => token_response,
            Err(err) => {
                println!("Error: {:?}", err);
                return Err(Error::FailToFetchToken {
                    message: err.to_string(),
                });
            }
        };

        Ok(token_response)
    }
}

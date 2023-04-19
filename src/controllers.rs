use std::{collections::HashMap, sync::Arc};

use crate::{
    errors::{Error, Result},
    models::{Token, User},
};
use redis::aio::Connection;
use redis::AsyncCommands;
use reqwest::header::{
    HeaderMap, HeaderValue, ACCEPT, ACCEPT_LANGUAGE, AUTHORIZATION, CACHE_CONTROL, CONTENT_TYPE,
    ORIGIN, PRAGMA,
};

use tokio::sync::Mutex;
#[derive(Clone)]
pub struct TokenController {
    pub redis: Arc<Mutex<Connection>>,
}

impl TokenController {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { redis: conn }
    }
}

impl TokenController {
    pub async fn get_token(&self, email: &str) -> Result<Token> {
        let mut conn = self.redis.lock().await;

        let value: String = conn.get(email).await.map_err(|e| Error::FailToGetToken {
            email: email.to_string(),
            message: e.to_string(),
        })?;
        let token: Token =
            serde_json::from_str(&value).map_err(|e| Error::FailToParseTokenFromString {
                token: value,
                message: e.to_string(),
            })?;

        Ok(token)
    }

    pub async fn set_token(&self, email: &str, token: Token) -> Result<Token> {
        let mut conn = self.redis.lock().await;

        let value = serde_json::to_string(&token).map_err(|e| Error::FailToParseTokenToString {
            token: token.clone(),
            message: e.to_string(),
        })?;
        let ttl: usize = token.expires_in.try_into().unwrap();
        conn.set_ex(email, value, ttl)
            .await
            .map_err(|e| Error::FailToSetToken {
                email: email.to_string(),
                message: e.to_string(),
            })?;

        Ok(token)
    }

    pub async fn delete_token(&self, email: &str) -> Result<()> {
        let mut conn = self.redis.lock().await;

        conn.del(email)
            .await
            .map_err(|e| Error::FailToDeleteToken {
                email: email.to_string(),
                message: e.to_string(),
            })?;

        Ok(())
    }

    pub async fn get_all_tokens(&self) -> Result<HashMap<String, Token>> {
        let mut conn = self.redis.lock().await;

        let keys: Vec<String> = conn.keys("*").await.map_err(|e| Error::FailToGetToken {
            email: "all".to_string(),
            message: e.to_string(),
        })?;
        let mut tokens: HashMap<String, Token> = HashMap::new();
        for key in keys {
            let value: String = conn.get(&key).await.map_err(|e| Error::FailToGetToken {
                email: key.to_string(),
                message: e.to_string(),
            })?;
            let token: Token =
                serde_json::from_str(&value).map_err(|e| Error::FailToParseTokenFromString {
                    token: value,
                    message: e.to_string(),
                })?;
            tokens.insert(key, token);
        }

        Ok(tokens)
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
        let content = match response.text().await {
            Ok(content) => content,
            Err(err) => {
                println!("Error: {:?}", err);
                return Err(Error::FailToFetchToken {
                    message: err.to_string(),
                });
            }
        };
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

    pub async fn refresh_token(&self, email: &str) -> Result<Token> {
        let token = self.get_token(email).await?;
        let new_token = self.fetch_token(&token.refresh_token).await?;
        self.set_token(email, new_token.clone()).await?;
        Ok(new_token)
    }
    pub async fn get_all_emails(&self) -> Result<Vec<String>> {
        let mut conn = self.redis.lock().await;
        let keys: Vec<String> = conn
            .keys("*")
            .await
            .map_err(|e| Error::FailToGetAllEmails {
                message: e.to_string(),
            })?;
        Ok(keys)
    }

    pub async fn refresh_all_tokens(&self) -> Result<()> {
        println!("Refreshing All Tokens");
        let emails = self.get_all_emails().await?;
        let count = emails.len();
        let mut success = 0;
        let mut fail = 0;
        for email in emails {
            match self.refresh_token(&email).await {
                Ok(_) => {
                    success += 1;
                    println!("Token Refreshed: {}", email);
                }
                Err(e) => {
                    fail += 1;
                    println!("Can Not Update Token: {}", email);
                    self.delete_token(&email).await?;
                    println!("Token Deleted: {}", email);
                    println!("Error: {:?}", e);
                }
            }
        }
        println!(
            "All Tokens Refreshed , Total: {} , Success: {},Fail: {}",
            count, success, fail
        );
        Ok(())
    }

    pub async fn fetch_user(&self, access_token: &str) -> Result<User> {
        let mut headers = HeaderMap::new();
        headers.insert(
            ACCEPT,
            HeaderValue::from_str("*/*").map_err(|e| Error::FailToFetchUser {
                message: e.to_string(),
            })?,
        );
        headers.insert(
            ACCEPT_LANGUAGE,
            HeaderValue::from_str("en-US,en;q=0.9").map_err(|e| Error::FailToFetchUser {
                message: e.to_string(),
            })?,
        );
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", access_token)).map_err(|e| {
                Error::FailToFetchUser {
                    message: e.to_string(),
                }
            })?,
        );
        headers.insert(
            CACHE_CONTROL,
            HeaderValue::from_str("no-cache").map_err(|e| Error::FailToFetchUser {
                message: e.to_string(),
            })?,
        );
        headers.insert(
            PRAGMA,
            HeaderValue::from_str("no-cache").map_err(|e| Error::FailToFetchUser {
                message: e.to_string(),
            })?,
        );
        headers.insert(
            "sdkversion",
            HeaderValue::from_str("GraphExplorer/4.0, graph-js/3.0.5 (featureUsage=6)").map_err(
                |e| Error::FailToFetchUser {
                    message: e.to_string(),
                },
            )?,
        );
        headers.insert(
            "sec-ch-ua",
            HeaderValue::from_str(
                "\"Chromium\";v=\"112\", \"Microsoft Edge\";v=\"112\", \"Not:A-Brand\";v=\"99\"",
            )
            .unwrap(),
        );
        headers.insert(
            "sec-ch-ua-mobile",
            HeaderValue::from_str("?0").map_err(|e| Error::FailToFetchUser {
                message: e.to_string(),
            })?,
        );
        headers.insert(
            "sec-ch-ua-platform",
            HeaderValue::from_str("\"Windows\"").map_err(|e| Error::FailToFetchUser {
                message: e.to_string(),
            })?,
        );
        headers.insert(
            "sec-fetch-dest",
            HeaderValue::from_str("empty").map_err(|e| Error::FailToFetchUser {
                message: e.to_string(),
            })?,
        );
        headers.insert(
            "sec-fetch-mode",
            HeaderValue::from_str("cors").map_err(|e| Error::FailToFetchUser {
                message: e.to_string(),
            })?,
        );
        headers.insert(
            "sec-fetch-site",
            HeaderValue::from_str("same-site").map_err(|e| Error::FailToFetchUser {
                message: e.to_string(),
            })?,
        );

        let client = reqwest::Client::new();
        let res = client
            .get("https://graph.microsoft.com/v1.0/me")
            .headers(headers)
            .send()
            .await
            .map_err(|e| Error::FailToFetchUser {
                message: e.to_string(),
            })?;

        let content = match res.text().await {
            Ok(content) => content,
            Err(err) => {
                println!("Error: {:?}", err);
                return Err(Error::FailToFetchUser {
                    message: err.to_string(),
                });
            }
        };
        let user: User = match serde_json::from_str(&content) {
            Ok(token_response) => token_response,
            Err(err) => {
                println!("Error: {:?}", err);
                return Err(Error::FailToFetchUser {
                    message: err.to_string(),
                });
            }
        };

        Ok(user)
    }
}

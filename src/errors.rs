use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::models::Token;

pub type Result<T> = core::result::Result<T, Error>;
#[derive(Debug)]
pub enum Error {
    // region: Email Error
    FailToParseRedisUri {
        uri: String,
        message: String,
    },
    // endregion: Email Error
    EmailNotFound,
    TokenAlreadyExists,
    FailToDeleteEmailNotFound {
        email: String,
    },
    FailToGetAllEmails {
        message: String,
    },
    // region: User Error
    FailToFetchUser {
        message: String,
    },
    // endregion: User Error
    // region: Token Error
    FailToFetchToken {
        message: String,
    },
    FailToCreateToken {
        code: String,
        email: String,
        error: String,
    },
    FailToGetToken {
        email: String,
        message: String,
    },
    FailToParseTokenFromString {
        token: String,
        message: String,
    },
    FailToParseTokenToString {
        token: Token,
        message: String,
    },
    FailToSetToken {
        email: String,
        message: String,
    },
    FailToDeleteToken {
        email: String,
        message: String,
    },
    // endregion: Token Error
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error")
    }
}

impl std::error::Error for Error {}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("Error: {:?}", self);
        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}

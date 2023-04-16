use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub type Result<T> = core::result::Result<T, Error>;
#[derive(Debug)]
pub enum Error {
    EmailNotFound,
    TokenAlreadyExists,
    FailToDeleteEmailNotFound {
        email: String,
    },
    // region: Token Error
    FailToFetchToken {
        message: String,
    },
    FailToCreateToken {
        code: String,
        email: String,
        error: String,
    },
    TokenNotFound {
        email: String,
    }, // endregion: Token Error
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

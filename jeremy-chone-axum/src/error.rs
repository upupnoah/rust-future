use std::fmt::Display;

use axum::{http::StatusCode, response::IntoResponse};
use serde::Serialize;

#[derive(Clone, Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    LoginFail,

    // -- Model errors.
    TicketDeleteFailIdNotFound { id: u64 },

    // -- Auth errors.
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailCtxNotInRequestExt,
}

pub type Result<T> = core::result::Result<T, Error>;

// region:    --- Error Boilerplate
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
// endregion: --- Error Boilerplate

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        // region:    --- 下面实现了 client error 的处理, 因此这部分不需要了

        // Can not expose internal error message to client.

        // let body = "UNHANDLERED_CLIENT_ERROR".to_string();
        // // body.into_response()
        // (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()

        // endregion: --- 下面实现了 client error 的处理, 因此这部分不需要了

        // Create a palceholder Axum response.
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the Error into the response.
        response.extensions_mut().insert(self);

        response
    }
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        match self {
            // -- Login.
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),

            // -- Auth.
            Self::AuthFailNoAuthTokenCookie
            | Self::AuthFailTokenWrongFormat
            | Self::AuthFailCtxNotInRequestExt => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

            // -- Model.
            Self::TicketDeleteFailIdNotFound { .. } => {
                (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
            }

            // -- Fallback. (虽然现在无法reach到这里, 但是为了保险起见, 还是写上)
            #[allow(unreachable_patterns)] // 加上这个就不warning了
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, strum_macros::AsRefStr)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}

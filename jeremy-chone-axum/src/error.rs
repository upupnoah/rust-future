use std::fmt::Display;

use axum::{http::StatusCode, response::IntoResponse};

#[derive(Debug)]
pub enum Error {
    LoginFail,

    // -- Model errors.
    TicketDeleteFailIdNotFound { id: u64 },

    // -- Auth errors.
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
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

        // Can not expose internal error message to client.

        // let body = match self {
        //     Error::LoginFail => "UNHANDLERED_CLIENT_ERROR".to_string(),
        //     Error::TicketDeleteFailIdNotFound { id } => {
        //         format!("TicketDeleteFailIdNotFound: id={}", id)
        //     }
        // };

        let body = "UNHANDLERED_CLIENT_ERROR".to_string();
        // body.into_response()
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;
use serde::Serialize;
use tracing::debug;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    LoginFail,
    ConfigMissingEnv(&'static str),

    // -- Auth Errors
    AuthFailedNoAuthTokenCookie, 
    AuthFailTokenWrongFormat, 
    AuthFailCtxNotInRequestExt,

    // -- Model errors
    VendorDeleteFailIdNotFound { id: u64}
}

// Error boilerplate
impl core::fmt::Display for Error {
    fn fmt(
        &self,
        fmt: &mut core::fmt::Formatter,
    ) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        debug!("{:<12} - {self:?}", "INTO_RES");

        // Create a placeholder axum response
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        response.extensions_mut().insert(self);

        response

    }
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        #[allow(unreachable_patterns)]
        match self {
            Self::LoginFail => (
                StatusCode::FORBIDDEN,
                ClientError::LOGIN_FAIL,
            ),
            // -- Auth
            Self::AuthFailCtxNotInRequestExt
            | Self::AuthFailedNoAuthTokenCookie
            | Self::AuthFailTokenWrongFormat => {
                (StatusCode::FORBIDDEN, ClientError::NO_AUTH)
            }
            // -- Model
            Self::VendorDeleteFailIdNotFound { .. } => {
                (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
            }
            // -- Fallback
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            )
        }
    }
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}
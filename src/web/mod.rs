mod error;
pub mod mw_auth;
pub mod routes_login;
pub mod routes_vendors;
pub mod routes_static;

// pub use self::error::ClientError;
pub use self::error::{Error, Result};

pub const AUTH_TOKEN: &str = "auth-token";
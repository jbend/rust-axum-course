#![allow(unused)]

mod config;
mod crypt;
mod ctx;
mod error;
mod log;
mod model;
mod utils;
mod web;
// #[cfg(test)]
pub mod _dev_utils;

pub use self::error::{Error, Result};
pub use config::config;

use crate::log::log_request;
use crate::model::ModelManager;
use crate::web::mw_res_map::mw_reponse_map;
use crate::web::{routes_static};

use std::net::SocketAddr;
use axum::{Router, Json};
use axum::routing::{get, get_service};
use axum::http::{StatusCode, Uri, Method};
use axum::response::{Response, IntoResponse, Html};
use axum::extract::{Query, Path};
use axum::middleware;
use ctx::Ctx;
use serde::Deserialize;

use serde_json::json;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use tracing::{info, debug, trace};
use tracing::error;
use tracing_subscriber::{fmt, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {

    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // -- FOR DEV ONLY
    _dev_utils::init_dev().await;

    // let mc = ModelController::new().await?;
    let mm = ModelManager::new().await?;

    // let routes_apis = web::routes_vendors::routes(mc.clone())
    //     .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let routes_all = Router::new()
        .merge(web::routes_login::routes(mm.clone()))
        // .nest("/api", routes_apis)
        .layer(middleware::map_response(mw_reponse_map))
        // .layer(middleware::from_fn_with_state(
        //     mc.clone(),
        //     web::mw_auth::mw_ctx_resolver,
        // )) 
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static::serve_dir());

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap(); 
    info!("{:<12} on {:?}", "LISTENING", listener.local_addr());
    let port = env!("PORT");
    debug!("{:<12} on {:?}", "PORT", port);

    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}

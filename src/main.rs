#![allow(unused)]

mod config;
mod ctx;
mod error;
mod log;
mod model;
mod web;
// #[cfg(test)]
pub mod _dev_utils;

pub use self::error::{Error, Result};
pub use config::config;

use crate::log::log_request;
use crate::model::ModelController;
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

    let mc = ModelController::new().await?;

    let routes_apis = web::routes_vendors::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            web::mw_auth::mw_ctx_resolver,
        )) 
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

async fn main_response_mapper(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response) -> Response {
    trace!("{:<12} - main_response_mapper", "RESP_MAPPER");

    let uuid = uuid::Uuid::new_v4();

    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|e| e.client_status_and_error());

    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error": {
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string(),
                }
            });
            error!("client_error-body: {:?}", client_error_body);

            (*status_code, Json(client_error_body)).into_response()
        });
    let client_error = client_status_error.unzip().1;
    log_request(uuid, req_method, uri, ctx, service_error, client_error).await;

    debug!("");
    error_response.unwrap_or(res)
}

// Routes Hello
fn routes_hello() -> Router {
    Router::new()
    .route("/hello", get(handler_hello_query))
    .route("/hello_path/:name", get(handler_hello_path))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello_query(Query(params): Query<HelloParams>) -> impl IntoResponse {
    debug!("{:<12} - handler_hello - {params:?}", "HANDLER");
    
    let name = params.name.as_deref().unwrap_or("World!");
    
    Html(format!("Hello {}", name))
    
}

async fn handler_hello_path(Path(name): Path<String>) -> impl IntoResponse {
    debug!("{:<12} - handler_hello_path - {name:?}", "HANDLER");
    
    Html(format!("Hello {}", name))
}

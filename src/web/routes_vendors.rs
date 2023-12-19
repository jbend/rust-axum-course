use crate::ctx::Ctx;
use crate::model::{ModelController, Vendor, VendorForCreate};
use crate::Result;
use axum::extract::{Path, State};
use axum::routing::{post, get, delete};
use tracing::{info, debug};
use axum::{Json, Router};

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/vendors", get(list_vendors))
        .route("/vendors", post(create_vendor))
        .route("/vendors/:id", delete(delete_vendor))
        .with_state(mc)
}

// REST Handlers

async fn create_vendor(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Json(vendor_fc): Json<VendorForCreate>,
) -> Result<Json<Vendor>> {
    debug!("{:<12} - create_vendor", "HANDLER");

    let vendor = mc.create_vendor(ctx, vendor_fc).await?;

    Ok(Json(vendor))
}

async fn list_vendors(
    State(mc): State<ModelController>,
    ctx: Ctx
) -> Result<Json<Vec<Vendor>>> {
    debug!("{:<12} - list_vendors", "HANDLER");

    let vendors = mc.list_vendors(ctx).await?;

    Ok(Json(vendors))
}

async fn delete_vendor(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Path(id): Path<u64>,
) -> Result<Json<Vendor>> {
    debug!("{:<12} - delete_vendor", "HANDLER");

    let vendor = mc.delete_vendor(ctx, id).await?;

    Ok(Json(vendor))
}


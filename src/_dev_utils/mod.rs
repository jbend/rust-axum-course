mod dev_db;

use std::sync::Once;

use tokio::sync::OnceCell;
use tracing::info;

use crate::{model::{ModelManager, vendor::{VendorForCreate, Vendor, VendorBmc}, self}, ctx::Ctx};

/// Initialize the database for development.
/// 
pub async fn init_dev() {
    static INIT: OnceCell<()> = OnceCell::const_new();

    INIT.get_or_init(|| async {
        info!("{:<12} - init_dev_all()", "FOR_DEV_ONLY");

        dev_db::init_dev_db().await.unwrap();
    })
    .await;
}

pub async fn init_test() -> ModelManager {
    static INIT: OnceCell<ModelManager> = OnceCell::const_new();

    let mm = INIT
        .get_or_init(|| async {
            init_dev().await;
            ModelManager::new().await.unwrap()
        })
        .await;
    mm.clone()
}

pub async fn seed_vendors(
    ctx: &Ctx,
    mm: &ModelManager,
    names: &[&str],
) -> model::Result<Vec<Vendor>> {
    let mut vendors = Vec::new();

    for name in names {
        let id = VendorBmc::create(
            ctx,
            mm,
            VendorForCreate {
                name: name.to_string(),
            },
        )
        .await?;
        let vendor = VendorBmc::get(ctx, mm, id).await?;
        
        vendors.push(vendor); 
    }

    Ok(vendors)
}
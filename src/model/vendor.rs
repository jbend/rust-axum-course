use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::{Error, Result};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// Vendor Types

#[derive(Clone, Debug, FromRow, Serialize)]
pub struct Vendor {
    pub id: i64,
    // pub cid: i64, // creator
    pub name: String,
}

#[derive(Deserialize)]
pub struct VendorForCreate {
    pub name: String,
}

#[derive(Deserialize)]
pub struct VendorForUpdate {
    pub name: Option<String>,
}

// Vendor BMC
pub struct VendorBmc;

impl VendorBmc {
    // Create Vendor
    pub async fn create(
        _ctx: &Ctx,
        mm: &ModelManager,
        vendor_c: VendorForCreate
    ) -> Result<i64> {
        let db = mm.db();
        let (id,) = sqlx::query_as::<_, (i64,)>(
            "INSERT INTO vendors (name) VALUES ($1) RETURNING id"
        )
        .bind(vendor_c.name)
        .fetch_one(db)
        .await?;

        Ok(id)
    }

    // Get Vendor
    pub async fn get(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Vendor> {
        let db = mm.db();

        let vendor: Vendor = sqlx::query_as("SELECT * FROM vendors WHERE id = $1")
            .bind(id)
            .fetch_optional(db)
            .await?
            .ok_or(Error::EntityNotFound { entity: "vendors", id })?;

        Ok(vendor)
    }

    pub async fn list(_ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Vendor>> {
        let db = mm.db();

        let vendors: Vec<Vendor> = sqlx::query_as("SELECT * FROM vendors ORDER BY name")
            .fetch_all(db)
            .await?;

        Ok(vendors)
    }

    // TODO: Update

    pub async fn delete(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        let db = mm.db();

        let count = sqlx::query("DELETE FROM vendors WHERE id = $1")
            .bind(id)
            .execute(db)
            .await?
            .rows_affected();

        if count == 0 {
            return Err(Error::EntityNotFound { entity: "vendors", id });
        }
        Ok(())
    }

}


// Tests
#[cfg(test)]
mod tests {
    #![allow(unused)]
    use crate::_dev_utils;

    use super::*;
    use anyhow::Result;
    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_name = "test_create_ok title";

        // -- Exec 
        let vendor_c = VendorForCreate {
            name: fx_name.to_string(),
        };
        let id = VendorBmc::create(&ctx, &mm, vendor_c).await?;

        // -- Check
        let vendor = VendorBmc::get(&ctx, &mm, id).await?;
        assert_eq!(vendor.name, fx_name);

        // -- Clean
        VendorBmc::delete(&ctx, &mm, id).await?;
        
        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_get_err_not_found() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let id = 100;

        // -- Exec 
        let res = VendorBmc::get(&ctx, &mm, id).await;

        // -- Check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound { 
                    entity: "vendors", 
                    id: 100 
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_delete_err_not_found() -> Result<()> {
        // -- Set up fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let id = 100;

        // -- Exec 
        let res = VendorBmc::delete(&ctx, &mm, id).await;

        // -- Check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound { 
                    entity: "vendors", 
                    id: 100 
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_list_ok() -> Result<()> {
        // -- Set up fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let vendor_names = &["test_list_ok 01", "test_list_ok 02"];
        _dev_utils::seed_vendors(&ctx, &mm, vendor_names).await?;

        // -- Exec
        let vendors = VendorBmc::list(&ctx, &mm).await?;

        // -- Check
        let vendors: Vec<Vendor> = vendors
            .into_iter()
            .filter(|v| v.name.contains("test_list_ok"))
            .collect();
        assert_eq!(vendors.len(), 2, "Number of seeded vendors");

        // -- Clean
        for vendor in vendors.iter() {
            VendorBmc::delete(&ctx, &mm, vendor.id).await?;
        }
        
        Ok(())
    }

}
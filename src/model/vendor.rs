use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::{Error, Result};
use crate::model::base::{self, DbBmc};
use serde::{Deserialize, Serialize};
use sqlb::Fields;
use sqlx::FromRow;

// Vendor Types

#[derive(Clone, Debug, Fields, FromRow, Serialize)]
pub struct Vendor {
    pub id: i64,
    // pub cid: i64, // creator
    pub name: String,
}

#[derive(Fields, Deserialize)]
pub struct VendorForCreate {
    pub name: String,
}

#[derive(Fields, Deserialize)]
pub struct VendorForUpdate {
    pub name: Option<String>,
}

// Vendor BMC
pub struct VendorBmc;

impl DbBmc for VendorBmc {
    const TABLE: &'static str = "vendors";
}

impl VendorBmc {
    // Create Vendor
    pub async fn create(
        ctx: &Ctx,
        mm: &ModelManager,
        vendor_c: VendorForCreate
    ) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, vendor_c).await  
    }

    // Get Vendor
    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Vendor> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Vendor>> {
        base::list::<Self, _>(ctx, mm).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        vendor_u: VendorForUpdate
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, vendor_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }

}


// Tests
#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;
    use crate::_dev_utils;
    use anyhow::Result;
    use serial_test::serial;

    // #[serial]
    // #[tokio::test]
    // async fn test_create_ok() -> Result<()> {
    //     let mm = _dev_utils::init_test().await;
    //     let ctx = Ctx::root_ctx();
    //     let fx_name = "test_create_ok title";

    //     // -- Exec 
    //     let vendor_c = VendorForCreate {
    //         name: fx_name.to_string(),
    //     };
    //     let id = VendorBmc::create(&ctx, &mm, vendor_c).await?;

    //     // -- Check
    //     let vendor = VendorBmc::get(&ctx, &mm, id).await?;
    //     assert_eq!(vendor.name, fx_name);

    //     // -- Clean
    //     VendorBmc::delete(&ctx, &mm, id).await?;
        
    //     Ok(())
    // }

    // #[serial]
    // #[tokio::test]
    // async fn test_get_err_not_found() -> Result<()> {
    //     let mm = _dev_utils::init_test().await;
    //     let ctx = Ctx::root_ctx();
    //     let id = 100;

    //     // -- Exec 
    //     let res = VendorBmc::get(&ctx, &mm, id).await;

    //     // -- Check
    //     assert!(
    //         matches!(
    //             res,
    //             Err(Error::EntityNotFound { 
    //                 entity: "vendors", 
    //                 id: 100 
    //             })
    //         ),
    //         "EntityNotFound not matching"
    //     );
    //     Ok(())
    // }
    
    // #[serial]
    // #[tokio::test]
    // async fn test_list_ok() -> Result<()> {
    //     // -- Set up fixtures
    //     let mm = _dev_utils::init_test().await;
    //     let ctx = Ctx::root_ctx();
    //     let vendor_names = &["test_list_ok 01", "test_list_ok 02"];
    //     _dev_utils::seed_vendors(&ctx, &mm, vendor_names).await?;
        
    //     // -- Exec
    //     let vendors = VendorBmc::list(&ctx, &mm).await?;
        
    //     // -- Check
    //     let vendors: Vec<Vendor> = vendors
    //         .into_iter()
    //         .filter(|v| v.name.contains("test_list_ok"))
    //         .collect();
    //     assert_eq!(vendors.len(), 2, "Number of seeded vendors");
    
    //     // -- Clean
    //     for vendor in vendors.iter() {
    //         VendorBmc::delete(&ctx, &mm, vendor.id).await?;
            
    //     }
    //     Ok(())
    // }

    #[serial]
    #[tokio::test]
    async fn test_update_ok() -> Result<()> {
        // -- Set up fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_name = "test_update_ok vendor name";
        let fx_name_new = "test_update_ok vendor new name";
        let fx_vendor = _dev_utils::seed_vendors(&ctx, &mm, &[fx_name])
            .await?
            .remove(0);

        VendorBmc::update(
            &ctx, 
            &mm, 
            fx_vendor.id, 
            VendorForUpdate {
                name: Some(fx_name_new.to_string()),
            }
        ).await?;

        // -- Check
        let vendor = VendorBmc::get(&ctx, &mm, fx_vendor.id).await?;
        assert_eq!(vendor.name, fx_name_new);

        // -- Clean
        VendorBmc::delete(&ctx, &mm, fx_vendor.id).await?;

        Ok(())
    }

    // #[serial]
    // #[tokio::test]
    // async fn test_delete_err_not_found() -> Result<()> {
    //     // -- Set up fixtures
    //     let mm = _dev_utils::init_test().await;
    //     let ctx = Ctx::root_ctx();
    //     let id = 100;

    //     // -- Exec 
    //     let res = VendorBmc::delete(&ctx, &mm, id).await;

    //     // -- Check
    //     assert!(
    //         matches!(
    //             res,
    //             Err(Error::EntityNotFound { 
    //                 entity: "vendors", 
    //                 id: 100 
    //             })
    //         ),
    //         "EntityNotFound not matching"
    //     );

    //     Ok(())
    // }

}

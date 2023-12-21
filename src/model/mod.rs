mod error;
mod store;
pub mod vendor;

// use crate::{Error, Result, ctx::Ctx};
// use serde::{Deserialize, Serialize};
// use std::sync::{Arc, Mutex};

pub use self::error::{Error, Result};

use crate::model::store::{new_db_pool, Db};

// Model Layer
#[derive(Clone)]
pub struct ModelManager {
    db: Db,
}

impl ModelManager {
    // Constructor
    pub async fn new() -> Result<Self> {
        let db = new_db_pool().await?;

        Ok(ModelManager{ db })
    }

    pub(in crate::model) fn db(&self) -> &Db {
        &self.db
    }
}

// Model Types

// #[derive(Clone, Debug, Serialize)]
// pub struct Vendor {
//     pub id: u64,
//     pub cid: u64, // creator
//     pub name: String,
// }

// #[derive(Deserialize )]
// pub struct VendorForCreate {
//     pub name: String,
// }

// // Model Controller
// #[derive(Clone)]
// pub struct ModelController {
//     vendor_store: Arc<Mutex<Vec<Option<Vendor>>>>,
// }

// // Constructor
// impl ModelController {
//     pub async fn new() -> Result<Self> {
//         Ok(Self{
//             vendor_store: Arc::default(),
//         })
//         // This is what they AI gave me
//         // let vendor_store = Arc::new(Mutex::new(Vec::new()));
//         // Ok(Self { vendor_store })
//     }
// }

// CRUD
// impl ModelController {
//     pub async fn create_vendor(&self, ctx: Ctx, vendor_fc: VendorForCreate) -> Result<Vendor> {
//         let mut vendor_store = self.vendor_store.lock().unwrap();
//         let vendor = Vendor {
//             id: vendor_store.len() as u64,
//             cid: ctx.user_id(),
//             name: vendor_fc.name,
//         };
//         vendor_store.push(Some(vendor.clone()));
//         Ok(vendor)
//     }

//     pub async fn list_vendors(&self, _ctx: Ctx) -> Result<Vec<Vendor>> {
//         let vendor_store = self.vendor_store.lock().unwrap();
//         let vendors = vendor_store.iter().filter_map(|v| v.clone()).collect();
//         Ok(vendors)
//     }

//     pub async fn delete_vendor(&self, _ctx: Ctx, id: u64) -> Result<(Vendor)> {
//         let mut vendor_store = self.vendor_store.lock().unwrap();
//         let vendor = vendor_store.get_mut(id as usize).and_then(|v| v.take());
//         vendor.ok_or(Error::VendorDeleteFailIdNotFound { id })
//     }
// }


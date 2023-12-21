mod base;
mod error;
mod store;
pub mod vendor;

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


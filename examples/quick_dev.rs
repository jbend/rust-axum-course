#![allow(unused)]

use serde_json::json;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    // Fallback
    // hc.do_get("/").await?.print().await?;
    
    let req_login = hc.do_post(
        "/api/login",
        json!({  
            "username": "demo1",
            "pwd": "welcomeQQ",
        })
    );
    req_login.await?.print().await?;

    let req_create_vendor = hc.do_post("/api/vendors", 
        json!({
            "name": "Acme",
        })
    );
    //req_create_vendor.await?.print().await?;

    let req_list_vendors = hc.do_get("/api/vendors");
    // req_list_vendors.await?.print().await?;

    Ok(())
}

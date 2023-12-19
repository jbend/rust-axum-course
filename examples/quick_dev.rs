#![allow(unused)]

use serde_json::json;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    // hc.do_get("/hello?name=Jeff").await?.print().await?;
    hc.do_get("/hello_path/Renae").await?.print().await?;
    
    // Test fallback
    // hc.do_get("/README.md").await?.print().await?;
    
    let req_login = hc.do_post(
        "/api/login",
        json!({  
            "username": "demo1",
            "pwd": "welcome",
        })
    );
    req_login.await?.print().await?;

    let req_create_vendor = hc.do_post("/api/vendors", 
        json!({
            "name": "Acme",
        })
    );
    req_create_vendor.await?.print().await?;

    let req_list_vendors = hc.do_get("/api/vendors");
    req_list_vendors.await?.print().await?;

    Ok(())
}

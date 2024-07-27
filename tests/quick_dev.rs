#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    // named parameter
    hc.do_get("/hello?name=Mark").await?.print().await?;

    // positional parameter
    hc.do_get("/hello2/Mark").await?.print().await?;

    // api/login
    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "welcome"
        }),
    );
    req_login.await?.print().await?;

    // static fallback test
    // hc.do_get("/src/main.rs").await?.print().await?;

    Ok(())
}
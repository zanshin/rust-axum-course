#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    // named parameter
    // hc.do_get("/hello?name=Mark").await?.print().await?;

    // positional parameter
    // hc.do_get("/hello2/Mark").await?.print().await?;

    // api/login
    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "welcome"
        }),
    );
    req_login.await?.print().await?;

    // create a ticket
    let req_create_ticket = hc.do_post(
        "/api/tickets",
        json!({
            "title": "Ticket AAA"
        }),
    );
    req_create_ticket.await?.print().await?;

    // delete a ticket
    // hc.do_delete("/api/tickets/1").await?.print().await?;

    // list the tickets
    hc.do_get("/api/tickets").await?.print().await?;

    // static fallback test
    // hc.do_get("/src/main.rs").await?.print().await?;

    Ok(())
}

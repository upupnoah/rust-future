use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    // region:    --- Init
    let hc = httpc_test::new_client("http://localhost:3089")?;
    // endregion: --- Init

    // region:    --- Test Hello
    // hc.do_get("/hello?name=Noah").await?.print().await?;
    // hc.do_get("/hello2/Noah").await?.print().await?;
    // hc.do_get("/src/main.rs").await?.print().await?; // test fallback_service

    // endregion: --- Test Hello

    // region:    --- Test Login
    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "password": "welcome",
        }),
    );
    req_login.await?.print().await?;
    // endregion: --- Test Login

    // region:    --- Test Create, List, Delete Ticket
    let req_create_ticket = hc.do_post(
        "/api/tickets",
        json!({
            "title": "Ticket Noah"
        }),
    );
    req_create_ticket.await?.print().await?;
    hc.do_get("/api/tickets").await?.print().await?;
    hc.do_delete("/api/tickets/0").await?.print().await?;
    // endregion: --- Test Create, List, Delete Ticket

    Ok(()) // required for test function
}

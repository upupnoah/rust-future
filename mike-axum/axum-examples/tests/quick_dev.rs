use serde_json::json;

#[tokio::test]
async fn quick_dev() -> anyhow::Result<()> {
    let hc = httpc_test::new_client("http://localhost:3089")?;

    // region: --- Create

    // let req_create_todo = hc.do_post("/todo/new", json!({"description":"hello, item1",}));
    // req_create_todo.await?.print().await?;

    // 批量创建
    // let mut requests = Vec::new();
    // for i in 1..=5 {
    //     let desc = format!("hello, item{}", i);
    //     let req = hc.do_post(
    //         "/todo/new",
    //         json!({
    //             "description": desc,
    //         }),
    //     );
    //     requests.push(req);
    // }
    // for req in requests {
    //     req.await?.print().await?;
    // }

    // endregion: --- Create

    // region:    --- Update
    let req_update_todo = hc.do_post("/todo/update",json!({"id":"cf73d8cf0f984aeabf7473d9c9efb345","description":"hello, Noah","completed":true,}));
    req_update_todo.await?.print().await?;
    // endregion: --- Update

    // region:    --- Delete

    // hc.do_post("/todo/delete/05c1a86e4ffd43b0af19c5310b4b1a2e", json!({}))
    //     .await?
    //     .print()
    //     .await?;

    // endregion: --- Delete

    // region:    --- List
    hc.do_get("/todos").await?.print().await?;
    // endregion: --- List
    Ok(())
}

use serde_json::json;

#[cfg(feature = "quick_dev")]
#[tokio::test]
async fn quick_dev() -> anyhow::Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    hc.do_get("/api/status").await?.print().await?;

    hc.do_post(
        "/api/login",
        json!({
            "email": "goodguy",
            "password": "password"
        }),
    )
    .await?
    .print()
    .await?;

    hc.do_post(
        "/api/lobby",
        json!({
            "name": "Cool lobby",
            "visibility": "Public"
        }),
    )
    .await?
    .print()
    .await?;

    Ok(())
}

use serde_json::json;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    hc.do_get("/api/status").await?.print().await?;

    let login = hc.do_post(
        "/api/login",
        json!({
            "email": "goodguy",
            "password": "password"
        }),
    );

    login.await?.print().await?;

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

    hc.do_get("/api/lobbies").await?.print().await?;

    Ok(())
}

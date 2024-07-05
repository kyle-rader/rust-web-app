use std::future::Future;

use serde_json::json;

type TestResponse = Result<httpc_test::Response, httpc_test::Error>;

async fn run(action: impl Future<Output = TestResponse>) -> anyhow::Result<()> {
    Ok(action.await?.print().await?)
}

macro_rules! run {
    ($action:expr) => {
        run($action).await?
    };
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    run!(hc.do_get("/api/status"));

    run!(hc.do_post(
        "/api/login",
        json!({
            "email": "badguy",
            "password": "hackerz!"
        })
    ));

    let login = hc.do_post(
        "/api/login",
        json!({
            "email": "goodguy",
            "password": "password"
        }),
    );

    run!(login);

    run!(hc.do_post(
        "/api/lobby",
        json!({
            "name": "Cool lobby",
            "visibility": "Public"
        }),
    ));

    run!(hc.do_get("/api/lobbies"));

    Ok(())
}

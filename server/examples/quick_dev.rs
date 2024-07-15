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

    // can get status un-authenticated
    run!(hc.do_get("/api/status"));

    // Cannot get lobbies un-authenticated
    // run!(hc.do_get("/api/lobbies"));

    // Bad login attempt
    // run!(hc.do_post(
    //     "/api/login",
    //     json!({
    //         "email": "badguy",
    //         "password": "hackerz!"
    //     })
    // ));

    // Register a user, but too short of a password
    // run!(hc.do_post(
    //     "/api/user/register",
    //     json!({
    //         "display_name": "Good Guy",
    //         "email": "goodguy@contoso.com",
    //         "password": "password",
    //     })
    // ));

    // Register a user - good attempt
    run!(hc.do_post(
        "/api/user/register",
        json!({
            "display_name": "Good Guy",
            "email": "goodguy@contoso.com",
            "password": "password1234",
        })
    ));

    run!(hc.do_post(
        "/api/login",
        json!({
            "email": "goodguy@contoso.com",
            "password": "password1234"
        }),
    ));

    run!(hc.do_post(
        "/api/lobby",
        json!({
            "name": "Cool lobby",
            "visibility": "public"
        }),
    ));

    run!(hc.do_post(
        "/api/lobby",
        json!({
            "name": "Another lobby",
            "visibility": "private"
        }),
    ));

    run!(hc.do_get("/api/lobbies"));

    Ok(())
}

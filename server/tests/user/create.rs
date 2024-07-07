use std::time::{Duration, SystemTime};

use automata::model::user::{create, UserNewFields};

use crate::shared::db::TestDb;
use crate::shared::time::assert_within;

#[tokio::test]
async fn create_new_user() -> anyhow::Result<()> {
    let db = TestDb::new().await?;
    let fields = UserNewFields {
        handle: "john22".to_string(),
        email: "john@contoso.com".to_string(),
        password: "password".to_string(),
    };

    let now = SystemTime::now();
    let user = create(db.conn()?, fields).await?;

    dbg!(&user);

    assert_eq!(user.id, 1);
    assert_eq!(user.handle, "john22");
    assert_eq!(user.email, "john@contoso.com");
    assert_within(user.created_at, now, Duration::from_secs(5));
    assert_within(user.updated_at, now, Duration::from_secs(5));

    Ok(())
}

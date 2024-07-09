use std::time::{Duration, SystemTime};

use automata::model::user::{create, ErrorUser, UserNewFields};

use crate::shared::db::TestDb;
use crate::shared::time::assert_within;

#[tokio::test]
async fn create_user() -> anyhow::Result<()> {
    let db = TestDb::new().await?;
    let fields = UserNewFields {
        handle: "john22".to_string(),
        email: "john@contoso.com".to_string(),
        password: "password".to_string(),
    };

    let now = SystemTime::now();
    let user = create(db.conn()?, fields).await?;

    assert_eq!(user.id, 1);
    assert_eq!(user.handle, "john22");
    assert_eq!(user.email, "john@contoso.com");
    assert_within(user.created_at, now, Duration::from_secs(5));
    assert_within(user.updated_at, now, Duration::from_secs(5));

    Ok(())
}

#[tokio::test]
async fn create_user_with_existing_handle() -> anyhow::Result<()> {
    let db = TestDb::new().await?;
    let fields = UserNewFields {
        handle: "best_handle".into(),
        email: "bob@contoso.com".into(),
        password: "password".into(),
    };

    // Create the first user
    create(db.conn()?, fields).await?;

    // Create the second user with the same handle
    let fields = UserNewFields {
        handle: "best_handle".into(),
        email: "joe@contoso.com".into(),
        password: "password".into(),
    };

    let result = create(db.conn()?, fields).await;
    assert!(matches!(result, Err(ErrorUser::HandleAlreadyExists)));
    Ok(())
}

#[tokio::test]
async fn create_user_with_existing_email() -> anyhow::Result<()> {
    let db = TestDb::new().await?;
    let fields = UserNewFields {
        handle: "bob".into(),
        email: "bob@contoso.com".into(),
        password: "password".into(),
    };

    // Create the first user
    create(db.conn()?, fields).await?;

    // Create the second user with the same email
    let fields = UserNewFields {
        handle: "joe".into(),
        email: "bob@contoso.com".into(),
        password: "password".into(),
    };

    let result = create(db.conn()?, fields).await;
    assert_eq!(result, Err(ErrorUser::EmailAlreadyExists));
    Ok(())
}

#[tokio::test]
async fn create_user_with_invalid_email() -> anyhow::Result<()> {
    let db = TestDb::new().await?;
    let fields = UserNewFields {
        handle: "bob".into(),
        email: "bob".into(),
        password: "password".into(),
    };

    let result = create(db.conn()?, fields).await;
    assert_eq!(result, Err(ErrorUser::InvalidEmail));
    Ok(())
}

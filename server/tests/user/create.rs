use std::time::{Duration, SystemTime};

use rustwebapp::model::user::{create, ErrorPassword, ErrorUser, UserNewFields};

use crate::shared::db::TestDb;
use crate::shared::time::assert_within;

#[tokio::test]
async fn create_user() -> anyhow::Result<()> {
    let db = TestDb::new().await?;
    let fields = UserNewFields {
        display_name: "john22".into(),
        email: "john@contoso.com".into(),
        password: "password1234".into(),
    };

    let now = SystemTime::now();
    let user = create(db.conn()?, fields).await?;

    assert_eq!(user.id, 1);
    assert_eq!(user.display_name, "john22");
    assert_eq!(user.email, "john@contoso.com");
    assert_within(user.created_at, now, Duration::from_secs(5));
    assert_within(user.updated_at, now, Duration::from_secs(5));

    Ok(())
}

#[tokio::test]
async fn create_user_with_existing_display_name() -> anyhow::Result<()> {
    let db = TestDb::new().await?;
    let fields = UserNewFields {
        display_name: "Bob the Builder".into(),
        email: "bob@contoso.com".into(),
        password: "password1234".into(),
    };

    // Create the first user
    let _bob = create(db.conn()?, fields).await?;

    // Create the second user with the same handle
    let fields = UserNewFields {
        display_name: "Bob the Builder".into(),
        email: "joe@contoso.com".into(),
        password: "password1234".into(),
    };

    let joe = create(db.conn()?, fields)
        .await
        .expect("Joe can be created with same display name");
    assert_eq!(joe.display_name, "Bob the Builder");
    Ok(())
}

#[tokio::test]
async fn create_user_with_existing_email() -> anyhow::Result<()> {
    let db = TestDb::new().await?;
    let fields = UserNewFields {
        display_name: "bob".into(),
        email: "bob@contoso.com".into(),
        password: "password1234".into(),
    };

    // Create the first user
    create(db.conn()?, fields).await?;

    // Create the second user with the same email (different case)
    let fields = UserNewFields {
        display_name: "joe".into(),
        email: "BoB@Contoso.com".into(),
        password: "password1234".into(),
    };

    let result = create(db.conn()?, fields).await;
    assert_eq!(result, Err(ErrorUser::EmailAlreadyExists));
    Ok(())
}

#[tokio::test]
async fn create_user_with_invalid_email() -> anyhow::Result<()> {
    let db = TestDb::new().await?;
    let fields = UserNewFields {
        display_name: "bob".into(),
        email: "bob".into(),
        password: "password1234".into(),
    };

    let result = create(db.conn()?, fields).await;
    assert_eq!(result, Err(ErrorUser::InvalidEmail));
    Ok(())
}

#[tokio::test]
async fn create_user_with_short_password() -> anyhow::Result<()> {
    let db = TestDb::new().await?;
    let fields = UserNewFields {
        display_name: "bob".into(),
        email: "bob@contoso.com".into(),
        password: "pass".into(),
    };

    let result = create(db.conn()?, fields).await;
    assert_eq!(result, Err(ErrorUser::Password(ErrorPassword::TooShort)));
    Ok(())
}

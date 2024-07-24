use automata::{
    model::user::{self, create, UserNewFields},
    service::jwt::Claims,
};

use crate::shared::db::TestDb;

#[tokio::test]
async fn login_new_user() -> anyhow::Result<()> {
    let db = TestDb::new().await?;
    let fields = UserNewFields {
        display_name: "bob".into(),
        email: "bob@gmail.com".into(),
        password: "password1234".into(),
    };

    // Create the user
    create(db.conn()?, fields).await?;

    // Login the user
    let user_claims: Claims = user::login(db.conn()?, "bob@gmail.com", "password1234").await?;
    assert_eq!(user_claims.sub, 1);
    assert_eq!(user_claims.display_name, "bob");
    assert_eq!(user_claims.email, "bob@gmail.com");
    Ok(())
}

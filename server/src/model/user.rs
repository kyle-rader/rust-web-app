use std::time::SystemTime;

use diesel::{deserialize::Queryable, Connection, Selectable};
use serde::{Deserialize, Serialize};

// region: -- Account Types
#[derive(Debug, Clone, Serialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub email: String,
    pub handle: String,
    pub password_salt: String,
    pub password_hash: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Debug, Deserialize)]
pub struct UserNewFields {
    pub email: String,
    pub handle: String,
    pub password: String,
}

// endregion

// region: -- Account Controller
#[derive(Debug, thiserror::Error)]
pub enum ErrorUser {
    #[error("Internal server error")]
    Internal,

    #[error(transparent)]
    Time(#[from] crate::service::time::ErrorTime),

    #[error("Account not found")]
    NotFound,
}

#[derive(Default, Clone)]
pub struct ControllerUser {}

// TODO: Connect to database
impl ControllerUser {
    pub async fn create(
        &self,
        connection: impl Connection,
        UserNewFields {
            email,
            handle,
            password,
        }: UserNewFields,
    ) -> Result<User, ErrorUser> {
        // TODO: Confirm that email is unique

        // TODO: Create real password salt and hash
        let salt = "salt".to_string();
        let now = SystemTime::now();

        todo!("Connect to database")
    }

    pub async fn get_account(&self, id: u64) -> Result<User, ErrorUser> {
        todo!("Connect to database")
    }
}

// endregion

#[cfg(test)]
mod tests {
    use crate::model::user::{ControllerUser, ErrorUser, UserNewFields};

    #[tokio::test]
    async fn test_create_account() -> anyhow::Result<()> {
        let new_user = UserNewFields {
            email: "joe@contoso.com".to_string(),
            handle: "joe".to_string(),
            password: "password".to_string(),
        };

        let account_ctl = ControllerUser::default();
        todo!("Connect to database")
        // let account = account_ctl.create().await?;

        // assert_eq!(account.id, 0);

        // // Can get created account:
        // let account = account_ctl.get_account(0).await?;
        // assert_eq!(account.id, 0);
        // assert_eq!(account.email, "joe@contoso.com");
        // assert_eq!(account.handle, "joe");
        // assert_eq!(account.password_hash, "passwordsalt");
        // Ok(())
    }

    #[tokio::test]
    async fn get_missing_account() -> anyhow::Result<()> {
        let account_ctl = ControllerUser::default();
        let account = account_ctl.get_account(0).await;

        assert!(matches!(account, Err(ErrorUser::NotFound)));
        Ok(())
    }
}

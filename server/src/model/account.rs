use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

// region: -- Account Types
#[derive(Debug, Clone, Serialize)]
pub struct Account {
    pub id: u64,
    pub email: String,
    pub handle: String,
    pub password_salt: String,
    pub password_hash: String,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Debug, Deserialize)]
pub struct AccountForCreate {
    pub email: String,
    pub handle: String,
    pub password: String,
}

// endregion

// region: -- Account Controller
#[derive(Debug, thiserror::Error)]
pub enum ErrorAccount {
    #[error("Internal server error")]
    Internal,

    #[error(transparent)]
    Time(#[from] crate::service::time::ErrorTime),

    #[error("Account not found")]
    NotFound,
}

#[derive(Debug, Clone)]
pub struct AccountController {
    accounts: Arc<Mutex<Vec<Option<Account>>>>,
}

// TODO: Connect to database
impl AccountController {
    pub async fn new() -> Result<Self, ErrorAccount> {
        let accounts = Arc::default();
        Ok(Self { accounts })
    }

    pub async fn create_account(
        &self,
        AccountForCreate {
            email,
            handle,
            password,
        }: AccountForCreate,
    ) -> Result<Account, ErrorAccount> {
        let mut accounts = self.accounts.lock().map_err(|_| ErrorAccount::Internal)?;

        let id = accounts.len() as u64;
        let salt = "salt".to_string();
        let now = crate::service::time::now_unix()?;

        let account = Account {
            id,
            email,
            handle,
            password_hash: format!("{password}{salt}"),
            password_salt: salt,
            created_at: now,
            updated_at: now,
        };

        accounts.push(Some(account.clone()));
        Ok(account)
    }

    pub async fn get_account(&self, id: u64) -> Result<Account, ErrorAccount> {
        let accounts = self.accounts.lock().map_err(|_| ErrorAccount::Internal)?;

        let account = accounts
            .get(id as usize)
            .ok_or(ErrorAccount::NotFound)?
            .as_ref()
            .ok_or(ErrorAccount::NotFound)?;

        Ok(account.clone())
    }
}

// endregion

#[cfg(test)]
mod tests {
    use crate::model::account::{AccountController, AccountForCreate, ErrorAccount};

    #[tokio::test]
    async fn test_create_account() -> anyhow::Result<()> {
        let account_ctl = AccountController::new().await?;
        let account = account_ctl
            .create_account(AccountForCreate {
                email: "joe@contoso.com".to_string(),
                handle: "joe".to_string(),
                password: "password".to_string(),
            })
            .await?;

        assert_eq!(account.id, 0);

        // Can get created account:
        let account = account_ctl.get_account(0).await?;
        assert_eq!(account.id, 0);
        assert_eq!(account.email, "joe@contoso.com");
        assert_eq!(account.handle, "joe");
        assert_eq!(account.password_hash, "passwordsalt");
        Ok(())
    }

    #[tokio::test]
    async fn get_missing_account() -> anyhow::Result<()> {
        let account_ctl = AccountController::new().await?;
        let account = account_ctl.get_account(0).await;

        assert!(matches!(account, Err(ErrorAccount::NotFound)));
        Ok(())
    }
}

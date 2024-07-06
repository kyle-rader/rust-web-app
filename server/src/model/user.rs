use std::time::SystemTime;

use diesel::{deserialize::Queryable, prelude::Insertable, Connection, Selectable};
use serde::{Deserialize, Serialize};

use crate::{schema::users, service};

const USER_SALT_LEN: usize = 16;

// region: -- Account Types
#[derive(Debug, Clone, Serialize, Queryable, Selectable, Insertable)]
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

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::users)]
struct UserForInsert {
    email: String,
    handle: String,
    password_salt: String,
    password_hash: String,
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

pub fn create(conn: impl Connection, new_user: UserNewFields) -> Result<User, ErrorUser> {
    // TODO: Confirm that email is unique

    // TODO: Create real password salt and hash
    let salt = service::crypto::salt(USER_SALT_LEN);
    let now = SystemTime::now();

    todo!("Connect to database")
}

// endregion

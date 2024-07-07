use diesel::{
    deserialize::Queryable, prelude::Insertable, r2d2::PooledConnection, Connection, PgConnection,
    Selectable,
};
use diesel::{insert_into, prelude::*};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

use crate::schema::users::id;
use crate::{schema::users, service};

use super::PooledPgConnection;

const USER_SALT_LEN: usize = 16;

// region: -- Account Types
#[derive(Debug, Clone, Serialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub handle: String,
    pub email: String,
    pub password_hash: String,
    pub password_salt: String,
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

    #[error("Account not found")]
    NotFound,
}

pub async fn create(
    mut conn: PooledPgConnection,
    fields: UserNewFields,
) -> Result<User, ErrorUser> {
    // TODO: Confirm that email is unique

    // TODO: Create real password salt and hash
    let salt = service::crypto::salt(USER_SALT_LEN);
    let user_insert = UserForInsert {
        handle: fields.handle,
        email: fields.email,
        password_hash: format!("{}{}", fields.password, salt),
        password_salt: salt,
    };

    dbg!(&user_insert);

    diesel::insert_into(users::table)
        .values(user_insert)
        .get_result::<User>(&mut conn)
        .map_err(|_| ErrorUser::Internal)
}

// endregion

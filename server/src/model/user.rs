use diesel::prelude::*;
use diesel::result::DatabaseErrorKind;
use diesel::result::Error::DatabaseError;
use diesel::{deserialize::Queryable, prelude::Insertable, Selectable};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::time::SystemTime;

use crate::{schema::users, service};

use super::PooledPgConnection;

const USER_SALT_LEN: usize = 32;
static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
        .expect("Failed to compile email regex")
});

// region: -- Account Types
#[derive(Debug, Clone, PartialEq, Serialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub handle: String,
    pub email: String,
    pub password_salt: String,
    pub password_hash: Vec<u8>,
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
    password_hash: Vec<u8>,
}

impl From<UserNewFields> for UserForInsert {
    fn from(fields: UserNewFields) -> Self {
        let password_salt = service::crypto::salt(USER_SALT_LEN);
        let password_salted = format!("{}{}", fields.password, password_salt);
        let password_hash = sha2::Sha384::digest(password_salted.as_bytes()).to_vec();

        UserForInsert {
            email: fields.email,
            handle: fields.handle,
            password_salt,
            password_hash,
        }
    }
}
// endregion

// region: -- Account Controller
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ErrorUser {
    #[error("Internal server error")]
    Internal,

    #[error("Account not found")]
    NotFound,

    #[error("Handle already exists")]
    HandleAlreadyExists,

    #[error("Email already exists")]
    EmailAlreadyExists,

    #[error("Email is invalid and must be in the format '[letters|numbers|symbols]@[letters|numbers].[letters]'")]
    InvalidEmail,
}

pub async fn create(
    mut conn: PooledPgConnection,
    fields: UserNewFields,
) -> Result<User, ErrorUser> {
    let user_insert: UserForInsert = fields.into();

    valid_email(&user_insert.email)?;

    diesel::insert_into(users::table)
        .values(user_insert)
        .get_result::<User>(&mut conn)
        .map_err(create_db_error_map)
}

fn valid_email(email: &str) -> Result<(), ErrorUser> {
    if EMAIL_REGEX.is_match(email) {
        Ok(())
    } else {
        Err(ErrorUser::InvalidEmail)
    }
}

fn create_db_error_map(error: diesel::result::Error) -> ErrorUser {
    match error {
        DatabaseError(DatabaseErrorKind::UniqueViolation, info) => info
            .constraint_name()
            .map(|constraint| match constraint {
                "users_email_key" => ErrorUser::EmailAlreadyExists,
                "users_handle_key" => ErrorUser::HandleAlreadyExists,
                _ => ErrorUser::Internal,
            })
            .unwrap_or(ErrorUser::Internal),
        _ => ErrorUser::Internal,
    }
}

// endregion

#[cfg(test)]
mod tests {
    use crate::model::user::USER_SALT_LEN;

    use super::{UserForInsert, UserNewFields};

    #[test]
    fn user_fields_into_user_for_insert() {
        let fields = UserNewFields {
            handle: "john89".into(),
            email: "john89@contoso.com".into(),
            password: "password".into(),
        };

        let user_insert: UserForInsert = fields.into();

        assert_eq!(user_insert.handle, "john89");
        assert_eq!(user_insert.email, "john89@contoso.com");
        assert_eq!(user_insert.password_salt.len(), USER_SALT_LEN);
        assert_ne!(user_insert.password_hash, Vec::<u8>::new());
    }
}

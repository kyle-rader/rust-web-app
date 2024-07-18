use core::fmt;
use diesel::prelude::*;
use diesel::result::DatabaseErrorKind;
use diesel::result::Error::DatabaseError;
use diesel::{deserialize::Queryable, prelude::Insertable, Selectable};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::time::SystemTime;
use tracing::trace;

use crate::db::DbConn;
use crate::service::jwt::Claims;
use crate::web::error::MainError;
use crate::{schema::users, service};

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
    pub display_name: String,
    pub email: String,
    pub password_salt: String,
    pub password_hash: Vec<u8>,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Deserialize)]
pub struct UserNewFields {
    pub display_name: String,
    pub email: String,
    pub password: String,
}

impl fmt::Debug for UserNewFields {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UserNewFields")
            .field("display_name", &self.display_name)
            .field("email", &self.email)
            .field(
                "password",
                &self.password.chars().map(|_| '*').collect::<String>(),
            )
            .finish()
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
struct UserForInsert {
    display_name: String,
    email: String,
    password_salt: String,
    password_hash: Vec<u8>,
}

impl std::fmt::Debug for UserForInsert {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UserForInsert")
            .field("display_name", &self.display_name)
            .field("email", &self.email)
            .field(
                "password_salt",
                &format!(
                    "{}...",
                    self.password_salt.chars().take(5).collect::<String>()
                ),
            )
            .field(
                "password_hash",
                &format!(
                    "{}...",
                    &self.password_hash[0..5]
                        .iter()
                        .map(|b| b.to_string())
                        .collect::<String>()
                ),
            )
            .finish()
    }
}

impl From<UserNewFields> for UserForInsert {
    fn from(fields: UserNewFields) -> Self {
        let password_salt = service::crypto::salt(USER_SALT_LEN);
        let password_salted = format!("{}{}", fields.password, password_salt);
        let password_hash = sha2::Sha384::digest(password_salted.as_bytes()).to_vec();

        UserForInsert {
            display_name: fields.display_name,
            email: fields.email.to_lowercase(),
            password_salt,
            password_hash,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct UserPublic {
    pub id: i32,
    pub display_name: String,
    pub email: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

impl From<User> for UserPublic {
    fn from(user: User) -> Self {
        UserPublic {
            id: user.id,
            display_name: user.display_name,
            email: user.email,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
// endregion

// region: -- Account Controller
#[derive(Debug, thiserror::Error, PartialEq, Clone, Serialize)]
pub enum ErrorUser {
    #[error("{0}")]
    Db(String),

    #[error("Account not found")]
    NotFound,

    #[error("Display name already exists")]
    DisplayNameAlreadyExists,

    #[error("Email already exists")]
    EmailAlreadyExists,

    #[error("Email is invalid and must be in the format '[letters|numbers|symbols]@[letters|numbers].[letters]'")]
    InvalidEmail,

    #[error("Password or email is invalid")]
    InvalidCredentials,

    #[error(transparent)]
    Password(#[from] ErrorPassword),
}

#[derive(Debug, thiserror::Error, PartialEq, Clone, Serialize)]
pub enum ErrorPassword {
    #[error("Password must be at least 12 characters long")]
    TooShort,
}

pub async fn create(mut conn: DbConn, fields: UserNewFields) -> Result<User, ErrorUser> {
    trace!("User Create:\n{fields:#?}");
    valid_password(&fields.password)?;
    valid_email(&fields.email)?;

    let user_insert: UserForInsert = fields.into();
    trace!("User Insert:\n{user_insert:#?}");

    diesel::insert_into(users::table)
        .values(user_insert)
        .get_result::<User>(&mut conn)
        .map_err(create_db_error_map)
}

pub async fn login(
    mut conn: DbConn,
    email: impl AsRef<str>,
    password: impl AsRef<str>,
) -> Result<Claims, MainError> {
    let user: User = users::table
        .filter(users::email.eq(email.as_ref()))
        .get_result::<User>(&mut conn)
        .map_err(|_| MainError::LoginFail)?;

    let password_salted = format!("{}{}", password.as_ref(), user.password_salt);
    let password_hash = sha2::Sha384::digest(password_salted.as_bytes()).to_vec();

    if password_hash == user.password_hash {
        Ok(Claims::new(user.id as u64, user.display_name, user.email))
    } else {
        Err(MainError::LoginFail)
    }
}

fn valid_password(password: &str) -> Result<(), ErrorPassword> {
    if password.len() >= 12 {
        Ok(())
    } else {
        Err(ErrorPassword::TooShort)
    }
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
        DatabaseError(DatabaseErrorKind::UniqueViolation, info) => match info.constraint_name() {
            Some("users_email_key") => ErrorUser::EmailAlreadyExists,
            Some("users_display_name_key") => ErrorUser::DisplayNameAlreadyExists,
            Some(constraint) => ErrorUser::Db(format!("Unique violation {constraint}")),
            None => ErrorUser::Db(info.message().to_string()),
        },
        DatabaseError(kind, info) => ErrorUser::Db(format!("{kind:?} {}", info.message())),
        e => ErrorUser::Db(e.to_string()),
    }
}

// endregion

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::model::user::{valid_password, ErrorPassword, USER_SALT_LEN};

    use super::{UserForInsert, UserNewFields};

    #[test]
    fn user_fields_into_user_for_insert() {
        let fields = UserNewFields {
            display_name: "john89".into(),
            email: "JohN89@contoso.com".into(),
            password: "password".into(),
        };

        let user_insert: UserForInsert = fields.into();

        assert_eq!(user_insert.display_name, "john89");
        assert_eq!(user_insert.email, "john89@contoso.com");
        assert_eq!(user_insert.password_salt.len(), USER_SALT_LEN);
        assert_ne!(user_insert.password_hash, Vec::<u8>::new());
    }

    #[test]
    fn password_too_short() {
        assert_eq!(valid_password("1234567890"), Err(ErrorPassword::TooShort));
    }

    #[test_case("john@contoso.com")]
    #[test_case("john.doe@contoso.com")]
    #[test_case("john.doe@contoso.net")]
    #[test_case("john.doe@contoso.org")]
    #[test_case("john.doe@contoso.games")]
    #[test_case("john.doe-1980@contoso.games")]
    fn is_valid_email(email: &str) {
        assert!(super::valid_email(email).is_ok());
    }
}

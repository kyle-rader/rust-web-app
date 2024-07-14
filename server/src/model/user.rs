use diesel::prelude::*;
use diesel::result::DatabaseErrorKind;
use diesel::result::Error::DatabaseError;
use diesel::{deserialize::Queryable, prelude::Insertable, Selectable};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::time::SystemTime;

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

#[derive(Debug, Deserialize)]
pub struct UserNewFields {
    pub display_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::users)]
struct UserForInsert {
    display_name: String,
    email: String,
    password_salt: String,
    password_hash: Vec<u8>,
}

impl From<UserNewFields> for UserForInsert {
    fn from(fields: UserNewFields) -> Self {
        let password_salt = service::crypto::salt(USER_SALT_LEN);
        let password_salted = format!("{}{}", fields.password, password_salt);
        let password_hash = sha2::Sha384::digest(password_salted.as_bytes()).to_vec();

        UserForInsert {
            display_name: fields.display_name,
            email: fields.email,
            password_salt,
            password_hash,
        }
    }
}
// endregion

// region: -- Account Controller
#[derive(Debug, thiserror::Error, PartialEq, Clone)]
pub enum ErrorUser {
    #[error("Internal server error")]
    Internal,

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

#[derive(Debug, thiserror::Error, PartialEq, Clone)]
pub enum ErrorPassword {
    #[error("Password must be at least 12 characters long")]
    TooShort,
}

pub async fn create(mut conn: DbConn, fields: UserNewFields) -> Result<User, ErrorUser> {
    valid_password(&fields.password)?;
    let user_insert: UserForInsert = fields.into();
    valid_email(&user_insert.email)?;

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
        DatabaseError(DatabaseErrorKind::UniqueViolation, info) => info
            .constraint_name()
            .map(|constraint| match constraint {
                "users_email_key" => ErrorUser::EmailAlreadyExists,
                "users_display_name_key" => ErrorUser::DisplayNameAlreadyExists,
                _ => ErrorUser::Internal,
            })
            .unwrap_or(ErrorUser::Internal),
        _ => ErrorUser::Internal,
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
            email: "john89@contoso.com".into(),
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

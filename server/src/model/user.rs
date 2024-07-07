use diesel::prelude::*;
use diesel::{deserialize::Queryable, prelude::Insertable, Selectable};
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::time::SystemTime;

use crate::{schema::users, service};

use super::PooledPgConnection;

const USER_SALT_LEN: usize = 32;

// region: -- Account Types
#[derive(Debug, Clone, Serialize, Queryable, Selectable)]
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

    let user_insert: UserForInsert = fields.into();

    dbg!(&user_insert);

    diesel::insert_into(users::table)
        .values(user_insert)
        .get_result::<User>(&mut conn)
        .map_err(|_| ErrorUser::Internal)
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

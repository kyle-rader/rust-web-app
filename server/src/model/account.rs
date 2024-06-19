// region: -- Account Types
#[derive(Debug)]
pub struct Account {
    pub id: i32,
    pub email: String,
    pub handle: String,
    pub password_salt: String,
    pub password_hash: String,
    pub created_at: u64,
    pub updated_at: u64,
}

pub struct AccountForCreate {
    pub email: String,
    pub handle: String,
    pub password: String,
}
// endregion

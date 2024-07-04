#[derive(Debug, Clone)]
pub struct Ctx {
    pub account_id: u64,
}

impl Ctx {
    pub fn new(account_id: u64) -> Self {
        Self { account_id }
    }
}

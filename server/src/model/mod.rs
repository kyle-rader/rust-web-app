use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};

pub mod lobby;
pub mod user;

pub type PooledPgConnection = PooledConnection<ConnectionManager<PgConnection>>;

use diesel::{
    migration,
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use tracing::{debug, error, info};

use crate::web::error::MainError;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConn = PooledConnection<ConnectionManager<PgConnection>>;
const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn get_db_pool() -> anyhow::Result<DbPool> {
    debug!("🛜  Connecting to database...");
    let db_uri = std::env::var("DATABASE_URL").map_err(|_e| {
        let msg = "DATABASE_URL not set";
        error!(msg);
        anyhow::anyhow!(msg)
    })?;

    debug!("{db_uri}");

    let manager = ConnectionManager::<PgConnection>::new(db_uri);
    Pool::builder().build(manager).map_err(|e| {
        error!("Failed to create pool: {:?}", e);
        anyhow::anyhow!(e)
    })
}

pub fn get_db_conn(pool: &DbPool) -> Result<DbConn, MainError> {
    pool.get()
        .map_err(|e| MainError::InternalWithMsg(e.to_string()))
}

pub fn run_migrations(mut conn: DbConn) -> anyhow::Result<()> {
    info!("Running database migrations...");
    let migrations: Vec<migration::MigrationVersion> = conn
        .run_pending_migrations(MIGRATIONS)
        .map_err(|e| anyhow::anyhow!(e))?;

    if migrations.is_empty() {
        info!("⚠️  No migrations to run");
    }

    for migration in migrations {
        info!("-  Migration: {}", migration);
    }

    Ok(())
}

use diesel::{
    r2d2::{ConnectionManager, ManageConnection, Pool, PooledConnection},
    PgConnection,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use testcontainers::{runners::AsyncRunner, ContainerAsync};
use testcontainers_modules::postgres;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub struct TestDb {
    pub container: ContainerAsync<postgres::Postgres>,
    pub pool: Pool<ConnectionManager<PgConnection>>,
    pub connection_uri: String,
}

impl TestDb {
    pub async fn new() -> anyhow::Result<Self> {
        // Create Postgres Docker container and connection URI
        let container = postgres::Postgres::default().start().await?;
        let host_port = container.get_host_port_ipv4(5432).await?;
        let connection_uri = format!("postgres://postgres:postgres@127.0.0.1:{host_port}/postgres");

        // Create database pool
        let manager = ConnectionManager::<PgConnection>::new(&connection_uri);
        let pool = Pool::builder().max_size(1).build(manager)?;

        // Run migrations
        let mut conn = pool.get()?;

        conn.run_pending_migrations(MIGRATIONS)
            .map_err(|e| anyhow::anyhow!(e))?;

        drop(conn);

        Ok(TestDb {
            container,
            pool,
            connection_uri,
        })
    }

    pub fn conn(&self) -> anyhow::Result<PooledConnection<ConnectionManager<PgConnection>>> {
        self.pool.get().map_err(|e| anyhow::anyhow!(e))
    }
}

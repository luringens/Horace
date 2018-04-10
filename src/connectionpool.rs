use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use r2d2::{self, Pool, PooledConnection};
use typemap::Key;
use env;

#[derive(Clone)]
pub struct ConnectionPool {
    pub pool: Pool<PostgresConnectionManager>,
}

impl Default for ConnectionPool {
    fn default() -> Self {
        ConnectionPool::new()
    }
}

impl Key for ConnectionPool {
    type Value = ConnectionPool;
}

impl ConnectionPool {
    pub fn new() -> ConnectionPool {
        let connstring = env::var("POSTGRES_CONNSTRING")
            .expect("Expected a PostgreSQL connection string in the environment");
        let manager = PostgresConnectionManager::new(connstring, TlsMode::None)
            .expect("Failed to set up Postgres connection manager.");
        let pool = Pool::new(manager).expect("Failed to set up R2D2 connection pool.");

        ConnectionPool { pool }
    }

    pub fn get_conn(&mut self) -> Result<PooledConnection<PostgresConnectionManager>, r2d2::Error> {
        self.pool.get()
    }
}

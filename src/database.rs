use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;

const DATABASE_DEFINITION_SQL: &str = include_str!("../database.sql");

#[derive(Clone)]
pub struct Database {
    pool: Pool<SqliteConnectionManager>,
}

impl Database {
    pub fn new(database_file: &str) -> Self {
        let manager = SqliteConnectionManager::file(database_file)
            .with_init(|c| c.execute_batch("PRAGMA busy_timeout = 60000; PRAGMA journal_mode = WAL; PRAGMA synchronous = NORMAL; PRAGMA foreign_keys = 1; PRAGMA auto_vacuum = INCREMENTAL; PRAGMA recursive_triggers = 1;"));

        let pool = r2d2::Pool::new(manager).expect("Error occurred connecting to database");
        pool.get()
            .expect("Error occurred getting database connection for initialization")
            .execute_batch(DATABASE_DEFINITION_SQL)
            .expect("Error occurred while running database initialization commands");

        Self { pool }
    }

    pub fn get(&self) -> PooledConnection<SqliteConnectionManager> {
        self.pool
            .get()
            .expect("Error occurred getting database connection from connection pool")
    }
}

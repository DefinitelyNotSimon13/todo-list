use color_eyre::Result;
use diesel::{r2d2::ConnectionManager, Connection, PgConnection};
use r2d2::PooledConnection;

use crate::{DbConnection, DbPool};

pub struct Database {
    connection_pool: DbPool,
}

impl Database {
    pub fn new(url: &str) -> Result<Self> {
        let manager = ConnectionManager::<PgConnection>::new(url);
        let connection_pool = r2d2::Pool::builder().build(manager)?;
        Ok(Self { connection_pool })
    }

    pub fn get_connection(&mut self) -> DbConnection {
        self.connection_pool
            .get()
            .expect("failed to get connection from pool")
    }

    pub fn get_connection_pool(&mut self) -> DbPool {
        self.connection_pool.clone()
    }
}

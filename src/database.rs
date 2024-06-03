use color_eyre::Result;
use std::{env, path::Path};
use uuid::Uuid;

use sqlx::{
    postgres::{PgPoolOptions, PgRow},
    query, query_as, Connection, PgPool, Postgres,
};

pub struct Database {
    connection: PgPool,
}

impl Database {
    pub async fn new(url: &str) -> Result<Self> {
        let connection = PgPoolOptions::new().max_connections(5).connect(url).await?;
        Ok(Self { connection })
    }

    pub fn get_connection(&self) -> &PgPool {
        &self.connection
    }
}

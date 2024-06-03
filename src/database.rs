use color_eyre::Result;



use sqlx::{
    postgres::{PgPoolOptions}, Connection, PgPool,
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

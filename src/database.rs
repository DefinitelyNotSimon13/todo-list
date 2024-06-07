use color_eyre::Result;
use diesel::{Connection, PgConnection};

pub struct Database {
    connection: PgConnection,
}

impl Database {
    pub fn new(url: &str) -> Result<Self> {
        Ok(Self {
            connection: PgConnection::establish(url)?,
        })
    }

    pub fn get_connection(&mut self) -> &mut PgConnection {
        &mut self.connection
    }
}

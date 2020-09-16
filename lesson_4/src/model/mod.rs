use postgres::NoTls;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Movie {
    pub title: String,
    pub genre: String,
}

#[derive(Debug)]
pub struct Storage {
    pub database: Pool<PostgresConnectionManager<NoTls>>
}
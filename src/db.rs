use core::error;
use std::env;

use sqlx::{PgPool, postgres::PgPoolOptions};

#[derive(Clone)]
pub struct Appstate {
    pub db: PgPool,
}

impl Appstate {
    pub async fn connect_db() -> Result<Self, Box<dyn error::Error>> {
        dotenvy::dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("Invalid Database url");

        let pool = PgPoolOptions::new()
            .max_connections(6)
            .connect(&database_url)
            .await
            .expect("Failed to connect to Postgress");

        Ok(Self { db: pool })
    }
}

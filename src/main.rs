pub mod domain;
pub mod infrastructure;

use infrastructure::database::pg_connection::create_pool;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    dotenvy::dotenv().ok();

    let db_url = env::var("DATABASE_URL")?;
    let pool = create_pool(&db_url).await?;

    println!("Database connected");

    let _pool = pool;

    Ok(())
}

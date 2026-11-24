pub mod bootstrap;
pub mod infrastructure;
pub mod modules;

use infrastructure::database::pg_connection::create_pool;
use std::env;

use crate::bootstrap::create_app;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    dotenvy::dotenv().ok();

    let db_url = env::var("DATABASE_URL")?;
    let pool = create_pool(&db_url).await?;
    println!("database connected");

    let port = env::var("APP_PORT")
        .unwrap_or_else(|_| 7000.to_string())
        .parse::<u16>()
        .expect("APP_PORT must be a valid port");

    let app = create_app(pool).await?;
    infrastructure::http::server::start_server(app, port).await?;

    Ok(())
}

use std::env;

use axum::Router;
use tokio::net::TcpListener;

pub async fn start_server(app: Router, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("0.0.0.0:{port}");
    let listener = TcpListener::bind(&addr).await?;

    let app_name = env::var("APP_ALIAS").unwrap();
    println!("{app_name} running on port {port}");

    tracing::info!("Server running on {}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}

use std::sync::Arc;

use axum::Router;
use tokio::net::TcpListener;

pub mod auth;
pub mod composition_root;
pub mod positions;
pub mod shared;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let config = Arc::new(shared::config::Config::default());
    let pool = composition_root::get_or_create_postgres_pool(&config).await;
    let user_repo = Box::new(composition_root::create_user_postgres_repository(pool.clone()).await);
    let auth_service = composition_root::create_auth_service(user_repo).await;
    let position_repo = Box::new(composition_root::create_position_postgres_repository(pool).await);
    let position_service = composition_root::create_position_service(position_repo).await;

    let app = Router::new()
        .nest(
            "/positions",
            positions::presentation::routes::create_position_routes(
                Arc::new(position_service),
                config.clone(),
            ),
        )
        .nest(
            "/auth",
            auth::presentation::routes::create_auth_routes(Arc::new(auth_service), config.clone()),
        );

    let addr = format!("{}:{}", config.server_host, config.server_port);
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("Listening on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}

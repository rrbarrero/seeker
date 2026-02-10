use std::sync::Arc;
use utoipa::OpenApi;

use axum::http::HeaderName;
use axum::{Router, middleware};
use tokio::net::TcpListener;
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, RequestId, SetRequestIdLayer},
    trace::{DefaultOnResponse, TraceLayer},
};

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
    let auth_service =
        composition_root::create_auth_service(user_repo, pool.clone(), config.clone()).await;
    let position_repo =
        Box::new(composition_root::create_position_postgres_repository(pool.clone()).await);
    let comment_repo =
        Box::new(composition_root::create_comment_postgres_repository(pool.clone()).await);
    let position_service = composition_root::create_position_service(position_repo).await;
    let comment_service = composition_root::create_comment_service(comment_repo).await;
    let observability = if config.observability_enabled {
        match shared::infrastructure::observability::init_observability(
            &config.service_name,
            &config.otlp_endpoint,
        ) {
            Ok(obs) => Some(obs),
            Err(err) => {
                eprintln!("Observability disabled: {err}");
                None
            }
        }
    } else {
        None
    };

    let user_repo_checker =
        Box::new(composition_root::create_user_postgres_repository(pool.clone()).await);
    let user_checker = Arc::new(
        auth::application::user_status_checker::UserStatusCheckerImpl::new(Arc::new(
            user_repo_checker,
        )),
    );

    let app = Router::new()
        .merge(utoipa_swagger_ui::SwaggerUi::new("/swagger-ui").url(
            "/api-docs/openapi.json",
            shared::presentation::openapi::ApiDoc::openapi(),
        ))
        .nest(
            "/positions",
            positions::presentation::routes::create_position_routes(
                Arc::new(position_service),
                Arc::new(comment_service),
                config.clone(),
                user_checker,
            ),
        )
        .nest(
            "/auth",
            auth::presentation::routes::create_auth_routes(Arc::new(auth_service), config.clone()),
        )
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_origin(
                    config
                        .allowed_origin
                        .parse::<axum::http::HeaderValue>()
                        .unwrap(),
                )
                .allow_headers([
                    axum::http::header::CONTENT_TYPE,
                    axum::http::header::AUTHORIZATION,
                    HeaderName::from_static("x-request-id"),
                ])
                .allow_methods([
                    axum::http::Method::GET,
                    axum::http::Method::POST,
                    axum::http::Method::PUT,
                    axum::http::Method::DELETE,
                ])
                .expose_headers([HeaderName::from_static("x-request-id")]),
        );
    let app = if let Some(observability) = observability.clone() {
        let trace_layer = TraceLayer::new_for_http()
            .make_span_with(|request: &axum::http::Request<_>| {
                let request_id = request
                    .extensions()
                    .get::<RequestId>()
                    .and_then(|id| id.header_value().to_str().ok())
                    .unwrap_or("-");
                tracing::span!(
                    tracing::Level::WARN,
                    "http_request",
                    request_id = %request_id,
                    http.method = %request.method(),
                    http.route = %request.uri().path()
                )
            })
            .on_response(DefaultOnResponse::new().level(tracing::Level::INFO));

        app.layer(middleware::from_fn_with_state(
            observability,
            shared::infrastructure::http::observability_middleware::request_observability,
        ))
        .layer(trace_layer)
        .layer(PropagateRequestIdLayer::new(HeaderName::from_static(
            "x-request-id",
        )))
        .layer(SetRequestIdLayer::new(
            HeaderName::from_static("x-request-id"),
            MakeRequestUuid,
        ))
    } else {
        app
    };

    let addr = format!("{}:{}", config.server_host, config.server_port);
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("Listening on http://{}", addr);
    let result = axum::serve(listener, app).await;
    if let Some(ref observability) = observability {
        shared::infrastructure::observability::shutdown_observability(observability);
    }
    result.unwrap();
}

pub mod api;
pub mod database;
pub mod helpers;
pub mod services;

use std::{net::SocketAddr, path::Path, sync::Arc};

use axum::{
    http::{header, Method},
    middleware,
    routing::get,
    Router,
};
use helpers::api_request_logging;
use tokio::{net::TcpListener, signal};
use tower::ServiceBuilder;
use tower_http::{
    catch_panic::CatchPanicLayer,
    compression::CompressionLayer,
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
};

use database::Database;
use services::{audit::AuditService, auth::AuthService, users::UsersService};

const STATIC_FILE_ROOT: &str = "./client/dist/";
const STATIC_FILE_INDEX: &str = "index.html";
const DATABASE_FILE: &str = "./database.sqlite3";
const ENABLE_API_REQUEST_LOGGING: bool = true;

const CORS_ALLOWED_ORIGINS: &[&str] = &["http://localhost:5173", "http://127.0.0.1:5173"];

pub struct AppState {
    pub database: Database,
    pub audit_service: AuditService,
    pub auth_service: AuthService,
    pub users_service: UsersService,
}

pub async fn license() -> &'static str {
    include_str!("../LICENSE")
}
pub async fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[tokio::main]
pub async fn main() {
    let database = Database::new(DATABASE_FILE);
    let app_state = Arc::new(AppState {
        audit_service: AuditService::new(database.clone()),
        auth_service: AuthService::new(database.clone()),
        users_service: UsersService::new(database.clone()),
        database,
    });

    let static_file_index = Path::new(STATIC_FILE_ROOT).join(STATIC_FILE_INDEX);

    let cors_origins: Vec<_> = CORS_ALLOWED_ORIGINS
        .iter()
        .map(|x| x.parse().unwrap())
        .collect();

    let app = Router::new()
        .route_service("/", ServeFile::new(&static_file_index))
        .route_service(
            "/*path",
            ServeDir::new(STATIC_FILE_ROOT).fallback(ServeFile::new(&static_file_index)),
        )
        .route(
            "/about/version",
            get(version),
        )
        .route(
            "/about/license",
            get(license),
        )
        .nest(
            "/api",
            if ENABLE_API_REQUEST_LOGGING {
                api::route().layer(middleware::from_fn_with_state(
                    app_state.clone(),
                    api_request_logging::request_logging,
                ))
            } else {
                api::route()
            },
        )
        .layer(
            ServiceBuilder::new()
                .layer(
                    CorsLayer::new()
                        .allow_origin(cors_origins)
                        .allow_credentials(true)
                        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
                        .allow_methods([
                            Method::GET,
                            Method::POST,
                            Method::PUT,
                            Method::PATCH,
                            Method::DELETE,
                        ]),
                )
                .layer(CatchPanicLayer::new())
                .layer(CompressionLayer::new()),
        )
        .with_state(app_state.clone());

    let address = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = TcpListener::bind(address).await.unwrap();

    app_state.audit_service.log(None, "startup");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    app_state.audit_service.log(None, "shutdown");
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c().await.unwrap();
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .unwrap()
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

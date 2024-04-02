pub mod api;
pub mod database;
pub mod services;

use std::{net::SocketAddr, path::Path, sync::Arc};

use axum::Router;
use tokio::{net::TcpListener, signal};
use tower::ServiceBuilder;
use tower_http::{
    catch_panic::CatchPanicLayer,
    compression::CompressionLayer,
    services::{ServeDir, ServeFile},
};

use database::Database;
use services::auth::Auth;

const STATIC_FILE_ROOT: &str = "./client/dist/";
const STATIC_FILE_INDEX: &str = "index.html";
const DATABASE_FILE: &str = "./database.sqlite3";

pub struct AppState {
    pub database: Database,
    pub auth: Auth,
}

#[tokio::main]
pub async fn main() {
    let database = Database::new(DATABASE_FILE);
    let app_state = Arc::new(AppState {
        auth: Auth::new(database.clone()),
        database,
    });

    let static_file_index = Path::new(STATIC_FILE_ROOT).join(STATIC_FILE_INDEX);

    let app = Router::new()
        .route_service("/", ServeFile::new(&static_file_index))
        .route_service(
            "/*path",
            ServeDir::new(STATIC_FILE_ROOT).fallback(ServeFile::new(&static_file_index)),
        )
        .nest("/api", api::route())
        .layer(
            ServiceBuilder::new()
                .layer(CatchPanicLayer::new())
                .layer(CompressionLayer::new()),
        )
        .with_state(app_state.clone());

    let address = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = TcpListener::bind(address).await.unwrap();

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
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

pub mod api;
pub mod config;
pub mod database;
pub mod helpers;
pub mod services;

use std::{net::SocketAddr, path::Path, sync::Arc};

use axum::{
    http::{header, HeaderValue, Method},
    middleware,
    routing::{get, post},
    Router,
};
use config::AppConfig;
use helpers::api_request_logging;
use tokio::{net::TcpListener, signal};
use tower::ServiceBuilder;
use tower_http::{
    catch_panic::CatchPanicLayer,
    compression::CompressionLayer,
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
    set_header::SetResponseHeaderLayer,
};

use database::Database;
use services::{
    audit::AuditService, auth::AuthService, competition_events::CompetitionEventsService,
    competitions::CompetitionsService, events::EventsService,
    group_participation::GroupParticipationsService, groups::GroupsService,
    score_calculators::ScoreCalculatorsService, scores::ScoresService,
    season_competitions::SeasonCompetitionsService, seasons::SeasonsService, teams::TeamsService,
    users::UsersService,
};

const CONFIG_FILE: &str = "./config.json";

pub struct AppState {
    pub config: AppConfig,
    pub database: Database,
    pub audit_service: AuditService,
    pub auth_service: AuthService,
    pub users_service: UsersService,
    pub score_calculators_service: ScoreCalculatorsService,
    pub seasons_service: SeasonsService,
    pub competitions_service: CompetitionsService,
    pub season_competitions_service: SeasonCompetitionsService,
    pub events_service: EventsService,
    pub competition_events_service: CompetitionEventsService,
    pub groups_service: GroupsService,
    pub group_participations_service: GroupParticipationsService,
    pub teams_service: TeamsService,
    pub scores_service: ScoresService,
}

pub async fn ping(request: String) -> String {
    request
}
pub async fn license() -> &'static str {
    include_str!("../LICENSE")
}
pub async fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[tokio::main]
pub async fn main() {
    let config = AppConfig::load(CONFIG_FILE).await;

    let database = Database::new(&config);

    let app_state = Arc::new(AppState {
        audit_service: AuditService::new(database.clone()),
        auth_service: AuthService::new(database.clone()),
        users_service: UsersService::new(database.clone()),
        score_calculators_service: ScoreCalculatorsService::new(database.clone()),
        seasons_service: SeasonsService::new(database.clone()),
        competitions_service: CompetitionsService::new(database.clone()),
        season_competitions_service: SeasonCompetitionsService::new(database.clone()),
        events_service: EventsService::new(database.clone()),
        competition_events_service: CompetitionEventsService::new(database.clone()),
        groups_service: GroupsService::new(database.clone()),
        group_participations_service: GroupParticipationsService::new(database.clone()),
        teams_service: TeamsService::new(database.clone()),
        scores_service: ScoresService::new(database.clone()),
        database,
        config: config.clone(),
    });

    let static_file_index = Path::new(&config.static_file_root).join(config.static_file_index);

    let cors_origins: Vec<_> = config
        .cors_allowed_origins
        .iter()
        .map(|x| x.parse().unwrap())
        .collect();

    let app = Router::new()
        .nest(
            "/",
            Router::new()
                .route_service("/", ServeFile::new(&static_file_index))
                .route_service(
                    "/*path",
                    ServeDir::new(&config.static_file_root)
                        .fallback(ServeFile::new(&static_file_index)),
                )
                .layer(SetResponseHeaderLayer::if_not_present(
                    header::CACHE_CONTROL,
                    HeaderValue::from_str(&format!("max-age={}", config.http_caching_max_age))
                        .unwrap(),
                )),
        )
        .nest(
            "/system",
            Router::new()
                .route("/ping", post(ping))
                .route("/version", get(version))
                .route("/license", get(license)),
        )
        .nest(
            "/api",
            if config.enable_api_request_logging {
                api::route().layer(middleware::from_fn_with_state(
                    app_state.clone(),
                    api_request_logging::request_logging,
                ))
            } else {
                api::route()
            }
            .layer(SetResponseHeaderLayer::if_not_present(
                header::CACHE_CONTROL,
                HeaderValue::from_static(
                    "no-store, no-cache, max-age=0, must-revalidate, proxy-revalidate",
                ),
            )),
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

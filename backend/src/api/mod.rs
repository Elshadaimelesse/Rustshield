pub mod handlers;
use crate::AppState;
use axum::{routing::{get, post}, Router};
use tower_http::cors::{Any, CorsLayer};

pub fn create_router(state: AppState) -> Router {
    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);
    Router::new()
        .route("/api/health", get(handlers::health_check))
        .route("/api/incidents", get(handlers::list_incidents))
        .route("/api/incidents/:id/triage", post(handlers::triage_incident))
        .route("/api/logs", get(handlers::get_recent_logs))
        .route("/api/logs/ingest/auth", post(handlers::ingest_auth_log))
        .route("/api/logs/ingest/netflow", post(handlers::ingest_netflow_log))
        .route("/api/rules", get(handlers::list_detection_rules))
        .route("/api/rules/toggle", post(handlers::toggle_rule))
        .route("/api/ioc", get(handlers::list_iocs))
        .route("/api/ioc", post(handlers::add_ioc))
        .route("/api/network/telemetry", get(handlers::get_network_telemetry))
        .route("/api/reports/executive", get(handlers::get_executive_report))
        .layer(cors)
        .with_state(state)
}

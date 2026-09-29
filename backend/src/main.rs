mod api;
mod auth;
mod detection;
mod incidents;
mod ioc;
mod logs;
mod network;
mod reports;

use std::net::SocketAddr;
use std::sync::Arc;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(Clone)]
pub struct AppState {
    pub auth_service: Arc<auth::AuthService>,
    pub detection_engine: Arc<detection::DetectionEngine>,
    pub incident_manager: Arc<incidents::IncidentManager>,
    pub log_ingestor: Arc<logs::LogIngestionService>,
    pub ioc_db: Arc<ioc::IocDatabase>,
    pub network_monitor: Arc<network::NetworkMonitor>,
    pub report_generator: Arc<reports::ReportGenerator>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to initialize tracing subscriber");

    info!("Starting RustShield Core SIEM Engine v0.1.0...");

    let auth_service = Arc::new(auth::AuthService::new("rustshield-secret-key-change-in-prod"));
    let incident_manager = Arc::new(incidents::IncidentManager::new());
    let ioc_db = Arc::new(ioc::IocDatabase::new());
    let detection_engine = Arc::new(detection::DetectionEngine::new(
        incident_manager.clone(),
        ioc_db.clone(),
    ));
    let network_monitor = Arc::new(network::NetworkMonitor::new(incident_manager.clone()));
    let log_ingestor = Arc::new(logs::LogIngestionService::new(detection_engine.clone()));
    let report_generator = Arc::new(reports::ReportGenerator::new(
        incident_manager.clone(),
        ioc_db.clone(),
    ));

    ioc_db.load_default_feeds().await;
    detection_engine.load_rules_dir("./rules").await?;

    let state = AppState {
        auth_service,
        detection_engine,
        incident_manager,
        log_ingestor,
        ioc_db,
        network_monitor,
        report_generator,
    };

    let app = api::create_router(state);
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("RustShield SOC API listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

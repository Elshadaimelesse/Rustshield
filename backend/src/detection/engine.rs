use super::rules::DetectionRule;
use crate::incidents::{IncidentManager, IncidentSeverity, MitreAttackMapping};
use crate::ioc::IocDatabase;
use crate::logs::ParsedLog;
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use parking_lot::RwLock;
use std::sync::Arc;

pub struct DetectionEngine {
    rules: RwLock<Vec<DetectionRule>>,
    correlation_window: DashMap<String, Vec<EventBucketEntry>>,
    incident_manager: Arc<IncidentManager>,
    ioc_db: Arc<IocDatabase>,
}

#[derive(Debug, Clone)]
struct EventBucketEntry {
    timestamp: DateTime<Utc>,
    raw_message: String,
    metadata: std::collections::HashMap<String, String>,
}

impl DetectionEngine {
    pub fn new(incident_manager: Arc<IncidentManager>, ioc_db: Arc<IocDatabase>) -> Self {
        Self {
            rules: RwLock::new(Vec::new()),
            correlation_window: DashMap::new(),
            incident_manager,
            ioc_db,
        }
    }
}

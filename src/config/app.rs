use std::sync::Arc;
use std::time::Duration;

use dotenvy::dotenv;
use scylla::{CachingSession, Session, SessionBuilder};

use crate::config::config::Config;
use crate::repositories::user_repository::UserRepository;
use crate::services::metrics::{get_scylladb_user_metrics_service, ScyllaDBUserMetrics};

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub database: Arc<CachingSession>,
    pub user_repo: Arc<UserRepository<ScyllaDBUserMetrics>>,
    pub metrics: Arc<ScyllaDBUserMetrics>,
}

impl AppState {
    pub async fn new() -> Self {
        dotenv().expect(".env file not found");

        let config = Config::new();
        let session: Session = SessionBuilder::new()
            .known_nodes(config.database.nodes)
            .connection_timeout(Duration::from_secs(5))
            .user(config.database.username, config.database.password)
            .build()
            .await
            .expect("Connection Refused. Check your credentials and your IP linked on the ScyllaDB Cloud.");

        session.use_keyspace(config.database.keyspace, false).await.expect("Keyspace not found");
        let session = Arc::new(CachingSession::from(session, config.database.cached_queries));
        let metrics = Arc::new(get_scylladb_user_metrics_service(Arc::clone(&session)));

        Self {
            config: Config::new(),
            database: Arc::clone(&session),
            metrics: Arc::clone(&metrics),
            user_repo: Arc::new(UserRepository::new(Arc::clone(&session), Arc::clone(&metrics))),
        }
    }
}
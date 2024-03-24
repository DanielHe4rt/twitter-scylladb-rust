use std::sync::Arc;

use scylla::CachingSession;

use crate::models::timeline::Timeline;

pub struct TimelineRepository {
    db: Arc<CachingSession>,
}

pub trait TimelineRepositoryTrait {
    fn get_timeline(&self, username: &str) -> impl std::future::Future<Output = Result<Vec<Timeline>, crate::Error>> + Send;
}

impl TimelineRepository {
    pub fn new(db: Arc<CachingSession>) -> Self {
        Self { db }
    }
}

impl TimelineRepositoryTrait for TimelineRepository {
    async fn get_timeline(&self, username: &str) -> Result<Vec<Timeline>, crate::Error> {

        let results = Timeline::find_by_username(username.to_owned()).execute(&self.db).await?;
        Ok(results.try_collect().await?)
    }
}

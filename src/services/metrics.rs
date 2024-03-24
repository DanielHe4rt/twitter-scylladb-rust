use std::sync::Arc;
use std::sync::atomic::AtomicUsize;

use charybdis::operations::Update;
use scylla::CachingSession;

use crate::Error;
use crate::models::user_metrics::UserMetrics;

pub enum MetricsType {
    Tweets,
    Followers,
    Followings,
}

pub trait UserMetricsService {
    async fn increment(&self, metric_type: MetricsType, username: impl Into<String>) -> Result<(), Error>;
    async fn decrement(&self, metric_type: MetricsType, username: impl Into<String>) -> Result<(), Error>;
}

struct Metrics {
    followings: AtomicUsize,
}

pub struct ScyllaDBUserMetrics {
    db: Arc<CachingSession>,
    // counts: Mutex<HashMap<String, Metrics>>,
    // count: std::sync::atomic::AtomicUsize,
}

pub fn get_scylladb_user_metrics_service(db: Arc<CachingSession>) -> ScyllaDBUserMetrics {
   ScyllaDBUserMetrics { db }
}

impl UserMetricsService for ScyllaDBUserMetrics {
    // async fn increment_followers(&self, username: impl Into<String>) -> Result<(), Error> {
    //     let model = UserMetrics { username: username.into(), ..Default::default() };
    //
    //     if self.count.fetch_add(1, Ordering::Relaxed) % 1000 == 0 {
    //         model.increment_followers_count(1).execute(&self.db).await?;
    //     } else {
    //         let guard = self.counts.lock().unwrap();
    //         guard.get(&model.username).unwrap().followings.fetch_add(1, Ordering::Relaxed);
    //     }
    //
    //     Ok(())
    // }

    async fn increment(&self, metrics_type: MetricsType, username: impl Into<String>) -> Result<(), Error> {
        let model = UserMetrics { username: username.into(), ..Default::default() };
        match metrics_type {
            MetricsType::Tweets => model.increment_tweets_count(1).execute(&self.db).await?,
            MetricsType::Followers => model.increment_followers_count(1).execute(&self.db).await?,
            MetricsType::Followings => model.increment_following_count(1).execute(&self.db).await?,
        };

        Ok(())
    }

    async fn decrement(&self, metrics_type: MetricsType, username: impl Into<String>) -> Result<(), Error> {
        let model = UserMetrics { username: username.into(), ..Default::default() };


        match metrics_type {
            MetricsType::Tweets => model.decrement_tweets_count(1).execute(&self.db).await?,
            MetricsType::Followers => model.decrement_followers_count(1).execute(&self.db).await?,
            MetricsType::Followings => model.decrement_following_count(1).execute(&self.db).await?,
        };

        Ok(())
    }
}

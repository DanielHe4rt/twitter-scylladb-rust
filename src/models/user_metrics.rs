use charybdis::macros::charybdis_model;
use charybdis::types::{Counter, Text};

#[charybdis_model(
table_name = user_metrics,
partition_keys = [username],
clustering_keys = [],
global_secondary_indexes = [],
local_secondary_indexes = [],
)]
pub struct UserMetrics {
    pub username: Text,
    pub followers_count: Counter,
    pub following_count: Counter,
    pub tweets_count: Counter,
}
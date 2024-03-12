use charybdis::macros::charybdis_model;
use charybdis::types::{Text, Timestamp, Timeuuid, Uuid};
use serde::{Deserialize, Serialize};

#[charybdis_model(
table_name = tweets,
partition_keys = [tweet_id],
clustering_keys = [time],
table_options = "CLUSTERING ORDER BY (time DESC)"
)]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Tweet {
    pub tweet_id: Uuid,
    pub author: Text,
    pub text: Text,
    #[serde(skip_serializing, skip_deserializing)]
    pub time: Option<Timeuuid>,
    pub created_at: Timestamp
}

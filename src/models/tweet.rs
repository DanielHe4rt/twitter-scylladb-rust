use charybdis::macros::charybdis_model;
use charybdis::types::{Text, Timeuuid, Uuid};

#[charybdis_model(
table_name = tweets,
partition_keys = [tweet_id],
clustering_keys = [created_at],
table_options = "CLUSTERING ORDER BY (created_at DESC)"
)]
pub struct Tweet {
    pub tweet_id: Uuid,
    pub author: Text,
    pub text: Text,
    pub created_at: Timeuuid
}

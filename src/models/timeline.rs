use charybdis::macros::charybdis_model;
use charybdis::types::{Boolean, Text, Timeuuid, Uuid};

#[charybdis_model(
table_name = timeline,
partition_keys = [username],
clustering_keys = [created_at],
table_options = "CLUSTERING ORDER BY (created_at DESC)"
)]
#[derive(Debug, Default)]

pub struct Timeline {
    pub username: Text,
    pub tweet_id: Uuid,
    pub author: Text,
    pub text: Text,
    pub liked: Boolean,
    pub bookmarked: Boolean,
    pub retweeted: Boolean,
    pub created_at: Timeuuid
}

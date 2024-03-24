use charybdis::macros::charybdis_model;
use charybdis::types::{Text, Timestamp};
use serde::{Deserialize, Serialize};

#[charybdis_model(
table_name = users_followers,
partition_keys = [username],
clustering_keys = [followed_by],
global_secondary_indexes = [],
local_secondary_indexes = [],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserFollow {
    pub username: Text,
    pub followed_by: Text,
    pub followed_at: Timestamp,
}
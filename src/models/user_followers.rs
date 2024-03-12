use charybdis::macros::charybdis_model;
use charybdis::types::{Text, Timestamp};

#[charybdis_model(
table_name = users_followers,
partition_keys = [username],
clustering_keys = [followed_by],
global_secondary_indexes = [],
local_secondary_indexes = [],
)]
pub struct UserFollow {
    pub username: Text,
    pub followed_by: Text,
    pub followed_at: Timestamp,
}
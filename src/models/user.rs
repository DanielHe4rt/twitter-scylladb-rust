use charybdis::macros::charybdis_model;
use charybdis::types::{Date, Text, Timestamp};
use serde::{Deserialize, Serialize};

#[charybdis_model(
table_name = users,
partition_keys = [username],
clustering_keys = [],
global_secondary_indexes = [],
local_secondary_indexes = [],
)]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct User {
    pub username: Text,
    pub biography: Text,
    pub birthdate: Date,
    pub created_at: Timestamp
}
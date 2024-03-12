use std::sync::Arc;

use charybdis::errors::CharybdisError;
use scylla::{CachingSession, IntoTypedRows};

use crate::http::HttpError;
use crate::models::user::User;

pub struct UserRepository {
    db: Arc<CachingSession>,
}

impl UserRepository {
    pub fn new(db: Arc<CachingSession>) -> Self {
        UserRepository { db }
    }

    pub async fn get_user(&self, username: String) -> Result<User, HttpError> {
        let query = format!("SELECT * FROM users WHERE username = '{}' LIMIT 1", username);
        println!("{}", query);

        let rows = self.db.execute(query, &[]).await.unwrap().rows.unwrap();

        if rows.len() == 0 {
            return Err(HttpError::CharybdisError(CharybdisError::NotFoundError("User not found".to_string())));
        }

        let user = rows.into_typed::<User>().next().unwrap().unwrap();
        Ok(user)
    }
}
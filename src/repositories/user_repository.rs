use std::sync::Arc;

use charybdis::errors::CharybdisError;
use charybdis::operations::Insert;
use charybdis::types::Date;
use scylla::{CachingSession, IntoTypedRows};

use crate::dtos::user_dto::UserDTO;
use crate::models::user::User;
use crate::models::user_followers::UserFollow;

pub struct UserRepository {
    db: Arc<CachingSession>,
}

impl UserRepository
{
    pub fn new(db: Arc<CachingSession>) -> Self {
        Self { db }
    }


    pub async fn create(&self, dto: UserDTO) -> Result<User, crate::Error> {
        let user = User {
            username: dto.username.to_string(),
            biography: dto.biography.into(),
            created_at: chrono::Utc::now(),
            birthdate: Date::from(dto.birthdate),
        };

        user.insert().execute(&self.db).await?;

        Ok(user)
    }
    pub async fn get_user(&self, username: String) -> Result<User, crate::Error>
    {
        let query = format!("SELECT * FROM users WHERE username = '{}' LIMIT 1", username);

        let rows = self.db.execute(query, &[]).await.unwrap().rows.unwrap();

        if rows.len() == 0 {
            return Err(crate::Error::CharybdisError(CharybdisError::NotFoundError("User not found".to_string())));
        }

        let user = rows.into_typed::<User>().next().unwrap().unwrap();
        Ok(user)
    }

    pub async fn get_followers(&self, username: String) -> Result<Vec<UserFollow>, crate::Error> {
        let query = format!("SELECT * FROM users_followers WHERE username = '{}'", username);

        let rows = self.db.execute(query, &[]).await.unwrap().rows.unwrap();

        let followers = rows.into_typed::<UserFollow>().collect::<Result<Vec<UserFollow>, _>>().unwrap();
        Ok(followers)
    }
}
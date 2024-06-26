use std::sync::Arc;

use charybdis::errors::CharybdisError;
use charybdis::operations::Insert;
use charybdis::types::{Timestamp, Uuid};
use scylla::{CachingSession, IntoTypedRows};

use crate::http::controllers::post_tweet::TweetRequestDTO;
use crate::models::tweet::Tweet;

pub struct TweetRepository {
    db: Arc<CachingSession>,
}

impl TweetRepository {
    pub fn new(db: Arc<CachingSession>) -> Self {
        TweetRepository { db }
    }

    pub async fn find_tweet(&self, tweet_id: Uuid) -> Result<Tweet, crate::Error> {
        let query = format!("SELECT tweet_id, author, text, created_at FROM tweets WHERE tweet_id = {} LIMIT 1", tweet_id);
        println!("{}", query);

        let rows = self.db.execute(query, &[]).await.unwrap().rows.unwrap();
        println!("{:?}", rows);

        if rows.len() == 0 {
            return Err(crate::Error::CharybdisError(CharybdisError::NotFoundError("Tweet not found".to_string())));
        }

        let row = rows.into_typed::<(Uuid, String, String, Timestamp)>().next().unwrap().unwrap();
        let tweet = Tweet {
            tweet_id,
            author: row.1,
            text: row.2,
            time: None,
            created_at: row.3,
        };

        Ok(tweet)
    }

    pub async fn tweet(&self, payload: TweetRequestDTO) -> Result<Tweet, crate::Error> {

        let tweet = Tweet {
            tweet_id: Uuid::new_v4(),
            author: payload.username.clone(),
            text: payload.content.clone(),
            created_at: chrono::Utc::now(),
            ..Default::default()
        };

        tweet.insert().execute(&self.db).await?;

        Ok(tweet)
    }
}
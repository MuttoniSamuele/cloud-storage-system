use crate::{errors::SessionError, models::RedisPool};
use bb8_redis::redis::AsyncCommands;
use rand_chacha::ChaCha8Rng;
use rand_core::RngCore;

pub struct SessionsModel<'r> {
    pool: RedisPool,
    rng: &'r mut ChaCha8Rng,
}

impl<'r> SessionsModel<'r> {
    pub fn new(pool: RedisPool, rng: &'r mut ChaCha8Rng) -> Self {
        SessionsModel { pool, rng }
    }

    pub async fn new_session(&mut self, user_id: i32) -> Result<u128, SessionError> {
        // Generate a new session id
        let mut bytes = [0u8; 16];
        self.rng.fill_bytes(&mut bytes);
        let session_id = u128::from_le_bytes(bytes);
        // Connect to the Redis database
        let mut conn = self.pool.get().await.map_err(|_| SessionError)?;
        // Save the session
        let _: () = conn
            .set_ex(&session_id.to_string(), user_id, 3600) // TODO: Read expiry from .env
            .await
            .map_err(|_| SessionError)?;
        Ok(session_id)
    }
}

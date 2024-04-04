use crate::{errors::InternalError, models::RedisPool, SESSION_TTL};
use bb8_redis::redis::AsyncCommands;
use rand_chacha::ChaCha8Rng;
use rand_core::RngCore;

pub async fn new_session(
    redis_pool: &RedisPool,
    rng: &mut ChaCha8Rng,
    user_id: i32,
) -> Result<u128, InternalError> {
    // Generate a new session id
    let mut bytes = [0u8; 16];
    rng.fill_bytes(&mut bytes);
    let session_id = u128::from_le_bytes(bytes);
    // Connect to the Redis database
    let mut conn = redis_pool
        .get()
        .await
        .map_err(|_| InternalError("Session error".to_string()))?;
    // Save the session
    let _: () = conn
        .set_ex(&session_id.to_string(), user_id, *SESSION_TTL)
        .await
        .map_err(|_| InternalError("Session error".to_string()))?;
    Ok(session_id)
}

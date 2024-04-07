use crate::{errors::InternalError, models::RedisPool, SESSION_TTL};
use bb8_redis::redis::{AsyncCommands, Expiry};
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

pub async fn get_session_user_id(
    redis_pool: &RedisPool,
    session_id: u128,
) -> Result<Option<i32>, InternalError> {
    // Connect to the Redis database
    let mut conn = redis_pool
        .get()
        .await
        .map_err(|_| InternalError("Session error".to_string()))?;
    let session_id_str = &session_id.to_string();
    // Check if the session id exists in the db
    let session_exists: bool = conn
        .exists(session_id_str)
        .await
        .map_err(|_| InternalError("Error while validating session".to_string()))?;
    // If it doesn't then there is no user id
    if !session_exists {
        return Ok(None);
    }
    // Otherwise get the user id
    let user_id: i32 = conn
        // TODO: I'm not sure if Expiry::EX are actually seconds lol
        .get_ex(session_id_str, Expiry::EX(*SESSION_TTL as usize))
        .await
        .map_err(|_| InternalError("Error while reading session".to_string()))?;
    Ok(Some(user_id))
}

pub async fn delete_session(redis_pool: &RedisPool, session_id: u128) -> Result<(), InternalError> {
    // Connect to the Redis database
    let mut conn = redis_pool
        .get()
        .await
        .map_err(|_| InternalError("Session error".to_string()))?;
    // Delete the session
    conn.del(session_id.to_string())
        .await
        .map_err(|_| InternalError("Error while deleting session".to_string()))?;
    Ok(())
}

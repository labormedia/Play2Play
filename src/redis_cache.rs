pub use redis::{
    AsyncCommands,
    Commands,
};
use crate::event::Event;

pub async fn redis_connect() -> redis::aio::MultiplexedConnection {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    client.get_multiplexed_tokio_connection().await.unwrap()
}

pub async fn cache_event(redis_conn: &mut redis::aio::MultiplexedConnection, event: &Event) -> redis::RedisResult<()> {
    let event_json = serde_json::to_string(event).unwrap();
    redis_conn.set::<_, _, ()>(format!("event:{}", event.agent_id), event_json).await
}

pub async fn retrieve_event(redis_conn: &mut redis::aio::MultiplexedConnection, agent_id: &str) -> redis::RedisResult<Option<Event>> {
    let event_json: Option<String> = redis_conn.get(format!("event:{}", agent_id)).await?;
    match event_json {
        Some(json) => Ok(Some(serde_json::from_str(&json).unwrap())),
        None => Ok(None),
    }
}
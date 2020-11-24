use crate::types::*;
use redis::aio::ConnectionLike;
use redis::{cmd, RedisFuture, ToRedisArgs};

pub trait AsyncGraphCommands: ConnectionLike + Send + Sized {
    fn graph_query<'a, K: ToRedisArgs + Send + Sync + 'a, Q: ToRedisArgs + Send + Sync + 'a>(
        &'a mut self,
        key: K,
        query: Q,
    ) -> RedisFuture<GraphResultSet> {
        Box::pin(async move {
            cmd("GRAPH.QUERY")
                .arg(key)
                .arg(query)
                .query_async(self)
                .await
        })
    }
}

impl<T> AsyncGraphCommands for T where T: Send + ConnectionLike {}

use crate::types::*;
use redis::{cmd, ConnectionLike, RedisResult, ToRedisArgs};

pub trait GraphCommands: ConnectionLike + Sized {
    fn graph_query<K: ToRedisArgs, Q: ToRedisArgs>(
        &mut self,
        key: K,
        query: Q,
    ) -> RedisResult<GraphResultSet> {
        cmd("GRAPH.QUERY").arg(key).arg(query).query(self)
    }
}

impl<T> GraphCommands for T where T: ConnectionLike {}

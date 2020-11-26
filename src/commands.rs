use crate::types::*;
use redis::{cmd, ConnectionLike, RedisResult, ToRedisArgs};

/// Provides a high level synchronous API to work with Redis graph data types.
/// The graph command becomes directly available on ConnectionLike types from 
/// the redis crate when you import the GraphCommands trait.
/// ```rust,no_run
/// # fn run() -> redis::RedisResult<()> {
/// use redis::Commands;
/// use redis_graph::{GraphCommands, GraphResultSet};
///
/// let client = redis::Client::open("redis://127.0.0.1/")?;
/// let mut con = client.get_connection()?;
///
/// let res:GraphResultSet = con.graph_query(
///     "my_graph", 
///     "CREATE (:Rider {name:'Valentino Rossi'})-[:rides]->(:Team {name:'Yamaha'})"
/// )?;
/// # Ok(()) }
/// ```
///
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

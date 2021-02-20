use crate::types::*;
use redis::{cmd, ConnectionLike, FromRedisValue, RedisResult, ToRedisArgs};

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
///
/// let res_read_only:GraphResultSet = con.graph_ro_query(
///     "my_graph",
///     "MATCH (rider:Rider)-[:rides]->(:Team {name:'Yamaha'}) RETURN rider"
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

    fn graph_ro_query<K: ToRedisArgs, Q: ToRedisArgs>(
        &mut self,
        key: K,
        query: Q,
    ) -> RedisResult<GraphResultSet> {
        cmd("GRAPH.RO_QUERY").arg(key).arg(query).query(self)
    }

    fn graph_profile<K: ToRedisArgs, Q: ToRedisArgs, RV: FromRedisValue>(
        &mut self,
        key: K,
        query: Q,
    ) -> RedisResult<RV> {
        cmd("GRAPH.PROFILE").arg(key).arg(query).query(self)
    }

    fn graph_delete<K: ToRedisArgs>(&mut self, key: K) -> RedisResult<String> {
        cmd("GRAPH.DELETE").arg(key).query(self)
    }

    fn graph_explain<K: ToRedisArgs, Q: ToRedisArgs, RV: FromRedisValue>(
        &mut self,
        key: K,
        query: Q,
    ) -> RedisResult<RV> {
        cmd("GRAPH.EXPLAIN").arg(key).arg(query).query(self)
    }

    fn graph_slowlog<K: ToRedisArgs>(&mut self, key: K) -> RedisResult<Vec<SlowLogEntry>> {
        cmd("GRAPH.SLOWLOG").arg(key).query(self)
    }

    fn graph_config_set<K: ToRedisArgs, V: ToRedisArgs>(
        &mut self,
        name: K,
        value: V,
    ) -> RedisResult<bool> {
        cmd("GRAPH.CONFIG")
            .arg("SET")
            .arg(name)
            .arg(value)
            .query(self)
    }

    fn graph_config_get<K: ToRedisArgs, RV: FromRedisValue>(&mut self, name: K) -> RedisResult<RV> {
        value_from_pair(&cmd("GRAPH.CONFIG").arg("GET").arg(name).query(self)?)
    }

    fn graph_config_get_all(&mut self) -> RedisResult<GraphConfig> {
        cmd("GRAPH.CONFIG").arg("GET").arg("*").query(self)
    }
}

impl<T> GraphCommands for T where T: ConnectionLike {}

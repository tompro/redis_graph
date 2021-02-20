use crate::types::*;
use redis::aio::ConnectionLike;
use redis::{cmd, FromRedisValue, RedisFuture, ToRedisArgs};

/// Provides a high level asynchronous API to work with Redis graph data types.
/// The graph command becomes directly available on ConnectionLike types from
/// the redis crate when you import the GraphCommands trait.
/// ```rust,no_run
/// # async fn run() -> redis::RedisResult<()> {
/// use redis::AsyncCommands;
/// use redis_graph::{AsyncGraphCommands, GraphResultSet};
///
/// let client = redis::Client::open("redis://127.0.0.1/")?;
/// let mut con = client.get_async_connection().await?;
///
/// let res:GraphResultSet = con.graph_query(
///     "my_graph",
///     "CREATE (:Rider {name:'Valentino Rossi'})-[:rides]->(:Team {name:'Yamaha'})"
/// ).await?;
///
/// let res_read_only:GraphResultSet = con.graph_ro_query(
///     "my_graph",
///     "MATCH (rider:Rider)-[:rides]->(:Team {name:'Yamaha'}) RETURN rider"
/// ).await?;
/// # Ok(()) }
/// ```
///
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

    fn graph_ro_query<'a, K: ToRedisArgs + Send + Sync + 'a, Q: ToRedisArgs + Send + Sync + 'a>(
        &'a mut self,
        key: K,
        query: Q,
    ) -> RedisFuture<GraphResultSet> {
        Box::pin(async move {
            cmd("GRAPH.RO_QUERY")
                .arg(key)
                .arg(query)
                .query_async(self)
                .await
        })
    }

    fn graph_profile<
        'a,
        K: ToRedisArgs + Send + Sync + 'a,
        Q: ToRedisArgs + Send + Sync + 'a,
        RV: FromRedisValue,
    >(
        &'a mut self,
        key: K,
        query: Q,
    ) -> RedisFuture<RV> {
        Box::pin(async move {
            cmd("GRAPH.PROFILE")
                .arg(key)
                .arg(query)
                .query_async(self)
                .await
        })
    }

    fn graph_delete<'a, K: ToRedisArgs + Send + Sync + 'a>(
        &'a mut self,
        key: K,
    ) -> RedisFuture<String> {
        Box::pin(async move { cmd("GRAPH.DELETE").arg(key).query_async(self).await })
    }

    fn graph_explain<
        'a,
        K: ToRedisArgs + Send + Sync + 'a,
        Q: ToRedisArgs + Send + Sync + 'a,
        RV: FromRedisValue,
    >(
        &'a mut self,
        key: K,
        query: Q,
    ) -> RedisFuture<RV> {
        Box::pin(async move {
            cmd("GRAPH.EXPLAIN")
                .arg(key)
                .arg(query)
                .query_async(self)
                .await
        })
    }

    fn graph_slowlog<'a, K: ToRedisArgs + Send + Sync + 'a>(
        &'a mut self,
        key: K,
    ) -> RedisFuture<Vec<SlowLogEntry>> {
        Box::pin(async move { cmd("GRAPH.SLOWLOG").arg(key).query_async(self).await })
    }

    fn graph_config_set<
        'a,
        K: ToRedisArgs + Send + Sync + 'a,
        V: ToRedisArgs + Send + Sync + 'a,
    >(
        &'a mut self,
        name: K,
        value: V,
    ) -> RedisFuture<bool> {
        Box::pin(async move {
            cmd("GRAPH.CONFIG")
                .arg("SET")
                .arg(name)
                .arg(value)
                .query_async(self)
                .await
        })
    }

    fn graph_config_get<'a, K: ToRedisArgs + Send + Sync + 'a, RV: FromRedisValue>(
        &'a mut self,
        name: K,
    ) -> RedisFuture<RV> {
        Box::pin(async move {
            value_from_pair(
                &cmd("GRAPH.CONFIG")
                    .arg("GET")
                    .arg(name)
                    .query_async(self)
                    .await?,
            )
        })
    }

    fn graph_config_get_all<'a>(&'a mut self) -> RedisFuture<GraphConfig> {
        Box::pin(async move {
            cmd("GRAPH.CONFIG")
                .arg("GET")
                .arg("*")
                .query_async(self)
                .await
        })
    }
}

impl<T> AsyncGraphCommands for T where T: Send + ConnectionLike {}

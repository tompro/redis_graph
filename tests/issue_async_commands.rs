extern crate redis;
extern crate redis_graph;

#[cfg(any(feature = "tokio-comp", feature = "async-std-comp"))]
use redis::aio::Connection;
#[cfg(any(feature = "tokio-comp", feature = "async-std-comp"))]
use redis::{AsyncCommands, RedisResult};
#[cfg(any(feature = "tokio-comp", feature = "async-std-comp"))]
use redis_graph::*;
#[cfg(any(feature = "tokio-comp", feature = "async-std-comp"))]
use std::env;

#[cfg(any(feature = "tokio-comp", feature = "async-std-comp"))]
async fn get_con() -> Connection {
    let client = redis::Client::open(get_redis_url()).unwrap();
    client.get_async_connection().await.unwrap()
}

#[cfg(any(feature = "tokio-comp", feature = "async-std-comp"))]
async fn ensure_simple_data(name: &str) {
    let mut con = get_con().await;
    let _: () = con.del(name).await.unwrap();
    let _ = con.graph_query(
        name,
        "CREATE (:person {name:'Pam', age:27})-[:works {since: 2010}]->(:employer {name:'Dunder Mifflin'})"
    ).await;
}

#[cfg(any(feature = "tokio-comp", feature = "async-std-comp"))]
async fn ensure_test_data(name: &str) {
    let mut con = get_con().await;
    let _: () = con.del(name).await.unwrap();
    let _ = con.graph_query(name,
        "CREATE (:Rider {name:'Valentino Rossi', born:1979})-[:rides]->(:Team {name:'Yamaha'}), \
        (:Rider {name:'Dani Pedrosa', born:1985})-[:rides]->(:Team {name:'Honda'}), \
        (:Rider {name:'Andrea Dovizioso'})-[:rides]->(:Team {name:'Ducati'})"
    ).await;
}

#[cfg(any(feature = "tokio-comp", feature = "async-std-comp"))]
pub async fn issue_graph_create_command(name: &str) -> GraphResultSet {
    let mut con = get_con().await;
    let _: () = con.del(name).await.unwrap();
    con.graph_query(
        name,
        "CREATE (:person {name:'Pam', age:27})-[:works {since: 2010}]->(:employer {name:'Dunder Mifflin'})"
    ).await.unwrap()
}

#[cfg(any(feature = "tokio-comp", feature = "async-std-comp"))]
pub async fn issue_match_query_command(name: &str) -> GraphResultSet {
    ensure_simple_data(name).await;
    get_con()
        .await
        .graph_query(name, "MATCH (n1)-[r]->(n2) RETURN n1, r, n2.name")
        .await
        .unwrap()
}

#[cfg(any(feature = "tokio-comp", feature = "async-std-comp"))]
pub async fn issue_match_ro_query_command(name: &str) -> GraphResultSet {
    ensure_simple_data(name).await;
    get_con()
        .await
        .graph_ro_query(name, "MATCH (n1)-[r]->(n2) RETURN n1, r, n2.name")
        .await
        .unwrap()
}

#[cfg(any(feature = "tokio-comp", feature = "async-std-comp"))]
pub async fn issue_match_scalar_result(name: &str) -> GraphResultSet {
    ensure_test_data(name).await;
    get_con()
        .await
        .graph_query(
            name,
            "MATCH (r:Rider)-[:rides]->(t:Team) WHERE t.name = 'Yamaha' RETURN r.name, t.name",
        )
        .await
        .unwrap()
}

#[cfg(any(feature = "tokio-comp", feature = "async-std-comp"))]
pub async fn issue_query_all_nodes(name: &str) -> GraphResultSet {
    ensure_test_data(name).await;
    get_con()
        .await
        .graph_query(name, "MATCH (r:Rider) RETURN r")
        .await
        .unwrap()
}

#[cfg(any(feature = "tokio-comp", feature = "async-std-comp"))]
pub async fn issue_query_option(name: &str) -> GraphResultSet {
    ensure_test_data(name).await;
    get_con()
        .await
        .graph_query(name, "MATCH (r:Rider) RETURN r.born")
        .await
        .unwrap()
}

#[cfg(any(feature = "tokio-comp", feature = "async-std-comp"))]
pub async fn issue_graph_profile_query(name: &str) -> Vec<String> {
    ensure_test_data(name).await;
    get_con()
        .await
        .graph_profile(name, "MATCH (r:Rider) RETURN r")
        .await
        .unwrap()
}

#[cfg(any(feature = "tokio-comp", feature = "async-std-comp"))]
pub async fn issue_graph_slowlog_query(name: &str) -> Vec<SlowLogEntry> {
    ensure_test_data(name).await;
    get_con().await.graph_slowlog(name).await.unwrap()
}

#[cfg(any(feature = "tokio-comp", feature = "async-std-comp"))]
pub async fn issue_graph_config_set() -> RedisResult<bool> {
    get_con()
        .await
        .graph_config_set("RESULTSET_SIZE", 500)
        .await
}

#[cfg(any(feature = "tokio-comp", feature = "async-std-comp"))]
pub async fn issue_graph_config_set_invalid() -> RedisResult<bool> {
    get_con().await.graph_config_set("SOME", 500).await
}

#[cfg(any(feature = "tokio-comp", feature = "async-std-comp"))]
pub async fn issue_graph_config_get() -> RedisResult<i32> {
    let _: bool = get_con()
        .await
        .graph_config_set("RESULTSET_SIZE", 500)
        .await
        .unwrap();
    get_con().await.graph_config_get("RESULTSET_SIZE").await
}

#[cfg(any(feature = "tokio-comp", feature = "async-std-comp"))]
pub async fn issue_graph_config_get_all() -> GraphConfig {
    let _: bool = get_con()
        .await
        .graph_config_set("RESULTSET_SIZE", 500)
        .await
        .unwrap();
    get_con().await.graph_config_get_all().await.unwrap()
}

#[cfg(any(feature = "tokio-comp", feature = "async-std-comp"))]
pub async fn issue_graph_delete(name: &str) -> RedisResult<String> {
    ensure_test_data(name).await;
    get_con().await.graph_delete(name).await
}

#[cfg(any(feature = "tokio-comp", feature = "async-std-comp"))]
pub async fn issue_graph_explain(name: &str) -> RedisResult<Vec<String>> {
    ensure_test_data(name).await;
    get_con()
        .await
        .graph_explain(name, "MATCH (r:Rider) RETURN r")
        .await
}

#[cfg(any(feature = "tokio-comp", feature = "async-std-comp"))]
fn get_redis_url() -> String {
    let redis_host_key = "REDIS_HOST";
    let redis_host_port = "REDIS_PORT";

    let redis_host = match env::var(redis_host_key) {
        Ok(host) => host,
        _ => "localhost".to_string(),
    };

    let redis_port = match env::var(redis_host_port) {
        Ok(port) => port,
        _ => "6379".to_string(),
    };

    format!("redis://{}:{}/", redis_host, redis_port)
}

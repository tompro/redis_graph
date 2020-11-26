extern crate async_std;
extern crate redis;
extern crate redis_graph;

#[cfg(test)]
#[path = "./test_graph_assertions.rs"]
mod test_graph_assertions;

use async_std::task;
use redis::aio::Connection;
use redis::AsyncCommands;
use redis_graph::*;
use test_graph_assertions::*;

async fn get_con() -> Connection {
    let client = redis::Client::open("redis://localhost/").unwrap();
    client.get_async_connection().await.unwrap()
}

async fn ensure_simple_data(name: &str) {
    let mut con = get_con().await;
    let _: () = con.del(name).await.unwrap();
    let _ = con.graph_query(
        name,
        "CREATE (:person {name:'Pam', age:27})-[:works {since: 2010}]->(:employer {name:'Dunder Mifflin'})"
    ).await;
}

async fn ensure_test_data(name: &str) {
    let mut con = get_con().await;
    let _: () = con.del(name).await.unwrap();
    let _ = con.graph_query(name,
        "CREATE (:Rider {name:'Valentino Rossi', born:1979})-[:rides]->(:Team {name:'Yamaha'}), \
        (:Rider {name:'Dani Pedrosa', born:1985})-[:rides]->(:Team {name:'Honda'}), \
        (:Rider {name:'Andrea Dovizioso'})-[:rides]->(:Team {name:'Ducati'})"
    ).await;
}

#[test]
fn test_issue_graph_create_command() {
    let r = task::block_on(async {
        let mut con = get_con().await;
        let _: () = con.del("test_issue_graph_create_command").await.unwrap();
        con.graph_query(
            "test_issue_graph_create_command", 
            "CREATE (:person {name:'Pam', age:27})-[:works {since: 2010}]->(:employer {name:'Dunder Mifflin'})"
        ).await.unwrap()
    });

    check_graph_create_command(r);
}

#[test]
fn test_match_query_result() {
    let r = task::block_on(async {
        ensure_simple_data("test_match_query_result").await;
        get_con()
            .await
            .graph_query(
                "test_match_query_result",
                "MATCH (n1)-[r]->(n2) RETURN n1, r, n2.name",
            )
            .await
            .unwrap()
    });
    check_match_query_result(r);
}

#[test]
fn test_match_scalar_result() {
    let res =
        task::block_on(async {
            ensure_test_data("test_match_scalar_result").await;
            get_con().await.graph_query(
            "test_match_scalar_result", 
            "MATCH (r:Rider)-[:rides]->(t:Team) WHERE t.name = 'Yamaha' RETURN r.name, t.name"
        ).await.unwrap()
        });
    check_match_scalar_result(res);
}

#[test]
fn test_query_all_nodes() {
    let res = task::block_on(async {
        ensure_test_data("test_query_all_nodes").await;
        get_con()
            .await
            .graph_query("test_query_all_nodes", "MATCH (r:Rider) RETURN r")
            .await
            .unwrap()
    });
    check_query_all_nodes(res);
}

#[test]
fn test_unserialize_option() {
    let res = task::block_on(async {
        ensure_test_data("test_unserialize_option").await;
        get_con()
            .await
            .graph_query("test_unserialize_option", "MATCH (r:Rider) RETURN r.born")
            .await
            .unwrap()
    });
    check_unserialize_option(res);
}

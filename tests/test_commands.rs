extern crate redis;
extern crate redis_graph;

#[cfg(test)]
#[path = "./test_graph_assertions.rs"]
mod test_graph_assertions;

use redis::{Commands, Connection};
use redis_graph::*;
use std::env;
use test_graph_assertions::*;

fn get_con() -> Connection {
    let client = redis::Client::open(get_redis_url()).unwrap();
    client
        .get_connection()
        .expect("Failed to get redis connection!")
}

fn ensure_simple_data(name: &str) {
    let mut con = get_con();
    let _: () = con.del(name).unwrap();
    let _ = con.graph_query(
        name,
        "CREATE (:person {name:'Pam', age:27})-[:works {since: 2010}]->(:employer {name:'Dunder Mifflin'})"
    );
}

fn ensure_test_data(name: &str) {
    let mut con = get_con();
    let _: () = con.del(name).unwrap();
    let _ = con.graph_query(
        name,
        "CREATE (:Rider {name:'Valentino Rossi', born:1979})-[:rides]->(:Team {name:'Yamaha'}), \
        (:Rider {name:'Dani Pedrosa', born:1985})-[:rides]->(:Team {name:'Honda'}), \
        (:Rider {name:'Andrea Dovizioso'})-[:rides]->(:Team {name:'Ducati'})",
    );
}

#[test]
fn test_issue_graph_create_command() {
    let _: () = get_con().del("test_issue_graph_create_command").unwrap();
    let r = get_con().graph_query(
        "test_issue_graph_create_command", 
        "CREATE (:person {name:'Pam', age:27})-[:works {since: 2010}]->(:employer {name:'Dunder Mifflin'})"
    ).unwrap();
    check_graph_create_command(r);
}

#[test]
fn test_match_query_result() {
    ensure_simple_data("test_match_query_result");

    let r = get_con()
        .graph_query(
            "test_match_query_result",
            "MATCH (n1)-[r]->(n2) RETURN n1, r, n2.name",
        )
        .unwrap();

    check_match_query_result(r);
}

#[test]
fn test_match_ro_query_result() {
    ensure_simple_data("test_match_ro_query_result");

    let r = get_con()
        .graph_ro_query(
            "test_match_ro_query_result",
            "MATCH (n1)-[r]->(n2) RETURN n1, r, n2.name",
        )
        .unwrap();

    check_match_query_result(r);
}

#[test]
fn test_match_scalar_result() {
    ensure_test_data("test_match_scalar_result");
    let res = get_con()
        .graph_query(
            "test_match_scalar_result",
            "MATCH (r:Rider)-[:rides]->(t:Team) WHERE t.name = 'Yamaha' RETURN r.name, t.name",
        )
        .unwrap();
    check_match_scalar_result(res);
}

#[test]
fn test_query_all_nodes() {
    ensure_test_data("test_query_all_nodes");
    let res = get_con()
        .graph_query("test_query_all_nodes", "MATCH (r:Rider) RETURN r")
        .unwrap();
    check_query_all_nodes(res);
}

#[test]
fn test_unserialize_option() {
    ensure_test_data("test_unserialize_option");
    let res = get_con()
        .graph_query("test_unserialize_option", "MATCH (r:Rider) RETURN r.born")
        .unwrap();
    check_unserialize_option(res);
}

#[test]
fn test_graph_profile() {
    ensure_test_data("test_graph_profile");
    let res: Vec<String> = get_con()
        .graph_profile("test_graph_profile", "MATCH (r:Rider) RETURN r")
        .unwrap();
    check_graph_profile(res);
}

#[test]
fn test_graph_slowlog() {
    ensure_test_data("test_graph_slowlog");
    let res = get_con().graph_slowlog("test_graph_slowlog").unwrap();
    check_graph_slowlog(res);
}

#[test]
fn test_graph_config_set() {
    let err_res = get_con().graph_config_set("SOME", 1000);
    check_graph_config_set_invalid(err_res);
    let res = get_con().graph_config_set("RESULTSET_SIZE", 500);
    check_graph_config_set_valid(res);
}

#[test]
fn test_graph_config_get() {
    let _ = get_con().graph_config_set("RESULTSET_SIZE", 500).unwrap();
    check_graph_config_get(get_con().graph_config_get("RESULTSET_SIZE"));
}

#[test]
fn test_graph_config_get_all() {
    let _ = get_con().graph_config_set("RESULTSET_SIZE", 500).unwrap();
    check_graph_config_get_all(get_con().graph_config_get_all().unwrap());
}

#[test]
fn test_graph_delete() {
    ensure_test_data("test_graph_delete");
    check_graph_delete_success(get_con().graph_delete("test_graph_delete"));
}

#[test]
fn test_graph_explain() {
    ensure_test_data("test_graph_explain");
    check_graph_explain_result(
        get_con().graph_explain("test_graph_explain", "MATCH (r:Rider) RETURN r"),
    );
}

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

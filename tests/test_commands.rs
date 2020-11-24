extern crate redis;
extern crate redis_graph;

use redis::{Commands, Connection, from_redis_value};
use redis_graph::GraphCommands;
use redis_graph::GraphResult;

fn get_con() -> Connection {
    let client = redis::Client::open("redis://localhost/").unwrap();
    client.get_connection().expect("Failed to get redis connection!")
}

#[test]
fn test_issue_graph_command() {
    let _:() = get_con().del("test_issue_graph_command").unwrap();
    let r = get_con().graph_query(
        "test_issue_graph_command", 
        "CREATE (:person {name:'Pam', age:27})-[:works {since: 2010}]->(:employer {name:'Dunder Mifflin'})"
    ).unwrap();
    assert!(!r.metadata.is_empty());
}

#[test]
fn test_match_query_result() {
    let _:() = get_con().del("test_match_query_result").unwrap();
    let _ = get_con().graph_query(
        "test_match_query_result", 
        "CREATE (:person {name:'Pam', age:27})-[:works {since: 2010}]->(:employer {name:'Dunder Mifflin'})"
    ).unwrap();

    let r = get_con().graph_query(
        "test_match_query_result", 
        "MATCH (n1)-[r]->(n2) RETURN n1, r, n2.name"
    ).unwrap();
    
    assert_eq!(r.header, ["n1", "r", "n2.name"]);
    assert_eq!(r.data.len(), 1);
    let data = r.data.get(0).unwrap();
    assert_eq!(data.len(), 3);
    
    match data.get(0) {
        Some(GraphResult::Node(node)) => {
            assert_eq!(node.labels, ["person"]);
            let name:String = from_redis_value(node.properties.get("name").unwrap()).unwrap();
            assert_eq!(name, "Pam");
        },
        _ => assert!(false),
    }

    match data.get(1) {
        Some(GraphResult::Relation(rel)) => {
            assert_eq!(rel.rel_type, "works");
            let since:usize = from_redis_value(rel.properties.get("since").unwrap()).unwrap();
            assert_eq!(since, 2010);
        },
        _ => assert!(false),
    }

    match data.get(2) {
        Some(GraphResult::Scalar(s)) => {
            let v:String = from_redis_value(s).unwrap();
            assert_eq!(v, "Dunder Mifflin")
        },
        _ => assert!(false),
    }

    assert!(!r.metadata.is_empty());

    println!("{:?}", r);
}



extern crate redis;
extern crate redis_graph;

use redis::{Commands, Connection, from_redis_value};
use redis_graph::GraphCommands;
use redis_graph::GraphValue;

fn get_con() -> Connection {
    let client = redis::Client::open("redis://localhost/").unwrap();
    client.get_connection().expect("Failed to get redis connection!")
}

fn ensure_test_data(name:&str) {
    let mut con = get_con();
    let exists:bool = con.exists(name).unwrap();
    if !exists {
        let _ = con.graph_query(name, 
            "CREATE (:Rider {name:'Valentino Rossi'})-[:rides]->(:Team {name:'Yamaha'}), \
            (:Rider {name:'Dani Pedrosa'})-[:rides]->(:Team {name:'Honda'}), \
            (:Rider {name:'Andrea Dovizioso'})-[:rides]->(:Team {name:'Ducati'})"
        );
    }
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
    assert_eq!(data.data.len(), 3);
    
    match data.data.get("n1") {
        Some(GraphValue::Node(node)) => {
            assert_eq!(node.labels, ["person"]);
            let name:String = from_redis_value(node.properties.get("name").unwrap()).unwrap();
            assert_eq!(name, "Pam");
        },
        _ => assert!(false),
    }

    match data.data.get("r") {
        Some(GraphValue::Relation(rel)) => {
            assert_eq!(rel.rel_type, "works");
            let since:usize = from_redis_value(rel.properties.get("since").unwrap()).unwrap();
            assert_eq!(since, 2010);
        },
        _ => assert!(false),
    }

    match data.data.get("n2.name") {
        Some(GraphValue::Scalar(s)) => {
            let v:String = from_redis_value(s).unwrap();
            assert_eq!(v, "Dunder Mifflin")
        },
        _ => assert!(false),
    }

    assert!(!r.metadata.is_empty());

}

#[test]
fn test_match_scalar_result() {
    ensure_test_data("test_match_scalar_result");
    let res = get_con().graph_query(
        "test_match_scalar_result", 
        "MATCH (r:Rider)-[:rides]->(t:Team) WHERE t.name = 'Yamaha' RETURN r.name, t.name"
    ).unwrap();

    assert_eq!(res.data.len(), 1);
    assert_eq!(res.data.get(0).unwrap().data.len(), 2);
}



extern crate redis;
extern crate redis_graph;

use redis::{Commands, Connection, from_redis_value};
use redis_graph::*;

fn get_con() -> Connection {
    let client = redis::Client::open("redis://localhost/").unwrap();
    client.get_connection().expect("Failed to get redis connection!")
}

fn ensure_simple_data(name:&str) {
    let mut con = get_con();
    let _:() = con.del(name).unwrap();
    let _ = con.graph_query(
        name, 
        "CREATE (:person {name:'Pam', age:27})-[:works {since: 2010}]->(:employer {name:'Dunder Mifflin'})"
    );
}

fn ensure_test_data(name:&str) {
    let mut con = get_con();
    let _:() = con.del(name).unwrap();
    let _ = con.graph_query(name, 
        "CREATE (:Rider {name:'Valentino Rossi', born:1979})-[:rides]->(:Team {name:'Yamaha'}), \
        (:Rider {name:'Dani Pedrosa', born:1985})-[:rides]->(:Team {name:'Honda'}), \
        (:Rider {name:'Andrea Dovizioso'})-[:rides]->(:Team {name:'Ducati'})"
    );
}

#[test]
fn test_issue_graph_create_command() {
    let _:() = get_con().del("test_issue_graph_create_command").unwrap();
    let r = get_con().graph_query(
        "test_issue_graph_create_command", 
        "CREATE (:person {name:'Pam', age:27})-[:works {since: 2010}]->(:employer {name:'Dunder Mifflin'})"
    ).unwrap();
    assert!(!r.metadata.is_empty());
}


#[test]
fn test_match_query_result() {
    ensure_simple_data("test_match_query_result");

    let r = get_con().graph_query(
        "test_match_query_result", 
        "MATCH (n1)-[r]->(n2) RETURN n1, r, n2.name"
    ).unwrap();
    
    assert_eq!(r.header, ["n1", "r", "n2.name"]);
    assert_eq!(r.data.len(), 1);
    let data = r.data.get(0).unwrap();
    assert_eq!(data.data.len(), 3);
    
    match data.get_value("n1") {
        Some(GraphValue::Node(node)) => {
            assert_eq!(node.labels, ["person"]);
            let name:String = from_redis_value(node.properties.get("name").unwrap()).unwrap();
            assert_eq!(name, "Pam");
        },
        _ => assert!(false),
    }
    
    let n1 = data.get_node("n1").unwrap();
    let name:Option<String> = n1.get_property_option("name");
    assert_eq!(name, Some("Pam".to_string()));

    let not_exists:Option<usize> = n1.get_property_option("not_there");
    assert_eq!(not_exists, None);

    let invalid:Option<usize> = n1.get_property_option("name");
    assert_eq!(invalid, None);

    match data.get_value("r") {
        Some(GraphValue::Relation(rel)) => {
            assert_eq!(rel.rel_type, "works");
            let since:usize = from_redis_value(rel.properties.get("since").unwrap()).unwrap();
            assert_eq!(since, 2010);
        },
        _ => assert!(false),
    }

    let relation = data.get_relation("r").unwrap();
    assert_eq!(relation.rel_type, "works");

    let since:usize = relation.get_property_option("since").unwrap();
    assert_eq!(since, 2010);
    
    let name:String = data.get_scalar("n2.name").unwrap();
    assert_eq!(name, "Dunder Mifflin");

    assert!(!r.metadata.is_empty());

}

#[test]
fn test_match_scalar_result() {
    ensure_test_data("test_match_scalar_result");
    let res = get_con().graph_query(
        "test_match_scalar_result", 
        "MATCH (r:Rider)-[:rides]->(t:Team) WHERE t.name = 'Yamaha' RETURN r.name, t.name"
    ).unwrap();
    println!("{:?}", res);
    assert_eq!(res.data.len(), 1);
    assert_eq!(res.data.get(0).unwrap().data.len(), 2);
    let driver:String = res.data.get(0).unwrap().get_scalar("r.name").unwrap();
    let team:String = res.data.get(0).unwrap().get_scalar("t.name").unwrap();
    assert_eq!(driver, "Valentino Rossi");
    assert_eq!(team, "Yamaha");
}

#[test]
fn test_query_all_nodes() {
    ensure_test_data("test_query_all_nodes");
    let res = get_con().graph_query("test_query_all_nodes", "MATCH (r:Rider) RETURN r").unwrap();
    for data in res.data.iter() {
        let node = data.get_node("r").unwrap();
        assert!(node.labels.contains(&"Rider".to_string()));
    }
}

#[test]
fn test_unserialize_option() {
    ensure_test_data("test_unserialize_option");
    let res = get_con().graph_query("test_unserialize_option", "MATCH (r:Rider) RETURN r.born").unwrap();
    let born:Vec<Option<usize>> = res.data.iter().map(|v| v.get_scalar("r.born")).collect();
    assert_eq!(born.iter().filter(|v| v.is_none()).count(), 1);
    assert_eq!(born.iter().filter(|v| v.is_some()).count(), 2);
}

use redis::{from_redis_value, RedisResult};
use redis_graph::*;

pub fn check_graph_create_command(r: GraphResultSet) {
    assert!(!r.metadata.is_empty());
}

// Deep check a GraphResultSet
pub fn check_match_query_result(r: GraphResultSet) {
    assert_eq!(r.header, ["n1", "r", "n2.name"]);
    assert_eq!(r.data.len(), 1);
    let data = r.data.get(0).unwrap();
    assert_eq!(data.data.len(), 3);

    match data.get_value("n1") {
        Some(GraphValue::Node(node)) => {
            assert_eq!(node.labels, ["person"]);
            let name: String = from_redis_value(node.properties.get("name").unwrap()).unwrap();
            assert_eq!(name, "Pam");
        }
        _ => assert!(false),
    }

    let n1 = data.get_node("n1").unwrap();
    let name: Option<String> = n1.get_property_option("name");
    assert_eq!(name, Some("Pam".to_string()));

    let not_exists: Option<usize> = n1.get_property_option("not_there");
    assert_eq!(not_exists, None);

    let invalid: Option<usize> = n1.get_property_option("name");
    assert_eq!(invalid, None);

    match data.get_value("r") {
        Some(GraphValue::Relation(rel)) => {
            assert_eq!(rel.rel_type, "works");
            let since: usize = from_redis_value(rel.properties.get("since").unwrap()).unwrap();
            assert_eq!(since, 2010);
        }
        _ => assert!(false),
    }

    let relation = data.get_relation("r").unwrap();
    assert_eq!(relation.rel_type, "works");

    let since: usize = relation.get_property_option("since").unwrap();
    assert_eq!(since, 2010);

    let name: String = data.get_scalar("n2.name").unwrap();
    assert_eq!(name, "Dunder Mifflin");

    assert!(!r.metadata.is_empty());
}

pub fn check_match_scalar_result(res: GraphResultSet) {
    assert_eq!(res.data.len(), 1);
    assert_eq!(res.data.get(0).unwrap().data.len(), 2);
    let driver: String = res.data.get(0).unwrap().get_scalar("r.name").unwrap();
    let team: String = res.data.get(0).unwrap().get_scalar("t.name").unwrap();
    assert_eq!(driver, "Valentino Rossi");
    assert_eq!(team, "Yamaha");
}

pub fn check_query_all_nodes(res: GraphResultSet) {
    for data in res.data.iter() {
        let node = data.get_node("r").unwrap();
        assert!(node.labels.contains(&"Rider".to_string()));
    }
}

pub fn check_unserialize_option(res: GraphResultSet) {
    let born: Vec<Option<usize>> = res.data.iter().map(|v| v.get_scalar("r.born")).collect();
    assert_eq!(born.iter().filter(|v| v.is_none()).count(), 1);
    assert_eq!(born.iter().filter(|v| v.is_some()).count(), 2);
}

pub fn check_graph_profile(res: Vec<String>) {
    assert!(res.len() > 0);
}

pub fn check_graph_slowlog(res: Vec<SlowLogEntry>) {
    let first = res.first().unwrap();
    assert_eq!(first.command, "GRAPH.QUERY");
    assert!(res.len() > 0);
}

pub fn check_graph_config_set_valid(r: RedisResult<bool>) {
    assert!(r.unwrap());
}

pub fn check_graph_config_set_invalid(r: RedisResult<bool>) {
    assert_eq!(true, r.is_err());
}

pub fn check_graph_config_get(r: RedisResult<i32>) {
    assert_eq!(r.unwrap(), 500);
}

pub fn check_graph_config_get_all(r: GraphConfig) {
    assert!(!r.values.is_empty());
    let v: i32 = r.get_value("RESULTSET_SIZE").unwrap().unwrap();
    assert_eq!(v, 500);
}

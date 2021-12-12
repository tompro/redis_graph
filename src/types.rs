use redis::{from_redis_value, FromRedisValue, RedisError, RedisResult, Value};
use std::collections::HashMap;

/// Contains the result of a Redis graph operation. All types of graph
/// operations will return a result in this format. Some (for example
/// CREATE) will only return data for select fields.
#[derive(Default, Clone, Debug)]
pub struct GraphResultSet {
    /// A list of string keys occuring in the RETURN statement of a query.
    pub header: Vec<String>,

    /// A list of GraphResults with one entry per match of the query.
    pub data: Vec<GraphResult>,

    /// List of metadata returned with the query (eg. affected rows).
    pub metadata: Vec<String>,
}

/// A graph query can return one or multiple values for every matching entry.
/// A GraphResult contains a map of values for a single match. The map keys
/// are the query RETURN statements, values can be of any GraphValue type in
/// any order.
/// The impl also contains some helper methods for easier extraction of graph
/// values.
///
/// ```rust
///use redis_graph::GraphResult;
///let res = GraphResult::default();
///
///let name:Option<String> = res.get_scalar("person2.name");
///let person = res.get_node("person");
///let friend_rel = res.get_relation("friend");
///
/// ```
///
#[derive(Default, Clone, Debug)]
pub struct GraphResult {
    /// A map of raw return keys to GraphValues.
    pub data: HashMap<String, GraphValue>,
}

/// Redis graph values can be one of 3 different types. Scalars are single
/// values of any type supported by Redis. Nodes are sort of objects which
/// can contain multiple name value pairs and Relations are relations between
/// Nodes which themself can contain multiple name value pairs.
#[derive(Clone, Debug)]
pub enum GraphValue {
    Scalar(Value),
    Node(NodeValue),
    Relation(RelationValue),
}

/// Represents a Redis graph node which is an object like structure with
/// potentially multiple named fields.
#[derive(Default, Clone, Debug)]
pub struct NodeValue {
    pub id: u64,
    pub labels: Vec<String>,
    pub properties: HashMap<String, Value>,
}

/// Represents a Redis graph relation between two nodes. Like a node it can
/// potentially contain one or multiple named fields.
#[derive(Default, Clone, Debug)]
pub struct RelationValue {
    pub id: u64,
    pub rel_type: String,
    pub src_node: u64,
    pub dest_node: u64,
    pub properties: HashMap<String, Value>,
}

/// Represents an entry returned from the GRAPH.SLOWLOG command.
#[derive(Default, Clone, Debug)]
pub struct SlowLogEntry {
    /// A unix timestamp at which the log entry was processed.
    pub timestamp: u64,
    /// The issued command.
    pub command: String,
    /// The issued query.
    pub query: String,
    /// The amount of time needed for its execution, in milliseconds.
    pub time: f64,
}

/// Simple wrapper around a graph config map that allows derserializing config
/// values into rust types.
#[derive(Default, Clone, Debug)]
pub struct GraphConfig {
    pub values: HashMap<String, Value>,
}

impl GraphConfig {
    /// Extracts a config Redis value at key into an Option of the desired type. Will
    /// return None in case the key did not exists. Will return an error in case the
    /// value at key failed to be parsed into T.
    pub fn get_value<T: FromRedisValue>(&self, key: &str) -> RedisResult<Option<T>> {
        match self.values.get(key) {
            Some(value) => from_redis_value(value),
            _ => Ok(None),
        }
    }
}

impl GraphResultSet {
    fn from_metadata(metadata: Vec<String>) -> Self {
        GraphResultSet {
            header: Vec::default(),
            data: Vec::default(),
            metadata,
        }
    }
}

/// Represents a group of returned graph values for a single matched result in
/// the query. Contains some helper methods for easier extraction of graph values.
impl GraphResult {
    /// Returns a single GraphValue by it's key.
    pub fn get_value(&self, key: &str) -> Option<&GraphValue> {
        self.data.get(key)
    }

    /// Tries to extract a graph Scalar value into target type T. Will return
    /// None in case the key does not exist, the target value is not a Scalar
    /// or the value could not be parsed into T.
    pub fn get_scalar<T: FromRedisValue>(&self, key: &str) -> Option<T> {
        match self.get_value(key) {
            Some(GraphValue::Scalar(value)) => from_redis_value(value).unwrap_or(None),
            _ => None,
        }
    }

    /// Tries to extract a graph Node value from Value at key. Will return
    /// None in case the key does not exist or the target value is not a  
    /// Node.
    pub fn get_node(&self, key: &str) -> Option<&NodeValue> {
        match self.get_value(key) {
            Some(GraphValue::Node(value)) => Some(value),
            _ => None,
        }
    }

    /// Tries to extract a graph Relation value from Value at key. Will return
    /// None in case the key does not exist or the target value is not a  
    /// Relation.
    pub fn get_relation(&self, key: &str) -> Option<&RelationValue> {
        match self.get_value(key) {
            Some(GraphValue::Relation(value)) => Some(value),
            _ => None,
        }
    }
}

/// Enhances object like graph values (Node, Relation) that contain a map of
/// properties with extraction function that allow parsing of the inner
/// Redis values into requested types.
pub trait WithProperties {
    /// Returns a raw Redis value at key.
    fn get_property_value(&self, key: &str) -> Option<&Value>;

    /// Extracts a property Redis value at key into an Option of the desired type. Will
    /// return None in case the key did not exists. Will return an error in case the
    /// value at key failed to be parsed into T.
    fn get_property<T: FromRedisValue>(&self, key: &str) -> RedisResult<Option<T>> {
        match self.get_property_value(key) {
            Some(value) => from_redis_value(value),
            _ => Ok(None),
        }
    }

    /// Extracts a property Redis value at key into an Option of the desired type. Will
    /// return None in case of the key did not exist or the value at key failed to be
    /// parsed into T.
    fn get_property_option<T: FromRedisValue>(&self, key: &str) -> Option<T> {
        self.get_property(key).unwrap_or(None)
    }
}

/// Allows property extraction on NodeValues.
impl WithProperties for NodeValue {
    fn get_property_value(&self, key: &str) -> Option<&Value> {
        self.properties.get(key)
    }
}

/// Allows property extraction on RelationValues.
impl WithProperties for RelationValue {
    fn get_property_value(&self, key: &str) -> Option<&Value> {
        self.properties.get(key)
    }
}

impl FromRedisValue for GraphResultSet {
    fn from_redis_value(v: &Value) -> RedisResult<Self> {
        match *v {
            Value::Bulk(ref values) if values.is_empty() => Ok(GraphResultSet::default()),
            Value::Bulk(ref values) if values.len() == 1 => match values.get(0) {
                Some(v) => {
                    let data: Vec<String> = from_redis_value(v)?;
                    Ok(GraphResultSet::from_metadata(data))
                }
                _ => Ok(GraphResultSet::default()),
            },
            Value::Bulk(ref values) => {
                let header: Vec<String> = match values.get(0) {
                    Some(v) => from_redis_value(v)?,
                    _ => Vec::default(),
                };

                let data: Vec<GraphResult> = match values.get(1) {
                    Some(Value::Bulk(v)) => v
                        .iter()
                        .map(|bulk| {
                            let items: Vec<GraphValue> = from_redis_value(bulk)?;
                            let mut data: HashMap<String, GraphValue> = HashMap::new();
                            for (idx, name) in header.iter().enumerate() {
                                if let Some(value) = items.get(idx) {
                                    data.insert(name.to_string(), value.to_owned());
                                }
                            }
                            Ok(GraphResult { data })
                        })
                        .collect::<RedisResult<Vec<GraphResult>>>()?,
                    _ => Vec::default(),
                };

                let metadata: Vec<String> = match values.get(2) {
                    Some(v) => from_redis_value(v)?,
                    _ => Vec::default(),
                };

                Ok(GraphResultSet {
                    header,
                    data,
                    metadata,
                })
            }
            _ => Err(create_error("Could not parse graph result")),
        }
    }
}

impl FromRedisValue for GraphValue {
    fn from_redis_value(v: &Value) -> RedisResult<Self> {
        match v {
            Value::Bulk(ref values) if values.len() == 3 => {
                let res: NodeValue = from_redis_value(v)?;
                Ok(GraphValue::Node(res))
            }
            Value::Bulk(_) => {
                let res: RelationValue = from_redis_value(v)?;
                Ok(GraphValue::Relation(res))
            }
            value => Ok(GraphValue::Scalar(value.clone())),
        }
    }
}

impl FromRedisValue for NodeValue {
    fn from_redis_value(v: &Value) -> RedisResult<Self> {
        let values = to_property_map(v)?;
        let id: u64 = values
            .get("id")
            .map_or(Ok(Some(0)), from_redis_value)?
            .unwrap();
        let labels: Vec<String> = if values.get("labels").is_some() {
            from_redis_value(values.get("labels").unwrap())?
        } else {
            Vec::default()
        };
        let properties: HashMap<String, Value> = if values.get("properties").is_some() {
            to_property_map(values.get("properties").unwrap())?
        } else {
            HashMap::default()
        };

        Ok(NodeValue {
            id,
            labels,
            properties,
        })
    }
}

impl FromRedisValue for RelationValue {
    fn from_redis_value(v: &Value) -> RedisResult<Self> {
        let values = to_property_map(v)?;
        let id: u64 = values
            .get("id")
            .map_or(Ok(Some(0)), from_redis_value)?
            .unwrap();
        let rel_type: String = values
            .get("type")
            .map_or(Ok(Some("".to_string())), from_redis_value)?
            .unwrap();
        let src_node: u64 = values
            .get("src_node")
            .map_or(Ok(Some(0)), from_redis_value)?
            .unwrap();
        let dest_node: u64 = values
            .get("dest_node")
            .map_or(Ok(Some(0)), from_redis_value)?
            .unwrap();
        let properties: HashMap<String, Value> = if values.get("properties").is_some() {
            to_property_map(values.get("properties").unwrap())?
        } else {
            HashMap::new()
        };

        Ok(RelationValue {
            id,
            rel_type,
            src_node,
            dest_node,
            properties,
        })
    }
}

impl FromRedisValue for SlowLogEntry {
    fn from_redis_value(v: &Value) -> RedisResult<Self> {
        match v {
            Value::Bulk(ref values) if values.len() == 4 => Ok(SlowLogEntry {
                timestamp: from_redis_value(values.get(0).unwrap())?,
                command: from_redis_value(values.get(1).unwrap())?,
                query: from_redis_value(values.get(2).unwrap())?,
                time: from_redis_value(values.get(3).unwrap())?,
            }),
            _ => Err(create_error("invalid_slow_log_entry")),
        }
    }
}

impl FromRedisValue for GraphConfig {
    fn from_redis_value(v: &Value) -> RedisResult<Self> {
        match v {
            Value::Bulk(_) => Ok(GraphConfig {
                values: to_property_map(v)?,
            }),
            _ => Ok(GraphConfig {
                values: HashMap::default(),
            }),
        }
    }
}

// Wraps a string error msg into a RedisError
pub fn create_error(msg: &str) -> RedisError {
    RedisError::from(std::io::Error::new(
        std::io::ErrorKind::Other,
        msg.to_string(),
    ))
}

// Extracts a list of name value pairs from a graph result
pub fn to_property_map(v: &Value) -> RedisResult<HashMap<String, Value>> {
    let t: Vec<Vec<Value>> = match from_redis_value(v) {
        Ok(v) => v,
        _ => vec![],
    };
    let mut values: HashMap<String, Value> = HashMap::default();
    for pair in t {
        if pair.len() == 2 {
            let key: String = from_redis_value(&pair[0])?;
            values.insert(key, pair[1].clone());
        }
    }
    Ok(values)
}

pub fn value_from_pair<T: FromRedisValue>(v: &Value) -> RedisResult<T> {
    let r: (String, T) = from_redis_value(v)?;
    Ok(r.1)
}

use redis::{Value, FromRedisValue, RedisError, RedisResult, from_redis_value};
use std::collections::HashMap;

#[derive(Default, Clone, Debug)]
pub struct GraphResultSet {
    pub header: Vec<String>,
    pub data: Vec<GraphResult>,
    pub metadata: Vec<String>,
}

#[derive(Default, Clone, Debug)]
pub struct GraphResult {
    pub data: HashMap<String, GraphValue>,
}

#[derive(Clone, Debug)]
pub enum GraphValue {
    Scalar(Value),
    Node(NodeValue),
    Relation(RelationValue),
}

#[derive(Default, Clone, Debug)]
pub struct NodeValue {
    pub id: u64,
    pub labels: Vec<String>,
    pub properties: HashMap<String, Value>,
}

#[derive(Default, Clone, Debug)]
pub struct RelationValue {
    pub id: u64,
    pub rel_type: String,
    pub src_node: u64,
    pub dest_node: u64,
    pub properties: HashMap<String, Value>,
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

impl FromRedisValue for GraphResultSet {
    
    fn from_redis_value(v: &Value) -> RedisResult<Self> {
        match *v {
            Value::Bulk(ref values) if values.is_empty() => {
                Ok(GraphResultSet::default())
            },
            Value::Bulk(ref values) if values.len() == 1 => {
                match values.get(0) {
                    Some(v) => {
                        let data:Vec<String> = from_redis_value(v)?;
                        Ok(GraphResultSet::from_metadata(data))
                    },
                    _ => Ok(GraphResultSet::default()),
                }
            },
            Value::Bulk(ref values) => {
                
                let header:Vec<String> = match values.get(0) {
                    Some(v) => from_redis_value(v)?,
                    _ => Vec::default(),
                };

                let data:Vec<GraphResult> = match values.get(1) {
                    Some(Value::Bulk(v)) => {
                        v.iter().map(|bulk| {
                            let items:Vec<GraphValue> = from_redis_value(bulk)?;
                            let mut data:HashMap<String,GraphValue> = HashMap::new();
                            for (idx, name) in header.iter().enumerate() {
                                match items.get(idx) {
                                    Some(value) => {data.insert(name.to_string(), value.to_owned());},
                                    _ => ()
                                }
                            }
                            Ok(GraphResult { data })
                        }).collect::<RedisResult<Vec<GraphResult>>>()?
                    },
                    _ => Vec::default(),
                };


                let metadata:Vec<String> = match values.get(2) {
                    Some(v) => from_redis_value(v)?,
                    _ => Vec::default(),
                };

                Ok(GraphResultSet { header, data, metadata })
            },
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
            },
            Value::Bulk(_) => {
                let res: RelationValue = from_redis_value(v)?;
                Ok(GraphValue::Relation(res))
            },
            value => Ok(GraphValue::Scalar(value.clone())),
        }
    }
}

impl FromRedisValue for NodeValue {
    fn from_redis_value(v: &Value) -> RedisResult<Self> {
        let values = to_property_map(v)?;
        let id:u64 = values.get("id").map_or(Ok(Some(0)), |v| from_redis_value(v))?.unwrap();
        let labels:Vec<String> = if values.get("labels").is_some() {
            from_redis_value(values.get("labels").unwrap())?
        } else {
            Vec::default()
        };
        let properties:HashMap<String,Value> = if values.get("properties").is_some() {
            to_property_map(values.get("properties").unwrap())?
        } else {
            HashMap::default()
        };

        Ok(NodeValue { id, labels, properties })
    }
}

impl FromRedisValue for RelationValue {
    fn from_redis_value(v: &Value) -> RedisResult<Self> {
        let values = to_property_map(v)?;
        let id:u64 = values.get("id").map_or(Ok(Some(0)), |v| from_redis_value(v))?.unwrap();
        let rel_type:String = values.get("type").map_or(Ok(Some("".to_string())), |v| from_redis_value(v))?.unwrap();
        let src_node:u64 = values.get("src_node").map_or(Ok(Some(0)), |v| from_redis_value(v))?.unwrap();
        let dest_node:u64 = values.get("dest_node").map_or(Ok(Some(0)), |v| from_redis_value(v))?.unwrap();
        let properties:HashMap<String,Value> = if values.get("properties").is_some() {
            to_property_map(values.get("properties").unwrap())?
        } else {
            HashMap::new()
        };
        
        Ok(RelationValue { id, rel_type, src_node, dest_node, properties })
    }
}

// Wraps a string error msg into a RedisError
fn create_error(msg:&str) -> RedisError {
    RedisError::from(std::io::Error::new(
        std::io::ErrorKind::Other, 
        msg.to_string()
    ))
}

// Extracts a list of name value pairs from a graph result
fn to_property_map(v:&Value) -> RedisResult<HashMap<String,Value>> {
    let t:Vec<HashMap<String,Value>> = from_redis_value(v)?;
    let mut values:HashMap<String,Value> = HashMap::default();
    for pair in t {
        for (key, value) in pair {
            values.insert(key, value);
        }
    }
    Ok(values)
}


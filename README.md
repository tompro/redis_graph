# redis_graph

[![crates.io](https://img.shields.io/badge/crates.io-v0.1.0-orange)](https://crates.io/crates/redis_graph)
![Continuous integration](https://github.com/tompro/redis_graph/workflows/Continuous%20integration/badge.svg)

redis_graph proivdes a small trait with an extension function for the
[redis](https://docs.rs/redis/) crate to allow working with redis graph 
data types that can be installed as a [redis module](https://oss.redislabs.com/redisgraph). 
Redis graph operation are only using a single top level Redis command, so 
this crate only adds a single function to the redis commands.
The Graph command is available in a synchronous and asynchronous version.

The crate is called `redis_graph` and you can depend on it via cargo. You will
also need redis in your dependencies.

```ini
[dependencies]
redis = "0.17.0"
redis-graph = "*"
```

Or via git:

```ini
[dependencies.redis-graph]
git = "https://github.com/tompro/redis_graph.git"
branch = "main"
```


## Synchronous usage

To enable the redis graph command you simply load the trait
redis_graph::GraphCommands into scope. The redis graph
command will then be available on your redis connection.
To also have access to the value extractor traits simply import 
the whole crate redis_graph::*.

 
```rust
use redis::Commands;
use redis_graph::*;

let client = redis::Client::open("redis://127.0.0.1/")?;
let mut con = client.get_connection()?;

let _:GraphResultSet = con.graph_query(
    "my_graph", 
    "CREATE (:Rider {name:'Valentino Rossi'})-[:rides]->(:Team {name:'Yamaha'})"
)?;
```


## Asynchronous usage

To enable the redis graph async command you simply load the
redis_graph::AsyncGraphCommands into the scope. To also have access 
to the value extractor traits simply import the whole crate redis_graph::*.

```rust
use redis::AsyncCommands;
use redis_graph::*;

let client = redis::Client::open("redis://127.0.0.1/")?;
let mut con = client.get_async_connection().await?;

let _:GraphResultSet = con.graph_query(
    "my_graph", 
    "CREATE (:Rider {name:'Valentino Rossi'})-[:rides]->(:Team {name:'Yamaha'})"
).await?;
```

## Other rust Redis graph libraries

[redisgraph-rs](https://github.com/malte-v/redisgraph-rs) is more high level crate 
for the Redis graph module. At time of writing it did not support async operations. 

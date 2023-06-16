# redis_graph

[![crates.io](https://img.shields.io/badge/crates.io-v0.4.3-orange)](https://crates.io/crates/redis_graph)
![Continuous integration](https://github.com/tompro/redis_graph/workflows/Continuous%20integration/badge.svg)

redis-graph provides a small trait with an extension function for the
[redis](https://docs.rs/redis/) crate to allow working with redis graph 
data types that can be installed as a [redis module](https://oss.redislabs.com/redisgraph). 
Redis graph operations are mostly using two top level Redis commands
(one for read/write operations and one for read-only operations). In addition 
to those there are some more maintenance oriented commands for perfomance, 
configuration and clean-up which starting from v0.4.0 are also supported.
The Graph commands are available in synchronous and asynchronous versions.

The crate is called `redis-graph` and you can depend on it via cargo. You will
also need redis in your dependencies. This version was tested against redis 0.21.0 
but should run with versions higher than that.

```ini
[dependencies]
redis = "0.23.0"
redis-graph = "0.4.4"
```

Or via git:

```ini
[dependencies.redis-graph]
git = "https://github.com/tompro/redis_graph.git"
branch = "main"
```

With async feature inherited from the [redis](https://docs.rs/redis) crate (either: 'async-std-comp' or 'tokio-comp):

```ini
[dependencies]
redis = "0.23.0"
redis-graph = { version = "0.4.4", features = ['tokio-comp'] }
```

## Synchronous usage

To enable the redis graph commands you simply load the trait
redis_graph::GraphCommands into scope. The redis graph
commands will then be available on your redis connection.
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

let _:GraphResultSet = con.graph_ro_query(
    "my_graph",
    "MATCH (rider:Rider)-[:rides]->(:Team {name:'Yamaha'}) RETURN rider"
)?;
```


## Asynchronous usage

To enable the redis graph async commands you simply load the
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

let _:GraphResultSet = con.graph_ro_query(
    "my_graph", 
    "MATCH (rider:Rider)-[:rides]->(:Team {name:'Yamaha'}) RETURN rider"
).await?;
```

## Other rust Redis graph libraries

[redisgraph-rs](https://github.com/malte-v/redisgraph-rs) is more high level crate 
for the Redis graph module. At time of writing it did not support async operations. 

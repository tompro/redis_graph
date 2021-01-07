extern crate redis;
extern crate redis_graph;
extern crate tokio;

#[cfg(test)]
#[path = "./test_graph_assertions.rs"]
mod test_graph_assertions;

#[cfg(test)]
#[path = "issue_async_commands.rs"]
mod issue_async_commands;

use issue_async_commands::*;
use test_graph_assertions::*;
use tokio::runtime::Runtime;

fn create_runtime() -> Runtime {
    let mut builder = tokio::runtime::Builder::new_current_thread();
    builder.enable_io().build().unwrap()
}

#[test]
fn test_issue_graph_create_command() {
    let r = create_runtime().block_on(issue_graph_create_command(
        "test_issue_graph_create_command_std",
    ));
    check_graph_create_command(r);
}

#[test]
fn test_match_query_result() {
    let r = create_runtime().block_on(issue_match_query_command("test_match_query_result_std"));
    check_match_query_result(r);
}

#[test]
fn test_match_scalar_result() {
    let res = create_runtime().block_on(issue_match_scalar_result("test_match_scalar_result_std"));
    check_match_scalar_result(res);
}

#[test]
fn test_query_all_nodes() {
    let res = create_runtime().block_on(issue_query_all_nodes("test_query_all_nodes_std"));
    check_query_all_nodes(res);
}

#[test]
fn test_unserialize_option() {
    let res = create_runtime().block_on(issue_query_option("test_unserialize_option_std"));
    check_unserialize_option(res);
}

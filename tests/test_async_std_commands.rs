extern crate async_std;
extern crate redis;
extern crate redis_graph;

#[cfg(test)]
#[path = "./test_graph_assertions.rs"]
mod test_graph_assertions;

#[cfg(test)]
#[path = "issue_async_commands.rs"]
mod issue_async_commands;

use async_std::task;
use issue_async_commands::*;
use test_graph_assertions::*;

#[test]
fn test_issue_graph_create_command() {
    let r = task::block_on(issue_graph_create_command(
        "test_issue_graph_create_command_std",
    ));
    check_graph_create_command(r);
}

#[test]
fn test_match_query_result() {
    let r = task::block_on(issue_match_query_command("test_match_query_result_std"));
    check_match_query_result(r);
}

#[test]
fn test_match_ro_query_result() {
    let r = task::block_on(issue_match_ro_query_command(
        "test_match_ro_query_result_std",
    ));
    check_match_query_result(r);
}

#[test]
fn test_match_scalar_result() {
    let res = task::block_on(issue_match_scalar_result("test_match_scalar_result_std"));
    check_match_scalar_result(res);
}

#[test]
fn test_query_all_nodes() {
    let res = task::block_on(issue_query_all_nodes("test_query_all_nodes_std"));
    check_query_all_nodes(res);
}

#[test]
fn test_unserialize_option() {
    let res = task::block_on(issue_query_option("test_unserialize_option_std"));
    check_unserialize_option(res);
}

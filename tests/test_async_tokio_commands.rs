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
        "test_issue_graph_create_command_tokio",
    ));
    check_graph_create_command(r);
}

#[test]
fn test_match_query_result() {
    let r = create_runtime().block_on(issue_match_query_command("test_match_query_result_tokio"));
    check_match_query_result(r);
}

#[test]
fn test_match_ro_query_result() {
    let r = create_runtime().block_on(issue_match_ro_query_command(
        "test_match_ro_query_result_tokio",
    ));
    check_match_query_result(r);
}

#[test]
fn test_match_scalar_result() {
    let res =
        create_runtime().block_on(issue_match_scalar_result("test_match_scalar_result_tokio"));
    check_match_scalar_result(res);
}

#[test]
fn test_query_all_nodes() {
    let res = create_runtime().block_on(issue_query_all_nodes("test_query_all_nodes_tokio"));
    check_query_all_nodes(res);
}

#[test]
fn test_unserialize_option() {
    let res = create_runtime().block_on(issue_query_option("test_unserialize_option_tokio"));
    check_unserialize_option(res);
}

#[test]
fn test_graph_profile() {
    let res = create_runtime().block_on(issue_graph_profile_query("test_graph_profile_tokio"));
    check_graph_profile(res);
}

#[test]
fn test_graph_slowlog() {
    let res = create_runtime().block_on(issue_graph_slowlog_query("test_graph_slowlog_tokio"));
    check_graph_slowlog(res);
}

#[test]
fn test_graph_config_set_invalid() {
    let err_res = create_runtime().block_on(issue_graph_config_set_invalid());
    check_graph_config_set_invalid(err_res);
}

#[test]
fn test_graph_config_set() {
    let res = create_runtime().block_on(issue_graph_config_set());
    check_graph_config_set_valid(res);
}

#[test]
fn test_graph_config_get() {
    let res = create_runtime().block_on(issue_graph_config_get());
    check_graph_config_get(res);
}

#[test]
fn test_graph_config_get_all() {
    let res = create_runtime().block_on(issue_graph_config_get_all());
    check_graph_config_get_all(res);
}

#[test]
fn test_graph_delete() {
    let res = create_runtime().block_on(issue_graph_delete("test_graph_delete_tokio"));
    check_graph_delete_success(res);
}

#[test]
fn test_graph_explain() {
    let res = create_runtime().block_on(issue_graph_explain("test_graph_explain_tokio"));
    check_graph_explain_result(res);
}

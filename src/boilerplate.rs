//! common rust boilerplate

use tracing_subscriber::EnvFilter;

/// Start boilerplate tracing subscriber, with a default log level, when none is specified
///
/// Allows easily setting regular tracing or console input via config.  
/// Does its job alright for now.
///
/// # Note:
/// - Not optimal.  But has more info than default and allows easily setting a default log level.
/// - We should return a `Result`, but I don't want to mess with the flow of this repo's tour by adding different error types and consequent handling strategies.
pub fn tracing_subscriber_boilerplate(log_default: &str) {
    let filter =
        EnvFilter::try_new(std::env::var("RUST_LOG").unwrap_or_else(|_| log_default.to_string()))
            .expect("Valid filter input provided.");

    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(filter)
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(true)
        .init();
}

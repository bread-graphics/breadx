// MIT/Apache2 License

#![cfg(test)]

/// Set up a `tracing` subscriber for testing purposes.
pub(crate) fn setup_tracing() {
    tracing_subscriber::fmt::fmt()
        //.with_max_level(tracing::Level::TRACE)
        .try_init()
        .ok();
}
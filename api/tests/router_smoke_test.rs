//! Router construction smoke test.
//!
//! axum panics at `Router::route()` time — not compile time — on invalid
//! path syntax (e.g. the 0.7-style `/:param` after the 0.8 upgrade) and on
//! conflicting route registrations. `cargo check` and clippy cannot catch
//! that class of failure; constructing the full router here can. This test
//! is part of the no-DB CI gate and must stay dependency-free.

#[test]
fn api_router_constructs_without_panicking() {
    let _ = revolution_api::routes::api_router();
}

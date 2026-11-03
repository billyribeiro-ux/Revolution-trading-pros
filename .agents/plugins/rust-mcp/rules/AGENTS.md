# Rust MCP Server (`rust-mcp-server`)

The `rust` MCP server (`rust-mcp-server` v0.4.1, targeting `./api`) is configured for this repository.

## Available Tools

- **Cargo:** `cargo-build`, `cargo-check`, `cargo-test`, `cargo-clippy`, `cargo-fmt`, `cargo-doc`
- **Dependencies:** `cargo-add`, `cargo-remove`, `cargo-update`, `cargo-search`, `cargo-info`
- **Quality & Audits:** `cargo-deny-check`, `cargo-machete`, `cargo-hack`
- **Toolchain:** `rustc-explain`, `rustup-show`, `rustup-toolchain-add`, `rustup-update`

## Discipline

Run `cargo-check` and `cargo-clippy` after any edit to `api/src/`, and run `cargo-deny-check` whenever adding or updating crates in `api/Cargo.toml`.

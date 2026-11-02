//! Outbound HTTP clients.
//!
//! Every external call (Bunny, Anthropic, form webhooks, Stripe, Postmark,
//! OAuth providers) must go through a client that sets BOTH a total request
//! `timeout` and a `connect_timeout`; a bare `reqwest::Client::new()` has
//! neither and lets a stalled upstream pin a task forever.
//!
//! Building a client only fails when the TLS backend cannot initialise, which
//! would break every outbound call anyway. [`build_client`] therefore fails
//! loudly instead of silently falling back to an untimed client.

use std::{sync::LazyLock, time::Duration};

use reqwest::{Client, redirect::Policy};

/// Connect timeout shared by every client below.
const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);

/// Build a client with both timeouts set. Panics if the TLS backend cannot
/// initialise (see module docs) rather than returning an untimed client.
pub fn build_client(request_timeout: Duration) -> Client {
    Client::builder()
        .timeout(request_timeout)
        .connect_timeout(CONNECT_TIMEOUT)
        .build()
        .expect("reqwest Client with timeouts must build (TLS backend init failed)")
}

/// General-purpose API calls (Bunny REST, status polling): 30s.
pub static OUTBOUND_HTTP: LazyLock<Client> =
    LazyLock::new(|| build_client(Duration::from_secs(30)));

/// Video body uploads to Bunny. The route accepts bodies up to 5 GiB, so the
/// total timeout is generous (30 min); the connect timeout still bounds a dead host.
pub static UPLOAD_HTTP: LazyLock<Client> =
    LazyLock::new(|| build_client(Duration::from_secs(30 * 60)));

/// LLM calls (Anthropic Messages API, incl. streaming): 3 min total.
pub static LLM_HTTP: LazyLock<Client> = LazyLock::new(|| build_client(Duration::from_secs(180)));

/// User-configured form webhooks: short timeout, and redirects disabled so a
/// 3xx cannot bounce a request past the SSRF URL validation done before send.
pub static WEBHOOK_HTTP: LazyLock<Client> = LazyLock::new(|| {
    Client::builder()
        .timeout(Duration::from_secs(10))
        .connect_timeout(Duration::from_secs(5))
        .redirect(Policy::none())
        .build()
        .expect("reqwest webhook Client must build (TLS backend init failed)")
});

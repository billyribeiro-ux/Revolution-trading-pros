//! P1-3 (FULL_REPO_AUDIT_2026-05-17): spoof-resistant client-IP resolution.
//!
//! These are pure, no-DB unit tests for `routes::auth::resolve_client_ip`
//! — the function that decides which IP a per-IP rate-limit bucket is
//! keyed by. The whole point of the audit fix is that an attacker can no
//! longer mint a fresh bucket per request by rotating `X-Forwarded-For`,
//! so the adversarial cases (untrusted peer + spoofed XFF, malformed XFF,
//! all-trusted chains) are the load-bearing assertions here.
//!
//! They bind to the REAL production function via the library crate
//! (`revolution_api::routes::auth::resolve_client_ip`) and the REAL CIDR
//! type (`revolution_api::config::IpCidr`) so a regression that re-trusts
//! forwarded headers — or weakens the CIDR math — fails the suite.

use std::net::IpAddr;

use axum::http::HeaderMap;
use revolution_api::config::IpCidr;
use revolution_api::routes::auth::resolve_client_ip;

// ── helpers ─────────────────────────────────────────────────────────

fn ip(s: &str) -> IpAddr {
    s.parse().expect("test IP literal must parse")
}

fn cidrs(specs: &[&str]) -> Vec<IpCidr> {
    specs
        .iter()
        .map(|s| IpCidr::parse(s).expect("test CIDR literal must parse"))
        .collect()
}

/// Build a `HeaderMap` from `(name, value)` pairs.
fn headers(pairs: &[(&str, &str)]) -> HeaderMap {
    let mut h = HeaderMap::new();
    for (k, v) in pairs {
        h.insert(
            axum::http::HeaderName::from_bytes(k.as_bytes()).unwrap(),
            axum::http::HeaderValue::from_str(v).unwrap(),
        );
    }
    h
}

// ── 1. Untrusted peer + spoofed XFF → peer IP, spoof IGNORED ─────────

#[test]
fn untrusted_peer_ignores_spoofed_xff() {
    let trusted = cidrs(&["10.0.0.0/8"]); // our LB range
    let peer = ip("203.0.113.99"); // attacker connecting directly
    let h = headers(&[("x-forwarded-for", "1.2.3.4")]); // attacker-chosen

    // The forged header MUST be discarded; the real socket peer wins.
    assert_eq!(resolve_client_ip(peer, &h, &trusted), peer);
}

#[test]
fn untrusted_peer_ignores_x_real_ip_too() {
    let trusted = cidrs(&["10.0.0.0/8"]);
    let peer = ip("203.0.113.99");
    let h = headers(&[
        ("x-forwarded-for", "8.8.8.8, 9.9.9.9"),
        ("x-real-ip", "7.7.7.7"),
    ]);
    assert_eq!(resolve_client_ip(peer, &h, &trusted), peer);
}

#[test]
fn rotating_xff_cannot_change_the_bucket_key_when_peer_untrusted() {
    // This is the exact attack the audit filed: a script rotating XFF to
    // dodge the limiter. With an untrusted peer every request resolves to
    // the SAME ip (the socket peer), so the limiter sees one bucket.
    let trusted = cidrs(&["10.0.0.0/8"]);
    let peer = ip("198.51.100.7");
    let forged = [
        "1.1.1.1",
        "2.2.2.2, 3.3.3.3",
        "4.4.4.4, 5.5.5.5, 6.6.6.6",
        "", // empty
    ];
    for f in forged {
        let h = headers(&[("x-forwarded-for", f)]);
        assert_eq!(
            resolve_client_ip(peer, &h, &trusted),
            peer,
            "spoofed XFF {f:?} must not change the resolved IP"
        );
    }
}

// ── 2. Trusted peer + XFF chain → rightmost UNTRUSTED hop ────────────

#[test]
fn trusted_peer_returns_real_client_from_xff() {
    let trusted = cidrs(&["10.0.0.0/8"]);
    let peer = ip("10.0.0.1"); // our load balancer
                               // [ real client , (nothing else) ] — single proxy in front.
    let h = headers(&[("x-forwarded-for", "203.0.113.42")]);
    assert_eq!(resolve_client_ip(peer, &h, &trusted), ip("203.0.113.42"));
}

#[test]
fn trusted_peer_picks_rightmost_untrusted_hop() {
    // Chain as the rightmost proxy sees it (left→right is client→edge):
    //   spoofed_client, real_client, trusted_proxy_a
    // The leftmost value is attacker-injected (a client can prepend
    // anything). Walking right-to-left and skipping trusted hops yields
    // `real_client`, NOT the injected `1.2.3.4`.
    let trusted = cidrs(&["10.0.0.0/8"]);
    let peer = ip("10.0.0.5");
    let h = headers(&[("x-forwarded-for", "1.2.3.4, 203.0.113.42, 10.0.0.9")]);
    assert_eq!(resolve_client_ip(peer, &h, &trusted), ip("203.0.113.42"));
}

// ── 3. Multiple chained trusted proxies ─────────────────────────────

#[test]
fn multiple_chained_trusted_proxies_are_all_skipped() {
    // client -> edge LB -> internal LB -> app. Both LBs are trusted and
    // appear (rightmost-first) as 10.0.0.9 then 172.16.0.4.
    let trusted = cidrs(&["10.0.0.0/8", "172.16.0.0/12"]);
    let peer = ip("10.0.0.9");
    let h = headers(&[("x-forwarded-for", "198.51.100.23, 172.16.0.4, 10.0.0.9")]);
    assert_eq!(resolve_client_ip(peer, &h, &trusted), ip("198.51.100.23"));
}

#[test]
fn all_hops_trusted_falls_back_to_peer() {
    // Pathological: every XFF entry is one of our own proxies. There is
    // no external client to attribute, so the trusted peer IP is used.
    let trusted = cidrs(&["10.0.0.0/8"]);
    let peer = ip("10.0.0.2");
    let h = headers(&[("x-forwarded-for", "10.0.0.7, 10.0.0.8, 10.0.0.9")]);
    assert_eq!(resolve_client_ip(peer, &h, &trusted), peer);
}

// ── 4. IPv6 ─────────────────────────────────────────────────────────

#[test]
fn ipv6_trusted_peer_resolves_ipv6_client() {
    let trusted = cidrs(&["2001:db8:0::/48"]);
    let peer = ip("2001:db8:0::1");
    let h = headers(&[("x-forwarded-for", "2001:db8:cafe::99")]);
    assert_eq!(
        resolve_client_ip(peer, &h, &trusted),
        ip("2001:db8:cafe::99")
    );
}

#[test]
fn ipv6_untrusted_peer_ignores_xff() {
    let trusted = cidrs(&["2001:db8::/32"]);
    let peer = ip("2001:dead::1"); // not in the trusted /32
    let h = headers(&[("x-forwarded-for", "2001:db8::5")]);
    assert_eq!(resolve_client_ip(peer, &h, &trusted), peer);
}

#[test]
fn ipv4_mapped_v6_peer_matches_ipv4_allowlist() {
    // Dual-stack listeners report IPv4 clients as ::ffff:a.b.c.d. An
    // operator-written IPv4 trusted range must still recognise the proxy.
    let trusted = cidrs(&["10.0.0.0/8"]);
    let peer = ip("::ffff:10.0.0.4");
    let h = headers(&[("x-forwarded-for", "203.0.113.7")]);
    assert_eq!(resolve_client_ip(peer, &h, &trusted), ip("203.0.113.7"));
}

#[test]
fn bracketed_ipv6_xff_entries_parse() {
    let trusted = cidrs(&["10.0.0.0/8"]);
    let peer = ip("10.0.0.1");
    // RFC 7239 / proxy implementations sometimes bracket IPv6, with port.
    let h = headers(&[("x-forwarded-for", "[2001:db8::42]:51234")]);
    assert_eq!(resolve_client_ip(peer, &h, &trusted), ip("2001:db8::42"));
}

// ── 5. Empty / missing headers ──────────────────────────────────────

#[test]
fn missing_xff_with_trusted_peer_uses_peer() {
    let trusted = cidrs(&["10.0.0.0/8"]);
    let peer = ip("10.0.0.3");
    let h = HeaderMap::new(); // no headers at all
    assert_eq!(resolve_client_ip(peer, &h, &trusted), peer);
}

#[test]
fn empty_xff_value_with_trusted_peer_uses_peer() {
    let trusted = cidrs(&["10.0.0.0/8"]);
    let peer = ip("10.0.0.3");
    let h = headers(&[("x-forwarded-for", "   ,  , ")]);
    assert_eq!(resolve_client_ip(peer, &h, &trusted), peer);
}

#[test]
fn missing_xff_with_untrusted_peer_uses_peer() {
    let trusted = cidrs(&["10.0.0.0/8"]);
    let peer = ip("203.0.113.50");
    let h = HeaderMap::new();
    assert_eq!(resolve_client_ip(peer, &h, &trusted), peer);
}

// ── 6. Malformed XFF ────────────────────────────────────────────────

#[test]
fn malformed_xff_entries_are_skipped_not_trusted() {
    // The rightmost (closest to us) entry is garbage. The resolver must
    // NOT treat garbage as the client and must NOT abort to the peer
    // while a valid client hop still exists further left.
    let trusted = cidrs(&["10.0.0.0/8"]);
    let peer = ip("10.0.0.1");
    let h = headers(&[("x-forwarded-for", "203.0.113.77, not-an-ip, %%%garbage%%%")]);
    assert_eq!(resolve_client_ip(peer, &h, &trusted), ip("203.0.113.77"));
}

#[test]
fn fully_malformed_xff_falls_back_to_trusted_peer() {
    let trusted = cidrs(&["10.0.0.0/8"]);
    let peer = ip("10.0.0.1");
    let h = headers(&[("x-forwarded-for", "garbage, more-garbage, 999.999.999.999")]);
    assert_eq!(resolve_client_ip(peer, &h, &trusted), peer);
}

#[test]
fn malformed_xff_with_untrusted_peer_still_ignores_everything() {
    let trusted = cidrs(&["10.0.0.0/8"]);
    let peer = ip("203.0.113.5");
    let h = headers(&[("x-forwarded-for", "garbage, 1.2.3.4")]);
    assert_eq!(resolve_client_ip(peer, &h, &trusted), peer);
}

// ── 7. No trusted CIDRs configured → XFF ALWAYS ignored ─────────────

#[test]
fn no_trusted_cidrs_means_xff_is_always_ignored() {
    // Default deployment posture (TRUSTED_PROXY_CIDRS unset). NO peer can
    // ever be trusted, so forwarded headers are inert no matter what the
    // peer is. The socket peer is always authoritative.
    let trusted: Vec<IpCidr> = vec![];

    for peer_s in ["203.0.113.1", "10.0.0.1", "::1", "::ffff:10.0.0.1"] {
        let peer = ip(peer_s);
        let h = headers(&[
            ("x-forwarded-for", "1.2.3.4, 5.6.7.8"),
            ("x-real-ip", "9.9.9.9"),
        ]);
        assert_eq!(
            resolve_client_ip(peer, &h, &trusted),
            peer,
            "with no trusted CIDRs, peer {peer_s} must be authoritative"
        );
    }
}

#[test]
fn no_trusted_cidrs_no_headers_uses_peer() {
    let trusted: Vec<IpCidr> = vec![];
    let peer = ip("203.0.113.200");
    assert_eq!(resolve_client_ip(peer, &HeaderMap::new(), &trusted), peer);
}

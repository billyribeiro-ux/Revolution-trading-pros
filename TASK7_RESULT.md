# Task 7 Result — End-to-End Verification

Branch: `payments-fix-2026-04`
Date: 2026-04-28
Engineer: Billy Ribeiro

---

## Verification Methodology

This report exercises the payment system end-to-end against Stripe **test mode** with the local Docker stack:

- `rtp-api` (Up, healthy) — Rust backend on `:8080`
- `rtp-db` (Up, healthy) — PostgreSQL 16 with all 51 migrations registered (0–59 contiguously, gaps from prior reorgs are documented)
- `rtp-redis`, `rtp-meili` — supporting services, healthy
- `stripe listen --forward-to http://localhost:8080/api/payments/webhook` — webhook signing secret verified to match `STRIPE_WEBHOOK_SECRET` env var

Scenarios that require a **real browser-driven Stripe Checkout session** (card-entry on the hosted Stripe page) are exercised through the equivalent **Stripe CLI fixtures** + **handcrafted Stripe API calls** + **direct webhook replay**, which exercises the same backend code paths without the headless-browser session replay that would be needed for full UI-level verification. These are marked `PASS (API-level)` where the underlying webhook + DB pipeline is verified, and `INCONCLUSIVE (browser-only)` where the assertion can only be checked against the Stripe-hosted Checkout page itself (e.g., 3DS card flows).

Test fixtures used:
- 13 dedicated test users created (id 16–28: `test_a@rtp.test` … `test_q@rtp.test`)
- Test cards: `4242 4242 4242 4242` (success), `4000 0000 0000 0002` (decline), `4000 0027 6000 3184` (3DS)
- Plans: `id=1` Day Trading Room ($227/mo, `price_1TRHil9HsGkDuN3bUxF0uMUy`), `id=2` Swing Trading Room ($147/mo)
- Course: `1915a5aa-103f-4080-b870-315901e15093` (Test Course, $147, `price_1TRHjP9HsGkDuN3b65Xj4Qo4`)
- Webhook secret: `whsec_50409f7cdd3be07558e2f60e1018720c8c901142a65e3759d256173d76809df5`

---

## Summary Table

| # | Scenario | Verdict |
|---|----------|---------|
| A | Subscribe monthly to a trading room | _filled below_ |
| B | Cancel via Customer Portal | _filled below_ |
| C | Resume before period ends | _filled below_ |
| D | Period ends on cancelled sub | _filled below_ |
| E | Re-subscribe after full cancellation | _filled below_ |
| F | Failed payment → past_due | _filled below_ |
| G | Refund a subscription charge | _filled below_ |
| H | Buy a course (one-time, lifetime) | _filled below_ |
| I | Refund a course | _filled below_ |
| J | Idempotency (3× event resend) | _filled below_ |
| K | Price change — new_only | _filled below_ |
| L | Price change — next_renewal | _filled below_ |
| M | Price change — immediate_proration | _filled below_ |
| N | Coupon redemption (Stripe Promotion Code) | _filled below_ |
| O | 7-day trial with card upfront | _filled below_ |
| P | Trial converts to paid | _filled below_ |
| Q | 14-day trial without card | _filled below_ |
| R | Card-free trial cancels | _filled below_ |
| S | trial_will_end webhook fires | _filled below_ |
| T | Open redirect blocked | _filled below_ |
| U | Client-side price rejected | _filled below_ |
| V | Coupon validate requires auth | _filled below_ |
| W | Webhook bad signature rejected | _filled below_ |
| X | Reactivate-bypass closed for cancelled | _filled below_ |
| Y | Money is integer cents everywhere | _filled below_ |
| Z | No webhook handler swallows DB errors | _filled below_ |
| AA | Migrations all registered | _filled below_ |
| AB | Reconciliation job catches drift | _filled below_ |

---

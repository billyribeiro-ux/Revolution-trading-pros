# Batch 5b — unify-admin-auth

**Branch:** `unify-admin-auth` (off `main` at `9094cac87`)
**Status:** code complete + gates green; awaiting PR review.

## PR title (suggested)
```
refactor: replace local require_admin helpers with AdminUser extractor (Batch 5b)
```

## PR body (suggested)

### Summary

Three route files (`admin.rs`, `admin_members.rs`, `cms_v2_enterprise.rs`) maintained their own `require_admin` / `require_super_admin` / `require_superadmin` functions, each called as the first line of every admin handler. Meanwhile [api/src/middleware/admin.rs](api/src/middleware/admin.rs) already exposed `AdminUser` and `SuperAdminUser` extractors with the same role logic. The local helpers were a redundant gate that *also* allowed handlers to forget the check (and several `cms_v2_enterprise` routes did, which we left untouched here because the spec calls for a like-for-like conversion, not a security expansion).

This branch replaces every `require_admin(&user)?;` and `require_superadmin(&user)?;` call with the corresponding extractor in the function signature. The local helpers are deleted. Auth is now enforced by Axum at request-parse time rather than at function-body time, eliminating the "forgot the check" failure mode for converted handlers.

### Numbers

| File | `require_admin` → `AdminUser` | `require_super*` → `SuperAdminUser` | Helpers deleted |
|---|---|---|---|
| `api/src/routes/admin.rs` | 33 | 1 | 2 (`require_admin`, `require_super_admin`) |
| `api/src/routes/admin_members.rs` | 23 | 4 | 2 (`require_admin`, `require_superadmin`) |
| `api/src/routes/cms_v2_enterprise.rs` | 1 | 0 | 1 (`require_admin`) |

Total: **62 handlers converted, 5 helper functions deleted, ~140 net lines removed.**

### Diff stat

```
api/src/routes/admin.rs             | 167 +++++--------------------------
api/src/routes/admin_members.rs     | 126 +++++-----------------------
api/src/routes/cms_v2_enterprise.rs |  13 +--
3 files changed, 82 insertions(+), 224 deletions(-)
```

### Behavioral note (response shape change)

The deleted local helpers returned 403 with a JSON body:
```json
{"error":"Access denied","message":"This action requires admin privileges","your_role":"user"}
```
The `AdminUser` / `SuperAdminUser` extractors return 403 with the plain text body `"Admin access required"` / `"Super admin access required"`. Admin clients should treat 403 as "not authorized" without parsing the response body. The existing super-admin-on-role-grant inline check inside `update_user` ([api/src/routes/admin.rs:230-245](api/src/routes/admin.rs#L230-L245)) is preserved — that's a finer-grained "admin can update users but cannot grant privileged roles" guard, not redundant with `AdminUser`.

### Gates

| Gate | Result |
|---|---|
| `cargo check` (api/, --locked) | ✅ 0/0 |
| `cargo clippy --no-deps` (api/, --locked) | ✅ 0/0 |
| `pnpm check` (frontend/) | ✅ 0 errors / 0 warnings (5217 files) |

### Files

- [api/src/routes/admin.rs](api/src/routes/admin.rs)
- [api/src/routes/admin_members.rs](api/src/routes/admin_members.rs)
- [api/src/routes/cms_v2_enterprise.rs](api/src/routes/cms_v2_enterprise.rs)

### Test plan

- [ ] CI green.
- [ ] Smoke: an admin user can still call e.g. `GET /api/admin/coupons` — 200.
- [ ] Smoke: a non-admin user gets 403 from same endpoint (now plain-text body).
- [ ] Smoke: super-admin-only routes (e.g. `DELETE /api/admin/members/segments/:id`) return 403 for plain admins.

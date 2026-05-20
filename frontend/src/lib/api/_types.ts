/**
 * Shared response envelope types for the API client modules.
 *
 * Private to `frontend/src/lib/api/` — the leading underscore signals that
 * external callers should import the typed response from the specific API
 * module (e.g. `crmAPI.getContacts(...)` returns `Promise<PaginatedResponse<Contact>>`),
 * not from this file directly.
 *
 * ## Why this exists
 *
 * Per R2-03 (typed API client investigation): the Rust backend returns
 * `Json<serde_json::Value>` from ~808 of ~850 route handlers (95%), so
 * OpenAPI codegen would emit `unknown` for every body. The leverage win
 * is hand-typing the canonical envelope on the TypeScript side and
 * replacing `Promise<{ data: X[]; meta: any }>` repeated 28 times in
 * `crm.ts` with `Promise<PaginatedResponse<X>>`.
 *
 * ## Field shape — evidence from `api/src/routes/crm.rs`
 *
 * Three handler shapes emit pagination meta. The full shape (used by
 * `list_contacts`, line 328-336; `list_deals`, line 1843-1851; etc.) is:
 *
 * ```rust
 * "meta": {
 *     "current_page": page,
 *     "per_page": per_page,
 *     "total": total.0,
 *     "total_pages": (total.0 as f64 / per_page as f64).ceil() as i64
 * }
 * ```
 *
 * Many "list-only" handlers (e.g. `list_lists` line 549, `list_tags` line
 * 660, `list_segments` line 752) emit a minimal `{ "total": N }`. One
 * outlier (`list_X` line 1706) emits `{ "total": 0, "per_page": N }`.
 *
 * So: `total` is universal; every other field is optional in practice.
 * Reflect that here — callers that grab `current_page` without a fallback
 * will hit a `number | undefined` narrowing requirement, which is the
 * correct outcome.
 *
 * The backend does NOT emit `last_page` (a Laravel idiom). Frontend code
 * that reads `response.meta.last_page` against a CRM endpoint is reading
 * `undefined` at runtime today — `meta: any` was hiding it. Surfacing
 * that as a TS error is the point of this refactor.
 */

export interface PaginationMeta {
	/** Total number of records across all pages. Always present. */
	total: number;
	/** Current page (1-indexed). Present on full-pagination endpoints. */
	current_page?: number;
	/** Records per page. Present on full-pagination endpoints. */
	per_page?: number;
	/** Total page count, derived from `total / per_page` on the backend. */
	total_pages?: number;
}

export interface PaginatedResponse<T> {
	data: T[];
	meta: PaginationMeta;
}

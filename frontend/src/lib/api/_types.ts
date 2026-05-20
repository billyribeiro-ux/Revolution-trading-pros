/**
 * Shared API envelope + JSON-value helpers — used by the typed-api-client
 * refactor (R2-03 / R4-B / R5-A / R6-A).
 *
 * ─────────────────────────────────────────────────────────────────────────
 * `PaginatedResponse<T>` replaces the ad-hoc `Promise<{ data: X[]; meta: any }>`
 * pattern that was repeated across the api-client modules. Field names match
 * the backend reality (see `api/src/routes/crm.rs` for evidence):
 *
 *     `current_page` + `per_page` + `total` + `total_pages?`
 *
 * NOT Laravel-style `last_page` — the Rust handlers in this repo emit
 * `total_pages`. Callers that previously read `meta.last_page` were silently
 * receiving `undefined` and need migrating to `meta.total_pages`.
 *
 * ─────────────────────────────────────────────────────────────────────────
 * `JsonValue` is the right shape for genuinely heterogeneous JSON columns
 * (settings, metadata, custom-field maps, criteria DSLs). Narrower than
 * `any` — callers must `typeof` / `Array.isArray()` before use — but doesn't
 * pretend to know a shape we don't actually know.
 */

export interface PaginationMeta {
	total: number;
	current_page?: number;
	per_page?: number;
	total_pages?: number;
}

export interface PaginatedResponse<T> {
	data: T[];
	meta: PaginationMeta;
}

export type JsonValue =
	| string
	| number
	| boolean
	| null
	| { [k: string]: JsonValue | undefined }
	| JsonValue[];

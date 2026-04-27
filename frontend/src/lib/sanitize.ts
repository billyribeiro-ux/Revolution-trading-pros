/**
 * FIX-2026-04-26 (audit 04-content-longform):
 * Convenience re-export of the canonical sanitization API at
 * `$lib/utils/sanitize.ts`. The audit instructions reference this module path,
 * so we expose the same exports here to avoid divergence between callers.
 *
 * Prefer importing from `$lib/utils/sanitize` for new code; this barrel is
 * provided so that existing `$lib/sanitize` imports keep working and remain
 * a single source of truth.
 */

export {
	sanitizeHtml,
	sanitizeBlogContent,
	sanitizeFormContent,
	sanitizePopupContent,
	sanitizeVideoOverlay,
	stripHtml,
	containsDangerousHtml,
	default
} from '$lib/utils/sanitize';
export type { SanitizeProfile } from '$lib/utils/sanitize';

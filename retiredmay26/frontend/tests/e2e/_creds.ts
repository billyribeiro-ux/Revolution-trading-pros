/// <reference types="node" />
/**
 * Shared E2E super-admin credentials
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * SECURITY: these credentials are NEVER baked into the repo. Previously the
 * e2e specs carried the real super-admin email/password as hardcoded string
 * fallbacks (committed-secret site flagged by FULL_REPO_AUDIT_2026-05-17,
 * P0-2). They are now sourced exclusively from the environment.
 *
 * There is intentionally NO literal fallback. If either var is unset we throw
 * loudly at import time so a run can never silently authenticate with a
 * baked-in credential — fail closed, not open.
 *
 * Set these in CI / your local shell env (see frontend/.env.example). Point
 * them at the seeded local super-admin: `api/scripts/seed-local-admin.sh`.
 */

function required(name: string): string {
	const value = process.env[name];
	if (!value || value.trim() === '') {
		throw new Error(
			`Missing required env var ${name}. The e2e suite refuses to run with a ` +
				`baked-in credential. Set ${name} (see frontend/.env.example) before ` +
				`running Playwright — e.g. point it at the seeded local super-admin.`
		);
	}
	return value;
}

export const SUPERADMIN_EMAIL: string = required('E2E_SUPERADMIN_EMAIL');
export const SUPERADMIN_PASSWORD: string = required('E2E_SUPERADMIN_PASSWORD');

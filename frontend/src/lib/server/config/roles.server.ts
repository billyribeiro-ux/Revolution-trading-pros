/**
 * Server-only role configuration
 * ═══════════════════════════════════════════════════════════════════════════
 * SECURITY: Uses $env/static/private — never exposed to client bundle.
 * VITE_ prefix would leak these email lists to the browser JS bundle.
 *
 * @security This file must NEVER be imported from client-side code.
 */

import { env } from '$env/dynamic/private';

function parseEmailList(envVar: string | undefined): readonly string[] {
	if (!envVar || typeof envVar !== 'string') return [];
	return Object.freeze(
		envVar
			.split(',')
			.map((e) => e.trim().toLowerCase())
			.filter((e) => e.length > 0 && e.includes('@'))
	);
}

export const SUPERADMIN_EMAILS_SERVER: readonly string[] = parseEmailList(
	env.SUPERADMIN_EMAILS ?? env.VITE_SUPERADMIN_EMAILS
);

export const DEVELOPER_EMAILS_SERVER: readonly string[] = parseEmailList(
	env.DEVELOPER_EMAILS ?? env.VITE_DEVELOPER_EMAILS
);

export function isSuperadminEmailServer(email: string | null | undefined): boolean {
	if (!email) return false;
	return SUPERADMIN_EMAILS_SERVER.includes(email.toLowerCase());
}

export function isDeveloperEmailServer(email: string | null | undefined): boolean {
	if (!email) return false;
	return DEVELOPER_EMAILS_SERVER.includes(email.toLowerCase());
}

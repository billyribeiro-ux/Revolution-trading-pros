/** Shared runtime/build boundary for pages requiring an authenticated session. */
export const AUTHENTICATED_PREFIXES = [
	'/dashboard',
	'/account',
	'/checkout',
	'/trading-room',
	'/admin',
	'/my'
] as const;
export function isAuthenticatedPath(pathname: string): boolean {
	return AUTHENTICATED_PREFIXES.some(
		(prefix) => pathname === prefix || pathname.startsWith(`${prefix}/`)
	);
}

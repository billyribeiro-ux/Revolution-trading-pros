/**
 * SvelteKit Generated Types Stub
 * This file provides temporary type definitions until .svelte-kit is generated
 *
 * @version December 2025 - SvelteKit 2.x compatible
 */

/**
 * PageServerLoad type for server-side load functions
 * Matches SvelteKit 2.x signature with proper event object
 */
export type PageServerLoad<
	OutputData extends Record<string, any> | void = Record<string, any> | void
> = (event: {
	url: URL;
	params: Record<string, string>;
	route: { id: string | null };
	setHeaders: (headers: Record<string, string>) => void;
	fetch: typeof fetch;
	cookies: any;
	locals: any;
	request: Request;
	platform: any;
	isDataRequest: boolean;
	isSubRequest: boolean;
}) => OutputData | Promise<OutputData>;

/**
 * PageLoad type for client-side load functions
 */
export type PageLoad<OutputData extends Record<string, any> | void = Record<string, any> | void> =
	(event: {
		url: URL;
		params: Record<string, string>;
		route: { id: string | null };
		data: any;
		fetch: typeof fetch;
	}) => OutputData | Promise<OutputData>;

export type LayoutServerLoad = PageServerLoad;
export type LayoutLoad = PageLoad;

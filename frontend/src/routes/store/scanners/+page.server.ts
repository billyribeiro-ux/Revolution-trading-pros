/**
 * Scanners Landing Page - Server Load
 * ═══════════════════════════════════════════════════════════════════════════
 * 
 * Server-side data loading for optimal SEO and performance
 * Google December 2025: Server-rendered content is indexed immediately
 * 
 * Svelte 5 / SvelteKit (Dec 2025):
 * - No ./$types imports needed
 * - Explicit typing for server load functions
 * 
 * @version 2.0.0
 * @author Revolution Trading Pros
 */

// ═══════════════════════════════════════════════════════════════════════════
// TYPES - Explicit typing (Svelte 5 pattern)
// ═══════════════════════════════════════════════════════════════════════════

interface ServerData {
	meta: {
		generatedAt: string;
		version: string;
		environment: string;
	};
	preconnect: string[];
	dnsPrefetch: string[];
}

export const load = async ({ setHeaders }: { setHeaders: (headers: Record<string, string>) => void }): Promise<ServerData> => {
	// ═══════════════════════════════════════════════════════════════════════════
	// CACHE HEADERS - Google Dec 2025 Best Practice
	// ═══════════════════════════════════════════════════════════════════════════
	
	// Cache for 1 hour, revalidate in background
	setHeaders({
		'Cache-Control': 'public, max-age=3600, stale-while-revalidate=86400',
		'CDN-Cache-Control': 'public, max-age=3600',
		'Vercel-CDN-Cache-Control': 'public, max-age=3600, stale-while-revalidate=86400'
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// SERVER-SIDE DATA
	// ═══════════════════════════════════════════════════════════════════════════

	// In production, this would fetch from database/API
	// For now, return static data that matches client-side
	const serverData = {
		// Page metadata for SSR
		meta: {
			generatedAt: new Date().toISOString(),
			version: '1.0.0',
			environment: process.env.NODE_ENV || 'development'
		},
		
		// Performance hints for browser
		preconnect: [
			'https://fonts.googleapis.com',
			'https://fonts.gstatic.com',
			'https://revolution-trading-pros-api.fly.dev'
		],
		
		// DNS prefetch for external resources
		dnsPrefetch: [
			'https://www.googletagmanager.com',
			'https://www.google-analytics.com'
		]
	};

	return serverData;
};

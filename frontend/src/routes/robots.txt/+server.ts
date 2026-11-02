/**
 * RFC 9309 Compliant robots.txt Endpoint (`IDX-07`, `IDX-08`)
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * - Explicitly allows `/_app/immutable/` and static assets so Googlebot WRS
 *   can render SvelteKit JS/CSS bundles.
 * - Disallows private/auth/admin/checkout/API prefixes from `NOINDEX_PREFIXES`.
 * - Separates AI training crawlers (`AI_TRAINING_BOTS` — Disallow: /) from
 *   AI search/citation bots (`AI_SEARCH_BOTS` — controlled by `AI_SEARCH_OPT_OUT`).
 * - Uses only valid RFC 9309 `User-agent` product tokens (no wildcards in User-agent).
 */

import type { RequestHandler } from '@sveltejs/kit';
import {
	PRODUCTION_ORIGIN,
	NOINDEX_PREFIXES,
	AI_TRAINING_BOTS,
	AI_SEARCH_BOTS,
	AI_SEARCH_OPT_OUT
} from '#lib/seo/config.js';

export const prerender = true;

function buildRobotsTxt(): string {
	const lines: string[] = [
		'# Revolution Trading Pros — robots.txt (RFC 9309)',
		`# Canonical Origin: ${PRODUCTION_ORIGIN}`,
		'',
		'# Default rules for all standard search crawlers (Googlebot, Bingbot, etc.)',
		'User-agent: *',
		'Allow: /',
		'Allow: /_app/immutable/',
		'Allow: /_app/version.json',
		...NOINDEX_PREFIXES.map((prefix) => `Disallow: ${prefix}`),
		''
	];

	// AI Search & Citation bots (OAI-SearchBot, ChatGPT-User, PerplexityBot, Claude-SearchBot, Claude-User)
	lines.push('# AI Search & Citation Crawlers');
	for (const bot of AI_SEARCH_BOTS) {
		lines.push(`User-agent: ${bot}`);
		if (AI_SEARCH_OPT_OUT) {
			lines.push('Disallow: /');
		} else {
			lines.push('Allow: /');
			lines.push('Allow: /_app/immutable/');
			for (const prefix of NOINDEX_PREFIXES) {
				lines.push(`Disallow: ${prefix}`);
			}
		}
		lines.push('');
	}

	// AI Model-Training Crawlers (GPTBot, Google-Extended, ClaudeBot, CCBot, etc.)
	lines.push('# AI Model-Training Crawlers (Opt-Out)');
	for (const bot of AI_TRAINING_BOTS) {
		lines.push(`User-agent: ${bot}`);
		lines.push('Disallow: /');
		lines.push('');
	}

	lines.push(`Sitemap: ${PRODUCTION_ORIGIN}/sitemap.xml`);
	return `${lines.join('\n')}\n`;
}

export const GET: RequestHandler = async () => {
	const body = buildRobotsTxt();
	return new Response(body, {
		status: 200,
		headers: {
			'Content-Type': 'text/plain; charset=utf-8',
			'Cache-Control': 'public, max-age=3600, s-maxage=86400'
		}
	});
};

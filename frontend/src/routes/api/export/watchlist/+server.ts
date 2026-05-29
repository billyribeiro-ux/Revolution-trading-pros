/**
 * Watchlist Export API - CSV/JSON export for trade plans
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * GET /api/export/watchlist?room_slug=explosive-swings&format=csv
 * Exports the current week's trade plan as CSV or JSON
 *
 * R22-A: Deleted the 3-row NVDA/META/AMD mock fallback. A trader clicking
 *   "Export CSV" on a backend hiccup used to download a CSV with the same
 *   three hardcoded tickers regardless of which room they were in — easy to
 *   mistake for the real week's plan and act on phantom trades. Now: a
 *   backend non-2xx surfaces as 502 so the download fails loudly.
 *
 * @version 2.0.0 - ICT 11 Principal Engineer Grade
 */

import { error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

// CLAUDE.md hard rule — API_BASE_URL primary, BACKEND_URL fallback,
// localhost last. R21-A: restored full fallback chain (was: BACKEND_URL only).
const BACKEND_URL = env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

interface TradePlanEntry {
	ticker: string;
	bias: string;
	entry: string;
	target1: string;
	target2: string;
	target3: string;
	runner: string;
	stop: string;
	options_strike: string;
	options_exp: string;
	notes: string;
}

export const GET: RequestHandler = async ({ url, cookies }) => {
	const roomSlug = url.searchParams.get('room_slug') || 'explosive-swings';
	const format = url.searchParams.get('format') || 'csv';

	try {
		// Fetch trade plan from backend
		const headers: Record<string, string> = {
			'Content-Type': 'application/json',
			Accept: 'application/json'
		};

		// FIX-2026-04-26: comment-out, verify, delete in follow-up. Wrong cookie name — login proxy sets rtp_access_token, not session.
		// const session = cookies.get('session');
		const session = cookies.get('rtp_access_token');
		if (session) {
			headers['Cookie'] = `session=${session}`;
		}

		const response = await fetch(`${BACKEND_URL}/api/room-content/rooms/${roomSlug}/trade-plan`, {
			headers
		});

		if (!response.ok) {
			// R22-A: was: fall back to 3-row NVDA/META/AMD mock and proceed to
			// generate the CSV/JSON. Now: fail loudly so the trader doesn't
			// download phantom tickers and act on them.
			console.error(`[Export watchlist] Backend ${response.status} for room '${roomSlug}'`);
			error(502, `Unable to fetch trade plan for ${roomSlug}.`);
		}

		const data = await response.json();
		const entries: TradePlanEntry[] = data.data || [];
		const weekOf = data.week_of || new Date().toISOString().split('T')[0];

		if (format === 'json') {
			return new Response(JSON.stringify({ week_of: weekOf, entries }, null, 2), {
				headers: {
					'Content-Type': 'application/json',
					'Content-Disposition': `attachment; filename="watchlist-${roomSlug}-${weekOf}.json"`
				}
			});
		}

		// Generate CSV
		const csvHeader =
			'Ticker,Bias,Entry,Target 1,Target 2,Target 3,Runner,Stop,Options Strike,Options Exp,Notes';
		const csvRows = entries.map((e) =>
			[
				e.ticker,
				e.bias,
				e.entry,
				e.target1,
				e.target2,
				e.target3,
				e.runner,
				e.stop,
				e.options_strike,
				e.options_exp,
				`"${(e.notes || '').replace(/"/g, '""')}"`
			].join(',')
		);

		const csv = [csvHeader, ...csvRows].join('\n');

		return new Response(csv, {
			headers: {
				'Content-Type': 'text/csv',
				'Content-Disposition': `attachment; filename="watchlist-${roomSlug}-${weekOf}.csv"`
			}
		});
	} catch (err) {
		console.error('Export failed:', err);
		error(500, 'Failed to export watchlist');
	}
};

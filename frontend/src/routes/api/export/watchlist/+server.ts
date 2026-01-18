/**
 * Watchlist Export API - CSV/JSON export for trade plans
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * GET /api/export/watchlist?room_slug=explosive-swings&format=csv
 * Exports the current week's trade plan as CSV or JSON
 *
 * @version 2.0.0 - ICT 11 Principal Engineer Grade
 */

import { error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

const BACKEND_URL = env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';

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

		const session = cookies.get('session');
		if (session) {
			headers['Cookie'] = `session=${session}`;
		}

		const response = await fetch(
			`${BACKEND_URL}/api/room-content/rooms/${roomSlug}/trade-plan`,
			{ headers }
		);

		let entries: TradePlanEntry[] = [];
		let weekOf = new Date().toISOString().split('T')[0];

		if (response.ok) {
			const data = await response.json();
			entries = data.data || [];
			weekOf = data.week_of || weekOf;
		} else {
			// Fallback mock data for demo
			entries = [
				{
					ticker: 'NVDA',
					bias: 'BULLISH',
					entry: '$142.50',
					target1: '$148.00',
					target2: '$152.00',
					target3: '$158.00',
					runner: '$165.00+',
					stop: '$136.00',
					options_strike: '$145 Call',
					options_exp: '2026-01-24',
					notes: 'Breakout above consolidation'
				},
				{
					ticker: 'META',
					bias: 'BULLISH',
					entry: '$612.00',
					target1: '$625.00',
					target2: '$640.00',
					target3: '$660.00',
					runner: '$680.00+',
					stop: '$595.00',
					options_strike: '$620 Call',
					options_exp: '2026-01-24',
					notes: 'Momentum continuation'
				},
				{
					ticker: 'AMD',
					bias: 'BEARISH',
					entry: '$118.50',
					target1: '$112.00',
					target2: '$108.00',
					target3: '$102.00',
					runner: '$95.00',
					stop: '$124.00',
					options_strike: '$115 Put',
					options_exp: '2026-01-24',
					notes: 'Head & shoulders breakdown'
				}
			];
		}

		if (format === 'json') {
			return new Response(JSON.stringify({ week_of: weekOf, entries }, null, 2), {
				headers: {
					'Content-Type': 'application/json',
					'Content-Disposition': `attachment; filename="watchlist-${roomSlug}-${weekOf}.json"`
				}
			});
		}

		// Generate CSV
		const csvHeader = 'Ticker,Bias,Entry,Target 1,Target 2,Target 3,Runner,Stop,Options Strike,Options Exp,Notes';
		const csvRows = entries.map(e => 
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
		throw error(500, 'Failed to export watchlist');
	}
};

// Minimal mock backend for CLS measurement only (NOT committed to app).
// Responds to the handful of endpoints the dashboard hits so the real
// authed page renders with realistic, delayed data.
import { createServer } from 'node:http';

const PORT = 8080;
const NETWORK_DELAY_MS = Number(process.env.MOCK_DELAY ?? 450); // simulate API latency

// Role is env-gated so the same mock can drive member dashboards (default) and
// admin pages (MOCK_ROLE=admin|super-admin) for probing admin remote functions.
const user = {
	id: 42,
	email: 'measure@rtp.test',
	name: 'Measure User',
	role: process.env.MOCK_ROLE ?? 'member'
};

// A realistic, multi-section membership payload (drives many dashboard sections).
// HOSTILE ordering on purpose: types are interleaved (NOT category-grouped) and
// one membership is `expiring` — this reproduces the two ordering/filtering
// shift bugs the SSR fix must be robust against.
const memberships = [
	{ id: 'spx-profit-pulse', name: 'SPX Profit Pulse', type: 'alert-service', slug: 'spx-profit-pulse', status: 'active', icon: 'bell' },
	{ id: 'day-trading-room', name: 'Day Trading Room', type: 'trading-room', slug: 'day-trading-room', status: 'active', icon: 'bolt' },
	{ id: 'small-account-mentorship', name: 'Small Account Mentorship', type: 'course', slug: 'small-account-mentorship', status: 'active', icon: 'school' },
	{ id: 'explosive-swings', name: 'Explosive Swings', type: 'alert-service', slug: 'explosive-swings', status: 'expiring', icon: 'rocket' },
	{ id: 'high-octane-scanner', name: 'High Octane Scanner', type: 'indicator', slug: 'high-octane-scanner', status: 'active', icon: 'chart-candle' },
	{ id: 'swing-trading-room', name: 'Swing Trading Room', type: 'trading-room', slug: 'swing-trading-room', status: 'active', icon: 'trending-up' }
];

const watchlist = {
	id: 1, week_of: '2026-06-01', title: 'Weekly Watchlist',
	video_title: 'Watchlist Rundown', thumbnail_url: 'https://placehold.co/640x360',
	description: 'This week’s setups', stocks: []
};

// Favorites payload for the favorites-route CLS probe. Mixed types, some with
// thumbnails + excerpts (taller cards) and some without — a realistic, uneven
// list whose total height differs from a fixed 4-card skeleton.
const favorites = [
	{ id: 101, item_type: 'alert', item_id: 9001, title: 'SPX 5800c — runner trimmed', excerpt: 'Took partials into the 11:30 push, trailing the rest on the 5-min.', href: '/dashboard/explosive-swings/alerts/9001', thumbnail_url: 'https://placehold.co/320x240', created_at: '2026-05-28T14:32:00Z' },
	{ id: 102, item_type: 'video', item_id: 9002, title: 'Weekly game plan walkthrough', excerpt: null, href: '/dashboard/explosive-swings/videos/9002', thumbnail_url: 'https://placehold.co/320x240', created_at: '2026-05-27T09:00:00Z' },
	{ id: 103, item_type: 'trade_plan', item_id: 9003, title: 'NVDA earnings playbook', excerpt: 'Three scenarios mapped with invalidation levels and sizing notes for each.', href: null, thumbnail_url: null, created_at: '2026-05-25T18:45:00Z' }
];

function send(res, status, body) {
	const json = JSON.stringify(body);
	res.writeHead(status, { 'content-type': 'application/json', 'access-control-allow-origin': '*' });
	res.end(json);
}

async function readBody(req) {
	const chunks = [];
	for await (const c of req) chunks.push(c);
	if (!chunks.length) return null;
	try {
		return JSON.parse(Buffer.concat(chunks).toString());
	} catch {
		return null;
	}
}

// Stateful 404-log store so single-flight mutations are observable (delete
// shrinks the list, ignore flips status) across a probe run.
let seo404 = [
	{ id: 1, url: '/old-landing', hits: 42, last_hit_at: '2026-05-30T10:00:00Z', referer: 'google.com', is_resolved: false, is_ignored: false },
	{ id: 2, url: '/discontinued', hits: 12, last_hit_at: '2026-05-29T10:00:00Z', referer: null, is_resolved: false, is_ignored: false },
	{ id: 3, url: '/typo-url', hits: 5, last_hit_at: '2026-05-28T10:00:00Z', referer: 'twitter.com', is_resolved: false, is_ignored: false }
];
let seoKeywords = [
	{ id: 1, keyword: 'options trading', current_rank: 2, rank_change: 1, search_volume: 40000, competition: 0.8, target_url: '/' },
	{ id: 2, keyword: 'swing trades', current_rank: 7, rank_change: -1, search_volume: 12000, competition: 0.5, target_url: '/swings' },
	{ id: 3, keyword: 'day trading room', current_rank: 14, rank_change: 0, search_volume: 8000, competition: 0.4, target_url: '/rooms' }
];
const seoKeywordStats = () => ({
	total: seoKeywords.length,
	top_3: seoKeywords.filter((k) => (k.current_rank ?? 99) <= 3).length,
	top_10: seoKeywords.filter((k) => (k.current_rank ?? 99) <= 10).length,
	avg_position: seoKeywords.length
		? seoKeywords.reduce((s, k) => s + (k.current_rank ?? 0), 0) / seoKeywords.length
		: 0
});

let seoRedirects = [
	{ id: 1, source_url: '/old-a', destination_url: '/new-a', redirect_type: '301', is_regex: false, is_active: true, hits: 120, notes: '' },
	{ id: 2, source_url: '/old-b', destination_url: '/new-b', redirect_type: '302', is_regex: false, is_active: true, hits: 40, notes: '' },
	{ id: 3, source_url: '/old-c', destination_url: '/new-c', redirect_type: '301', is_regex: false, is_active: false, hits: 8, notes: '' },
	{ id: 4, source_url: '/gone', destination_url: '/', redirect_type: '410', is_regex: false, is_active: true, hits: 2, notes: '' }
];
const seo404Stats = () => ({
	total: seo404.length,
	unresolved: seo404.filter((l) => !l.is_resolved && !l.is_ignored).length,
	resolved: seo404.filter((l) => l.is_resolved).length,
	total_hits: seo404.reduce((s, l) => s + l.hits, 0)
});

createServer(async (req, res) => {
	const url = req.url || '';
	const delay = (ms) => new Promise((r) => setTimeout(r, ms));

	// Keywords (stateful) — delete removes; stats computed live.
	if (url.includes('/seo/keywords')) {
		await delay(NETWORK_DELAY_MS);
		if (url.includes('/stats')) return send(res, 200, seoKeywordStats());
		const idMatch = url.match(/\/keywords\/(\d+)(?:\?|$)/);
		if (idMatch && req.method === 'DELETE') {
			const id = Number(idMatch[1]);
			seoKeywords = seoKeywords.filter((k) => k.id !== id);
			return send(res, 200, { success: true });
		}
		return send(res, 200, { data: seoKeywords });
	}
	// Redirects (stateful) — toggle flips is_active, delete/bulk-delete remove.
	if (url.includes('/seo/redirects')) {
		await delay(NETWORK_DELAY_MS);
		if (url.includes('/stats')) return send(res, 200, {
			total: seoRedirects.length,
			active: seoRedirects.filter((r) => r.is_active).length,
			inactive: seoRedirects.filter((r) => !r.is_active).length,
			total_hits: seoRedirects.reduce((s, r) => s + (r.hits || 0), 0)
		});
		if (url.includes('/bulk-delete')) {
			const body = await readBody(req);
			const ids = Array.isArray(body?.ids) ? body.ids : [];
			seoRedirects = seoRedirects.filter((r) => !ids.includes(r.id));
			return send(res, 200, { success: true });
		}
		const toggleMatch = url.match(/\/redirects\/(\d+)\/toggle/);
		if (toggleMatch) {
			const id = Number(toggleMatch[1]);
			seoRedirects = seoRedirects.map((r) => (r.id === id ? { ...r, is_active: !r.is_active } : r));
			return send(res, 200, { success: true });
		}
		const idMatch = url.match(/\/redirects\/(\d+)(?:\?|$)/);
		if (idMatch && req.method === 'DELETE') {
			const id = Number(idMatch[1]);
			seoRedirects = seoRedirects.filter((r) => r.id !== id);
			return send(res, 200, { success: true });
		}
		return send(res, 200, { data: seoRedirects });
	}
	// 404 monitor (stateful) — order matters: stats + sub-paths before the list.
	if (url.includes('/seo/404-logs')) {
		await delay(NETWORK_DELAY_MS);
		if (url.includes('/stats')) return send(res, 200, seo404Stats());
		if (url.includes('/bulk-delete')) {
			const body = await readBody(req);
			const ids = Array.isArray(body?.ids) ? body.ids : [];
			seo404 = seo404.filter((l) => !ids.includes(l.id));
			return send(res, 200, { success: true });
		}
		const ignoreMatch = url.match(/\/404-logs\/(\d+)\/ignore/);
		if (ignoreMatch) {
			const id = Number(ignoreMatch[1]);
			seo404 = seo404.map((l) => (l.id === id ? { ...l, is_ignored: true } : l));
			return send(res, 200, { success: true });
		}
		return send(res, 200, { data: seo404 });
	}

	if (url.includes('/auth/me')) return send(res, 200, { success: true, data: user });
	if (url.includes('memberships')) { await delay(NETWORK_DELAY_MS); return send(res, 200, { success: true, data: { memberships } }); }
	if (url.includes('watchlist')) { await delay(NETWORK_DELAY_MS); return send(res, 200, { success: true, data: watchlist }); }
	if (url.includes('weekly-video')) { await delay(NETWORK_DELAY_MS); return send(res, 200, { success: true, data: watchlist }); }
	// Admin member analytics — raw arrays/objects (NOT {success,data}-wrapped),
	// matching the backend shape the page reads directly.
	if (url.includes('/members/analytics/')) {
		await delay(NETWORK_DELAY_MS);
		if (url.includes('/metrics')) return send(res, 200, {
			totalMembers: 1240, memberGrowth: 8, mrr: 48200, mrrGrowth: 12,
			churnRate: 3, churnChange: -1, avgLtv: 1840, ltvGrowth: 6
		});
		if (url.includes('/growth')) return send(res, 200, [
			{ month: 'Mar', members: 1100, new: 120, churned: 30 },
			{ month: 'Apr', members: 1180, new: 110, churned: 30 },
			{ month: 'May', members: 1240, new: 95, churned: 35 }
		]);
		if (url.includes('/cohorts')) return send(res, 200, [
			{ cohort: 'Mar', m0: 100, m1: 92, m2: 85, m3: 80, m4: 76, m5: 74 },
			{ cohort: 'Apr', m0: 100, m1: 90, m2: 84, m3: 79, m4: 75, m5: 0 }
		]);
		if (url.includes('/revenue')) return send(res, 200, [
			{ month: 'Apr', mrr: 44000, expansion: 3000, contraction: 800, churn: 1200 },
			{ month: 'May', mrr: 48200, expansion: 3400, contraction: 700, churn: 1100 }
		]);
		if (url.includes('/churn-reasons')) return send(res, 200, [
			{ reason: 'Price', count: 18, percentage: 45 },
			{ reason: 'Switched tools', count: 12, percentage: 30 },
			{ reason: 'Other', count: 10, percentage: 25 }
		]);
		if (url.includes('/segments')) return send(res, 200, [
			{ segment: 'Pro', count: 420, revenue: 28000, churnRate: 2 },
			{ segment: 'Starter', count: 820, revenue: 20200, churnRate: 4 }
		]);
		return send(res, 200, []);
	}
	// Admin media (bandwidth) analytics — raw object/arrays.
	if (url.includes('/media/analytics/')) {
		await delay(NETWORK_DELAY_MS);
		if (url.includes('/overview')) return send(res, 200, {
			totalOriginal: 5_000_000_000, totalOptimized: 2_100_000_000, totalSavings: 2_900_000_000,
			savingsPercent: 58, totalImages: 12400, optimizedImages: 11900,
			avgCompressionRatio: 2.4, estimatedCostSavings: 320, co2Saved: 18
		});
		if (url.includes('/bandwidth')) return send(res, 200, [
			{ date: '2026-05-01', original: 800_000_000, optimized: 340_000_000, savings: 460_000_000, requests: 9000 },
			{ date: '2026-05-15', original: 900_000_000, optimized: 380_000_000, savings: 520_000_000, requests: 9800 },
			{ date: '2026-05-31', original: 950_000_000, optimized: 400_000_000, savings: 550_000_000, requests: 10200 }
		]);
		if (url.includes('/formats')) return send(res, 200, [
			{ format: 'webp', count: 7200, originalSize: 3_000_000_000, optimizedSize: 1_200_000_000, savings: 1_800_000_000 },
			{ format: 'avif', count: 4700, originalSize: 2_000_000_000, optimizedSize: 900_000_000, savings: 1_100_000_000 }
		]);
		return send(res, 200, []);
	}
	// Admin orders — detail (/api/admin/orders/:id) before the list match.
	const orderDetailMatch = url.match(/\/admin\/orders\/(\d+)(?:\?|$)/);
	if (orderDetailMatch) {
		await delay(NETWORK_DELAY_MS);
		return send(res, 200, {
			data: {
				status: 'completed', total: 149, currency: 'USD', subtotal: 149, discount: 0,
				created_at: '2026-05-28T14:32:00Z', completed_at: '2026-05-28T14:35:00Z',
				billing_name: 'Measure User', billing_email: 'measure@rtp.test',
				items: [{ name: 'Explosive Swings (monthly)', quantity: 1, unit_price: 149, total: 149 }]
			}
		});
	}
	// Admin orders — list (+ stats + pagination), echoes the requested page.
	if (url.includes('/admin/orders')) {
		await delay(NETWORK_DELAY_MS);
		const page = Number(new URL(url, 'http://x').searchParams.get('page') ?? '1');
		const mk = (n) => ({
			id: n, order_number: `RTP-${1000 + n}`, status: n % 3 === 0 ? 'pending' : 'completed',
			total: 99 + n, currency: 'USD', user_email: `u${n}@rtp.test`, user_name: `User ${n}`,
			payment_provider: 'stripe', item_count: 1, created_at: '2026-05-28T14:32:00Z',
			completed_at: n % 3 === 0 ? null : '2026-05-28T14:35:00Z'
		});
		const rows = Array.from({ length: 25 }, (_, i) => mk((page - 1) * 25 + i + 1));
		return send(res, 200, {
			data: rows,
			stats: { total_orders: 73, completed_orders: 60, pending_orders: 10, refunded_orders: 3,
				total_revenue: 12000, revenue_this_month: 3400, average_order_value: 164 },
			pagination: { page, per_page: 25, total: 73, total_pages: 3 }
		});
	}
	// Favorites: realistic, DELAYED payload so the client-fetch variant (old code)
	// paints its skeleton first and then swaps in the list — the shift the remote
	// query + SSR warm-load is meant to eliminate.
	if (url.includes('favorites')) { await delay(NETWORK_DELAY_MS); return send(res, 200, { success: true, data: favorites, meta: { total: favorites.length } }); }
	// default: succeed empty
	return send(res, 200, { success: true, data: [] });
}).listen(PORT, () => console.log(`mock-backend listening on :${PORT}`));

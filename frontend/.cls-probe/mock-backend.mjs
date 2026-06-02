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

createServer(async (req, res) => {
	const url = req.url || '';
	const delay = (ms) => new Promise((r) => setTimeout(r, ms));

	if (url.includes('/auth/me')) return send(res, 200, { success: true, data: user });
	if (url.includes('memberships')) { await delay(NETWORK_DELAY_MS); return send(res, 200, { success: true, data: { memberships } }); }
	if (url.includes('watchlist')) { await delay(NETWORK_DELAY_MS); return send(res, 200, { success: true, data: watchlist }); }
	if (url.includes('weekly-video')) { await delay(NETWORK_DELAY_MS); return send(res, 200, { success: true, data: watchlist }); }
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

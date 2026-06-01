// Minimal mock backend for CLS measurement only (NOT committed to app).
// Responds to the handful of endpoints the dashboard hits so the real
// authed page renders with realistic, delayed data.
import { createServer } from 'node:http';

const PORT = 8080;
const NETWORK_DELAY_MS = Number(process.env.MOCK_DELAY ?? 450); // simulate API latency

const user = { id: 42, email: 'measure@rtp.test', name: 'Measure User', role: 'member' };

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
	// default: succeed empty
	return send(res, 200, { success: true, data: [] });
}).listen(PORT, () => console.log(`mock-backend listening on :${PORT}`));

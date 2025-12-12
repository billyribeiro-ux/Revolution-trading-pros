/**
 * Tags API Endpoint
 *
 * Returns available blog post tags.
 *
 * @version 1.0.0 - December 2025
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

// Predefined tags matching the blog categories
const mockTags = [
	{ id: 'options', name: 'Options', slug: 'options', count: 15 },
	{ id: 'day-trading', name: 'Day Trading', slug: 'day-trading', count: 23 },
	{ id: 'swing-trading', name: 'Swing Trading', slug: 'swing-trading', count: 18 },
	{ id: 'technical-analysis', name: 'Technical Analysis', slug: 'technical-analysis', count: 31 },
	{ id: 'risk-management', name: 'Risk Management', slug: 'risk-management', count: 12 },
	{ id: 'psychology', name: 'Psychology', slug: 'psychology', count: 9 },
	{ id: 'beginner', name: 'Beginner', slug: 'beginner', count: 27 },
	{ id: 'advanced', name: 'Advanced', slug: 'advanced', count: 14 },
	{ id: 'spx', name: 'SPX', slug: 'spx', count: 8 },
	{ id: 'futures', name: 'Futures', slug: 'futures', count: 11 },
	{ id: 'forex', name: 'Forex', slug: 'forex', count: 7 },
	{ id: 'crypto', name: 'Crypto', slug: 'crypto', count: 5 },
	{ id: 'stocks', name: 'Stocks', slug: 'stocks', count: 21 },
	{ id: 'earnings', name: 'Earnings', slug: 'earnings', count: 6 },
	{ id: 'market-analysis', name: 'Market Analysis', slug: 'market-analysis', count: 19 },
	{ id: 'education', name: 'Education', slug: 'education', count: 16 },
	{ id: 'strategy', name: 'Strategy', slug: 'strategy', count: 25 },
	{ id: 'small-accounts', name: 'Small Accounts', slug: 'small-accounts', count: 10 }
];

export const GET: RequestHandler = async () => {
	return json({
		success: true,
		data: mockTags
	});
};

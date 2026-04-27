/**
 * Single source of truth for the blog's predefined category set.
 *
 * Was duplicated 3× across `routes/admin/blog/{,create/,edit/[id]/}+page.svelte`
 * with color drift between copies (Technical Analysis was `#E6B800` in two
 * files and `#6366f1` in the third; Psychology was `#B38F00` vs `#8b5cf6`).
 * The list+create pages won; their values are canonical here.
 *
 * The same slugs are seeded into the `categories` table by migration
 * 045_blog_post_metadata_and_categories.sql, so the backend's slug-resolution
 * logic in api/src/routes/posts.rs::sync_post_categories will find them.
 */
export interface BlogCategory {
	id: string;
	name: string;
	color: string;
}

export const predefinedCategories: BlogCategory[] = [
	{ id: 'market-analysis', name: 'Market Analysis', color: '#3b82f6' },
	{ id: 'trading-strategies', name: 'Trading Strategies', color: '#10b981' },
	{ id: 'risk-management', name: 'Risk Management', color: '#ef4444' },
	{ id: 'options-trading', name: 'Options Trading', color: '#f59e0b' },
	{ id: 'technical-analysis', name: 'Technical Analysis', color: '#E6B800' },
	{ id: 'fundamental-analysis', name: 'Fundamental Analysis', color: '#ec4899' },
	{ id: 'psychology', name: 'Psychology', color: '#B38F00' },
	{ id: 'education', name: 'Education', color: '#14b8a6' },
	{ id: 'news', name: 'News & Updates', color: '#06b6d4' },
	{ id: 'earnings', name: 'Earnings', color: '#f97316' },
	{ id: 'stocks', name: 'Stocks', color: '#84cc16' },
	{ id: 'futures', name: 'Futures', color: '#22c55e' },
	{ id: 'forex', name: 'Forex', color: '#0ea5e9' },
	{ id: 'crypto', name: 'Crypto', color: '#a855f7' },
	{ id: 'small-accounts', name: 'Small Accounts', color: '#eab308' },
	{ id: 'day-trading', name: 'Day Trading', color: '#d946ef' },
	{ id: 'swing-trading', name: 'Swing Trading', color: '#64748b' },
	{ id: 'beginners', name: 'Beginners Guide', color: '#fb7185' }
];

export function getPredefinedCategoryById(id: string): BlogCategory | undefined {
	return predefinedCategories.find((c) => c.id === id);
}

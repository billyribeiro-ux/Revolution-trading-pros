/**
 * Resources Components
 * Apple Principal Engineer ICT 7 Grade - February 2026
 *
 * Export all resource-related components for easy importing:
 * - ResourceCard: Individual resource display
 * - ResourceGrid: Grid layout with filtering and pagination
 * - ResourceViewer: Modal for preview and download
 * - FavoriteButton: Toggle favorite state for resources
 * - RecentlyAccessed: Display user's recently accessed resources
 * - StockListViewer: Display stock/ETF watchlists
 * - ResourceAnalytics: Admin analytics dashboard
 */

// Core Components
export { default as ResourceCard } from './ResourceCard.svelte';
export { default as ResourceGrid } from './ResourceGrid.svelte';
export { default as ResourceViewer } from './ResourceViewer.svelte';

// ICT 7 NEW: User Engagement Components
export { default as FavoriteButton } from './FavoriteButton.svelte';
export { default as RecentlyAccessed } from './RecentlyAccessed.svelte';

// ICT 7 NEW: Stock/ETF Lists
export { default as StockListViewer } from './StockListViewer.svelte';

// ICT 7 NEW: Admin Analytics
export { default as ResourceAnalytics } from './ResourceAnalytics.svelte';

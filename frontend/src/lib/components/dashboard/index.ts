/**
 * Dashboard Components - Simpler Trading Style
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Svelte 5 components for the member dashboard.
 * All components use scoped CSS matching Simpler Trading's design.
 *
 * @version 1.0.0
 */

// Layout Components
export { default as DashboardSidebar } from './DashboardSidebar.svelte';
export { default as DashboardHeader } from './DashboardHeader.svelte';
export { default as DashboardBreadcrumbs } from './DashboardBreadcrumbs.svelte';
export { default as MobileToggle } from './MobileToggle.svelte';

// Card Components
export { default as MembershipCard } from './MembershipCard.svelte';
export { default as ArticleCard } from './ArticleCard.svelte';

// Section Components
export { default as WeeklyWatchlistSection } from './WeeklyWatchlistSection.svelte';
export { default as SectionTitle } from './SectionTitle.svelte';

// Grid Components (existing)
export { default as DashboardGrid } from './DashboardGrid.svelte';
export { default as WidgetCard } from './WidgetCard.svelte';

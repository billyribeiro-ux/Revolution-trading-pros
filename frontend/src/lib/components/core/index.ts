/**
 * ═══════════════════════════════════════════════════════════════════════════════
 * CORE COMPONENTS BARREL EXPORT - Svelte 5 / SvelteKit (Nov/Dec 2025)
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * PURPOSE:
 * This file provides a centralized export point for all core dashboard components.
 * Using barrel exports enables cleaner imports throughout the application:
 *
 * INSTEAD OF:
 *   import MembershipCard from '$lib/components/core/MembershipCard.svelte';
 *   import ToolCard from '$lib/components/core/ToolCard.svelte';
 *
 * YOU CAN:
 *   import { MembershipCard, ToolCard } from '$lib/components/core';
 *
 * BENEFITS:
 * - Cleaner import statements
 * - Single source of truth for component exports
 * - Easy to add/remove components from the public API
 * - Better tree-shaking support in bundlers
 *
 * @version 3.0.0 - Svelte 5 Runes Compatible
 * @updated December 2025
 * ═══════════════════════════════════════════════════════════════════════════════
 */

// ═══════════════════════════════════════════════════════════════════════════════
// SVELTE COMPONENT EXPORTS
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * MembershipCard - Displays a single membership/subscription card
 *
 * Features:
 * - Icon with circular background
 * - Membership name
 * - Dashboard and Access action buttons
 * - Variant styling (options, foundation, ww, default)
 *
 * @example
 * <MembershipCard
 *   membership={membershipData}
 *   variant="options"
 *   showActions={true}
 * />
 */
export { default as MembershipCard } from './MembershipCard.svelte';

/**
 * MembershipCardsGrid - Grid container for multiple membership cards
 *
 * Features:
 * - Responsive Bootstrap-style grid layout
 * - Loading and error states
 * - Empty state handling
 *
 * @example
 * <MembershipCardsGrid
 *   memberships={membershipsData}
 *   isLoading={loading}
 * />
 */
export { default as MembershipCardsGrid } from './MembershipCardsGrid.svelte';

/**
 * TradingRoomDropdown - "Enter a Trading Room" dropdown button
 *
 * Features:
 * - Orange styled toggle button
 * - Flyout menu with room list
 * - Click-outside close behavior
 * - SSO link handling
 *
 * @example
 * <TradingRoomDropdown
 *   rooms={tradingRooms}
 *   isOpen={dropdownOpen}
 *   onToggle={() => dropdownOpen = !dropdownOpen}
 * />
 */
export { default as TradingRoomDropdown } from './TradingRoomDropdown.svelte';

/**
 * DashboardHeader - Page header with title and actions
 *
 * Features:
 * - Page title (h1)
 * - Trading Room Rules link
 * - Trading Room dropdown integration
 *
 * @example
 * <DashboardHeader
 *   title="Member Dashboard"
 *   tradingRooms={rooms}
 *   showRulesLink={true}
 * />
 */
export { default as DashboardHeader } from './DashboardHeader.svelte';

/**
 * WeeklyWatchlistSection - Featured weekly watchlist content
 *
 * Features:
 * - Two-column responsive layout
 * - Video thumbnail with "Watch Now" CTA
 * - Loading state with skeleton
 *
 * @example
 * <WeeklyWatchlistSection
 *   watchlist={watchlistData}
 *   isLoading={false}
 * />
 */
export { default as WeeklyWatchlistSection } from './WeeklyWatchlistSection.svelte';

/**
 * ToolCard - Simple tool/utility card
 *
 * Features:
 * - Orange-themed icon
 * - Single "Dashboard" action button
 * - Used for Weekly Watchlist, Scanner, etc.
 *
 * @example
 * <ToolCard
 *   name="Weekly Watchlist"
 *   slug="ww"
 *   href="/dashboard/ww/"
 * />
 */
export { default as ToolCard } from './ToolCard.svelte';

/**
 * DashboardIcon - SVG Icon Component with Font Fallback
 *
 * Features:
 * - Maps st-icon-* classes to Tabler SVG icons
 * - Automatic fallback to font icons if mapping doesn't exist
 * - Tree-shakeable - only imported icons are bundled
 * - Crisp SVG scaling at any size
 *
 * @example
 * <DashboardIcon name="home" size={32} />
 * <DashboardIcon name="mastering-the-trade" />
 * <DashboardIcon name="st-icon-courses" />  <!-- Legacy format works too -->
 */
export { default as DashboardIcon } from './DashboardIcon.svelte';

// ═══════════════════════════════════════════════════════════════════════════════
// TYPE EXPORTS
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Re-export all types for external use
 * Allows consumers to import types alongside components:
 *
 * import { MembershipCard, type Membership } from '$lib/components/core';
 */
export type {
	// Core data types
	Membership,
	MembershipType,
	CardVariant,
	TradingRoom,
	RoomSchedule,
	WeeklyWatchlist,
	UserProfile,
	NavItem,
	NavCategory,
	MembershipsApiResponse,

	// Component prop types
	MembershipCardProps,
	TradingRoomDropdownProps,
	WeeklyWatchlistSectionProps,
	DashboardHeaderProps,
	ToolCardProps,

	// Event types
	MembershipActionEvent,
	TradingRoomSelectEvent
} from './types';

// ═══════════════════════════════════════════════════════════════════════════════
// UTILITY FUNCTION EXPORTS
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Re-export utility functions for use outside components
 */
export {
	getCardVariant,
	getActionLabel,
	shouldOpenNewTab
} from './types';

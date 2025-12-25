/**
 * ═══════════════════════════════════════════════════════════════════════════════
 * CORE DASHBOARD TYPE DEFINITIONS - Svelte 5 / SvelteKit (Nov/Dec 2025)
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Principal Engineer ICT 11+ Pattern:
 * - All types are strictly defined to ensure type safety across the dashboard
 * - These interfaces mirror the API response structure from the backend
 * - Used for data-driven rendering of memberships, trading rooms, and tools
 *
 * @version 3.0.0 - Svelte 5 Runes Compatible
 * @updated December 2025
 * ═══════════════════════════════════════════════════════════════════════════════
 */

// ═══════════════════════════════════════════════════════════════════════════════
// MEMBERSHIP TYPES - Core data structures for user subscriptions
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Represents a single user membership (subscription to a trading room, course, etc.)
 *
 * @property id - Unique identifier from the database
 * @property name - Display name (e.g., "Mastering the Trade")
 * @property slug - URL-safe identifier (e.g., "mastering-the-trade")
 * @property type - Category of membership for conditional rendering
 * @property icon - CSS class for the custom icon font (st-icon-*)
 * @property accessUrl - Direct URL to the trading room or content
 * @property roomLabel - Custom label for trading room access button
 * @property dashboardUrl - URL to the membership's dashboard page
 * @property isActive - Whether the subscription is currently active
 * @property expiresAt - ISO date string for subscription expiry (optional)
 */
export interface Membership {
	id: string | number;
	name: string;
	slug: string;
	type: MembershipType;
	icon?: string;
	accessUrl?: string;
	roomLabel?: string;
	dashboardUrl?: string;
	isActive: boolean;
	expiresAt?: string;
}

/**
 * Enum-like type for membership categories
 * Used to determine card styling and action button behavior
 */
export type MembershipType =
	| 'trading-room'      // Live trading rooms with SSO access
	| 'alert-service'     // Alert notification services
	| 'course'            // Educational courses and classes
	| 'indicator'         // Trading indicators and tools
	| 'premium-report'    // Premium research reports
	| 'newsletter'        // Newsletter subscriptions
	| 'tool';             // General tools (watchlist, scanner, etc.)

/**
 * Card style variants for visual differentiation
 * Maps to CSS classes for background colors and styling
 */
export type CardVariant =
	| 'options'      // Blue gradient - Trading rooms
	| 'foundation'   // Green gradient - Courses/foundation
	| 'ww'           // Orange gradient - Weekly watchlist
	| 'default';     // Neutral - Default fallback

// ═══════════════════════════════════════════════════════════════════════════════
// TRADING ROOM TYPES - Live trading room access and SSO
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Trading room with Single Sign-On (SSO) access
 *
 * @property id - Unique room identifier
 * @property name - Display name
 * @property slug - URL-safe identifier
 * @property ssoUrl - JWT-authenticated SSO URL for direct room access
 * @property roomLabel - Custom button label (e.g., "Mastering The Trade Room")
 * @property icon - Custom icon class
 * @property isLive - Whether the room is currently live
 * @property schedule - Room trading hours
 */
export interface TradingRoom {
	id: string | number;
	name: string;
	slug: string;
	ssoUrl: string;
	roomLabel?: string;
	icon?: string;
	isLive?: boolean;
	schedule?: RoomSchedule;
}

/**
 * Trading room schedule for live session times
 */
export interface RoomSchedule {
	timezone: string;
	days: string[];
	startTime: string;
	endTime: string;
}

// ═══════════════════════════════════════════════════════════════════════════════
// WEEKLY WATCHLIST TYPES - Featured content section
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Weekly watchlist featured content data
 *
 * @property title - Section heading
 * @property subtitle - Author/presenter name
 * @property description - Week description (e.g., "Week of December 22, 2025")
 * @property imageUrl - Thumbnail image URL
 * @property videoUrl - Link to the watchlist video
 * @property publishedAt - Publication date
 */
export interface WeeklyWatchlist {
	id: string | number;
	title: string;
	subtitle?: string;
	description: string;
	imageUrl: string;
	videoUrl: string;
	publishedAt: string;
}

// ═══════════════════════════════════════════════════════════════════════════════
// USER PROFILE TYPES - Authentication and profile data
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * User profile data for sidebar and header display
 *
 * @property id - User ID
 * @property name - Display name
 * @property email - Email address
 * @property avatar - Gravatar or custom avatar URL
 * @property memberships - Array of active memberships
 */
export interface UserProfile {
	id: string | number;
	name: string;
	email: string;
	avatar?: string;
	memberships: Membership[];
}

// ═══════════════════════════════════════════════════════════════════════════════
// NAVIGATION TYPES - Sidebar and menu structures
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Navigation item for sidebar menus
 *
 * @property id - Unique identifier
 * @property label - Display text
 * @property href - Navigation URL
 * @property icon - Icon class name
 * @property isActive - Whether this item is currently selected
 * @property isExternal - Opens in new tab if true
 * @property children - Nested submenu items
 */
export interface NavItem {
	id: string;
	label: string;
	href: string;
	icon?: string;
	isActive?: boolean;
	isExternal?: boolean;
	children?: NavItem[];
}

/**
 * Navigation category for grouped menu sections
 */
export interface NavCategory {
	id: string;
	label: string;
	items: NavItem[];
}

// ═══════════════════════════════════════════════════════════════════════════════
// API RESPONSE TYPES - Backend data structures
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Full user memberships API response
 * Matches the getUserMemberships() API return type
 */
export interface MembershipsApiResponse {
	memberships: Membership[];
	tradingRooms: TradingRoom[];
	courses: Membership[];
	indicators: Membership[];
	premiumReports: Membership[];
	weeklyWatchlist: WeeklyWatchlist[];
	alertServices: Membership[];
}

// ═══════════════════════════════════════════════════════════════════════════════
// COMPONENT PROP TYPES - Svelte 5 $props() interfaces
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Props for MembershipCard component
 * Using Svelte 5 $props() rune pattern
 */
export interface MembershipCardProps {
	membership: Membership;
	variant?: CardVariant;
	showActions?: boolean;
	onDashboardClick?: (membership: Membership) => void;
	onAccessClick?: (membership: Membership) => void;
}

/**
 * Props for TradingRoomDropdown component
 */
export interface TradingRoomDropdownProps {
	rooms: TradingRoom[];
	isOpen?: boolean;
	onToggle?: () => void;
	onSelect?: (room: TradingRoom) => void;
}

/**
 * Props for WeeklyWatchlistSection component
 */
export interface WeeklyWatchlistSectionProps {
	watchlist?: WeeklyWatchlist;
	isLoading?: boolean;
}

/**
 * Props for DashboardHeader component
 */
export interface DashboardHeaderProps {
	title: string;
	tradingRooms?: TradingRoom[];
	showRulesLink?: boolean;
}

/**
 * Props for ToolCard component
 */
export interface ToolCardProps {
	name: string;
	slug: string;
	icon?: string;
	href: string;
}

// ═══════════════════════════════════════════════════════════════════════════════
// EVENT TYPES - Svelte 5 custom event handlers
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Custom event detail for membership card actions
 */
export interface MembershipActionEvent {
	membership: Membership;
	action: 'dashboard' | 'access' | 'download';
}

/**
 * Custom event detail for trading room selection
 */
export interface TradingRoomSelectEvent {
	room: TradingRoom;
}

// ═══════════════════════════════════════════════════════════════════════════════
// UTILITY TYPES - Helper functions and mappings
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Map membership type to card variant for styling
 */
export function getCardVariant(type: MembershipType): CardVariant {
	switch (type) {
		case 'trading-room':
			return 'options';
		case 'course':
		case 'indicator':
			return 'foundation';
		case 'tool':
			return 'ww';
		default:
			return 'default';
	}
}

/**
 * Get action button label based on membership type
 */
export function getActionLabel(type: MembershipType): string {
	switch (type) {
		case 'trading-room':
			return 'Trading Room';
		case 'alert-service':
			return 'View Alerts';
		case 'course':
			return 'View Course';
		case 'indicator':
			return 'Download';
		default:
			return 'Access';
	}
}

/**
 * Check if action should open in new tab
 */
export function shouldOpenNewTab(type: MembershipType): boolean {
	return type === 'trading-room';
}

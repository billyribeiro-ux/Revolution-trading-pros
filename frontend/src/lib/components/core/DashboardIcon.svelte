<!--
═══════════════════════════════════════════════════════════════════════════════════
DASHBOARD ICON COMPONENT - Apple Principal Engineer ICT 11+ Standard
═══════════════════════════════════════════════════════════════════════════════════

PURPOSE:
Renders trading dashboard icons using Tabler Icons SVG library as the primary
source, with intelligent fallback to the legacy st-icon font classes.

This component solves the "missing icon font" problem by:
1. Mapping each st-icon-* class to an equivalent Tabler icon
2. Rendering crisp SVG icons that scale perfectly
3. Providing fallback to font icons if the mapping doesn't exist

SVELTE 5 PATTERNS:
- $props() with type-safe icon name mapping
- $derived() for computed icon selection
- Dynamic component rendering with svelte:component

USAGE:
<DashboardIcon name="home" size={32} class="text-white" />
<DashboardIcon name="mastering-the-trade" size={24} />
<DashboardIcon name="st-icon-courses" />  <!-- Legacy format also works -->

@version 3.0.0 - Svelte 5 Runes
@updated December 2025
═══════════════════════════════════════════════════════════════════════════════════
-->
<script lang="ts">
	/**
	 * ─────────────────────────────────────────────────────────────────────────────
	 * TABLER ICON IMPORTS
	 * ─────────────────────────────────────────────────────────────────────────────
	 * Import all icons we'll use in the dashboard.
	 * These are tree-shakeable - only imported icons are bundled.
	 */
	import IconHome from '@tabler/icons-svelte/icons/home';
	import IconHomeFilled from '@tabler/icons-svelte/icons/home-filled';
	import IconDashboard from '@tabler/icons-svelte/icons/dashboard';
	import IconBook from '@tabler/icons-svelte/icons/book';
	import IconBooks from '@tabler/icons-svelte/icons/books';
	import IconSchool from '@tabler/icons-svelte/icons/school';
	import IconChartCandle from '@tabler/icons-svelte/icons/chart-candle';
	import IconChartLine from '@tabler/icons-svelte/icons/chart-line';
	import IconChartBar from '@tabler/icons-svelte/icons/chart-bar';
	import IconChartPie from '@tabler/icons-svelte/icons/chart-pie';
	import IconTrendingUp from '@tabler/icons-svelte/icons/trending-up';
	import IconTrendingDown from '@tabler/icons-svelte/icons/trending-down';
	import IconCoin from '@tabler/icons-svelte/icons/coin';
	import IconCurrencyDollar from '@tabler/icons-svelte/icons/currency-dollar';
	import IconWallet from '@tabler/icons-svelte/icons/wallet';
	import IconReportAnalytics from '@tabler/icons-svelte/icons/report-analytics';
	import IconReportMoney from '@tabler/icons-svelte/icons/report-money';
	import IconListCheck from '@tabler/icons-svelte/icons/list-check';
	import IconListDetails from '@tabler/icons-svelte/icons/list-details';
	import IconCalendarEvent from '@tabler/icons-svelte/icons/calendar-event';
	import IconCalendarWeek from '@tabler/icons-svelte/icons/calendar-week';
	import IconSettings from '@tabler/icons-svelte/icons/settings';
	import IconSettingsFilled from '@tabler/icons-svelte/icons/settings-filled';
	import IconHelp from '@tabler/icons-svelte/icons/help';
	import IconHelpCircle from '@tabler/icons-svelte/icons/help-circle';
	import IconMessage from '@tabler/icons-svelte/icons/message';
	import IconMessages from '@tabler/icons-svelte/icons/messages';
	import IconVideo from '@tabler/icons-svelte/icons/video';
	import IconPlayerPlay from '@tabler/icons-svelte/icons/player-play';
	import IconPlayerPlayFilled from '@tabler/icons-svelte/icons/player-play-filled';
	import IconUsers from '@tabler/icons-svelte/icons/users';
	import IconUserCircle from '@tabler/icons-svelte/icons/user-circle';
	import IconBriefcase from '@tabler/icons-svelte/icons/briefcase';
	import IconBuildingStore from '@tabler/icons-svelte/icons/building-store';
	import IconArchive from '@tabler/icons-svelte/icons/archive';
	import IconHistory from '@tabler/icons-svelte/icons/history';
	import IconStar from '@tabler/icons-svelte/icons/star';
	import IconStarFilled from '@tabler/icons-svelte/icons/star-filled';
	import IconBolt from '@tabler/icons-svelte/icons/bolt';
	import IconFlame from '@tabler/icons-svelte/icons/flame';
	import IconRocket from '@tabler/icons-svelte/icons/rocket';
	import IconTarget from '@tabler/icons-svelte/icons/target';
	import IconTargetArrow from '@tabler/icons-svelte/icons/target-arrow';
	import IconEye from '@tabler/icons-svelte/icons/eye';
	import IconBell from '@tabler/icons-svelte/icons/bell';
	import IconBellFilled from '@tabler/icons-svelte/icons/bell-filled';
	import IconDownload from '@tabler/icons-svelte/icons/download';
	import IconExternalLink from '@tabler/icons-svelte/icons/external-link';
	import IconLogout from '@tabler/icons-svelte/icons/logout';
	import IconLogin from '@tabler/icons-svelte/icons/login';
	import IconMoodHappy from '@tabler/icons-svelte/icons/mood-happy';
	import IconBrandYoutube from '@tabler/icons-svelte/icons/brand-youtube';
	import IconPresentation from '@tabler/icons-svelte/icons/presentation';
	import IconPresentationAnalytics from '@tabler/icons-svelte/icons/presentation-analytics';
	import IconBrain from '@tabler/icons-svelte/icons/brain';
	import IconBulb from '@tabler/icons-svelte/icons/bulb';
	import IconAward from '@tabler/icons-svelte/icons/award';
	import IconTrophy from '@tabler/icons-svelte/icons/trophy';
	import IconNews from '@tabler/icons-svelte/icons/news';
	import IconFileAnalytics from '@tabler/icons-svelte/icons/file-analytics';
	import type { ComponentType } from 'svelte';

	/**
	 * ─────────────────────────────────────────────────────────────────────────────
	 * ICON MAPPING - st-icon-* to Tabler Icon
	 * ─────────────────────────────────────────────────────────────────────────────
	 *
	 * Maps legacy st-icon class names to Tabler SVG components.
	 * This allows seamless migration from font icons to SVG icons.
	 */
	const ICON_MAP: Record<string, ComponentType> = {
		// Navigation icons
		'home': IconHomeFilled,
		'dashboard': IconDashboard,
		'learning-center': IconSchool,
		'chatroom-archive': IconArchive,
		'forum': IconMessages,
		'courses': IconBooks,
		'indicators': IconChartCandle,
		'support': IconHelpCircle,
		'settings': IconSettings,
		'logout': IconLogout,

		// Trading room icons
		'mastering-the-trade': IconChartLine,
		'options-day-trading': IconChartCandle,
		'simpler-showcase': IconStar,
		'revolution-showcase': IconStarFilled,
		'explosive-swings': IconBolt,
		'swing-trading': IconTrendingUp,
		'small-accounts': IconWallet,
		'day-trading': IconChartBar,
		'training-room': IconPresentation,
		'small-lot': IconCoin,
		'trendy': IconTrendingUp,
		'bias': IconTarget,
		'sector-secrets': IconReportAnalytics,
		'moxie': IconFlame,
		'compounding-growth': IconTrendingUp,
		'voodoo-mastery': IconBrain,
		'long-term-growth': IconChartLine,
		'stock-mavericks': IconRocket,

		// Tools
		'trade-of-the-week': IconCalendarWeek,
		'weekly-watchlist': IconListCheck,
		'scanner': IconEye,

		// Content types
		'course': IconBook,
		'indicator': IconChartCandle,
		'alert': IconBellFilled,
		'daily-videos': IconVideo,
		'premium-newsletter': IconNews,
		'top-tier-outlook': IconFileAnalytics,

		// UI elements
		'chart': IconChartLine,
		'candle-stick': IconChartCandle,
		'handle-stick': IconChartBar,
		'spx-profit-pulse': IconBell,
		'stacked-profits': IconTrendingUp,
		'big-market-scorecard': IconPresentationAnalytics,

		// Membership types
		'options': IconChartCandle,
		'foundation': IconAward,
		'mastery': IconTrophy,
		'futures': IconChartBar,
		'fibonacci': IconChartPie,
		'scanner-nav': IconEye,
		'edge': IconTargetArrow,

		// Fallback default
		'default': IconDashboard
	};

	/**
	 * ─────────────────────────────────────────────────────────────────────────────
	 * PROPS
	 * ─────────────────────────────────────────────────────────────────────────────
	 */
	interface Props {
		// Icon name (e.g., "home", "mastering-the-trade", or "st-icon-home")
		name: string;
		// Icon size in pixels
		size?: number;
		// Additional CSS classes
		class?: string;
		// Stroke width (Tabler icons default to 2)
		stroke?: number;
		// Custom color (overrides CSS)
		color?: string;
	}

	let {
		name,
		size = 24,
		class: className = '',
		stroke = 2,
		color = undefined
	}: Props = $props();

	/**
	 * ─────────────────────────────────────────────────────────────────────────────
	 * DERIVED STATE
	 * ─────────────────────────────────────────────────────────────────────────────
	 */

	// Normalize icon name (remove "st-icon-" prefix if present)
	let normalizedName = $derived(
		name.startsWith('st-icon-') ? name.replace('st-icon-', '') : name
	);

	// Get the Tabler icon component (or undefined if not found)
	let IconComponent = $derived(ICON_MAP[normalizedName] || null);

	// Should we fallback to the font icon?
	let useFontFallback = $derived(!IconComponent);

	// Font icon class (for fallback)
	let fontIconClass = $derived(`st-icon-${normalizedName}`);
</script>

<!--
═══════════════════════════════════════════════════════════════════════════════════
TEMPLATE - Render SVG or Font Icon
═══════════════════════════════════════════════════════════════════════════════════
-->

{#if IconComponent}
	<!--
	SVG Icon (Preferred)
	Uses svelte:component for dynamic component rendering
	-->
	<svelte:component
		this={IconComponent}
		{size}
		{stroke}
		{color}
		class="dashboard-icon {className}"
	/>
{:else}
	<!--
	Font Icon Fallback
	Uses the legacy st-icon font classes
	-->
	<span
		class="dashboard-icon dashboard-icon--font {fontIconClass} {className}"
		style="font-size: {size}px; width: {size}px; height: {size}px; {color ? `color: ${color};` : ''}"
		aria-hidden="true"
	></span>
{/if}

<!--
═══════════════════════════════════════════════════════════════════════════════════
STYLES
═══════════════════════════════════════════════════════════════════════════════════
-->
<style>
	.dashboard-icon {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
		vertical-align: middle;
	}

	.dashboard-icon--font {
		font-family: 'StIcons', system-ui, sans-serif;
		font-style: normal;
		font-weight: normal;
		line-height: 1;
		-webkit-font-smoothing: antialiased;
		-moz-osx-font-smoothing: grayscale;
	}

	/* Ensure SVG icons inherit color */
	:global(.dashboard-icon svg) {
		color: inherit;
	}
</style>

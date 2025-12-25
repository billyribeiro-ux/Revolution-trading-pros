<!--
═══════════════════════════════════════════════════════════════════════════════════
DASHBOARD HEADER COMPONENT - Svelte 5 / SvelteKit (Nov/Dec 2025)
═══════════════════════════════════════════════════════════════════════════════════

PURPOSE:
This component renders the dashboard header section containing:
- Page title (h1)
- Trading Room Rules link
- "Enter a Trading Room" dropdown button

The design matches the Simpler Trading reference (core file lines 2848-2873).

SVELTE 5 PATTERNS USED:
- $props() rune: Type-safe prop declaration
- $state() rune: Dropdown open/close state
- Component composition: Uses TradingRoomDropdown child component

LAYOUT:
┌─────────────────────────────────────────────────────────────────────────────────┐
│ Member Dashboard                      Trading Room Rules | [Enter Trading Room]│
│                                       By logging in, you agree to our rules... │
└─────────────────────────────────────────────────────────────────────────────────┘

@version 3.0.0 - Svelte 5 Runes
@updated December 2025
═══════════════════════════════════════════════════════════════════════════════════
-->
<script lang="ts">
	/**
	 * ─────────────────────────────────────────────────────────────────────────────
	 * IMPORTS
	 * ─────────────────────────────────────────────────────────────────────────────
	 */
	import type { TradingRoom, DashboardHeaderProps } from './types';
	import TradingRoomDropdown from './TradingRoomDropdown.svelte';

	/**
	 * ─────────────────────────────────────────────────────────────────────────────
	 * PROPS - Svelte 5 $props() Rune
	 * ─────────────────────────────────────────────────────────────────────────────
	 */
	let {
		// Required: Page title to display
		title,
		// Optional: Array of trading rooms for the dropdown
		tradingRooms = [],
		// Optional: Show the Trading Room Rules link
		showRulesLink = true
	}: DashboardHeaderProps = $props();

	/**
	 * ─────────────────────────────────────────────────────────────────────────────
	 * LOCAL STATE - Svelte 5 $state() Rune
	 * ─────────────────────────────────────────────────────────────────────────────
	 */

	// Dropdown open/close state
	let dropdownOpen = $state(false);

	/**
	 * ─────────────────────────────────────────────────────────────────────────────
	 * CONSTANTS - Static content
	 * ─────────────────────────────────────────────────────────────────────────────
	 */

	// Trading Room Rules PDF URL
	const RULES_PDF_URL = 'https://cdn.simplertrading.com/2024/02/07192341/Simpler-Tradings-Rules-of-the-Room.pdf';

	// Rules disclaimer text
	const RULES_DISCLAIMER = 'By logging into any of our Live Trading Rooms, You are agreeing to our Rules of the Room.';

	/**
	 * ─────────────────────────────────────────────────────────────────────────────
	 * DERIVED STATE
	 * ─────────────────────────────────────────────────────────────────────────────
	 */

	// Check if we have trading rooms to show the dropdown
	let hasTradingRooms = $derived(tradingRooms && tradingRooms.length > 0);

	/**
	 * ─────────────────────────────────────────────────────────────────────────────
	 * EVENT HANDLERS
	 * ─────────────────────────────────────────────────────────────────────────────
	 */

	/**
	 * Toggle the trading room dropdown
	 */
	function handleDropdownToggle(): void {
		dropdownOpen = !dropdownOpen;
	}

	/**
	 * Close dropdown when clicking outside
	 * This is handled by the TradingRoomDropdown component internally
	 */
	function handleWindowClick(): void {
		if (dropdownOpen) {
			dropdownOpen = false;
		}
	}
</script>

<!--
═══════════════════════════════════════════════════════════════════════════════════
TEMPLATE - Dashboard Header
═══════════════════════════════════════════════════════════════════════════════════

Matches WordPress structure from core file:
<header class="dashboard__header">
  <div class="dashboard__header-left">...</div>
  <div class="dashboard__header-right">...</div>
</header>
-->

<!-- Global click handler to close dropdown -->
<svelte:window onclick={handleWindowClick} />

<header class="dashboard__header">
	<!--
	─────────────────────────────────────────────────────────────────────────────
	LEFT SECTION - Page Title
	─────────────────────────────────────────────────────────────────────────────
	-->
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">{title}</h1>
	</div>

	<!--
	─────────────────────────────────────────────────────────────────────────────
	RIGHT SECTION - Rules Link and Trading Room Dropdown
	─────────────────────────────────────────────────────────────────────────────
	-->
	<div class="dashboard__header-right">
		<!--
		Trading Room Rules Link
		Only shown if showRulesLink is true
		-->
		{#if showRulesLink}
			<ul class="ultradingroom">
				<li class="litradingroom">
					<a
						href={RULES_PDF_URL}
						target="_blank"
						rel="noopener noreferrer"
						class="btn btn-xs btn-link"
					>
						Trading Room Rules
					</a>
				</li>
				<li class="litradingroomhind btn btn-xs btn-link">
					{RULES_DISCLAIMER}
				</li>
			</ul>
		{/if}

		<!--
		Trading Room Dropdown
		Only shown if we have trading rooms
		-->
		{#if hasTradingRooms}
			<TradingRoomDropdown
				rooms={tradingRooms}
				isOpen={dropdownOpen}
				onToggle={handleDropdownToggle}
			/>
		{/if}
	</div>
</header>

<!--
═══════════════════════════════════════════════════════════════════════════════════
STYLES - Dashboard Header CSS
═══════════════════════════════════════════════════════════════════════════════════
-->
<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   HEADER CONTAINER
	   Matches WordPress exactly (core file lines 2848-2870)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__header {
		background-color: #fff;
		border-bottom: 1px solid #dbdbdb;
		max-width: 100%;
		padding: 20px;
		display: flex;
		flex-wrap: wrap;
		justify-content: space-between;
		align-items: center;
	}

	/* Tablet and up - horizontal layout */
	@media screen and (min-width: 820px) {
		.dashboard__header {
			display: flex;
			flex-wrap: wrap;
			justify-content: space-between;
		}
	}

	/* Desktop - more padding */
	@media screen and (min-width: 1280px) {
		.dashboard__header {
			padding: 30px;
		}
	}

	@media screen and (min-width: 1440px) {
		.dashboard__header {
			padding: 30px 40px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   LEFT SECTION - Title
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__header-left {
		display: flex;
		align-items: center;
		flex-direction: column;
	}

	@media screen and (min-width: 577px) {
		.dashboard__header-left {
			flex-direction: row;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   PAGE TITLE
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__page-title {
		margin: 0;
		color: #333;
		font-size: 36px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		line-height: 1.2;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RIGHT SECTION - Rules and Dropdown
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__header-right {
		display: flex;
		align-items: center;
		flex-direction: column;
		margin-top: 10px;
		text-align: right;
		gap: 10px;
	}

	@media screen and (min-width: 577px) {
		.dashboard__header-right {
			flex-direction: row;
			justify-content: flex-end;
		}
	}

	@media screen and (min-width: 820px) {
		.dashboard__header-right {
			flex-direction: row;
			justify-content: flex-end;
			margin-top: 0;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   TRADING ROOM RULES LIST
	   Matches WordPress exactly (ultradingroom class)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.ultradingroom {
		text-align: right;
		list-style: none;
		margin: 0;
		padding: 0;
		max-width: 299px;
	}

	.ultradingroom .litradingroom {
		display: block;
	}

	.ultradingroom .litradingroom a {
		font-weight: 700 !important;
		font-size: 12px;
		color: #1e73be;
		text-decoration: none;
	}

	.ultradingroom .litradingroom a:hover {
		text-decoration: underline;
	}

	.ultradingroom .litradingroomhind {
		font-size: 11px;
		color: #666;
		display: block;
		width: 300px;
		float: right;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   BUTTON UTILITIES
	   ═══════════════════════════════════════════════════════════════════════════ */

	.btn-xs {
		padding: 1px 5px;
		font-size: 12px;
		line-height: 1.5;
		border-radius: 3px;
	}

	.btn-link {
		background: none;
		border: none;
		color: #1e73be;
	}
</style>

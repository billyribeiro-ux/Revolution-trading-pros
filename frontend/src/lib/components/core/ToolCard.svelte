<!--
═══════════════════════════════════════════════════════════════════════════════════
TOOL CARD COMPONENT - Svelte 5 / SvelteKit (Nov/Dec 2025)
═══════════════════════════════════════════════════════════════════════════════════

PURPOSE:
This component renders a tool card for the Tools section of the dashboard.
Tools include Weekly Watchlist, Scanner, and other trading utilities.
Simpler design than MembershipCard - only shows Dashboard link.

SVELTE 5 PATTERNS USED:
- $props() rune: Type-safe prop declaration
- Minimal state: Pure presentational component
- CSS custom properties: Dynamic icon styling

USAGE:
<ToolCard
  name="Weekly Watchlist"
  slug="ww"
  icon="st-icon-trade-of-the-week"
  href="/dashboard/ww/"
/>

@version 3.0.0 - Svelte 5 Runes
@updated December 2025
═══════════════════════════════════════════════════════════════════════════════════
-->
<script lang="ts">
	/**
	 * ─────────────────────────────────────────────────────────────────────────────
	 * IMPORTS
	 * ─────────────────────────────────────────────────────────────────────────────
	 * Import DashboardIcon for crisp SVG icons (replaces font icons)
	 */
	import type { ToolCardProps } from './types';
	import DashboardIcon from './DashboardIcon.svelte';

	/**
	 * ─────────────────────────────────────────────────────────────────────────────
	 * PROPS - Svelte 5 $props() Rune
	 * ─────────────────────────────────────────────────────────────────────────────
	 *
	 * Pure presentational component - all data comes from props.
	 * No internal state management needed.
	 */
	let {
		// Required: Display name of the tool
		name,
		// Required: URL-safe slug for routing
		slug,
		// Optional: Custom icon class (defaults to slug-based icon)
		icon = undefined,
		// Required: Link destination
		href
	}: ToolCardProps = $props();

	/**
	 * ─────────────────────────────────────────────────────────────────────────────
	 * DERIVED STATE
	 * ─────────────────────────────────────────────────────────────────────────────
	 */

	// Compute icon class - use provided icon or derive from slug
	let iconClass = $derived(icon || `st-icon-${slug}`);
</script>

<!--
═══════════════════════════════════════════════════════════════════════════════════
TEMPLATE - Tool Card Structure
═══════════════════════════════════════════════════════════════════════════════════

Matches WordPress structure from core file (lines 2929-2943):
<article class="membership-card membership-card--ww">
  <a class="membership-card__header">...</a>
  <div class="membership-card__actions">...</div>
</article>
-->
<article class="membership-card membership-card--ww">
	<!--
	─────────────────────────────────────────────────────────────────────────────
	CARD HEADER - Icon and Tool Name
	─────────────────────────────────────────────────────────────────────────────
	-->
	<a href={href} class="membership-card__header">
		<!-- Icon Container -->
		<span class="mem_icon">
			<span class="membership-card__icon">
				<!--
				DashboardIcon maps tool slugs to Tabler SVG icons.
				Falls back to font icons if no mapping exists.
				-->
				<DashboardIcon name={slug} size={24} />
			</span>
		</span>

		<!-- Tool Name -->
		<span class="mem_div">{name}</span>
	</a>

	<!--
	─────────────────────────────────────────────────────────────────────────────
	ACTION BUTTONS - Only Dashboard link for tools
	─────────────────────────────────────────────────────────────────────────────
	Tools typically only have a Dashboard action (no trading room access)
	-->
	<div class="membership-card__actions membership-card__actions--single">
		<a href={href}>Dashboard</a>
	</div>
</article>

<!--
═══════════════════════════════════════════════════════════════════════════════════
STYLES - Tool Card CSS
═══════════════════════════════════════════════════════════════════════════════════
-->
<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   TOOL CARD BASE STYLES
	   Inherits from membership-card with orange theme
	   ═══════════════════════════════════════════════════════════════════════════ */

	.membership-card {
		background: #fff;
		border-radius: 5px;
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.1);
		transition: all 0.2s ease-in-out;
		overflow: hidden;
	}

	.membership-card:hover {
		box-shadow: 0 8px 35px rgba(0, 0, 0, 0.12);
		transform: translateY(-2px);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CARD HEADER
	   ═══════════════════════════════════════════════════════════════════════════ */

	.membership-card__header {
		display: block;
		padding: 20px;
		color: #333;
		font-weight: 700;
		white-space: nowrap;
		transition: all 0.15s ease-in-out;
		text-decoration: none;
		font-family: 'Open Sans', sans-serif;
		font-size: 17px;
		line-height: 1.4;
	}

	.membership-card__header:visited {
		color: #333;
	}

	.membership-card__header:focus,
	.membership-card__header:hover {
		color: #0984ae;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ICON CONTAINER
	   Orange theme for tools (Weekly Watchlist variant)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.mem_icon,
	.mem_div {
		display: inline-block;
		vertical-align: middle;
	}

	.mem_div {
		white-space: normal;
		width: calc(100% - 43px);
	}

	.membership-card__icon {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 50px;
		height: 50px;
		margin-right: 9px;
		line-height: 50px;
		color: #fff;
		text-align: center;
		border-radius: 50%;
		transition: all 0.15s ease-in-out;
	}

	/* Weekly Watchlist Orange Theme */
	.membership-card--ww .membership-card__icon {
		background-color: #f69532;
		box-shadow: 0 10px 20px rgba(246, 149, 50, 0.25);
	}

	.membership-card--ww .membership-card__header:hover .membership-card__icon {
		background-color: #dc7309;
		box-shadow: 0 15px 30px rgba(246, 149, 50, 0.2);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ICON SIZING
	   ═══════════════════════════════════════════════════════════════════════════ */

	.icon {
		display: inline-block;
		vertical-align: middle;
	}

	.icon--md {
		font-size: 24px;
		width: 24px;
		height: 24px;
		line-height: 24px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ACTION BUTTONS
	   Single action variant for tools (full width)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.membership-card__actions {
		display: flex;
		font-size: 14px;
		border-top: 1px solid #ededed;
		justify-content: center;
	}

	/* Single action takes full width */
	.membership-card__actions--single a {
		flex-basis: 100%;
		width: 100%;
	}

	.membership-card__actions a {
		display: block;
		flex: 0 0 auto;
		height: 100%;
		padding: 15px;
		text-align: center;
		text-decoration: none;
		color: #666;
		font-family: 'Open Sans', sans-serif;
		font-size: 14px;
		font-weight: 400;
		transition: all 0.15s ease-in-out;
	}

	.membership-card__actions a:hover {
		background-color: #f4f4f4;
		color: #0984ae;
	}

	.membership-card__actions a:focus {
		outline: 2px solid #0984ae;
		outline-offset: -2px;
	}
</style>

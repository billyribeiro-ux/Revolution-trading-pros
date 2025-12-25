<!--
═══════════════════════════════════════════════════════════════════════════════════
MEMBERSHIP CARD COMPONENT - Svelte 5 / SvelteKit (Nov/Dec 2025)
═══════════════════════════════════════════════════════════════════════════════════

PURPOSE:
This component renders a single membership card in the dashboard grid. Each card
displays the membership name, icon, and action buttons for accessing the dashboard
or trading room. The design matches the Simpler Trading reference exactly.

SVELTE 5 PATTERNS USED:
- $props() rune: Type-safe prop destructuring with defaults
- $derived rune: Computed values that auto-update when dependencies change
- Event handlers: Modern onclick syntax (not on:click)
- CSS Variables: Dynamic styling based on props

USAGE:
<MembershipCard
  membership={membershipData}
  variant="options"
  showActions={true}
  onDashboardClick={(m) => goto(`/dashboard/${m.slug}`)}
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
	 * Import types from our centralized type definitions
	 */
	import type {
		Membership,
		CardVariant,
		MembershipCardProps
	} from './types';
	import {
		getCardVariant,
		getActionLabel,
		shouldOpenNewTab
	} from './types';

	/**
	 * ─────────────────────────────────────────────────────────────────────────────
	 * PROPS - Svelte 5 $props() Rune
	 * ─────────────────────────────────────────────────────────────────────────────
	 *
	 * The $props() rune is the Svelte 5 way to declare component props.
	 * It replaces the old `export let` syntax and provides:
	 * - Type safety with TypeScript interfaces
	 * - Default values directly in destructuring
	 * - Automatic reactivity (no need for $: statements)
	 *
	 * @see https://svelte.dev/docs/svelte/$props
	 */
	let {
		// Required: The membership data object
		membership,
		// Optional: Visual variant (options, foundation, ww, default)
		variant = undefined,
		// Optional: Show action buttons (default: true)
		showActions = true,
		// Optional: Callback when dashboard link is clicked
		onDashboardClick = undefined,
		// Optional: Callback when access/trading room link is clicked
		onAccessClick = undefined
	}: MembershipCardProps = $props();

	/**
	 * ─────────────────────────────────────────────────────────────────────────────
	 * DERIVED STATE - Svelte 5 $derived Rune
	 * ─────────────────────────────────────────────────────────────────────────────
	 *
	 * The $derived rune creates computed values that automatically update
	 * when their dependencies change. It replaces the `$: derived = ...` syntax.
	 *
	 * Benefits:
	 * - Cleaner syntax than reactive statements
	 * - Explicit dependency tracking
	 * - Better performance (only recalculates when needed)
	 *
	 * @see https://svelte.dev/docs/svelte/$derived
	 */

	// Compute the card variant from props or derive from membership type
	let cardVariant = $derived(variant ?? getCardVariant(membership.type));

	// Compute the dashboard URL from membership data
	let dashboardUrl = $derived(
		membership.dashboardUrl || `/dashboard/${membership.slug}`
	);

	// Compute the access URL (trading room, alerts, etc.)
	let accessUrl = $derived(
		membership.accessUrl || `/dashboard/${membership.slug}`
	);

	// Compute the action button label based on membership type
	let actionLabel = $derived(
		membership.roomLabel || getActionLabel(membership.type)
	);

	// Determine if the access link should open in a new tab
	let opensNewTab = $derived(shouldOpenNewTab(membership.type));

	// Check if this is a Simpler Showcase membership (special styling)
	let isShowcase = $derived(membership.slug === 'simpler-showcase');

	/**
	 * ─────────────────────────────────────────────────────────────────────────────
	 * EVENT HANDLERS - Svelte 5 Modern Syntax
	 * ─────────────────────────────────────────────────────────────────────────────
	 *
	 * Svelte 5 uses standard JavaScript event syntax:
	 * - onclick={handler} instead of on:click={handler}
	 * - Better TypeScript support
	 * - Consistent with web standards
	 */

	/**
	 * Handle dashboard link click
	 * Calls the optional callback and allows default navigation
	 */
	function handleDashboardClick(event: MouseEvent): void {
		if (onDashboardClick) {
			onDashboardClick(membership);
		}
		// Allow default link behavior (navigation)
	}

	/**
	 * Handle access/trading room link click
	 * Calls the optional callback and allows default navigation
	 */
	function handleAccessClick(event: MouseEvent): void {
		if (onAccessClick) {
			onAccessClick(membership);
		}
		// Allow default link behavior (navigation)
	}
</script>

<!--
═══════════════════════════════════════════════════════════════════════════════════
TEMPLATE - Card Structure
═══════════════════════════════════════════════════════════════════════════════════

The card follows the Simpler Trading design:
1. Header with icon and membership name (clickable)
2. Action bar with Dashboard and Trading Room/Access buttons

Structure mirrors the WordPress HTML from the core file:
<article class="membership-card membership-card--options">
  <a class="membership-card__header">...</a>
  <div class="membership-card__actions">...</div>
</article>
-->
<article class="membership-card membership-card--{cardVariant}">
	<!--
	─────────────────────────────────────────────────────────────────────────────
	CARD HEADER - Icon and Name
	─────────────────────────────────────────────────────────────────────────────
	Clickable header that navigates to the membership dashboard
	-->
	<a
		href={dashboardUrl}
		class="membership-card__header"
		onclick={handleDashboardClick}
	>
		<!-- Icon Container -->
		<span class="mem_icon">
			<span class="membership-card__icon" class:simpler-showcase-icon={isShowcase}>
				<!--
				Icon uses custom font icons (st-icon-*) matching WordPress
				Falls back to a generic icon if not specified
				-->
				<span class="icon icon--lg st-icon-{membership.slug}"></span>
			</span>
		</span>

		<!-- Membership Name -->
		<span class="mem_div">{membership.name}</span>
	</a>

	<!--
	─────────────────────────────────────────────────────────────────────────────
	ACTION BUTTONS - Dashboard and Access Links
	─────────────────────────────────────────────────────────────────────────────
	Conditional rendering using Svelte's {#if} block
	-->
	{#if showActions}
		<div class="membership-card__actions">
			<!-- Dashboard Link -->
			<a href={dashboardUrl} onclick={handleDashboardClick}>
				Dashboard
			</a>

			<!-- Access Link (Trading Room, Alerts, Course, etc.) -->
			{#if opensNewTab}
				<!--
				External links open in new tab with security attributes:
				- target="_blank": Opens in new tab
				- rel="nofollow noopener": SEO and security best practices
				-->
				<a
					href={accessUrl}
					target="_blank"
					rel="nofollow noopener"
					onclick={handleAccessClick}
				>
					{actionLabel}
				</a>
			{:else}
				<a href={accessUrl} onclick={handleAccessClick}>
					{actionLabel}
				</a>
			{/if}
		</div>
	{/if}
</article>

<!--
═══════════════════════════════════════════════════════════════════════════════════
STYLES - Scoped Component CSS
═══════════════════════════════════════════════════════════════════════════════════

Svelte automatically scopes these styles to this component only.
Styles match the Simpler Trading design system exactly.
-->
<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   MEMBERSHIP CARD BASE STYLES
	   Matches WordPress reference exactly (core file lines 3408-3458)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.membership-card {
		/* Card container with elevation shadow */
		background: #fff;
		border-radius: 5px;
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.1);
		transition: all 0.2s ease-in-out;
		overflow: hidden;
	}

	/* Subtle lift on hover for interactivity feedback */
	.membership-card:hover {
		box-shadow: 0 8px 35px rgba(0, 0, 0, 0.12);
		transform: translateY(-2px);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CARD HEADER - Clickable area with icon and name
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

	/* Maintain color on visited state */
	.membership-card__header:visited {
		color: #333;
	}

	/* Highlight on hover/focus for accessibility */
	.membership-card__header:focus,
	.membership-card__header:hover {
		color: #0984ae;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ICON STYLING - Circular icon container
	   ═══════════════════════════════════════════════════════════════════════════ */

	.mem_icon,
	.mem_div {
		display: inline-block;
		vertical-align: middle;
	}

	/* Name wrapper allows text to wrap on narrow screens */
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
		/* Default blue gradient background */
		background-color: #0984ae;
		box-shadow: 0 10px 20px rgba(9, 132, 174, 0.25);
	}

	/* Darken icon background on hover */
	.membership-card__header:hover .membership-card__icon {
		background-color: #076787;
		box-shadow: 0 15px 30px rgba(9, 132, 174, 0.2);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SIMPLER SHOWCASE SPECIAL STYLING
	   Black background with orange text - matches WordPress exactly
	   ═══════════════════════════════════════════════════════════════════════════ */

	.simpler-showcase-icon {
		background: black !important;
		color: orange !important;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ICON SIZE CLASSES
	   Font-based icons from custom icon font
	   ═══════════════════════════════════════════════════════════════════════════ */

	.icon {
		display: inline-block;
		vertical-align: middle;
	}

	.icon--lg {
		font-size: 32px;
		width: 32px;
		height: 32px;
		line-height: 32px;
	}

	.icon--md {
		font-size: 24px;
		width: 24px;
		height: 24px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ACTION BUTTONS - Dashboard and Access links
	   ═══════════════════════════════════════════════════════════════════════════ */

	.membership-card__actions {
		display: flex;
		font-size: 14px;
		border-top: 1px solid #ededed;
		justify-content: center;
	}

	.membership-card__actions a {
		display: block;
		flex: 0 0 auto;
		flex-basis: 50%;
		width: 50%;
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

	/* Hover state for action buttons */
	.membership-card__actions a:hover {
		background-color: #f4f4f4;
		color: #0984ae;
	}

	/* Separator between action buttons */
	.membership-card__actions a + a {
		border-left: 1px solid #ededed;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CARD VARIANTS - Color theming based on membership type
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* Options variant - Blue theme for trading rooms */
	.membership-card--options .membership-card__icon {
		background-color: #0984ae;
		box-shadow: 0 10px 20px rgba(9, 132, 174, 0.25);
	}

	/* Foundation variant - Green theme for courses */
	.membership-card--foundation .membership-card__icon {
		background-color: #28a745;
		box-shadow: 0 10px 20px rgba(40, 167, 69, 0.25);
	}

	/* Weekly Watchlist variant - Orange theme */
	.membership-card--ww .membership-card__icon {
		background-color: #f69532;
		box-shadow: 0 10px 20px rgba(246, 149, 50, 0.25);
	}

	/* Default variant - Neutral gray */
	.membership-card--default .membership-card__icon {
		background-color: #6c757d;
		box-shadow: 0 10px 20px rgba(108, 117, 125, 0.25);
	}
</style>

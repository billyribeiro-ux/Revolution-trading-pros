<!--
═══════════════════════════════════════════════════════════════════════════════════
MEMBERSHIP CARDS GRID COMPONENT - Svelte 5 / SvelteKit (Nov/Dec 2025)
═══════════════════════════════════════════════════════════════════════════════════

PURPOSE:
This component renders a responsive grid of membership cards. It handles:
- Loading states with skeleton loaders
- Error states with retry functionality
- Empty states for users without memberships
- Responsive Bootstrap-style grid layout

SVELTE 5 PATTERNS USED:
- $props() rune: Type-safe prop declaration with defaults
- $derived rune: Computed validation checks
- {#if}/{:else if}/{:else} blocks: Conditional rendering
- {#each} blocks: List iteration with keyed rendering
- {#snippet} blocks: Reusable template fragments (Svelte 5 feature)

LAYOUT:
Responsive grid using CSS Flexbox (matches Bootstrap 4/5 grid):
- Mobile: 1 column (100%)
- Tablet: 2 columns (50%)
- Desktop: 3 columns (33.33%)

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
	import type { Membership, CardVariant } from './types';
	import { getCardVariant } from './types';
	import MembershipCard from './MembershipCard.svelte';

	/**
	 * ─────────────────────────────────────────────────────────────────────────────
	 * PROP TYPES - Interface for component props
	 * ─────────────────────────────────────────────────────────────────────────────
	 */
	interface Props {
		// Required: Array of membership data to display
		memberships: Membership[];
		// Optional: Loading state indicator
		isLoading?: boolean;
		// Optional: Error message to display
		error?: string | null;
		// Optional: Section title (h2)
		title?: string;
		// Optional: Show action buttons on cards
		showActions?: boolean;
		// Optional: Callback when retry button is clicked
		onRetry?: () => void;
		// Optional: Callback when a card's dashboard is clicked
		onDashboardClick?: (membership: Membership) => void;
		// Optional: Callback when a card's access button is clicked
		onAccessClick?: (membership: Membership) => void;
	}

	/**
	 * ─────────────────────────────────────────────────────────────────────────────
	 * PROPS - Svelte 5 $props() Rune
	 * ─────────────────────────────────────────────────────────────────────────────
	 */
	let {
		memberships,
		isLoading = false,
		error = null,
		title = 'Memberships',
		showActions = true,
		onRetry = undefined,
		onDashboardClick = undefined,
		onAccessClick = undefined
	}: Props = $props();

	/**
	 * ─────────────────────────────────────────────────────────────────────────────
	 * DERIVED STATE - Computed values
	 * ─────────────────────────────────────────────────────────────────────────────
	 */

	// Check if we have any memberships to display
	let hasMemberships = $derived(memberships && memberships.length > 0);

	// Count for display (e.g., "3 Memberships")
	let membershipCount = $derived(memberships?.length || 0);

	/**
	 * ─────────────────────────────────────────────────────────────────────────────
	 * EVENT HANDLERS
	 * ─────────────────────────────────────────────────────────────────────────────
	 */

	/**
	 * Handle retry button click
	 */
	function handleRetry(): void {
		if (onRetry) {
			onRetry();
		} else {
			// Default: Reload the page
			window.location.reload();
		}
	}
</script>

<!--
═══════════════════════════════════════════════════════════════════════════════════
TEMPLATE - Membership Cards Grid
═══════════════════════════════════════════════════════════════════════════════════

Conditional rendering pattern:
1. Loading → Show skeleton cards
2. Error → Show error message with retry
3. Empty → Show empty state message
4. Data → Show membership cards grid
-->
<section class="dashboard__content-section">
	<!-- Section Title -->
	<h2 class="section-title">{title}</h2>

	<!--
	─────────────────────────────────────────────────────────────────────────────
	LOADING STATE - Skeleton Cards
	─────────────────────────────────────────────────────────────────────────────
	Shows animated placeholder cards while data is loading
	-->
	{#if isLoading}
		<div class="membership-cards row">
			<!-- Render 3 skeleton cards as placeholders -->
			{#each [1, 2, 3] as i (i)}
				<div class="col-sm-6 col-xl-4">
					<article class="membership-card membership-card--skeleton">
						<div class="skeleton-header">
							<div class="skeleton skeleton--icon"></div>
							<div class="skeleton skeleton--text"></div>
						</div>
						<div class="skeleton-actions">
							<div class="skeleton skeleton--button"></div>
							<div class="skeleton skeleton--button"></div>
						</div>
					</article>
				</div>
			{/each}
		</div>

	<!--
	─────────────────────────────────────────────────────────────────────────────
	ERROR STATE - Error Message with Retry
	─────────────────────────────────────────────────────────────────────────────
	-->
	{:else if error}
		<div class="error-state">
			<p class="error-message">{error}</p>
			<button
				type="button"
				class="btn btn-primary"
				onclick={handleRetry}
			>
				Retry
			</button>
		</div>

	<!--
	─────────────────────────────────────────────────────────────────────────────
	EMPTY STATE - No Memberships
	─────────────────────────────────────────────────────────────────────────────
	-->
	{:else if !hasMemberships}
		<div class="empty-state">
			<p>You don't have any active memberships.</p>
			<a href="/pricing" class="btn btn-primary">
				View Available Plans
			</a>
		</div>

	<!--
	─────────────────────────────────────────────────────────────────────────────
	DATA STATE - Membership Cards Grid
	─────────────────────────────────────────────────────────────────────────────
	Renders the actual membership cards in a responsive grid
	-->
	{:else}
		<div class="membership-cards row">
			<!--
			Iterate over memberships with keyed rendering
			(membership.id) provides a unique key for efficient DOM updates
			-->
			{#each memberships as membership (membership.id)}
				<div class="col-sm-6 col-xl-4">
					<MembershipCard
						{membership}
						variant={getCardVariant(membership.type)}
						{showActions}
						{onDashboardClick}
						{onAccessClick}
					/>
				</div>
			{/each}
		</div>
	{/if}
</section>

<!--
═══════════════════════════════════════════════════════════════════════════════════
STYLES - Grid and State CSS
═══════════════════════════════════════════════════════════════════════════════════
-->
<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   SECTION CONTAINER
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content-section {
		padding: 30px 40px;
		background-color: #fff;
		margin-bottom: 20px;
		overflow-x: auto;
		overflow-y: hidden;
	}

	@media screen and (min-width: 1280px) {
		.dashboard__content-section {
			padding: 30px;
		}
	}

	@media screen and (min-width: 1440px) {
		.dashboard__content-section {
			padding: 40px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SECTION TITLE
	   ═══════════════════════════════════════════════════════════════════════════ */

	.section-title {
		color: #333;
		font-size: 32px;
		font-weight: 700;
		margin: 0 0 30px;
		font-family: 'Open Sans', sans-serif;
		line-height: 1.2;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   GRID LAYOUT - Bootstrap-style flexbox grid
	   ═══════════════════════════════════════════════════════════════════════════ */

	.membership-cards {
		margin-top: -30px;
	}

	.membership-cards.row {
		display: flex;
		flex-wrap: wrap;
		margin: -30px -15px 0;
	}

	.row {
		margin-left: -15px;
		margin-right: -15px;
	}

	/* Column base styles */
	.col-sm-6 {
		flex: 0 0 100%;
		max-width: 100%;
		padding: 0 15px;
		box-sizing: border-box;
		margin-top: 30px;
	}

	/* Tablet: 2 columns */
	@media (min-width: 641px) {
		.col-sm-6 {
			flex: 0 0 50%;
			max-width: 50%;
		}
	}

	/* Desktop: 3 columns */
	.col-xl-4 {
		flex: 0 0 33.333%;
		max-width: 33.333%;
	}

	/* Tablet override: back to 2 columns */
	@media (max-width: 992px) {
		.col-xl-4 {
			flex: 0 0 50%;
			max-width: 50%;
		}
	}

	/* Mobile: 1 column */
	@media (max-width: 641px) {
		.col-sm-6,
		.col-xl-4 {
			flex: 0 0 100%;
			max-width: 100%;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   LOADING STATE - Skeleton Cards
	   ═══════════════════════════════════════════════════════════════════════════ */

	.membership-card--skeleton {
		background: #fff;
		border: 1px solid #dbdbdb;
		border-radius: 8px;
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.1);
		overflow: hidden;
	}

	.skeleton-header {
		display: flex;
		align-items: center;
		padding: 20px;
		gap: 12px;
	}

	.skeleton-actions {
		display: flex;
		border-top: 1px solid #ededed;
	}

	.skeleton {
		background: linear-gradient(90deg, #f0f0f0 25%, #e0e0e0 50%, #f0f0f0 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 4px;
	}

	.skeleton--icon {
		width: 40px;
		height: 40px;
		border-radius: 50%;
		flex-shrink: 0;
	}

	.skeleton--text {
		width: 120px;
		height: 20px;
	}

	.skeleton--button {
		flex: 1;
		height: 50px;
	}

	.skeleton--button + .skeleton--button {
		border-left: 1px solid #ededed;
	}

	@keyframes shimmer {
		0% {
			background-position: 200% 0;
		}
		100% {
			background-position: -200% 0;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ERROR STATE
	   ═══════════════════════════════════════════════════════════════════════════ */

	.error-state {
		padding: 40px;
		text-align: center;
		background: #fff8f8;
		border: 1px solid #ffdddd;
		border-radius: 8px;
	}

	.error-message {
		color: #dc3545;
		margin: 0 0 20px 0;
		font-size: 14px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   EMPTY STATE
	   ═══════════════════════════════════════════════════════════════════════════ */

	.empty-state {
		padding: 40px;
		text-align: center;
		background: #f8f9fa;
		border-radius: 8px;
	}

	.empty-state p {
		color: #666;
		margin: 0 0 20px 0;
		font-size: 14px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   BUTTON STYLES
	   ═══════════════════════════════════════════════════════════════════════════ */

	.btn {
		display: inline-block;
		padding: 10px 20px;
		text-decoration: none;
		border: none;
		border-radius: 5px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		cursor: pointer;
		transition: all 0.2s ease-in-out;
		font-size: 14px;
		text-align: center;
	}

	.btn-primary {
		background: #f69532;
		color: #fff;
		box-shadow: 0 2px 5px rgba(0, 0, 0, 0.16);
	}

	.btn-primary:hover {
		background: #dc7309;
	}

	.btn-primary:focus {
		outline: 2px solid #0984ae;
		outline-offset: 2px;
	}
</style>

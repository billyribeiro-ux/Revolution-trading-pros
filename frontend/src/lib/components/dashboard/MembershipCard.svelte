<script lang="ts">
	/**
	 * Membership Card Component - Simpler Trading Exact Match
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * 100% match to WordPress Simpler Trading structure:
	 * <article class="membership-card membership-card--{type}">
	 *   <a class="membership-card__header">
	 *     <span class="mem_icon">
	 *       <span class="membership-card__icon">
	 *         <span class="icon icon--lg st-icon-{name}"></span>
	 *       </span>
	 *     </span>
	 *     <span class="mem_div">{name}</span>
	 *   </a>
	 *   <div class="membership-card__actions">
	 *     <a>Dashboard</a>
	 *     <a>Trading Room</a>
	 *   </div>
	 * </article>
	 *
	 * @version 4.0.0 (Simpler Trading Exact / December 2025)
	 */

	import '$lib/styles/st-icons.css';

	// ═══════════════════════════════════════════════════════════════════════════
	// TYPES
	// ═══════════════════════════════════════════════════════════════════════════

	// Card display types (for CSS modifiers)
	type CardType = 'options' | 'foundation' | 'moxie' | 'ww' | 'trading-room' | 'alert' | 'course' | 'indicator';

	// API types that need normalization
	type ApiType = 'trading-room' | 'alert-service' | 'course' | 'indicator' | 'weekly-watchlist';

	interface Props {
		id?: string;
		name: string;
		type?: CardType | ApiType | string;
		slug: string;
		icon?: string;
		dashboardUrl?: string;
		roomUrl?: string;
		roomLabel?: string;
		skeleton?: boolean;
		status?: 'active' | 'expiring' | 'expired';
		daysUntilExpiry?: number;
		accessUrl?: string;
		onclick?: () => void;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// PROPS (Svelte 5 Runes)
	// ═══════════════════════════════════════════════════════════════════════════

	let {
		id,
		name,
		type = 'options',
		slug,
		icon,
		dashboardUrl,
		roomUrl,
		roomLabel,
		skeleton = false,
		status = 'active',
		daysUntilExpiry,
		accessUrl,
		onclick
	}: Props = $props();

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE
	// ═══════════════════════════════════════════════════════════════════════════

	// Normalize API types to display types
	const normalizedType = $derived.by((): CardType => {
		const typeMap: Record<string, CardType> = {
			// API types → Display types
			'alert-service': 'alert',
			'weekly-watchlist': 'ww',
			// Slug-based mappings for specific services
			'mastering-the-trade': 'options',
			'simpler-showcase': 'options',
			// Pass-through types
			'trading-room': 'trading-room',
			'options': 'options',
			'foundation': 'foundation',
			'moxie': 'moxie',
			'ww': 'ww',
			'alert': 'alert',
			'course': 'course',
			'indicator': 'indicator'
		};

		// First check if the slug has a specific type mapping
		if (slug && typeMap[slug]) {
			return typeMap[slug];
		}

		// Then check the type prop
		return typeMap[type ?? ''] || (type as CardType) || 'options';
	});

	// Card modifier class (WordPress: membership-card--options, membership-card--moxie, etc.)
	const cardModifier = $derived(`membership-card--${normalizedType}`);

	// Icon class based on slug
	const iconClass = $derived.by(() => {
		const iconMap: Record<string, string> = {
			'day-trading': 'st-icon-day-trading',
			'swing-trading': 'st-icon-swing-trading',
			'small-accounts': 'st-icon-small-accounts',
			'spx-profit-pulse': 'st-icon-spx-profit-pulse',
			'explosive-swings': 'st-icon-explosive-swings',
			'mastering-the-trade': 'st-icon-mastering-the-trade',
			'simpler-showcase': 'st-icon-simpler-showcase',
			'moxie': 'st-icon-moxie',
			'weekly-watchlist': 'st-icon-trade-of-the-week'
		};
		return iconMap[slug] || `st-icon-${slug}`;
	});

	// Icon size class (Weekly Watchlist uses smaller icon)
	const iconSizeClass = $derived(normalizedType === 'ww' ? 'icon--md' : 'icon--lg');

	// Dashboard URL
	const finalDashboardUrl = $derived(dashboardUrl || `/dashboard/${slug}`);

	// Room/Access URL
	const finalRoomUrl = $derived(roomUrl || accessUrl || `/dashboard/${slug}/room`);

	// Room label based on normalized type
	const finalRoomLabel = $derived.by(() => {
		if (roomLabel) return roomLabel;
		switch (normalizedType) {
			case 'trading-room':
			case 'options':
			case 'foundation':
			case 'moxie':
				return 'Trading Room';
			case 'alert':
				return 'Alerts';
			case 'course':
				return 'Course';
			case 'indicator':
				return 'Indicator';
			case 'ww':
				return null; // Weekly watchlist has no second link
			default:
				return 'Trading Room';
		}
	});

	// Show room link (Weekly Watchlist has no Trading Room link)
	const showRoomLink = $derived(normalizedType !== 'ww' && finalRoomLabel !== null);
</script>

<!-- ═══════════════════════════════════════════════════════════════════════════
     TEMPLATE - Matches Simpler Trading WordPress exactly
     ═══════════════════════════════════════════════════════════════════════════ -->

{#if skeleton}
	<!-- Skeleton Loading State -->
	<article class="membership-card skeleton" aria-busy="true">
		<div class="membership-card__header">
			<span class="mem_icon">
				<span class="membership-card__icon skeleton-icon"></span>
			</span>
			<span class="mem_div skeleton-text"></span>
		</div>
		<div class="membership-card__actions">
			<span class="skeleton-button"></span>
			<span class="skeleton-button"></span>
		</div>
	</article>
{:else}
	<!-- WordPress: <article class="membership-card membership-card--options"> -->
	<article class="membership-card {cardModifier}" class:is-expiring={status === 'expiring'} class:is-expired={status === 'expired'}>
		<!-- Status Badge -->
		{#if status === 'expiring' && daysUntilExpiry !== undefined}
			<span class="status-badge status-expiring">{daysUntilExpiry}d</span>
		{:else if status === 'expired'}
			<span class="status-badge status-expired">Expired</span>
		{/if}

		<!-- WordPress: <a href="..." class="membership-card__header"> -->
		<a href={finalRoomUrl} class="membership-card__header" onclick={onclick}>
			<!-- WordPress: <span class="mem_icon"> -->
			<span class="mem_icon">
				<!-- WordPress: <span class="membership-card__icon"> -->
				<span class="membership-card__icon {slug}-icon">
					<!-- WordPress: <span class="icon icon--lg st-icon-mastering-the-trade"></span> -->
					<span class="icon {iconSizeClass} {iconClass}"></span>
				</span>
			</span>
			<!-- WordPress: <span class="mem_div">Mastering the Trade</span> -->
			<span class="mem_div">{name}</span>
		</a>

		<!-- WordPress: <div class="membership-card__actions"> -->
		<div class="membership-card__actions">
			<a href={finalDashboardUrl}>Dashboard</a>
			{#if showRoomLink}
				<a href={finalRoomUrl} target="_blank" rel="nofollow">{finalRoomLabel}</a>
			{/if}
		</div>
	</article>
{/if}

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES - Matches Simpler Trading CSS exactly
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   MEMBERSHIP CARD - WordPress Simpler Trading Exact
	   ═══════════════════════════════════════════════════════════════════════════ */

	.membership-card {
		position: relative;
		background: #fff;
		border-radius: 5px;
		box-shadow: 0 5px 30px rgb(0 0 0 / 10%);
		transition: all 0.15s ease-in-out;
		overflow: hidden;
	}

	.membership-card:hover {
		transform: translateY(-4px);
		box-shadow: 0 10px 40px rgb(0 0 0 / 15%);
	}

	.membership-card.is-expiring {
		border: 2px solid #f59e0b;
	}

	.membership-card.is-expired {
		border: 2px solid #ef4444;
		opacity: 0.7;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   STATUS BADGE
	   ═══════════════════════════════════════════════════════════════════════════ */

	.status-badge {
		position: absolute;
		top: 10px;
		right: 10px;
		padding: 2px 8px;
		border-radius: 10px;
		font-size: 11px;
		font-weight: 700;
		color: #fff;
		z-index: 1;
	}

	.status-expiring {
		background: #f59e0b;
	}

	.status-expired {
		background: #ef4444;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   HEADER - WordPress: .membership-card__header
	   ═══════════════════════════════════════════════════════════════════════════ */

	.membership-card__header {
		display: flex;
		align-items: center;
		padding: 20px;
		color: #333;
		font-weight: 700;
		white-space: normal;
		text-decoration: none;
		transition: color 0.15s ease-in-out;
	}

	.membership-card__header:hover {
		color: #0e6ac4;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ICON WRAPPER - WordPress: .mem_icon, .membership-card__icon
	   ═══════════════════════════════════════════════════════════════════════════ */

	.mem_icon,
	.mem_div {
		display: inline-block;
		vertical-align: middle;
	}

	.mem_div {
		white-space: normal;
		width: calc(100% - 60px);
		font-size: 17px;
		font-family: 'Open Sans', system-ui, sans-serif;
		line-height: 1.3;
		margin-left: 12px;
	}

	.membership-card__icon {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 50px;
		height: 50px;
		line-height: 50px;
		color: #000;
		text-align: center;
		border-radius: 50%;
		transition: all 0.15s ease-in-out;
		background-color: #0984ae;
		box-shadow: 0 10px 20px rgba(12, 36, 52, 0.25);
	}

	.membership-card:hover .membership-card__icon {
		transform: scale(1.05);
		box-shadow: 0 5px 20px rgba(0, 0, 0, 0.5);
	}

	/* Icon inside */
	.icon {
		font-family: 'StIcons', system-ui, sans-serif;
		font-style: normal;
		font-weight: normal;
		color: #fff;
	}

	.icon--lg {
		font-size: 24px;
		line-height: 24px;
	}

	.icon--md {
		font-size: 20px;
		line-height: 20px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ACTIONS - WordPress: .membership-card__actions
	   ═══════════════════════════════════════════════════════════════════════════ */

	.membership-card__actions {
		display: flex;
		font-size: 16px;
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
		color: #1e73be;
		font-weight: 400;
		font-size: 14px;
		text-decoration: none;
		transition: background-color 0.15s ease-in-out, color 0.15s ease-in-out;
	}

	.membership-card__actions a:hover {
		background-color: #f4f4f4;
		color: #0984ae;
	}

	.membership-card__actions a + a {
		border-left: 1px solid #ededed;
	}

	/* Single action (no room link) */
	.membership-card__actions a:only-child {
		flex-basis: 100%;
		width: 100%;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SKELETON LOADING
	   ═══════════════════════════════════════════════════════════════════════════ */

	.skeleton {
		pointer-events: none;
	}

	.skeleton-icon {
		width: 50px;
		height: 50px;
		border-radius: 50%;
		background: linear-gradient(90deg, #e0e0e0 25%, #f0f0f0 50%, #e0e0e0 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
	}

	.skeleton-text {
		width: 120px;
		height: 20px;
		margin-left: 12px;
		background: linear-gradient(90deg, #e0e0e0 25%, #f0f0f0 50%, #e0e0e0 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 4px;
	}

	.skeleton-button {
		flex-basis: 50%;
		height: 46px;
		background: linear-gradient(90deg, #e0e0e0 25%, #f0f0f0 50%, #e0e0e0 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
	}

	.skeleton-button + .skeleton-button {
		border-left: 1px solid #ededed;
	}

	@keyframes shimmer {
		0% { background-position: 200% 0; }
		100% { background-position: -200% 0; }
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media screen and (max-width: 768px) {
		.membership-card__header {
			padding: 16px;
		}

		.membership-card__icon {
			width: 44px;
			height: 44px;
		}

		.mem_div {
			font-size: 15px;
			width: calc(100% - 56px);
		}

		.membership-card__actions a {
			padding: 12px;
			font-size: 13px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   REDUCED MOTION
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (prefers-reduced-motion: reduce) {
		.membership-card,
		.membership-card__icon,
		.membership-card__actions a {
			transition: none;
		}

		.skeleton-icon,
		.skeleton-text,
		.skeleton-button {
			animation: none;
			background: #e0e0e0;
		}
	}
</style>

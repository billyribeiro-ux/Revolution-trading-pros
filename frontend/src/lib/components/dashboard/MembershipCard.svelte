<script lang="ts">
	/**
	 * Membership Card Component - Simpler Trading EXACT Match
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * 100% EXACT match to Simpler Trading WordPress structure
	 *
	 * @version 6.0.0 (Simpler Trading EXACT / December 2025)
	 */

	import '$lib/styles/st-icons.css';
	import { enterTradingRoom } from '$lib/api/trading-room-sso';

	// ═══════════════════════════════════════════════════════════════════════════
	// TYPES
	// ═══════════════════════════════════════════════════════════════════════════

	type CardType = 'options' | 'foundation' | 'moxie' | 'ww' | 'trading-room' | 'alert' | 'course' | 'indicator';
	type MembershipStatusType = 'active' | 'pending' | 'cancelled' | 'expired' | 'expiring';

	interface Props {
		id?: string;
		name: string;
		type?: CardType | string;
		slug: string;
		icon?: string;
		dashboardUrl?: string;
		roomUrl?: string;
		roomLabel?: string;
		skeleton?: boolean;
		status?: MembershipStatusType;
		daysUntilExpiry?: number;
		accessUrl?: string;
		useSSO?: boolean;
		onclick?: () => void;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// PROPS
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
		useSSO = true,
		onclick
	}: Props = $props();

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let isEnteringRoom = $state(false);

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE
	// ═══════════════════════════════════════════════════════════════════════════

	// Card modifier class - Simpler Trading uses membership-card--options, membership-card--foundation, etc.
	const cardModifier = $derived.by((): string => {
		// Map slugs to card types
		if (slug === 'mastering-the-trade') return 'membership-card--options';
		if (slug === 'simpler-showcase') return 'membership-card--foundation';
		if (slug === 'ww' || type === 'ww') return 'membership-card--ww';
		if (slug === 'moxie' || type === 'moxie') return 'membership-card--moxie';
		return `membership-card--${type}`;
	});

	// Is this the Simpler Showcase card (needs special icon styling)
	const isShowcase = $derived(slug === 'simpler-showcase');

	// Icon class based on slug
	const iconClass = $derived.by(() => {
		const iconMap: Record<string, string> = {
			'mastering-the-trade': 'st-icon-mastering-the-trade',
			'simpler-showcase': 'st-icon-simpler-showcase',
			'ww': 'st-icon-trade-of-the-week',
			'moxie': 'st-icon-moxie'
		};
		return iconMap[slug] || `st-icon-${slug}`;
	});

	// Icon size class
	const iconSizeClass = $derived(slug === 'ww' || type === 'ww' ? 'icon--md' : 'icon--lg');

	// Dashboard URL
	const finalDashboardUrl = $derived(dashboardUrl || `/dashboard/${slug}`);

	// Room/Access URL
	const finalRoomUrl = $derived(roomUrl || accessUrl || `/dashboard/${slug}/room`);

	// Room label
	const finalRoomLabel = $derived.by(() => {
		if (roomLabel) return roomLabel;
		if (slug === 'ww' || type === 'ww') return null;
		return 'Trading Room';
	});

	// Show room link
	const showRoomLink = $derived(finalRoomLabel !== null);

	// ═══════════════════════════════════════════════════════════════════════════
	// FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	async function handleEnterRoom(event: MouseEvent): Promise<void> {
		if (!useSSO || slug === 'ww' || type === 'ww') return;

		event.preventDefault();
		isEnteringRoom = true;

		try {
			await enterTradingRoom(slug);
		} catch (e) {
			console.error('[MembershipCard] Failed to enter trading room:', e);
			window.open(finalRoomUrl, '_blank', 'noopener,noreferrer');
		} finally {
			isEnteringRoom = false;
		}
	}
</script>

<!-- ═══════════════════════════════════════════════════════════════════════════
     TEMPLATE - Simpler Trading EXACT
     ═══════════════════════════════════════════════════════════════════════════ -->

{#if skeleton}
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
	<article class="membership-card {cardModifier}">
		<a href={finalDashboardUrl} class="membership-card__header">
			<span class="mem_icon">
				<span class="membership-card__icon" class:simpler-showcase-icon={isShowcase}>
					<span class="icon {iconSizeClass} {iconClass}"></span>
				</span>
			</span>
			<span class="mem_div">{name}</span>
		</a>
		<div class="membership-card__actions">
			<a href={finalDashboardUrl}>Dashboard</a>
			{#if showRoomLink}
				<a
					href={finalRoomUrl}
					target="_blank"
					rel="nofollow"
					onclick={handleEnterRoom}
					class:is-loading={isEnteringRoom}
				>
					{#if isEnteringRoom}
						Entering...
					{:else}
						{finalRoomLabel}
					{/if}
				</a>
			{/if}
		</div>
	</article>
{/if}

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES - Simpler Trading EXACT
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   MEMBERSHIP CARD - Simpler Trading EXACT
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

	/* ═══════════════════════════════════════════════════════════════════════════
	   HEADER - Simpler Trading EXACT
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
	   ICON - Simpler Trading EXACT
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
		text-align: center;
		border-radius: 50%;
		transition: all 0.15s ease-in-out;
		background-color: #0984ae;
		box-shadow: 0 10px 20px rgba(12, 36, 52, 0.25);
	}

	/* Simpler Showcase Icon - BLACK background with ORANGE icon */
	.membership-card__icon.simpler-showcase-icon {
		background: #000 !important;
	}

	.membership-card__icon.simpler-showcase-icon .icon {
		color: #f99e31 !important;
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
	   ACTIONS - Simpler Trading EXACT
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

	/* Single action (no room link) - Weekly Watchlist */
	.membership-card__actions a:only-child {
		flex-basis: 100%;
		width: 100%;
	}

	/* Loading state */
	.membership-card__actions a.is-loading {
		background-color: #f4f4f4;
		color: #0984ae;
		cursor: wait;
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
</style>

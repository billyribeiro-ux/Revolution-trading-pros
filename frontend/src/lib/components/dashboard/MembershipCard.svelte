<script lang="ts">
	/**
	 * Membership Card Component - Svelte 5 Implementation
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Displays a user's active membership/subscription with:
	 * - Svelte 5 runes ($state, $derived, $props)
	 * - Service-specific icon backgrounds (matches WordPress)
	 * - Skeleton loading states
	 * - Hover animations
	 * - Accessibility improvements
	 *
	 * @version 3.0.0 (Svelte 5 / December 2025)
	 */

	import { browser } from '$app/environment';
	import '$lib/styles/st-icons.css';

	// ═══════════════════════════════════════════════════════════════════════════
	// TYPES
	// ═══════════════════════════════════════════════════════════════════════════

	type MembershipType = 'trading-room' | 'alert-service' | 'course' | 'indicator';

	interface Props {
		id: string;
		name: string;
		type: MembershipType;
		slug: string;
		icon?: string;
		iconClass?: string;
		dashboardUrl?: string;
		roomUrl?: string;
		isLoading?: boolean;
		expiresAt?: string;
		status?: 'active' | 'expiring' | 'expired';
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// PROPS (Svelte 5 Runes)
	// ═══════════════════════════════════════════════════════════════════════════

	let {
		id,
		name,
		type,
		slug,
		icon,
		iconClass,
		dashboardUrl,
		roomUrl,
		isLoading = false,
		expiresAt,
		status = 'active'
	}: Props = $props();

	// Local state
	let isHovered = $state(false);
	let imageLoaded = $state(false);
	let imageError = $state(false);

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE
	// ═══════════════════════════════════════════════════════════════════════════

	// Generate URLs based on type
	const basePath = $derived.by(() => {
		switch (type) {
			case 'trading-room': return '/live-trading-rooms';
			case 'alert-service': return '/alerts';
			case 'course': return '/dashboard/courses';
			case 'indicator': return '/dashboard/indicators';
			default: return '/dashboard';
		}
	});

	const finalDashboardUrl = $derived(dashboardUrl || `${basePath}/${slug}/dashboard`);
	const finalRoomUrl = $derived(roomUrl || `${basePath}/${slug}`);

	// Get the second action label based on type
	const secondActionLabel = $derived.by(() => {
		switch (type) {
			case 'trading-room': return 'Trading Room';
			case 'alert-service': return 'Alerts';
			case 'course': return 'Course';
			case 'indicator': return 'Indicator';
			default: return 'View';
		}
	});

	// Get icon class based on slug
	const computedIconClass = $derived.by(() => {
		if (iconClass) return iconClass;

		const iconMap: Record<string, string> = {
			'day-trading': 'st-icon-day-trading',
			'small-accounts': 'st-icon-small-accounts',
			'spx-profit-pulse': 'st-icon-spx-profit-pulse',
			'explosive-swings': 'st-icon-explosive-swings',
			'swing-trading': 'st-icon-swing-trading'
		};

		return iconMap[slug] || `st-icon-${slug}`;
	});

	// Service-specific shadow colors
	const shadowColor = $derived.by(() => {
		const shadowMap: Record<string, string> = {
			'day-trading': 'rgba(243, 110, 27, 0.25)',
			'small-accounts': 'rgba(0, 171, 175, 0.25)',
			'spx-profit-pulse': 'rgba(12, 36, 52, 0.25)'
		};
		return shadowMap[slug] || 'rgba(9, 132, 174, 0.25)';
	});

	// Status indicator
	const statusConfig = $derived.by(() => {
		switch (status) {
			case 'expiring':
				return { color: '#f59e0b', label: 'Expiring Soon' };
			case 'expired':
				return { color: '#ef4444', label: 'Expired' };
			default:
				return { color: '#10b981', label: 'Active' };
		}
	});

	// Days until expiration
	const daysUntilExpiry = $derived.by(() => {
		if (!expiresAt) return null;
		const expiry = new Date(expiresAt);
		const now = new Date();
		const diff = Math.ceil((expiry.getTime() - now.getTime()) / (1000 * 60 * 60 * 24));
		return diff > 0 ? diff : 0;
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// HANDLERS
	// ═══════════════════════════════════════════════════════════════════════════

	function handleImageLoad(): void {
		imageLoaded = true;
	}

	function handleImageError(): void {
		imageError = true;
	}
</script>

<!-- ═══════════════════════════════════════════════════════════════════════════
     TEMPLATE
     ═══════════════════════════════════════════════════════════════════════════ -->

{#if isLoading}
	<!-- Skeleton Loading State -->
	<div class="membership-cards skeleton" aria-busy="true" aria-label="Loading membership card">
		<div class="membership-card__header">
			<span class="mbrship-card_icn skeleton-icon"></span>
			<span class="membership-card__title skeleton-text"></span>
		</div>
		<div class="membership-card__actions">
			<span class="skeleton-button"></span>
			<span class="skeleton-button"></span>
		</div>
	</div>
{:else}
	<article
		class="membership-cards {slug}-card"
		class:is-hovered={isHovered}
		class:is-expiring={status === 'expiring'}
		class:is-expired={status === 'expired'}
		onmouseenter={() => isHovered = true}
		onmouseleave={() => isHovered = false}
		role="article"
		aria-label="{name} membership"
	>
		<!-- Status Indicator -->
		{#if status !== 'active'}
			<div
				class="status-indicator"
				style:background-color={statusConfig.color}
				aria-label={statusConfig.label}
			>
				{#if daysUntilExpiry !== null && daysUntilExpiry <= 7}
					<span class="status-text">{daysUntilExpiry}d</span>
				{/if}
			</div>
		{/if}

		<!-- Header with Icon and Title -->
		<a
			href={finalRoomUrl}
			class="membership-card__header"
			aria-label="Go to {name}"
		>
			<span
				class="mbrship-card_icn {computedIconClass}"
				style:box-shadow="0 10px 20px {shadowColor}"
				aria-hidden="true"
			>
				{#if icon && !imageError}
					<img
						src={icon}
						alt=""
						class="mbrship-card_icn-img"
						class:loaded={imageLoaded}
						loading="lazy"
						onload={handleImageLoad}
						onerror={handleImageError}
					/>
				{:else}
					<i class="fa fa-graduation-cap" aria-hidden="true"></i>
				{/if}
			</span>
			<span class="membership-card__title">{name}</span>
		</a>

		<!-- Actions -->
		<div class="membership-card__actions" role="group" aria-label="Membership actions">
			<a
				href={finalDashboardUrl}
				class="action-btn action-dashboard"
				aria-label="Go to {name} Dashboard"
			>
				Dashboard
			</a>
			<a
				href={finalRoomUrl}
				class="action-btn action-primary"
				aria-label="Go to {name} {secondActionLabel}"
			>
				{secondActionLabel}
			</a>
		</div>
	</article>
{/if}

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   CSS CUSTOM PROPERTIES
	   ═══════════════════════════════════════════════════════════════════════════ */

	:root {
		--card-bg: #fff;
		--card-border: #ededed;
		--card-shadow: 0 5px 30px rgb(0 0 0 / 10%);
		--card-shadow-hover: 0 10px 40px rgb(0 0 0 / 15%);
		--card-radius: 8px;
		--icon-bg: #0984ae;
		--icon-bg-hover: #076787;
		--text-primary: #333;
		--text-link: #1e73be;
		--text-link-hover: #0984ae;
		--action-bg-hover: #f4f4f4;
		--transition-fast: 0.15s ease-in-out;
		--transition-medium: 0.25s ease-in-out;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MAIN CARD
	   ═══════════════════════════════════════════════════════════════════════════ */

	.membership-cards {
		position: relative;
		margin-top: 30px;
		background: var(--card-bg);
		border-radius: var(--card-radius);
		box-shadow: var(--card-shadow);
		transition:
			transform var(--transition-medium),
			box-shadow var(--transition-medium);
		overflow: hidden;
	}

	.membership-cards.is-hovered {
		transform: translateY(-4px);
		box-shadow: var(--card-shadow-hover);
	}

	.membership-cards.is-expiring {
		border: 2px solid #f59e0b;
	}

	.membership-cards.is-expired {
		border: 2px solid #ef4444;
		opacity: 0.8;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   STATUS INDICATOR
	   ═══════════════════════════════════════════════════════════════════════════ */

	.status-indicator {
		position: absolute;
		top: 12px;
		right: 12px;
		width: 24px;
		height: 24px;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1;
		font-size: 10px;
		font-weight: 700;
		color: white;
	}

	.status-text {
		line-height: 1;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   HEADER
	   ═══════════════════════════════════════════════════════════════════════════ */

	.membership-card__header {
		display: flex;
		align-items: center;
		padding: 20px;
		color: var(--text-primary);
		font-weight: 700;
		white-space: normal;
		transition: color var(--transition-fast);
		text-decoration: none;
		gap: 12px;
	}

	.membership-card__header:hover {
		color: #0e6ac4;
	}

	.membership-card__header:focus-visible {
		outline: 2px solid var(--icon-bg);
		outline-offset: -2px;
		border-radius: var(--card-radius) var(--card-radius) 0 0;
	}

	.membership-card__title {
		font-size: 17px;
		font-family: 'Open Sans', system-ui, sans-serif;
		line-height: 1.3;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ICON
	   ═══════════════════════════════════════════════════════════════════════════ */

	.mbrship-card_icn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 50px;
		height: 50px;
		line-height: 50px;
		color: #fff;
		text-align: center;
		border-radius: 50%;
		transition: all var(--transition-fast);
		background-color: var(--icon-bg);
		flex-shrink: 0;
		position: relative;
		overflow: hidden;
	}

	.membership-cards.is-hovered .mbrship-card_icn {
		background-color: var(--icon-bg-hover);
		transform: scale(1.05);
	}

	.mbrship-card_icn-img {
		width: 100%;
		height: 100%;
		object-fit: cover;
		border-radius: 50%;
		opacity: 0;
		transition: opacity var(--transition-fast);
	}

	.mbrship-card_icn-img.loaded {
		opacity: 1;
	}

	.mbrship-card_icn :global(i),
	.mbrship-card_icn :global(svg) {
		font-size: 24px;
		color: #fff;
	}

	/* Service-specific shadows */
	:global(.day-trading-card) .mbrship-card_icn {
		box-shadow: 0 10px 20px rgba(243, 110, 27, 0.25) !important;
	}

	:global(.small-accounts-card) .mbrship-card_icn {
		box-shadow: 0 10px 20px rgba(0, 171, 175, 0.25) !important;
	}

	:global(.spx-profit-pulse-card) .mbrship-card_icn {
		box-shadow: 0 10px 20px rgba(12, 36, 52, 0.25) !important;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ACTIONS
	   ═══════════════════════════════════════════════════════════════════════════ */

	.membership-card__actions {
		display: flex;
		font-size: 16px;
		border-top: 1px solid var(--card-border);
		justify-content: center;
	}

	.action-btn {
		display: block;
		flex: 0 0 auto;
		flex-basis: 50%;
		width: 50%;
		height: 100%;
		padding: 15px;
		text-align: center;
		color: var(--text-link);
		font-weight: 400;
		font-size: 14px;
		text-decoration: none;
		transition:
			background-color var(--transition-fast),
			color var(--transition-fast);
	}

	.action-btn:hover {
		background-color: var(--action-bg-hover);
		color: var(--text-link-hover);
	}

	.action-btn:focus-visible {
		outline: 2px solid var(--icon-bg);
		outline-offset: -2px;
	}

	.action-btn + .action-btn {
		border-left: 1px solid var(--card-border);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SKELETON LOADING
	   ═══════════════════════════════════════════════════════════════════════════ */

	.skeleton {
		pointer-events: none;
	}

	.skeleton-icon {
		background: linear-gradient(90deg, #e0e0e0 25%, #f0f0f0 50%, #e0e0e0 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
	}

	.skeleton-text {
		width: 60%;
		height: 20px;
		background: linear-gradient(90deg, #e0e0e0 25%, #f0f0f0 50%, #e0e0e0 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 4px;
	}

	.skeleton-button {
		width: 50%;
		height: 46px;
		background: linear-gradient(90deg, #e0e0e0 25%, #f0f0f0 50%, #e0e0e0 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
	}

	.skeleton-button + .skeleton-button {
		border-left: 1px solid var(--card-border);
	}

	@keyframes shimmer {
		0% { background-position: 200% 0; }
		100% { background-position: -200% 0; }
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media screen and (max-width: 768px) {
		.membership-cards {
			margin-top: 20px;
		}

		.membership-card__header {
			padding: 16px;
		}

		.mbrship-card_icn {
			width: 44px;
			height: 44px;
		}

		.membership-card__title {
			font-size: 15px;
		}

		.action-btn {
			padding: 12px;
			font-size: 13px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   REDUCED MOTION
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (prefers-reduced-motion: reduce) {
		.membership-cards,
		.mbrship-card_icn,
		.mbrship-card_icn-img,
		.action-btn {
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

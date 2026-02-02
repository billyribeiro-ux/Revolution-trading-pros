<!--
	TradingRoomHeader Component
	═══════════════════════════════════════════════════════════════════════════
	
	Reusable header component for all Trading Room dashboard pages.
	Matches WordPress reference implementation exactly.
	
	@props roomName - Display name of the trading room (e.g., "Day Trading Room")
	@props startHereUrl - URL for the "New? Start Here" button
	
	@version 1.0.0
	@author Revolution Trading Pros
-->
<script lang="ts">
	import RtpIcon from '$lib/components/icons/RtpIcon.svelte';

	// Props
	interface TradingRoom {
		name: string;
		slug?: string;
		href: string;
		icon: string;
	}

	interface Props {
		roomName: string;
		startHereUrl: string;
		pageTitle?: string;
		tradingRooms?: TradingRoom[];
	}

	let props: Props = $props();

	// Derived props with defaults
	let roomName = $derived(props.roomName);
	let startHereUrl = $derived(props.startHereUrl);
	let pageTitle = $derived(props.pageTitle);
	let tradingRooms = $derived(props.tradingRooms ?? []);

	// Use custom pageTitle if provided, otherwise default to "{roomName} Dashboard"
	let displayTitle = $derived(pageTitle || `${roomName} Dashboard`);

	// Dropdown state
	let isDropdownOpen = $state(false);

	// Use provided trading rooms or fallback to defaults
	const displayRooms = $derived(
		tradingRooms.length > 0
			? tradingRooms
			: [
					{
						name: 'Day Trading Room',
						href: '/dashboard/day-trading-room',
						icon: 'chart-line'
					},
					{
						name: 'Swing Trading Room',
						href: '/dashboard/swing-trading-room',
						icon: 'trending-up'
					},
					{
						name: 'Small Account Mentorship',
						href: '/dashboard/small-account-mentorship',
						icon: 'dollar-sign'
					}
				]
	);

	function toggleDropdown(event: Event): void {
		event.stopPropagation();
		isDropdownOpen = !isDropdownOpen;
	}

	function closeDropdown(): void {
		isDropdownOpen = false;
	}

	// Close dropdown when clicking outside
	$effect(() => {
		if (isDropdownOpen && typeof window !== 'undefined') {
			const handleClickOutside = (e: MouseEvent) => {
				const target = e.target as HTMLElement;
				if (!target.closest('.dropdown')) {
					closeDropdown();
				}
			};
			const handleEscape = (e: KeyboardEvent) => {
				if (e.key === 'Escape') {
					closeDropdown();
				}
			};
			document.addEventListener('click', handleClickOutside);
			document.addEventListener('keydown', handleEscape);
			return () => {
				document.removeEventListener('click', handleClickOutside);
				document.removeEventListener('keydown', handleEscape);
			};
		}
		return undefined;
	});
</script>

<!-- Dashboard Header - Matching Member Dashboard -->
<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">{displayTitle}</h1>
		<a href={startHereUrl} class="btn btn-xs btn-default"> New? Start Here </a>
	</div>
	<div class="dashboard__header-right">
		<div class="dropdown" class:is-open={isDropdownOpen}>
			<button
				class="btn btn-orange btn-tradingroom"
				onclick={toggleDropdown}
				aria-expanded={isDropdownOpen}
				aria-haspopup="true"
				type="button"
			>
				<strong>Enter the Trading Room</strong>
				<span class="dropdown-arrow">
					<RtpIcon name="chevron-down" size={14} />
				</span>
			</button>

			{#if isDropdownOpen}
				<div class="dropdown-menu" role="menu">
					{#each displayRooms as room}
						<a href={room.href} class="dropdown-item" onclick={closeDropdown} role="menuitem">
							<span class="dropdown-item__icon">
								<RtpIcon name={room.icon} size={20} />
							</span>
							<span class="dropdown-item__text">{room.name}</span>
						</a>
					{/each}
				</div>
			{/if}
		</div>

		<!-- Trading Room Rules - Legal Compliance -->
		<div class="trading-room-rules">
			<a
				href="/trading-room-rules.pdf"
				target="_blank"
				rel="noopener noreferrer"
				class="trading-room-rules__link"
			>
				Trading Room Rules
			</a>
			<p class="trading-room-rules__disclaimer">
				By logging into any of our Live Trading Rooms, You are agreeing to our Rules of the Room.
			</p>
		</div>
	</div>
</header>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * DASHBOARD HEADER - 2026 Mobile-First Responsive Design
	 * ═══════════════════════════════════════════════════════════════════════════
	 * Breakpoints: xs(360px), sm(640px), md(768px), lg(1024px), xl(1280px)
	 * Touch Targets: 44x44px minimum
	 * Safe Areas: env(safe-area-inset-*) for notched devices
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Mobile-First Base - Full width, stacked layout */
	.dashboard__header {
		display: flex;
		flex-direction: column;
		align-items: stretch;
		gap: 16px;
		background-color: #fff;
		border-bottom: 1px solid #dbdbdb;
		padding: 16px;
		padding-top: max(16px, env(safe-area-inset-top, 16px));
		padding-left: max(16px, env(safe-area-inset-left, 16px));
		padding-right: max(16px, env(safe-area-inset-right, 16px));
		margin: 0 !important;
		width: 100% !important;
		max-width: none !important;
		box-sizing: border-box;
	}

	.dashboard__header-left {
		display: flex;
		flex-direction: column;
		align-items: flex-start;
		gap: 12px;
		width: 100%;
	}

	.dashboard__page-title {
		margin: 0;
		color: #333;
		font-size: 24px;
		font-weight: 600;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		line-height: 1.2;
		word-wrap: break-word;
	}

	.dashboard__header-right {
		display: flex;
		flex-direction: column;
		align-items: stretch;
		gap: 12px;
		width: 100%;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * RESPONSIVE BREAKPOINTS - Progressive Enhancement
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* xs: 360px+ - Small phones */
	@media (min-width: 360px) {
		.dashboard__header {
			padding: 20px;
		}

		.dashboard__page-title {
			font-size: 26px;
		}
	}

	/* sm: 640px+ - Large phones / small tablets */
	@media (min-width: 640px) {
		.dashboard__header {
			flex-direction: row;
			flex-wrap: wrap;
			align-items: center;
			justify-content: space-between;
		}

		.dashboard__header-left {
			flex-direction: row;
			align-items: center;
			flex: 1 1 auto;
			width: auto;
		}

		.dashboard__header-right {
			flex-direction: column;
			align-items: flex-end;
			width: auto;
		}

		.dashboard__page-title {
			font-size: 28px;
		}
	}

	/* md: 768px+ - Tablets */
	@media (min-width: 768px) {
		.dashboard__header {
			padding: 24px;
		}

		.dashboard__page-title {
			font-size: 32px;
		}
	}

	/* lg: 1024px+ - Desktop */
	@media (min-width: 1024px) {
		.dashboard__header {
			padding: 28px;
			border-right: 1px solid #dbdbdb;
		}

		.dashboard__page-title {
			font-size: 34px;
			font-weight: 400;
		}
	}

	/* xl: 1280px+ - Large desktop */
	@media (min-width: 1280px) {
		.dashboard__header {
			padding: 30px;
		}

		.dashboard__page-title {
			font-size: 36px;
		}
	}

	/* xxl: 1440px+ - Extra large desktop */
	@media (min-width: 1440px) {
		.dashboard__header {
			padding: 30px 40px;
		}
	}

	/* 1920px+ - Ultra wide */
	@media (min-width: 1920px) {
		.dashboard__header {
			padding: 40px 60px;
		}
	}

	/* Button Styles - 2026 Touch-Friendly (44x44px minimum) */
	.btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		min-height: 44px;
		min-width: 44px;
		padding: 12px 20px;
		font-size: 14px;
		font-weight: 600;
		text-align: center;
		text-decoration: none;
		border-radius: 8px;
		transition: all 0.15s ease-in-out;
		cursor: pointer;
		border: none;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		-webkit-tap-highlight-color: transparent;
		touch-action: manipulation;
	}

	.btn-xs {
		padding: 10px 16px;
		font-size: 13px;
		min-height: 44px;
	}

	.btn-default {
		background-color: #f4f4f4;
		color: #0a84ae;
		border: 1px solid #f4f4f4;
	}

	.btn-default:hover {
		background-color: #d0d0d0;
		color: #0a84ae;
	}

	.btn-default:active,
	.btn-default:focus {
		background-color: #e7e7e7;
		border: 2px solid #c9a0dc;
		outline: none;
		outline-offset: 2px;
		color: #0a84ae;
	}

	.btn-default:focus-visible {
		outline: 2px solid #0a84ae;
		outline-offset: 2px;
	}

	/* Enter Trading Room Button - Touch-Friendly */
	.btn-orange {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		min-height: 48px;
		padding: 12px 20px;
		background-color: #f69532;
		color: #fff;
		font-size: 14px;
		font-weight: 600;
		text-decoration: none;
		border: none;
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.15s ease-in-out;
		-webkit-tap-highlight-color: transparent;
		touch-action: manipulation;
	}

	.btn-orange:hover {
		background-color: #dc7309;
	}

	.btn-orange:focus-visible {
		outline: 2px solid #fff;
		outline-offset: 2px;
		box-shadow: 0 0 0 4px rgba(246, 149, 50, 0.5);
	}

	.btn-orange strong {
		font-weight: 700;
	}

	.btn-tradingroom {
		text-transform: none;
		width: 100%;
		max-width: 280px;
		padding: 14px 20px;
	}

	@media (min-width: 640px) {
		.btn-tradingroom {
			width: 280px;
		}
	}

	.dropdown-arrow {
		font-size: 10px;
		transition: transform 0.15s ease-in-out;
		display: flex;
		align-items: center;
	}

	.dropdown.is-open .dropdown-arrow {
		transform: rotate(180deg);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * DROPDOWN MENU - WordPress Exact Match
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dropdown {
		position: relative;
		display: inline-block;
	}

	.dropdown-menu {
		position: absolute;
		top: 100%;
		right: 0;
		padding: 15px;
		min-width: 260px;
		max-width: 280px;
		margin: 5px 0 0;
		font-size: 14px;
		background-color: #ffffff;
		border: none;
		border-radius: 5px;
		box-shadow: 0 10px 30px rgba(0, 0, 0, 0.15);
		z-index: 1000;
		opacity: 1;
		visibility: visible;
		transform: translateY(0);
		transition: all 0.15s ease-in-out;
	}

	.dropdown:not(.is-open) .dropdown-menu {
		opacity: 0;
		visibility: hidden;
		transform: translateY(-5px);
		pointer-events: none;
	}

	/* Dropdown Items - Touch-Friendly (44px minimum) */
	.dropdown-item {
		display: flex;
		align-items: center;
		gap: 10px;
		min-height: 48px;
		padding: 12px 16px;
		color: #666;
		font-size: 14px;
		font-weight: 400;
		text-decoration: none;
		transition: background-color 0.15s ease-in-out;
		border-radius: 8px;
		white-space: nowrap;
		-webkit-tap-highlight-color: transparent;
		touch-action: manipulation;
	}

	.dropdown-item:hover {
		background-color: #f4f4f4;
	}

	.dropdown-item:focus-visible {
		outline: 2px solid #143e59;
		outline-offset: -2px;
	}

	.dropdown-item__icon {
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
		color: #143e59;
		width: 24px;
		height: 24px;
	}

	.dropdown-item__text {
		flex: 1;
	}

	/* Trading Room Rules - Legal Compliance - Mobile-First */
	.trading-room-rules {
		text-align: center;
		margin-top: 12px;
		width: 100%;
		padding: 0 8px;
	}

	.trading-room-rules__link {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		min-height: 44px;
		margin-bottom: 8px;
		padding: 8px 16px;
		font-size: 16px;
		font-weight: 700;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		color: #1e73be;
		text-decoration: none;
		transition: color 0.15s ease-in-out;
		-webkit-tap-highlight-color: transparent;
		touch-action: manipulation;
	}

	.trading-room-rules__link:hover {
		color: #0984ae;
		text-decoration: underline;
	}

	.trading-room-rules__link:focus-visible {
		outline: 2px solid #1e73be;
		outline-offset: 2px;
		border-radius: 4px;
	}

	.trading-room-rules__disclaimer {
		margin: 0;
		font-size: 12px;
		font-weight: 400;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		color: #666;
		line-height: 1.5;
		text-align: center;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * MOBILE ENHANCEMENTS
	 * ═══════════════════════════════════════════════════════════════════════════ */

	@media (min-width: 640px) {
		.trading-room-rules {
			max-width: 300px;
			margin-left: auto;
			margin-right: 0;
		}

		.trading-room-rules__link {
			font-size: 18px;
		}

		.trading-room-rules__disclaimer {
			font-size: 13px;
		}
	}

	/* Landscape on mobile - condensed layout */
	@media (max-width: 767px) and (orientation: landscape) {
		.dashboard__header {
			padding: 12px 20px;
		}

		.dashboard__page-title {
			font-size: 20px;
		}

		.trading-room-rules__disclaimer {
			font-size: 11px;
		}
	}

	/* High contrast / reduced motion preferences */
	@media (prefers-reduced-motion: reduce) {
		.btn,
		.btn-orange,
		.dropdown-item,
		.dropdown-menu,
		.trading-room-rules__link {
			transition: none;
		}

		.dropdown-arrow {
			transition: none;
		}
	}
</style>

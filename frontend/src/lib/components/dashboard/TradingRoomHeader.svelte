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
	interface Props {
		roomName: string;
		startHereUrl: string;
		pageTitle?: string; // Optional custom page title (defaults to "{roomName} Dashboard")
	}

	let { roomName, startHereUrl, pageTitle }: Props = $props();
	
	// Use custom pageTitle if provided, otherwise default to "{roomName} Dashboard"
	let displayTitle = $derived(pageTitle || `${roomName} Dashboard`);

	// Dropdown state
	let isDropdownOpen = $state(false);

	// Trading rooms for dropdown
	const tradingRooms = [
		{
			name: 'Day Trading Room',
			href: '#', // TODO: Provide URL
			icon: 'chart-line'
		},
		{
			name: 'Swing Trading Room',
			href: '#', // TODO: Provide URL
			icon: 'trending-up'
		},
		{
			name: 'Small Accounts Mentorship',
			href: '#', // TODO: Provide URL
			icon: 'dollar-sign'
		}
	];

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
		<a href={startHereUrl} class="btn btn-xs btn-default">
			New? Start Here
		</a>
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
					{#each tradingRooms as room}
						<a 
							href={room.href} 
							class="dropdown-item" 
							onclick={closeDropdown}
							role="menuitem"
						>
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
	 * DASHBOARD HEADER - WordPress Exact Match
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__header {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		background-color: #fff;
		border-bottom: 1px solid #dbdbdb;
		border-right: 1px solid #dbdbdb;
		padding: 20px;
	}

	@media (min-width: 1280px) {
		.dashboard__header {
			padding: 30px;
		}
	}

	@media (min-width: 1440px) {
		.dashboard__header {
			padding: 30px 40px;
		}
	}

	.dashboard__header-left {
		display: flex;
		align-items: center;
		justify-content: flex-start;
		flex: 1;
		gap: 15px;
	}

	.dashboard__page-title {
		margin: 0;
		color: #333;
		font-size: 36px;
		font-weight: 400;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
	}

	.dashboard__header-right {
		display: flex;
		flex-direction: column;
		align-items: flex-end;
		margin-top: 10px;
	}

	@media (min-width: 820px) {
		.dashboard__header-right {
			flex-direction: column;
			margin-top: 0;
		}
	}

	/* Button Styles */
	.btn {
		display: inline-block;
		padding: 10px 20px;
		font-size: 14px;
		font-weight: 600;
		text-align: center;
		text-decoration: none;
		border-radius: 4px;
		transition: all 0.15s ease-in-out;
		cursor: pointer;
		border: none;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
	}

	.btn-xs {
		padding: 6px 12px;
		font-size: 12px;
		height: 40px;
		display: inline-flex;
		align-items: center;
	}

	.btn-default {
		background-color: #F4F4F4;
		color: #0A84AE;
		border: 1px solid #F4F4F4;
	}

	.btn-default:hover {
		background-color: #d0d0d0;
		color: #0A84AE;
	}

	.btn-default:active,
	.btn-default:focus {
		background-color: #e7e7e7;
		border: 3px solid #c9a0dc;
		outline: none;
		color: #0A84AE;
	}

	/* Enter Trading Room Button */
	.btn-orange {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		padding: 10px 20px;
		background-color: #f69532;
		color: #fff;
		font-size: 14px;
		font-weight: 600;
		text-decoration: none;
		border: none;
		border-radius: 4px;
		cursor: pointer;
		transition: all 0.15s ease-in-out;
	}

	.btn-orange:hover {
		background-color: #dc7309;
	}

	.btn-orange strong {
		font-weight: 700;
	}

	.btn-tradingroom {
		text-transform: none;
		width: 280px;
		padding: 12px 18px;
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

	.dropdown-item {
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 15px 20px;
		color: #666;
		font-size: 14px;
		font-weight: 400;
		text-decoration: none;
		transition: background-color 0.15s ease-in-out;
		border-radius: 5px;
		white-space: nowrap;
	}

	.dropdown-item:hover {
		background-color: #f4f4f4;
	}

	.dropdown-item__icon {
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
		color: #143E59;
	}

	.dropdown-item__text {
		flex: 1;
	}

	/* Trading Room Rules - Legal Compliance */
	.trading-room-rules {
		text-align: center;
		margin-top: 10px;
		width: 100%;
		max-width: 300px;
		margin-left: auto;
		margin-right: auto;
	}

	.trading-room-rules__link {
		display: block;
		margin-bottom: 8px;
		font-size: 18px;
		font-weight: 700;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		color: #1e73be;
		text-decoration: none;
		transition: color 0.15s ease-in-out;
		text-align: center;
	}

	.trading-room-rules__link:hover {
		color: #0984ae;
		text-decoration: underline;
	}

	.trading-room-rules__disclaimer {
		margin: 0;
		font-size: 13px;
		font-weight: 400;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		color: #666;
		line-height: 1.4;
		text-align: center;
	}

	@media (max-width: 768px) {
		.dashboard__header {
			padding: 20px;
			flex-direction: column;
			align-items: flex-start;
			gap: 15px;
		}

		.dashboard__page-title {
			font-size: 24px;
		}

		.dashboard__header-right {
			width: 100%;
			align-items: stretch;
		}

		.btn-tradingroom {
			width: 100%;
		}

		.trading-room-rules {
			max-width: 100%;
		}
	}
</style>

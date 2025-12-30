<script lang="ts">
	/**
	 * DashboardHeader - Page header with title and actions
	 *
	 * Svelte 5 Runes:
	 * - $props() for component props
	 * - $state() for dropdown state
	 *
	 * Features:
	 * - Page title display
	 * - Trading room quick access dropdown
	 * - Mobile menu toggle
	 */

	import IconMenu2 from '@tabler/icons-svelte/icons/menu-2';
	import IconChevronDown from '@tabler/icons-svelte/icons/chevron-down';
	import type { Snippet } from 'svelte';

	interface TradingRoom {
		name: string;
		href: string;
		icon?: string;
	}

	interface Props {
		title: string;
		tradingRooms?: TradingRoom[];
		showMobileToggle?: boolean;
		onMobileToggle?: () => void;
		children?: Snippet;
	}

	let {
		title,
		tradingRooms = [],
		showMobileToggle = true,
		onMobileToggle,
		children
	}: Props = $props();

	// Dropdown state
	let isDropdownOpen = $state(false);

	function toggleDropdown() {
		isDropdownOpen = !isDropdownOpen;
	}

	function closeDropdown() {
		isDropdownOpen = false;
	}

	// Close dropdown on click outside
	function handleClickOutside(event: MouseEvent) {
		const target = event.target as HTMLElement;
		if (!target.closest('.dropdown')) {
			closeDropdown();
		}
	}
</script>

<svelte:window onclick={handleClickOutside} />

<header class="dashboard__header">
	<div class="dashboard__header-left">
		<!-- Mobile Menu Toggle -->
		{#if showMobileToggle}
			<button
				class="mobile-menu-toggle"
				onclick={onMobileToggle}
				aria-label="Toggle menu"
			>
				<IconMenu2 size={24} />
			</button>
		{/if}

		<h1 class="dashboard__page-title">{title}</h1>
	</div>

	<div class="dashboard__header-right">
		<!-- Custom content slot -->
		{#if children}
			{@render children()}
		{/if}

		<!-- Trading Room Rules -->
		{#if tradingRooms.length > 0}
			<ul class="trading-room-rules">
				<li>
					<a
						href="https://cdn.simplertrading.com/2024/02/07192341/Simpler-Tradings-Rules-of-the-Room.pdf"
						target="_blank"
						rel="noopener noreferrer"
						class="btn-link"
					>
						Trading Room Rules
					</a>
				</li>
				<li class="rules-disclaimer">
					By logging into any of our Live Trading Rooms, You are agreeing to our Rules of the Room.
				</li>
			</ul>
		{/if}

		<!-- Trading Room Dropdown -->
		{#if tradingRooms.length > 0}
			<div class="dropdown" class:is-open={isDropdownOpen}>
				<button
					class="btn-orange btn-tradingroom dropdown-toggle"
					onclick={toggleDropdown}
					aria-expanded={isDropdownOpen}
					aria-haspopup="true"
				>
					<strong>Enter a Trading Room</strong>
					<IconChevronDown size={16} />
				</button>

				<nav class="dropdown-menu" aria-label="Trading rooms">
					<ul class="dropdown-menu__menu">
						{#each tradingRooms as room}
							<li>
								<a
									href={room.href}
									target="_blank"
									rel="noopener noreferrer nofollow"
									onclick={closeDropdown}
								>
									<span class="room-icon">
										<IconChevronDown size={20} />
									</span>
									{room.name}
								</a>
							</li>
						{/each}
					</ul>
				</nav>
			</div>
		{/if}
	</div>
</header>

<style>
	.mobile-menu-toggle {
		display: none;
		padding: 8px;
		background: transparent;
		border: none;
		color: var(--dashboard-text-primary);
		cursor: pointer;
		border-radius: 6px;
		transition: background-color var(--dashboard-transition);
	}

	.mobile-menu-toggle:hover {
		background-color: var(--dashboard-nav-hover);
	}

	@media (max-width: 991px) {
		.mobile-menu-toggle {
			display: flex;
		}
	}

	.trading-room-rules {
		list-style: none;
		margin: 0;
		padding: 0;
		text-align: right;
	}

	.btn-link {
		font-size: 13px;
		font-weight: 700;
		color: var(--dashboard-text-primary);
		text-decoration: none;
	}

	.btn-link:hover {
		text-decoration: underline;
	}

	.rules-disclaimer {
		font-size: 11px;
		color: var(--dashboard-text-muted);
		margin-top: 4px;
	}

	.dropdown-toggle {
		display: inline-flex;
		align-items: center;
		gap: 8px;
	}

	.room-icon {
		width: 24px;
		height: 24px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	@media (max-width: 768px) {
		.trading-room-rules {
			display: none;
		}
	}
</style>

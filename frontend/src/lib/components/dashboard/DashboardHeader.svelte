<script lang="ts">
	/**
	 * DashboardHeader - Simpler Trading Style
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Svelte 5 component for the dashboard header:
	 * - Page title
	 * - Trading Room Rules link
	 * - Orange "Enter a Trading Room" dropdown button
	 *
	 * @version 1.0.0 - Svelte 5 with $props()
	 */
	import type { Snippet } from 'svelte';

	interface TradingRoom {
		id: number | string;
		name: string;
		slug: string;
		roomLabel?: string;
	}

	interface Props {
		/** Page title */
		title: string;
		/** Trading rooms for dropdown */
		tradingRooms?: TradingRoom[];
		/** Whether to show trading room rules */
		showRules?: boolean;
		/** Custom right content */
		rightContent?: Snippet;
	}

	let {
		title,
		tradingRooms = [],
		showRules = true,
		rightContent
	}: Props = $props();

	let dropdownOpen = $state(false);

	function toggleDropdown(e: MouseEvent) {
		e.preventDefault();
		e.stopPropagation();
		dropdownOpen = !dropdownOpen;
	}

	function closeDropdown() {
		dropdownOpen = false;
	}
</script>

<svelte:window onclick={closeDropdown} />

<header class="header">
	<div class="header__left">
		<h1 class="header__title">{title}</h1>
	</div>

	<div class="header__right">
		{#if rightContent}
			{@render rightContent()}
		{:else}
			<!-- Trading Room Rules -->
			{#if showRules}
				<ul class="trading-rules">
					<li class="trading-rules__link">
						<a href="https://cdn.simplertrading.com/2024/02/07192341/Simpler-Tradings-Rules-of-the-Room.pdf" target="_blank" rel="noopener noreferrer">
							Trading Room Rules
						</a>
					</li>
					<li class="trading-rules__disclaimer">
						By logging into any of our Live Trading Rooms, You are agreeing to our Rules of the Room.
					</li>
				</ul>
			{/if}

			<!-- Trading Room Dropdown -->
			{#if tradingRooms.length > 0}
				<div class="dropdown" class:is-open={dropdownOpen}>
					<button
						type="button"
						class="dropdown__toggle"
						onclick={toggleDropdown}
						aria-expanded={dropdownOpen}
					>
						<strong>Enter a Trading Room</strong>
					</button>

					{#if dropdownOpen}
						<nav class="dropdown__menu">
							<ul>
								{#each tradingRooms as room (room.id)}
									<li>
										<a href="/dashboard/{room.slug}/" target="_blank" rel="noopener noreferrer">
											<span class="dropdown__icon">
												<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
													<path d="M9 5v4"></path>
													<rect width="4" height="6" x="7" y="9" rx="1"></rect>
													<path d="M9 15v2"></path>
													<path d="M17 3v2"></path>
													<rect width="4" height="8" x="15" y="5" rx="1"></rect>
													<path d="M17 13v3"></path>
													<path d="M3 3v18h18"></path>
												</svg>
											</span>
											{room.roomLabel || room.name}
										</a>
									</li>
								{/each}
							</ul>
						</nav>
					{/if}
				</div>
			{/if}
		{/if}
	</div>
</header>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD HEADER - Exact Simpler Trading Match
	   ═══════════════════════════════════════════════════════════════════════════ */
	.header {
		background-color: #fff;
		border-bottom: 1px solid #dbdbdb;
		padding: 20px;
		display: flex;
		flex-wrap: wrap;
		justify-content: space-between;
		align-items: center;
	}

	@media (min-width: 1280px) {
		.header { padding: 30px; }
	}

	@media (min-width: 1440px) {
		.header { padding: 30px 40px; }
	}

	/* Left Side */
	.header__left {
		display: flex;
		align-items: center;
	}

	.header__title {
		margin: 0;
		color: #333;
		font-size: 36px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		line-height: 1.2;
	}

	/* Right Side */
	.header__right {
		display: flex;
		align-items: center;
		gap: 10px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   TRADING ROOM RULES
	   ═══════════════════════════════════════════════════════════════════════════ */
	.trading-rules {
		list-style: none;
		margin: 0;
		padding: 0;
		text-align: right;
	}

	.trading-rules__link a {
		font-weight: 700;
		font-size: 12px;
		color: #0984ae;
		text-decoration: none;
		transition: color 0.15s ease-in-out;
	}

	.trading-rules__link a:hover {
		text-decoration: underline;
		color: #065a75;
	}

	.trading-rules__disclaimer {
		font-size: 11px;
		color: #666;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DROPDOWN
	   ═══════════════════════════════════════════════════════════════════════════ */
	.dropdown {
		position: relative;
		display: inline-block;
	}

	.dropdown__toggle {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 6px;
		background: #F69532;
		color: #fff;
		border: none;
		border-radius: 4px;
		padding: 12px 18px;
		width: 280px;
		font-size: 14px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		cursor: pointer;
		box-shadow: 0 2px 5px rgba(0, 0, 0, 0.16);
		transition: background-color 0.2s ease-in-out;
	}

	.dropdown__toggle:hover {
		background: #dc7309;
	}

	/* Dropdown caret */
	.dropdown__toggle::after {
		content: "";
		display: inline-block;
		margin-left: 0.255em;
		border-top: 0.3em solid;
		border-right: 0.3em solid transparent;
		border-left: 0.3em solid transparent;
	}

	/* Dropdown Menu */
	.dropdown__menu {
		position: absolute;
		top: 100%;
		right: 0;
		margin-top: 5px;
		background: #fff;
		border-radius: 4px;
		box-shadow: 0 10px 30px rgba(0, 0, 0, 0.15);
		min-width: 260px;
		padding: 20px;
		z-index: 1000;
	}

	.dropdown__menu ul {
		list-style: none;
		margin: -10px;
		padding: 0;
	}

	.dropdown__menu a {
		display: flex;
		align-items: center;
		padding: 10px 15px;
		color: #666;
		font-size: 14px;
		font-family: 'Open Sans', sans-serif;
		text-decoration: none;
		white-space: nowrap;
		transition: all 0.15s ease-in-out;
	}

	.dropdown__menu a:hover {
		background-color: #f4f4f4;
		color: #0984ae;
	}

	.dropdown__icon {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 24px;
		height: 24px;
		margin-right: 10px;
		color: #999;
		transition: color 0.15s ease-in-out;
	}

	.dropdown__menu a:hover .dropdown__icon {
		color: #0984ae;
	}

	.dropdown__icon svg {
		width: 20px;
		height: 20px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════════ */
	@media (max-width: 820px) {
		.header {
			flex-direction: column;
			align-items: flex-start;
			gap: 15px;
		}

		.header__right {
			flex-direction: column;
			align-items: flex-start;
			width: 100%;
		}

		.trading-rules {
			text-align: left;
		}

		.dropdown__toggle {
			width: 100%;
		}
	}

	@media (max-width: 576px) {
		.header__title {
			font-size: 28px;
		}
	}
</style>

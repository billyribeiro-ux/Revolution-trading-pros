<!--
	TradingRoomDropdown - Reusable Trading Room Entry Button with Dropdown
	WordPress Match: Lines 2963-2975 (premium-daily-videos)
	
	Svelte 5 Component - Reusable across all dashboard pages
	@version 1.0.0
-->
<script lang="ts">
	import { onMount } from 'svelte';

	interface TradingRoom {
		name: string;
		icon: string;
		url: string;
	}

	// Trading rooms list - matches WordPress structure
	const tradingRooms: TradingRoom[] = [
		{
			name: 'Day Trading Room',
			icon: 'st-icon-day-trading-room',
			url: '/trading-rooms/day-trading-room'
		},
		{
			name: 'Swing Trading Room',
			icon: 'st-icon-swing-trading-room',
			url: '/trading-rooms/swing-trading-room'
		},
		{
			name: 'Small Account Mentorship',
			icon: 'st-icon-small-account',
			url: '/trading-rooms/small-account-mentorship'
		},
		{
			name: 'Explosive Swings',
			icon: 'st-icon-explosive-swings',
			url: '/trading-rooms/explosive-swings'
		},
		{
			name: 'SPX Profit Pulse',
			icon: 'st-icon-spx-profit-pulse',
			url: '/trading-rooms/spx-profit-pulse'
		}
	];

	// Svelte 5 reactive state
	let isOpen = $state(false);

	// Toggle dropdown
	function toggleDropdown() {
		isOpen = !isOpen;
	}

	// Close dropdown when clicking outside
	function handleClickOutside(event: MouseEvent) {
		const target = event.target as HTMLElement;
		const dropdown = document.querySelector('.dropdown');
		
		if (dropdown && !dropdown.contains(target)) {
			isOpen = false;
		}
	}

	// Add click outside listener on mount
	onMount(() => {
		const handleClick = (event: MouseEvent) => {
			if (isOpen) {
				handleClickOutside(event);
			}
		};
		
		document.addEventListener('click', handleClick);
		
		return () => {
			document.removeEventListener('click', handleClick);
		};
	});
</script>

<div class="dropdown display-inline-block">
	<button
		type="button"
		class="btn btn-xs btn-orange btn-tradingroom dropdown-toggle"
		class:active={isOpen}
		onclick={toggleDropdown}
		aria-expanded={isOpen}
		aria-haspopup="true"
	>
		<strong>Enter a Trading Room</strong>
		<span class="dropdown-chevron" class:open={isOpen}>
			<svg viewBox="0 0 330 512" aria-hidden="true" role="img" width="1em" height="1em">
				<path d="M305.913 197.085c0 2.266-1.133 4.815-2.833 6.514L171.087 335.593c-1.7 1.7-4.249 2.832-6.515 2.832s-4.815-1.133-6.515-2.832L26.064 203.599c-1.7-1.7-2.832-4.248-2.832-6.514s1.132-4.816 2.832-6.515l14.162-14.163c1.7-1.699 3.966-2.832 6.515-2.832 2.266 0 4.815 1.133 6.515 2.832l111.316 111.317 111.316-111.317c1.7-1.699 4.249-2.832 6.515-2.832s4.815 1.133 6.515 2.832l14.162 14.163c1.7 1.7 2.833 4.249 2.833 6.515z" fill-rule="nonzero"/>
			</svg>
		</span>
	</button>
	
	{#if isOpen}
		<nav class="dropdown-menu dropdown-menu--full-width" aria-label="Trading Rooms">
			<ul class="dropdown-menu__menu">
				{#each tradingRooms as room}
					<li>
						<a href={room.url} target="_blank" rel="nofollow">
							<span class="{room.icon} icon icon--md"></span>
							{room.name}
						</a>
					</li>
				{/each}
			</ul>
		</nav>
	{/if}
</div>

<style>
	/* Dropdown Container */
	.dropdown {
		position: relative;
		display: inline-block;
	}

	/* Button Styling - WordPress Match */
	.btn-tradingroom {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 8px 16px;
		background: #F69532;
		color: #fff;
		border: none;
		border-radius: 4px;
		font-family: 'Montserrat', sans-serif;
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s ease;
		white-space: nowrap;
	}

	.btn-tradingroom:hover {
		background: #dc7309;
	}

	.btn-tradingroom.active {
		background: #dc7309;
	}

	/* Chevron Icon */
	.dropdown-chevron {
		display: inline-flex;
		align-items: center;
		margin-left: 4px;
		transition: transform 0.2s ease;
	}

	.dropdown-chevron.open {
		transform: rotate(180deg);
	}

	.dropdown-chevron svg {
		fill: currentColor;
	}

	/* Dropdown Menu */
	.dropdown-menu {
		position: absolute;
		top: calc(100% + 5px);
		right: 0;
		min-width: 280px;
		background: #fff;
		border: 1px solid #e6e6e6;
		border-radius: 4px;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
		z-index: 1000;
		animation: dropdownFadeIn 0.2s ease;
	}

	@keyframes dropdownFadeIn {
		from {
			opacity: 0;
			transform: translateY(-10px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	.dropdown-menu__menu {
		list-style: none;
		margin: 0;
		padding: 8px 0;
	}

	.dropdown-menu__menu li {
		margin: 0;
	}

	.dropdown-menu__menu a {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 12px 20px;
		color: #333;
		text-decoration: none;
		font-family: 'Montserrat', sans-serif;
		font-size: 14px;
		font-weight: 500;
		transition: background-color 0.2s ease;
	}

	.dropdown-menu__menu a:hover {
		background-color: #f5f5f5;
	}

	/* Icon Styling */
	.icon {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 24px;
		height: 24px;
		font-size: 20px;
		color: #F69532;
	}

	.icon--md {
		width: 24px;
		height: 24px;
	}

	/* Responsive */
	@media (max-width: 767px) {
		.dropdown-menu {
			right: auto;
			left: 0;
			min-width: 240px;
		}

		.btn-tradingroom {
			font-size: 13px;
			padding: 6px 12px;
		}

		.dropdown-menu__menu a {
			padding: 10px 16px;
			font-size: 13px;
		}
	}
</style>

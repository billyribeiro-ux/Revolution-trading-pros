<script lang="ts">
	/**
	 * Trading Room Dropdown Component
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Dropdown menu for quick access to trading rooms with JWT SSO.
	 * Matches Simpler Trading's "Enter a Trading Room" dropdown functionality.
	 *
	 * @version 1.0.0 - December 2025
	 */

	import IconChevronDown from '@tabler/icons-svelte/icons/chevron-down';
	import IconDoorEnter from '@tabler/icons-svelte/icons/door-enter';
	import IconLock from '@tabler/icons-svelte/icons/lock';
	import IconLoader from '@tabler/icons-svelte/icons/loader';
	import {
		getAccessibleRooms,
		enterTradingRoom,
		getMembershipStatusLabel,
		type AccessibleRoom
	} from '$lib/api/trading-room-sso';

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let isOpen = $state(false);
	let isLoading = $state(false);
	let rooms = $state<AccessibleRoom[]>([]);
	let enteringRoom = $state<string | null>(null);
	let error = $state<string | null>(null);

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED
	// ═══════════════════════════════════════════════════════════════════════════

	const accessibleRooms = $derived(rooms.filter((r) => r.has_access));
	const hasAccess = $derived(accessibleRooms.length > 0);

	// ═══════════════════════════════════════════════════════════════════════════
	// FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	async function loadRooms(): Promise<void> {
		if (rooms.length > 0) return; // Already loaded

		isLoading = true;
		error = null;

		try {
			const response = await getAccessibleRooms();
			if (response.success) {
				rooms = response.data.rooms;
			}
		} catch (e) {
			error = 'Failed to load rooms';
			console.error('[TradingRoomDropdown] Error loading rooms:', e);
		} finally {
			isLoading = false;
		}
	}

	function toggleDropdown(): void {
		isOpen = !isOpen;
		if (isOpen) {
			loadRooms();
		}
	}

	function closeDropdown(): void {
		isOpen = false;
	}

	async function handleEnterRoom(room: AccessibleRoom): Promise<void> {
		if (!room.has_access) return;

		enteringRoom = room.slug;

		try {
			await enterTradingRoom(room.slug);
			closeDropdown();
		} catch (e) {
			console.error('[TradingRoomDropdown] Error entering room:', e);
			error = 'Failed to enter trading room';
		} finally {
			enteringRoom = null;
		}
	}

	function handleKeydown(event: KeyboardEvent): void {
		if (event.key === 'Escape') {
			closeDropdown();
		}
	}

	function handleClickOutside(event: MouseEvent): void {
		const target = event.target as HTMLElement;
		if (!target.closest('.trading-room-dropdown')) {
			closeDropdown();
		}
	}

	// Close on click outside
	$effect(() => {
		if (isOpen) {
			document.addEventListener('click', handleClickOutside);
			document.addEventListener('keydown', handleKeydown);
		}
		return () => {
			document.removeEventListener('click', handleClickOutside);
			document.removeEventListener('keydown', handleKeydown);
		};
	});
</script>

<!-- ═══════════════════════════════════════════════════════════════════════════
     TEMPLATE - WordPress EXACT Match
     ═══════════════════════════════════════════════════════════════════════════ -->

<!-- WordPress: .dropdown.display-inline-block -->
<div class="dropdown display-inline-block">
	<!-- WordPress: .btn.btn-xs.btn-orange.btn-tradingroom.dropdown-toggle -->
	<a
		href="#"
		class="btn btn-xs btn-orange btn-tradingroom dropdown-toggle"
		id="dLabel"
		role="button"
		aria-expanded={isOpen}
		onclick={(e) => { e.preventDefault(); toggleDropdown(); }}
	>
		<strong>Enter a Trading Room</strong>
	</a>

	<!-- WordPress: .dropdown-menu.dropdown-menu--full-width -->
	{#if isOpen}
		<nav class="dropdown-menu dropdown-menu--full-width" aria-labelledby="dLabel">
			{#if isLoading}
				<div class="dropdown-loading">
					<IconLoader size={20} class="spin" />
					<span>Loading rooms...</span>
				</div>
			{:else if error}
				<div class="dropdown-error">
					<span>{error}</span>
					<button class="retry-btn" onclick={loadRooms}>Retry</button>
				</div>
			{:else if rooms.length === 0}
				<div class="dropdown-empty">
					<IconLock size={20} />
					<span>No trading rooms available</span>
				</div>
			{:else}
				<!-- WordPress: ul.dropdown-menu__menu -->
				<ul class="dropdown-menu__menu">
					{#each rooms as room (room.id)}
						<li>
							<!-- WordPress EXACT: room link with icon -->
							<a
								href={room.room_url || '#'}
								target="_blank"
								rel="nofollow"
								class:no-access={!room.has_access}
								onclick={(e) => { if (room.has_access) { e.preventDefault(); handleEnterRoom(room); } else { e.preventDefault(); } }}
							>
								<span class="st-icon-{room.slug || 'mastering-the-trade'} icon icon--md"></span>
								{room.name}
								{#if enteringRoom === room.slug}
									<span class="entering-indicator">...</span>
								{/if}
							</a>
						</li>
					{/each}
				</ul>
			{/if}
		</nav>
	{/if}
</div>

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES - WordPress EXACT Match
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	/* WordPress: .dropdown.display-inline-block */
	.dropdown {
		position: relative;
	}

	.display-inline-block {
		display: inline-block;
	}

	/* WordPress EXACT: .btn.btn-xs.btn-orange.btn-tradingroom.dropdown-toggle */
	.btn-orange {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		background: #f99e31;
		color: #fff;
		font-size: 14px;
		padding: 12px 20px;
		border-radius: 5px;
		border: none;
		cursor: pointer;
		text-decoration: none;
		transition: background-color 0.15s ease;
	}

	.btn-orange:hover {
		background: #dc7309;
		color: #fff;
	}

	.btn-orange strong {
		font-weight: 700;
	}

	.btn-tradingroom {
		white-space: nowrap;
	}

	/* WordPress EXACT: .dropdown-menu.dropdown-menu--full-width */
	.dropdown-menu {
		position: absolute;
		top: calc(100% + 4px);
		right: 0;
		min-width: 280px;
		background: #fff;
		border-radius: 5px;
		box-shadow: 0 10px 40px rgba(0, 0, 0, 0.15);
		z-index: 1000;
		overflow: hidden;
		padding: 10px 0;
	}

	.dropdown-menu--full-width {
		min-width: 100%;
	}

	/* WordPress EXACT: .dropdown-menu__menu */
	.dropdown-menu__menu {
		list-style: none;
		margin: 0;
		padding: 0;
		max-height: 400px;
		overflow-y: auto;
	}

	.dropdown-menu__menu li {
		margin: 0;
		padding: 0;
	}

	.dropdown-menu__menu a {
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 12px 20px;
		color: #333;
		text-decoration: none;
		font-size: 14px;
		font-weight: 500;
		transition: background-color 0.1s ease, color 0.1s ease;
	}

	.dropdown-menu__menu a:hover {
		background: #f4f4f4;
		color: #0984ae;
	}

	.dropdown-menu__menu a.no-access {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* WordPress: icon styles */
	.dropdown-menu__menu .icon {
		font-family: 'StIcons', system-ui, sans-serif;
		font-style: normal;
		font-weight: normal;
		width: 24px;
		height: 24px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #0984ae;
	}

	.icon--md {
		font-size: 20px;
	}

	.entering-indicator {
		margin-left: auto;
		color: #0984ae;
	}

	/* Loading, error, empty states */
	.dropdown-loading,
	.dropdown-error,
	.dropdown-empty {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		padding: 20px;
		color: #64748b;
		font-size: 14px;
	}

	.dropdown-error {
		flex-direction: column;
		gap: 12px;
	}

	.retry-btn {
		padding: 6px 12px;
		background: #0984ae;
		color: #fff;
		border: none;
		border-radius: 4px;
		font-size: 12px;
		cursor: pointer;
	}

	.retry-btn:hover {
		background: #076787;
	}

	/* Animation for loading spinner */
	:global(.spin) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* Responsive */
	@media (max-width: 576px) {
		.btn-orange {
			padding: 10px 14px;
			font-size: 13px;
		}

		.dropdown-menu {
			min-width: 260px;
			right: 0;
		}
	}

	@media (max-width: 430px) {
		.btn-orange strong {
			font-size: 12px;
		}
	}
</style>

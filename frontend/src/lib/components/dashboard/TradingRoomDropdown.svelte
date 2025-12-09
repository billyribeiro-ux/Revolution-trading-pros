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
     TEMPLATE
     ═══════════════════════════════════════════════════════════════════════════ -->

<div class="trading-room-dropdown">
	<button
		class="dropdown-trigger"
		onclick={toggleDropdown}
		aria-expanded={isOpen}
		aria-haspopup="listbox"
	>
		<IconDoorEnter size={18} />
		<span>Enter a Trading Room</span>
		<span class="chevron" class:rotated={isOpen}>
			<IconChevronDown size={16} />
		</span>
	</button>

	{#if isOpen}
		<div class="dropdown-menu" role="listbox">
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
				<ul class="room-list">
					{#each rooms as room (room.id)}
						<li>
							<button
								class="room-item"
								class:has-access={room.has_access}
								class:no-access={!room.has_access}
								onclick={() => handleEnterRoom(room)}
								disabled={!room.has_access || enteringRoom === room.slug}
							>
								<span class="room-name">{room.name}</span>
								{#if room.has_access}
									{#if enteringRoom === room.slug}
										<span class="spin"><IconLoader size={14} /></span>
									{:else}
										<span class="membership-badge {room.membership_status}">
											{getMembershipStatusLabel(room.membership_status)}
										</span>
									{/if}
								{:else}
									<IconLock size={14} />
								{/if}
							</button>
						</li>
					{/each}
				</ul>
				{#if accessibleRooms.length > 0}
					<div class="dropdown-footer">
						<span>{accessibleRooms.length} room{accessibleRooms.length > 1 ? 's' : ''} available</span>
					</div>
				{/if}
			{/if}
		</div>
	{/if}
</div>

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	.trading-room-dropdown {
		position: relative;
	}

	.dropdown-trigger {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		background: var(--st-orange, #f99e31);
		color: #fff;
		font-weight: 700;
		font-size: 14px;
		padding: 12px 20px;
		border-radius: 5px;
		border: none;
		cursor: pointer;
		transition: background-color 0.15s ease;
	}

	.dropdown-trigger:hover {
		background: var(--st-orange-hover, #dc7309);
	}

	.dropdown-trigger .chevron {
		transition: transform 0.15s ease;
	}

	.dropdown-trigger .chevron.rotated {
		transform: rotate(180deg);
	}

	.dropdown-menu {
		position: absolute;
		top: calc(100% + 8px);
		right: 0;
		min-width: 280px;
		max-width: 320px;
		background: #fff;
		border-radius: 8px;
		box-shadow: 0 10px 40px rgba(0, 0, 0, 0.15);
		z-index: 100;
		overflow: hidden;
	}

	.dropdown-loading,
	.dropdown-error,
	.dropdown-empty {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		padding: 20px;
		color: var(--st-text-muted, #64748b);
		font-size: 14px;
	}

	.dropdown-error {
		flex-direction: column;
		gap: 12px;
	}

	.retry-btn {
		padding: 6px 12px;
		background: var(--st-primary, #0984ae);
		color: #fff;
		border: none;
		border-radius: 4px;
		font-size: 12px;
		cursor: pointer;
	}

	.room-list {
		list-style: none;
		margin: 0;
		padding: 8px 0;
		max-height: 300px;
		overflow-y: auto;
	}

	.room-item {
		display: flex;
		align-items: center;
		justify-content: space-between;
		width: 100%;
		padding: 12px 16px;
		background: transparent;
		border: none;
		cursor: pointer;
		transition: background-color 0.1s ease;
		text-align: left;
	}

	.room-item.has-access:hover {
		background: #f4f4f4;
	}

	.room-item.no-access {
		cursor: not-allowed;
		opacity: 0.6;
	}

	.room-name {
		font-size: 14px;
		font-weight: 500;
		color: var(--st-text-color, #333);
	}

	.room-item.no-access .room-name {
		color: var(--st-text-muted, #64748b);
	}

	.membership-badge {
		padding: 2px 8px;
		border-radius: 10px;
		font-size: 11px;
		font-weight: 600;
		text-transform: uppercase;
	}

	.membership-badge.active {
		background: #dcfce7;
		color: #166534;
	}

	.membership-badge.trial {
		background: #dbeafe;
		color: #1e40af;
	}

	.membership-badge.expiring {
		background: #fef3c7;
		color: #92400e;
	}

	.membership-badge.paused {
		background: #f3f4f6;
		color: #4b5563;
	}

	.membership-badge.complimentary {
		background: #f3e8ff;
		color: #7c3aed;
	}

	.dropdown-footer {
		padding: 10px 16px;
		border-top: 1px solid #e5e7eb;
		font-size: 12px;
		color: var(--st-text-muted, #64748b);
		text-align: center;
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
		.dropdown-trigger {
			padding: 10px 16px;
			font-size: 13px;
		}

		.dropdown-trigger span:not(.chevron) {
			display: none;
		}

		.dropdown-menu {
			min-width: 240px;
			right: -10px;
		}
	}
</style>

<script lang="ts">
	/**
	 * Room Selector Component
	 * ═══════════════════════════════════════════════════════════════════════════
	 * Beautiful multi-select room picker with "Select All" functionality
	 * @version 1.0.0 - December 2025
	 */

	import {
		ROOMS,
		ALL_ROOM_IDS,
		getLiveTradingRooms,
		getAlertsOnlyServices,
		isAllRooms,
		type Room
	} from '$lib/config/rooms';

	// Props interface
	interface Props {
		selectedRooms?: string[];
		disabled?: boolean;
	}

	let props: Props = $props();

	// Bindable state for two-way binding - sync with props
	let selectedRooms = $state<string[]>(props.selectedRooms ?? []);
	const disabled = $derived(props.disabled ?? false);

	// Sync selectedRooms back to parent via effect
	$effect(() => {
		if (props.selectedRooms !== undefined) {
			selectedRooms = props.selectedRooms;
		}
	});

	// State
	let isExpanded = $state(false);

	// Computed
	const liveTradingRooms = getLiveTradingRooms();
	const alertsOnlyServices = getAlertsOnlyServices();
	const allSelected = $derived(isAllRooms(selectedRooms));
	const noneSelected = $derived(selectedRooms.length === 0);

	// Selection text
	const selectionText = $derived.by(() => {
		if (allSelected) return 'All Rooms & Services';
		if (noneSelected) return 'Select rooms...';
		if (selectedRooms.length === 1) {
			const room = ROOMS.find((r) => r.id === selectedRooms[0]);
			return room?.name || selectedRooms[0];
		}
		return `${selectedRooms.length} rooms selected`;
	});

	// Toggle functions
	function toggleRoom(roomId: string) {
		if (disabled) return;
		if (selectedRooms.includes(roomId)) {
			selectedRooms = selectedRooms.filter((id) => id !== roomId);
		} else {
			selectedRooms = [...selectedRooms, roomId];
		}
	}

	function selectAll() {
		if (disabled) return;
		selectedRooms = [...ALL_ROOM_IDS];
	}

	function clearAll() {
		if (disabled) return;
		selectedRooms = [];
	}

	function selectGroup(rooms: Room[]) {
		if (disabled) return;
		const groupIds = rooms.map((r) => r.id);
		const allInGroup = groupIds.every((id) => selectedRooms.includes(id));

		if (allInGroup) {
			// Deselect all in group
			selectedRooms = selectedRooms.filter((id) => !groupIds.includes(id));
		} else {
			// Select all in group
			const newSelection = new Set([...selectedRooms, ...groupIds]);
			selectedRooms = Array.from(newSelection);
		}
	}

	function isGroupSelected(rooms: Room[]): boolean {
		return rooms.every((r) => selectedRooms.includes(r.id));
	}

	function isGroupPartial(rooms: Room[]): boolean {
		const count = rooms.filter((r) => selectedRooms.includes(r.id)).length;
		return count > 0 && count < rooms.length;
	}

	function toggleDropdown() {
		if (!disabled) {
			isExpanded = !isExpanded;
		}
	}

	function closeDropdown() {
		isExpanded = false;
	}
</script>

<div class="room-selector" class:disabled>
	<!-- Trigger Button with accessible label -->
	<button
		type="button"
		class="selector-trigger"
		class:expanded={isExpanded}
		onclick={toggleDropdown}
		{disabled}
	>
		<span class="trigger-text">
			{#if allSelected}
				<span class="all-badge">ALL</span>
			{:else if selectedRooms.length > 0}
				<span class="count-badge">{selectedRooms.length}</span>
			{/if}
			{selectionText}
		</span>
		<svg class="trigger-icon" class:rotated={isExpanded} viewBox="0 0 20 20" fill="currentColor">
			<path
				fill-rule="evenodd"
				d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z"
				clip-rule="evenodd"
			/>
		</svg>
	</button>

	<!-- Dropdown Panel -->
	{#if isExpanded}
		<div
			class="dropdown-backdrop"
			onclick={closeDropdown}
			onkeydown={(e) => e.key === 'Escape' && closeDropdown()}
			role="button"
			tabindex="0"
			aria-label="Close dropdown"
		></div>
		<div class="dropdown-panel">
			<!-- Quick Actions -->
			<div class="quick-actions">
				<button type="button" class="quick-btn" onclick={selectAll} disabled={allSelected}>
					Select All
				</button>
				<button type="button" class="quick-btn" onclick={clearAll} disabled={noneSelected}>
					Clear All
				</button>
			</div>

			<!-- Live Trading Rooms Group -->
			<div class="room-group">
				<button type="button" class="group-header" onclick={() => selectGroup(liveTradingRooms)}>
					<div
						class="group-checkbox"
						class:checked={isGroupSelected(liveTradingRooms)}
						class:partial={isGroupPartial(liveTradingRooms)}
					>
						{#if isGroupSelected(liveTradingRooms)}
							<svg viewBox="0 0 20 20" fill="currentColor"
								><path
									fill-rule="evenodd"
									d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
									clip-rule="evenodd"
								/></svg
							>
						{:else if isGroupPartial(liveTradingRooms)}
							<svg viewBox="0 0 20 20" fill="currentColor"
								><path
									fill-rule="evenodd"
									d="M3 10a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z"
									clip-rule="evenodd"
								/></svg
							>
						{/if}
					</div>
					<span class="group-title">Live Trading Rooms</span>
					<span class="group-count"
						>{liveTradingRooms.filter((r) => selectedRooms.includes(r.id))
							.length}/{liveTradingRooms.length}</span
					>
				</button>
				<div class="room-list">
					{#each liveTradingRooms as room}
						<button
							type="button"
							class="room-item"
							class:selected={selectedRooms.includes(room.id)}
							onclick={() => toggleRoom(room.id)}
						>
							<div class="room-checkbox" class:checked={selectedRooms.includes(room.id)}>
								{#if selectedRooms.includes(room.id)}
									<svg viewBox="0 0 20 20" fill="currentColor"
										><path
											fill-rule="evenodd"
											d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
											clip-rule="evenodd"
										/></svg
									>
								{/if}
							</div>
							<span class="room-icon">{room.icon}</span>
							<span class="room-name">{room.name}</span>
							<span class="room-badge" style="background-color: {room.color}20; color: {room.color}"
								>{room.shortName}</span
							>
						</button>
					{/each}
				</div>
			</div>

			<!-- Alerts Only Services Group -->
			<div class="room-group">
				<button type="button" class="group-header" onclick={() => selectGroup(alertsOnlyServices)}>
					<div
						class="group-checkbox"
						class:checked={isGroupSelected(alertsOnlyServices)}
						class:partial={isGroupPartial(alertsOnlyServices)}
					>
						{#if isGroupSelected(alertsOnlyServices)}
							<svg viewBox="0 0 20 20" fill="currentColor"
								><path
									fill-rule="evenodd"
									d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
									clip-rule="evenodd"
								/></svg
							>
						{:else if isGroupPartial(alertsOnlyServices)}
							<svg viewBox="0 0 20 20" fill="currentColor"
								><path
									fill-rule="evenodd"
									d="M3 10a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z"
									clip-rule="evenodd"
								/></svg
							>
						{/if}
					</div>
					<span class="group-title">Alerts Only Services</span>
					<span class="group-count"
						>{alertsOnlyServices.filter((r) => selectedRooms.includes(r.id))
							.length}/{alertsOnlyServices.length}</span
					>
				</button>
				<div class="room-list">
					{#each alertsOnlyServices as room}
						<button
							type="button"
							class="room-item"
							class:selected={selectedRooms.includes(room.id)}
							onclick={() => toggleRoom(room.id)}
						>
							<div class="room-checkbox" class:checked={selectedRooms.includes(room.id)}>
								{#if selectedRooms.includes(room.id)}
									<svg viewBox="0 0 20 20" fill="currentColor"
										><path
											fill-rule="evenodd"
											d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
											clip-rule="evenodd"
										/></svg
									>
								{/if}
							</div>
							<span class="room-icon">{room.icon}</span>
							<span class="room-name">{room.name}</span>
							<span class="room-badge" style="background-color: {room.color}20; color: {room.color}"
								>{room.shortName}</span
							>
						</button>
					{/each}
				</div>
			</div>

			<!-- Done Button -->
			<div class="dropdown-footer">
				<button type="button" class="done-btn" onclick={closeDropdown}> Done </button>
			</div>
		</div>
	{/if}

	<!-- Selected Tags (visible when collapsed) -->
	{#if !isExpanded && selectedRooms.length > 0 && selectedRooms.length <= 4 && !allSelected}
		<div class="selected-tags">
			{#each selectedRooms as roomId}
				{@const room = ROOMS.find((r) => r.id === roomId)}
				{#if room}
					<span
						class="tag"
						style="background-color: {room.color}20; color: {room.color}; border-color: {room.color}40"
					>
						{room.icon}
						{room.shortName}
					</span>
				{/if}
			{/each}
		</div>
	{/if}
</div>

<style>
	.room-selector {
		position: relative;
		width: 100%;
	}

	.room-selector.disabled {
		opacity: 0.6;
		pointer-events: none;
	}

	/* Trigger Button */
	.selector-trigger {
		width: 100%;
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 12px 16px;
		background: #0f172a;
		border: 1px solid #334155;
		border-radius: 8px;
		color: #e2e8f0;
		font-size: 14px;
		cursor: pointer;
		transition: all 0.2s;
	}

	.selector-trigger:hover:not(:disabled) {
		border-color: #0984ae;
		background: #1e293b;
	}

	.selector-trigger.expanded {
		border-color: #0984ae;
		border-bottom-left-radius: 0;
		border-bottom-right-radius: 0;
	}

	.trigger-text {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.all-badge {
		display: inline-flex;
		align-items: center;
		padding: 2px 8px;
		background: linear-gradient(135deg, #10b981, #0984ae);
		color: white;
		font-size: 10px;
		font-weight: 700;
		border-radius: 4px;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.count-badge {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 22px;
		height: 22px;
		background: #0984ae;
		color: white;
		font-size: 11px;
		font-weight: 700;
		border-radius: 50%;
	}

	.trigger-icon {
		width: 20px;
		height: 20px;
		color: #64748b;
		transition: transform 0.2s;
	}

	.trigger-icon.rotated {
		transform: rotate(180deg);
	}

	/* Dropdown */
	.dropdown-backdrop {
		position: fixed;
		inset: 0;
		z-index: 40;
	}

	.dropdown-panel {
		position: absolute;
		top: 100%;
		left: 0;
		right: 0;
		z-index: 50;
		background: #0f172a;
		border: 1px solid #0984ae;
		border-top: none;
		border-radius: 0 0 8px 8px;
		max-height: 400px;
		overflow-y: auto;
		box-shadow: 0 10px 40px rgba(0, 0, 0, 0.5);
	}

	/* Quick Actions */
	.quick-actions {
		display: flex;
		gap: 8px;
		padding: 12px;
		background: #1e293b;
		border-bottom: 1px solid #334155;
	}

	.quick-btn {
		flex: 1;
		padding: 8px 12px;
		background: #334155;
		border: none;
		border-radius: 6px;
		color: #e2e8f0;
		font-size: 13px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.quick-btn:hover:not(:disabled) {
		background: #475569;
	}

	.quick-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* Room Groups */
	.room-group {
		border-bottom: 1px solid #334155;
	}

	.room-group:last-of-type {
		border-bottom: none;
	}

	.group-header {
		width: 100%;
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 12px 16px;
		background: #1e293b;
		border: none;
		color: #e2e8f0;
		font-size: 13px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		cursor: pointer;
		transition: background 0.2s;
	}

	.group-header:hover {
		background: #334155;
	}

	.group-checkbox {
		width: 18px;
		height: 18px;
		border: 2px solid #475569;
		border-radius: 4px;
		display: flex;
		align-items: center;
		justify-content: center;
		transition: all 0.2s;
	}

	.group-checkbox.checked {
		background: #0984ae;
		border-color: #0984ae;
	}

	.group-checkbox.partial {
		background: #475569;
		border-color: #475569;
	}

	.group-checkbox svg {
		width: 12px;
		height: 12px;
		color: white;
	}

	.group-title {
		flex: 1;
		text-align: left;
	}

	.group-count {
		font-size: 11px;
		color: #64748b;
		font-weight: 500;
	}

	/* Room Items */
	.room-list {
		padding: 4px 8px;
	}

	.room-item {
		width: 100%;
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 10px 12px;
		background: transparent;
		border: none;
		border-radius: 6px;
		color: #cbd5e1;
		font-size: 14px;
		cursor: pointer;
		transition: all 0.2s;
	}

	.room-item:hover {
		background: #1e293b;
	}

	.room-item.selected {
		background: rgba(9, 132, 174, 0.15);
	}

	.room-checkbox {
		width: 18px;
		height: 18px;
		border: 2px solid #475569;
		border-radius: 4px;
		display: flex;
		align-items: center;
		justify-content: center;
		transition: all 0.2s;
		flex-shrink: 0;
	}

	.room-checkbox.checked {
		background: #0984ae;
		border-color: #0984ae;
	}

	.room-checkbox svg {
		width: 12px;
		height: 12px;
		color: white;
	}

	.room-icon {
		font-size: 18px;
		flex-shrink: 0;
	}

	.room-name {
		flex: 1;
		text-align: left;
		font-weight: 500;
	}

	.room-badge {
		padding: 3px 8px;
		border-radius: 4px;
		font-size: 11px;
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.3px;
	}

	/* Footer */
	.dropdown-footer {
		padding: 12px;
		background: #1e293b;
		border-top: 1px solid #334155;
	}

	.done-btn {
		width: 100%;
		padding: 10px;
		background: linear-gradient(135deg, #0984ae, #10b981);
		border: none;
		border-radius: 6px;
		color: white;
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.done-btn:hover {
		filter: brightness(1.1);
		transform: translateY(-1px);
	}

	/* Selected Tags */
	.selected-tags {
		display: flex;
		flex-wrap: wrap;
		gap: 6px;
		margin-top: 8px;
	}

	.tag {
		display: inline-flex;
		align-items: center;
		gap: 4px;
		padding: 4px 10px;
		border-radius: 20px;
		font-size: 12px;
		font-weight: 600;
		border: 1px solid;
	}
</style>

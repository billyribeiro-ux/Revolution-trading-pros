<script lang="ts">
	import { IconSearch, IconRefresh, IconTrash } from '$lib/icons';

	interface Props {
		searchQuery: string;
		statusFilter: string;
		selectedCount: number;
		onsearch: () => void;
		onrefresh: () => void;
		onbulkdelete: () => void;
	}

	let {
		searchQuery = $bindable(),
		statusFilter = $bindable(),
		selectedCount,
		onsearch,
		onrefresh,
		onbulkdelete
	}: Props = $props();
</script>

<div class="filters-bar">
	<div class="search-box">
		<IconSearch size={18} />
		<input
			id="page-searchquery"
			name="page-searchquery"
			type="text"
			placeholder="Search by email or name..."
			bind:value={searchQuery}
			onkeydown={(e: KeyboardEvent) => e.key === 'Enter' && onsearch()}
		/>
	</div>

	<div class="filter-group">
		<select bind:value={statusFilter} onchange={onsearch}>
			<option value="all">All Status</option>
			<option value="subscribed">Subscribed</option>
			<option value="unsubscribed">Unsubscribed</option>
			<option value="bounced">Bounced</option>
			<option value="complained">Complained</option>
		</select>
	</div>

	<button class="btn-icon" onclick={onrefresh}>
		<IconRefresh size={18} />
	</button>

	{#if selectedCount > 0}
		<div class="bulk-actions">
			<span>{selectedCount} selected</span>
			<button class="btn-danger small" onclick={onbulkdelete}>
				<IconTrash size={16} />
				Delete
			</button>
		</div>
	{/if}
</div>

<style>
	.filters-bar {
		display: flex;
		align-items: center;
		gap: 1rem;
		margin-bottom: 1.5rem;
		padding: 1rem;
		background: rgba(30, 41, 59, 0.4);
		border-radius: 12px;
	}

	.search-box {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		flex: 1;
		max-width: 400px;
		padding: 0.5rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		color: #94a3b8;
	}

	.search-box input {
		flex: 1;
		background: none;
		border: none;
		color: #f1f5f9;
		font-size: 0.9375rem;
	}

	.search-box input:focus {
		outline: none;
	}

	.filter-group select {
		padding: 0.625rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		color: #f1f5f9;
		font-size: 0.875rem;
	}

	.bulk-actions {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		margin-left: auto;
		padding-left: 1rem;
		border-left: 1px solid rgba(148, 163, 184, 0.2);
		color: #94a3b8;
		font-size: 0.875rem;
	}

	.btn-icon {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(148, 163, 184, 0.1);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		color: #94a3b8;
		cursor: pointer;
	}

	.btn-danger {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		background: rgba(239, 68, 68, 0.1);
		color: #f87171;
		border: 1px solid rgba(239, 68, 68, 0.2);
	}

	.btn-danger.small {
		padding: 0.5rem 0.875rem;
		font-size: 0.8125rem;
	}

	@media (max-width: 767.98px) {
		.filters-bar {
			flex-wrap: wrap;
		}

		.search-box {
			width: 100%;
			max-width: none;
		}
	}
</style>

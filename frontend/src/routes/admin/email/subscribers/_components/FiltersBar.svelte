<script lang="ts">
	import { IconSearch, IconRefresh, IconTrash } from '#lib/icons/index.js';

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
		<input autocomplete="email"
			id="page-searchquery"
			name="page-searchquery"
			type="text"
			placeholder="Search by email or name..."
			bind:value={searchQuery}
			onkeydown={(e: KeyboardEvent) => e.key === 'Enter' && onsearch()}
		/>
	</div>

	<div class="filter-group">
		<select id="filtersbar-statusfilter" name="filtersbar_statusfilter" bind:value={statusFilter} onchange={onsearch}>
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
		background: var(--bg-elevated, #161b22);
		border-radius: 12px;
	}

	.search-box {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		flex: 1;
		max-width: 400px;
		padding: 0.5rem 1rem;
		background: var(--bg-base, #0d1117);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 8px;
		color: var(--text-secondary, #8b949e);
	}

	.search-box input {
		flex: 1;
		background: none;
		border: none;
		color: var(--text-primary, #f0f6fc);
		font-size: 0.9375rem;
	}

	.search-box input:focus {
		outline: none;
	}

	.filter-group select {
		padding: 0.625rem 1rem;
		background: var(--bg-base, #0d1117);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 8px;
		color: var(--text-primary, #f0f6fc);
		font-size: 0.875rem;
	}

	.bulk-actions {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		margin-left: auto;
		padding-left: 1rem;
		border-left: 1px solid var(--border-default, #30363d);
		color: var(--text-secondary, #8b949e);
		font-size: 0.875rem;
	}

	.btn-icon {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--bg-surface, #1c2128);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 8px;
		color: var(--text-secondary, #8b949e);
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
		background: rgba(248, 81, 73, 0.1);
		color: var(--error-emphasis, #f85149);
		border: 1px solid rgba(248, 81, 73, 0.2);
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

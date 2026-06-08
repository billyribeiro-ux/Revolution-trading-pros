<script lang="ts">
	import { IconSearch, IconFilter } from '$lib/icons';

	interface Props {
		searchQuery: string;
		showFilters: boolean;
		onsearch: () => void;
		ontoggleFilters: () => void;
	}

	let { searchQuery = $bindable(''), showFilters, onsearch, ontoggleFilters }: Props = $props();
</script>

<div class="toolbar">
	<div class="search-box">
		<IconSearch size={18} />
		<input
			type="text"
			placeholder="Search by order #, email, or name..."
			bind:value={searchQuery}
			onkeydown={(e) => e.key === 'Enter' && onsearch()}
		/>
	</div>

	<div class="toolbar-actions">
		<button class={['filter-toggle', { active: showFilters }]} onclick={ontoggleFilters}>
			<IconFilter size={18} />
			Filters
		</button>
	</div>
</div>

<style>
	.toolbar {
		display: flex;
		justify-content: space-between;
		align-items: center;
		gap: 1rem;
		margin-bottom: 1rem;
	}

	.search-box {
		flex: 1;
		max-width: 400px;
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem 1rem;
		background: rgba(22, 27, 34, 0.85);
		border: 1px solid var(--border-muted);
		border-radius: 12px;
		color: var(--text-secondary);
		backdrop-filter: blur(10px);
	}

	.search-box:focus-within {
		border-color: rgba(230, 184, 0, 0.5);
		box-shadow: 0 0 0 3px rgba(230, 184, 0, 0.15);
	}

	.search-box input {
		flex: 1;
		background: none;
		border: none;
		color: var(--text-primary);
		font-size: 0.9375rem;
		outline: none;
	}

	.search-box input::placeholder {
		color: var(--text-tertiary);
	}

	.toolbar-actions {
		display: flex;
		gap: 0.75rem;
	}

	.filter-toggle {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1rem;
		background: rgba(100, 116, 139, 0.2);
		border: 1px solid rgba(100, 116, 139, 0.3);
		border-radius: 12px;
		color: var(--text-primary);
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.filter-toggle:hover,
	.filter-toggle.active {
		background: rgba(230, 184, 0, 0.15);
		border-color: rgba(230, 184, 0, 0.3);
		color: var(--primary-500);
	}

	@media (max-width: 767.98px) {
		.toolbar {
			flex-direction: column;
		}
		.search-box {
			max-width: 100%;
		}
	}
</style>

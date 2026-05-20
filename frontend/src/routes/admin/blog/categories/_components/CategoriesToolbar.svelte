<script lang="ts">
	/**
	 * R28-C extraction (2026-05-20): unified search/filter toolbar used to
	 * narrow both the categories and tags lists at the same time. Parent
	 * binds `search` and `showHidden`; a single `$effect` in the parent
	 * re-runs `applyFilters()` when either changes.
	 */
	import { IconSearch } from '$lib/icons';

	interface Props {
		search: string;
		showHidden: boolean;
	}

	let { search = $bindable(), showHidden = $bindable() }: Props = $props();
</script>

<div class="toolbar">
	<div class="search-group">
		<IconSearch size={18} />
		<input
			id="page-categorysearch"
			name="page-categorysearch"
			type="text"
			class="search-input"
			placeholder="Search categories and tags..."
			bind:value={search}
		/>
	</div>
	<label class="checkbox-label">
		<input
			id="page-showhidden"
			name="page-showhidden"
			type="checkbox"
			bind:checked={showHidden}
		/>
		<span>Show Hidden</span>
	</label>
</div>

<style>
	.toolbar {
		display: flex;
		gap: 1rem;
		align-items: center;
		margin-bottom: 2rem;
		padding: 1rem;
		background: rgba(30, 41, 59, 0.4);
		border-radius: 8px;
		border: 1px solid rgba(148, 163, 184, 0.2);
	}

	.search-group {
		flex: 1;
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border-radius: 6px;
		color: #94a3b8;
	}

	.search-input {
		flex: 1;
		border: none;
		background: none;
		outline: none;
		font-size: 0.95rem;
		color: #f1f5f9;
	}

	.search-input::placeholder {
		color: #64748b;
	}

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		cursor: pointer;
		user-select: none;
		color: #cbd5e1;
	}
</style>

<script lang="ts">
	/**
	 * R28-C extraction (2026-05-20): unified search/filter toolbar used to
	 * narrow both the categories and tags lists at the same time. Parent
	 * binds `search` and `showHidden`; a single `$effect` in the parent
	 * re-runs `applyFilters()` when either changes.
	 */
	import { IconSearch } from '#lib/icons/index.js';

	interface Props {
		search: string;
		showHidden: boolean;
	}

	let { search = $bindable(), showHidden = $bindable() }: Props = $props();
</script>

<div class="toolbar">
	<div class="search-group">
		<IconSearch size={18} />
		<input autocomplete="off"
			id="page-categorysearch"
			name="page-categorysearch"
			type="text"
			class="search-input"
			placeholder="Search categories and tags..."
			bind:value={search}
		/>
	</div>
	<label class="checkbox-label">
		<input id="page-showhidden" name="page-showhidden" type="checkbox" bind:checked={showHidden} />
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
		background: var(--bg-elevated, #161b22);
		border-radius: 8px;
		border: 1px solid var(--border-default, #30363d);
	}

	.search-group {
		flex: 1;
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		background: var(--bg-base, #0d1117);
		border-radius: 6px;
		color: var(--text-secondary, #8b949e);
	}

	.search-input {
		flex: 1;
		border: none;
		background: none;
		outline: none;
		font-size: 0.95rem;
		color: var(--text-primary, #f0f6fc);
	}

	.search-input::placeholder {
		color: var(--text-tertiary, #6e7681);
	}

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		cursor: pointer;
		user-select: none;
		color: var(--text-secondary, #8b949e);
	}
</style>

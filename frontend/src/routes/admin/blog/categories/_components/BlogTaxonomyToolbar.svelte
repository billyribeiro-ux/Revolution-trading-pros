<script lang="ts">
	/**
	 * R27-E extraction (2026-05-20): search input + show-hidden
	 * checkbox for the blog categories/tags admin page. Uses
	 * `$bindable` so the parent's `categorySearch` / `showHidden`
	 * runes stay the single source of truth — the parent's filter
	 * $effect re-runs automatically when either changes.
	 *
	 * Incidental fix: the original `id="page-categorysearch"` /
	 * `id="page-showhidden"` ids carried `page-` prefixes that look
	 * like the leftover of a code-generator; after extraction they
	 * become natural ids (`search-input`, `show-hidden`).
	 */
	import { IconSearch } from '$lib/icons';

	interface Props {
		search: string;
		showHidden: boolean;
	}

	let {
		search = $bindable(),
		showHidden = $bindable()
	}: Props = $props();
</script>

<div class="toolbar">
	<div class="search-group">
		<label for="search-input" class="visually-hidden">Search categories and tags</label>
		<IconSearch size={18} />
		<input
			id="search-input"
			name="search"
			type="text"
			class="search-input"
			placeholder="Search categories and tags..."
			bind:value={search}
		/>
	</div>
	<label class="checkbox-label">
		<input id="show-hidden" name="show-hidden" type="checkbox" bind:checked={showHidden} />
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

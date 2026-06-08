<script lang="ts">
	/**
	 * PageHeader — centered "Media Library" title + search/view-toggle/analytics/upload actions.
	 * Extracted from `admin/media/+page.svelte` (R10-C).
	 *
	 * Props:
	 *   - totalItems: number
	 *   - searchQuery: string                ($bindable — bound to <input>)
	 *   - viewMode: 'grid' | 'list'          ($bindable — flipped by toggle buttons)
	 *   - showUploadPanel: boolean           ($bindable — flipped by Upload button)
	 *   - onSearchSubmit: () => void         (Enter key on search input)
	 *
	 * 3 bindable scalars, 1 callback. Within R8-C thresholds.
	 */
	import IconLayoutGrid from '@tabler/icons-svelte-runes/icons/layout-grid';
	import IconList from '@tabler/icons-svelte-runes/icons/list';
	import IconChartBar from '@tabler/icons-svelte-runes/icons/chart-bar';
	import IconUpload from '@tabler/icons-svelte-runes/icons/upload';

	let {
		totalItems,
		searchQuery = $bindable(''),
		viewMode = $bindable('grid'),
		showUploadPanel = $bindable(false),
		onSearchSubmit
	}: {
		totalItems: number;
		searchQuery?: string;
		viewMode?: 'grid' | 'list';
		showUploadPanel?: boolean;
		onSearchSubmit: () => void;
	} = $props();
</script>

<div class="page-header">
	<h1>Media Library</h1>
	<p class="subtitle">
		Manage your media files, optimize images, and organize assets ({totalItems} items)
	</p>
	<div class="header-actions">
		<div class="search-box">
			<input
				id="page-searchquery"
				name="page-searchquery"
				type="text"
				placeholder="Search media..."
				bind:value={searchQuery}
				class="search-input"
				onkeydown={(e: KeyboardEvent) => e.key === 'Enter' && onSearchSubmit()}
			/>
		</div>
		<div class="view-toggle">
			<button
				class={{ active: viewMode === 'grid' }}
				onclick={() => (viewMode = 'grid')}
				title="Grid view"
			>
				<IconLayoutGrid size={20} aria-hidden="true" />
			</button>
			<button
				class={{ active: viewMode === 'list' }}
				onclick={() => (viewMode = 'list')}
				title="List view"
			>
				<IconList size={20} aria-hidden="true" />
			</button>
		</div>
		<a href="/admin/media/analytics" class="btn-secondary" title="View bandwidth analytics">
			<IconChartBar size={20} aria-hidden="true" />
			Analytics
		</a>
		<button class="btn-primary" onclick={() => (showUploadPanel = !showUploadPanel)}>
			<IconUpload size={20} aria-hidden="true" />
			Upload
		</button>
	</div>
</div>

<style>
	.page-header {
		text-align: center;
		margin-bottom: 2rem;
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.25rem;
	}

	.subtitle {
		color: #64748b;
		font-size: 0.875rem;
		margin: 0 0 1.5rem;
	}

	.header-actions {
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 0.75rem;
		flex-wrap: wrap;
	}

	.search-box {
		position: relative;
	}

	.search-input {
		width: 240px;
		padding: 0.625rem 1rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		color: #f1f5f9;
		font-size: 0.875rem;
	}

	.search-input::placeholder {
		color: #64748b;
	}

	.search-input:focus {
		outline: none;
		border-color: rgba(99, 102, 241, 0.5);
	}

	.view-toggle {
		display: flex;
		background: rgba(30, 41, 59, 0.6);
		border-radius: 8px;
		padding: 2px;
		border: 1px solid rgba(148, 163, 184, 0.2);
	}

	.view-toggle button {
		padding: 6px 10px;
		background: transparent;
		border: none;
		border-radius: 6px;
		cursor: pointer;
		transition: all 0.2s;
		color: #94a3b8;
	}

	.view-toggle button.active {
		background: rgba(99, 102, 241, 0.3);
		color: #a5b4fc;
	}

	.view-toggle button:hover:not(.active) {
		background: rgba(148, 163, 184, 0.1);
	}

	.view-toggle button :global(svg) {
		width: 18px;
		height: 18px;
	}

	/* Buttons (duplicated from parent so the header is self-contained) */
	.btn-primary,
	.btn-secondary {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 1rem;
		border-radius: 6px;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
		text-decoration: none;
	}

	.btn-primary {
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		color: white;
		border: none;
	}

	.btn-primary:hover {
		opacity: 0.9;
		transform: translateY(-1px);
	}

	.btn-secondary {
		background: rgba(100, 116, 139, 0.2);
		color: #cbd5e1;
		border: 1px solid rgba(100, 116, 139, 0.3);
	}

	.btn-secondary:hover {
		background: rgba(100, 116, 139, 0.3);
		border-color: rgba(100, 116, 139, 0.4);
	}

	.btn-primary :global(svg),
	.btn-secondary :global(svg) {
		width: 14px;
		height: 14px;
	}

	@media (max-width: 767.98px) {
		.page-header h1 {
			font-size: 1.5rem;
		}

		.header-actions {
			flex-direction: column;
			gap: 0.5rem;
		}

		.search-input {
			width: 100%;
		}
	}
</style>

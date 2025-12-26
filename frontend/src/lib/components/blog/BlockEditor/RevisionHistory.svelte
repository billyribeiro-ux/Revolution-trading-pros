<script lang="ts">
	/**
	 * Revision History - Enterprise-Grade Version Control
	 * ====================================================
	 * Complete revision management with diff view, restore,
	 * compare, and auto-save tracking.
	 *
	 * @version 1.0.0
	 * @author Revolution Trading Pros
	 */

	import type { Revision, Block } from './types';

	interface Props {
		currentBlocks: Block[];
		revisions: Revision[];
		onRestore: (revision: Revision) => void;
		onCompare?: (revisionA: Revision, revisionB: Revision) => void;
	}

	let {
		currentBlocks,
		revisions,
		onRestore,
		onCompare
	}: Props = $props();

	// State
	let selectedRevision = $state<Revision | null>(null);
	let compareMode = $state(false);
	let compareRevisionA = $state<Revision | null>(null);
	let compareRevisionB = $state<Revision | null>(null);
	let showPreview = $state(false);
	let searchQuery = $state('');
	let filterType = $state<'all' | 'manual' | 'autosave'>('all');

	// Filtered revisions
	let filteredRevisions = $derived(() => {
		let filtered = [...revisions];

		// Filter by type
		if (filterType !== 'all') {
			filtered = filtered.filter(r =>
				filterType === 'autosave' ? r.isAutosave : !r.isAutosave
			);
		}

		// Filter by search
		if (searchQuery) {
			const query = searchQuery.toLowerCase();
			filtered = filtered.filter(r =>
				r.title?.toLowerCase().includes(query) ||
				r.description?.toLowerCase().includes(query) ||
				r.author?.toLowerCase().includes(query)
			);
		}

		return filtered;
	});

	// Format relative time
	function formatRelativeTime(dateStr: string): string {
		const date = new Date(dateStr);
		const now = new Date();
		const diff = now.getTime() - date.getTime();
		const seconds = Math.floor(diff / 1000);
		const minutes = Math.floor(seconds / 60);
		const hours = Math.floor(minutes / 60);
		const days = Math.floor(hours / 24);
		const weeks = Math.floor(days / 7);
		const months = Math.floor(days / 30);

		if (seconds < 60) return 'Just now';
		if (minutes < 60) return `${minutes}m ago`;
		if (hours < 24) return `${hours}h ago`;
		if (days < 7) return `${days}d ago`;
		if (weeks < 4) return `${weeks}w ago`;
		if (months < 12) return `${months}mo ago`;
		return date.toLocaleDateString();
	}

	// Format full date
	function formatFullDate(dateStr: string): string {
		const date = new Date(dateStr);
		return date.toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'long',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	// Get diff between current and revision
	function getDiff(revision: Revision): { added: number; removed: number; modified: number } {
		const currentIds = new Set(currentBlocks.map(b => b.id));
		const revisionIds = new Set(revision.blocks.map(b => b.id));

		let added = 0;
		let removed = 0;
		let modified = 0;

		// Count added blocks
		currentBlocks.forEach(block => {
			if (!revisionIds.has(block.id)) {
				added++;
			}
		});

		// Count removed blocks
		revision.blocks.forEach(block => {
			if (!currentIds.has(block.id)) {
				removed++;
			}
		});

		// Count modified blocks (simplified - just check if content differs)
		currentBlocks.forEach(current => {
			const rev = revision.blocks.find(b => b.id === current.id);
			if (rev && JSON.stringify(current.content) !== JSON.stringify(rev.content)) {
				modified++;
			}
		});

		return { added, removed, modified };
	}

	// Select revision for preview
	function selectRevision(revision: Revision): void {
		if (compareMode) {
			if (!compareRevisionA) {
				compareRevisionA = revision;
			} else if (!compareRevisionB) {
				compareRevisionB = revision;
			} else {
				compareRevisionA = revision;
				compareRevisionB = null;
			}
		} else {
			selectedRevision = revision;
			showPreview = true;
		}
	}

	// Handle restore
	function handleRestore(): void {
		if (selectedRevision) {
			onRestore(selectedRevision);
			showPreview = false;
			selectedRevision = null;
		}
	}

	// Diff view state
	let showDiffView = $state(false);

	// Handle compare
	function handleCompare(): void {
		if (compareRevisionA && compareRevisionB) {
			showDiffView = true;
			if (onCompare) {
				onCompare(compareRevisionA, compareRevisionB);
			}
		}
	}

	// Get detailed diff between two revisions
	function getDetailedDiff(revA: Revision, revB: Revision): {
		added: Block[];
		removed: Block[];
		modified: { before: Block; after: Block }[];
		unchanged: Block[];
	} {
		const blocksA = revA.blocks;
		const blocksB = revB.blocks;
		const idsA = new Set(blocksA.map(b => b.id));
		const idsB = new Set(blocksB.map(b => b.id));

		const added: Block[] = [];
		const removed: Block[] = [];
		const modified: { before: Block; after: Block }[] = [];
		const unchanged: Block[] = [];

		// Find removed (in A but not in B)
		blocksA.forEach(block => {
			if (!idsB.has(block.id)) {
				removed.push(block);
			}
		});

		// Find added (in B but not in A)
		blocksB.forEach(block => {
			if (!idsA.has(block.id)) {
				added.push(block);
			}
		});

		// Find modified and unchanged
		blocksB.forEach(blockB => {
			const blockA = blocksA.find(b => b.id === blockB.id);
			if (blockA) {
				if (JSON.stringify(blockA.content) !== JSON.stringify(blockB.content)) {
					modified.push({ before: blockA, after: blockB });
				} else {
					unchanged.push(blockB);
				}
			}
		});

		return { added, removed, modified, unchanged };
	}

	// Get text content from block for diff display
	function getBlockText(block: Block): string {
		return block.content.text || block.content.html?.replace(/<[^>]*>/g, '') || `[${block.type}]`;
	}

	// Toggle compare mode
	function toggleCompareMode(): void {
		compareMode = !compareMode;
		if (!compareMode) {
			compareRevisionA = null;
			compareRevisionB = null;
		}
	}

	// Get block preview text
	function getBlockPreview(block: Block): string {
		if (block.content.text) {
			return block.content.text.substring(0, 100) + (block.content.text.length > 100 ? '...' : '');
		}
		return `[${block.type} block]`;
	}
</script>

<div class="revision-history">
	<!-- Header -->
	<div class="header">
		<h3>Revision History</h3>
		<div class="header-actions">
			<button
				class="compare-btn"
				class:active={compareMode}
				onclick={toggleCompareMode}
			>
				{compareMode ? 'Cancel Compare' : 'Compare'}
			</button>
		</div>
	</div>

	<!-- Filters -->
	<div class="filters">
		<div class="search-box">
			<span class="search-icon">🔍</span>
			<input
				type="text"
				placeholder="Search revisions..."
				bind:value={searchQuery}
			/>
		</div>
		<div class="filter-tabs">
			<button
				class="filter-tab"
				class:active={filterType === 'all'}
				onclick={() => filterType = 'all'}
			>
				All
			</button>
			<button
				class="filter-tab"
				class:active={filterType === 'manual'}
				onclick={() => filterType = 'manual'}
			>
				Manual
			</button>
			<button
				class="filter-tab"
				class:active={filterType === 'autosave'}
				onclick={() => filterType = 'autosave'}
			>
				Autosave
			</button>
		</div>
	</div>

	<!-- Compare Mode Banner -->
	{#if compareMode}
		<div class="compare-banner">
			<p>
				{#if !compareRevisionA}
					Select first revision to compare
				{:else if !compareRevisionB}
					Select second revision to compare
				{:else}
					<button class="compare-action" onclick={handleCompare}>
						View Comparison
					</button>
				{/if}
			</p>
			{#if compareRevisionA || compareRevisionB}
				<div class="selected-revisions">
					{#if compareRevisionA}
						<span class="selected-tag">
							A: {formatRelativeTime(compareRevisionA.createdAt)}
							<button onclick={() => compareRevisionA = null}>✕</button>
						</span>
					{/if}
					{#if compareRevisionB}
						<span class="selected-tag">
							B: {formatRelativeTime(compareRevisionB.createdAt)}
							<button onclick={() => compareRevisionB = null}>✕</button>
						</span>
					{/if}
				</div>
			{/if}
		</div>
	{/if}

	<!-- Revision List -->
	<div class="revision-list">
		{#if filteredRevisions().length === 0}
			<div class="empty-state">
				<p>No revisions found</p>
			</div>
		{:else}
			{#each filteredRevisions() as revision, index}
				{@const diff = getDiff(revision)}
				{@const isSelected = compareMode
					? (compareRevisionA?.id === revision.id || compareRevisionB?.id === revision.id)
					: selectedRevision?.id === revision.id}
				<button
					class="revision-item"
					class:selected={isSelected}
					class:compare-a={compareRevisionA?.id === revision.id}
					class:compare-b={compareRevisionB?.id === revision.id}
					onclick={() => selectRevision(revision)}
				>
					<div class="revision-header">
						<div class="revision-meta">
							{#if revision.isAutosave}
								<span class="autosave-badge">Auto</span>
							{/if}
							<span class="revision-time" title={formatFullDate(revision.createdAt)}>
								{formatRelativeTime(revision.createdAt)}
							</span>
							{#if index === 0 && !revision.isAutosave}
								<span class="current-badge">Current</span>
							{/if}
						</div>
						<div class="revision-diff">
							{#if diff.added > 0}
								<span class="diff-added">+{diff.added}</span>
							{/if}
							{#if diff.removed > 0}
								<span class="diff-removed">-{diff.removed}</span>
							{/if}
							{#if diff.modified > 0}
								<span class="diff-modified">~{diff.modified}</span>
							{/if}
						</div>
					</div>
					{#if revision.title}
						<div class="revision-title">{revision.title}</div>
					{/if}
					{#if revision.description}
						<div class="revision-description">{revision.description}</div>
					{/if}
					<div class="revision-info">
						{#if revision.author}
							<span class="revision-author">
								<span class="author-avatar">
									{revision.author.charAt(0).toUpperCase()}
								</span>
								{revision.author}
							</span>
						{/if}
						<span class="revision-blocks">{revision.blocks.length} blocks</span>
					</div>
				</button>
			{/each}
		{/if}
	</div>

	<!-- Preview Modal -->
	{#if showPreview && selectedRevision && !compareMode}
		<div 
			class="preview-overlay" 
			onclick={() => showPreview = false}
			onkeydown={(e: KeyboardEvent) => { if (e.key === 'Escape') showPreview = false; }}
			role="dialog"
			aria-modal="true"
			aria-labelledby="preview-modal-title"
			tabindex="-1"
		>
			<div class="preview-modal" role="document">
				<div class="preview-header">
					<div>
						<h4 id="preview-modal-title">Revision Preview</h4>
						<p class="preview-date">{formatFullDate(selectedRevision.createdAt)}</p>
					</div>
					<button class="close-btn" onclick={() => showPreview = false}>✕</button>
				</div>

				<div class="preview-content">
					{#if selectedRevision.title}
						<h5 class="preview-title">{selectedRevision.title}</h5>
					{/if}
					{#if selectedRevision.description}
						<p class="preview-description">{selectedRevision.description}</p>
					{/if}

					{#if selectedRevision}
						{@const previewDiff = getDiff(selectedRevision)}
						{#if previewDiff}
							<div class="diff-summary">
								<span class="diff-item added">
									<strong>{previewDiff.added}</strong> added
								</span>
								<span class="diff-item removed">
									<strong>{previewDiff.removed}</strong> removed
								</span>
								<span class="diff-item modified">
									<strong>{previewDiff.modified}</strong> modified
								</span>
							</div>
						{/if}
					{/if}

					<div class="preview-blocks">
						<h6>Blocks in this revision:</h6>
						<div class="block-list">
							{#each selectedRevision.blocks as block}
								<div class="block-item">
									<span class="block-type">{block.type}</span>
									<span class="block-preview">{getBlockPreview(block)}</span>
								</div>
							{/each}
						</div>
					</div>

				<div class="preview-footer">
					<button class="cancel-btn" onclick={() => showPreview = false}>
						Cancel
					</button>
					<button class="restore-btn" onclick={handleRestore}>
						Restore This Version
					</button>
				</div>
			</div>
		</div>
		</div>
	{/if}

	<!-- Diff Viewer Modal -->
	{#if showDiffView && compareRevisionA && compareRevisionB}
		{@const detailedDiff = getDetailedDiff(compareRevisionA, compareRevisionB)}
		<div
			class="diff-overlay"
			onclick={() => showDiffView = false}
			onkeydown={(e: KeyboardEvent) => { if (e.key === 'Escape') showDiffView = false; }}
			role="dialog"
			aria-modal="true"
			aria-labelledby="diff-modal-title"
			tabindex="-1"
		>
			<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
			<div 
				class="diff-modal" 
				onclick={(e: MouseEvent) => e.stopPropagation()}
				onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
				role="document"
			>
				<div class="diff-header">
					<h4 id="diff-modal-title">Revision Comparison</h4>
					<div class="diff-header-meta">
						<span class="revision-label revision-a">
							{formatRelativeTime(compareRevisionA.createdAt)}
						</span>
						<span class="diff-arrow">→</span>
						<span class="revision-label revision-b">
							{formatRelativeTime(compareRevisionB.createdAt)}
						</span>
					</div>
					<button class="close-btn" onclick={() => showDiffView = false} aria-label="Close">✕</button>
				</div>

				<div class="diff-stats">
					<span class="stat stat-added">
						<strong>+{detailedDiff.added.length}</strong> added
					</span>
					<span class="stat stat-removed">
						<strong>-{detailedDiff.removed.length}</strong> removed
					</span>
					<span class="stat stat-modified">
						<strong>~{detailedDiff.modified.length}</strong> modified
					</span>
					<span class="stat stat-unchanged">
						<strong>{detailedDiff.unchanged.length}</strong> unchanged
					</span>
				</div>

				<div class="diff-content">
					<!-- Removed blocks -->
					{#if detailedDiff.removed.length > 0}
						<div class="diff-section">
							<h5 class="section-title removed-title">Removed Blocks</h5>
							{#each detailedDiff.removed as block}
								<div class="diff-block removed">
									<div class="block-header">
										<span class="block-type-badge">{block.type}</span>
										<span class="diff-indicator">−</span>
									</div>
									<div class="block-content">
										{getBlockText(block)}
									</div>
								</div>
							{/each}
						</div>
					{/if}

					<!-- Added blocks -->
					{#if detailedDiff.added.length > 0}
						<div class="diff-section">
							<h5 class="section-title added-title">Added Blocks</h5>
							{#each detailedDiff.added as block}
								<div class="diff-block added">
									<div class="block-header">
										<span class="block-type-badge">{block.type}</span>
										<span class="diff-indicator">+</span>
									</div>
									<div class="block-content">
										{getBlockText(block)}
									</div>
								</div>
							{/each}
						</div>
					{/if}

					<!-- Modified blocks (side-by-side) -->
					{#if detailedDiff.modified.length > 0}
						<div class="diff-section">
							<h5 class="section-title modified-title">Modified Blocks</h5>
							{#each detailedDiff.modified as { before, after }}
								<div class="diff-block-pair">
									<div class="diff-block before">
										<div class="block-header">
											<span class="block-type-badge">{before.type}</span>
											<span class="version-label">Before</span>
										</div>
										<div class="block-content">
											{getBlockText(before)}
										</div>
									</div>
									<div class="diff-arrow-container">
										<span class="change-arrow">→</span>
									</div>
									<div class="diff-block after">
										<div class="block-header">
											<span class="block-type-badge">{after.type}</span>
											<span class="version-label">After</span>
										</div>
										<div class="block-content">
											{getBlockText(after)}
										</div>
									</div>
								</div>
							{/each}
						</div>
					{/if}

					<!-- Unchanged blocks (collapsed) -->
					{#if detailedDiff.unchanged.length > 0}
						<details class="diff-section unchanged-section">
							<summary class="section-title unchanged-title">
								{detailedDiff.unchanged.length} Unchanged Blocks
							</summary>
							{#each detailedDiff.unchanged as block}
								<div class="diff-block unchanged">
									<div class="block-header">
										<span class="block-type-badge">{block.type}</span>
									</div>
									<div class="block-content">
										{getBlockText(block)}
									</div>
								</div>
							{/each}
						</details>
					{/if}

					{#if detailedDiff.added.length === 0 && detailedDiff.removed.length === 0 && detailedDiff.modified.length === 0}
						<div class="no-changes">
							<p>No differences found between these revisions.</p>
						</div>
					{/if}
				</div>

				<div class="diff-footer">
					<button class="cancel-btn" onclick={() => showDiffView = false}>
						Close
					</button>
					<button class="restore-btn" onclick={() => { if (compareRevisionB) onRestore(compareRevisionB); showDiffView = false; }}>
						Restore Newer Version
					</button>
				</div>
			</div>
		</div>
	{/if}
</div>

<style>
	.revision-history {
		display: flex;
		flex-direction: column;
		height: 100%;
	}

	.header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem;
		border-bottom: 1px solid var(--border-color, #e5e7eb);
	}

	.header h3 {
		margin: 0;
		font-size: 1rem;
		font-weight: 600;
		color: var(--text-primary, #1f2937);
	}

	.compare-btn {
		padding: 0.375rem 0.75rem;
		background: var(--bg-secondary, #f9fafb);
		border: 1px solid var(--border-color, #e5e7eb);
		border-radius: 0.375rem;
		font-size: 0.75rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.compare-btn:hover {
		background: var(--bg-hover, #f3f4f6);
	}

	.compare-btn.active {
		background: var(--primary, #3b82f6);
		color: white;
		border-color: var(--primary, #3b82f6);
	}

	.filters {
		padding: 0.75rem 1rem;
		border-bottom: 1px solid var(--border-color, #e5e7eb);
	}

	.search-box {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0.75rem;
		background: var(--bg-secondary, #f9fafb);
		border: 1px solid var(--border-color, #e5e7eb);
		border-radius: 0.375rem;
		margin-bottom: 0.75rem;
	}

	.search-icon {
		font-size: 0.875rem;
		opacity: 0.5;
	}

	.search-box input {
		flex: 1;
		border: none;
		background: transparent;
		font-size: 0.875rem;
		outline: none;
	}

	.filter-tabs {
		display: flex;
		gap: 0.25rem;
	}

	.filter-tab {
		padding: 0.375rem 0.75rem;
		background: transparent;
		border: none;
		border-radius: 0.25rem;
		font-size: 0.75rem;
		color: var(--text-secondary, #6b7280);
		cursor: pointer;
		transition: all 0.2s;
	}

	.filter-tab:hover {
		background: var(--bg-secondary, #f9fafb);
	}

	.filter-tab.active {
		background: var(--primary, #3b82f6);
		color: white;
	}

	.compare-banner {
		padding: 0.75rem 1rem;
		background: #eff6ff;
		border-bottom: 1px solid #bfdbfe;
	}

	.compare-banner p {
		margin: 0;
		font-size: 0.8125rem;
		color: #1e40af;
	}

	.compare-action {
		padding: 0.375rem 0.75rem;
		background: var(--primary, #3b82f6);
		color: white;
		border: none;
		border-radius: 0.25rem;
		font-size: 0.75rem;
		cursor: pointer;
	}

	.selected-revisions {
		display: flex;
		gap: 0.5rem;
		margin-top: 0.5rem;
	}

	.selected-tag {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.25rem 0.5rem;
		background: white;
		border-radius: 0.25rem;
		font-size: 0.75rem;
		color: #1e40af;
	}

	.selected-tag button {
		padding: 0;
		background: none;
		border: none;
		color: #1e40af;
		cursor: pointer;
		opacity: 0.7;
	}

	.selected-tag button:hover {
		opacity: 1;
	}

	.revision-list {
		flex: 1;
		overflow-y: auto;
		padding: 0.5rem;
	}

	.empty-state {
		padding: 2rem;
		text-align: center;
		color: var(--text-tertiary, #9ca3af);
		font-size: 0.875rem;
	}

	.revision-item {
		width: 100%;
		padding: 0.875rem;
		margin-bottom: 0.5rem;
		background: var(--bg-primary, #ffffff);
		border: 1px solid var(--border-color, #e5e7eb);
		border-radius: 0.5rem;
		text-align: left;
		cursor: pointer;
		transition: all 0.2s;
	}

	.revision-item:hover {
		border-color: var(--primary, #3b82f6);
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
	}

	.revision-item.selected {
		border-color: var(--primary, #3b82f6);
		background: #eff6ff;
	}

	.revision-item.compare-a {
		border-color: #10b981;
		background: #ecfdf5;
	}

	.revision-item.compare-b {
		border-color: #f59e0b;
		background: #fffbeb;
	}

	.revision-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.375rem;
	}

	.revision-meta {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.autosave-badge {
		padding: 0.125rem 0.375rem;
		background: var(--bg-secondary, #f9fafb);
		border-radius: 0.25rem;
		font-size: 0.625rem;
		font-weight: 600;
		color: var(--text-secondary, #6b7280);
		text-transform: uppercase;
	}

	.revision-time {
		font-size: 0.8125rem;
		font-weight: 600;
		color: var(--text-primary, #1f2937);
	}

	.current-badge {
		padding: 0.125rem 0.375rem;
		background: #10b981;
		color: white;
		border-radius: 0.25rem;
		font-size: 0.625rem;
		font-weight: 600;
		text-transform: uppercase;
	}

	.revision-diff {
		display: flex;
		gap: 0.375rem;
		font-size: 0.6875rem;
		font-weight: 600;
	}

	.diff-added {
		color: #10b981;
	}

	.diff-removed {
		color: #ef4444;
	}

	.diff-modified {
		color: #f59e0b;
	}

	.revision-title {
		font-size: 0.875rem;
		font-weight: 500;
		color: var(--text-primary, #1f2937);
		margin-bottom: 0.25rem;
	}

	.revision-description {
		font-size: 0.75rem;
		color: var(--text-secondary, #6b7280);
		margin-bottom: 0.5rem;
		line-height: 1.4;
	}

	.revision-info {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		font-size: 0.6875rem;
		color: var(--text-tertiary, #9ca3af);
	}

	.revision-author {
		display: flex;
		align-items: center;
		gap: 0.375rem;
	}

	.author-avatar {
		width: 16px;
		height: 16px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--primary, #3b82f6);
		color: white;
		border-radius: 50%;
		font-size: 0.5rem;
		font-weight: 600;
	}

	/* Preview Modal */
	.preview-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.5);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 2rem;
	}

	.preview-modal {
		width: 100%;
		max-width: 600px;
		max-height: 80vh;
		background: var(--bg-primary, #ffffff);
		border-radius: 0.75rem;
		display: flex;
		flex-direction: column;
		overflow: hidden;
		box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1),
			0 10px 10px -5px rgba(0, 0, 0, 0.04);
	}

	.preview-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		padding: 1.25rem;
		border-bottom: 1px solid var(--border-color, #e5e7eb);
	}

	.preview-header h4 {
		margin: 0;
		font-size: 1.125rem;
		font-weight: 600;
		color: var(--text-primary, #1f2937);
	}

	.preview-date {
		margin: 0.25rem 0 0;
		font-size: 0.8125rem;
		color: var(--text-secondary, #6b7280);
	}

	.close-btn {
		padding: 0.5rem;
		background: var(--bg-secondary, #f9fafb);
		border: none;
		border-radius: 0.375rem;
		cursor: pointer;
		font-size: 1rem;
		line-height: 1;
	}

	.close-btn:hover {
		background: var(--bg-hover, #f3f4f6);
	}

	.preview-content {
		flex: 1;
		overflow-y: auto;
		padding: 1.25rem;
	}

	.preview-title {
		margin: 0 0 0.5rem;
		font-size: 1rem;
		font-weight: 600;
		color: var(--text-primary, #1f2937);
	}

	.preview-description {
		margin: 0 0 1rem;
		font-size: 0.875rem;
		color: var(--text-secondary, #6b7280);
		line-height: 1.5;
	}

	.diff-summary {
		display: flex;
		gap: 1rem;
	}

	.diff-item {
		font-size: 0.8125rem;
	}

	.diff-item.added {
		color: #10b981;
	}

	.diff-item.removed {
		color: #ef4444;
	}

	.diff-item.modified {
		color: #f59e0b;
	}

	.preview-blocks h6 {
		margin: 0 0 0.75rem;
		font-size: 0.8125rem;
		font-weight: 600;
		color: var(--text-primary, #1f2937);
	}

	.block-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.block-item {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
		padding: 0.625rem 0.75rem;
		background: var(--bg-secondary, #f9fafb);
		border-radius: 0.375rem;
	}

	.block-type {
		flex-shrink: 0;
		padding: 0.125rem 0.375rem;
		background: var(--primary, #3b82f6);
		color: white;
		border-radius: 0.25rem;
		font-size: 0.625rem;
		font-weight: 600;
		text-transform: uppercase;
	}

	.block-preview {
		flex: 1;
		font-size: 0.8125rem;
		color: var(--text-secondary, #6b7280);
		line-height: 1.4;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.preview-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1rem 1.25rem;
		border-top: 1px solid var(--border-color, #e5e7eb);
		background: var(--bg-secondary, #f9fafb);
	}

	.cancel-btn,
	.restore-btn {
		padding: 0.625rem 1.25rem;
		border-radius: 0.375rem;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.cancel-btn {
		background: var(--bg-primary, #ffffff);
		border: 1px solid var(--border-color, #e5e7eb);
		color: var(--text-secondary, #6b7280);
	}

	.cancel-btn:hover {
		background: var(--bg-hover, #f3f4f6);
	}

	.restore-btn {
		background: var(--primary, #3b82f6);
		border: none;
		color: white;
	}

	.restore-btn:hover {
		background: var(--primary-hover, #2563eb);
	}

	/* Diff Viewer Modal Styles */
	.diff-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.6);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1100;
		padding: 1rem;
	}

	.diff-modal {
		width: 100%;
		max-width: 900px;
		max-height: 90vh;
		background: var(--bg-primary, #ffffff);
		border-radius: 0.75rem;
		display: flex;
		flex-direction: column;
		overflow: hidden;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
	}

	.diff-header {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.25rem;
		border-bottom: 1px solid var(--border-color, #e5e7eb);
	}

	.diff-header h4 {
		margin: 0;
		font-size: 1.125rem;
		font-weight: 600;
		color: var(--text-primary, #1f2937);
	}

	.diff-header-meta {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-left: auto;
		margin-right: 1rem;
	}

	.revision-label {
		padding: 0.25rem 0.625rem;
		border-radius: 0.25rem;
		font-size: 0.75rem;
		font-weight: 600;
	}

	.revision-label.revision-a {
		background: #fef2f2;
		color: #b91c1c;
	}

	.revision-label.revision-b {
		background: #ecfdf5;
		color: #047857;
	}

	.diff-arrow {
		color: var(--text-tertiary, #9ca3af);
	}

	.diff-stats {
		display: flex;
		gap: 1.5rem;
		padding: 0.875rem 1.25rem;
		background: var(--bg-secondary, #f9fafb);
		border-bottom: 1px solid var(--border-color, #e5e7eb);
	}

	.stat {
		font-size: 0.8125rem;
	}

	.stat-added {
		color: #059669;
	}

	.stat-removed {
		color: #dc2626;
	}

	.stat-modified {
		color: #d97706;
	}

	.stat-unchanged {
		color: var(--text-tertiary, #9ca3af);
	}

	.diff-content {
		flex: 1;
		overflow-y: auto;
		padding: 1.25rem;
	}

	.diff-section {
		margin-bottom: 1.5rem;
	}

	.diff-section:last-child {
		margin-bottom: 0;
	}

	.section-title {
		margin: 0 0 0.75rem;
		padding: 0.5rem 0.75rem;
		border-radius: 0.375rem;
		font-size: 0.8125rem;
		font-weight: 600;
	}

	.removed-title {
		background: #fef2f2;
		color: #b91c1c;
	}

	.added-title {
		background: #ecfdf5;
		color: #047857;
	}

	.modified-title {
		background: #fffbeb;
		color: #b45309;
	}

	.unchanged-title {
		background: var(--bg-secondary, #f9fafb);
		color: var(--text-secondary, #6b7280);
		cursor: pointer;
	}

	.unchanged-section {
		border: 1px solid var(--border-color, #e5e7eb);
		border-radius: 0.5rem;
		padding: 0;
	}

	.unchanged-section .section-title {
		margin: 0;
		padding: 0.75rem 1rem;
		border-radius: 0.5rem;
	}

	.unchanged-section[open] .section-title {
		border-bottom: 1px solid var(--border-color, #e5e7eb);
		border-radius: 0.5rem 0.5rem 0 0;
	}

	.unchanged-section .diff-block {
		margin: 0.5rem 1rem;
	}

	.unchanged-section .diff-block:last-child {
		margin-bottom: 1rem;
	}

	.diff-block {
		padding: 0.875rem;
		border-radius: 0.5rem;
		margin-bottom: 0.5rem;
	}

	.diff-block.removed {
		background: #fef2f2;
		border-left: 3px solid #ef4444;
	}

	.diff-block.added {
		background: #ecfdf5;
		border-left: 3px solid #10b981;
	}

	.diff-block.before {
		background: #fef2f2;
		border: 1px solid #fecaca;
	}

	.diff-block.after {
		background: #ecfdf5;
		border: 1px solid #a7f3d0;
	}

	.diff-block.unchanged {
		background: var(--bg-secondary, #f9fafb);
		border: 1px solid var(--border-color, #e5e7eb);
	}

	.diff-block-pair {
		display: grid;
		grid-template-columns: 1fr auto 1fr;
		gap: 0.5rem;
		margin-bottom: 0.75rem;
		align-items: stretch;
	}

	.diff-arrow-container {
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 0 0.5rem;
	}

	.change-arrow {
		font-size: 1.25rem;
		color: var(--text-tertiary, #9ca3af);
	}

	.block-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-bottom: 0.5rem;
	}

	.block-type-badge {
		padding: 0.125rem 0.5rem;
		background: rgba(0, 0, 0, 0.1);
		border-radius: 0.25rem;
		font-size: 0.625rem;
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.025em;
	}

	.diff-indicator {
		margin-left: auto;
		font-size: 1rem;
		font-weight: 700;
	}

	.diff-block.removed .diff-indicator {
		color: #dc2626;
	}

	.diff-block.added .diff-indicator {
		color: #059669;
	}

	.version-label {
		font-size: 0.6875rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.025em;
		opacity: 0.7;
	}

	.block-content {
		font-size: 0.875rem;
		line-height: 1.5;
		color: var(--text-secondary, #6b7280);
		word-break: break-word;
	}

	.no-changes {
		padding: 3rem;
		text-align: center;
	}

	.no-changes p {
		margin: 0;
		color: var(--text-tertiary, #9ca3af);
	}

	.diff-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1rem 1.25rem;
		border-top: 1px solid var(--border-color, #e5e7eb);
		background: var(--bg-secondary, #f9fafb);
	}

	@media (max-width: 640px) {
		.diff-block-pair {
			grid-template-columns: 1fr;
		}

		.diff-arrow-container {
			padding: 0.5rem 0;
		}

		.change-arrow {
			transform: rotate(90deg);
		}
	}
</style>

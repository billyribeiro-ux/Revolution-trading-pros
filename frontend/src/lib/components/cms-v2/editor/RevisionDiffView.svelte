<!--
  RevisionDiffView - Enterprise-Grade Revision Comparison Component
  ================================================================
  A comprehensive diff viewer for CMS content revisions with:
  - Side-by-side visual diff between any two revisions
  - Visual diff highlighting (additions, deletions, modifications)
  - One-click revision restore
  - Revision metadata display
  - Synchronized scrolling
  - Accessible keyboard navigation

  @version 1.0.0
  @author Revolution Trading Pros
  @date January 2026
-->

<script lang="ts">
	import type { PageBlock } from '$lib/page-builder/types';

	// =========================================================================
	// Types
	// =========================================================================

	export interface Revision {
		id: string;
		revision_number: number;
		created_at: string;
		created_by: string | null;
		change_summary: string | null;
		changed_fields: string[] | null;
		data?: RevisionData;
	}

	interface RevisionData {
		title?: string;
		content?: string;
		content_blocks?: PageBlock[];
		[key: string]: unknown;
	}

	interface DiffSegment {
		type: 'equal' | 'insert' | 'delete';
		text: string;
	}

	interface FieldDiff {
		field: string;
		label: string;
		type: 'added' | 'removed' | 'modified' | 'unchanged';
		fromValue: unknown;
		toValue: unknown;
		textDiff?: DiffSegment[];
	}

	interface DiffStats {
		added: number;
		removed: number;
		modified: number;
		unchanged: number;
	}

	// =========================================================================
	// Props
	// =========================================================================

	interface Props {
		contentId: string;
		revisions: Revision[];
		onRestore: (version: number) => Promise<void>;
		onClose: () => void;
	}

	let { contentId, revisions, onRestore, onClose }: Props = $props();

	// =========================================================================
	// State
	// =========================================================================

	let fromRevision = $state<Revision | null>(null);
	let toRevision = $state<Revision | null>(null);
	let selectionMode = $state<'from' | 'to'>('from');
	let isRestoring = $state(false);
	let showBlocksExpanded = $state(false);
	let searchQuery = $state('');
	let filterType = $state<'all' | 'manual' | 'auto'>('all');

	// References for synchronized scrolling
	let leftPaneRef = $state<HTMLDivElement | null>(null);
	let rightPaneRef = $state<HTMLDivElement | null>(null);
	let isSyncingScroll = $state(false);

	// =========================================================================
	// Derived State
	// =========================================================================

	let sortedRevisions = $derived(
		[...revisions].sort((a, b) => b.revision_number - a.revision_number)
	);

	let filteredRevisions = $derived(() => {
		let filtered = sortedRevisions;

		// Filter by type
		if (filterType === 'manual') {
			filtered = filtered.filter((r) => r.change_summary && !r.change_summary.includes('Auto'));
		} else if (filterType === 'auto') {
			filtered = filtered.filter((r) => r.change_summary?.includes('Auto'));
		}

		// Filter by search
		if (searchQuery.trim()) {
			const query = searchQuery.toLowerCase();
			filtered = filtered.filter(
				(r) =>
					r.change_summary?.toLowerCase().includes(query) ||
					r.created_by?.toLowerCase().includes(query) ||
					r.changed_fields?.some((f) => f.toLowerCase().includes(query))
			);
		}

		return filtered;
	});

	let fieldDiffs = $derived<FieldDiff[]>(() => {
		if (!fromRevision?.data || !toRevision?.data) return [];
		return computeFieldDiffs(fromRevision.data, toRevision.data);
	});

	let diffStats = $derived<DiffStats>(() => {
		const diffs = fieldDiffs();
		return {
			added: diffs.filter((d) => d.type === 'added').length,
			removed: diffs.filter((d) => d.type === 'removed').length,
			modified: diffs.filter((d) => d.type === 'modified').length,
			unchanged: diffs.filter((d) => d.type === 'unchanged').length
		};
	});

	let blockDiffs = $derived(() => {
		if (!fromRevision?.data?.content_blocks || !toRevision?.data?.content_blocks) return null;
		return computeBlockDiffs(fromRevision.data.content_blocks, toRevision.data.content_blocks);
	});

	// =========================================================================
	// Effects
	// =========================================================================

	$effect(() => {
		// Auto-select first two revisions if available
		if (sortedRevisions.length >= 2 && !fromRevision && !toRevision) {
			toRevision = sortedRevisions[0];
			fromRevision = sortedRevisions[1];
		} else if (sortedRevisions.length === 1 && !toRevision) {
			toRevision = sortedRevisions[0];
		}
	});

	// =========================================================================
	// Text Diffing Algorithm (Myers Diff)
	// =========================================================================

	function computeTextDiff(oldText: string, newText: string): DiffSegment[] {
		if (oldText === newText) {
			return [{ type: 'equal', text: oldText }];
		}

		if (!oldText) {
			return [{ type: 'insert', text: newText }];
		}

		if (!newText) {
			return [{ type: 'delete', text: oldText }];
		}

		// Simple word-level diff for readability
		const oldWords = oldText.split(/(\s+)/);
		const newWords = newText.split(/(\s+)/);

		const result: DiffSegment[] = [];
		const lcs = longestCommonSubsequence(oldWords, newWords);

		let oldIdx = 0;
		let newIdx = 0;
		let lcsIdx = 0;

		while (oldIdx < oldWords.length || newIdx < newWords.length) {
			if (lcsIdx < lcs.length && oldWords[oldIdx] === lcs[lcsIdx] && newWords[newIdx] === lcs[lcsIdx]) {
				// Equal
				if (result.length > 0 && result[result.length - 1].type === 'equal') {
					result[result.length - 1].text += oldWords[oldIdx];
				} else {
					result.push({ type: 'equal', text: oldWords[oldIdx] });
				}
				oldIdx++;
				newIdx++;
				lcsIdx++;
			} else if (oldIdx < oldWords.length && (lcsIdx >= lcs.length || oldWords[oldIdx] !== lcs[lcsIdx])) {
				// Delete
				if (result.length > 0 && result[result.length - 1].type === 'delete') {
					result[result.length - 1].text += oldWords[oldIdx];
				} else {
					result.push({ type: 'delete', text: oldWords[oldIdx] });
				}
				oldIdx++;
			} else if (newIdx < newWords.length) {
				// Insert
				if (result.length > 0 && result[result.length - 1].type === 'insert') {
					result[result.length - 1].text += newWords[newIdx];
				} else {
					result.push({ type: 'insert', text: newWords[newIdx] });
				}
				newIdx++;
			}
		}

		return result;
	}

	function longestCommonSubsequence(a: string[], b: string[]): string[] {
		const m = a.length;
		const n = b.length;
		const dp: number[][] = Array(m + 1)
			.fill(null)
			.map(() => Array(n + 1).fill(0));

		for (let i = 1; i <= m; i++) {
			for (let j = 1; j <= n; j++) {
				if (a[i - 1] === b[j - 1]) {
					dp[i][j] = dp[i - 1][j - 1] + 1;
				} else {
					dp[i][j] = Math.max(dp[i - 1][j], dp[i][j - 1]);
				}
			}
		}

		// Backtrack to find LCS
		const lcs: string[] = [];
		let i = m,
			j = n;
		while (i > 0 && j > 0) {
			if (a[i - 1] === b[j - 1]) {
				lcs.unshift(a[i - 1]);
				i--;
				j--;
			} else if (dp[i - 1][j] > dp[i][j - 1]) {
				i--;
			} else {
				j--;
			}
		}

		return lcs;
	}

	// =========================================================================
	// Field Diffing
	// =========================================================================

	function computeFieldDiffs(fromData: RevisionData, toData: RevisionData): FieldDiff[] {
		const diffs: FieldDiff[] = [];
		const allKeys = new Set([...Object.keys(fromData), ...Object.keys(toData)]);

		// Fields to skip (internal or handled separately)
		const skipFields = new Set(['content_blocks', 'id', 'created_at', 'updated_at', 'version']);

		// Human-readable field labels
		const fieldLabels: Record<string, string> = {
			title: 'Title',
			subtitle: 'Subtitle',
			slug: 'URL Slug',
			content: 'Content',
			excerpt: 'Excerpt',
			meta_title: 'Meta Title',
			meta_description: 'Meta Description',
			status: 'Status',
			featured_image_id: 'Featured Image',
			author_id: 'Author',
			template: 'Template',
			locale: 'Language',
			canonical_url: 'Canonical URL',
			robots_directives: 'Robots Directives'
		};

		for (const key of allKeys) {
			if (skipFields.has(key)) continue;

			const fromValue = fromData[key];
			const toValue = toData[key];
			const label = fieldLabels[key] || formatFieldName(key);

			if (fromValue === undefined && toValue !== undefined) {
				diffs.push({
					field: key,
					label,
					type: 'added',
					fromValue,
					toValue
				});
			} else if (fromValue !== undefined && toValue === undefined) {
				diffs.push({
					field: key,
					label,
					type: 'removed',
					fromValue,
					toValue
				});
			} else if (JSON.stringify(fromValue) !== JSON.stringify(toValue)) {
				const isTextContent = typeof fromValue === 'string' && typeof toValue === 'string';
				diffs.push({
					field: key,
					label,
					type: 'modified',
					fromValue,
					toValue,
					textDiff: isTextContent ? computeTextDiff(fromValue, toValue) : undefined
				});
			} else {
				diffs.push({
					field: key,
					label,
					type: 'unchanged',
					fromValue,
					toValue
				});
			}
		}

		// Sort: modified first, then added, removed, unchanged
		const typeOrder = { modified: 0, added: 1, removed: 2, unchanged: 3 };
		return diffs.sort((a, b) => typeOrder[a.type] - typeOrder[b.type]);
	}

	function formatFieldName(key: string): string {
		return key
			.replace(/_/g, ' ')
			.replace(/([A-Z])/g, ' $1')
			.replace(/^./, (str) => str.toUpperCase())
			.trim();
	}

	// =========================================================================
	// Block Diffing
	// =========================================================================

	interface BlockDiff {
		type: 'added' | 'removed' | 'modified' | 'unchanged';
		fromBlock?: PageBlock;
		toBlock?: PageBlock;
	}

	function computeBlockDiffs(fromBlocks: PageBlock[], toBlocks: PageBlock[]): BlockDiff[] {
		const diffs: BlockDiff[] = [];
		const fromBlockMap = new Map(fromBlocks.map((b) => [b.id, b]));
		const toBlockMap = new Map(toBlocks.map((b) => [b.id, b]));

		// Find removed and modified blocks
		for (const block of fromBlocks) {
			const toBlock = toBlockMap.get(block.id);
			if (!toBlock) {
				diffs.push({ type: 'removed', fromBlock: block });
			} else if (JSON.stringify(block.config) !== JSON.stringify(toBlock.config)) {
				diffs.push({ type: 'modified', fromBlock: block, toBlock });
			} else {
				diffs.push({ type: 'unchanged', fromBlock: block, toBlock });
			}
		}

		// Find added blocks
		for (const block of toBlocks) {
			if (!fromBlockMap.has(block.id)) {
				diffs.push({ type: 'added', toBlock: block });
			}
		}

		return diffs;
	}

	// =========================================================================
	// Utility Functions
	// =========================================================================

	function formatRelativeTime(dateStr: string): string {
		const date = new Date(dateStr);
		const now = new Date();
		const diff = now.getTime() - date.getTime();
		const seconds = Math.floor(diff / 1000);
		const minutes = Math.floor(seconds / 60);
		const hours = Math.floor(minutes / 60);
		const days = Math.floor(hours / 24);

		if (seconds < 60) return 'Just now';
		if (minutes < 60) return `${minutes}m ago`;
		if (hours < 24) return `${hours}h ago`;
		if (days < 7) return `${days}d ago`;
		if (days < 30) return `${Math.floor(days / 7)}w ago`;
		return date.toLocaleDateString();
	}

	function formatFullDate(dateStr: string): string {
		return new Date(dateStr).toLocaleString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function getInitials(name: string | null): string {
		if (!name) return '?';
		return name
			.split(' ')
			.map((n) => n[0])
			.join('')
			.toUpperCase()
			.slice(0, 2);
	}

	function getChangeTypeBadge(summary: string | null): { text: string; class: string } {
		if (!summary) return { text: 'Unknown', class: 'bg-gray-100 text-gray-600' };

		const lower = summary.toLowerCase();
		if (lower.includes('create') || lower.includes('initial'))
			return { text: 'Created', class: 'bg-green-100 text-green-700' };
		if (lower.includes('publish'))
			return { text: 'Published', class: 'bg-blue-100 text-blue-700' };
		if (lower.includes('auto'))
			return { text: 'Autosave', class: 'bg-gray-100 text-gray-600' };
		if (lower.includes('restore'))
			return { text: 'Restored', class: 'bg-purple-100 text-purple-700' };
		return { text: 'Updated', class: 'bg-amber-100 text-amber-700' };
	}

	function truncateValue(value: unknown, maxLength = 100): string {
		const str = typeof value === 'string' ? value : JSON.stringify(value);
		if (str.length <= maxLength) return str;
		return str.slice(0, maxLength) + '...';
	}

	// =========================================================================
	// Event Handlers
	// =========================================================================

	function selectRevision(revision: Revision): void {
		if (selectionMode === 'from') {
			fromRevision = revision;
			selectionMode = 'to';
		} else {
			toRevision = revision;
			selectionMode = 'from';
		}
	}

	function swapRevisions(): void {
		const temp = fromRevision;
		fromRevision = toRevision;
		toRevision = temp;
	}

	async function handleRestore(): Promise<void> {
		if (!toRevision || isRestoring) return;

		isRestoring = true;
		try {
			await onRestore(toRevision.revision_number);
		} finally {
			isRestoring = false;
		}
	}

	function handleKeyDown(event: KeyboardEvent): void {
		if (event.key === 'Escape') {
			onClose();
		}
	}

	function syncScroll(source: 'left' | 'right'): void {
		if (isSyncingScroll) return;

		isSyncingScroll = true;
		requestAnimationFrame(() => {
			const sourcePane = source === 'left' ? leftPaneRef : rightPaneRef;
			const targetPane = source === 'left' ? rightPaneRef : leftPaneRef;

			if (sourcePane && targetPane) {
				const scrollRatio = sourcePane.scrollTop / (sourcePane.scrollHeight - sourcePane.clientHeight);
				targetPane.scrollTop = scrollRatio * (targetPane.scrollHeight - targetPane.clientHeight);
			}

			isSyncingScroll = false;
		});
	}
</script>

<svelte:window onkeydown={handleKeyDown} />

<!-- Modal Overlay -->
<div
	class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4 backdrop-blur-sm"
	role="dialog"
	aria-modal="true"
	aria-labelledby="diff-modal-title"
>
	<!-- Modal Container -->
	<div
		class="flex h-[90vh] w-full max-w-7xl flex-col overflow-hidden rounded-xl bg-white shadow-2xl"
		role="document"
	>
		<!-- Header -->
		<header class="flex shrink-0 items-center justify-between border-b border-gray-200 px-6 py-4">
			<div class="flex items-center gap-4">
				<h2 id="diff-modal-title" class="text-lg font-semibold text-gray-900">
					Revision Comparison
				</h2>
				{#if fromRevision && toRevision}
					<div class="flex items-center gap-2 text-sm text-gray-500">
						<span class="rounded bg-red-50 px-2 py-0.5 font-medium text-red-700">
							v{fromRevision.revision_number}
						</span>
						<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3" />
						</svg>
						<span class="rounded bg-green-50 px-2 py-0.5 font-medium text-green-700">
							v{toRevision.revision_number}
						</span>
						<button
							class="ml-1 rounded p-1 text-gray-400 transition-colors hover:bg-gray-100 hover:text-gray-600"
							onclick={swapRevisions}
							title="Swap revisions"
							aria-label="Swap from and to revisions"
						>
							<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16V4m0 0L3 8m4-4l4 4m6 0v12m0 0l4-4m-4 4l-4-4" />
							</svg>
						</button>
					</div>
				{/if}
			</div>

			<div class="flex items-center gap-3">
				<!-- Diff Stats -->
				{#if fromRevision && toRevision}
					<div class="flex items-center gap-3 rounded-lg bg-gray-50 px-3 py-1.5 text-xs font-medium">
						<span class="flex items-center gap-1 text-green-600" title="Added fields">
							<span class="text-base">+</span>{diffStats().added}
						</span>
						<span class="flex items-center gap-1 text-red-600" title="Removed fields">
							<span class="text-base">-</span>{diffStats().removed}
						</span>
						<span class="flex items-center gap-1 text-amber-600" title="Modified fields">
							<span class="text-base">~</span>{diffStats().modified}
						</span>
					</div>
				{/if}

				<!-- Restore Button -->
				{#if toRevision}
					<button
						class="inline-flex items-center gap-2 rounded-lg bg-blue-600 px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
						onclick={handleRestore}
						disabled={isRestoring || toRevision.revision_number === sortedRevisions[0]?.revision_number}
						aria-busy={isRestoring}
					>
						{#if isRestoring}
							<svg class="h-4 w-4 animate-spin" fill="none" viewBox="0 0 24 24">
								<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
								<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
							</svg>
							Restoring...
						{:else}
							<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
							</svg>
							Restore v{toRevision.revision_number}
						{/if}
					</button>
				{/if}

				<!-- Close Button -->
				<button
					class="rounded-lg p-2 text-gray-400 transition-colors hover:bg-gray-100 hover:text-gray-600 focus:outline-none focus:ring-2 focus:ring-gray-300"
					onclick={onClose}
					aria-label="Close diff view"
				>
					<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
					</svg>
				</button>
			</div>
		</header>

		<!-- Main Content -->
		<div class="flex flex-1 overflow-hidden">
			<!-- Revision List Sidebar -->
			<aside
				class="flex w-72 shrink-0 flex-col border-r border-gray-200 bg-gray-50"
				aria-label="Revision list"
			>
				<!-- Sidebar Header -->
				<div class="border-b border-gray-200 p-3">
					<div class="relative">
						<svg class="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
						</svg>
						<input
							type="text"
							placeholder="Search revisions..."
							bind:value={searchQuery}
							class="w-full rounded-lg border border-gray-200 bg-white py-2 pl-9 pr-3 text-sm placeholder-gray-400 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
						/>
					</div>

					<div class="mt-2 flex gap-1">
						<button
							class="flex-1 rounded px-2 py-1 text-xs font-medium transition-colors {filterType === 'all' ? 'bg-blue-100 text-blue-700' : 'text-gray-600 hover:bg-gray-200'}"
							onclick={() => (filterType = 'all')}
						>
							All
						</button>
						<button
							class="flex-1 rounded px-2 py-1 text-xs font-medium transition-colors {filterType === 'manual' ? 'bg-blue-100 text-blue-700' : 'text-gray-600 hover:bg-gray-200'}"
							onclick={() => (filterType = 'manual')}
						>
							Manual
						</button>
						<button
							class="flex-1 rounded px-2 py-1 text-xs font-medium transition-colors {filterType === 'auto' ? 'bg-blue-100 text-blue-700' : 'text-gray-600 hover:bg-gray-200'}"
							onclick={() => (filterType = 'auto')}
						>
							Auto
						</button>
					</div>
				</div>

				<!-- Selection Mode Indicator -->
				<div class="border-b border-gray-200 bg-blue-50 px-3 py-2 text-xs">
					<span class="font-medium text-blue-700">
						{selectionMode === 'from' ? 'Select "From" revision' : 'Select "To" revision'}
					</span>
				</div>

				<!-- Revision List -->
				<div class="flex-1 overflow-y-auto p-2" role="listbox" aria-label="Available revisions">
					{#each filteredRevisions() as revision (revision.id)}
						{@const isFrom = fromRevision?.id === revision.id}
						{@const isTo = toRevision?.id === revision.id}
						{@const isCurrent = revision.revision_number === sortedRevisions[0]?.revision_number}
						{@const badge = getChangeTypeBadge(revision.change_summary)}

						<button
							class="mb-2 w-full rounded-lg border p-3 text-left transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-1
								{isFrom ? 'border-red-300 bg-red-50 ring-2 ring-red-200' : ''}
								{isTo ? 'border-green-300 bg-green-50 ring-2 ring-green-200' : ''}
								{!isFrom && !isTo ? 'border-gray-200 bg-white hover:border-gray-300 hover:shadow-sm' : ''}"
							onclick={() => selectRevision(revision)}
							role="option"
							aria-selected={isFrom || isTo}
						>
							<!-- Header Row -->
							<div class="flex items-start justify-between gap-2">
								<div class="flex items-center gap-2">
									<span class="text-sm font-semibold text-gray-900">v{revision.revision_number}</span>
									{#if isCurrent}
										<span class="rounded bg-emerald-100 px-1.5 py-0.5 text-[10px] font-semibold uppercase text-emerald-700">
											Current
										</span>
									{/if}
								</div>
								<span class="rounded px-1.5 py-0.5 text-[10px] font-medium {badge.class}">
									{badge.text}
								</span>
							</div>

							<!-- Timestamp -->
							<div class="mt-1 flex items-center gap-1 text-xs text-gray-500" title={formatFullDate(revision.created_at)}>
								<svg class="h-3 w-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
								</svg>
								{formatRelativeTime(revision.created_at)}
							</div>

							<!-- Author -->
							{#if revision.created_by}
								<div class="mt-2 flex items-center gap-2">
									<span
										class="flex h-5 w-5 items-center justify-center rounded-full bg-gradient-to-br from-blue-500 to-purple-600 text-[10px] font-bold text-white"
									>
										{getInitials(revision.created_by)}
									</span>
									<span class="truncate text-xs text-gray-600">{revision.created_by}</span>
								</div>
							{/if}

							<!-- Changed Fields -->
							{#if revision.changed_fields && revision.changed_fields.length > 0}
								<div class="mt-2 flex flex-wrap gap-1">
									{#each revision.changed_fields.slice(0, 3) as field}
										<span class="rounded bg-gray-100 px-1.5 py-0.5 text-[10px] text-gray-600">
											{field}
										</span>
									{/each}
									{#if revision.changed_fields.length > 3}
										<span class="rounded bg-gray-100 px-1.5 py-0.5 text-[10px] text-gray-500">
											+{revision.changed_fields.length - 3}
										</span>
									{/if}
								</div>
							{/if}

							<!-- Selection Indicator -->
							{#if isFrom || isTo}
								<div class="mt-2 flex items-center gap-1 text-[10px] font-medium uppercase tracking-wide {isFrom ? 'text-red-600' : 'text-green-600'}">
									{isFrom ? 'From (Old)' : 'To (New)'}
								</div>
							{/if}
						</button>
					{/each}

					{#if filteredRevisions().length === 0}
						<div class="py-8 text-center text-sm text-gray-500">
							No revisions found
						</div>
					{/if}
				</div>
			</aside>

			<!-- Diff View -->
			<main class="flex flex-1 flex-col overflow-hidden">
				{#if fromRevision && toRevision}
					<!-- Split Pane Headers -->
					<div class="grid shrink-0 grid-cols-2 border-b border-gray-200 bg-gray-50">
						<div class="border-r border-gray-200 px-4 py-2">
							<div class="flex items-center gap-2">
								<span class="rounded bg-red-100 px-2 py-0.5 text-xs font-semibold text-red-700">
									v{fromRevision.revision_number}
								</span>
								<span class="text-sm text-gray-600">
									{formatRelativeTime(fromRevision.created_at)}
								</span>
							</div>
							{#if fromRevision.created_by}
								<div class="mt-0.5 text-xs text-gray-500">by {fromRevision.created_by}</div>
							{/if}
						</div>
						<div class="px-4 py-2">
							<div class="flex items-center gap-2">
								<span class="rounded bg-green-100 px-2 py-0.5 text-xs font-semibold text-green-700">
									v{toRevision.revision_number}
								</span>
								<span class="text-sm text-gray-600">
									{formatRelativeTime(toRevision.created_at)}
								</span>
								{#if toRevision.revision_number === sortedRevisions[0]?.revision_number}
									<span class="rounded bg-emerald-100 px-1.5 py-0.5 text-[10px] font-semibold uppercase text-emerald-700">
										Current
									</span>
								{/if}
							</div>
							{#if toRevision.created_by}
								<div class="mt-0.5 text-xs text-gray-500">by {toRevision.created_by}</div>
							{/if}
						</div>
					</div>

					<!-- Split Pane Content -->
					<div class="grid flex-1 grid-cols-2 overflow-hidden">
						<!-- Left Pane (From/Old) -->
						<div
							class="overflow-y-auto border-r border-gray-200 bg-red-50/30 p-4"
							bind:this={leftPaneRef}
							onscroll={() => syncScroll('left')}
						>
							{#each fieldDiffs() as diff (diff.field)}
								<div class="mb-4 rounded-lg border border-gray-200 bg-white p-3 shadow-sm">
									<div class="mb-2 flex items-center gap-2">
										<span class="text-sm font-medium text-gray-700">{diff.label}</span>
										{#if diff.type === 'removed'}
											<span class="rounded bg-red-100 px-1.5 py-0.5 text-[10px] font-semibold uppercase text-red-700">
												Removed
											</span>
										{:else if diff.type === 'modified'}
											<span class="rounded bg-amber-100 px-1.5 py-0.5 text-[10px] font-semibold uppercase text-amber-700">
												Modified
											</span>
										{:else if diff.type === 'added'}
											<span class="rounded bg-gray-100 px-1.5 py-0.5 text-[10px] font-semibold uppercase text-gray-500">
												N/A
											</span>
										{/if}
									</div>

									{#if diff.type === 'added'}
										<div class="rounded bg-gray-100 px-3 py-2 text-sm italic text-gray-400">
											Field did not exist
										</div>
									{:else if diff.textDiff}
										<div class="rounded bg-gray-50 p-2 font-mono text-sm">
											{#each diff.textDiff as segment}
												{#if segment.type === 'delete'}
													<span class="rounded bg-red-200 text-red-900 line-through">{segment.text}</span>
												{:else if segment.type === 'equal'}
													<span class="text-gray-700">{segment.text}</span>
												{/if}
											{/each}
										</div>
									{:else}
										<div class="rounded bg-gray-50 p-2 text-sm text-gray-600 {diff.type === 'removed' ? 'bg-red-50' : ''}">
											{truncateValue(diff.fromValue, 500)}
										</div>
									{/if}
								</div>
							{/each}

							<!-- Content Blocks Diff -->
							{#if blockDiffs()}
								<div class="mt-6">
									<button
										class="mb-3 flex w-full items-center gap-2 text-sm font-medium text-gray-700"
										onclick={() => (showBlocksExpanded = !showBlocksExpanded)}
									>
										<svg
											class="h-4 w-4 transition-transform {showBlocksExpanded ? 'rotate-90' : ''}"
											fill="none"
											stroke="currentColor"
											viewBox="0 0 24 24"
										>
											<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
										</svg>
										Content Blocks ({fromRevision.data?.content_blocks?.length ?? 0} blocks)
									</button>

									{#if showBlocksExpanded}
										<div class="space-y-2">
											{#each blockDiffs() ?? [] as blockDiff}
												{#if blockDiff.type !== 'added' && blockDiff.fromBlock}
													<div class="rounded border p-2 {blockDiff.type === 'removed' ? 'border-red-200 bg-red-50' : blockDiff.type === 'modified' ? 'border-amber-200 bg-amber-50' : 'border-gray-200 bg-white'}">
														<div class="flex items-center gap-2 text-xs">
															<span class="rounded bg-gray-200 px-1.5 py-0.5 font-medium uppercase">{blockDiff.fromBlock.type}</span>
															{#if blockDiff.type === 'removed'}
																<span class="text-red-600">Removed</span>
															{:else if blockDiff.type === 'modified'}
																<span class="text-amber-600">Modified</span>
															{/if}
														</div>
													</div>
												{:else if blockDiff.type === 'added'}
													<div class="rounded border border-gray-200 bg-gray-100 p-2 text-xs italic text-gray-400">
														Block added in newer version
													</div>
												{/if}
											{/each}
										</div>
									{/if}
								</div>
							{/if}
						</div>

						<!-- Right Pane (To/New) -->
						<div
							class="overflow-y-auto bg-green-50/30 p-4"
							bind:this={rightPaneRef}
							onscroll={() => syncScroll('right')}
						>
							{#each fieldDiffs() as diff (diff.field)}
								<div class="mb-4 rounded-lg border border-gray-200 bg-white p-3 shadow-sm">
									<div class="mb-2 flex items-center gap-2">
										<span class="text-sm font-medium text-gray-700">{diff.label}</span>
										{#if diff.type === 'added'}
											<span class="rounded bg-green-100 px-1.5 py-0.5 text-[10px] font-semibold uppercase text-green-700">
												Added
											</span>
										{:else if diff.type === 'modified'}
											<span class="rounded bg-amber-100 px-1.5 py-0.5 text-[10px] font-semibold uppercase text-amber-700">
												Modified
											</span>
										{:else if diff.type === 'removed'}
											<span class="rounded bg-gray-100 px-1.5 py-0.5 text-[10px] font-semibold uppercase text-gray-500">
												N/A
											</span>
										{/if}
									</div>

									{#if diff.type === 'removed'}
										<div class="rounded bg-gray-100 px-3 py-2 text-sm italic text-gray-400">
											Field was removed
										</div>
									{:else if diff.textDiff}
										<div class="rounded bg-gray-50 p-2 font-mono text-sm">
											{#each diff.textDiff as segment}
												{#if segment.type === 'insert'}
													<span class="rounded bg-green-200 text-green-900">{segment.text}</span>
												{:else if segment.type === 'equal'}
													<span class="text-gray-700">{segment.text}</span>
												{/if}
											{/each}
										</div>
									{:else}
										<div class="rounded bg-gray-50 p-2 text-sm text-gray-600 {diff.type === 'added' ? 'bg-green-50' : ''}">
											{truncateValue(diff.toValue, 500)}
										</div>
									{/if}
								</div>
							{/each}

							<!-- Content Blocks Diff -->
							{#if blockDiffs()}
								<div class="mt-6">
									<button
										class="mb-3 flex w-full items-center gap-2 text-sm font-medium text-gray-700"
										onclick={() => (showBlocksExpanded = !showBlocksExpanded)}
									>
										<svg
											class="h-4 w-4 transition-transform {showBlocksExpanded ? 'rotate-90' : ''}"
											fill="none"
											stroke="currentColor"
											viewBox="0 0 24 24"
										>
											<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
										</svg>
										Content Blocks ({toRevision.data?.content_blocks?.length ?? 0} blocks)
									</button>

									{#if showBlocksExpanded}
										<div class="space-y-2">
											{#each blockDiffs() ?? [] as blockDiff}
												{#if blockDiff.type !== 'removed' && blockDiff.toBlock}
													<div class="rounded border p-2 {blockDiff.type === 'added' ? 'border-green-200 bg-green-50' : blockDiff.type === 'modified' ? 'border-amber-200 bg-amber-50' : 'border-gray-200 bg-white'}">
														<div class="flex items-center gap-2 text-xs">
															<span class="rounded bg-gray-200 px-1.5 py-0.5 font-medium uppercase">{blockDiff.toBlock.type}</span>
															{#if blockDiff.type === 'added'}
																<span class="text-green-600">Added</span>
															{:else if blockDiff.type === 'modified'}
																<span class="text-amber-600">Modified</span>
															{/if}
														</div>
													</div>
												{:else if blockDiff.type === 'removed'}
													<div class="rounded border border-gray-200 bg-gray-100 p-2 text-xs italic text-gray-400">
														Block removed from this version
													</div>
												{/if}
											{/each}
										</div>
									{/if}
								</div>
							{/if}
						</div>
					</div>
				{:else}
					<!-- Empty State -->
					<div class="flex flex-1 items-center justify-center">
						<div class="text-center">
							<svg class="mx-auto h-16 w-16 text-gray-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
							</svg>
							<h3 class="mt-4 text-lg font-medium text-gray-900">Select Revisions to Compare</h3>
							<p class="mt-1 text-sm text-gray-500">
								Click on revisions in the sidebar to select the versions you want to compare.
							</p>
						</div>
					</div>
				{/if}
			</main>
		</div>

		<!-- Footer -->
		<footer class="shrink-0 border-t border-gray-200 bg-gray-50 px-6 py-3">
			<div class="flex items-center justify-between">
				<div class="text-xs text-gray-500">
					{revisions.length} revision{revisions.length !== 1 ? 's' : ''} available
					{#if contentId}
						<span class="mx-1">|</span>
						Content ID: <code class="rounded bg-gray-200 px-1">{contentId.slice(0, 8)}...</code>
					{/if}
				</div>
				<div class="flex items-center gap-2">
					<span class="flex items-center gap-1 text-xs text-gray-500">
						<kbd class="rounded border border-gray-300 bg-white px-1.5 py-0.5 font-mono text-[10px]">Esc</kbd>
						to close
					</span>
				</div>
			</div>
		</footer>
	</div>
</div>

<style>
	/* Smooth animations */
	:global(.revision-diff-enter) {
		animation: fadeIn 0.2s ease-out;
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
			transform: scale(0.95);
		}
		to {
			opacity: 1;
			transform: scale(1);
		}
	}

	/* Scrollbar styling */
	::-webkit-scrollbar {
		width: 8px;
		height: 8px;
	}

	::-webkit-scrollbar-track {
		background: transparent;
	}

	::-webkit-scrollbar-thumb {
		background: #d1d5db;
		border-radius: 4px;
	}

	::-webkit-scrollbar-thumb:hover {
		background: #9ca3af;
	}

	/* Focus visible for keyboard navigation */
	button:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}
</style>

<script lang="ts">
	import {
		IconPlus,
		IconSearch,
		IconTrendingUp,
		IconTrendingDown,
		IconMinus,
		IconTrash,
		IconRefresh
	} from '#lib/icons/index.js';
	import ConfirmationModal from '#lib/components/admin/ConfirmationModal.svelte';
	import { logger } from '#lib/utils/logger.js';
	import { getKeywords, getKeywordStats, removeKeyword } from './keywords.remote';

	let searchQuery = $state('');
	let showDeleteModal = $state(false);
	let pendingDeleteId = $state<number | null>(null);

	// Reactive no-arg queries; `removeKeyword` single-flight-refreshes both.
	const keywordsQuery = getKeywords();
	const statsQuery = getKeywordStats();
	const keywords = $derived(keywordsQuery.current ?? []);
	const stats = $derived(statsQuery.current ?? null);
	const loading = $derived(keywordsQuery.loading);

	function deleteKeyword(id: number) {
		pendingDeleteId = id;
		showDeleteModal = true;
	}

	async function confirmDeleteKeyword() {
		if (!pendingDeleteId) return;
		showDeleteModal = false;
		const id = pendingDeleteId;
		pendingDeleteId = null;
		try {
			await removeKeyword(id); // single-flight refreshes list + stats
		} catch (error) {
			logger.error('Failed to delete keyword:', error);
		}
	}

	function getRankClass(rank: number) {
		if (rank <= 3) return 'top-3';
		if (rank <= 10) return 'top-10';
		if (rank <= 20) return 'top-20';
		return 'other';
	}

	function getTrendIcon(change: number) {
		if (change > 0) return IconTrendingUp;
		if (change < 0) return IconTrendingDown;
		return IconMinus;
	}

	function getTrendClass(change: number) {
		if (change > 0) return 'positive';
		if (change < 0) return 'negative';
		return 'neutral';
	}

	let filteredKeywords = $derived(
		keywords.filter((kw) => kw.keyword.toLowerCase().includes(searchQuery.toLowerCase()))
	);
</script>

<svelte:head>
	<title>Keyword Tracking | SEO</title>
</svelte:head>

<div class="keywords-page">
	<header class="page-header">
		<div>
			<h1>Keyword Tracking</h1>
			<p>Monitor keyword rankings and performance</p>
		</div>
		<button class="btn-primary">
			<IconPlus size={18} />
			Add Keyword
		</button>
	</header>

	{#if stats}
		<div class="stats-grid">
			<div class="stat-card">
				<div class="stat-value">{stats.total}</div>
				<div class="stat-label">Total Keywords</div>
			</div>
			<div class="stat-card success">
				<div class="stat-value">{stats.top_3}</div>
				<div class="stat-label">Top 3 Rankings</div>
			</div>
			<div class="stat-card primary">
				<div class="stat-value">{stats.top_10}</div>
				<div class="stat-label">Top 10 Rankings</div>
			</div>
			<div class="stat-card">
				<div class="stat-value">{stats.avg_position ? Math.round(stats.avg_position) : '—'}</div>
				<div class="stat-label">Avg. Position</div>
			</div>
		</div>
	{/if}

	<div class="controls-bar">
		<div class="search-box">
			<IconSearch size={20} />
			<label for="search-keywords" class="sr-only">Search keywords</label>
			<input autocomplete="off"
				type="text"
				id="search-keywords"
				name="search-keywords"
				bind:value={searchQuery}
				placeholder="Search keywords..."
			/>
		</div>

		<button
			class="btn-secondary"
			onclick={() => {
				keywordsQuery.refresh();
				statsQuery.refresh();
			}}
		>
			<IconRefresh size={18} />
			Refresh
		</button>
	</div>

	<div class="keywords-table">
		{#if loading}
			<div class="loading">Loading keywords...</div>
		{:else if filteredKeywords.length === 0}
			<div class="empty-state">
				<h3>No keywords tracked yet</h3>
				<p>Add keywords to start tracking their rankings</p>
				<button class="btn-primary">
					<IconPlus size={18} />
					Add Your First Keyword
				</button>
			</div>
		{:else}
			<table>
				<thead>
					<tr>
						<th>Keyword</th>
						<th>Current Rank</th>
						<th>Change</th>
						<th>Search Volume</th>
						<th>Competition</th>
						<th>Target URL</th>
						<th style="width: 100px">Actions</th>
					</tr>
				</thead>
				<tbody>
					{#each filteredKeywords as keyword (keyword.keyword)}
						<tr>
							<td>
								<div class="keyword-cell">{keyword.keyword}</div>
							</td>
							<td>
								{#if keyword.current_rank}
									<span class={['rank-badge', getRankClass(keyword.current_rank)]}>
										#{keyword.current_rank}
									</span>
								{:else}
									<span class="rank-badge unranked">Not ranked</span>
								{/if}
							</td>
							<td>
								{#if keyword.rank_change != null && keyword.rank_change !== 0}
									{@const TrendIcon = getTrendIcon(keyword.rank_change)}
									<div class={['trend', getTrendClass(keyword.rank_change)]}>
										<TrendIcon size={16} />
										{Math.abs(keyword.rank_change)}
									</div>
								{:else}
									<span class="neutral">—</span>
								{/if}
							</td>
							<td>{keyword.search_volume?.toLocaleString() || '—'}</td>
							<td>
								{#if keyword.competition != null}
									<div class="competition-bar">
										<div
											class="competition-fill"
											style:width={`${keyword.competition * 100}%`}
										></div>
										<span class="competition-value">{Math.round(keyword.competition * 100)}%</span>
									</div>
								{:else}
									—
								{/if}
							</td>
							<td class="url-cell">{keyword.target_url || '—'}</td>
							<td>
								<div class="actions">
									<button
										class="action-btn danger"
										onclick={() => deleteKeyword(keyword.id)}
										title="Delete"
									>
										<IconTrash size={18} />
									</button>
								</div>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		{/if}
	</div>

	{#if stats?.top_keywords && stats.top_keywords.length > 0}
		<div class="top-keywords-section">
			<h2>Top Performing Keywords</h2>
			<div class="top-keywords-grid">
				{#each stats.top_keywords as keyword (keyword.keyword)}
					<div class="top-keyword-card">
						<div class="keyword-rank">#{keyword.current_rank}</div>
						<div class="keyword-name">{keyword.keyword}</div>
						<div class="keyword-volume">{keyword.search_volume?.toLocaleString()} searches/mo</div>
					</div>
				{/each}
			</div>
		</div>
	{/if}

	{#if stats?.opportunity_keywords && stats.opportunity_keywords.length > 0}
		<div class="opportunities-section">
			<h2>Quick Win Opportunities</h2>
			<p class="section-desc">Keywords ranking 11-30 with high potential</p>
			<div class="opportunities-grid">
				{#each stats.opportunity_keywords as keyword (keyword.keyword)}
					<div class="opportunity-card">
						<div class="opp-header">
							<span class="rank-badge top-20">#{keyword.current_rank}</span>
							<span class="competition-badge"
								>{Math.round((keyword.competition ?? 0) * 100)}% comp.</span
							>
						</div>
						<div class="opp-keyword">{keyword.keyword}</div>
						<div class="opp-volume">{keyword.search_volume?.toLocaleString()} searches/mo</div>
					</div>
				{/each}
			</div>
		</div>
	{/if}
</div>

<ConfirmationModal
	isOpen={showDeleteModal}
	title="Delete Keyword"
	message="Delete this keyword?"
	confirmText="Delete"
	variant="danger"
	onConfirm={confirmDeleteKeyword}
	onCancel={() => {
		showDeleteModal = false;
		pendingDeleteId = null;
	}}
/>

<style>
	.keywords-page {
		padding: 2rem;
		max-width: 1400px;
		margin: 0 auto;
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 2rem;
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: var(--text-primary, #f0f6fc);
		margin-bottom: 0.5rem;
	}

	.page-header p {
		color: var(--text-secondary, #8b949e);
		font-size: 0.95rem;
	}

	.btn-primary,
	.btn-secondary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		border-radius: 6px;
		font-weight: 500;
		border: none;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary {
		background: var(--primary-500, #e6b800);
		color: var(--bg-base, #0d1117);
	}

	.btn-primary:hover {
		background: var(--primary-500, #e6b800);
	}

	.btn-secondary {
		background: white;
		color: var(--text-secondary, #8b949e);
		border: 1px solid var(--border-default, #30363d);
	}

	.btn-secondary:hover {
		background: var(--bg-elevated, #161b22);
	}

	.stats-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.stat-card {
		background: white;
		padding: 1.5rem;
		border-radius: 8px;
		border: 1px solid var(--border-default, #30363d);
		text-align: center;
	}

	.stat-card.success {
		background: var(--success-soft, rgba(46, 160, 67, 0.15));
		border-color: var(--border-default, #30363d);
	}

	.stat-card.primary {
		background: var(--info-soft, rgba(56, 139, 253, 0.15));
		border-color: var(--border-default, #30363d);
	}

	.stat-value {
		font-size: 2rem;
		font-weight: 700;
		color: var(--text-primary, #f0f6fc);
		margin-bottom: 0.5rem;
	}

	.stat-label {
		color: var(--text-secondary, #8b949e);
		font-size: 0.9rem;
	}

	.controls-bar {
		display: flex;
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.search-box {
		flex: 1;
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem 1rem;
		background: white;
		border: 1px solid var(--border-default, #30363d);
		border-radius: 6px;
	}

	.search-box input {
		flex: 1;
		border: none;
		outline: none;
		font-size: 0.95rem;
	}

	.keywords-table {
		background: white;
		border-radius: 8px;
		border: 1px solid var(--border-default, #30363d);
		overflow: hidden;
		margin-bottom: 3rem;
	}

	table {
		width: 100%;
		border-collapse: collapse;
	}

	th {
		text-align: left;
		padding: 1rem;
		background: var(--bg-elevated, #161b22);
		border-bottom: 1px solid var(--border-default, #30363d);
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
		font-size: 0.9rem;
	}

	td {
		padding: 1rem;
		border-bottom: 1px solid var(--border-default, #30363d);
		font-size: 0.95rem;
	}

	tbody tr:hover {
		background: var(--bg-elevated, #161b22);
	}

	.keyword-cell {
		font-weight: 500;
		color: var(--text-primary, #f0f6fc);
	}

	.rank-badge {
		display: inline-block;
		padding: 0.375rem 0.75rem;
		border-radius: 12px;
		font-size: 0.85rem;
		font-weight: 600;
	}

	.rank-badge.top-3 {
		background: var(--success-soft, rgba(46, 160, 67, 0.15));
		color: var(--success-emphasis, #3fb950);
	}

	.rank-badge.top-10 {
		background: var(--info-soft, rgba(56, 139, 253, 0.15));
		color: var(--info-emphasis, #58a6ff);
	}

	.rank-badge.top-20 {
		background: var(--warning-soft, rgba(187, 128, 9, 0.15));
		color: var(--warning-emphasis, #d29922);
	}

	.rank-badge.other {
		background: var(--bg-surface, #1c2128);
		color: var(--text-secondary, #8b949e);
	}

	.rank-badge.unranked {
		background: var(--error-soft, rgba(218, 54, 51, 0.15));
		color: var(--error-emphasis, #f85149);
	}

	.trend {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		font-weight: 600;
		font-size: 0.9rem;
	}

	.trend.positive {
		color: var(--success-emphasis, #3fb950);
	}

	.trend.negative {
		color: var(--error-emphasis, #f85149);
	}

	.trend.neutral {
		color: var(--text-tertiary, #6e7681);
	}

	.competition-bar {
		position: relative;
		width: 100px;
		height: 24px;
		background: var(--bg-surface, #1c2128);
		border-radius: 12px;
		overflow: hidden;
	}

	.competition-fill {
		position: absolute;
		top: 0;
		left: 0;
		height: 100%;
		background: var(--success-base, #2ea043);
		transition: width 0.3s;
	}

	.competition-value {
		position: relative;
		display: flex;
		align-items: center;
		justify-content: center;
		height: 100%;
		font-size: 0.75rem;
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
	}

	.url-cell {
		color: var(--text-secondary, #8b949e);
		font-size: 0.85rem;
		max-width: 200px;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.actions {
		display: flex;
		gap: 0.5rem;
	}

	.action-btn {
		padding: 0.5rem;
		background: none;
		border: 1px solid var(--border-default, #30363d);
		border-radius: 4px;
		color: var(--text-secondary, #8b949e);
		cursor: pointer;
		transition: all 0.2s;
	}

	.action-btn.danger {
		color: var(--error-emphasis, #f85149);
		border-color: var(--border-default, #30363d);
	}

	.action-btn.danger:hover {
		background: var(--error-soft, rgba(218, 54, 51, 0.15));
	}

	.empty-state {
		text-align: center;
		padding: 4rem 2rem;
	}

	.empty-state h3 {
		color: var(--text-primary, #f0f6fc);
		margin-bottom: 0.5rem;
	}

	.empty-state p {
		color: var(--text-secondary, #8b949e);
		margin-bottom: 1.5rem;
	}

	.loading {
		text-align: center;
		padding: 3rem;
		color: var(--text-tertiary, #6e7681);
	}

	.top-keywords-section,
	.opportunities-section {
		margin-bottom: 3rem;
	}

	.top-keywords-section h2,
	.opportunities-section h2 {
		font-size: 1.5rem;
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
		margin-bottom: 0.5rem;
	}

	.section-desc {
		color: var(--text-secondary, #8b949e);
		margin-bottom: 1.5rem;
	}

	.top-keywords-grid,
	.opportunities-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
		gap: 1rem;
	}

	.top-keyword-card {
		padding: 1.5rem;
		background: white;
		border: 1px solid var(--border-default, #30363d);
		border-radius: 8px;
		text-align: center;
	}

	.keyword-rank {
		font-size: 2rem;
		font-weight: 700;
		color: var(--success-emphasis, #3fb950);
		margin-bottom: 0.5rem;
	}

	.keyword-name {
		font-weight: 500;
		color: var(--text-primary, #f0f6fc);
		margin-bottom: 0.5rem;
	}

	.keyword-volume {
		font-size: 0.85rem;
		color: var(--text-secondary, #8b949e);
	}

	.opportunity-card {
		padding: 1.25rem;
		background: white;
		border: 1px solid var(--border-default, #30363d);
		border-radius: 8px;
	}

	.opp-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.75rem;
	}

	.competition-badge {
		font-size: 0.75rem;
		color: var(--text-secondary, #8b949e);
	}

	.opp-keyword {
		font-weight: 500;
		color: var(--text-primary, #f0f6fc);
		margin-bottom: 0.5rem;
	}

	.opp-volume {
		font-size: 0.85rem;
		color: var(--text-secondary, #8b949e);
	}

	.sr-only {
		position: absolute;
		width: 1px;
		height: 1px;
		padding: 0;
		margin: -1px;
		overflow: hidden;
		clip: rect(0, 0, 0, 0);
		white-space: nowrap;
		border-width: 0;
	}
</style>

<script lang="ts">
	import { onMount } from 'svelte';
	import {
		IconPlus,
		IconSearch,
		IconTrendingUp,
		IconTrendingDown,
		IconMinus,
		IconTrash,
		IconRefresh
	} from '$lib/icons';

	let keywords: any[] = $state([]);
	let stats: any = $state(null);
	let loading = $state(false);
	let searchQuery = $state('');

	onMount(() => {
		loadKeywords();
		loadStats();
	});

	async function loadKeywords() {
		loading = true;
		try {
			const response = await fetch('/api/seo/keywords');
			const data = await response.json();
			keywords = data.data || [];
		} catch (error) {
			console.error('Failed to load keywords:', error);
		} finally {
			loading = false;
		}
	}

	async function loadStats() {
		try {
			const response = await fetch('/api/seo/keywords/stats');
			stats = await response.json();
		} catch (error) {
			console.error('Failed to load stats:', error);
		}
	}

	async function deleteKeyword(id: number) {
		if (!confirm('Delete this keyword?')) return;

		try {
			await fetch(`/api/seo/keywords/${id}`, { method: 'DELETE' });
			loadKeywords();
			loadStats();
		} catch (error) {
			console.error('Failed to delete keyword:', error);
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
			<input
				type="text"
				id="search-keywords"
				bind:value={searchQuery}
				placeholder="Search keywords..."
			/>
		</div>

		<button class="btn-secondary" onclick={loadKeywords}>
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
					{#each filteredKeywords as keyword}
						<tr>
							<td>
								<div class="keyword-cell">{keyword.keyword}</div>
							</td>
							<td>
								{#if keyword.current_rank}
									<span class="rank-badge {getRankClass(keyword.current_rank)}">
										#{keyword.current_rank}
									</span>
								{:else}
									<span class="rank-badge unranked">Not ranked</span>
								{/if}
							</td>
							<td>
								{#if keyword.rank_change !== null && keyword.rank_change !== 0}
									{@const TrendIcon = getTrendIcon(keyword.rank_change)}
									<div class="trend {getTrendClass(keyword.rank_change)}">
										<TrendIcon size={16} />
										{Math.abs(keyword.rank_change)}
									</div>
								{:else}
									<span class="neutral">—</span>
								{/if}
							</td>
							<td>{keyword.search_volume?.toLocaleString() || '—'}</td>
							<td>
								{#if keyword.competition !== null}
									<div class="competition-bar">
										<div class="competition-fill" style="width: {keyword.competition * 100}%"></div>
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
				{#each stats.top_keywords as keyword}
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
				{#each stats.opportunity_keywords as keyword}
					<div class="opportunity-card">
						<div class="opp-header">
							<span class="rank-badge top-20">#{keyword.current_rank}</span>
							<span class="competition-badge">{Math.round(keyword.competition * 100)}% comp.</span>
						</div>
						<div class="opp-keyword">{keyword.keyword}</div>
						<div class="opp-volume">{keyword.search_volume?.toLocaleString()} searches/mo</div>
					</div>
				{/each}
			</div>
		</div>
	{/if}
</div>

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
		color: #1a1a1a;
		margin-bottom: 0.5rem;
	}

	.page-header p {
		color: #666;
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
		background: #3b82f6;
		color: white;
	}

	.btn-primary:hover {
		background: #2563eb;
	}

	.btn-secondary {
		background: white;
		color: #666;
		border: 1px solid #e5e5e5;
	}

	.btn-secondary:hover {
		background: #f8f9fa;
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
		border: 1px solid #e5e5e5;
		text-align: center;
	}

	.stat-card.success {
		background: #dcfce7;
		border-color: #86efac;
	}

	.stat-card.primary {
		background: #dbeafe;
		border-color: #93c5fd;
	}

	.stat-value {
		font-size: 2rem;
		font-weight: 700;
		color: #1a1a1a;
		margin-bottom: 0.5rem;
	}

	.stat-label {
		color: #666;
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
		border: 1px solid #e5e5e5;
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
		border: 1px solid #e5e5e5;
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
		background: #f8f9fa;
		border-bottom: 1px solid #e5e5e5;
		font-weight: 600;
		color: #1a1a1a;
		font-size: 0.9rem;
	}

	td {
		padding: 1rem;
		border-bottom: 1px solid #f0f0f0;
		font-size: 0.95rem;
	}

	tbody tr:hover {
		background: #f8f9fa;
	}

	.keyword-cell {
		font-weight: 500;
		color: #1a1a1a;
	}

	.rank-badge {
		display: inline-block;
		padding: 0.375rem 0.75rem;
		border-radius: 12px;
		font-size: 0.85rem;
		font-weight: 600;
	}

	.rank-badge.top-3 {
		background: #dcfce7;
		color: #16a34a;
	}

	.rank-badge.top-10 {
		background: #dbeafe;
		color: #3b82f6;
	}

	.rank-badge.top-20 {
		background: #fef3c7;
		color: #d97706;
	}

	.rank-badge.other {
		background: #f0f0f0;
		color: #666;
	}

	.rank-badge.unranked {
		background: #fee2e2;
		color: #dc2626;
	}

	.trend {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		font-weight: 600;
		font-size: 0.9rem;
	}

	.trend.positive {
		color: #16a34a;
	}

	.trend.negative {
		color: #dc2626;
	}

	.trend.neutral {
		color: #999;
	}

	.competition-bar {
		position: relative;
		width: 100px;
		height: 24px;
		background: #f0f0f0;
		border-radius: 12px;
		overflow: hidden;
	}

	.competition-fill {
		position: absolute;
		top: 0;
		left: 0;
		height: 100%;
		background: linear-gradient(90deg, #22c55e, #eab308, #ef4444);
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
		color: #1a1a1a;
	}

	.url-cell {
		color: #666;
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
		border: 1px solid #e5e5e5;
		border-radius: 4px;
		color: #666;
		cursor: pointer;
		transition: all 0.2s;
	}

	.action-btn.danger {
		color: #ef4444;
		border-color: #fee2e2;
	}

	.action-btn.danger:hover {
		background: #fee2e2;
	}

	.empty-state {
		text-align: center;
		padding: 4rem 2rem;
	}

	.empty-state h3 {
		color: #1a1a1a;
		margin-bottom: 0.5rem;
	}

	.empty-state p {
		color: #666;
		margin-bottom: 1.5rem;
	}

	.loading {
		text-align: center;
		padding: 3rem;
		color: #999;
	}

	.top-keywords-section,
	.opportunities-section {
		margin-bottom: 3rem;
	}

	.top-keywords-section h2,
	.opportunities-section h2 {
		font-size: 1.5rem;
		font-weight: 600;
		color: #1a1a1a;
		margin-bottom: 0.5rem;
	}

	.section-desc {
		color: #666;
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
		border: 1px solid #e5e5e5;
		border-radius: 8px;
		text-align: center;
	}

	.keyword-rank {
		font-size: 2rem;
		font-weight: 700;
		color: #16a34a;
		margin-bottom: 0.5rem;
	}

	.keyword-name {
		font-weight: 500;
		color: #1a1a1a;
		margin-bottom: 0.5rem;
	}

	.keyword-volume {
		font-size: 0.85rem;
		color: #666;
	}

	.opportunity-card {
		padding: 1.25rem;
		background: white;
		border: 1px solid #e5e5e5;
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
		color: #666;
	}

	.opp-keyword {
		font-weight: 500;
		color: #1a1a1a;
		margin-bottom: 0.5rem;
	}

	.opp-volume {
		font-size: 0.85rem;
		color: #666;
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

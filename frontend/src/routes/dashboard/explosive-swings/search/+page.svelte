<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * Explosive Swings - Search Page
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Full-text search interface for alerts, trades, and trade plans
	 * @version 1.0.0
	 * @standards Svelte 5 January 2026 | Apple Principal Engineer ICT 7+ | WCAG 2.1 AA
	 */
	import { browser } from '$app/environment';
	import { createSearchState } from './search.state.svelte';

	// Layout Components
	import TradingRoomHeader from '$lib/components/dashboard/TradingRoomHeader.svelte';
	import Pagination from '$lib/components/dashboard/pagination/Pagination.svelte';

	// Search Components
	import SearchInput from './components/SearchInput.svelte';
	import SearchFilters from './components/SearchFilters.svelte';
	import SearchResults from './components/SearchResults.svelte';
	import SearchSkeleton from './components/SearchSkeleton.svelte';
	import SearchEmptyState from './components/SearchEmptyState.svelte';

	// Initialize search state
	const search = createSearchState();

	$effect(() => {
		if (browser) {
			search.initialize();
		}
	});
</script>

<svelte:head>
	<title>Search | Explosive Swings | Revolution Trading Pros</title>
	<meta name="description" content="Search alerts, trades, and trade plans in Explosive Swings" />
</svelte:head>

<TradingRoomHeader
	roomName="Explosive Swings"
	startHereUrl="/dashboard/explosive-swings/start-here"
	showTradingRoomControls={false}
/>

<div class="search-page">
	<div class="search-container">
		<!-- Search Header -->
		<header class="search-header">
			<h1 class="search-title">Search</h1>
			<p class="search-subtitle">Find alerts, trades, and trade plans across your entire history</p>
		</header>

		<!-- Search Input -->
		<SearchInput
			query={search.query}
			suggestions={search.suggestions}
			showSuggestions={search.showSuggestions}
			searchHistory={search.searchHistory}
			isLoading={search.isLoading}
			onQueryChange={(q) => search.setQuery(q)}
			onSearch={() => search.performSearch()}
			onSuggestionSelect={(s) => search.selectSuggestion(s)}
			onHistorySelect={(h) => search.selectFromHistory(h)}
			onFetchSuggestions={(p) => search.fetchSuggestions(p)}
			onShowSuggestions={(v) => (search.showSuggestions = v)}
			onClearHistory={() => search.clearHistory()}
		/>

		<!-- Filters -->
		<SearchFilters
			selectedTypes={search.selectedTypes}
			dateRange={search.dateRange}
			tickerFilter={search.tickerFilter}
			onToggleType={(t) => search.toggleType(t)}
			onSetDateRange={(from, to) => search.setDateRange(from, to)}
			onSetTicker={(t) => search.updateFilters({ ticker: t })}
			onClearFilters={() => search.clearFilters()}
		/>

		<!-- Results Section -->
		<section class="results-section" aria-live="polite">
			{#if search.isLoading}
				<SearchSkeleton />
			{:else if search.error}
				<div class="error-state">
					<div class="error-icon">
						<svg
							width="48"
							height="48"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
						>
							<circle cx="12" cy="12" r="10" />
							<line x1="12" y1="8" x2="12" y2="12" />
							<line x1="12" y1="16" x2="12.01" y2="16" />
						</svg>
					</div>
					<h3>Search Error</h3>
					<p>{search.error}</p>
					<button class="retry-btn" onclick={() => search.performSearch()}> Try Again </button>
				</div>
			{:else if search.query && !search.hasResults}
				<SearchEmptyState query={search.query} onClearSearch={() => search.clearSearch()} />
			{:else if search.hasResults && search.results}
				<!-- Results Meta -->
				<div class="results-meta">
					<span class="results-count">
						{search.totalResults.toLocaleString()}
						{search.totalResults === 1 ? 'result' : 'results'}
					</span>
					<span class="results-time">({search.searchTime}ms)</span>
				</div>

				<!-- Results Display -->
				<SearchResults results={search.results} query={search.query} />

				<!-- Pagination -->
				{#if search.totalPages > 1}
					<div class="pagination-wrapper">
						<Pagination
							currentPage={search.currentPage}
							totalPages={search.totalPages}
							totalItems={search.totalResults}
							itemsPerPage={20}
							onPageChange={(p) => search.goToPage(p)}
							itemLabel="results"
						/>
					</div>
				{/if}
			{:else}
				<!-- Initial State -->
				<div class="initial-state">
					<div class="initial-icon">
						<svg
							width="64"
							height="64"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="1.5"
						>
							<circle cx="11" cy="11" r="8" />
							<path d="M21 21l-4.35-4.35" />
						</svg>
					</div>
					<h3>Search Your Trading History</h3>
					<p>
						Search across alerts, trades, and trade plans. Try searching for a ticker like "NVDA" or
						keywords like "breakout".
					</p>
					{#if search.searchHistory.length > 0}
						<div class="recent-searches">
							<span class="recent-label">Recent searches:</span>
							<div class="recent-list">
								{#each search.searchHistory.slice(0, 5) as historyItem}
									<button
										class="history-chip"
										onclick={() => search.selectFromHistory(historyItem)}
									>
										{historyItem}
									</button>
								{/each}
							</div>
						</div>
					{/if}
				</div>
			{/if}
		</section>
	</div>
</div>

<style>
	.search-page {
		background: var(--color-bg-page);
		min-height: 100vh;
		padding-bottom: 48px;
	}

	.search-container {
		max-width: 900px;
		margin: 0 auto;
		padding: 24px;
	}

	.search-header {
		text-align: center;
		margin-bottom: 32px;
	}

	.search-title {
		font-size: 32px;
		font-weight: 700;
		color: var(--color-text-primary);
		margin: 0 0 8px 0;
	}

	.search-subtitle {
		font-size: 16px;
		color: var(--color-text-secondary);
		margin: 0;
	}

	.results-section {
		margin-top: 32px;
	}

	.results-meta {
		display: flex;
		align-items: center;
		gap: 8px;
		margin-bottom: 16px;
		font-size: 14px;
	}

	.results-count {
		color: var(--color-text-primary);
		font-weight: 600;
	}

	.results-time {
		color: var(--color-text-tertiary);
	}

	.pagination-wrapper {
		margin-top: 24px;
		padding-top: 24px;
		border-top: 1px solid var(--color-border-default);
	}

	/* Error State */
	.error-state {
		text-align: center;
		padding: 64px 24px;
		background: var(--color-bg-card);
		border: 1px solid var(--color-border-default);
		border-radius: 12px;
	}

	.error-icon {
		color: var(--color-loss);
		margin-bottom: 16px;
	}

	.error-state h3 {
		font-size: 20px;
		font-weight: 600;
		color: var(--color-text-primary);
		margin: 0 0 8px 0;
	}

	.error-state p {
		font-size: 14px;
		color: var(--color-text-secondary);
		margin: 0 0 16px 0;
	}

	.retry-btn {
		padding: 10px 20px;
		background: var(--color-brand-primary);
		color: white;
		border: none;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		transition: background 0.15s;
	}

	.retry-btn:hover {
		background: var(--color-brand-primary-hover);
	}

	/* Initial State */
	.initial-state {
		text-align: center;
		padding: 64px 24px;
		background: var(--color-bg-card);
		border: 1px solid var(--color-border-default);
		border-radius: 12px;
	}

	.initial-icon {
		color: var(--color-text-tertiary);
		margin-bottom: 16px;
	}

	.initial-state h3 {
		font-size: 20px;
		font-weight: 600;
		color: var(--color-text-primary);
		margin: 0 0 8px 0;
	}

	.initial-state p {
		font-size: 14px;
		color: var(--color-text-secondary);
		margin: 0 0 24px 0;
		max-width: 400px;
		margin-left: auto;
		margin-right: auto;
	}

	.recent-searches {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 12px;
	}

	.recent-label {
		font-size: 12px;
		color: var(--color-text-tertiary);
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.recent-list {
		display: flex;
		flex-wrap: wrap;
		justify-content: center;
		gap: 8px;
	}

	.history-chip {
		padding: 6px 12px;
		background: var(--color-bg-subtle);
		color: var(--color-text-secondary);
		border: 1px solid var(--color-border-default);
		border-radius: 16px;
		font-size: 13px;
		cursor: pointer;
		transition: all 0.15s;
	}

	.history-chip:hover {
		background: var(--color-brand-primary);
		color: white;
		border-color: var(--color-brand-primary);
	}

	/* Mobile Responsive */
	@media (max-width: 768px) {
		.search-container {
			padding: 16px;
		}

		.search-title {
			font-size: 24px;
		}

		.search-subtitle {
			font-size: 14px;
		}

		.initial-state,
		.error-state {
			padding: 48px 16px;
		}
	}
</style>

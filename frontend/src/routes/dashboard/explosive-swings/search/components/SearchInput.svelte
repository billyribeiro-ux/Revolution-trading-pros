<script lang="ts">
	/**
	 * SearchInput - Debounced search input with suggestions and history
	 * @standards Svelte 5 January 2026 | Apple Principal Engineer ICT 7+ | WCAG 2.1 AA
	 */

	interface Props {
		query: string;
		suggestions: string[];
		showSuggestions: boolean;
		searchHistory: string[];
		isLoading: boolean;
		onQueryChange: (query: string) => void;
		onSearch: () => void;
		onSuggestionSelect: (suggestion: string) => void;
		onHistorySelect: (query: string) => void;
		onFetchSuggestions: (prefix: string) => void;
		onShowSuggestions: (show: boolean) => void;
		onClearHistory: () => void;
	}

	let {
		query,
		suggestions,
		showSuggestions,
		searchHistory,
		isLoading,
		onQueryChange,
		onSearch,
		onSuggestionSelect,
		onHistorySelect,
		onFetchSuggestions,
		onShowSuggestions,
		onClearHistory
	}: Props = $props();

	let inputRef: HTMLInputElement | null = $state(null);
	let showDropdown = $state(false);

	// Computed: should show dropdown
	const hasDropdownContent = $derived(
		(showSuggestions && suggestions.length > 0) ||
			(!query && searchHistory.length > 0 && showDropdown)
	);

	function handleInput(event: Event) {
		const target = event.target as HTMLInputElement;
		const value = target.value;
		onQueryChange(value);

		// Fetch suggestions for ticker-like input (uppercase, 1-5 chars)
		if (value.length >= 1 && value.length <= 5 && /^[A-Za-z]+$/.test(value)) {
			onFetchSuggestions(value.toUpperCase());
		} else {
			onShowSuggestions(false);
		}
	}

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter') {
			event.preventDefault();
			onShowSuggestions(false);
			showDropdown = false;
			onSearch();
		} else if (event.key === 'Escape') {
			onShowSuggestions(false);
			showDropdown = false;
		}
	}

	function handleFocus() {
		showDropdown = true;
		if (query.length >= 1 && query.length <= 5 && /^[A-Za-z]+$/.test(query)) {
			onFetchSuggestions(query.toUpperCase());
		}
	}

	function handleBlur() {
		// Delay to allow click on dropdown items
		setTimeout(() => {
			showDropdown = false;
			onShowSuggestions(false);
		}, 200);
	}

	function clearInput() {
		onQueryChange('');
		onShowSuggestions(false);
		inputRef?.focus();
	}
</script>

<div class="search-input-wrapper">
	<div class="search-input-container">
		<div class="search-icon">
			<svg
				width="20"
				height="20"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
			>
				<circle cx="11" cy="11" r="8" />
				<path d="M21 21l-4.35-4.35" />
			</svg>
		</div>

		<input
			bind:this={inputRef}
			type="text"
			class="search-input"
			placeholder="Search alerts, trades, trade plans..."
			value={query}
			oninput={handleInput}
			onkeydown={handleKeydown}
			onfocus={handleFocus}
			onblur={handleBlur}
			aria-label="Search"
			aria-autocomplete="list"
			aria-expanded={hasDropdownContent}
			aria-controls="search-dropdown"
		/>

		{#if isLoading}
			<div class="loading-spinner">
				<svg width="20" height="20" viewBox="0 0 24 24" class="spinner">
					<circle
						cx="12"
						cy="12"
						r="10"
						stroke="currentColor"
						stroke-width="3"
						fill="none"
						opacity="0.25"
					/>
					<path
						d="M12 2a10 10 0 0 1 10 10"
						stroke="currentColor"
						stroke-width="3"
						fill="none"
						stroke-linecap="round"
					/>
				</svg>
			</div>
		{:else if query}
			<button class="clear-btn" onclick={clearInput} aria-label="Clear search">
				<svg
					width="16"
					height="16"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
				>
					<line x1="18" y1="6" x2="6" y2="18" />
					<line x1="6" y1="6" x2="18" y2="18" />
				</svg>
			</button>
		{/if}

		<button class="search-btn" onclick={onSearch} aria-label="Submit search"> Search </button>
	</div>

	<!-- Dropdown -->
	{#if hasDropdownContent}
		<div id="search-dropdown" class="search-dropdown" role="listbox">
			{#if showSuggestions && suggestions.length > 0}
				<div class="dropdown-section">
					<span class="section-label">Tickers</span>
					{#each suggestions as suggestion}
						<button
							class="dropdown-item ticker-item"
							onclick={() => onSuggestionSelect(suggestion)}
							role="option"
						>
							<span class="ticker-badge">{suggestion}</span>
						</button>
					{/each}
				</div>
			{:else if !query && searchHistory.length > 0}
				<div class="dropdown-section">
					<div class="section-header">
						<span class="section-label">Recent Searches</span>
						<button class="clear-history-btn" onclick={onClearHistory}> Clear </button>
					</div>
					{#each searchHistory.slice(0, 5) as historyItem}
						<button
							class="dropdown-item history-item"
							onclick={() => onHistorySelect(historyItem)}
							role="option"
						>
							<svg
								width="14"
								height="14"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
							>
								<circle cx="12" cy="12" r="10" />
								<polyline points="12 6 12 12 16 14" />
							</svg>
							<span>{historyItem}</span>
						</button>
					{/each}
				</div>
			{/if}
		</div>
	{/if}
</div>

<style>
	.search-input-wrapper {
		position: relative;
		width: 100%;
	}

	.search-input-container {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 12px 16px;
		background: var(--color-bg-card);
		border: 2px solid var(--color-border-default);
		border-radius: 12px;
		transition:
			border-color 0.15s,
			box-shadow 0.15s;
	}

	.search-input-container:focus-within {
		border-color: var(--color-brand-primary);
		box-shadow: 0 0 0 3px rgba(var(--color-brand-primary-rgb), 0.15);
	}

	.search-icon {
		color: var(--color-text-tertiary);
		flex-shrink: 0;
	}

	.search-input {
		flex: 1;
		border: none;
		background: transparent;
		font-size: 16px;
		color: var(--color-text-primary);
		outline: none;
	}

	.search-input::placeholder {
		color: var(--color-text-tertiary);
	}

	.loading-spinner {
		color: var(--color-brand-primary);
	}

	.spinner {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	.clear-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 24px;
		height: 24px;
		padding: 0;
		border: none;
		background: var(--color-bg-subtle);
		border-radius: 50%;
		color: var(--color-text-tertiary);
		cursor: pointer;
		transition: all 0.15s;
	}

	.clear-btn:hover {
		background: var(--color-bg-muted);
		color: var(--color-text-primary);
	}

	.search-btn {
		padding: 8px 20px;
		background: var(--color-brand-primary);
		color: white;
		border: none;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		transition: background 0.15s;
	}

	.search-btn:hover {
		background: var(--color-brand-primary-hover);
	}

	/* Dropdown */
	.search-dropdown {
		position: absolute;
		top: calc(100% + 8px);
		left: 0;
		right: 0;
		background: var(--color-bg-card);
		border: 1px solid var(--color-border-default);
		border-radius: 12px;
		box-shadow: var(--shadow-lg);
		z-index: 50;
		overflow: hidden;
	}

	.dropdown-section {
		padding: 8px;
	}

	.section-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 4px 8px 8px;
	}

	.section-label {
		display: block;
		padding: 4px 8px 8px;
		font-size: 11px;
		font-weight: 600;
		color: var(--color-text-tertiary);
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.section-header .section-label {
		padding: 0;
	}

	.clear-history-btn {
		padding: 2px 8px;
		background: transparent;
		border: none;
		font-size: 12px;
		color: var(--color-text-tertiary);
		cursor: pointer;
		transition: color 0.15s;
	}

	.clear-history-btn:hover {
		color: var(--color-brand-primary);
	}

	.dropdown-item {
		display: flex;
		align-items: center;
		gap: 10px;
		width: 100%;
		padding: 10px 12px;
		background: transparent;
		border: none;
		border-radius: 8px;
		font-size: 14px;
		color: var(--color-text-primary);
		text-align: left;
		cursor: pointer;
		transition: background 0.1s;
	}

	.dropdown-item:hover {
		background: var(--color-bg-subtle);
	}

	.ticker-item {
		justify-content: flex-start;
	}

	.ticker-badge {
		display: inline-flex;
		padding: 4px 10px;
		background: var(--color-brand-primary);
		color: white;
		font-size: 12px;
		font-weight: 700;
		border-radius: 4px;
		letter-spacing: 0.5px;
	}

	.history-item svg {
		color: var(--color-text-tertiary);
		flex-shrink: 0;
	}

	/* Mobile */
	@media (max-width: 640px) {
		.search-input-container {
			padding: 10px 12px;
		}

		.search-btn {
			padding: 8px 14px;
		}
	}
</style>

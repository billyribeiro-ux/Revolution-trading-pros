/**
 * ═══════════════════════════════════════════════════════════════════════════════
 * Explosive Swings - Search State Module
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * @description Centralized state management for full-text search using Svelte 5 runes
 * @version 1.0.0
 * @standards Svelte 5 January 2026 Syntax | Apple Principal Engineer ICT 7+
 *
 * Features:
 * - Debounced search with configurable delay
 * - Content type filtering (alerts, trades, trade_plans)
 * - Date range filtering
 * - Ticker filtering
 * - Pagination
 * - Search history (local storage)
 * - Autocomplete suggestions
 */

// ═══════════════════════════════════════════════════════════════════════════════
// TYPE DEFINITIONS
// ═══════════════════════════════════════════════════════════════════════════════

export interface DateRange {
	from: string | null;
	to: string | null;
}

export interface SearchFilters {
	types: SearchableType[];
	dateRange: DateRange;
	ticker: string | null;
}

export type SearchableType = 'alerts' | 'trades' | 'trade_plans';

export interface AlertSearchResult {
	id: number;
	ticker: string;
	title: string | null;
	alert_type: string;
	message: string;
	published_at: string;
	relevance_score: number;
	highlight: string;
}

export interface TradeSearchResult {
	id: number;
	ticker: string;
	direction: string;
	status: string;
	entry_price: number;
	exit_price: number | null;
	entry_date: string;
	exit_date: string | null;
	pnl_percent: number | null;
	result: string | null;
	notes: string | null;
	relevance_score: number;
	highlight: string;
}

export interface TradePlanSearchResult {
	id: number;
	ticker: string;
	bias: string;
	entry: string | null;
	target1: string | null;
	stop: string | null;
	week_of: string;
	notes: string | null;
	is_active: boolean;
	relevance_score: number;
	highlight: string;
}

export interface SearchResults {
	alerts: AlertSearchResult[];
	trades: TradeSearchResult[];
	trade_plans: TradePlanSearchResult[];
	total_count: number;
	query: string;
	took_ms: number;
	pagination: {
		limit: number;
		offset: number;
		has_more: boolean;
	};
}

export interface SearchSuggestions {
	tickers: string[];
}

// ═══════════════════════════════════════════════════════════════════════════════
// CONSTANTS
// ═══════════════════════════════════════════════════════════════════════════════

const ROOM_SLUG = 'explosive-swings';
const DEBOUNCE_DELAY = 300;
const RESULTS_PER_PAGE = 20;
const SEARCH_HISTORY_KEY = 'explosive_swings_search_history';
const MAX_HISTORY_ITEMS = 10;

// ═══════════════════════════════════════════════════════════════════════════════
// STATE FACTORY
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Creates the reactive search state store.
 * Call this once in +page.svelte.
 */
export function createSearchState() {
	// ═══════════════════════════════════════════════════════════════════════════
	// CORE STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let query = $state('');
	let results = $state<SearchResults | null>(null);
	let isLoading = $state(false);
	let error = $state<string | null>(null);

	// ═══════════════════════════════════════════════════════════════════════════
	// FILTER STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let selectedTypes = $state<SearchableType[]>(['alerts', 'trades', 'trade_plans']);
	let dateRange = $state<DateRange>({ from: null, to: null });
	let tickerFilter = $state<string | null>(null);

	// ═══════════════════════════════════════════════════════════════════════════
	// PAGINATION STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let currentPage = $state(1);
	let totalPages = $derived(
		results ? Math.ceil(results.total_count / RESULTS_PER_PAGE) : 1
	);

	// ═══════════════════════════════════════════════════════════════════════════
	// UI STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let suggestions = $state<string[]>([]);
	let showSuggestions = $state(false);
	let searchHistory = $state<string[]>([]);
	let debounceTimer: ReturnType<typeof setTimeout> | null = null;

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE
	// ═══════════════════════════════════════════════════════════════════════════

	const hasResults = $derived(
		results !== null &&
			(results.alerts.length > 0 ||
				results.trades.length > 0 ||
				results.trade_plans.length > 0)
	);

	const totalResults = $derived(results?.total_count ?? 0);

	const searchTime = $derived(results?.took_ms ?? 0);

	const filters = $derived<SearchFilters>({
		types: selectedTypes,
		dateRange,
		ticker: tickerFilter
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// ACTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	/**
	 * Load search history from local storage
	 */
	function loadSearchHistory() {
		try {
			const stored = localStorage.getItem(SEARCH_HISTORY_KEY);
			if (stored) {
				searchHistory = JSON.parse(stored);
			}
		} catch {
			searchHistory = [];
		}
	}

	/**
	 * Save search to history
	 */
	function saveToHistory(searchQuery: string) {
		if (!searchQuery.trim()) return;

		// Remove if already exists and add to front
		const filtered = searchHistory.filter((q) => q !== searchQuery);
		const updated = [searchQuery, ...filtered].slice(0, MAX_HISTORY_ITEMS);
		searchHistory = updated;

		try {
			localStorage.setItem(SEARCH_HISTORY_KEY, JSON.stringify(updated));
		} catch {
			// Ignore storage errors
		}
	}

	/**
	 * Clear search history
	 */
	function clearHistory() {
		searchHistory = [];
		try {
			localStorage.removeItem(SEARCH_HISTORY_KEY);
		} catch {
			// Ignore storage errors
		}
	}

	/**
	 * Perform search with current query and filters
	 */
	async function performSearch(resetPagination = true) {
		const searchQuery = query.trim();

		if (!searchQuery) {
			results = null;
			return;
		}

		if (resetPagination) {
			currentPage = 1;
		}

		isLoading = true;
		error = null;

		try {
			const params = new URLSearchParams();
			params.set('q', searchQuery);

			if (selectedTypes.length > 0 && selectedTypes.length < 3) {
				params.set('types', selectedTypes.join(','));
			}

			if (dateRange.from) {
				params.set('from', dateRange.from);
			}

			if (dateRange.to) {
				params.set('to', dateRange.to);
			}

			if (tickerFilter) {
				params.set('ticker', tickerFilter);
			}

			params.set('limit', String(RESULTS_PER_PAGE));
			params.set('offset', String((currentPage - 1) * RESULTS_PER_PAGE));

			const response = await fetch(`/api/room-search/${ROOM_SLUG}?${params}`, {
				credentials: 'include'
			});

			if (!response.ok) {
				const errorData = await response.json().catch(() => ({}));
				throw new Error(errorData.error || `HTTP ${response.status}`);
			}

			const data = await response.json();

			if (!data.success) {
				throw new Error(data.error || 'Search failed');
			}

			results = data.data;
			saveToHistory(searchQuery);
		} catch (err) {
			error = err instanceof Error ? err.message : 'Search failed';
			results = null;
		} finally {
			isLoading = false;
		}
	}

	/**
	 * Debounced search handler
	 */
	function debouncedSearch() {
		if (debounceTimer) {
			clearTimeout(debounceTimer);
		}

		debounceTimer = setTimeout(() => {
			performSearch();
		}, DEBOUNCE_DELAY);
	}

	/**
	 * Fetch autocomplete suggestions
	 */
	async function fetchSuggestions(prefix: string) {
		if (prefix.length < 1) {
			suggestions = [];
			showSuggestions = false;
			return;
		}

		try {
			const response = await fetch(
				`/api/room-search/${ROOM_SLUG}/suggestions?q=${encodeURIComponent(prefix)}&limit=8`,
				{ credentials: 'include' }
			);

			if (response.ok) {
				const data = await response.json();
				if (data.success) {
					suggestions = data.data.tickers;
					showSuggestions = suggestions.length > 0;
				}
			}
		} catch {
			suggestions = [];
		}
	}

	/**
	 * Update filters and re-search
	 */
	function updateFilters(newFilters: Partial<SearchFilters>) {
		if (newFilters.types !== undefined) {
			selectedTypes = newFilters.types;
		}
		if (newFilters.dateRange !== undefined) {
			dateRange = newFilters.dateRange;
		}
		if (newFilters.ticker !== undefined) {
			tickerFilter = newFilters.ticker;
		}

		// Re-search if there's a query
		if (query.trim()) {
			performSearch();
		}
	}

	/**
	 * Toggle a content type filter
	 */
	function toggleType(type: SearchableType) {
		if (selectedTypes.includes(type)) {
			// Don't allow deselecting all types
			if (selectedTypes.length > 1) {
				selectedTypes = selectedTypes.filter((t) => t !== type);
			}
		} else {
			selectedTypes = [...selectedTypes, type];
		}

		if (query.trim()) {
			performSearch();
		}
	}

	/**
	 * Set date range filter
	 */
	function setDateRange(from: string | null, to: string | null) {
		dateRange = { from, to };
		if (query.trim()) {
			performSearch();
		}
	}

	/**
	 * Clear all filters
	 */
	function clearFilters() {
		selectedTypes = ['alerts', 'trades', 'trade_plans'];
		dateRange = { from: null, to: null };
		tickerFilter = null;

		if (query.trim()) {
			performSearch();
		}
	}

	/**
	 * Set query and trigger search
	 */
	function setQuery(newQuery: string) {
		query = newQuery;
		debouncedSearch();
	}

	/**
	 * Select a suggestion
	 */
	function selectSuggestion(suggestion: string) {
		query = suggestion;
		showSuggestions = false;
		performSearch();
	}

	/**
	 * Select from history
	 */
	function selectFromHistory(historyQuery: string) {
		query = historyQuery;
		performSearch();
	}

	/**
	 * Go to a specific page
	 */
	async function goToPage(page: number) {
		if (page < 1 || page > totalPages || page === currentPage) return;
		currentPage = page;
		await performSearch(false);
	}

	/**
	 * Clear search and results
	 */
	function clearSearch() {
		query = '';
		results = null;
		error = null;
		currentPage = 1;
		suggestions = [];
		showSuggestions = false;
	}

	/**
	 * Initialize the search state
	 */
	function initialize() {
		loadSearchHistory();
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// RETURN PUBLIC API
	// ═══════════════════════════════════════════════════════════════════════════

	return {
		// Core state
		get query() {
			return query;
		},
		set query(v: string) {
			query = v;
		},
		get results() {
			return results;
		},
		get isLoading() {
			return isLoading;
		},
		get error() {
			return error;
		},

		// Filter state
		get selectedTypes() {
			return selectedTypes;
		},
		get dateRange() {
			return dateRange;
		},
		get tickerFilter() {
			return tickerFilter;
		},
		get filters() {
			return filters;
		},

		// Pagination
		get currentPage() {
			return currentPage;
		},
		get totalPages() {
			return totalPages;
		},

		// UI state
		get suggestions() {
			return suggestions;
		},
		get showSuggestions() {
			return showSuggestions;
		},
		set showSuggestions(v: boolean) {
			showSuggestions = v;
		},
		get searchHistory() {
			return searchHistory;
		},

		// Derived state
		get hasResults() {
			return hasResults;
		},
		get totalResults() {
			return totalResults;
		},
		get searchTime() {
			return searchTime;
		},

		// Actions
		initialize,
		performSearch,
		debouncedSearch,
		fetchSuggestions,
		updateFilters,
		toggleType,
		setDateRange,
		clearFilters,
		setQuery,
		selectSuggestion,
		selectFromHistory,
		goToPage,
		clearSearch,
		clearHistory,

		// Constants
		ROOM_SLUG
	};
}

export type SearchState = ReturnType<typeof createSearchState>;

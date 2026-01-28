/**
 * Svelte 5 Reactive Data Fetching Hooks - TypeScript 5.9+ Implementation
 * =============================================================================
 *
 * Apple Principal Engineer ICT Level 7 Grade Quality
 * Built for the next 10 years - January 2026
 *
 * ARCHITECTURE PRINCIPLES:
 * 1. Runes-First Design - Native Svelte 5 reactivity with $state and $derived
 * 2. Type-Safe by Default - Full inference with const type parameters
 * 3. Composable - Small, focused hooks that combine elegantly
 * 4. Resource Management - Automatic cleanup on component destroy
 * 5. Suspense-Ready - Pending states for concurrent rendering
 *
 * HOOKS PROVIDED:
 * - createQuery: Declarative data fetching with caching
 * - createMutation: State machine for mutations
 * - createInfiniteQuery: Paginated/infinite scroll data
 * - createPrefetch: Preload data for navigation
 *
 * FEATURES:
 * - Automatic refetch on focus/reconnect
 * - Polling with configurable intervals
 * - Optimistic updates for mutations
 * - Error retry with backoff
 * - Stale-while-revalidate caching
 *
 * @version 2.0.0 - TypeScript 5.9+ Svelte 5 Rewrite
 * @license MIT
 */

import { onDestroy, untrack } from 'svelte';
import { browser } from '$app/environment';
import { getApiClient, type RequestOptions, type ApiResponse } from './client';
import { type ApiError, isApiError, isRetryableError } from './errors';
import { getCache, type CacheConfig, type CachedResponse } from './cache';

// =============================================================================
// TYPE DEFINITIONS
// =============================================================================

/** Query state lifecycle */
export type QueryStatus = 'idle' | 'pending' | 'success' | 'error';

/** Mutation state lifecycle */
export type MutationStatus = 'idle' | 'pending' | 'success' | 'error';

/** Query configuration options */
export interface QueryOptions<T = unknown> {
	/** Initial data (useful for SSR hydration) */
	initialData?: T;
	/** Cache configuration */
	cache?: Partial<CacheConfig>;
	/** Whether to fetch immediately */
	enabled?: boolean;
	/** Refetch interval in milliseconds (0 = disabled) */
	refetchInterval?: number;
	/** Refetch when window regains focus */
	refetchOnFocus?: boolean;
	/** Refetch when network reconnects */
	refetchOnReconnect?: boolean;
	/** Number of retries on error */
	retry?: number;
	/** Delay between retries in ms */
	retryDelay?: number;
	/** Stale time in ms (data is fresh for this duration) */
	staleTime?: number;
	/** Cache time in ms (keep in cache after unmount) */
	cacheTime?: number;
	/** Transform response data */
	select?: (data: T) => T;
	/** Called on successful fetch */
	onSuccess?: (data: T) => void;
	/** Called on fetch error */
	onError?: (error: ApiError) => void;
	/** Called on fetch settle (success or error) */
	onSettled?: (data: T | null, error: ApiError | null) => void;
}

/** Mutation configuration options */
export interface MutationOptions<TData = unknown, TVariables = unknown> {
	/** Called before mutation executes */
	onMutate?: (variables: TVariables) => Promise<unknown> | unknown;
	/** Called on successful mutation */
	onSuccess?: (data: TData, variables: TVariables, context: unknown) => void;
	/** Called on mutation error */
	onError?: (error: ApiError, variables: TVariables, context: unknown) => void;
	/** Called on mutation settle (success or error) */
	onSettled?: (
		data: TData | null,
		error: ApiError | null,
		variables: TVariables,
		context: unknown
	) => void;
	/** Number of retries on error */
	retry?: number;
	/** Retry delay in ms */
	retryDelay?: number;
}

/** Query result interface - reactive state */
export interface QueryResult<T> {
	/** Current data (null until first successful fetch) */
	readonly data: T | null;
	/** Loading state */
	readonly isLoading: boolean;
	/** Error state */
	readonly isError: boolean;
	/** Success state */
	readonly isSuccess: boolean;
	/** Current status */
	readonly status: QueryStatus;
	/** Error object if any */
	readonly error: ApiError | null;
	/** Whether data is stale */
	readonly isStale: boolean;
	/** Whether currently fetching (including background refetch) */
	readonly isFetching: boolean;
	/** Data fetch timestamp */
	readonly dataUpdatedAt: number | null;
	/** Error timestamp */
	readonly errorUpdatedAt: number | null;
	/** Manually refetch data */
	refetch: () => Promise<void>;
	/** Invalidate cache and refetch */
	invalidate: () => Promise<void>;
}

/** Mutation result interface - reactive state */
export interface MutationResult<TData, TVariables> {
	/** Mutation result data */
	readonly data: TData | null;
	/** Loading state */
	readonly isLoading: boolean;
	/** Error state */
	readonly isError: boolean;
	/** Success state */
	readonly isSuccess: boolean;
	/** Idle state */
	readonly isIdle: boolean;
	/** Current status */
	readonly status: MutationStatus;
	/** Error object if any */
	readonly error: ApiError | null;
	/** Variables used in last mutation */
	readonly variables: TVariables | null;
	/** Execute the mutation */
	mutate: (variables: TVariables) => void;
	/** Execute the mutation and await result */
	mutateAsync: (variables: TVariables) => Promise<TData>;
	/** Reset mutation state */
	reset: () => void;
}

/** Infinite query page result */
export interface InfiniteQueryResult<T> extends Omit<QueryResult<T[]>, 'data'> {
	/** All fetched pages */
	readonly data: T[][] | null;
	/** Flattened data from all pages */
	readonly flatData: T[] | null;
	/** Whether there's a next page */
	readonly hasNextPage: boolean;
	/** Whether there's a previous page */
	readonly hasPreviousPage: boolean;
	/** Whether fetching next page */
	readonly isFetchingNextPage: boolean;
	/** Whether fetching previous page */
	readonly isFetchingPreviousPage: boolean;
	/** Fetch next page */
	fetchNextPage: () => Promise<void>;
	/** Fetch previous page */
	fetchPreviousPage: () => Promise<void>;
}

// =============================================================================
// INTERNAL STATE CLASS - Svelte 5 Runes
// =============================================================================

/**
 * Internal reactive state manager using Svelte 5 runes
 */
class QueryState<T> {
	// Reactive state using $state rune
	data = $state<T | null>(null);
	error = $state<ApiError | null>(null);
	status = $state<QueryStatus>('idle');
	isFetching = $state(false);
	dataUpdatedAt = $state<number | null>(null);
	errorUpdatedAt = $state<number | null>(null);

	// Configuration - stored as reactive state for proper initialization
	private _staleTime = $state(0);

	// Derived state using $derived rune
	readonly isLoading = $derived(this.status === 'pending' && this.data === null);
	readonly isError = $derived(this.status === 'error');
	readonly isSuccess = $derived(this.status === 'success');
	readonly isStale = $derived(
		this.dataUpdatedAt !== null &&
		Date.now() - this.dataUpdatedAt > this._staleTime
	);

	constructor(
		initialData?: T,
		staleTime?: number
	) {
		this._staleTime = staleTime ?? 0;
		if (initialData !== undefined) {
			this.data = initialData;
			this.status = 'success';
			this.dataUpdatedAt = Date.now();
		}
	}

	setData(data: T): void {
		this.data = data;
		this.error = null;
		this.status = 'success';
		this.dataUpdatedAt = Date.now();
	}

	setError(error: ApiError): void {
		this.error = error;
		this.status = 'error';
		this.errorUpdatedAt = Date.now();
	}

	setPending(): void {
		if (this.data === null) {
			this.status = 'pending';
		}
		this.isFetching = true;
	}

	setIdle(): void {
		this.isFetching = false;
	}

	reset(): void {
		this.data = null;
		this.error = null;
		this.status = 'idle';
		this.isFetching = false;
		this.dataUpdatedAt = null;
		this.errorUpdatedAt = null;
	}
}

// =============================================================================
// CREATE QUERY - Main data fetching hook
// =============================================================================

/**
 * Create a reactive query with automatic caching and refetching
 *
 * @example
 * ```svelte
 * <script lang="ts">
 *   const alerts = createQuery(
 *     () => api.get<Alert[]>('/alerts/explosive-swings'),
 *     { refetchInterval: 30000 }
 *   );
 * </script>
 *
 * {#if alerts.isLoading}
 *   <Loading />
 * {:else if alerts.isError}
 *   <Error message={alerts.error?.userMessage} />
 * {:else}
 *   {#each alerts.data ?? [] as alert}
 *     <AlertCard {alert} />
 *   {/each}
 * {/if}
 * ```
 */
export function createQuery<T>(
	fetcher: () => Promise<T>,
	options: QueryOptions<T> = {}
): QueryResult<T> {
	const {
		initialData,
		cache: cacheConfig,
		enabled = true,
		refetchInterval = 0,
		refetchOnFocus = true,
		refetchOnReconnect = true,
		retry = 3,
		retryDelay = 1000,
		staleTime = 0,
		select,
		onSuccess,
		onError,
		onSettled,
	} = options;

	// Initialize reactive state
	const state = new QueryState<T>(initialData, staleTime);

	// Cache instance
	const cache = browser ? getCache() : null;

	// Generate cache key from fetcher
	const cacheKey = `query:${fetcher.toString().slice(0, 100)}`;

	// Cleanup refs
	let intervalId: ReturnType<typeof setInterval> | null = null;
	let abortController: AbortController | null = null;
	let isDestroyed = false;

	/**
	 * Execute the fetch operation
	 */
	async function executeFetch(forceRefresh = false): Promise<void> {
		if (isDestroyed || !browser) return;

		// Check cache first (unless forcing refresh)
		if (!forceRefresh && cache) {
			const cached = cache.get<T>(cacheKey);
			if (cached && !cached.isStale) {
				const data = select ? select(cached.data) : cached.data;
				state.setData(data);
				onSuccess?.(data);
				onSettled?.(data, null);
				return;
			}
		}

		// Cancel previous request
		abortController?.abort();
		abortController = new AbortController();

		state.setPending();

		let lastError: ApiError | null = null;

		for (let attempt = 0; attempt <= retry; attempt++) {
			try {
				const result = await fetcher();
				const data = select ? select(result) : result;

				// Cache the result
				if (cache) {
					cache.set(cacheKey, data, cacheConfig);
				}

				state.setData(data);
				onSuccess?.(data);
				onSettled?.(data, null);
				return;
			} catch (error) {
				lastError = isApiError(error)
					? error
					: new (await import('./errors')).UnknownError(
							error instanceof Error ? error.message : 'Unknown error'
						);

				// Don't retry non-retryable errors
				if (!isRetryableError(lastError) || attempt === retry) {
					break;
				}

				// Wait before retry
				await new Promise((resolve) => setTimeout(resolve, retryDelay * (attempt + 1)));
			}
		}

		// All retries failed
		if (lastError) {
			state.setError(lastError);
			onError?.(lastError);
			onSettled?.(null, lastError);
		}

		state.setIdle();
	}

	/**
	 * Refetch data (uses cache if fresh, fetches if stale)
	 */
	async function refetch(): Promise<void> {
		await executeFetch(false);
	}

	/**
	 * Invalidate cache and force refetch
	 */
	async function invalidate(): Promise<void> {
		cache?.invalidate(cacheKey);
		await executeFetch(true);
	}

	// Initial fetch
	if (browser && enabled) {
		// Use untrack to avoid creating reactive dependencies
		untrack(() => {
			executeFetch();
		});
	}

	// Set up refetch interval
	if (browser && refetchInterval > 0) {
		intervalId = setInterval(() => {
			if (state.isStale || !state.dataUpdatedAt) {
				executeFetch();
			}
		}, refetchInterval);
	}

	// Set up focus listener
	if (browser && refetchOnFocus) {
		const handleFocus = () => {
			if (state.isStale) {
				executeFetch();
			}
		};
		window.addEventListener('focus', handleFocus);

		onDestroy(() => {
			window.removeEventListener('focus', handleFocus);
		});
	}

	// Set up reconnect listener
	if (browser && refetchOnReconnect) {
		const handleOnline = () => {
			executeFetch();
		};
		window.addEventListener('online', handleOnline);

		onDestroy(() => {
			window.removeEventListener('online', handleOnline);
		});
	}

	// Cleanup on destroy
	onDestroy(() => {
		isDestroyed = true;
		abortController?.abort();
		if (intervalId) {
			clearInterval(intervalId);
		}
	});

	// Return reactive result object
	return {
		get data() { return state.data; },
		get isLoading() { return state.isLoading; },
		get isError() { return state.isError; },
		get isSuccess() { return state.isSuccess; },
		get status() { return state.status; },
		get error() { return state.error; },
		get isStale() { return state.isStale; },
		get isFetching() { return state.isFetching; },
		get dataUpdatedAt() { return state.dataUpdatedAt; },
		get errorUpdatedAt() { return state.errorUpdatedAt; },
		refetch,
		invalidate,
	};
}

// =============================================================================
// CREATE MUTATION - State machine for mutations
// =============================================================================

/**
 * Create a mutation handler with optimistic updates support
 *
 * @example
 * ```svelte
 * <script lang="ts">
 *   const closeTrade = createMutation(
 *     (tradeId: number) => api.post(`/trades/${tradeId}/close`),
 *     {
 *       onSuccess: () => tradesQuery.invalidate(),
 *       onError: (error) => toast.error(error.userMessage)
 *     }
 *   );
 * </script>
 *
 * <button
 *   onclick={() => closeTrade.mutate(trade.id)}
 *   disabled={closeTrade.isLoading}
 * >
 *   {closeTrade.isLoading ? 'Closing...' : 'Close Trade'}
 * </button>
 * ```
 */
export function createMutation<TData, TVariables = void>(
	mutationFn: (variables: TVariables) => Promise<TData>,
	options: MutationOptions<TData, TVariables> = {}
): MutationResult<TData, TVariables> {
	const {
		onMutate,
		onSuccess,
		onError,
		onSettled,
		retry = 0,
		retryDelay = 1000,
	} = options;

	// Reactive state
	let data = $state<TData | null>(null);
	let error = $state<ApiError | null>(null);
	let status = $state<MutationStatus>('idle');
	let variables = $state<TVariables | null>(null);

	// Derived state
	const isLoading = $derived(status === 'pending');
	const isError = $derived(status === 'error');
	const isSuccess = $derived(status === 'success');
	const isIdle = $derived(status === 'idle');

	/**
	 * Execute the mutation
	 */
	async function mutateAsync(vars: TVariables): Promise<TData> {
		status = 'pending';
		variables = vars;
		error = null;

		// Call onMutate for optimistic updates
		let context: unknown;
		try {
			context = await onMutate?.(vars);
		} catch (e) {
			// onMutate failed, abort mutation
			const apiError = isApiError(e)
				? e
				: new (await import('./errors')).UnknownError(
						e instanceof Error ? e.message : 'Mutation setup failed'
					);
			error = apiError;
			status = 'error';
			throw apiError;
		}

		let lastError: ApiError | null = null;

		for (let attempt = 0; attempt <= retry; attempt++) {
			try {
				const result = await mutationFn(vars);

				data = result;
				status = 'success';

				onSuccess?.(result, vars, context);
				onSettled?.(result, null, vars, context);

				return result;
			} catch (e) {
				lastError = isApiError(e)
					? e
					: new (await import('./errors')).UnknownError(
							e instanceof Error ? e.message : 'Mutation failed'
						);

				// Don't retry non-retryable errors
				if (!isRetryableError(lastError) || attempt === retry) {
					break;
				}

				// Wait before retry
				await new Promise((resolve) => setTimeout(resolve, retryDelay * (attempt + 1)));
			}
		}

		// All retries failed
		error = lastError;
		status = 'error';

		if (lastError) {
			onError?.(lastError, vars, context);
			onSettled?.(null, lastError, vars, context);
		}

		throw lastError;
	}

	/**
	 * Execute mutation (fire and forget)
	 */
	function mutate(vars: TVariables): void {
		mutateAsync(vars).catch(() => {
			// Error already handled in mutateAsync
		});
	}

	/**
	 * Reset mutation state
	 */
	function reset(): void {
		data = null;
		error = null;
		status = 'idle';
		variables = null;
	}

	return {
		get data() { return data; },
		get isLoading() { return isLoading; },
		get isError() { return isError; },
		get isSuccess() { return isSuccess; },
		get isIdle() { return isIdle; },
		get status() { return status; },
		get error() { return error; },
		get variables() { return variables; },
		mutate,
		mutateAsync,
		reset,
	};
}

// =============================================================================
// CREATE INFINITE QUERY - Paginated/infinite scroll data
// =============================================================================

/** Infinite query options */
export interface InfiniteQueryOptions<T, TPageParam = number> extends QueryOptions<T[]> {
	/** Get page parameter for next page */
	getNextPageParam: (lastPage: T[], allPages: T[][], lastPageParam: TPageParam) => TPageParam | null;
	/** Get page parameter for previous page */
	getPreviousPageParam?: (firstPage: T[], allPages: T[][], firstPageParam: TPageParam) => TPageParam | null;
	/** Initial page parameter */
	initialPageParam: TPageParam;
}

/**
 * Create an infinite/paginated query
 *
 * @example
 * ```svelte
 * <script lang="ts">
 *   const alerts = createInfiniteQuery(
 *     (pageParam) => api.get<Alert[]>(`/alerts?page=${pageParam}`),
 *     {
 *       initialPageParam: 1,
 *       getNextPageParam: (lastPage, _, pageParam) =>
 *         lastPage.length === 20 ? pageParam + 1 : null
 *     }
 *   );
 * </script>
 *
 * {#each alerts.flatData ?? [] as alert}
 *   <AlertCard {alert} />
 * {/each}
 *
 * {#if alerts.hasNextPage}
 *   <button onclick={() => alerts.fetchNextPage()}>
 *     {alerts.isFetchingNextPage ? 'Loading...' : 'Load More'}
 *   </button>
 * {/if}
 * ```
 */
export function createInfiniteQuery<T, TPageParam = number>(
	fetcher: (pageParam: TPageParam) => Promise<T[]>,
	options: InfiniteQueryOptions<T, TPageParam>
): InfiniteQueryResult<T> {
	const {
		initialData,
		enabled = true,
		refetchInterval = 0,
		retry = 3,
		retryDelay = 1000,
		initialPageParam,
		getNextPageParam,
		getPreviousPageParam,
		onSuccess,
		onError,
		onSettled,
	} = options;

	// Reactive state
	let pages = $state<T[][] | null>(initialData ? [initialData] : null);
	let pageParams = $state<TPageParam[]>([initialPageParam]);
	let error = $state<ApiError | null>(null);
	let status = $state<QueryStatus>(initialData ? 'success' : 'idle');
	let isFetching = $state(false);
	let isFetchingNextPage = $state(false);
	let isFetchingPreviousPage = $state(false);
	let dataUpdatedAt = $state<number | null>(initialData ? Date.now() : null);
	let errorUpdatedAt = $state<number | null>(null);

	// Derived state
	const isLoading = $derived(status === 'pending' && pages === null);
	const isError = $derived(status === 'error');
	const isSuccess = $derived(status === 'success');
	const isStale = $derived(dataUpdatedAt !== null && Date.now() - dataUpdatedAt > 0);

	const flatData = $derived(pages?.flat() ?? null);

	const hasNextPage = $derived(() => {
		if (!pages || pages.length === 0) return false;
		const lastPage = pages[pages.length - 1];
		const lastPageParam = pageParams[pageParams.length - 1];
		if (!lastPage || lastPageParam === undefined) return false;
		return getNextPageParam(lastPage, pages, lastPageParam) !== null;
	});

	const hasPreviousPage = $derived(() => {
		if (!pages || pages.length === 0 || !getPreviousPageParam) return false;
		const firstPage = pages[0];
		const firstPageParam = pageParams[0];
		if (!firstPage || firstPageParam === undefined) return false;
		return getPreviousPageParam(firstPage, pages, firstPageParam) !== null;
	});

	// Cleanup refs
	let intervalId: ReturnType<typeof setInterval> | null = null;
	let isDestroyed = false;

	/**
	 * Fetch a page
	 */
	async function fetchPage(pageParam: TPageParam, direction: 'next' | 'previous' | 'refresh'): Promise<void> {
		if (isDestroyed || !browser) return;

		if (direction === 'next') {
			isFetchingNextPage = true;
		} else if (direction === 'previous') {
			isFetchingPreviousPage = true;
		}
		isFetching = true;

		if (pages === null) {
			status = 'pending';
		}

		let lastError: ApiError | null = null;

		for (let attempt = 0; attempt <= retry; attempt++) {
			try {
				const pageData = await fetcher(pageParam);

				if (direction === 'refresh') {
					pages = [pageData];
					pageParams = [pageParam];
				} else if (direction === 'next') {
					pages = [...(pages ?? []), pageData];
					pageParams = [...pageParams, pageParam];
				} else {
					pages = [pageData, ...(pages ?? [])];
					pageParams = [pageParam, ...pageParams];
				}

				error = null;
				status = 'success';
				dataUpdatedAt = Date.now();

				onSuccess?.(pageData);
				onSettled?.(pageData, null);

				isFetching = false;
				isFetchingNextPage = false;
				isFetchingPreviousPage = false;
				return;
			} catch (e) {
				lastError = isApiError(e)
					? e
					: new (await import('./errors')).UnknownError(
							e instanceof Error ? e.message : 'Unknown error'
						);

				if (!isRetryableError(lastError) || attempt === retry) {
					break;
				}

				await new Promise((resolve) => setTimeout(resolve, retryDelay * (attempt + 1)));
			}
		}

		error = lastError;
		status = 'error';
		errorUpdatedAt = Date.now();

		if (lastError) {
			onError?.(lastError);
			onSettled?.(null, lastError);
		}

		isFetching = false;
		isFetchingNextPage = false;
		isFetchingPreviousPage = false;
	}

	/**
	 * Fetch next page
	 */
	async function fetchNextPage(): Promise<void> {
		if (!pages || pages.length === 0) return;

		const lastPage = pages[pages.length - 1];
		const lastPageParam = pageParams[pageParams.length - 1];
		if (!lastPage || lastPageParam === undefined) return;

		const nextParam = getNextPageParam(lastPage, pages, lastPageParam);
		if (nextParam === null) return;

		await fetchPage(nextParam, 'next');
	}

	/**
	 * Fetch previous page
	 */
	async function fetchPreviousPage(): Promise<void> {
		if (!pages || pages.length === 0 || !getPreviousPageParam) return;

		const firstPage = pages[0];
		const firstPageParam = pageParams[0];
		if (!firstPage || firstPageParam === undefined) return;

		const prevParam = getPreviousPageParam(firstPage, pages, firstPageParam);
		if (prevParam === null) return;

		await fetchPage(prevParam, 'previous');
	}

	/**
	 * Refetch all pages
	 */
	async function refetch(): Promise<void> {
		await fetchPage(initialPageParam, 'refresh');
	}

	/**
	 * Invalidate and refetch
	 */
	async function invalidate(): Promise<void> {
		pages = null;
		pageParams = [initialPageParam];
		await fetchPage(initialPageParam, 'refresh');
	}

	// Initial fetch
	if (browser && enabled) {
		untrack(() => {
			fetchPage(initialPageParam, 'refresh');
		});
	}

	// Polling
	if (browser && refetchInterval > 0) {
		intervalId = setInterval(() => {
			fetchPage(initialPageParam, 'refresh');
		}, refetchInterval);
	}

	// Cleanup
	onDestroy(() => {
		isDestroyed = true;
		if (intervalId) {
			clearInterval(intervalId);
		}
	});

	return {
		get data() { return pages; },
		get flatData() { return flatData; },
		get isLoading() { return isLoading; },
		get isError() { return isError; },
		get isSuccess() { return isSuccess; },
		get status() { return status; },
		get error() { return error; },
		get isStale() { return isStale; },
		get isFetching() { return isFetching; },
		get hasNextPage() { return hasNextPage(); },
		get hasPreviousPage() { return hasPreviousPage(); },
		get isFetchingNextPage() { return isFetchingNextPage; },
		get isFetchingPreviousPage() { return isFetchingPreviousPage; },
		get dataUpdatedAt() { return dataUpdatedAt; },
		get errorUpdatedAt() { return errorUpdatedAt; },
		refetch,
		invalidate,
		fetchNextPage,
		fetchPreviousPage,
	};
}

// =============================================================================
// PREFETCH HELPER - Preload data for navigation
// =============================================================================

/**
 * Prefetch data into cache for faster navigation
 *
 * @example
 * ```svelte
 * <a
 *   href="/dashboard/explosive-swings"
 *   onmouseenter={() => prefetch(() => api.get('/alerts/explosive-swings'))}
 * >
 *   Explosive Swings
 * </a>
 * ```
 */
export async function prefetch<T>(
	fetcher: () => Promise<T>,
	cacheConfig?: Partial<CacheConfig>
): Promise<void> {
	if (!browser) return;

	const cache = getCache();
	const cacheKey = `query:${fetcher.toString().slice(0, 100)}`;

	// Skip if already cached and fresh
	const cached = cache.get<T>(cacheKey);
	if (cached && !cached.isStale) return;

	try {
		const data = await fetcher();
		cache.set(cacheKey, data, cacheConfig);
	} catch {
		// Silently fail prefetch
	}
}

// =============================================================================
// CONVENIENCE FACTORIES - API Client Integration
// =============================================================================

/**
 * Create a query for an API endpoint
 */
export function useApiQuery<T>(
	endpoint: string,
	options?: QueryOptions<T> & { requestOptions?: RequestOptions }
): QueryResult<T> {
	const client = getApiClient();

	return createQuery(
		() => client.get<T>(endpoint, options?.requestOptions),
		options
	);
}

/**
 * Create a mutation for an API endpoint
 */
export function useApiMutation<TData, TVariables = unknown>(
	method: 'POST' | 'PUT' | 'PATCH' | 'DELETE',
	endpoint: string | ((variables: TVariables) => string),
	options?: MutationOptions<TData, TVariables>
): MutationResult<TData, TVariables> {
	const client = getApiClient();

	return createMutation(async (variables: TVariables) => {
		const url = typeof endpoint === 'function' ? endpoint(variables) : endpoint;

		switch (method) {
			case 'POST':
				return client.post<TData>(url, variables);
			case 'PUT':
				return client.put<TData>(url, variables);
			case 'PATCH':
				return client.patch<TData>(url, variables);
			case 'DELETE':
				return client.delete<TData>(url);
		}
	}, options);
}

// =============================================================================
// RE-EXPORT TYPES
// =============================================================================

export type {
	ApiError,
	RequestOptions,
	CacheConfig,
};

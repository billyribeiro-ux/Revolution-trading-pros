/**
 * ═══════════════════════════════════════════════════════════════════════════════
 * Explosive Swings - useAlerts Hook
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * @description Svelte 5 hook for managing alerts with real-time updates
 * @version 1.0.0
 * @standards Svelte 5 January 2026 Syntax
 *
 * Features:
 * - Paginated alert fetching
 * - Filter by alert type (ENTRY, EXIT, UPDATE)
 * - Auto-refresh capability
 * - Optimistic prepending for real-time updates
 */

import { getEnterpriseClient } from '$lib/api/enterprise/client';
import { ROOM_SLUG, ALERTS_PER_PAGE } from '../constants';
import type { FormattedAlert } from '../page.api';
import type { PaginationState, AlertFilter } from '../types';

// ═══════════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════════

export interface UseAlertsOptions {
	/** Initial page number (default: 1) */
	initialPage?: number;
	/** Initial filter (default: 'all') */
	filter?: AlertFilter;
	/** Enable auto-refresh every 30 seconds (default: false) */
	autoRefresh?: boolean;
	/** Auto-refresh interval in ms (default: 30000) */
	refreshInterval?: number;
}

export interface UseAlertsReturn {
	// State (read-only via getters)
	readonly alerts: FormattedAlert[];
	readonly isLoading: boolean;
	readonly error: string | null;
	readonly currentPage: number;
	readonly totalPages: number;
	readonly selectedFilter: AlertFilter;
	readonly hasNextPage: boolean;
	readonly hasPrevPage: boolean;
	readonly pagination: PaginationState;
	readonly showingFrom: number;
	readonly showingTo: number;

	// Actions
	fetchAlerts: () => Promise<void>;
	nextPage: () => void;
	prevPage: () => void;
	goToPage: (page: number) => Promise<void>;
	setFilter: (filter: AlertFilter) => void;
	prependAlert: (alert: FormattedAlert) => void;
	refresh: () => Promise<void>;
}

// ═══════════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════

function formatTimeAgo(dateString: string): string {
	const date = new Date(dateString);
	const now = new Date();
	const diffMs = now.getTime() - date.getTime();
	const diffMins = Math.floor(diffMs / 60000);
	const diffHours = Math.floor(diffMs / 3600000);
	const diffDays = Math.floor(diffMs / 86400000);

	if (diffMins < 60) return `${diffMins} min ago`;
	if (diffHours < 24) return `${diffHours}h ago`;
	if (diffDays === 0)
		return `Today at ${date.toLocaleTimeString('en-US', { hour: 'numeric', minute: '2-digit' })}`;
	if (diffDays === 1)
		return `Yesterday at ${date.toLocaleTimeString('en-US', { hour: 'numeric', minute: '2-digit' })}`;

	return `${date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' })} at ${date.toLocaleTimeString('en-US', { hour: 'numeric', minute: '2-digit' })}`;
}

function formatAlert(apiAlert: Record<string, unknown>): FormattedAlert {
	return {
		id: apiAlert.id as number,
		type: apiAlert.alert_type as 'ENTRY' | 'EXIT' | 'UPDATE',
		ticker: apiAlert.ticker as string,
		title: apiAlert.title as string,
		time: formatTimeAgo(apiAlert.published_at as string),
		message: apiAlert.message as string,
		isNew: apiAlert.is_new as boolean,
		notes: (apiAlert.notes as string) || '',
		tosString: (apiAlert.tos_string as string) || undefined
	};
}

// ═══════════════════════════════════════════════════════════════════════════════
// HOOK IMPLEMENTATION
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Creates a reactive alerts state manager for the Explosive Swings dashboard.
 * Uses Svelte 5 runes for reactive state management.
 */
export function useAlerts(options: UseAlertsOptions = {}): UseAlertsReturn {
	const { initialPage = 1, filter = 'all', autoRefresh = false, refreshInterval = 30000 } = options;

	// ═══════════════════════════════════════════════════════════════════════════
	// REACTIVE STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let alerts = $state<FormattedAlert[]>([]);
	let isLoading = $state(false);
	let error = $state<string | null>(null);
	let currentPage = $state(initialPage);
	let selectedFilter = $state<AlertFilter>(filter);
	let pagination = $state<PaginationState>({ total: 0, limit: ALERTS_PER_PAGE, offset: 0 });

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE
	// ═══════════════════════════════════════════════════════════════════════════

	const totalPages = $derived(Math.ceil(pagination.total / pagination.limit) || 1);

	const filteredAlerts = $derived(
		selectedFilter === 'all'
			? alerts
			: alerts.filter((a) => a.type.toLowerCase() === selectedFilter)
	);

	const hasNextPage = $derived(currentPage < totalPages);
	const hasPrevPage = $derived(currentPage > 1);

	const showingFrom = $derived(pagination.total > 0 ? (currentPage - 1) * ALERTS_PER_PAGE + 1 : 0);
	const showingTo = $derived(Math.min(currentPage * ALERTS_PER_PAGE, pagination.total));

	// ═══════════════════════════════════════════════════════════════════════════
	// API CLIENT
	// ═══════════════════════════════════════════════════════════════════════════

	const client = getEnterpriseClient();

	// ═══════════════════════════════════════════════════════════════════════════
	// ACTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	async function fetchAlerts(): Promise<void> {
		isLoading = true;
		error = null;

		try {
			const offset = (currentPage - 1) * ALERTS_PER_PAGE;
			const response = await client.get<{
				success: boolean;
				data: Record<string, unknown>[];
				total?: number;
				error?: string;
			}>(`/api/alerts/${ROOM_SLUG}`, {
				params: { limit: ALERTS_PER_PAGE, offset }
			});

			if (!response.success) {
				throw new Error(response.error || 'Failed to fetch alerts');
			}

			alerts = response.data.map(formatAlert);
			pagination = {
				total: response.total || response.data.length,
				limit: ALERTS_PER_PAGE,
				offset
			};
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to fetch alerts';
			console.error('Failed to fetch alerts:', e);
		} finally {
			isLoading = false;
		}
	}

	function nextPage(): void {
		if (hasNextPage) {
			currentPage++;
			fetchAlerts();
		}
	}

	function prevPage(): void {
		if (hasPrevPage) {
			currentPage--;
			fetchAlerts();
		}
	}

	async function goToPage(page: number): Promise<void> {
		if (page < 1 || page > totalPages || page === currentPage) return;
		currentPage = page;
		await fetchAlerts();
	}

	function setFilter(newFilter: AlertFilter): void {
		if (newFilter === selectedFilter) return;
		selectedFilter = newFilter;
		currentPage = 1;
		fetchAlerts();
	}

	function prependAlert(alert: FormattedAlert): void {
		alerts = [alert, ...alerts];
		pagination = { ...pagination, total: pagination.total + 1 };
	}

	async function refresh(): Promise<void> {
		await fetchAlerts();
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// EFFECTS
	// ═══════════════════════════════════════════════════════════════════════════

	// Auto-refresh effect
	$effect(() => {
		if (!autoRefresh) return;

		const interval = setInterval(fetchAlerts, refreshInterval);
		return () => clearInterval(interval);
	});

	// Initial fetch effect
	$effect(() => {
		fetchAlerts();
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// RETURN PUBLIC API
	// ═══════════════════════════════════════════════════════════════════════════

	return {
		// State (read-only via getters)
		get alerts() {
			return filteredAlerts;
		},
		get isLoading() {
			return isLoading;
		},
		get error() {
			return error;
		},
		get currentPage() {
			return currentPage;
		},
		get totalPages() {
			return totalPages;
		},
		get selectedFilter() {
			return selectedFilter;
		},
		get hasNextPage() {
			return hasNextPage;
		},
		get hasPrevPage() {
			return hasPrevPage;
		},
		get pagination() {
			return pagination;
		},
		get showingFrom() {
			return showingFrom;
		},
		get showingTo() {
			return showingTo;
		},

		// Actions
		fetchAlerts,
		nextPage,
		prevPage,
		goToPage,
		setFilter,
		prependAlert,
		refresh
	};
}

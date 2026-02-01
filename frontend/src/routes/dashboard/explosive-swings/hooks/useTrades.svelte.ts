/**
 * ═══════════════════════════════════════════════════════════════════════════════
 * Explosive Swings - useTrades Hook
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * @description Svelte 5 hook for managing trades (open and closed positions)
 * @version 1.0.0
 * @standards Svelte 5 January 2026 Syntax
 *
 * Features:
 * - Fetch all trades with automatic open/closed separation
 * - Position management (close, update, invalidate)
 * - Real-time position updates
 * - Derived calculations for display
 */

import { getEnterpriseClient } from '$lib/api/enterprise/client';
import { getPriceFeed } from '$lib/services/price-feed';
import { ROOM_SLUG, TRADES_PER_PAGE } from '../constants';
import type { ApiTrade, ActivePosition, ClosedTrade } from '../types';

/** Price data from the price feed service */
interface PriceData {
	ticker: string;
	price: number;
	change: number;
	changePercent: number;
	timestamp: number;
	marketOpen: boolean;
}

// ═══════════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════════

export interface UseTradesOptions {
	/** Enable auto-refresh (default: false) */
	autoRefresh?: boolean;
	/** Auto-refresh interval in ms (default: 60000) */
	refreshInterval?: number;
	/** Maximum closed trades to display (default: 10) */
	maxClosedTrades?: number;
	/** Enable real-time price updates for active positions (default: true) */
	enableRealTimePrices?: boolean;
}

export interface UseTradesReturn {
	// State (read-only via getters)
	readonly openTrades: ApiTrade[];
	readonly closedTrades: ApiTrade[];
	readonly activePositions: ActivePosition[];
	readonly recentClosedTrades: ClosedTrade[];
	readonly isLoading: boolean;
	readonly error: string | null;
	readonly totalTrades: number;
	readonly openCount: number;
	readonly closedCount: number;

	// Actions
	fetchTrades: () => Promise<void>;
	closeTrade: (tradeId: string | number, exitData: CloseTradeData) => Promise<void>;
	updateTrade: (tradeId: string | number, updateData: UpdateTradeData) => Promise<void>;
	invalidateTrade: (tradeId: string | number, reason: string) => Promise<void>;
	deleteTrade: (tradeId: string | number) => Promise<void>;
	addTrade: (tradeData: NewTradeData) => Promise<void>;
	refresh: () => Promise<void>;
}

export interface CloseTradeData {
	exitPrice: number;
	exitDate?: string;
	notes?: string;
}

export interface UpdateTradeData {
	entryPrice?: number;
	stopLoss?: number;
	target1?: number;
	target2?: number;
	target3?: number;
	notes?: string;
}

export interface NewTradeData {
	ticker: string;
	direction: 'long' | 'short';
	entryPrice: number;
	stopLoss?: number;
	target1?: number;
	target2?: number;
	target3?: number;
	notes?: string;
}

// ═══════════════════════════════════════════════════════════════════════════════
// EXTENDED TYPES FOR INTERNAL USE
// ═══════════════════════════════════════════════════════════════════════════════

/** Extended ApiTrade with optional fields that may come from backend */
interface ApiTradeExtended extends ApiTrade {
	stop_loss?: number;
	target1?: number;
	target2?: number;
	target3?: number;
	was_updated?: boolean;
	updated_at?: string;
}

// ═══════════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Builds an array of position targets from trade data.
 * Calculates percentage from entry for each target.
 */
function buildTargetsArray(
	trade: ApiTradeExtended,
	entryPrice: number
): { price: number; percentFromEntry: number; label: string }[] {
	const targets: { price: number; percentFromEntry: number; label: string }[] = [];

	if (entryPrice <= 0) return targets;

	if (trade.target1 && trade.target1 > 0) {
		const percent = ((trade.target1 - entryPrice) / entryPrice) * 100;
		targets.push({
			price: trade.target1,
			percentFromEntry: Math.round(percent * 100) / 100,
			label: 'Target 1'
		});
	}

	if (trade.target2 && trade.target2 > 0) {
		const percent = ((trade.target2 - entryPrice) / entryPrice) * 100;
		targets.push({
			price: trade.target2,
			percentFromEntry: Math.round(percent * 100) / 100,
			label: 'Target 2'
		});
	}

	if (trade.target3 && trade.target3 > 0) {
		const percent = ((trade.target3 - entryPrice) / entryPrice) * 100;
		targets.push({
			price: trade.target3,
			percentFromEntry: Math.round(percent * 100) / 100,
			label: 'Target 3'
		});
	}

	return targets;
}

// ═══════════════════════════════════════════════════════════════════════════════
// HOOK IMPLEMENTATION
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Creates a reactive trades state manager for the Explosive Swings dashboard.
 * Uses Svelte 5 runes for reactive state management.
 */
export function useTrades(options: UseTradesOptions = {}): UseTradesReturn {
	const {
		autoRefresh = false,
		refreshInterval = 60000,
		maxClosedTrades = 10,
		enableRealTimePrices = true
	} = options;

	// Price feed service for real-time price updates
	const priceFeed = enableRealTimePrices ? getPriceFeed() : null;
	let realTimePrices = $state<Map<string, PriceData>>(new Map());

	// ═══════════════════════════════════════════════════════════════════════════
	// REACTIVE STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let allTrades = $state<ApiTrade[]>([]);
	let isLoading = $state(false);
	let error = $state<string | null>(null);

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE
	// ═══════════════════════════════════════════════════════════════════════════

	const openTrades = $derived(allTrades.filter((t) => t.status === 'open'));
	const closedTrades = $derived(allTrades.filter((t) => t.status === 'closed'));

	const totalTrades = $derived(allTrades.length);
	const openCount = $derived(openTrades.length);
	const closedCount = $derived(closedTrades.length);

	// Transform open trades to ActivePosition format with real-time prices
	const activePositions = $derived<ActivePosition[]>(
		openTrades.map((t) => {
			const tickerUpper = t.ticker.toUpperCase();
			const priceData = realTimePrices.get(tickerUpper);

			// Use real-time price if available, otherwise estimate from entry
			const currentPrice = priceData?.price ?? t.entry_price;
			const entryPrice = t.entry_price;

			// Calculate unrealized P&L percentage
			const unrealizedPercent =
				entryPrice > 0 ? ((currentPrice - entryPrice) / entryPrice) * 100 : 0;

			// Calculate stop loss (use trade data if available, else default -5%)
			const stopPrice = (t as ApiTradeExtended).stop_loss ?? entryPrice * 0.95;
			const stopPercent = entryPrice > 0 ? ((stopPrice - entryPrice) / entryPrice) * 100 : -5;

			// Calculate progress to first target
			const target1 = (t as ApiTradeExtended).target1 ?? entryPrice * 1.05;
			const progressToTarget1 =
				target1 > entryPrice
					? Math.min(100, Math.max(0, ((currentPrice - entryPrice) / (target1 - entryPrice)) * 100))
					: 0;

			return {
				id: String(t.id),
				ticker: t.ticker,
				status: 'ACTIVE' as const,
				entryPrice,
				currentPrice,
				unrealizedPercent: Math.round(unrealizedPercent * 100) / 100,
				targets: buildTargetsArray(t as ApiTradeExtended, entryPrice),
				stopLoss: {
					price: stopPrice,
					percentFromEntry: Math.round(stopPercent * 100) / 100
				},
				progressToTarget1: Math.round(progressToTarget1),
				triggeredAt: new Date(t.entry_date),
				notes: t.notes,
				wasUpdated: (t as ApiTradeExtended).was_updated ?? false,
				updatedAt: (t as ApiTradeExtended).updated_at
					? new Date((t as ApiTradeExtended).updated_at!)
					: undefined
			};
		})
	);

	// Transform closed trades to ClosedTrade format
	const recentClosedTrades = $derived<ClosedTrade[]>(
		closedTrades.slice(0, maxClosedTrades).map((t) => ({
			id: String(t.id),
			ticker: t.ticker,
			percentageGain: t.pnl_percent ?? 0,
			isWinner: (t.pnl_percent ?? 0) > 0,
			closedAt: new Date(t.exit_date ?? t.entry_date),
			entryPrice: t.entry_price,
			exitPrice: t.exit_price ?? t.entry_price
		}))
	);

	// ═══════════════════════════════════════════════════════════════════════════
	// API CLIENT
	// ═══════════════════════════════════════════════════════════════════════════

	const client = getEnterpriseClient();

	// ═══════════════════════════════════════════════════════════════════════════
	// ACTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	async function fetchTrades(): Promise<void> {
		isLoading = true;
		error = null;

		try {
			const response = await client.get<{
				success: boolean;
				data: ApiTrade[];
				error?: string;
			}>(`/api/trades/${ROOM_SLUG}`, {
				params: { per_page: TRADES_PER_PAGE }
			});

			if (!response.success) {
				throw new Error(response.error || 'Failed to fetch trades');
			}

			allTrades = response.data;
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to fetch trades';
			console.error('Failed to fetch trades:', e);
		} finally {
			isLoading = false;
		}
	}

	async function closeTrade(tradeId: string | number, exitData: CloseTradeData): Promise<void> {
		isLoading = true;
		error = null;

		try {
			const response = await client.post<{ success: boolean; error?: string }>(
				`/api/admin/trades/${tradeId}/close`,
				{
					exit_price: exitData.exitPrice,
					exit_date: exitData.exitDate || new Date().toISOString(),
					notes: exitData.notes
				}
			);

			if (!response.success) {
				throw new Error(response.error || 'Failed to close trade');
			}

			// Refresh trades to get updated data
			await fetchTrades();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to close trade';
			console.error('Failed to close trade:', e);
			throw e;
		} finally {
			isLoading = false;
		}
	}

	async function updateTrade(tradeId: string | number, updateData: UpdateTradeData): Promise<void> {
		isLoading = true;
		error = null;

		try {
			const response = await client.patch<{ success: boolean; error?: string }>(
				`/api/admin/trades/${tradeId}`,
				{
					entry_price: updateData.entryPrice,
					stop_loss: updateData.stopLoss,
					target1: updateData.target1,
					target2: updateData.target2,
					target3: updateData.target3,
					notes: updateData.notes
				}
			);

			if (!response.success) {
				throw new Error(response.error || 'Failed to update trade');
			}

			// Refresh trades to get updated data
			await fetchTrades();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to update trade';
			console.error('Failed to update trade:', e);
			throw e;
		} finally {
			isLoading = false;
		}
	}

	async function invalidateTrade(tradeId: string | number, reason: string): Promise<void> {
		isLoading = true;
		error = null;

		try {
			const response = await client.post<{ success: boolean; error?: string }>(
				`/api/admin/trades/${tradeId}/invalidate`,
				{ reason }
			);

			if (!response.success) {
				throw new Error(response.error || 'Failed to invalidate trade');
			}

			// Refresh trades to get updated data
			await fetchTrades();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to invalidate trade';
			console.error('Failed to invalidate trade:', e);
			throw e;
		} finally {
			isLoading = false;
		}
	}

	async function deleteTrade(tradeId: string | number): Promise<void> {
		isLoading = true;
		error = null;

		try {
			const response = await client.delete<{ success: boolean; error?: string }>(
				`/api/admin/trades/${tradeId}`
			);

			if (!response.success) {
				throw new Error(response.error || 'Failed to delete trade');
			}

			// Remove from local state optimistically
			allTrades = allTrades.filter((t) => String(t.id) !== String(tradeId));
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to delete trade';
			console.error('Failed to delete trade:', e);
			throw e;
		} finally {
			isLoading = false;
		}
	}

	async function addTrade(tradeData: NewTradeData): Promise<void> {
		isLoading = true;
		error = null;

		try {
			const response = await client.post<{ success: boolean; data?: ApiTrade; error?: string }>(
				`/api/admin/trades/${ROOM_SLUG}`,
				{
					ticker: tradeData.ticker,
					direction: tradeData.direction,
					entry_price: tradeData.entryPrice,
					stop_loss: tradeData.stopLoss,
					target1: tradeData.target1,
					target2: tradeData.target2,
					target3: tradeData.target3,
					notes: tradeData.notes,
					entry_date: new Date().toISOString()
				}
			);

			if (!response.success) {
				throw new Error(response.error || 'Failed to add trade');
			}

			// Refresh trades to get the new trade with proper ID
			await fetchTrades();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to add trade';
			console.error('Failed to add trade:', e);
			throw e;
		} finally {
			isLoading = false;
		}
	}

	async function refresh(): Promise<void> {
		await fetchTrades();
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// EFFECTS
	// ═══════════════════════════════════════════════════════════════════════════

	// Auto-refresh effect
	$effect(() => {
		if (!autoRefresh) return;

		const interval = setInterval(fetchTrades, refreshInterval);
		return () => clearInterval(interval);
	});

	// Initial fetch effect
	$effect(() => {
		fetchTrades();
	});

	// Real-time price subscription effect
	$effect(() => {
		if (!priceFeed || openTrades.length === 0) return;

		// Subscribe to price updates for all open position tickers
		const tickers = openTrades.map((t) => t.ticker.toUpperCase());
		priceFeed.subscribe(tickers);

		// Register callback for price updates
		const unsubscribe = priceFeed.onUpdate((prices) => {
			realTimePrices = prices;
		});

		// Cleanup: unsubscribe when positions change or component unmounts
		return () => {
			unsubscribe();
			priceFeed.unsubscribe(tickers);
		};
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// RETURN PUBLIC API
	// ═══════════════════════════════════════════════════════════════════════════

	return {
		// State (read-only via getters)
		get openTrades() {
			return openTrades;
		},
		get closedTrades() {
			return closedTrades;
		},
		get activePositions() {
			return activePositions;
		},
		get recentClosedTrades() {
			return recentClosedTrades;
		},
		get isLoading() {
			return isLoading;
		},
		get error() {
			return error;
		},
		get totalTrades() {
			return totalTrades;
		},
		get openCount() {
			return openCount;
		},
		get closedCount() {
			return closedCount;
		},

		// Actions
		fetchTrades,
		closeTrade,
		updateTrade,
		invalidateTrade,
		deleteTrade,
		addTrade,
		refresh
	};
}

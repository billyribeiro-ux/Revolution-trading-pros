/**
 * ═══════════════════════════════════════════════════════════════════════════════
 * Explosive Swings Dashboard - Calculation Utilities
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * @description Calculation functions for the Explosive Swings dashboard
 * @version 4.0.0 - January 2026 - Nuclear Build Specification
 * @standards Apple Principal Engineer ICT 7+ Standards
 */

/**
 * Calculate progress percentage towards a target
 * @param entry - Entry price
 * @param current - Current price
 * @param target - Target price
 * @returns Progress percentage (0-100)
 */
export function calculateProgress(entry: number, current: number, target: number): number {
	const totalDistance = target - entry;
	if (totalDistance === 0) return 0;
	const currentDistance = current - entry;
	return Math.max(0, Math.min(100, (currentDistance / totalDistance) * 100));
}

/**
 * Calculate percentage change between two prices
 * @param entry - Entry price
 * @param current - Current price
 * @returns Percentage change
 */
export function calculatePercentChange(entry: number, current: number): number {
	if (entry === 0) return 0;
	return ((current - entry) / entry) * 100;
}

/**
 * Calculate win rate percentage
 * @param wins - Number of winning trades
 * @param total - Total number of trades
 * @returns Win rate as percentage (0-100)
 */
export function calculateWinRate(wins: number, total: number): number {
	if (total === 0) return 0;
	return (wins / total) * 100;
}

/**
 * Calculate risk/reward ratio
 * @param avgWin - Average win percentage
 * @param avgLoss - Average loss percentage (absolute value)
 * @returns Risk/reward ratio
 */
export function calculateRiskReward(avgWin: number, avgLoss: number): number {
	if (avgLoss === 0) return 0;
	return Math.abs(avgWin / avgLoss);
}

/**
 * Determine if an alert is "new" (< 30 minutes old)
 * @param timestamp - Alert timestamp
 * @returns True if the alert is less than 30 minutes old
 */
export function isAlertNew(timestamp: Date): boolean {
	const thirtyMinutesMs = 30 * 60 * 1000;
	const now = new Date();
	return now.getTime() - timestamp.getTime() < thirtyMinutesMs;
}

/**
 * Calculate percentage from entry to target
 * @param entry - Entry price
 * @param target - Target price
 * @returns Percentage to target
 */
export function calculatePercentToTarget(entry: number, target: number): number {
	if (entry === 0) return 0;
	return ((target - entry) / entry) * 100;
}

/**
 * Calculate percentage to stop loss (always negative for longs)
 * @param entry - Entry price
 * @param stop - Stop loss price
 * @returns Percentage to stop (typically negative)
 */
export function calculatePercentToStop(entry: number, stop: number): number {
	if (entry === 0) return 0;
	return ((stop - entry) / entry) * 100;
}

/**
 * Determine position status color class based on unrealized P&L
 * @param unrealizedPercent - Unrealized percentage (can be null for WATCHING)
 * @param status - Position status
 * @returns Tailwind color classes
 */
export function getPositionStatusColor(
	unrealizedPercent: number | null,
	status: 'ENTRY' | 'WATCHING' | 'ACTIVE'
): string {
	if (status === 'ENTRY') {
		return 'bg-teal-100 text-teal-700 border-teal-300';
	}
	if (status === 'WATCHING') {
		return 'bg-amber-100 text-amber-700 border-amber-300';
	}
	if (unrealizedPercent === null) {
		return 'bg-slate-100 text-slate-700 border-slate-300';
	}
	if (unrealizedPercent >= 0) {
		return 'bg-emerald-100 text-emerald-700 border-emerald-300';
	}
	return 'bg-red-100 text-red-700 border-red-300';
}

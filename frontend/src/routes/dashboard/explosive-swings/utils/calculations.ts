/**
 * ═══════════════════════════════════════════════════════════════════════════════
 * Explosive Swings Dashboard - Calculation Utilities
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * @description Calculation functions for the Explosive Swings dashboard
 * @version 4.1.0 - Visual Polish Pass
 * @standards Apple Principal Engineer ICT 7+ Standards
 */

import type { PositionStatus } from '../types';

/**
 * Calculate progress towards target as a percentage
 * @param entry - Entry price
 * @param current - Current price
 * @param target - Target price
 * @returns Progress percentage (0-100, clamped)
 */
export function calculateProgress(entry: number, current: number, target: number): number {
	if (entry === target) return 0;
	const totalDistance = target - entry;
	const currentDistance = current - entry;
	const progress = (currentDistance / totalDistance) * 100;
	return Math.max(0, Math.min(100, progress));
}

/**
 * Calculate percentage change between two prices
 * @param from - Starting price
 * @param to - Ending price
 * @returns Percentage change
 */
export function calculatePercentChange(from: number, to: number): number {
	if (from === 0) return 0;
	return ((to - from) / from) * 100;
}

/**
 * Get position status color class based on unrealized P&L and status
 * @param unrealizedPercent - Current unrealized percentage (can be null)
 * @param status - Position status
 * @returns Tailwind CSS class string for the status badge
 */
export function getPositionStatusColor(
	unrealizedPercent: number | null,
	status: PositionStatus
): string {
	switch (status) {
		case 'ENTRY':
			return 'badge-entry';
		case 'WATCHING':
			return 'badge-watching';
		case 'ACTIVE':
			if (unrealizedPercent !== null && unrealizedPercent >= 0) {
				return 'badge-active-profit';
			}
			return 'badge-active-loss';
		default:
			return 'badge-default';
	}
}

/**
 * Calculate risk/reward ratio from price levels
 * @param entry - Entry price
 * @param target - Target price
 * @param stop - Stop loss price
 * @returns Risk/reward ratio
 */
export function calculateRiskReward(entry: number, target: number, stop: number): number {
	const reward = Math.abs(target - entry);
	const risk = Math.abs(entry - stop);
	if (risk === 0) return 0;
	return reward / risk;
}

/**
 * Calculate risk/reward ratio from average percentages
 * @param avgWinPercent - Average win percentage
 * @param avgLossPercent - Average loss percentage (absolute value)
 * @returns Risk/reward ratio
 */
export function calculateRiskRewardFromPercentages(
	avgWinPercent: number,
	avgLossPercent: number
): number {
	const absLoss = Math.abs(avgLossPercent);
	if (absLoss === 0) return 0;
	return Math.abs(avgWinPercent) / absLoss;
}

/**
 * Determine if an alert should show as "new"
 * @param timestamp - Alert timestamp
 * @param thresholdMinutes - Minutes threshold (default: 30)
 * @returns Whether the alert is new
 */
export function isAlertNew(timestamp: Date, thresholdMinutes: number = 30): boolean {
	const now = new Date();
	const diffMs = now.getTime() - timestamp.getTime();
	const diffMinutes = diffMs / (1000 * 60);
	return diffMinutes < thresholdMinutes;
}

/**
 * Calculate win rate from trades
 * @param wins - Number of winning trades
 * @param total - Total number of trades
 * @returns Win rate as percentage (0-100)
 */
export function calculateWinRate(wins: number, total: number): number {
	if (total === 0) return 0;
	return (wins / total) * 100;
}

/**
 * Calculate average of an array of numbers
 * @param values - Array of numbers
 * @returns Average value
 */
export function calculateAverage(values: number[]): number {
	if (values.length === 0) return 0;
	const sum = values.reduce((acc, val) => acc + val, 0);
	return sum / values.length;
}

/**
 * Calculate expectancy per trade
 * @param winRate - Win rate as decimal (0-1)
 * @param avgWin - Average win percentage
 * @param avgLoss - Average loss percentage (positive number)
 * @returns Expected percentage gain per trade
 */
export function calculateExpectancy(winRate: number, avgWin: number, avgLoss: number): number {
	return winRate * avgWin - (1 - winRate) * avgLoss;
}

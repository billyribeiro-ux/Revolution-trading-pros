/**
 * RevolutionBehavior-L8-System - Svelte 5 Runes Store
 * @version 2.0.0 - Migrated to Svelte 5 Runes (February 2026)
 */

import type { BehaviorSession, BehaviorDashboardData } from '$lib/behavior/types';

// Svelte 5 reactive state
let currentSessionValue = $state<BehaviorSession | null>(null);
let dashboardDataValue = $state<BehaviorDashboardData | null>(null);
let isLoadingValue = $state(false);

// Derived state
const sessionScoreValue = $derived.by(() => {
	if (!currentSessionValue) return null;
	return {
		engagement: currentSessionValue.engagement_score,
		intent: currentSessionValue.intent_score,
		friction: currentSessionValue.friction_score,
		churnRisk: currentSessionValue.churn_risk_score
	};
});

// Exported store object with getters and setters
export const behaviorStore = {
	get currentSession() {
		return currentSessionValue;
	},
	set currentSession(value: BehaviorSession | null) {
		currentSessionValue = value;
	},
	get dashboardData() {
		return dashboardDataValue;
	},
	set dashboardData(value: BehaviorDashboardData | null) {
		dashboardDataValue = value;
	},
	get isLoading() {
		return isLoadingValue;
	},
	set isLoading(value: boolean) {
		isLoadingValue = value;
	},
	get sessionScore() {
		return sessionScoreValue;
	}
};

// Legacy exports for backward compatibility
export const currentSession = {
	get value() {
		return currentSessionValue;
	},
	set(value: BehaviorSession | null) {
		currentSessionValue = value;
	}
};

export const dashboardData = {
	get value() {
		return dashboardDataValue;
	},
	set(value: BehaviorDashboardData | null) {
		dashboardDataValue = value;
	}
};

export const isLoading = {
	get value() {
		return isLoadingValue;
	},
	set(value: boolean) {
		isLoadingValue = value;
	}
};

export const sessionScore = {
	get value() {
		return sessionScoreValue;
	}
};

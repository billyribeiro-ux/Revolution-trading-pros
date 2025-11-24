/**
 * RevolutionBehavior-L8-System - Svelte Store
 */

import { writable, derived } from 'svelte/store';
import type { BehaviorSession, BehaviorDashboardData } from '$lib/behavior/types';

export const currentSession = writable<BehaviorSession | null>(null);
export const dashboardData = writable<BehaviorDashboardData | null>(null);
export const isLoading = writable(false);

export const sessionScore = derived(currentSession, $session => {
	if (!$session) return null;
	return {
		engagement: $session.engagement_score,
		intent: $session.intent_score,
		friction: $session.friction_score,
		churnRisk: $session.churn_risk_score
	};
});

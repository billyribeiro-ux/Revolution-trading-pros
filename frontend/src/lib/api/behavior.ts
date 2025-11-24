/**
 * RevolutionBehavior-L8-System - API Client
 */

import type { BehaviorEventBatch, BehaviorSession, BehaviorDashboardData, FrictionPoint, IntentSignal } from '$lib/behavior/types';
import { apiClient } from './client';

export class BehaviorAPI {
	async sendEvents(batch: BehaviorEventBatch): Promise<void> {
		return apiClient.post('/admin/behavior/events', batch);
	}

	async getSession(sessionId: string): Promise<BehaviorSession> {
		return apiClient.get(`/admin/behavior/sessions/${sessionId}`);
	}

	async getDashboard(period: string = '7d'): Promise<BehaviorDashboardData> {
		return apiClient.get('/admin/behavior/dashboard', { params: { period } });
	}

	async getFrictionPoints(filters?: {
		page_url?: string;
		friction_type?: string;
		severity?: string;
		resolved?: boolean;
	}): Promise<FrictionPoint[]> {
		return apiClient.get('/admin/behavior/friction-points', { params: filters });
	}

	async getIntentSignals(filters?: {
		user_id?: string;
		signal_type?: string;
		converted?: boolean;
	}): Promise<IntentSignal[]> {
		return apiClient.get('/admin/behavior/intent-signals', { params: filters });
	}

	async resolveFrictionPoint(id: string, notes?: string): Promise<void> {
		return apiClient.patch(`/admin/behavior/friction-points/${id}/resolve`, { notes });
	}
}

export const behaviorAPI = new BehaviorAPI();

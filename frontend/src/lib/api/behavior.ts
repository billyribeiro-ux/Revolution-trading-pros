/**
 * ⚠️ FIX-2026-04-26: ORPHAN — no matching backend route exists.
 * This client file was identified by the 2026-04-26 audit as having no Rust
 * handler under api/src/routes/. Calls from this file will 404.
 *
 * Decision required: either build the matching backend route or delete this
 * client. For now, exports are kept for git history; importers should not
 * call into this file in production code paths.
 *
 * See docs/audits/AUDIT_REPORT.md §10 Finding 2 + §12 Finding 1.
 */

/**
 * RevolutionBehavior-L8-System - API Client
 */

import type {
	BehaviorEventBatch,
	BehaviorSession,
	BehaviorDashboardData,
	FrictionPoint,
	IntentSignal
} from '$lib/behavior/types';
import { apiClient } from './client.svelte';

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

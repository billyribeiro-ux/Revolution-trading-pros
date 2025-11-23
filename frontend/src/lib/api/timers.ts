import { API_ENDPOINTS, apiFetch } from './config';

export type TimerEventType =
	| 'start'
	| 'pause'
	| 'resume'
	| 'stop'
	| 'reset'
	| 'update'
	| 'warning'
	| 'danger'
	| 'milestone'
	| 'expire';

export interface TimerEventPayload {
	timer_id: string;
	type: TimerEventType;
	timestamp: string;
	remaining_ms: number;
	payload?: Record<string, any>;
}

/**
 * Record a timer lifecycle or analytics event on the backend.
 */
export async function recordTimerEvent(event: TimerEventPayload): Promise<void> {
	await apiFetch<{ status: string }>(API_ENDPOINTS.timers.events, {
		method: 'POST',
		body: JSON.stringify(event)
	});
}

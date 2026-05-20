import { apiClient } from './client.svelte';
import { API_ENDPOINTS } from './config';
import type { JsonValue } from './_types';

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
	/**
	 * Free-form event-specific payload. JSON-serialised onto the wire and
	 * stored as JSONB on the backend `timer_events.payload` column —
	 * `JsonValue` is the strictly-correct shape (no `any` leaking into the
	 * caller's spread).
	 */
	payload?: { [k: string]: JsonValue | undefined };
}

/**
 * Record a timer lifecycle or analytics event on the backend.
 */
export async function recordTimerEvent(event: TimerEventPayload): Promise<void> {
	await apiClient.post<{ status: string }>(API_ENDPOINTS.timers.events, event);
}

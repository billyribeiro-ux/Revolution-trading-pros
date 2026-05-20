// Shared types for /admin/schedules sub-components.
// Hoisted from +page.svelte in R21-C so child callbacks stay contravariant
// with the parent's ScheduleEvent type.

export interface ScheduleException {
	id: number;
	schedule_id: number;
	date: string; // YYYY-MM-DD
	type: 'cancelled' | 'rescheduled' | 'holiday';
	reason: string | null;
	new_start_time?: string;
	new_end_time?: string;
}

export interface ScheduleEvent {
	id: number;
	room_id: string;
	title: string;
	description: string | null;
	trader_name: string | null;
	day_of_week: number; // 0 = Sunday, 1 = Monday, etc.
	start_time: string; // HH:MM format
	end_time: string;
	timezone: string;
	is_active: boolean;
	room_type: 'live' | 'recorded' | 'hybrid';
	recurrence: 'weekly' | 'biweekly' | 'monthly' | null;
	exceptions: ScheduleException[];
	created_at: string;
	updated_at: string;
}

export interface ScheduleForm {
	room_id: string;
	title: string;
	description: string;
	trader_name: string;
	day_of_week: number;
	start_time: string;
	end_time: string;
	timezone: string;
	room_type: 'live' | 'recorded' | 'hybrid';
	recurrence: 'weekly' | 'biweekly' | 'monthly' | null;
	is_active: boolean;
}

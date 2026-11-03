/**
 * Shared types for the Contact Detail page components.
 *
 * Extracted from +page.svelte during R15-C maintenance pass —
 * see /tmp/maint-reports/R15-C-extractions.md.
 */

export interface Contact {
	id: string;
	email: string;
	phone?: string;
	mobile?: string;
	first_name?: string;
	last_name?: string;
	full_name: string;
	job_title?: string;
	department?: string;
	company_name?: string;
	company_id?: string;
	status: 'subscribed' | 'pending' | 'unsubscribed' | 'bounced' | 'complained';
	lifecycle_stage:
		| 'subscriber'
		| 'lead'
		| 'mql'
		| 'sql'
		| 'opportunity'
		| 'customer'
		| 'evangelist';
	lead_score: number;
	health_score: number;
	engagement_score: number;
	value_score: number;
	email_opens: number;
	email_clicks: number;
	total_sessions: number;
	avg_engagement_score: number;
	avg_intent_score: number;
	friction_events_count: number;
	subscription_status: string;
	subscription_mrr: number;
	lifetime_value: number;
	deals_count: number;
	activities_count: number;
	notes_count: number;
	tasks_count: number;
	is_verified: boolean;
	is_unsubscribed: boolean;
	do_not_contact: boolean;
	is_vip: boolean;
	address_line1?: string;
	address_line2?: string;
	city?: string;
	state?: string;
	postal_code?: string;
	country?: string;
	timezone?: string;
	website?: string;
	linkedin_url?: string;
	twitter_handle?: string;
	last_activity_at?: string;
	last_contacted_at?: string;
	next_followup_at?: string;
	first_touch_channel?: string;
	last_touch_channel?: string;
	created_at: string;
	updated_at: string;
	tags?: { id: string; name: string; color?: string }[];
	lists?: { id: string; name: string }[];
	custom_fields?: Record<string, unknown>;
}

export interface TimelineEvent {
	id: string;
	title: string;
	description?: string;
	type: string;
	occurred_at: string;
	created_by?: { name: string };
}

export interface EmailActivity {
	id: string;
	subject: string;
	status: 'sent' | 'opened' | 'clicked' | 'bounced' | 'failed';
	sent_at: string;
	opened_at?: string;
	clicked_at?: string;
	campaign_name?: string;
}

export interface Note {
	id: string;
	content: string;
	created_at: string;
	created_by: { name: string };
}

export interface TagOption {
	id: string;
	name: string;
	color?: string;
}

export interface ListOption {
	id: string;
	name: string;
}

export interface EmailTemplateOption {
	id: string | number;
	name: string;
	subject: string;
	body_html?: string;
	body_text?: string;
}

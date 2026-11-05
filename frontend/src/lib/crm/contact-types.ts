import type { JsonValue } from '#lib/api/_types.js';

export const contactStatuses = [
	'lead',
	'prospect',
	'customer',
	'churned',
	'unqualified',
	'new',
	'contacted',
	'qualified',
	'proposal',
	'negotiation',
	'won',
	'lost'
] as const;
export const lifecycleStages = [
	'subscriber',
	'lead',
	'mql',
	'sql',
	'opportunity',
	'customer',
	'evangelist'
] as const;
export type LifecycleStage = (typeof lifecycleStages)[number];
export interface ContactScoring {
	status:
		| 'calculated'
		| 'not_calculated'
		| 'stale'
		| 'disabled'
		| 'unconfigured'
		| 'invalid_policy';
	scope: 'configured_contact_rules';
	score: number | null;
	max_score: number | null;
	evaluated_at: string | null;
	policy_hash: string | null;
	reason: string | null;
	contributions: { rule_id: string; matched: boolean; points: number; applied_points: number }[];
}
export interface ContactFacts {
	scope: 'recorded_contact_links';
	deals: { total: number; open: number; won: number; activities: number; notes: number };
	email: {
		scope: 'crm_outbox';
		accepted: number;
		open_signals: number;
		click_signals: number;
		human_engagement: null;
	};
	orders: {
		scope: 'linked_member_orders';
		total: number;
		completed: number;
		refunded: number;
	} | null;
	unmeasured: {
		health_score: null;
		engagement_score: null;
		value_score: null;
		lifetime_value: null;
		human_email_opens: null;
	};
}
/** The measured contact DTO. Legacy association APIs retain their separate Contact type. */
export interface ContactRecord {
	id: string;
	user_id: string | null;
	owner_id: string | null;
	email: string;
	full_name: string;
	first_name: string | null;
	last_name: string | null;
	phone: string | null;
	company: string | null;
	job_title: string | null;
	status: string | null;
	source: string | null;
	lifecycle_stage: LifecycleStage | null;
	tags: string[] | null;
	custom_fields: Record<string, JsonValue> | null;
	created_at: string | null;
	updated_at: string | null;
	last_contacted_at: string | null;
	last_activity_at: string | null;
	score: number | null;
	lead_score: number | null;
	recorded_score: number | null;
	historical_score: number | null;
	score_assessed_at: string | null;
	scoring: ContactScoring;
	facts?: ContactFacts;
}
export interface ContactWrite {
	email?: string;
	first_name?: string;
	last_name?: string;
	phone?: string;
	company?: string;
	job_title?: string;
	status?: string;
	source?: string;
	lifecycle_stage?: LifecycleStage | null;
	tags?: string[];
	custom_fields?: Record<string, JsonValue> | null;
}
export interface ContactFormFields {
	email: string;
	first_name: string;
	last_name: string;
	phone: string;
	company: string;
	job_title: string;
	status: string;
	lifecycle_stage: LifecycleStage | '';
}
export interface ContactQuery {
	page?: number;
	per_page?: number;
	search?: string;
	status?: string;
	lifecycle_stage?: string;
	tag?: string;
	list_id?: string;
}
export interface ContactPage<T> {
	data: T[];
	meta: { current_page: number; per_page: number; total: number; total_pages: number };
}
export interface ContactAuthor {
	id: string;
	name: string;
}
export interface ContactTimelineEvent {
	id: string;
	type: string;
	category: string;
	title: string;
	description: string | null;
	metadata: Record<string, JsonValue>;
	occurred_at: string;
	created_by: ContactAuthor | null;
}
export interface ContactTimelinePage extends ContactPage<ContactTimelineEvent> {
	contact_id: string;
	scope: 'recorded_contact_links';
	coverage: { sources: string[]; human_email_engagement: null; unlinked_address_history: null };
}
export interface ContactNote {
	id: string;
	content: string;
	created_at: string;
	created_by: ContactAuthor | null;
}
export interface ContactTag {
	id: string | null;
	name: string;
	color: string | null;
	definition_status: 'registered' | 'unregistered';
}
export interface ContactListMembership {
	id: string;
	name: string;
	subscribed_at: string;
}
export type ContactMessageStatus =
	| 'accepted'
	| 'rejected'
	| 'unknown'
	| 'processing'
	| 'unavailable';
export interface ContactMessageIntent {
	request_id: string;
	expected_email: string;
	subject: string;
	body_html: string;
	text_body?: string | null;
	template_id?: number;
}
export interface ContactMessageReceipt {
	contact_id: string;
	request_id: string;
	source_subject: string;
	source_hash: string;
	template_id: string | null;
	metrics_scope: 'contact_message';
	created_at: string;
	submissions: {
		id: string;
		recipient: string;
		subject: string;
		provider: string;
		provider_message_id: string | null;
		status: ContactMessageStatus;
		error: string | null;
		queued_at: string;
		accepted_at: string | null;
	}[];
}
export interface ContactEmailActivity {
	id: string;
	contact_id: string;
	request_id: string | null;
	source_subject: string | null;
	source_hash: string | null;
	template_id: string | null;
	recipient: string | null;
	receipt_id: string | null;
	subject: string;
	status: ContactMessageStatus | 'queued' | 'skipped';
	provider: string | null;
	provider_message_id: string | null;
	queued_at: string | null;
	sent_at: string | null;
	accepted_at: string | null;
	opened_at: string | null;
	clicked_at: string | null;
	bounced_at: string | null;
	error: string | null;
	campaign_name: string | null;
	scope: 'contact_message' | 'crm_outbox' | 'linked_member_email_log';
}

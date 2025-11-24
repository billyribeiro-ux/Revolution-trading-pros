/**
 * RevolutionCRM-L8-System - Type Definitions
 */

export type ContactStatus = 'lead' | 'prospect' | 'customer' | 'churned' | 'unqualified';
export type LifecycleStage =
	| 'subscriber'
	| 'lead'
	| 'mql'
	| 'sql'
	| 'opportunity'
	| 'customer'
	| 'evangelist';
export type ContactSource = 'website' | 'form' | 'import' | 'api' | 'manual' | 'referral' | 'event';
export type SubscriptionStatus = 'none' | 'trial' | 'active' | 'paused' | 'cancelled' | 'expired';
export type DealStatus = 'open' | 'won' | 'lost' | 'abandoned';
export type DealPriority = 'low' | 'normal' | 'high' | 'urgent';
export type ActivityType =
	| 'note'
	| 'email_sent'
	| 'email_opened'
	| 'email_clicked'
	| 'call'
	| 'meeting'
	| 'task_completed'
	| 'form_submitted'
	| 'popup_converted'
	| 'deal_created'
	| 'deal_stage_changed'
	| 'deal_won'
	| 'deal_lost'
	| 'subscription_started'
	| 'subscription_cancelled'
	| 'behavior_event'
	| 'page_view'
	| 'friction_detected'
	| 'attribution_touchpoint'
	| 'status_changed'
	| 'owner_changed'
	| 'custom';

export interface Contact {
	id: string;
	user_id?: string;
	email: string;
	first_name: string;
	last_name: string;
	full_name: string;
	phone?: string;
	mobile?: string;
	company_id?: string;
	job_title?: string;
	department?: string;
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
	source: ContactSource;
	source_details?: string;
	owner_id?: string;
	status: ContactStatus;
	lifecycle_stage: LifecycleStage;
	lead_score: number;
	health_score: number;
	engagement_score: number;
	value_score: number;
	is_verified: boolean;
	is_unsubscribed: boolean;
	do_not_contact: boolean;
	is_vip: boolean;
	custom_fields?: Record<string, any>;
	tags?: string[];
	first_touch_channel?: string;
	first_touch_campaign?: string;
	last_touch_channel?: string;
	last_touch_campaign?: string;
	total_sessions: number;
	last_session_at?: string;
	avg_engagement_score: number;
	avg_intent_score: number;
	friction_events_count: number;
	email_opens: number;
	email_clicks: number;
	last_email_opened_at?: string;
	last_email_clicked_at?: string;
	subscription_status: SubscriptionStatus;
	subscription_plan_id?: string;
	subscription_mrr: number;
	lifetime_value: number;
	last_activity_at?: string;
	last_contacted_at?: string;
	next_followup_at?: string;
	activities_count: number;
	notes_count: number;
	tasks_count: number;
	deals_count: number;
	created_at: string;
	updated_at: string;
	last_seen_at?: string;
	converted_at?: string;
	owner?: User;
}

export interface Deal {
	id: string;
	name: string;
	contact_id: string;
	company_id?: string;
	pipeline_id: string;
	stage_id: string;
	amount: number;
	currency: string;
	probability: number;
	weighted_value: number;
	owner_id: string;
	status: DealStatus;
	close_date?: string;
	expected_close_date: string;
	lost_reason?: string;
	won_details?: string;
	days_in_stage: number;
	days_in_pipeline: number;
	stage_changes_count: number;
	custom_fields?: Record<string, any>;
	tags?: string[];
	priority: DealPriority;
	source_channel?: string;
	source_campaign?: string;
	created_at: string;
	updated_at: string;
	stage_entered_at: string;
	closed_at?: string;
	contact?: Contact;
	pipeline?: Pipeline;
	stage?: Stage;
	owner?: User;
}

export interface Pipeline {
	id: string;
	name: string;
	description?: string;
	is_default: boolean;
	is_active: boolean;
	deals_count: number;
	total_value: number;
	win_rate: number;
	avg_deal_size: number;
	avg_sales_cycle: number;
	color: string;
	icon?: string;
	position: number;
	created_at: string;
	updated_at: string;
	stages?: Stage[];
}

export interface Stage {
	id: string;
	pipeline_id: string;
	name: string;
	description?: string;
	position: number;
	probability: number;
	is_closed_won: boolean;
	is_closed_lost: boolean;
	auto_advance_after_days?: number;
	required_activities?: string[];
	deals_count: number;
	total_value: number;
	avg_time_in_stage: number;
	conversion_rate: number;
	color: string;
	created_at: string;
	updated_at: string;
}

export interface Activity {
	id: string;
	subject_type: string;
	subject_id: string;
	type: ActivityType;
	title?: string;
	description?: string;
	metadata?: Record<string, any>;
	created_by_id?: string;
	assigned_to_id?: string;
	due_date?: string;
	completed_at?: string;
	priority?: DealPriority;
	occurred_at: string;
	created_at: string;
	updated_at: string;
	created_by?: User;
}

export interface Note {
	id: string;
	contact_id?: string;
	deal_id?: string;
	company_id?: string;
	content: string;
	is_pinned: boolean;
	created_by_id: string;
	created_at: string;
	updated_at: string;
	created_by?: User;
}

export interface ContactSegment {
	id: string;
	name: string;
	description?: string;
	conditions: SegmentCondition[];
	is_dynamic: boolean;
	contacts_count: number;
	last_calculated_at?: string;
	created_by_id: string;
	is_shared: boolean;
	created_at: string;
	updated_at: string;
}

export interface SegmentCondition {
	field: string;
	operator:
		| 'equals'
		| 'not_equals'
		| 'contains'
		| 'not_contains'
		| 'greater_than'
		| 'less_than'
		| 'between'
		| 'in'
		| 'not_in'
		| 'is_null'
		| 'is_not_null';
	value: any;
	logic?: 'and' | 'or';
}

export interface TimelineEvent {
	id: string;
	type: string;
	category: string;
	title: string;
	description?: string;
	metadata?: Record<string, any>;
	occurred_at: string;
	created_by?: {
		id: string;
		name: string;
	};
}

export interface DealForecast {
	period: string;
	commit: number;
	best_case: number;
	pipeline: number;
	worst_case: number;
	deals_count: number;
}

export interface User {
	id: string;
	name: string;
	email: string;
	avatar?: string;
}

export interface ContactFilters {
	status?: ContactStatus;
	lifecycle_stage?: LifecycleStage;
	owner_id?: string;
	search?: string;
	sort_by?: string;
	sort_order?: 'asc' | 'desc';
	per_page?: number;
}

export interface DealFilters {
	pipeline_id?: string;
	stage_id?: string;
	status?: DealStatus;
	owner_id?: string;
	per_page?: number;
}

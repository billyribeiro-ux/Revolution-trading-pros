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

// =====================================================
// FLUENTCRM PRO - EMAIL SEQUENCES (Drip Campaigns)
// =====================================================

export type SequenceStatus = 'draft' | 'active' | 'paused' | 'completed';
export type SequenceMailStatus = 'draft' | 'active' | 'paused';
export type DelayUnit = 'minutes' | 'hours' | 'days' | 'weeks';
export type TrackerStatus = 'active' | 'paused' | 'completed' | 'cancelled' | 'failed';

export interface EmailSequence {
	id: string;
	title: string;
	slug: string;
	status: SequenceStatus;
	description?: string;
	design_template: string;
	settings?: SequenceSettings;
	subscriber_settings?: SubscriberSettings;
	emails_count: number;
	subscribers_count: number;
	total_sent: number;
	total_opened: number;
	total_clicked: number;
	total_revenue: number;
	currency: string;
	created_by?: string;
	created_at: string;
	updated_at: string;
	deleted_at?: string;
	emails?: SequenceMail[];
	creator?: User;
}

export interface SequenceMail {
	id: string;
	sequence_id: string;
	title: string;
	email_subject: string;
	email_pre_header?: string;
	email_body: string;
	delay: number;
	delay_unit: DelayUnit;
	delay_value: number;
	status: SequenceMailStatus;
	settings?: MailerSettings;
	position: number;
	sent_count: number;
	open_count: number;
	click_count: number;
	unsubscribe_count: number;
	revenue: number;
	created_by?: string;
	created_at: string;
	updated_at: string;
	delay_for_humans?: string;
	open_rate?: number;
	click_rate?: number;
}

export interface SequenceTracker {
	id: string;
	contact_id: string;
	sequence_id: string;
	last_sequence_mail_id?: string;
	next_sequence_mail_id?: string;
	status: TrackerStatus;
	started_at: string;
	last_executed_at?: string;
	next_execution_at?: string;
	completed_at?: string;
	notes?: TrackerNote[];
	emails_sent: number;
	emails_opened: number;
	emails_clicked: number;
	created_at: string;
	updated_at: string;
	contact?: Contact;
	lastMail?: SequenceMail;
	nextMail?: SequenceMail;
	progress_percentage?: number;
}

export interface TrackerNote {
	action: string;
	data?: Record<string, any>;
	at: string;
}

export interface SequenceSettings {
	mailer_settings: MailerSettings;
}

export interface MailerSettings {
	from_name: string;
	from_email: string;
	reply_to_name: string;
	reply_to_email: string;
	is_custom: boolean;
}

export interface SubscriberSettings {
	subscribers?: SubscriberFilter[];
	excluded_subscribers?: SubscriberFilter[];
	sending_filter?: string;
	dynamic_segment?: string;
	advanced_filters?: SegmentCondition[];
}

export interface SubscriberFilter {
	list: string;
	tag: string;
}

export interface SequenceStats {
	emails: number;
	subscribers: number;
	active_subscribers: number;
	completed: number;
	total_sent: number;
	total_opened: number;
	total_clicked: number;
	open_rate: number;
	click_rate: number;
	revenue: {
		amount: string;
		currency: string;
	};
}

// =====================================================
// FLUENTCRM PRO - RECURRING CAMPAIGNS
// =====================================================

export type RecurringCampaignStatus = 'draft' | 'active' | 'paused';
export type RecurringMailStatus = 'draft' | 'scheduled' | 'sending' | 'sent' | 'failed';
export type SchedulingType = 'daily' | 'weekly' | 'monthly';

export interface RecurringCampaign {
	id: string;
	title: string;
	slug: string;
	status: RecurringCampaignStatus;
	email_subject?: string;
	email_pre_header?: string;
	email_body?: string;
	design_template: string;
	settings?: SequenceSettings;
	scheduling_settings?: SchedulingSettings;
	subscriber_settings?: SubscriberSettings;
	template_config?: Record<string, any>;
	labels?: string[];
	total_campaigns_sent: number;
	total_emails_sent: number;
	total_revenue: number;
	last_sent_at?: string;
	next_scheduled_at?: string;
	created_by?: string;
	created_at: string;
	updated_at: string;
	deleted_at?: string;
	emails?: RecurringMail[];
	creator?: User;
}

export interface SchedulingSettings {
	type: SchedulingType;
	day?: string; // 'mon', 'tue', etc. for weekly
	day_of_month?: number; // for monthly
	time: string; // 'HH:MM' format
	timezone: string;
	send_automatically: boolean;
}

export interface RecurringMail {
	id: string;
	recurring_campaign_id: string;
	email_subject: string;
	email_pre_header?: string;
	email_body: string;
	status: RecurringMailStatus;
	recipients_count: number;
	sent_count: number;
	failed_count: number;
	open_count: number;
	click_count: number;
	unsubscribe_count: number;
	revenue: number;
	scheduled_at?: string;
	sent_at?: string;
	created_at: string;
	updated_at: string;
	open_rate?: number;
	click_rate?: number;
}

export interface RecurringCampaignStats {
	total_campaigns_sent: number;
	total_emails_sent: number;
	total_revenue: number;
	avg_open_rate: number;
	avg_click_rate: number;
	last_sent_at?: string;
	next_scheduled_at?: string;
	last_email_stats?: {
		sent: number;
		opened: number;
		clicked: number;
	};
}

// =====================================================
// FLUENTCRM PRO - SMART LINKS
// =====================================================

export type SmartLinkActionType =
	| 'add_tag'
	| 'remove_tag'
	| 'add_to_list'
	| 'remove_from_list'
	| 'update_field'
	| 'add_to_sequence'
	| 'remove_from_sequence'
	| 'trigger_automation';

export interface SmartLink {
	id: string;
	title: string;
	short: string;
	target_url?: string;
	actions?: SmartLinkAction[];
	notes?: string;
	is_active: boolean;
	click_count: number;
	unique_clicks: number;
	click_data?: Record<string, any>;
	created_by?: string;
	created_at: string;
	updated_at: string;
	short_url?: string;
	creator?: User;
}

export interface SmartLinkAction {
	type: SmartLinkActionType;
	tag_id?: string;
	list_id?: string;
	sequence_id?: string;
	funnel_id?: string;
	field?: string;
	value?: any;
}

export interface SmartLinkClick {
	id: string;
	smart_link_id: string;
	contact_id?: string;
	ip_address?: string;
	user_agent?: string;
	referrer?: string;
	country?: string;
	city?: string;
	device?: string;
	browser?: string;
	os?: string;
	utm_params?: Record<string, string>;
	clicked_at: string;
	contact?: Contact;
}

export interface SmartLinkStats {
	total_clicks: number;
	unique_clicks: number;
	click_through_rate: number;
	clicks_today: number;
	clicks_this_week: number;
	top_countries: Record<string, number>;
}

export interface SmartLinkAnalytics {
	total_clicks: number;
	unique_clicks: number;
	clicks_by_day: Record<string, number>;
	clicks_by_country: Record<string, number>;
	clicks_by_device: Record<string, number>;
	clicks_by_browser: Record<string, number>;
}

// =====================================================
// FLUENTCRM PRO - AUTOMATION FUNNELS
// =====================================================

export type FunnelStatus = 'draft' | 'active' | 'paused';
export type FunnelSubscriberStatus = 'active' | 'waiting' | 'completed' | 'cancelled' | 'failed';
export type ConditionType = 'yes' | 'no' | 'none';

export type TriggerType =
	| 'contact_created'
	| 'tag_applied'
	| 'tag_removed'
	| 'list_applied'
	| 'list_removed'
	| 'contact_status_changed'
	| 'form_submitted'
	| 'order_completed'
	| 'order_refunded'
	| 'subscription_started'
	| 'subscription_cancelled'
	| 'user_login'
	| 'user_registered'
	| 'birthday'
	| 'sequence_completed'
	| 'link_clicked'
	| 'email_opened'
	| 'custom_event';

export type ActionType =
	| 'send_email'
	| 'send_campaign_email'
	| 'wait'
	| 'add_tag'
	| 'remove_tag'
	| 'add_to_list'
	| 'remove_from_list'
	| 'add_to_sequence'
	| 'remove_from_sequence'
	| 'update_contact'
	| 'change_status'
	| 'add_activity'
	| 'http_request'
	| 'create_user'
	| 'change_user_role'
	| 'remove_user_role'
	| 'update_user_meta'
	| 'condition'
	| 'ab_test'
	| 'goal'
	| 'end_funnel'
	| 'remove_from_funnel';

export interface AutomationFunnel {
	id: string;
	title: string;
	slug: string;
	description?: string;
	status: FunnelStatus;
	trigger_type: TriggerType;
	trigger_settings?: Record<string, any>;
	conditions?: SegmentCondition[];
	subscribers_count: number;
	completed_count: number;
	total_revenue: number;
	created_by?: string;
	created_at: string;
	updated_at: string;
	deleted_at?: string;
	actions?: FunnelAction[];
	creator?: User;
}

export interface FunnelAction {
	id: string;
	funnel_id: string;
	parent_id?: string;
	action_type: ActionType;
	title?: string;
	settings?: Record<string, any>;
	position: number;
	condition_type: ConditionType;
	delay_seconds: number;
	execution_count: number;
	created_at: string;
	updated_at: string;
	children?: FunnelAction[];
}

export interface FunnelSubscriber {
	id: string;
	funnel_id: string;
	contact_id: string;
	current_action_id?: string;
	status: FunnelSubscriberStatus;
	entered_at: string;
	next_execution_at?: string;
	completed_at?: string;
	execution_log?: ExecutionLogEntry[];
	actions_completed: number;
	created_at: string;
	updated_at: string;
	contact?: Contact;
	currentAction?: FunnelAction;
	progress_percentage?: number;
	duration_in_funnel?: string;
}

export interface ExecutionLogEntry {
	action_id?: string;
	action_type?: string;
	result: Record<string, any>;
	at: string;
}

export interface FunnelStats {
	total_subscribers: number;
	active_subscribers: number;
	waiting_subscribers: number;
	completed: number;
	cancelled: number;
	failed: number;
	completion_rate: number;
	total_revenue: number;
}

// =====================================================
// FLUENTCRM PRO - CONTACT LISTS
// =====================================================

export type ListMemberStatus = 'subscribed' | 'unsubscribed' | 'pending';

export interface ContactList {
	id: string;
	title: string;
	slug: string;
	description?: string;
	is_public: boolean;
	contacts_count: number;
	created_by?: string;
	created_at: string;
	updated_at: string;
	creator?: User;
}

export interface ListMember {
	list_id: string;
	contact_id: string;
	status: ListMemberStatus;
	subscribed_at: string;
	unsubscribed_at?: string;
}

// =====================================================
// FLUENTCRM PRO - CONTACT TAGS
// =====================================================

export interface ContactTag {
	id: string;
	title: string;
	slug: string;
	description?: string;
	color: string;
	contacts_count: number;
	created_by?: string;
	created_at: string;
	updated_at: string;
	creator?: User;
}

export interface TagPivot {
	tag_id: string;
	contact_id: string;
	applied_at: string;
	applied_by?: string;
}

// =====================================================
// FLUENTCRM PRO - CRM COMPANIES (B2B)
// =====================================================

export type CompanySize =
	| '1-10'
	| '11-50'
	| '51-200'
	| '201-500'
	| '501-1000'
	| '1001-5000'
	| '5001+';
export type CompanyIndustry =
	| 'technology'
	| 'finance'
	| 'healthcare'
	| 'education'
	| 'retail'
	| 'manufacturing'
	| 'real_estate'
	| 'consulting'
	| 'marketing'
	| 'media'
	| 'nonprofit'
	| 'government'
	| 'other';

export interface CrmCompany {
	id: string;
	name: string;
	slug: string;
	website?: string;
	industry?: CompanyIndustry;
	size?: CompanySize;
	annual_revenue?: number;
	phone?: string;
	email?: string;
	description?: string;
	address_line1?: string;
	address_line2?: string;
	city?: string;
	state?: string;
	postal_code?: string;
	country?: string;
	linkedin_url?: string;
	twitter_handle?: string;
	logo_url?: string;
	custom_fields?: Record<string, any>;
	owner_id?: string;
	contacts_count: number;
	deals_count: number;
	total_deal_value: number;
	created_at: string;
	updated_at: string;
	deleted_at?: string;
	owner?: User;
	contacts?: Contact[];
	deals?: Deal[];
}

export interface CompanyStats {
	contacts_count: number;
	deals_count: number;
	total_won_value: number;
	open_pipeline_value: number;
	win_rate: number;
}

// =====================================================
// FLUENTCRM PRO - FILTERS
// =====================================================

export interface SequenceFilters {
	status?: SequenceStatus;
	search?: string;
	sort_by?: string;
	sort_order?: 'asc' | 'desc';
	per_page?: number;
}

export interface RecurringCampaignFilters {
	status?: RecurringCampaignStatus;
	search?: string;
	sort_by?: string;
	sort_order?: 'asc' | 'desc';
	per_page?: number;
}

export interface SmartLinkFilters {
	search?: string;
	is_active?: boolean;
	sort_by?: string;
	sort_order?: 'asc' | 'desc';
	per_page?: number;
}

export interface FunnelFilters {
	status?: FunnelStatus;
	trigger_type?: TriggerType;
	search?: string;
	sort_by?: string;
	sort_order?: 'asc' | 'desc';
	per_page?: number;
}

export interface CompanyFilters {
	search?: string;
	industry?: CompanyIndustry;
	size?: CompanySize;
	owner_id?: string;
	sort_by?: string;
	sort_order?: 'asc' | 'desc';
	per_page?: number;
}

// =====================================================
// FLUENTCRM PRO - ABANDONED CARTS
// =====================================================

export type AbandonedCartStatus = 'draft' | 'processing' | 'recovered' | 'lost' | 'opt_out';

export interface AbandonedCart {
	id: string;
	checkout_key: string;
	cart_hash?: string;
	contact_id?: string;
	is_optout: boolean;
	full_name?: string;
	email: string;
	provider: string;
	user_id?: string;
	order_id?: string;
	automation_id?: string;
	checkout_page_id?: string;
	status: AbandonedCartStatus;
	subtotal: number;
	shipping: number;
	discounts: number;
	fees: number;
	tax: number;
	total: number;
	currency: string;
	cart?: CartContents;
	note?: string;
	recovered_at?: string;
	abandoned_at?: string;
	click_counts: number;
	created_at: string;
	updated_at: string;
	customer_avatar?: string;
	order_url?: string;
	recovery_url?: string;
	contact?: Contact;
	automation?: AutomationFunnel;
}

export interface CartContents {
	cart_contents: CartItem[];
	customer_data?: CustomerData;
}

export interface CartItem {
	product_id: string;
	product_name: string;
	quantity: number;
	line_total: number;
	line_tax: number;
	product_image?: string;
	variation_id?: string;
	variation_data?: Record<string, string>;
}

export interface CustomerData {
	differentShipping?: string;
	billingAddress?: CartAddress;
	shippingAddress?: CartAddress;
}

export interface CartAddress {
	address_1?: string;
	address_2?: string;
	city?: string;
	state?: string;
	postcode?: string;
	country?: string;
}

export interface AbandonedCartSettings {
	enabled: string;
	capture_after_minutes: number;
	lost_cart_days: number;
	cool_off_period_days: number;
	gdpr_consent: string;
	gdpr_consent_text: string;
	disabled_user_roles: string[];
	track_add_to_cart: string;
	add_to_cart_exclude_user_roles: string[];
	tags_on_cart_abandoned: string[];
	lists_on_cart_abandoned: string[];
	tags_on_cart_lost: string[];
	lists_on_cart_lost: string[];
	new_contact_status: string;
	wc_recovered_statuses: string[];
}

export interface AbandonedCartStats {
	recovered_revenue: StatWidget;
	processing_revenue: StatWidget;
	lost_revenue: StatWidget;
	draft_revenue: StatWidget;
	optout_revenue: StatWidget;
	recovery_rate: StatWidget;
}

export interface StatWidget {
	title: string;
	value: string;
	count: string;
}

export interface AbandonedCartFilters {
	status?: AbandonedCartStatus;
	search?: string;
	date_range?: string[];
	per_page?: number;
}

// =====================================================
// FLUENTCRM PRO - EMAIL TEMPLATES
// =====================================================

export type TemplateType = 'raw' | 'visual';

export interface EmailTemplate {
	id: string;
	title: string;
	slug: string;
	subject?: string;
	content: string;
	design_template: TemplateType;
	template_config?: Record<string, any>;
	is_default: boolean;
	category?: string;
	thumbnail?: string;
	created_by?: string;
	created_at: string;
	updated_at: string;
	creator?: User;
}

export interface TemplateCategory {
	id: string;
	name: string;
	slug: string;
	templates_count: number;
}

// =====================================================
// FLUENTCRM PRO - WEBHOOKS
// =====================================================

export type WebhookEvent =
	| 'contact_created'
	| 'contact_updated'
	| 'contact_deleted'
	| 'tag_applied'
	| 'tag_removed'
	| 'list_applied'
	| 'list_removed'
	| 'email_opened'
	| 'email_clicked'
	| 'contact_unsubscribed'
	| 'deal_created'
	| 'deal_won'
	| 'deal_lost';

export interface Webhook {
	id: string;
	name: string;
	url: string;
	events: WebhookEvent[];
	secret?: string;
	is_active: boolean;
	headers?: Record<string, string>;
	last_triggered_at?: string;
	trigger_count: number;
	failure_count: number;
	created_by?: string;
	created_at: string;
	updated_at: string;
	creator?: User;
}

export interface WebhookLog {
	id: string;
	webhook_id: string;
	event: WebhookEvent;
	payload: Record<string, any>;
	response_code?: number;
	response_body?: string;
	is_success: boolean;
	error_message?: string;
	triggered_at: string;
}

// =====================================================
// FLUENTCRM PRO - IMPORT/EXPORT
// =====================================================

export type ImportType =
	| 'contacts'
	| 'tags'
	| 'lists'
	| 'sequences'
	| 'campaigns'
	| 'automations'
	| 'templates';
export type ImportStatus = 'pending' | 'processing' | 'completed' | 'failed';

export interface ImportJob {
	id: string;
	type: ImportType;
	file_name: string;
	file_path?: string;
	total_rows: number;
	processed_rows: number;
	success_count: number;
	error_count: number;
	status: ImportStatus;
	field_mapping?: Record<string, string>;
	settings?: ImportSettings;
	errors?: ImportError[];
	created_by?: string;
	started_at?: string;
	completed_at?: string;
	created_at: string;
	updated_at: string;
}

export interface ImportSettings {
	update_existing: boolean;
	skip_duplicates: boolean;
	add_to_lists?: string[];
	apply_tags?: string[];
	contact_status?: string;
}

export interface ImportError {
	row: number;
	field?: string;
	message: string;
}

export interface ExportJob {
	id: string;
	type: ImportType;
	filters?: Record<string, any>;
	fields: string[];
	total_records: number;
	status: ImportStatus;
	file_url?: string;
	created_by?: string;
	started_at?: string;
	completed_at?: string;
	created_at: string;
	updated_at: string;
}

// =====================================================
// FLUENTCRM PRO - MANAGER PERMISSIONS
// =====================================================

export interface ManagerRole {
	id: string;
	name: string;
	slug: string;
	description?: string;
	permissions: ManagerPermission[];
	users_count: number;
	is_default: boolean;
	created_at: string;
	updated_at: string;
}

export interface ManagerPermission {
	module: string;
	action: 'view' | 'create' | 'edit' | 'delete' | 'manage';
	allowed: boolean;
}

export interface ManagerUser {
	id: string;
	user_id: string;
	role_id: string;
	name: string;
	email: string;
	avatar?: string;
	assigned_at: string;
	role?: ManagerRole;
}

export interface PermissionModule {
	id: string;
	name: string;
	actions: string[];
}

// =====================================================
// FLUENTCRM PRO - DOUBLE OPT-IN
// =====================================================

export interface DoubleOptInSettings {
	enabled: boolean;
	email_subject: string;
	email_body: string;
	confirmation_page_url?: string;
	redirect_url?: string;
	after_confirmation_status: string;
	apply_tags_on_confirm?: string[];
	add_to_lists_on_confirm?: string[];
}

// =====================================================
// FLUENTCRM PRO - EMAIL PREFERENCES
// =====================================================

export interface EmailPreferencePage {
	enabled: boolean;
	title: string;
	intro_text: string;
	show_lists: boolean;
	show_tags: boolean;
	show_communication_types: boolean;
	show_unsubscribe_all: boolean;
	custom_css?: string;
	redirect_after_update?: string;
}

export interface CommunicationType {
	id: string;
	name: string;
	description?: string;
	is_active: boolean;
}

// =====================================================
// FLUENTCRM PRO - SYSTEM LOGS
// =====================================================

export type LogLevel = 'debug' | 'info' | 'warning' | 'error' | 'critical';
export type LogCategory = 'email' | 'automation' | 'import' | 'api' | 'webhook' | 'system';

export interface SystemLog {
	id: string;
	level: LogLevel;
	category: LogCategory;
	message: string;
	context?: Record<string, any>;
	contact_id?: string;
	user_id?: string;
	ip_address?: string;
	created_at: string;
	contact?: Contact;
}

export interface SystemLogFilters {
	level?: LogLevel;
	category?: LogCategory;
	search?: string;
	date_from?: string;
	date_to?: string;
	per_page?: number;
}

// =====================================================
// FLUENTCRM PRO - ONE-TIME CAMPAIGNS
// =====================================================

export type CampaignStatus = 'draft' | 'scheduled' | 'sending' | 'sent' | 'paused' | 'archived';
export type CampaignType = 'regular' | 'plain_text' | 'raw_html';

export interface Campaign {
	id: string;
	title: string;
	slug: string;
	status: CampaignStatus;
	type: CampaignType;
	email_subject: string;
	email_pre_header?: string;
	email_body: string;
	design_template: string;
	template_config?: Record<string, any>;
	settings?: SequenceSettings;
	subscriber_settings?: SubscriberSettings;
	utm_settings?: UtmSettings;
	scheduled_at?: string;
	sent_at?: string;
	recipients_count: number;
	sent_count: number;
	failed_count: number;
	open_count: number;
	click_count: number;
	unsubscribe_count: number;
	bounce_count: number;
	complaint_count: number;
	revenue: number;
	created_by?: string;
	created_at: string;
	updated_at: string;
	deleted_at?: string;
	creator?: User;
	open_rate?: number;
	click_rate?: number;
}

export interface UtmSettings {
	utm_source?: string;
	utm_medium?: string;
	utm_campaign?: string;
	utm_term?: string;
	utm_content?: string;
}

export interface CampaignStats {
	total: number;
	sent: number;
	scheduled: number;
	draft: number;
	total_emails_sent: number;
	total_opens: number;
	total_clicks: number;
	avg_open_rate: number;
	avg_click_rate: number;
}

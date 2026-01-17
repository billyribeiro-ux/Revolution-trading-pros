/**
 * Fluent Boards Pro - TypeScript Type Definitions
 * Complete project management system types
 */

// =====================================================
// BOARD TYPES
// =====================================================

export interface Board {
	id: string;
	title: string;
	description?: string;
	slug: string;
	type: BoardType;
	background_color?: string;
	background_image?: string;
	is_archived: boolean;
	is_favorite: boolean;
	is_public: boolean;
	folder_id?: string;
	settings: BoardSettings;
	permissions: BoardPermissions;
	created_by: string;
	created_at: string;
	updated_at: string;

	// Relations
	stages?: Stage[];
	members?: BoardMember[];
	labels?: Label[];
	custom_fields?: CustomFieldDefinition[];
	folder?: Folder;

	// Stats
	task_count?: number;
	completed_task_count?: number;
	member_count?: number;
}

export type BoardType = 'kanban' | 'list' | 'calendar' | 'timeline' | 'table';

export interface BoardSettings {
	default_stage_id?: string;
	allow_attachments: boolean;
	allow_comments: boolean;
	allow_subtasks: boolean;
	allow_time_tracking: boolean;
	require_due_dates: boolean;
	auto_archive_completed: boolean;
	auto_archive_days?: number;
	notification_settings: NotificationSettings;
}

export interface NotificationSettings {
	task_created: boolean;
	task_completed: boolean;
	task_assigned: boolean;
	task_due_soon: boolean;
	comment_added: boolean;
	mention: boolean;
}

export interface BoardPermissions {
	can_edit: boolean;
	can_delete: boolean;
	can_invite: boolean;
	can_manage_stages: boolean;
	can_manage_labels: boolean;
	can_manage_fields: boolean;
}

// =====================================================
// STAGE/COLUMN TYPES
// =====================================================

export interface Stage {
	id: string;
	board_id: string;
	title: string;
	color: string;
	position: number;
	is_collapsed: boolean;
	wip_limit?: number; // Work in progress limit
	auto_complete: boolean; // Auto-mark tasks as complete when moved here
	created_at: string;
	updated_at: string;

	// Relations
	tasks?: Task[];
	task_count?: number;
}

// =====================================================
// TASK TYPES
// =====================================================

export interface Task {
	id: string;
	board_id: string;
	stage_id: string;
	parent_id?: string; // For subtasks
	title: string;
	description?: string;
	position: number;
	priority: TaskPriority;
	status: TaskStatus;

	// Dates
	start_date?: string;
	due_date?: string;
	completed_at?: string;

	// Time tracking
	estimated_minutes?: number;
	logged_minutes?: number;

	// Assignment
	assignees: string[];
	watchers: string[];
	created_by: string;

	// Metadata
	cover_image?: string;
	is_archived: boolean;
	created_at: string;
	updated_at: string;

	// Relations
	labels?: Label[];
	attachments?: Attachment[];
	comments?: Comment[];
	subtasks?: Task[];
	checklists?: Checklist[];
	custom_field_values?: CustomFieldValue[];
	time_entries?: TimeEntry[];
	activities?: Activity[];

	// Stats
	subtask_count?: number;
	completed_subtask_count?: number;
	comment_count?: number;
	attachment_count?: number;
}

export type TaskPriority = 'urgent' | 'high' | 'medium' | 'low' | 'none';

export type TaskStatus = 'open' | 'in_progress' | 'completed' | 'blocked' | 'archived';

// =====================================================
// SUBTASK TYPES
// =====================================================

export interface Subtask {
	id: string;
	task_id: string;
	title: string;
	is_completed: boolean;
	position: number;
	assignee_id?: string;
	due_date?: string;
	created_by: string;
	created_at: string;
	updated_at: string;
	completed_at?: string;
}

// =====================================================
// CHECKLIST TYPES
// =====================================================

export interface Checklist {
	id: string;
	task_id: string;
	title: string;
	position: number;
	items: ChecklistItem[];
	created_at: string;
	updated_at: string;
}

export interface ChecklistItem {
	id: string;
	checklist_id: string;
	title: string;
	is_completed: boolean;
	position: number;
	assignee_id?: string;
	due_date?: string;
	completed_at?: string;
}

// =====================================================
// LABEL TYPES
// =====================================================

export interface Label {
	id: string;
	board_id: string;
	title: string;
	color: string;
	description?: string;
	created_at: string;
	updated_at: string;
}

// =====================================================
// CUSTOM FIELD TYPES
// =====================================================

export interface CustomFieldDefinition {
	id: string;
	board_id: string;
	name: string;
	type: CustomFieldType;
	options?: CustomFieldOption[];
	is_required: boolean;
	show_on_card: boolean;
	position: number;
	created_at: string;
	updated_at: string;
}

export type CustomFieldType =
	| 'text'
	| 'number'
	| 'date'
	| 'datetime'
	| 'select'
	| 'multiselect'
	| 'checkbox'
	| 'url'
	| 'email'
	| 'phone'
	| 'currency'
	| 'percentage'
	| 'rating'
	| 'user'
	| 'file';

export interface CustomFieldOption {
	id: string;
	label: string;
	color?: string;
}

export interface CustomFieldValue {
	id: string;
	task_id: string;
	field_id: string;
	value: any;
	created_at: string;
	updated_at: string;

	// Relations
	field?: CustomFieldDefinition;
}

// =====================================================
// ATTACHMENT TYPES
// =====================================================

export interface Attachment {
	id: string;
	task_id: string;
	filename: string;
	original_filename: string;
	file_type: string;
	file_size: number;
	url: string;
	thumbnail_url?: string;
	storage_driver: StorageDriver;
	uploaded_by: string;
	created_at: string;

	// Relations
	uploader?: BoardMember;
}

export type StorageDriver = 'local' | 's3' | 'r2' | 'digitalocean' | 'backblaze';

// =====================================================
// COMMENT TYPES
// =====================================================

export interface Comment {
	id: string;
	task_id: string;
	parent_id?: string;
	content: string;
	mentions: string[];
	attachments?: Attachment[];
	reactions?: Reaction[];
	is_edited: boolean;
	created_by: string;
	created_at: string;
	updated_at: string;

	// Relations
	author?: BoardMember;
	replies?: Comment[];
}

export interface Reaction {
	emoji: string;
	user_ids: string[];
	count: number;
}

// =====================================================
// TIME TRACKING TYPES
// =====================================================

export interface TimeEntry {
	id: string;
	task_id: string;
	user_id: string;
	description?: string;
	minutes: number;
	started_at?: string;
	ended_at?: string;
	is_billable: boolean;
	hourly_rate?: number;
	created_at: string;
	updated_at: string;

	// Relations
	user?: BoardMember;
	task?: Task;
}

export interface TimeTrackingStats {
	total_minutes: number;
	billable_minutes: number;
	non_billable_minutes: number;
	total_cost: number;
	by_user: UserTimeStats[];
	by_task: TaskTimeStats[];
	by_date: DateTimeStats[];
}

export interface UserTimeStats {
	user_id: string;
	user_name: string;
	total_minutes: number;
	billable_minutes: number;
	cost: number;
}

export interface TaskTimeStats {
	task_id: string;
	task_title: string;
	total_minutes: number;
	estimated_minutes: number;
	variance: number;
}

export interface DateTimeStats {
	date: string;
	total_minutes: number;
	billable_minutes: number;
}

// =====================================================
// BOARD MEMBER TYPES
// =====================================================

export interface BoardMember {
	id: string;
	user_id: string;
	board_id: string;
	role: BoardRole;
	joined_at: string;

	// User details
	name: string;
	email: string;
	avatar?: string;
}

export type BoardRole = 'owner' | 'admin' | 'member' | 'viewer';

// =====================================================
// FOLDER TYPES
// =====================================================

export interface Folder {
	id: string;
	title: string;
	description?: string;
	color?: string;
	parent_id?: string;
	position: number;
	created_by: string;
	created_at: string;
	updated_at: string;

	// Relations
	boards?: Board[];
	subfolders?: Folder[];
	board_count?: number;
}

// =====================================================
// ACTIVITY TYPES
// =====================================================

export interface Activity {
	id: string;
	board_id: string;
	task_id?: string;
	user_id: string;
	type: ActivityType;
	description: string;
	changes?: ActivityChange[];
	created_at: string;

	// Relations
	user?: BoardMember;
	task?: Task;
}

export type ActivityType =
	| 'task_created'
	| 'task_updated'
	| 'task_moved'
	| 'task_completed'
	| 'task_archived'
	| 'task_deleted'
	| 'task_assigned'
	| 'task_unassigned'
	| 'comment_added'
	| 'attachment_added'
	| 'attachment_removed'
	| 'label_added'
	| 'label_removed'
	| 'due_date_set'
	| 'due_date_removed'
	| 'checklist_added'
	| 'checklist_completed'
	| 'time_logged'
	| 'subtask_added'
	| 'subtask_completed'
	| 'member_added'
	| 'member_removed';

export interface ActivityChange {
	field: string;
	old_value: any;
	new_value: any;
}

// =====================================================
// IMPORT TYPES
// =====================================================

export interface ImportJob {
	id: string;
	source: ImportSource;
	status: ImportStatus;
	file_name?: string;
	total_items: number;
	processed_items: number;
	failed_items: number;
	errors?: ImportError[];
	mapping?: ImportMapping;
	created_by: string;
	created_at: string;
	completed_at?: string;
}

export type ImportSource = 'trello' | 'asana' | 'csv' | 'json' | 'fluent_boards';

export type ImportStatus =
	| 'pending'
	| 'mapping'
	| 'processing'
	| 'completed'
	| 'failed'
	| 'cancelled';

export interface ImportError {
	row?: number;
	item?: string;
	field?: string;
	message: string;
}

export interface ImportMapping {
	board_title?: string;
	stage_mapping?: Record<string, string>;
	field_mapping?: Record<string, string>;
	label_mapping?: Record<string, string>;
	user_mapping?: Record<string, string>;
}

export interface TrelloBoard {
	id: string;
	name: string;
	lists: TrelloList[];
	cards: TrelloCard[];
	labels: TrelloLabel[];
}

export interface TrelloList {
	id: string;
	name: string;
	pos: number;
}

export interface TrelloCard {
	id: string;
	name: string;
	desc: string;
	idList: string;
	due?: string;
	labels: TrelloLabel[];
}

export interface TrelloLabel {
	id: string;
	name: string;
	color: string;
}

// =====================================================
// WEBHOOK TYPES
// =====================================================

export interface BoardWebhook {
	id: string;
	board_id: string;
	name: string;
	url: string;
	secret?: string;
	events: WebhookEvent[];
	is_active: boolean;
	last_triggered_at?: string;
	failure_count: number;
	created_at: string;
	updated_at: string;
}

export type WebhookEvent =
	| 'task.created'
	| 'task.updated'
	| 'task.deleted'
	| 'task.moved'
	| 'task.completed'
	| 'task.assigned'
	| 'comment.created'
	| 'attachment.uploaded'
	| 'member.added'
	| 'member.removed';

export interface WebhookPayload {
	event: WebhookEvent;
	board_id: string;
	task_id?: string;
	data: any;
	timestamp: string;
}

// =====================================================
// REPORT TYPES
// =====================================================

export interface BoardReport {
	board_id: string;
	period: ReportPeriod;
	generated_at: string;

	// Overview
	total_tasks: number;
	completed_tasks: number;
	overdue_tasks: number;
	completion_rate: number;

	// By stage
	tasks_by_stage: StageStats[];

	// By priority
	tasks_by_priority: PriorityStats[];

	// By assignee
	tasks_by_assignee: AssigneeStats[];

	// By label
	tasks_by_label: LabelStats[];

	// Time metrics
	avg_completion_time: number;
	total_time_logged: number;

	// Trends
	daily_created: DailyStats[];
	daily_completed: DailyStats[];

	// Velocity
	velocity: VelocityStats;
}

export type ReportPeriod = 'week' | 'month' | 'quarter' | 'year' | 'all_time' | 'custom';

export interface StageStats {
	stage_id: string;
	stage_title: string;
	task_count: number;
	percentage: number;
}

export interface PriorityStats {
	priority: TaskPriority;
	task_count: number;
	percentage: number;
}

export interface AssigneeStats {
	user_id: string;
	user_name: string;
	assigned_count: number;
	completed_count: number;
	completion_rate: number;
	avg_completion_time: number;
}

export interface LabelStats {
	label_id: string;
	label_title: string;
	label_color: string;
	task_count: number;
}

export interface DailyStats {
	date: string;
	count: number;
}

export interface VelocityStats {
	current_period: number;
	previous_period: number;
	change_percentage: number;
	trend: 'up' | 'down' | 'stable';
}

// =====================================================
// FILTER TYPES
// =====================================================

export interface BoardFilters {
	search?: string;
	folder_id?: string;
	is_archived?: boolean;
	is_favorite?: boolean;
	type?: BoardType;
	created_by?: string;
	per_page?: number;
	page?: number;
}

export interface TaskFilters {
	search?: string;
	stage_id?: string;
	assignee_ids?: string[];
	label_ids?: string[];
	priority?: TaskPriority;
	status?: TaskStatus;
	due_date_from?: string;
	due_date_to?: string;
	is_overdue?: boolean;
	has_due_date?: boolean;
	is_archived?: boolean;
	created_by?: string;
	per_page?: number;
	page?: number;
	sort_by?: 'position' | 'due_date' | 'priority' | 'created_at' | 'updated_at';
	sort_order?: 'asc' | 'desc';
}

export interface TimeEntryFilters {
	user_id?: string;
	task_id?: string;
	board_id?: string;
	is_billable?: boolean;
	date_from?: string;
	date_to?: string;
	per_page?: number;
	page?: number;
}

export interface ActivityFilters {
	board_id?: string;
	task_id?: string;
	user_id?: string;
	type?: ActivityType;
	date_from?: string;
	date_to?: string;
	per_page?: number;
	page?: number;
}

// =====================================================
// CLOUD STORAGE TYPES
// =====================================================

export interface StorageConfig {
	driver: StorageDriver;
	bucket: string;
	region?: string;
	endpoint?: string;
	access_key?: string;
	secret_key?: string;
	public_url?: string;
}

export interface StorageStats {
	total_files: number;
	total_size: number;
	by_type: FileTypeStats[];
}

export interface FileTypeStats {
	type: string;
	count: number;
	size: number;
}

// =====================================================
// SETTINGS TYPES
// =====================================================

export interface BoardsSettings {
	default_board_type: BoardType;
	default_stages: DefaultStage[];
	default_labels: DefaultLabel[];
	storage_config?: StorageConfig;
	time_tracking_enabled: boolean;
	default_hourly_rate?: number;
	working_hours_per_day: number;
	working_days_per_week: number;
	webhooks_enabled: boolean;
	integrations: BoardIntegration[];
}

export interface DefaultStage {
	title: string;
	color: string;
	auto_complete: boolean;
}

export interface DefaultLabel {
	title: string;
	color: string;
}

export interface BoardIntegration {
	type: 'slack' | 'discord' | 'teams' | 'email' | 'fluentcrm';
	is_enabled: boolean;
	config: Record<string, any>;
}

// =====================================================
// TEMPLATE TYPES
// =====================================================

export interface BoardTemplate {
	id: string;
	title: string;
	description: string;
	category: string;
	thumbnail?: string;
	stages: TemplateStage[];
	labels: TemplateLabel[];
	custom_fields: CustomFieldDefinition[];
	sample_tasks?: TemplateTask[];
	is_default: boolean;
	created_at: string;
}

export interface TemplateStage {
	title: string;
	color: string;
	position: number;
	auto_complete: boolean;
}

export interface TemplateLabel {
	title: string;
	color: string;
}

export interface TemplateTask {
	title: string;
	description?: string;
	stage_index: number;
	priority: TaskPriority;
	labels?: number[];
	checklist?: string[];
}

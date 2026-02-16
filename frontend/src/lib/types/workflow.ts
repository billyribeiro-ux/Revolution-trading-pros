export type NodeType =
	| 'trigger'
	| 'condition'
	| 'action'
	| 'delay'
	| 'branch'
	| 'parallel'
	| 'merge'
	| 'end';

export type WorkflowStatus = 'active' | 'paused' | 'archived';

export type RunStatus = 'running' | 'completed' | 'failed' | 'paused' | 'cancelled';

export type ConditionType = 'always' | 'if_true' | 'if_false' | 'parallel';

export interface Workflow {
	id: number;
	user_id: string;
	name: string;
	description?: string;
	status: WorkflowStatus;
	trigger_config: TriggerConfig;
	version: number;
	execution_count: number;
	success_count: number;
	failure_count: number;
	last_executed_at?: string;
	avg_execution_time_ms?: number;
	created_at: string;
	updated_at: string;
}

export interface WorkflowNode {
	id: number;
	workflow_id: number;
	node_type: NodeType;
	config: Record<string, any>;
	position_x: number;
	position_y: number;
	parent_node_id?: number;
	order: number;
	created_at: string;
	updated_at: string;
}

export interface WorkflowEdge {
	id: number;
	workflow_id: number;
	from_node_id: number;
	to_node_id: number;
	condition_type: ConditionType;
	label?: string;
	created_at: string;
	updated_at: string;
}

export interface WorkflowRun {
	id: number;
	workflow_id: number;
	trigger_event_id?: string;
	status: RunStatus;
	started_at: string;
	completed_at?: string;
	duration_ms?: number;
	context: Record<string, any>;
	error_message?: string;
	triggered_by_user_id?: string;
}

export interface WorkflowRunLog {
	id: number;
	workflow_run_id: number;
	node_id: number;
	status: 'pending' | 'running' | 'completed' | 'failed' | 'skipped';
	executed_at: string;
	input_data?: Record<string, any>;
	output_data?: Record<string, any>;
	error?: string;
	duration_ms?: number;
}

export interface TriggerConfig {
	type: string;
	conditions?: ConditionRule[];
	[key: string]: any;
}

export interface ConditionRule {
	field: string;
	operator: string;
	value: any;
}

export interface ActionConfig {
	action_type: string;
	[key: string]: any;
}

// UI-specific types
export interface NodePosition {
	x: number;
	y: number;
}

export interface DraggedNode {
	type: NodeType;
	config?: Record<string, any>;
}

export interface WorkflowCanvas {
	nodes: WorkflowNode[];
	edges: WorkflowEdge[];
	selectedNode?: WorkflowNode;
	selectedEdge?: WorkflowEdge;
	zoom: number;
	pan: NodePosition;
}

// ═══════════════════════════════════════════════════════════════════════════
// Workflow Configuration Types - ICT 7+ Strict Type Safety
// ═══════════════════════════════════════════════════════════════════════════

/** Base configuration for workflow triggers and actions */
export type WorkflowConfig = Record<string, string | number | boolean | null | undefined>;

export interface TriggerTemplate {
	type: string;
	name: string;
	description: string;
	icon: string;
	category:
		| 'contact'
		| 'form'
		| 'email'
		| 'funnel'
		| 'popup'
		| 'behavior'
		| 'crm'
		| 'subscription'
		| 'ecommerce'
		| 'trading'
		| 'system';
	/** Configuration object with string, number, boolean, or null values */
	config: WorkflowConfig;
}

export interface ActionTemplate {
	type: string;
	name: string;
	description: string;
	icon: string;
	category:
		| 'contact'
		| 'email'
		| 'crm'
		| 'notification'
		| 'funnel'
		| 'popup'
		| 'subscription'
		| 'integration'
		| 'flow'
		| 'ai';
	/** Configuration object with string, number, boolean, or null values */
	config: WorkflowConfig;
	requiredFields: string[];
}

export interface WorkflowAnalytics {
	workflow_id: number;
	total_runs: number;
	successful_runs: number;
	failed_runs: number;
	success_rate: number;
	avg_duration_ms: number;
	runs_by_day: Array<{ date: string; count: number }>;
	failure_reasons: Array<{ reason: string; count: number }>;
}

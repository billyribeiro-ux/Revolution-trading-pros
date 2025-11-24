/**
 * RevolutionBehavior-L8-System - Type Definitions
 * Enterprise Behavioral Analytics Engine
 */

// ═══════════════════════════════════════════════════════════════════════════
// EVENT TYPES
// ═══════════════════════════════════════════════════════════════════════════

export type BehaviorEventType =
	// Navigation
	| 'page_view'
	| 'page_exit'
	| 'navigation_click'
	| 'back_button'
	// Scroll
	| 'scroll_depth'
	| 'speed_scroll'
	| 'scroll_backtrack'
	| 'scroll_pause'
	// Click
	| 'click'
	| 'rage_click'
	| 'dead_click'
	| 'cta_click'
	| 'cta_hesitation'
	// Hover & Cursor
	| 'hover_intent'
	| 'cursor_thrashing'
	| 'cursor_idle'
	| 'exit_intent'
	// Form
	| 'form_focus'
	| 'form_input'
	| 'form_blur'
	| 'form_abandon'
	| 'form_submit'
	| 'form_error'
	// Engagement
	| 'idle_start'
	| 'idle_end'
	| 'tab_blur'
	| 'tab_focus'
	| 'copy_text'
	| 'video_play'
	| 'video_pause'
	// Friction
	| 'friction_detected'
	| 'dead_zone_hover'
	| 'unexpected_scroll'
	| 'repeated_action';

// ═══════════════════════════════════════════════════════════════════════════
// EVENT INTERFACES
// ═══════════════════════════════════════════════════════════════════════════

export interface BehaviorEvent {
	event_type: BehaviorEventType;
	timestamp: number;
	page_url: string;
	element?: string;
	element_selector?: string;
	coordinates?: { x: number; y: number };
	event_value?: any;
	event_metadata?: Record<string, any>;
}

export interface PageViewEvent extends BehaviorEvent {
	event_type: 'page_view';
	event_metadata: {
		referrer: string;
		entry_type: 'direct' | 'referral' | 'search' | 'social' | 'campaign';
		viewport: { width: number; height: number };
		utm_source?: string;
		utm_campaign?: string;
		utm_medium?: string;
	};
}

export interface ScrollDepthEvent extends BehaviorEvent {
	event_type: 'scroll_depth';
	event_value: number; // percentage
	event_metadata: {
		depth_percent: number;
		time_to_depth: number;
		scroll_speed: number;
	};
}

export interface RageClickEvent extends BehaviorEvent {
	event_type: 'rage_click';
	event_metadata: {
		click_count: number;
		time_window: number;
		frustration_score: number;
		element_type: string;
	};
}

export interface FormAbandonEvent extends BehaviorEvent {
	event_type: 'form_abandon';
	event_metadata: {
		form_id: string;
		fields_filled: number;
		fields_total: number;
		abandonment_point: string;
		time_spent: number;
	};
}

export interface HoverIntentEvent extends BehaviorEvent {
	event_type: 'hover_intent';
	event_metadata: {
		hover_duration: number;
		cursor_stability: number;
		element_type: string;
		intent_strength: 'weak' | 'moderate' | 'strong';
	};
}

// ═══════════════════════════════════════════════════════════════════════════
// SESSION INTERFACES
// ═══════════════════════════════════════════════════════════════════════════

export interface BehaviorSession {
	session_id: string;
	visitor_id: string;
	user_id?: string;
	started_at: number;
	ended_at?: number;
	duration_seconds: number;
	page_count: number;
	event_count: number;

	// Scores
	engagement_score: number;
	intent_score: number;
	friction_score: number;
	churn_risk_score: number;

	// Flags
	has_rage_clicks: boolean;
	has_form_abandonment: boolean;
	has_speed_scrolls: boolean;
	has_exit_intent: boolean;
	has_dead_clicks: boolean;

	// Metadata
	device_type: 'mobile' | 'tablet' | 'desktop';
	browser: string;
	viewport_width: number;
	viewport_height: number;
	entry_url: string;
	exit_url?: string;
	referrer: string;
	utm_source?: string;
	utm_campaign?: string;
	utm_medium?: string;
}

// ═══════════════════════════════════════════════════════════════════════════
// SCORING INTERFACES
// ═══════════════════════════════════════════════════════════════════════════

export interface EngagementScoreData {
	scroll_depth: number;
	time_on_page: number;
	interaction_count: number;
	content_consumption: number;
}

export interface IntentScoreData {
	cta_interactions: number;
	hover_intents: number;
	form_engagements: number;
	goal_oriented_actions: number;
}

export interface FrictionScoreData {
	rage_clicks: number;
	form_abandonments: number;
	dead_clicks: number;
	speed_scrolls: number;
	backtracks: number;
	errors: number;
}

export interface ChurnRiskScoreData {
	exit_intent_detected: boolean;
	engagement_score: number;
	friction_score: number;
	abandonment_signals: number;
	idle_time_ratio: number;
}

// ═══════════════════════════════════════════════════════════════════════════
// FRICTION & INTENT INTERFACES
// ═══════════════════════════════════════════════════════════════════════════

export interface FrictionPoint {
	id: string;
	session_id: string;
	page_url: string;
	friction_type: 'rage_click' | 'form_abandon' | 'dead_click' | 'speed_scroll' | 'error' | 'other';
	severity: 'mild' | 'moderate' | 'severe' | 'critical';
	element?: string;
	element_selector?: string;
	description?: string;
	event_count: number;
	first_occurred_at: number;
	last_occurred_at: number;
	device_type: 'mobile' | 'tablet' | 'desktop';
	resolved: boolean;
}

export interface IntentSignal {
	id: string;
	session_id: string;
	user_id?: string;
	signal_type:
		| 'cta_hover'
		| 'form_start'
		| 'product_view'
		| 'pricing_view'
		| 'demo_request'
		| 'other';
	intent_strength: 'weak' | 'moderate' | 'strong';
	element?: string;
	page_url: string;
	timestamp: number;
	converted: boolean;
	conversion_timestamp?: number;
}

// ═══════════════════════════════════════════════════════════════════════════
// TRACKER CONFIG
// ═══════════════════════════════════════════════════════════════════════════

export interface BehaviorTrackerConfig {
	// API
	apiEndpoint: string;
	apiKey?: string;

	// Buffering
	bufferSize: number;
	bufferTimeout: number; // ms

	// Tracking Options
	trackScrollDepth: boolean;
	trackRageClicks: boolean;
	trackHoverIntent: boolean;
	trackFormBehavior: boolean;
	trackCursorMovement: boolean;
	trackIdleTime: boolean;

	// Thresholds
	rageClickThreshold: number; // clicks
	rageClickWindow: number; // ms
	hoverIntentDuration: number; // ms
	idleTimeout: number; // ms
	speedScrollThreshold: number; // px/sec

	// Privacy
	maskPII: boolean;
	respectDNT: boolean;
	anonymizeIP: boolean;

	// Performance
	sampleRate: number; // 0-1 (1 = 100%)
	maxEventsPerSession: number;
}

// ═══════════════════════════════════════════════════════════════════════════
// API INTERFACES
// ═══════════════════════════════════════════════════════════════════════════

export interface BehaviorEventBatch {
	session_id: string;
	visitor_id: string;
	user_id?: string;
	events: BehaviorEvent[];
	client_timestamp: number;
}

export interface BehaviorAnalytics {
	session: BehaviorSession;
	events: BehaviorEvent[];
	friction_points: FrictionPoint[];
	intent_signals: IntentSignal[];
}

export interface BehaviorDashboardData {
	overview: {
		total_sessions: number;
		avg_engagement_score: number;
		avg_friction_score: number;
		avg_intent_score: number;
		high_churn_risk_count: number;
	};
	friction_heatmap: Array<{
		page_url: string;
		friction_count: number;
		avg_severity: number;
		top_friction_type: string;
	}>;
	intent_funnel: Array<{
		stage: string;
		sessions: number;
		avg_intent_score: number;
		conversion_rate: number;
	}>;
	session_timeline: Array<{
		timestamp: number;
		sessions: number;
		avg_engagement: number;
	}>;
}

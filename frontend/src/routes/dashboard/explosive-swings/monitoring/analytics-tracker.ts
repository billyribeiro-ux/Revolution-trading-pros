/**
 * Analytics Tracker for Explosive Swings
 * Tracks user interactions and feature usage for A+++ grade observability
 */

import { browser } from '$app/environment';
import { track, Events } from '$lib/observability';
import { performanceMonitor } from './performance';

// ============================================================================
// Types
// ============================================================================

export interface UserInteraction {
	action: string;
	category: string;
	label?: string;
	value?: number;
	metadata?: Record<string, unknown>;
}

export interface FeatureUsage {
	feature: string;
	action: 'view' | 'click' | 'toggle' | 'submit' | 'close';
	timestamp: number;
	duration?: number;
	success?: boolean;
}

export interface SessionMetrics {
	pageViews: number;
	interactions: number;
	alertsViewed: number;
	tradesViewed: number;
	filtersApplied: number;
	modalOpens: number;
	sessionDuration: number;
}

// ============================================================================
// Event Names for Explosive Swings
// ============================================================================

export const ExplosiveSwingsEvents = {
	// Page Events
	PAGE_VIEWED: 'explosive_swings_page_viewed',
	PAGE_LEFT: 'explosive_swings_page_left',

	// Alert Events
	ALERT_VIEWED: 'explosive_swings_alert_viewed',
	ALERT_CLICKED: 'explosive_swings_alert_clicked',
	ALERT_COPIED: 'explosive_swings_alert_copied',
	ALERT_NOTES_TOGGLED: 'explosive_swings_alert_notes_toggled',
	ALERT_FILTER_APPLIED: 'explosive_swings_alert_filter_applied',
	ALERTS_PAGINATED: 'explosive_swings_alerts_paginated',

	// Trade Events
	TRADE_VIEWED: 'explosive_swings_trade_viewed',
	TRADE_PLAN_VIEWED: 'explosive_swings_trade_plan_viewed',
	POSITION_VIEWED: 'explosive_swings_position_viewed',
	POSITION_CLOSED: 'explosive_swings_position_closed',
	POSITION_UPDATED: 'explosive_swings_position_updated',
	POSITION_INVALIDATED: 'explosive_swings_position_invalidated',
	TRADE_ADDED: 'explosive_swings_trade_added',

	// Modal Events
	MODAL_OPENED: 'explosive_swings_modal_opened',
	MODAL_CLOSED: 'explosive_swings_modal_closed',
	MODAL_SUBMITTED: 'explosive_swings_modal_submitted',

	// Video Events
	VIDEO_VIEWED: 'explosive_swings_video_viewed',
	VIDEO_STARTED: 'explosive_swings_video_started',
	VIDEO_COMPLETED: 'explosive_swings_video_completed',

	// Performance Events
	SLOW_LOAD_DETECTED: 'explosive_swings_slow_load_detected',
	ERROR_OCCURRED: 'explosive_swings_error_occurred',

	// Admin Events
	ADMIN_ACTION_PERFORMED: 'explosive_swings_admin_action'
} as const;

export type ExplosiveSwingsEventName =
	(typeof ExplosiveSwingsEvents)[keyof typeof ExplosiveSwingsEvents];

// ============================================================================
// Analytics Tracker Class
// ============================================================================

class AnalyticsTracker {
	private static instance: AnalyticsTracker;
	private sessionStartTime: number;
	private interactionCount = 0;
	private pageViewCount = 0;
	private alertsViewedCount = 0;
	private tradesViewedCount = 0;
	private filtersAppliedCount = 0;
	private modalOpensCount = 0;
	private featureUsage: FeatureUsage[] = [];
	private isDebugMode: boolean;

	private constructor() {
		this.sessionStartTime = Date.now();
		this.isDebugMode = import.meta.env.DEV;
	}

	static getInstance(): AnalyticsTracker {
		if (!AnalyticsTracker.instance) {
			AnalyticsTracker.instance = new AnalyticsTracker();
		}
		return AnalyticsTracker.instance;
	}

	// =========================================================================
	// Core Tracking Methods
	// =========================================================================

	/**
	 * Track a user interaction
	 */
	trackInteraction(interaction: UserInteraction): void {
		if (!browser) return;

		this.interactionCount++;

		const payload = {
			action: interaction.action,
			category: interaction.category,
			...(interaction.label && { label: interaction.label }),
			...(interaction.value !== undefined && { value: interaction.value }),
			...interaction.metadata,
			session_duration_ms: this.getSessionDuration(),
			interaction_count: this.interactionCount
		};

		track(interaction.action, payload);

		if (this.isDebugMode) {
			console.debug('[AnalyticsTracker] Interaction:', interaction);
		}
	}

	/**
	 * Simple event tracking - convenience wrapper for trackInteraction
	 */
	trackEvent(
		category: string,
		action: string,
		label?: string,
		value?: number,
		metadata?: Record<string, unknown>
	): void {
		this.trackInteraction({
			action,
			category,
			label,
			value,
			metadata
		});
	}

	/**
	 * Track feature usage
	 */
	trackFeature(
		feature: string,
		action: FeatureUsage['action'],
		metadata?: Record<string, unknown>
	): void {
		if (!browser) return;

		const usage: FeatureUsage = {
			feature,
			action,
			timestamp: Date.now(),
			...metadata
		};

		this.featureUsage.push(usage);

		track(`feature_${action}`, {
			feature_name: feature,
			action,
			...metadata
		});

		if (this.isDebugMode) {
			console.debug('[AnalyticsTracker] Feature usage:', usage);
		}
	}

	// =========================================================================
	// Page Events
	// =========================================================================

	/**
	 * Track page view
	 */
	trackPageView(): void {
		this.pageViewCount++;
		performanceMonitor.startMark('page-load');

		this.trackInteraction({
			action: ExplosiveSwingsEvents.PAGE_VIEWED,
			category: 'navigation',
			metadata: {
				page_view_count: this.pageViewCount,
				referrer: browser ? document.referrer : undefined
			}
		});
	}

	/**
	 * Track page leave
	 */
	trackPageLeave(): void {
		const duration = performanceMonitor.endMark('page-load');

		this.trackInteraction({
			action: ExplosiveSwingsEvents.PAGE_LEFT,
			category: 'navigation',
			value: Math.round(duration),
			metadata: {
				session_metrics: this.getSessionMetrics()
			}
		});
	}

	// =========================================================================
	// Alert Events
	// =========================================================================

	/**
	 * Track alert viewed
	 */
	trackAlertViewed(alertId: number, alertType: string): void {
		this.alertsViewedCount++;

		this.trackInteraction({
			action: ExplosiveSwingsEvents.ALERT_VIEWED,
			category: 'alerts',
			label: alertType,
			value: alertId,
			metadata: {
				alert_id: alertId,
				alert_type: alertType,
				alerts_viewed_total: this.alertsViewedCount
			}
		});
	}

	/**
	 * Track alert clicked
	 */
	trackAlertClicked(alertId: number, ticker: string): void {
		this.trackInteraction({
			action: ExplosiveSwingsEvents.ALERT_CLICKED,
			category: 'alerts',
			label: ticker,
			value: alertId,
			metadata: {
				alert_id: alertId,
				ticker
			}
		});
	}

	/**
	 * Track alert copied
	 */
	trackAlertCopied(alertId: number, ticker: string): void {
		this.trackInteraction({
			action: ExplosiveSwingsEvents.ALERT_COPIED,
			category: 'alerts',
			label: ticker,
			value: alertId,
			metadata: {
				alert_id: alertId,
				ticker
			}
		});
	}

	/**
	 * Track alert notes toggled
	 */
	trackNotesToggled(alertId: number, expanded: boolean): void {
		this.trackInteraction({
			action: ExplosiveSwingsEvents.ALERT_NOTES_TOGGLED,
			category: 'alerts',
			value: alertId,
			metadata: {
				alert_id: alertId,
				expanded
			}
		});
	}

	/**
	 * Track filter applied
	 */
	trackFilterApplied(filterType: string, filterValue: string): void {
		this.filtersAppliedCount++;
		performanceMonitor.startMark('filter-change');

		this.trackInteraction({
			action: ExplosiveSwingsEvents.ALERT_FILTER_APPLIED,
			category: 'filters',
			label: `${filterType}:${filterValue}`,
			metadata: {
				filter_type: filterType,
				filter_value: filterValue,
				filters_applied_total: this.filtersAppliedCount
			}
		});
	}

	/**
	 * Track filter completed (after data loads)
	 */
	trackFilterCompleted(): void {
		performanceMonitor.endMark('filter-change');
	}

	/**
	 * Track pagination
	 */
	trackPagination(page: number, totalPages: number): void {
		performanceMonitor.startMark('pagination');

		this.trackInteraction({
			action: ExplosiveSwingsEvents.ALERTS_PAGINATED,
			category: 'pagination',
			value: page,
			metadata: {
				current_page: page,
				total_pages: totalPages
			}
		});
	}

	/**
	 * Track pagination completed (after data loads)
	 */
	trackPaginationCompleted(): void {
		performanceMonitor.endMark('pagination');
	}

	// =========================================================================
	// Trade Events
	// =========================================================================

	/**
	 * Track trade viewed
	 */
	trackTradeViewed(tradeId: string | number, ticker: string): void {
		this.tradesViewedCount++;

		this.trackInteraction({
			action: ExplosiveSwingsEvents.TRADE_VIEWED,
			category: 'trades',
			label: ticker,
			metadata: {
				trade_id: tradeId,
				ticker,
				trades_viewed_total: this.tradesViewedCount
			}
		});
	}

	/**
	 * Track trade plan viewed
	 */
	trackTradePlanViewed(entriesCount: number): void {
		this.trackInteraction({
			action: ExplosiveSwingsEvents.TRADE_PLAN_VIEWED,
			category: 'trades',
			value: entriesCount,
			metadata: {
				entries_count: entriesCount
			}
		});
	}

	/**
	 * Track position action
	 */
	trackPositionAction(
		action: 'closed' | 'updated' | 'invalidated',
		positionId: string,
		ticker: string
	): void {
		const eventMap = {
			closed: ExplosiveSwingsEvents.POSITION_CLOSED,
			updated: ExplosiveSwingsEvents.POSITION_UPDATED,
			invalidated: ExplosiveSwingsEvents.POSITION_INVALIDATED
		};

		this.trackInteraction({
			action: eventMap[action],
			category: 'positions',
			label: ticker,
			metadata: {
				position_id: positionId,
				ticker,
				action_type: action
			}
		});
	}

	/**
	 * Track trade added
	 */
	trackTradeAdded(ticker: string, success: boolean): void {
		this.trackInteraction({
			action: ExplosiveSwingsEvents.TRADE_ADDED,
			category: 'trades',
			label: ticker,
			metadata: {
				ticker,
				success
			}
		});
	}

	// =========================================================================
	// Modal Events
	// =========================================================================

	/**
	 * Track modal opened
	 */
	trackModalOpened(modalType: string): void {
		this.modalOpensCount++;
		performanceMonitor.startMark('modal-open');

		this.trackInteraction({
			action: ExplosiveSwingsEvents.MODAL_OPENED,
			category: 'modals',
			label: modalType,
			metadata: {
				modal_type: modalType,
				modals_opened_total: this.modalOpensCount
			}
		});

		performanceMonitor.endMark('modal-open');
	}

	/**
	 * Track modal closed
	 */
	trackModalClosed(modalType: string, submitted: boolean = false): void {
		this.trackInteraction({
			action: submitted
				? ExplosiveSwingsEvents.MODAL_SUBMITTED
				: ExplosiveSwingsEvents.MODAL_CLOSED,
			category: 'modals',
			label: modalType,
			metadata: {
				modal_type: modalType,
				submitted
			}
		});
	}

	// =========================================================================
	// Video Events
	// =========================================================================

	/**
	 * Track video viewed
	 */
	trackVideoViewed(videoTitle: string): void {
		this.trackInteraction({
			action: ExplosiveSwingsEvents.VIDEO_VIEWED,
			category: 'videos',
			label: videoTitle
		});
	}

	/**
	 * Track video started
	 */
	trackVideoStarted(videoTitle: string): void {
		this.trackInteraction({
			action: ExplosiveSwingsEvents.VIDEO_STARTED,
			category: 'videos',
			label: videoTitle
		});

		// Also track via main observability
		track(Events.VIDEO_STARTED, {
			video_title: videoTitle,
			source: 'explosive_swings'
		});
	}

	/**
	 * Track video completed
	 */
	trackVideoCompleted(videoTitle: string, watchTimeSeconds: number): void {
		this.trackInteraction({
			action: ExplosiveSwingsEvents.VIDEO_COMPLETED,
			category: 'videos',
			label: videoTitle,
			value: watchTimeSeconds,
			metadata: {
				watch_time_seconds: watchTimeSeconds
			}
		});

		// Also track via main observability
		track(Events.VIDEO_COMPLETED, {
			video_title: videoTitle,
			watch_time_seconds: watchTimeSeconds,
			source: 'explosive_swings'
		});
	}

	// =========================================================================
	// Error & Performance Events
	// =========================================================================

	/**
	 * Track slow load detected
	 */
	trackSlowLoad(operation: string, durationMs: number): void {
		this.trackInteraction({
			action: ExplosiveSwingsEvents.SLOW_LOAD_DETECTED,
			category: 'performance',
			label: operation,
			value: Math.round(durationMs),
			metadata: {
				operation,
				duration_ms: durationMs
			}
		});
	}

	/**
	 * Track error occurred
	 */
	trackError(errorType: string, errorMessage: string, context?: Record<string, unknown>): void {
		this.trackInteraction({
			action: ExplosiveSwingsEvents.ERROR_OCCURRED,
			category: 'errors',
			label: errorType,
			metadata: {
				error_type: errorType,
				error_message: errorMessage,
				...context
			}
		});

		// Also track via main observability
		track(Events.ERROR_OCCURRED, {
			error_type: errorType,
			error_message: errorMessage,
			source: 'explosive_swings',
			...context
		});
	}

	// =========================================================================
	// Admin Events
	// =========================================================================

	/**
	 * Track admin action
	 */
	trackAdminAction(action: string, resource: string, resourceId?: string | number): void {
		this.trackInteraction({
			action: ExplosiveSwingsEvents.ADMIN_ACTION_PERFORMED,
			category: 'admin',
			label: `${action}:${resource}`,
			metadata: {
				admin_action: action,
				resource,
				resource_id: resourceId
			}
		});
	}

	// =========================================================================
	// Session Metrics
	// =========================================================================

	/**
	 * Get session duration in milliseconds
	 */
	getSessionDuration(): number {
		return Date.now() - this.sessionStartTime;
	}

	/**
	 * Get session metrics
	 */
	getSessionMetrics(): SessionMetrics {
		return {
			pageViews: this.pageViewCount,
			interactions: this.interactionCount,
			alertsViewed: this.alertsViewedCount,
			tradesViewed: this.tradesViewedCount,
			filtersApplied: this.filtersAppliedCount,
			modalOpens: this.modalOpensCount,
			sessionDuration: this.getSessionDuration()
		};
	}

	/**
	 * Get feature usage summary
	 */
	getFeatureUsageSummary(): Record<string, number> {
		const summary: Record<string, number> = {};

		for (const usage of this.featureUsage) {
			const key = `${usage.feature}_${usage.action}`;
			summary[key] = (summary[key] || 0) + 1;
		}

		return summary;
	}

	/**
	 * Reset session metrics (for new session)
	 */
	resetSession(): void {
		this.sessionStartTime = Date.now();
		this.interactionCount = 0;
		this.pageViewCount = 0;
		this.alertsViewedCount = 0;
		this.tradesViewedCount = 0;
		this.filtersAppliedCount = 0;
		this.modalOpensCount = 0;
		this.featureUsage = [];
	}
}

// Export singleton instance
export const analyticsTracker = AnalyticsTracker.getInstance();

// Convenience exports
export const trackPageView = () => analyticsTracker.trackPageView();
export const trackPageLeave = () => analyticsTracker.trackPageLeave();
export const trackAlertViewed = (alertId: number, alertType: string) =>
	analyticsTracker.trackAlertViewed(alertId, alertType);
export const trackAlertClicked = (alertId: number, ticker: string) =>
	analyticsTracker.trackAlertClicked(alertId, ticker);
export const trackAlertCopied = (alertId: number, ticker: string) =>
	analyticsTracker.trackAlertCopied(alertId, ticker);
export const trackFilterApplied = (filterType: string, filterValue: string) =>
	analyticsTracker.trackFilterApplied(filterType, filterValue);
export const trackPagination = (page: number, totalPages: number) =>
	analyticsTracker.trackPagination(page, totalPages);
export const trackModalOpened = (modalType: string) => analyticsTracker.trackModalOpened(modalType);
export const trackModalClosed = (modalType: string, submitted?: boolean) =>
	analyticsTracker.trackModalClosed(modalType, submitted);
export const trackError = (
	errorType: string,
	errorMessage: string,
	context?: Record<string, unknown>
) => analyticsTracker.trackError(errorType, errorMessage, context);
export const getSessionMetrics = () => analyticsTracker.getSessionMetrics();

export default analyticsTracker;

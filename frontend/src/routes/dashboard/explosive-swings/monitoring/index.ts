/**
 * Explosive Swings - Monitoring Module
 * Central export point for performance monitoring and analytics tracking
 */

// Performance monitoring
export {
	performanceMonitor,
	startMark,
	endMark,
	measureAsync,
	measureSync,
	reportMetrics,
	getDetailedMetrics,
	getPerformanceRating,
	PERFORMANCE_THRESHOLDS,
	type PerformanceMetrics,
	type PerformanceThresholds
} from './performance';

// Analytics tracking
export {
	analyticsTracker,
	trackPageView,
	trackPageLeave,
	trackAlertViewed,
	trackAlertClicked,
	trackAlertCopied,
	trackFilterApplied,
	trackPagination,
	trackModalOpened,
	trackModalClosed,
	trackError,
	getSessionMetrics,
	ExplosiveSwingsEvents,
	type UserInteraction,
	type FeatureUsage,
	type SessionMetrics,
	type ExplosiveSwingsEventName
} from './analytics-tracker';

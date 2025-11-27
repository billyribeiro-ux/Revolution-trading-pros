/**
 * Observability - Barrel Export
 * Unified observability infrastructure
 * 
 * @version 2.0.0
 * @author Revolution Trading Pros
 */

// Metrics
export {
	metrics,
	track,
	trackPageView,
	identify,
	startTimer,
	Events,
	type EventProperties,
	type PageViewProperties,
	type UserProperties,
	type EventName,
} from './metrics';

// Experiments
export {
	useExperiment,
	useFeatureFlag,
	trackConversion,
	overrideExperiment,
	clearOverrides,
	experimentAssignments,
	EXPERIMENTS,
	FEATURE_FLAGS,
	type Experiment,
	type ExperimentAssignment,
	type FeatureFlag,
} from './experiments';

// Telemetry (existing)
export * from './telemetry';

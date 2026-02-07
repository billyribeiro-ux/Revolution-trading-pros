/**
 * Experiments - Feature flags and A/B testing infrastructure
 * Netflix E6 Level Experimentation
 *
 * @version 2.0.0
 * @author Revolution Trading Pros
 */

import { browser } from '$app/environment';
import { writable, derived, get } from 'svelte/store';
import { track } from './metrics';

// ═══════════════════════════════════════════════════════════════════════════
// Types
// ═══════════════════════════════════════════════════════════════════════════

export interface Experiment {
	id: string;
	name: string;
	variants: string[];
	defaultVariant: string;
	enabled: boolean;
}

export interface ExperimentAssignment {
	experimentId: string;
	variant: string;
	assignedAt: number;
}

export interface FeatureFlag {
	id: string;
	enabled: boolean;
	rolloutPercentage?: number;
}

// ═══════════════════════════════════════════════════════════════════════════
// Experiment Definitions
// ═══════════════════════════════════════════════════════════════════════════

export const EXPERIMENTS: Record<string, Experiment> = {
	// Example experiments - add your own here
	new_pricing_page: {
		id: 'new_pricing_page',
		name: 'New Pricing Page Design',
		variants: ['control', 'treatment_a', 'treatment_b'],
		defaultVariant: 'control',
		enabled: false
	},
	simplified_checkout: {
		id: 'simplified_checkout',
		name: 'Simplified Checkout Flow',
		variants: ['control', 'treatment'],
		defaultVariant: 'control',
		enabled: false
	},
	trading_room_layout: {
		id: 'trading_room_layout',
		name: 'Trading Room Layout',
		variants: ['control', 'side_panel', 'bottom_panel'],
		defaultVariant: 'control',
		enabled: false
	}
};

export const FEATURE_FLAGS: Record<string, FeatureFlag> = {
	// Example feature flags - add your own here
	dark_mode: {
		id: 'dark_mode',
		enabled: true
	},
	new_dashboard: {
		id: 'new_dashboard',
		enabled: false,
		rolloutPercentage: 0
	},
	live_chat: {
		id: 'live_chat',
		enabled: false,
		rolloutPercentage: 0
	},
	ai_alerts: {
		id: 'ai_alerts',
		enabled: false,
		rolloutPercentage: 0
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// Storage
// ═══════════════════════════════════════════════════════════════════════════

const STORAGE_KEY = 'rtp_experiments';
const USER_ID_KEY = 'rtp_experiment_user_id';

function getStoredAssignments(): Record<string, ExperimentAssignment> {
	if (!browser) return {};

	try {
		const stored = localStorage.getItem(STORAGE_KEY);
		return stored ? JSON.parse(stored) : {};
	} catch {
		return {};
	}
}

function storeAssignment(assignment: ExperimentAssignment): void {
	if (!browser) return;

	try {
		const assignments = getStoredAssignments();
		assignments[assignment.experimentId] = assignment;
		localStorage.setItem(STORAGE_KEY, JSON.stringify(assignments));
	} catch {
		// Ignore storage errors
	}
}

function getOrCreateUserId(): string {
	if (!browser) return 'server';

	let userId = localStorage.getItem(USER_ID_KEY);
	if (!userId) {
		userId = `anon_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
		localStorage.setItem(USER_ID_KEY, userId);
	}
	return userId;
}

// ═══════════════════════════════════════════════════════════════════════════
// Experiment Assignment
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Hash function for consistent assignment
 */
function hashString(str: string): number {
	let hash = 0;
	for (let i = 0; i < str.length; i++) {
		const char = str.charCodeAt(i);
		hash = (hash << 5) - hash + char;
		hash = hash & hash; // Convert to 32bit integer
	}
	return Math.abs(hash);
}

/**
 * Get variant assignment for an experiment
 */
function assignVariant(experiment: Experiment, userId: string): string {
	if (!experiment.enabled) {
		return experiment.defaultVariant;
	}

	// Check for stored assignment
	const stored = getStoredAssignments()[experiment.id];
	if (stored) {
		return stored.variant;
	}

	// Assign based on hash
	const hash = hashString(`${experiment.id}:${userId}`);
	const variantIndex = hash % experiment.variants.length;
	const variant = experiment.variants[variantIndex] || experiment.variants[0] || 'control';

	// Store assignment
	storeAssignment({
		experimentId: experiment.id,
		variant,
		assignedAt: Date.now()
	});

	return variant;
}

// ═══════════════════════════════════════════════════════════════════════════
// Stores
// ═══════════════════════════════════════════════════════════════════════════

interface ExperimentsState {
	userId: string;
	assignments: Record<string, string>;
	exposures: Set<string>;
}

const experimentsStore = writable<ExperimentsState>({
	userId: '',
	assignments: {},
	exposures: new Set()
});

/**
 * Fetch experiment config from backend
 */
async function fetchExperimentConfig(): Promise<void> {
	if (!browser) return;

	try {
		const userId = getOrCreateUserId();
		const response = await fetch(`/api/experiments/config?anonymous_id=${userId}`, {
			credentials: 'include'
		});

		if (response.ok) {
			const data = await response.json();

			// Update assignments from server
			if (data.experiments) {
				experimentsStore.update((state) => ({
					...state,
					assignments: {
						...state.assignments,
						...Object.fromEntries(
							Object.entries(data.experiments).map(([key, val]: [string, any]) => [
								key,
								val.variant
							])
						)
					}
				}));
			}

			// Update feature flags from server
			if (data.feature_flags) {
				Object.entries(data.feature_flags).forEach(([key, enabled]) => {
					if (FEATURE_FLAGS[key]) {
						FEATURE_FLAGS[key].enabled = enabled as boolean;
					}
				});
			}
		}
	} catch (_error) {
		// Silently fail - use local defaults
		console.debug('[Experiments] Failed to fetch config from server, using defaults');
	}
}

// Initialize on client
if (browser) {
	const userId = getOrCreateUserId();
	const assignments: Record<string, string> = {};

	// Pre-assign all experiments locally first
	for (const [id, experiment] of Object.entries(EXPERIMENTS)) {
		assignments[id] = assignVariant(experiment, userId);
	}

	experimentsStore.set({
		userId,
		assignments,
		exposures: new Set()
	});

	// Then fetch from server (async, will update store when ready)
	fetchExperimentConfig();
}

// ═══════════════════════════════════════════════════════════════════════════
// Public API
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Get the variant for an experiment
 * Automatically tracks exposure
 */
export function useExperiment(experimentId: string): string {
	const state = get(experimentsStore);
	const experiment = EXPERIMENTS[experimentId];

	if (!experiment) {
		console.warn(`[Experiments] Unknown experiment: ${experimentId}`);
		return 'control';
	}

	const variant = state.assignments[experimentId] || experiment.defaultVariant;

	// Track exposure (once per session)
	if (browser && !state.exposures.has(experimentId)) {
		state.exposures.add(experimentId);
		experimentsStore.update((s) => ({ ...s, exposures: state.exposures }));

		track('experiment_exposure', {
			experiment_id: experimentId,
			experiment_name: experiment.name,
			variant
		});
	}

	return variant;
}

/**
 * Check if a feature flag is enabled
 */
export function useFeatureFlag(flagId: string): boolean {
	const flag = FEATURE_FLAGS[flagId];

	if (!flag) {
		console.warn(`[Experiments] Unknown feature flag: ${flagId}`);
		return false;
	}

	if (!flag.enabled) {
		return false;
	}

	// Check rollout percentage if specified
	if (flag.rolloutPercentage !== undefined && flag.rolloutPercentage < 100) {
		const userId = browser ? getOrCreateUserId() : 'server';
		const hash = hashString(`${flagId}:${userId}`);
		const bucket = hash % 100;
		return bucket < flag.rolloutPercentage;
	}

	return true;
}

/**
 * Track experiment conversion
 */
export function trackConversion(
	experimentId: string,
	conversionType: string,
	value?: number
): void {
	const state = get(experimentsStore);
	const variant = state.assignments[experimentId];

	if (!variant) {
		console.warn(`[Experiments] No assignment for experiment: ${experimentId}`);
		return;
	}

	track('experiment_conversion', {
		experiment_id: experimentId,
		variant,
		conversion_type: conversionType,
		conversion_value: value
	});
}

/**
 * Override experiment variant (for testing)
 */
export function overrideExperiment(experimentId: string, variant: string): void {
	if (!browser) return;

	experimentsStore.update((state) => ({
		...state,
		assignments: {
			...state.assignments,
			[experimentId]: variant
		}
	}));

	storeAssignment({
		experimentId,
		variant,
		assignedAt: Date.now()
	});
}

/**
 * Clear all experiment overrides
 */
export function clearOverrides(): void {
	if (!browser) return;

	localStorage.removeItem(STORAGE_KEY);

	const userId = getOrCreateUserId();
	const assignments: Record<string, string> = {};

	for (const [id, experiment] of Object.entries(EXPERIMENTS)) {
		assignments[id] = assignVariant(experiment, userId);
	}

	experimentsStore.set({
		userId,
		assignments,
		exposures: new Set()
	});
}

// ═══════════════════════════════════════════════════════════════════════════
// Derived Stores
// ═══════════════════════════════════════════════════════════════════════════

export const experimentAssignments = derived(experimentsStore, ($state) => $state.assignments);

export default {
	useExperiment,
	useFeatureFlag,
	trackConversion,
	overrideExperiment,
	clearOverrides,
	EXPERIMENTS,
	FEATURE_FLAGS
};

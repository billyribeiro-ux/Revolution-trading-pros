/**
 * RevolutionBehavior-L8-System - Auto-Initialization
 *
 * Import this in your root layout to enable behavior tracking
 */

import { browser } from '$app/environment';
import { BehaviorTracker } from './tracker';

let tracker: BehaviorTracker | null = null;

export function initBehaviorTracking() {
	if (!browser) return;

	if (!tracker) {
		tracker = new BehaviorTracker({
			apiEndpoint: '/api/behavior/events',
			trackScrollDepth: true,
			trackRageClicks: true,
			trackHoverIntent: true,
			trackFormBehavior: true,
			trackIdleTime: true,
			sampleRate: 1.0 // 100% sampling
		});

		console.log('[RevolutionBehavior-L8] Tracker initialized');
	}

	return tracker;
}

export function getBehaviorTracker(): BehaviorTracker | null {
	return tracker;
}

export function setUserId(userId: string) {
	if (tracker) {
		tracker.setUserId(userId);
	}
}

// Auto-init on import
if (browser) {
	initBehaviorTracking();
}

/**
 * Consent Versioning System
 *
 * Tracks privacy policy versions and re-prompts users when:
 * - Privacy policy is updated
 * - New cookie categories are added
 * - Vendors change significantly
 *
 * @module consent/versioning
 * @version 1.0.0
 */

import { browser } from '$app/environment';
import { writable, get } from 'svelte/store';
import type { ConsentState } from './types';

/**
 * Version change type
 */
export type VersionChangeType = 'major' | 'minor' | 'patch';

/**
 * Policy version configuration
 */
export interface PolicyVersion {
	version: string;
	publishedAt: string;
	changelog: string[];
	requiresReconsent: boolean;
	changeType: VersionChangeType;
}

/**
 * Current policy version
 * Update this when your privacy policy changes
 */
export const CURRENT_POLICY_VERSION: PolicyVersion = {
	version: '1.0.0',
	publishedAt: '2024-01-01T00:00:00Z',
	changelog: ['Initial privacy policy release'],
	requiresReconsent: false,
	changeType: 'major',
};

/**
 * Policy version history
 */
export const POLICY_VERSIONS: PolicyVersion[] = [
	CURRENT_POLICY_VERSION,
	// Add new versions here when updating policy
	// {
	//   version: '1.1.0',
	//   publishedAt: '2024-06-01T00:00:00Z',
	//   changelog: ['Added TikTok Pixel tracking', 'Updated data retention policy'],
	//   requiresReconsent: true,
	//   changeType: 'minor'
	// }
];

/**
 * Version comparison result
 */
export interface VersionCompareResult {
	needsReconsent: boolean;
	userVersion: string | null;
	currentVersion: string;
	missedVersions: PolicyVersion[];
	changesSummary: string[];
}

/**
 * Parse semantic version
 */
function parseVersion(version: string): [number, number, number] {
	const parts = version.split('.').map((p) => parseInt(p, 10) || 0);
	return [parts[0] || 0, parts[1] || 0, parts[2] || 0];
}

/**
 * Compare two semantic versions
 * Returns: -1 if a < b, 0 if a == b, 1 if a > b
 */
function compareVersions(a: string, b: string): number {
	const [aMajor, aMinor, aPatch] = parseVersion(a);
	const [bMajor, bMinor, bPatch] = parseVersion(b);

	if (aMajor !== bMajor) return aMajor < bMajor ? -1 : 1;
	if (aMinor !== bMinor) return aMinor < bMinor ? -1 : 1;
	if (aPatch !== bPatch) return aPatch < bPatch ? -1 : 1;

	return 0;
}

/**
 * Check if user's consent needs to be refreshed due to policy changes
 */
export function checkVersionCompatibility(consent: ConsentState): VersionCompareResult {
	const userVersion = consent.policyVersion || null;
	const currentVersion = CURRENT_POLICY_VERSION.version;

	const result: VersionCompareResult = {
		needsReconsent: false,
		userVersion,
		currentVersion,
		missedVersions: [],
		changesSummary: [],
	};

	// No previous version - first time user
	if (!userVersion) {
		return result;
	}

	// Same version - no update needed
	if (compareVersions(userVersion, currentVersion) >= 0) {
		return result;
	}

	// Find all versions between user's version and current
	const missedVersions = POLICY_VERSIONS.filter((v) => {
		return (
			compareVersions(v.version, userVersion) > 0 &&
			compareVersions(v.version, currentVersion) <= 0
		);
	}).sort((a, b) => compareVersions(a.version, b.version));

	result.missedVersions = missedVersions;

	// Check if any missed version requires reconsent
	result.needsReconsent = missedVersions.some((v) => v.requiresReconsent);

	// Compile changelog
	result.changesSummary = missedVersions.flatMap((v) => v.changelog);

	return result;
}

/**
 * Store for policy update notification
 */
export const policyUpdateAvailable = writable<VersionCompareResult | null>(null);

/**
 * Check for policy updates and notify if needed
 */
export function checkForPolicyUpdates(consent: ConsentState): void {
	if (!browser) return;

	const result = checkVersionCompatibility(consent);

	if (result.needsReconsent || result.missedVersions.length > 0) {
		policyUpdateAvailable.set(result);
		console.debug('[Versioning] Policy update detected:', result);
	} else {
		policyUpdateAvailable.set(null);
	}
}

/**
 * Acknowledge policy update (after user reviews and consents)
 */
export function acknowledgePolicyUpdate(): void {
	policyUpdateAvailable.set(null);
}

/**
 * Get policy version to save with consent
 */
export function getCurrentPolicyVersion(): string {
	return CURRENT_POLICY_VERSION.version;
}

/**
 * Get policy changelog between two versions
 */
export function getChangelog(fromVersion: string, toVersion: string = CURRENT_POLICY_VERSION.version): string[] {
	return POLICY_VERSIONS.filter((v) => {
		return (
			compareVersions(v.version, fromVersion) > 0 &&
			compareVersions(v.version, toVersion) <= 0
		);
	})
		.sort((a, b) => compareVersions(a.version, b.version))
		.flatMap((v) => v.changelog);
}

/**
 * Format version date for display
 */
export function formatVersionDate(version: PolicyVersion): string {
	return new Date(version.publishedAt).toLocaleDateString(undefined, {
		year: 'numeric',
		month: 'long',
		day: 'numeric',
	});
}

/**
 * Get human-readable version info
 */
export function getVersionInfo(): {
	version: string;
	publishedAt: string;
	formattedDate: string;
} {
	return {
		version: CURRENT_POLICY_VERSION.version,
		publishedAt: CURRENT_POLICY_VERSION.publishedAt,
		formattedDate: formatVersionDate(CURRENT_POLICY_VERSION),
	};
}

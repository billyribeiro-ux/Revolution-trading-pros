// ============================================================
// SAVED CONFIGURATIONS — localStorage Persistence
// CRUD operations for saved calculator configurations.
// Storage key: rtp:calc:saved-configs
// Limit: 50 configs max.
// ============================================================

import type { SavedConfig } from '../engine/types.js';

const STORAGE_KEY = 'rtp:calc:saved-configs';
const MAX_CONFIGS = 50;

/**
 * Get all saved configurations, newest first.
 * Returns empty array on SSR or parse failure.
 */
export function getSavedConfigs(): SavedConfig[] {
	if (typeof window === 'undefined') return [];
	try {
		const raw = localStorage.getItem(STORAGE_KEY);
		if (!raw) return [];
		const parsed: unknown = JSON.parse(raw);
		if (!Array.isArray(parsed)) return [];
		return parsed as SavedConfig[];
	} catch {
		return [];
	}
}

/**
 * Save a new configuration.
 * Auto-generates id, createdAt, updatedAt.
 * Prepends to list (newest first) and trims to MAX_CONFIGS.
 */
export function saveConfig(
	config: Omit<SavedConfig, 'id' | 'createdAt' | 'updatedAt'>
): SavedConfig {
	const configs = getSavedConfigs();
	const now = new Date().toISOString();
	const newConfig: SavedConfig = {
		...config,
		id: crypto.randomUUID(),
		createdAt: now,
		updatedAt: now
	};

	configs.unshift(newConfig);
	const trimmed = configs.slice(0, MAX_CONFIGS);
	localStorage.setItem(STORAGE_KEY, JSON.stringify(trimmed));

	return newConfig;
}

/**
 * Update an existing configuration by ID.
 * Returns the updated config, or null if not found.
 */
export function updateConfig(
	id: string,
	updates: Partial<Omit<SavedConfig, 'id' | 'createdAt'>>
): SavedConfig | null {
	const configs = getSavedConfigs();
	const idx = configs.findIndex((c) => c.id === id);
	if (idx === -1) return null;

	configs[idx] = {
		...configs[idx],
		...updates,
		updatedAt: new Date().toISOString()
	};

	localStorage.setItem(STORAGE_KEY, JSON.stringify(configs));
	return configs[idx];
}

/**
 * Delete a saved configuration by ID.
 * Returns true if deleted, false if not found.
 */
export function deleteConfig(id: string): boolean {
	const configs = getSavedConfigs();
	const filtered = configs.filter((c) => c.id !== id);
	if (filtered.length === configs.length) return false;

	localStorage.setItem(STORAGE_KEY, JSON.stringify(filtered));
	return true;
}

/**
 * Export all configs as a formatted JSON string.
 * Suitable for file download.
 */
export function exportConfigs(): string {
	return JSON.stringify(getSavedConfigs(), null, 2);
}

/**
 * Import configs from a JSON string.
 * Merges with existing configs (skips duplicates by ID).
 * Returns the number of newly added configs.
 */
export function importConfigs(json: string): number {
	try {
		const imported: unknown = JSON.parse(json);
		if (!Array.isArray(imported)) return 0;

		const existing = getSavedConfigs();
		const existingIds = new Set(existing.map((c) => c.id));

		let added = 0;
		for (const config of imported as SavedConfig[]) {
			if (config.id && !existingIds.has(config.id)) {
				existing.push(config);
				existingIds.add(config.id);
				added++;
			}
		}

		localStorage.setItem(STORAGE_KEY, JSON.stringify(existing.slice(0, MAX_CONFIGS)));
		return added;
	} catch {
		return 0;
	}
}

/**
 * Clear all saved configurations.
 */
export function clearAllConfigs(): void {
	localStorage.removeItem(STORAGE_KEY);
}

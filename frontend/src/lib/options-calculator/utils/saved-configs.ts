// ============================================================
// SAVED CONFIGURATIONS — localStorage Persistence
// CRUD operations for saved calculator configurations.
// Storage key: rtp:calc:saved-configs
// Limit: 50 configs max.
//
// Reads never throw: on the server, with storage blocked, or on corrupt data they return
// what can be trusted (every entry is validated against `savedConfigSchema`). Writes throw
// when storage is full or unavailable, so callers can tell the user the save failed.
// ============================================================

import * as v from 'valibot';
import { browser } from '$app/env';
import type { SavedConfig } from '../engine/types.js';

const STORAGE_KEY = 'rtp:calc:saved-configs';
const MAX_CONFIGS = 50;

const finiteNumber = v.pipe(v.number(), v.finite());
const optionTypeSchema = v.picklist(['call', 'put']);

/**
 * An optional field as `SavedConfig` declares it (`field?: T`). A `null` (hand-edited or
 * foreign export files) is accepted and normalised away, as in `data/schemas.ts`.
 */
function optionalField<T>(schema: v.GenericSchema<T>) {
	return v.optional(
		v.pipe(
			v.nullable(schema),
			v.transform((value) => value ?? undefined)
		)
	);
}

/** A saved configuration as stored (and as accepted from an imported file). */
const savedConfigSchema = v.object({
	id: v.pipe(v.string(), v.minLength(1)),
	name: v.string(),
	description: optionalField(v.string()),
	createdAt: v.string(),
	updatedAt: v.string(),
	inputs: v.object({
		spotPrice: finiteNumber,
		strikePrice: finiteNumber,
		volatility: finiteNumber,
		timeToExpiry: finiteNumber,
		riskFreeRate: finiteNumber,
		dividendYield: finiteNumber
	}),
	optionType: optionTypeSchema,
	mode: v.picklist(['single', 'strategy', 'compare']),
	ticker: optionalField(v.string()),
	strategyLegs: optionalField(
		v.array(
			v.object({
				id: v.string(),
				type: optionTypeSchema,
				strike: finiteNumber,
				expiry: finiteNumber,
				position: v.picklist([1, -1]),
				quantity: finiteNumber,
				premium: finiteNumber
			})
		)
	),
	tags: optionalField(v.array(v.string()))
}) satisfies v.GenericSchema<unknown, SavedConfig>;

/** The valid configurations in an untrusted value; anything malformed is dropped. */
function validConfigs(value: unknown): SavedConfig[] {
	if (!Array.isArray(value)) return [];
	return value.flatMap((entry: unknown) => {
		const result = v.safeParse(savedConfigSchema, entry);
		return result.success ? [result.output] : [];
	});
}

/** Throws when storage is full or unavailable (private mode). */
function writeConfigs(configs: SavedConfig[]): void {
	localStorage.setItem(STORAGE_KEY, JSON.stringify(configs));
}

/**
 * Get all saved configurations, newest first.
 * Returns an empty array on the server, when storage is unavailable, or on corrupt data.
 */
export function getSavedConfigs(): SavedConfig[] {
	if (!browser) return [];
	try {
		const raw = localStorage.getItem(STORAGE_KEY);
		return raw ? validConfigs(JSON.parse(raw)) : [];
	} catch {
		return [];
	}
}

/**
 * Save a new configuration.
 * Auto-generates id, createdAt, updatedAt.
 * Prepends to list (newest first) and trims to MAX_CONFIGS.
 * @throws when storage is full or unavailable.
 */
export function saveConfig(
	config: Omit<SavedConfig, 'id' | 'createdAt' | 'updatedAt'>
): SavedConfig {
	const now = new Date().toISOString();
	const newConfig: SavedConfig = {
		...config,
		id: crypto.randomUUID(),
		createdAt: now,
		updatedAt: now
	};

	writeConfigs([newConfig, ...getSavedConfigs()].slice(0, MAX_CONFIGS));
	return newConfig;
}

/**
 * Update an existing configuration by ID.
 * Returns the updated config, or null if not found.
 * @throws when storage is full or unavailable.
 */
export function updateConfig(
	id: string,
	updates: Partial<Omit<SavedConfig, 'id' | 'createdAt'>>
): SavedConfig | null {
	const configs = getSavedConfigs();
	const existing = configs.find((c) => c.id === id);
	if (!existing) return null;

	const updated: SavedConfig = { ...existing, ...updates, updatedAt: new Date().toISOString() };
	writeConfigs(configs.map((c) => (c.id === id ? updated : c)));
	return updated;
}

/**
 * Delete a saved configuration by ID.
 * Returns true if deleted, false if not found.
 * @throws when storage is full or unavailable.
 */
export function deleteConfig(id: string): boolean {
	const configs = getSavedConfigs();
	const remaining = configs.filter((c) => c.id !== id);
	if (remaining.length === configs.length) return false;

	writeConfigs(remaining);
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
 * Merges valid entries with existing configs (skips duplicates by ID), keeping MAX_CONFIGS.
 * Returns the number of newly added configs (0 for invalid JSON or a failed write).
 */
export function importConfigs(json: string): number {
	try {
		const existing = getSavedConfigs();
		const seen = new Set(existing.map((c) => c.id));
		const additions = validConfigs(JSON.parse(json)).filter((config) => {
			if (seen.has(config.id)) return false;
			seen.add(config.id);
			return true;
		});

		const merged = [...existing, ...additions].slice(0, MAX_CONFIGS);
		writeConfigs(merged);
		return Math.max(merged.length - existing.length, 0);
	} catch {
		return 0;
	}
}

/**
 * Clear all saved configurations.
 * @throws when storage is unavailable.
 */
export function clearAllConfigs(): void {
	localStorage.removeItem(STORAGE_KEY);
}

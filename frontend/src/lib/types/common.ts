/**
 * Common Type Definitions
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Shared types used across the application
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 */

/**
 * JSON-compatible value type
 * Used for serializable data structures
 */
export type JsonValue =
	| string
	| number
	| boolean
	| null
	| undefined
	| JsonValue[]
	| { [key: string]: JsonValue }
	| Record<string, unknown>;

/**
 * JSON object type
 */
export type JsonObject = { [key: string]: JsonValue };

/**
 * JSON array type
 */
export type JsonArray = JsonValue[];

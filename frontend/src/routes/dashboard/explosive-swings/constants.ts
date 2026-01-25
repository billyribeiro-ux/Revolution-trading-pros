/**
 * ═══════════════════════════════════════════════════════════════════════════════
 * Explosive Swings Room - Constants
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * @description Centralized constants for the Explosive Swings trading room
 * @version 1.0.0
 * @standards Apple Principal Engineer ICT 7+ Standards
 *
 * ICT 7 Principle: Single Source of Truth for configuration values
 */

// ═══════════════════════════════════════════════════════════════════════════
// ROOM IDENTIFICATION
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Room slug used for API routing (room-content endpoints)
 * Used in: /api/alerts/:slug, /api/trades/:slug, /api/stats/:slug
 */
export const ROOM_SLUG = 'explosive-swings';

/**
 * Room ID for room-content API (alerts, trades, stats, trade plans)
 * Backend table: room_content.rooms
 */
export const ROOM_CONTENT_ID = 4;

/**
 * Room ID for room-resources API (videos, tutorials, documents, watchlists)
 * Backend table: room_resources.rooms
 * Note: Different ID due to separate resource management system
 */
export const ROOM_RESOURCES_ID = 2;

// ═══════════════════════════════════════════════════════════════════════════
// PAGINATION DEFAULTS
// ═══════════════════════════════════════════════════════════════════════════

/** Default number of alerts per page */
export const ALERTS_PER_PAGE = 10;

/** Default number of trades per page */
export const TRADES_PER_PAGE = 50;

// ═══════════════════════════════════════════════════════════════════════════
// ROOM METADATA
// ═══════════════════════════════════════════════════════════════════════════

/** Display name for the room */
export const ROOM_NAME = 'Explosive Swings';

/** Room description */
export const ROOM_DESCRIPTION = 'Swing trading opportunities with explosive profit potential';

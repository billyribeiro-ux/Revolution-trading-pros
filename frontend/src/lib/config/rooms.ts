/**
 * Room Configuration
 * ═══════════════════════════════════════════════════════════════════════════
 * Centralized configuration for all trading rooms and services
 * @version 1.0.0 - December 2025
 */

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

export type RoomType = 'live-trading' | 'alerts-only';

export interface Room {
	id: string;
	slug: string;
	name: string;
	shortName: string;
	type: RoomType;
	membershipId: number;
	color: string;
	icon: string;
}

// ═══════════════════════════════════════════════════════════════════════════
// ROOM DEFINITIONS
// ═══════════════════════════════════════════════════════════════════════════

export const ROOMS: Room[] = [
	// Live Trading Rooms
	{
		id: 'day-trading-room',
		slug: 'day-trading-room',
		name: 'Day Trading Room',
		shortName: 'DTR',
		type: 'live-trading',
		membershipId: 1,
		color: '#0984ae',
		icon: '📈'
	},
	{
		id: 'swing-trading-room',
		slug: 'swing-trading-room',
		name: 'Swing Trading Room',
		shortName: 'STR',
		type: 'live-trading',
		membershipId: 2,
		color: '#10b981',
		icon: '📊'
	},
	{
		id: 'small-account-mentorship',
		slug: 'small-account-mentorship',
		name: 'Small Account Mentorship Room',
		shortName: 'SAM',
		type: 'live-trading',
		membershipId: 3,
		color: '#8b5cf6',
		icon: '🎯'
	},
	// Alerts Only Services
	{
		id: 'alerts-only',
		slug: 'alerts-only',
		name: 'Alerts Only',
		shortName: 'AO',
		type: 'alerts-only',
		membershipId: 4,
		color: '#f59e0b',
		icon: '🔔'
	},
	{
		id: 'explosive-swings',
		slug: 'explosive-swings',
		name: 'Explosive Swings',
		shortName: 'ES',
		type: 'alerts-only',
		membershipId: 5,
		color: '#ef4444',
		icon: '💥'
	},
	{
		id: 'spx-profit-pulse',
		slug: 'spx-profit-pulse',
		name: 'SPX Profit Pulse',
		shortName: 'SPX',
		type: 'alerts-only',
		membershipId: 6,
		color: '#ec4899',
		icon: '💰'
	}
];

// ═══════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Get all room IDs
 */
export const ALL_ROOM_IDS = ROOMS.map((r) => r.id);

/**
 * Get rooms by type
 */
export function getRoomsByType(type: RoomType): Room[] {
	return ROOMS.filter((r) => r.type === type);
}

/**
 * Get live trading rooms only
 */
export function getLiveTradingRooms(): Room[] {
	return getRoomsByType('live-trading');
}

/**
 * Get alerts only services
 */
export function getAlertsOnlyServices(): Room[] {
	return getRoomsByType('alerts-only');
}

/**
 * Get room by ID/slug
 */
export function getRoomById(id: string): Room | undefined {
	return ROOMS.find((r) => r.id === id || r.slug === id);
}

/**
 * Get room by membership ID
 */
export function getRoomByMembershipId(membershipId: number): Room | undefined {
	return ROOMS.find((r) => r.membershipId === membershipId);
}

/**
 * Get multiple rooms by IDs
 */
export function getRoomsByIds(ids: string[]): Room[] {
	return ROOMS.filter((r) => ids.includes(r.id));
}

/**
 * Check if room IDs represent all rooms
 */
export function isAllRooms(roomIds: string[]): boolean {
	return roomIds.length === ROOMS.length && ROOMS.every((r) => roomIds.includes(r.id));
}

/**
 * Get display text for room selection
 */
export function getRoomSelectionText(roomIds: string[]): string {
	if (roomIds.length === 0) return 'No rooms selected';
	if (isAllRooms(roomIds)) return 'All Rooms';
	if (roomIds.length === 1) {
		const room = getRoomById(roomIds[0]);
		return room?.name || roomIds[0];
	}
	return `${roomIds.length} rooms selected`;
}

export default ROOMS;

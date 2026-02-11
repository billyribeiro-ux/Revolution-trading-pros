/**
 * Axum Videos Domain Adapter — Server-Only
 *
 * @version 1.0.0
 */

import { axum, AxumError } from './client';

// ═══════════════════════════════════════════════════════════════════════════
// Types — Raw Axum response shapes (snake_case)
// ═══════════════════════════════════════════════════════════════════════════

export interface AxumWeeklyVideo {
	id: number;
	video_title: string;
	video_url: string;
	thumbnail_url: string | null;
	duration: string | null;
	published_at: string;
	week_title: string;
}

// ═══════════════════════════════════════════════════════════════════════════
// API Functions
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Fetch weekly video for a trading room.
 * Returns null on 404 (no video published yet).
 */
export async function fetchWeeklyVideo(roomSlug: string): Promise<AxumWeeklyVideo | null> {
	try {
		return await axum.get<AxumWeeklyVideo>(`/api/weekly-video/${roomSlug}`);
	} catch (err) {
		if (err instanceof AxumError && err.isNotFound) return null;
		throw err;
	}
}

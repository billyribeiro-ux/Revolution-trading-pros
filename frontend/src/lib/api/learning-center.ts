/**
 * Learning Center API
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * API endpoints for the Learning Center feature, including courses,
 * lessons, progress tracking, and video archive.
 *
 * @version 1.0.0 (December 2025)
 */

import { apiClient } from './client';
import { getAuthToken } from '$lib/stores/auth';
import type {
	TradingRoom,
	LessonModule,
	Lesson,
	LessonWithRelations,
	LearningCenterData,
	UserRoomProgress,
	UserLessonProgress,
	PaginatedLessons
} from '$lib/types/learning-center';

// ═══════════════════════════════════════════════════════════════════════════
// TYPES FOR ROOM CONTENT
// ═══════════════════════════════════════════════════════════════════════════

export interface RoomContent {
	id: number;
	title: string;
	slug: string;
	type: string;
	description: string;
	excerpt: string;
	thumbnail_url: string;
	content_url?: string;
	duration?: number;
	category_id?: string;
	categories: string[];
	tags: string[];
	instructor: { id: string; name: string; image?: string };
	membership_ids: number[];
	is_premium: boolean;
	is_published: boolean;
	is_featured: boolean;
	sort_order: number;
	view_count: number;
	like_count: number;
	created_at?: string;
	updated_at?: string;
	published_at?: string;
}

export interface RoomContentResponse {
	success: boolean;
	data: {
		content: RoomContent[];
		categories: Array<{ id: string; label: string; color?: string }>;
		instructors: Array<{ id: string; name: string; image?: string }>;
		pagination: {
			page: number;
			limit: number;
			total: number;
			total_pages: number;
		};
	};
	error?: string;
}

export interface RoomContentParams {
	page?: number;
	limit?: number;
	category_id?: string;
	type?: string;
	search?: string;
	instructor_id?: string;
	featured?: boolean;
	published?: boolean;
}

// Room slug to membership ID mapping
const roomSlugToMembershipId: Record<string, number> = {
	'day-trading-room': 1,
	'swing-trading-room': 2,
	'small-account-mentorship': 3,
	'options-day-trading': 4,
	'simpler-showcase': 5,
	'spx-profit-pulse': 6,
	'explosive-swing': 7
};

// ═══════════════════════════════════════════════════════════════════════════
// COURSES
// ═══════════════════════════════════════════════════════════════════════════

export const learningCenterApi = {
	/**
	 * Get all courses for a membership
	 */
	async getCourses(membershipSlug: string): Promise<LearningCenterData> {
		const response = (await apiClient.get(
			`/memberships/${membershipSlug}/learning-center`
		)) as { data: LearningCenterData };
		return response.data;
	},

	/**
	 * Get a specific course by ID
	 */
	async getCourse(membershipSlug: string, courseId: string): Promise<TradingRoom> {
		const response = (await apiClient.get(
			`/memberships/${membershipSlug}/courses/${courseId}`
		)) as { data: TradingRoom };
		return response.data;
	},

	/**
	 * Get course modules
	 */
	async getCourseModules(membershipSlug: string, courseId: string): Promise<LessonModule[]> {
		const response = (await apiClient.get(
			`/memberships/${membershipSlug}/courses/${courseId}/modules`
		)) as { data: LessonModule[] };
		return response.data;
	},

	// ═══════════════════════════════════════════════════════════════════════════
	// LESSONS
	// ═══════════════════════════════════════════════════════════════════════════

	/**
	 * Get a specific lesson with navigation context
	 */
	async getLesson(membershipSlug: string, lessonId: string): Promise<LessonWithRelations> {
		const response = (await apiClient.get(
			`/memberships/${membershipSlug}/lessons/${lessonId}`
		)) as { data: LessonWithRelations };
		return response.data;
	},

	/**
	 * Get all lessons for a module
	 */
	async getModuleLessons(membershipSlug: string, moduleId: number): Promise<Lesson[]> {
		const response = (await apiClient.get(
			`/memberships/${membershipSlug}/modules/${moduleId}/lessons`
		)) as { data: Lesson[] };
		return response.data;
	},

	// ═══════════════════════════════════════════════════════════════════════════
	// PROGRESS
	// ═══════════════════════════════════════════════════════════════════════════

	/**
	 * Get user's course progress
	 */
	async getCourseProgress(membershipSlug: string, courseId: string): Promise<UserRoomProgress> {
		const response = (await apiClient.get(
			`/memberships/${membershipSlug}/courses/${courseId}/progress`
		)) as { data: UserRoomProgress };
		return response.data;
	},

	/**
	 * Update lesson progress (watch time)
	 */
	async updateLessonProgress(
		membershipSlug: string,
		lessonId: string,
		watchedSeconds: number
	): Promise<UserLessonProgress> {
		const response = (await apiClient.post(
			`/memberships/${membershipSlug}/lessons/${lessonId}/progress`,
			{ watchedSeconds }
		)) as { data: UserLessonProgress };
		return response.data;
	},

	/**
	 * Mark a lesson as complete
	 */
	async markLessonComplete(membershipSlug: string, lessonId: string): Promise<UserLessonProgress> {
		const response = (await apiClient.post(
			`/memberships/${membershipSlug}/lessons/${lessonId}/complete`
		)) as { data: UserLessonProgress };
		return response.data;
	},

	/**
	 * Mark a lesson as incomplete
	 */
	async markLessonIncomplete(membershipSlug: string, lessonId: string): Promise<UserLessonProgress> {
		const response = (await apiClient.post(
			`/memberships/${membershipSlug}/lessons/${lessonId}/incomplete`
		)) as { data: UserLessonProgress };
		return response.data;
	},

	/**
	 * Reset course progress
	 */
	async resetCourseProgress(membershipSlug: string, courseId: string): Promise<void> {
		await apiClient.delete(`/memberships/${membershipSlug}/courses/${courseId}/progress`);
	},

	// ═══════════════════════════════════════════════════════════════════════════
	// VIDEO ARCHIVE
	// ═══════════════════════════════════════════════════════════════════════════

	/**
	 * Get archived videos with pagination and filters
	 */
	async getArchivedVideos(
		membershipSlug: string,
		options: {
			page?: number;
			limit?: number;
			search?: string;
			month?: string;
			category?: string;
		} = {}
	): Promise<PaginatedLessons> {
		const params = new URLSearchParams();
		if (options.page) params.append('page', options.page.toString());
		if (options.limit) params.append('limit', options.limit.toString());
		if (options.search) params.append('search', options.search);
		if (options.month && options.month !== 'all') params.append('month', options.month);
		if (options.category && options.category !== 'all') params.append('category', options.category);

		const response = (await apiClient.get(
			`/memberships/${membershipSlug}/archive?${params.toString()}`
		)) as { data: PaginatedLessons };
		return response.data;
	},

	/**
	 * Get a specific archived video
	 */
	async getArchivedVideo(membershipSlug: string, videoId: string): Promise<LessonWithRelations> {
		const response = (await apiClient.get(
			`/memberships/${membershipSlug}/archive/${videoId}`
		)) as { data: LessonWithRelations };
		return response.data;
	},

	/**
	 * Increment video view count
	 */
	async recordVideoView(membershipSlug: string, videoId: string): Promise<void> {
		await apiClient.post(`/memberships/${membershipSlug}/archive/${videoId}/view`);
	},

	// ═══════════════════════════════════════════════════════════════════════════
	// NOTES & BOOKMARKS
	// ═══════════════════════════════════════════════════════════════════════════

	/**
	 * Save user notes for a lesson
	 */
	async saveLessonNotes(
		membershipSlug: string,
		lessonId: string,
		notes: string
	): Promise<{ notes: string }> {
		const response = (await apiClient.post(
			`/memberships/${membershipSlug}/lessons/${lessonId}/notes`,
			{ notes }
		)) as { data: { notes: string } };
		return response.data;
	},

	/**
	 * Get user notes for a lesson
	 */
	async getLessonNotes(membershipSlug: string, lessonId: string): Promise<{ notes: string }> {
		const response = (await apiClient.get(
			`/memberships/${membershipSlug}/lessons/${lessonId}/notes`
		)) as { data: { notes: string } };
		return response.data;
	},

	/**
	 * Bookmark a lesson
	 */
	async bookmarkLesson(membershipSlug: string, lessonId: string): Promise<void> {
		await apiClient.post(`/memberships/${membershipSlug}/lessons/${lessonId}/bookmark`);
	},

	/**
	 * Remove lesson bookmark
	 */
	async removeBookmark(membershipSlug: string, lessonId: string): Promise<void> {
		await apiClient.delete(`/memberships/${membershipSlug}/lessons/${lessonId}/bookmark`);
	},

	/**
	 * Get all bookmarked lessons
	 */
	async getBookmarkedLessons(membershipSlug: string): Promise<Lesson[]> {
		const response = (await apiClient.get(
			`/memberships/${membershipSlug}/bookmarks`
		)) as { data: Lesson[] };
		return response.data;
	},

	// ═══════════════════════════════════════════════════════════════════════════
	// ROOM CONTENT (via proxy endpoint)
	// ═══════════════════════════════════════════════════════════════════════════

	/**
	 * Get learning content for a specific room via the proxy endpoint
	 */
	async getRoomContent(roomSlug: string, params?: RoomContentParams): Promise<RoomContentResponse> {
		const token = typeof window !== 'undefined' ? getAuthToken() : null;
		const queryParams = new URLSearchParams();

		// Map room slug to membership ID
		const membershipId = roomSlugToMembershipId[roomSlug];
		if (membershipId) {
			queryParams.set('membership_id', membershipId.toString());
		}

		if (params) {
			if (params.page) queryParams.set('page', params.page.toString());
			if (params.limit) queryParams.set('limit', params.limit.toString());
			if (params.category_id) queryParams.set('category_id', params.category_id);
			if (params.type) queryParams.set('type', params.type);
			if (params.search) queryParams.set('search', params.search);
			if (params.instructor_id) queryParams.set('instructor_id', params.instructor_id);
			if (params.featured !== undefined) queryParams.set('featured', params.featured.toString());
			if (params.published !== undefined) queryParams.set('published', params.published.toString());
		}

		const qs = queryParams.toString();
		const url = `/api/learning-center${qs ? `?${qs}` : ''}`;

		const headers: Record<string, string> = {
			'Content-Type': 'application/json',
			Accept: 'application/json'
		};

		if (token) {
			headers['Authorization'] = `Bearer ${token}`;
		}

		const response = await fetch(url, { headers });

		if (!response.ok) {
			const errorData = await response.json().catch(() => ({ message: 'Request failed' }));
			throw new Error(errorData.message || `API Error: ${response.status}`);
		}

		return response.json();
	},

	/**
	 * Get learning content filtered by category for a room
	 */
	async getRoomContentByCategory(roomSlug: string, categoryId: string, params?: Omit<RoomContentParams, 'category_id'>): Promise<RoomContentResponse> {
		return this.getRoomContent(roomSlug, { ...params, category_id: categoryId });
	},

	/**
	 * Search learning content within a room
	 */
	async searchRoomContent(roomSlug: string, query: string, params?: Omit<RoomContentParams, 'search'>): Promise<RoomContentResponse> {
		return this.getRoomContent(roomSlug, { ...params, search: query });
	}
};

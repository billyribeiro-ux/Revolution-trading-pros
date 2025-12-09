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
import type {
	Course,
	CourseModule,
	Lesson,
	LessonData,
	LearningCenterData,
	CourseProgress,
	LessonProgress,
	VideoArchiveData,
	ArchivedVideo
} from '$lib/types/learning-center';

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
	async getCourse(membershipSlug: string, courseId: string): Promise<Course> {
		const response = (await apiClient.get(
			`/memberships/${membershipSlug}/courses/${courseId}`
		)) as { data: Course };
		return response.data;
	},

	/**
	 * Get course modules
	 */
	async getCourseModules(membershipSlug: string, courseId: string): Promise<CourseModule[]> {
		const response = (await apiClient.get(
			`/memberships/${membershipSlug}/courses/${courseId}/modules`
		)) as { data: CourseModule[] };
		return response.data;
	},

	// ═══════════════════════════════════════════════════════════════════════════
	// LESSONS
	// ═══════════════════════════════════════════════════════════════════════════

	/**
	 * Get a specific lesson with navigation context
	 */
	async getLesson(membershipSlug: string, lessonId: number): Promise<LessonData> {
		const response = (await apiClient.get(
			`/memberships/${membershipSlug}/lessons/${lessonId}`
		)) as { data: LessonData };
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
	async getCourseProgress(membershipSlug: string, courseId: string): Promise<CourseProgress> {
		const response = (await apiClient.get(
			`/memberships/${membershipSlug}/courses/${courseId}/progress`
		)) as { data: CourseProgress };
		return response.data;
	},

	/**
	 * Update lesson progress (watch time)
	 */
	async updateLessonProgress(
		membershipSlug: string,
		lessonId: number,
		watchedSeconds: number
	): Promise<LessonProgress> {
		const response = (await apiClient.post(
			`/memberships/${membershipSlug}/lessons/${lessonId}/progress`,
			{ watchedSeconds }
		)) as { data: LessonProgress };
		return response.data;
	},

	/**
	 * Mark a lesson as complete
	 */
	async markLessonComplete(membershipSlug: string, lessonId: number): Promise<LessonProgress> {
		const response = (await apiClient.post(
			`/memberships/${membershipSlug}/lessons/${lessonId}/complete`
		)) as { data: LessonProgress };
		return response.data;
	},

	/**
	 * Mark a lesson as incomplete
	 */
	async markLessonIncomplete(membershipSlug: string, lessonId: number): Promise<LessonProgress> {
		const response = (await apiClient.post(
			`/memberships/${membershipSlug}/lessons/${lessonId}/incomplete`
		)) as { data: LessonProgress };
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
	): Promise<VideoArchiveData> {
		const params = new URLSearchParams();
		if (options.page) params.append('page', options.page.toString());
		if (options.limit) params.append('limit', options.limit.toString());
		if (options.search) params.append('search', options.search);
		if (options.month && options.month !== 'all') params.append('month', options.month);
		if (options.category && options.category !== 'all') params.append('category', options.category);

		const response = (await apiClient.get(
			`/memberships/${membershipSlug}/archive?${params.toString()}`
		)) as { data: VideoArchiveData };
		return response.data;
	},

	/**
	 * Get a specific archived video
	 */
	async getArchivedVideo(membershipSlug: string, videoId: number): Promise<ArchivedVideo> {
		const response = (await apiClient.get(
			`/memberships/${membershipSlug}/archive/${videoId}`
		)) as { data: ArchivedVideo };
		return response.data;
	},

	/**
	 * Increment video view count
	 */
	async recordVideoView(membershipSlug: string, videoId: number): Promise<void> {
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
		lessonId: number,
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
	async getLessonNotes(membershipSlug: string, lessonId: number): Promise<{ notes: string }> {
		const response = (await apiClient.get(
			`/memberships/${membershipSlug}/lessons/${lessonId}/notes`
		)) as { data: { notes: string } };
		return response.data;
	},

	/**
	 * Bookmark a lesson
	 */
	async bookmarkLesson(membershipSlug: string, lessonId: number): Promise<void> {
		await apiClient.post(`/memberships/${membershipSlug}/lessons/${lessonId}/bookmark`);
	},

	/**
	 * Remove lesson bookmark
	 */
	async removeBookmark(membershipSlug: string, lessonId: number): Promise<void> {
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
	}
};

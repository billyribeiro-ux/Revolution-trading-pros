/**
 * Enhanced Courses API - Revolution Trading Pros
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Apple Principal Engineer ICT 7 Grade - January 2026
 *
 * API client for:
 * - Course Management (CRUD)
 * - Sections & Lessons
 * - Resources (PDF, DOC, etc.)
 * - Live Sessions
 * - Enrollments & Progress
 */

import { browser } from '$app/environment';

const API_BASE = browser ? (import.meta.env.VITE_API_URL || 'https://revolution-trading-pros-api.fly.dev') : 'https://revolution-trading-pros-api.fly.dev';

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

export interface Course {
	id: number;
	title: string;
	slug: string;
	subtitle?: string;
	description?: string;
	description_html?: string;
	thumbnail_url?: string;
	trailer_video_url?: string;
	difficulty_level: string;
	category?: string;
	tags: string[];
	instructor_id?: number;
	estimated_duration_minutes: number;
	formatted_duration: string;
	total_lessons: number;
	total_sections: number;
	is_published: boolean;
	is_featured: boolean;
	is_free: boolean;
	required_plan_id?: number;
	price_cents?: number;
	certificate_enabled: boolean;
	certificate_template?: string;
	completion_threshold_percent: number;
	sections?: CourseSection[];
	resources?: CourseResource[];
	live_sessions?: LiveSession[];
	created_at: string;
	updated_at?: string;
	published_at?: string;
}

export interface CourseSection {
	id: number;
	title: string;
	description?: string;
	sort_order: number;
	section_type: string;
	unlock_type: string;
	unlock_after_section_id?: number;
	unlock_date?: string;
	is_published: boolean;
	lesson_count: number;
	estimated_duration_minutes: number;
	lessons?: CourseLesson[];
	resources?: CourseResource[];
}

export interface CourseLesson {
	id: number;
	title: string;
	description?: string;
	content_html?: string;
	video_url?: string;
	embed_url?: string;
	bunny_video_guid?: string;
	thumbnail_url?: string;
	duration_seconds?: number;
	formatted_duration?: string;
	sort_order: number;
	lesson_type: string;
	is_preview: boolean;
	is_published: boolean;
	completion_type: string;
	required_watch_percent: number;
	resources?: CourseResource[];
}

export interface CourseResource {
	id: number;
	title: string;
	description?: string;
	file_url: string;
	file_name: string;
	file_type?: string;
	file_size_bytes?: number;
	formatted_size: string;
	version: string;
	download_count: number;
}

export interface LiveSession {
	id: number;
	title: string;
	description?: string;
	session_date: string;
	session_time?: string;
	formatted_date: string;
	video_url?: string;
	embed_url?: string;
	thumbnail_url?: string;
	duration_seconds?: number;
	formatted_duration?: string;
	is_published: boolean;
	sort_order: number;
}

export interface CourseEnrollment {
	id: number;
	user_id: number;
	enrolled_at: string;
	completed_lessons: number;
	total_lessons: number;
	progress_percent: number;
	is_completed: boolean;
	completed_at?: string;
	certificate_issued: boolean;
	certificate_url?: string;
	last_activity_at: string;
}

export interface LessonProgress {
	lesson_id: number;
	watch_position_seconds: number;
	watch_percent: number;
	is_completed: boolean;
	completed_at?: string;
}

export interface CourseProgress {
	enrollment: {
		enrolled_at: string;
		completed_lessons: number;
		total_lessons: number;
		progress_percent: number;
		is_completed: boolean;
		completed_at?: string;
		certificate_url?: string;
		last_lesson_id?: number;
		last_position_seconds: number;
	};
	lesson_progress: LessonProgress[];
}

export interface CreateCourseRequest {
	title: string;
	subtitle?: string;
	description?: string;
	description_html?: string;
	thumbnail_url?: string;
	trailer_video_url?: string;
	difficulty_level?: string;
	category?: string;
	tags?: string[];
	instructor_id?: number;
	is_published?: boolean;
	is_featured?: boolean;
	is_free?: boolean;
	required_plan_id?: number;
	price_cents?: number;
	prerequisite_course_ids?: number[];
	certificate_enabled?: boolean;
	completion_threshold_percent?: number;
}

export interface CreateSectionRequest {
	title: string;
	description?: string;
	section_type?: string;
	unlock_type?: string;
	unlock_after_section_id?: number;
	unlock_date?: string;
	is_published?: boolean;
}

export interface CreateLessonRequest {
	section_id: number;
	title: string;
	description?: string;
	content_html?: string;
	video_url?: string;
	bunny_video_guid?: string;
	thumbnail_url?: string;
	lesson_type?: string;
	is_preview?: boolean;
	is_published?: boolean;
	completion_type?: string;
	required_watch_percent?: number;
}

export interface CreateResourceRequest {
	section_id?: number;
	lesson_id?: number;
	title: string;
	description?: string;
	file_url: string;
	file_name: string;
	file_type?: string;
	file_size_bytes?: number;
	version?: string;
}

export interface CreateLiveSessionRequest {
	section_id?: number;
	title: string;
	description?: string;
	session_date: string;
	session_time?: string;
	video_url?: string;
	bunny_video_guid?: string;
	thumbnail_url?: string;
	replay_available_until?: string;
	is_published?: boolean;
}

export interface CourseListQuery {
	page?: number;
	per_page?: number;
	difficulty_level?: string;
	category?: string;
	instructor_id?: number;
	is_published?: boolean;
	is_featured?: boolean;
	search?: string;
}

export interface CourseStats {
	total_courses: number;
	published_courses: number;
	total_lessons: number;
	total_enrollments: number;
	completed_enrollments: number;
	completion_rate: number;
}

export interface ApiResult<T> {
	success: boolean;
	data?: T;
	error?: string;
}

// ═══════════════════════════════════════════════════════════════════════════
// API FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════

async function apiRequest<T>(
	endpoint: string,
	options: RequestInit = {}
): Promise<ApiResult<T>> {
	try {
		const response = await fetch(`${API_BASE}/admin/courses-enhanced${endpoint}`, {
			...options,
			headers: {
				'Content-Type': 'application/json',
				...options.headers
			},
			credentials: 'include'
		});

		const data = await response.json();

		if (!response.ok) {
			return { success: false, error: data.error || `HTTP ${response.status}` };
		}

		return { success: true, data };
	} catch (error) {
		return { success: false, error: String(error) };
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// COURSE API
// ═══════════════════════════════════════════════════════════════════════════

export const coursesApi = {
	list: (query: CourseListQuery = {}) => {
		const params = new URLSearchParams();
		Object.entries(query).forEach(([key, value]) => {
			if (value !== undefined) params.append(key, String(value));
		});
		return apiRequest<{ courses: Course[]; pagination: any }>(`?${params}`);
	},

	get: (courseId: number) => {
		return apiRequest<Course>(`/${courseId}`);
	},

	create: (data: CreateCourseRequest) => {
		return apiRequest<{ success: boolean; course: { id: number; title: string; slug: string } }>('', {
			method: 'POST',
			body: JSON.stringify(data)
		});
	},

	update: (courseId: number, data: Partial<CreateCourseRequest>) => {
		return apiRequest<{ success: boolean }>(`/${courseId}`, {
			method: 'PUT',
			body: JSON.stringify(data)
		});
	},

	delete: (courseId: number) => {
		return apiRequest<{ success: boolean }>(`/${courseId}`, {
			method: 'DELETE'
		});
	},

	clone: (courseId: number) => {
		return apiRequest<{ success: boolean; new_course: { id: number; title: string; slug: string } }>(
			`/${courseId}/clone`,
			{ method: 'POST' }
		);
	},

	getStats: () => {
		return apiRequest<CourseStats>('/stats');
	},

	getCategories: () => {
		return apiRequest<{ categories: { name: string; count: number }[] }>('/categories');
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// SECTION API
// ═══════════════════════════════════════════════════════════════════════════

export const sectionsApi = {
	create: (courseId: number, data: CreateSectionRequest) => {
		return apiRequest<{ success: boolean; section: { id: number; title: string } }>(
			`/${courseId}/sections`,
			{ method: 'POST', body: JSON.stringify(data) }
		);
	},

	update: (courseId: number, sectionId: number, data: Partial<CreateSectionRequest>) => {
		return apiRequest<{ success: boolean }>(`/${courseId}/sections/${sectionId}`, {
			method: 'PUT',
			body: JSON.stringify(data)
		});
	},

	delete: (courseId: number, sectionId: number) => {
		return apiRequest<{ success: boolean }>(`/${courseId}/sections/${sectionId}`, {
			method: 'DELETE'
		});
	},

	reorder: (courseId: number, items: { id: number; sort_order: number }[]) => {
		return apiRequest<{ success: boolean }>(`/${courseId}/sections/reorder`, {
			method: 'PUT',
			body: JSON.stringify({ items })
		});
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// LESSON API
// ═══════════════════════════════════════════════════════════════════════════

export const lessonsApi = {
	create: (courseId: number, data: CreateLessonRequest) => {
		return apiRequest<{ success: boolean; lesson: { id: number; title: string } }>(
			`/${courseId}/lessons`,
			{ method: 'POST', body: JSON.stringify(data) }
		);
	},

	update: (courseId: number, lessonId: number, data: Partial<CreateLessonRequest>) => {
		return apiRequest<{ success: boolean }>(`/${courseId}/lessons/${lessonId}`, {
			method: 'PUT',
			body: JSON.stringify(data)
		});
	},

	delete: (courseId: number, lessonId: number) => {
		return apiRequest<{ success: boolean }>(`/${courseId}/lessons/${lessonId}`, {
			method: 'DELETE'
		});
	},

	reorder: (courseId: number, sectionId: number, items: { id: number; sort_order: number }[]) => {
		return apiRequest<{ success: boolean }>(
			`/${courseId}/sections/${sectionId}/lessons/reorder`,
			{ method: 'PUT', body: JSON.stringify({ items }) }
		);
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// RESOURCE API
// ═══════════════════════════════════════════════════════════════════════════

export const resourcesApi = {
	create: (courseId: number, data: CreateResourceRequest) => {
		return apiRequest<{ success: boolean; resource: { id: number; title: string } }>(
			`/${courseId}/resources`,
			{ method: 'POST', body: JSON.stringify(data) }
		);
	},

	delete: (courseId: number, resourceId: number) => {
		return apiRequest<{ success: boolean }>(`/${courseId}/resources/${resourceId}`, {
			method: 'DELETE'
		});
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// LIVE SESSION API
// ═══════════════════════════════════════════════════════════════════════════

export const liveSessionsApi = {
	create: (courseId: number, data: CreateLiveSessionRequest) => {
		return apiRequest<{ success: boolean; session: { id: number; title: string } }>(
			`/${courseId}/live-sessions`,
			{ method: 'POST', body: JSON.stringify(data) }
		);
	},

	bulkCreate: (
		courseId: number,
		sectionId: number | null,
		sessions: { title: string; session_date: string; video_url?: string; bunny_video_guid?: string }[]
	) => {
		return apiRequest<{ success: boolean; created: number }>(`/${courseId}/live-sessions/bulk`, {
			method: 'POST',
			body: JSON.stringify({ section_id: sectionId, sessions })
		});
	},

	delete: (courseId: number, sessionId: number) => {
		return apiRequest<{ success: boolean }>(`/${courseId}/live-sessions/${sessionId}`, {
			method: 'DELETE'
		});
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// ENROLLMENT & PROGRESS API
// ═══════════════════════════════════════════════════════════════════════════

export const enrollmentsApi = {
	enroll: (courseId: number, userId: number, source?: string) => {
		return apiRequest<{ success: boolean; enrollment: { id: number } }>(`/${courseId}/enroll`, {
			method: 'POST',
			body: JSON.stringify({ user_id: userId, enrollment_source: source || 'admin' })
		});
	},

	list: (courseId: number, page = 1, perPage = 50) => {
		return apiRequest<{ enrollments: CourseEnrollment[]; pagination: any }>(
			`/${courseId}/enrollments?page=${page}&per_page=${perPage}`
		);
	},

	updateProgress: (
		courseId: number,
		lessonId: number,
		userId: number,
		watchPositionSeconds: number,
		watchPercent: number
	) => {
		return apiRequest<{ success: boolean; is_completed: boolean }>(
			`/${courseId}/lessons/${lessonId}/progress`,
			{
				method: 'POST',
				body: JSON.stringify({
					user_id: userId,
					watch_position_seconds: watchPositionSeconds,
					watch_percent: watchPercent
				})
			}
		);
	},

	getProgress: (courseId: number, userId: number) => {
		return apiRequest<CourseProgress>(`/${courseId}/progress/${userId}`);
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════

export function formatDuration(seconds: number): string {
	const hours = Math.floor(seconds / 3600);
	const minutes = Math.floor((seconds % 3600) / 60);
	const secs = seconds % 60;

	if (hours > 0) {
		return `${hours}:${minutes.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
	}
	return `${minutes}:${secs.toString().padStart(2, '0')}`;
}

export function formatFileSize(bytes: number): string {
	if (bytes < 1024) return `${bytes} B`;
	if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
	if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
	return `${(bytes / (1024 * 1024 * 1024)).toFixed(2)} GB`;
}

export function getDifficultyColor(level: string): string {
	switch (level.toLowerCase()) {
		case 'beginner':
			return '#22c55e';
		case 'intermediate':
			return '#f59e0b';
		case 'advanced':
			return '#ef4444';
		default:
			return '#6b7280';
	}
}

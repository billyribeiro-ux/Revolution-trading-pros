/**
 * Courses API Client
 * Apple Principal Engineer ICT 7 Grade - January 2026
 */

// ═══════════════════════════════════════════════════════════════════════════════════
// TYPE DEFINITIONS
// ═══════════════════════════════════════════════════════════════════════════════════

export interface Course {
	id: string;
	title: string;
	slug: string;
	description?: string;
	price_cents: number;
	instructor_id?: number;
	is_published: boolean;
	thumbnail?: string;
	preview_video_url?: string;
	duration_minutes?: number;
	level?: string;
	metadata?: Record<string, unknown>;
	created_at: string;
	updated_at: string;
	card_image_url?: string;
	card_description?: string;
	card_badge?: string;
	card_badge_color?: string;
	what_you_learn?: string[];
	requirements?: string[];
	target_audience?: string[];
	instructor_name?: string;
	instructor_title?: string;
	instructor_avatar_url?: string;
	instructor_bio?: string;
	is_free?: boolean;
	is_premium?: boolean;
	required_membership_ids?: number[];
	bunny_library_id?: number;
	meta_title?: string;
	meta_description?: string;
	og_image_url?: string;
	status?: string;
	published_at?: string;
	enrollment_count?: number;
	completion_rate?: number;
	avg_rating?: number;
	review_count?: number;
	module_count?: number;
	lesson_count?: number;
	total_duration_minutes?: number;
}

export interface CourseListItem {
	id: string;
	title: string;
	slug: string;
	description?: string;
	card_image_url?: string;
	card_description?: string;
	card_badge?: string;
	card_badge_color?: string;
	instructor_name?: string;
	instructor_avatar_url?: string;
	level?: string;
	price_cents: number;
	is_free?: boolean;
	is_published: boolean;
	status?: string;
	module_count?: number;
	lesson_count?: number;
	total_duration_minutes?: number;
	enrollment_count?: number;
	avg_rating?: number;
	review_count?: number;
	created_at: string;
}

export interface CourseModule {
	id: number;
	course_id: string;
	title: string;
	description?: string;
	sort_order: number;
	is_published: boolean;
	drip_enabled?: boolean;
	drip_days?: number;
	drip_date?: string;
	lesson_count?: number;
	total_duration_minutes?: number;
	created_at: string;
	updated_at: string;
	lessons?: LessonListItem[];
}

export interface Lesson {
	id: string;
	course_id: string;
	module_id?: number;
	title: string;
	slug: string;
	description?: string;
	video_url?: string;
	duration_minutes?: number;
	position: number;
	is_free: boolean;
	content?: string;
	created_at: string;
	updated_at: string;
	video_id?: number;
	bunny_video_guid?: string;
	thumbnail_url?: string;
	content_html?: string;
	resources?: unknown[];
	is_preview?: boolean;
	is_published?: boolean;
	drip_days?: number;
	prerequisite_lesson_ids?: string[];
	sort_order?: number;
}

export interface LessonListItem {
	id: string;
	course_id: string;
	module_id?: number;
	title: string;
	slug: string;
	description?: string;
	duration_minutes?: number;
	position: number;
	sort_order?: number;
	is_free: boolean;
	is_preview?: boolean;
	is_published?: boolean;
	bunny_video_guid?: string;
	thumbnail_url?: string;
}

export interface CourseDownload {
	id: number;
	course_id?: string;
	module_id?: number;
	lesson_id?: string;
	title: string;
	description?: string;
	file_name: string;
	file_path: string;
	file_size_bytes?: number;
	file_type?: string;
	mime_type?: string;
	bunny_file_id?: string;
	storage_zone?: string;
	download_url?: string;
	preview_url?: string;
	category?: string;
	sort_order?: number;
	is_public?: boolean;
	require_enrollment?: boolean;
	require_lesson_complete?: boolean;
	download_count?: number;
	uploaded_by?: number;
	created_at: string;
	updated_at: string;
}

export interface UserCourseEnrollment {
	id: number;
	user_id: number;
	course_id: string;
	current_module_id?: number;
	current_lesson_id?: string;
	completed_lesson_ids?: string[];
	progress_percent?: number;
	status?: string;
	enrolled_at: string;
	started_at?: string;
	completed_at?: string;
	last_accessed_at?: string;
	access_expires_at?: string;
	is_lifetime_access?: boolean;
	order_id?: number;
	price_paid_cents?: number;
	certificate_issued?: boolean;
	certificate_url?: string;
	certificate_issued_at?: string;
	notes?: unknown[];
	bookmarks?: unknown[];
}

export interface UserLessonProgress {
	id: number;
	user_id: number;
	lesson_id: string;
	enrollment_id: number;
	video_position_seconds?: number;
	video_duration_seconds?: number;
	video_watch_percent?: number;
	is_completed: boolean;
	completed_at?: string;
	time_spent_seconds?: number;
	view_count?: number;
	first_accessed_at: string;
	last_accessed_at: string;
}

export interface CourseReview {
	id: number;
	course_id: string;
	user_id: number;
	enrollment_id?: number;
	rating: number;
	title?: string;
	content?: string;
	is_verified_purchase?: boolean;
	is_featured?: boolean;
	is_approved?: boolean;
	helpful_count?: number;
	not_helpful_count?: number;
	created_at: string;
	updated_at: string;
}

export interface ModuleWithLessons extends CourseModule {
	lessons: LessonListItem[];
}

export interface CourseWithContent extends Course {
	modules: ModuleWithLessons[];
	downloads: CourseDownload[];
}

export interface CoursePlayerData {
	course: Course;
	modules: ModuleWithLessons[];
	downloads: CourseDownload[];
	enrollment?: UserCourseEnrollment;
	progress: UserLessonProgress[];
	is_enrolled: boolean;
}

export interface PaginatedCourses {
	courses: CourseListItem[];
	total: number;
	page: number;
	per_page: number;
	total_pages?: number;
}

export interface EnrollmentWithCourse {
	id: number;
	course_id: string;
	progress_percent: number;
	status: string;
	enrolled_at: string;
	last_accessed_at?: string;
	current_lesson_id?: string;
	course: CourseListItem;
}

// ═══════════════════════════════════════════════════════════════════════════════════
// REQUEST TYPES
// ═══════════════════════════════════════════════════════════════════════════════════

export interface CreateCourseRequest {
	title: string;
	slug?: string;
	description?: string;
	card_description?: string;
	card_image_url?: string;
	card_badge?: string;
	card_badge_color?: string;
	price_cents?: number;
	is_free?: boolean;
	level?: string;
	instructor_name?: string;
	instructor_title?: string;
	instructor_avatar_url?: string;
	instructor_bio?: string;
	what_you_learn?: string[];
	requirements?: string[];
	target_audience?: string[];
	meta_title?: string;
	meta_description?: string;
}

export interface UpdateCourseRequest extends Partial<CreateCourseRequest> {
	is_published?: boolean;
	status?: string;
	bunny_library_id?: number;
}

export interface CreateModuleRequest {
	title: string;
	description?: string;
	sort_order?: number;
	is_published?: boolean;
	drip_enabled?: boolean;
	drip_days?: number;
}

export interface UpdateModuleRequest extends Partial<CreateModuleRequest> {}

export interface CreateLessonRequest {
	title: string;
	slug?: string;
	description?: string;
	module_id?: number;
	video_url?: string;
	bunny_video_guid?: string;
	thumbnail_url?: string;
	duration_minutes?: number;
	content_html?: string;
	is_free?: boolean;
	is_preview?: boolean;
	is_published?: boolean;
	sort_order?: number;
	drip_days?: number;
}

export interface UpdateLessonRequest extends Partial<CreateLessonRequest> {}

export interface CreateDownloadRequest {
	title: string;
	description?: string;
	file_name: string;
	file_path: string;
	file_size_bytes?: number;
	file_type?: string;
	mime_type?: string;
	download_url?: string;
	category?: string;
	sort_order?: number;
	is_public?: boolean;
	course_id?: string;
	module_id?: number;
	lesson_id?: string;
}

export interface UpdateDownloadRequest {
	title?: string;
	description?: string;
	category?: string;
	sort_order?: number;
	is_public?: boolean;
}

export interface ReorderRequest {
	items: Array<{ id: string; sort_order: number }>;
}

export interface UpdateProgressRequest {
	lesson_id: string;
	video_position_seconds?: number;
	video_duration_seconds?: number;
	is_completed?: boolean;
}

export interface CourseQueryParams {
	status?: string;
	level?: string;
	is_free?: boolean;
	search?: string;
	page?: number;
	per_page?: number;
	sort_by?: string;
	sort_order?: string;
}

// ═══════════════════════════════════════════════════════════════════════════════════
// API RESPONSE WRAPPER
// ═══════════════════════════════════════════════════════════════════════════════════

interface ApiResponse<T> {
	success: boolean;
	data?: T;
	error?: string;
	message?: string;
}

// ═══════════════════════════════════════════════════════════════════════════════════
// ADMIN COURSES API
// ═══════════════════════════════════════════════════════════════════════════════════

export const adminCoursesApi = {
	// Courses
	async list(params?: CourseQueryParams): Promise<PaginatedCourses> {
		const searchParams = new URLSearchParams();
		if (params) {
			Object.entries(params).forEach(([key, value]) => {
				if (value !== undefined) searchParams.set(key, String(value));
			});
		}
		const res = await fetch(`/api/admin/courses?${searchParams}`);
		const data: ApiResponse<PaginatedCourses> = await res.json();
		if (!data.success) throw new Error(data.error || 'Failed to fetch courses');
		return data.data!;
	},

	async get(id: string): Promise<CourseWithContent> {
		const res = await fetch(`/api/admin/courses/${id}`);
		const data: ApiResponse<CourseWithContent> = await res.json();
		if (!data.success) throw new Error(data.error || 'Failed to fetch course');
		return data.data!;
	},

	async create(input: CreateCourseRequest): Promise<Course> {
		const res = await fetch('/api/admin/courses', {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify(input)
		});
		const data: ApiResponse<Course> = await res.json();
		if (!data.success) throw new Error(data.error || 'Failed to create course');
		return data.data!;
	},

	async update(id: string, input: UpdateCourseRequest): Promise<Course> {
		const res = await fetch(`/api/admin/courses/${id}`, {
			method: 'PUT',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify(input)
		});
		const data: ApiResponse<Course> = await res.json();
		if (!data.success) throw new Error(data.error || 'Failed to update course');
		return data.data!;
	},

	async delete(id: string): Promise<void> {
		const res = await fetch(`/api/admin/courses/${id}`, { method: 'DELETE' });
		const data: ApiResponse<void> = await res.json();
		if (!data.success) throw new Error(data.error || 'Failed to delete course');
	},

	async publish(id: string): Promise<Course> {
		const res = await fetch(`/api/admin/courses/${id}/publish`, { method: 'POST' });
		const data: ApiResponse<Course> = await res.json();
		if (!data.success) throw new Error(data.error || 'Failed to publish course');
		return data.data!;
	},

	async unpublish(id: string): Promise<Course> {
		const res = await fetch(`/api/admin/courses/${id}/unpublish`, { method: 'POST' });
		const data: ApiResponse<Course> = await res.json();
		if (!data.success) throw new Error(data.error || 'Failed to unpublish course');
		return data.data!;
	},

	// Modules
	async listModules(courseId: string): Promise<CourseModule[]> {
		const res = await fetch(`/api/admin/courses/${courseId}/modules`);
		const data: ApiResponse<CourseModule[]> = await res.json();
		if (!data.success) throw new Error(data.error || 'Failed to fetch modules');
		return data.data!;
	},

	async createModule(courseId: string, input: CreateModuleRequest): Promise<CourseModule> {
		const res = await fetch(`/api/admin/courses/${courseId}/modules`, {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify(input)
		});
		const data: ApiResponse<CourseModule> = await res.json();
		if (!data.success) throw new Error(data.error || 'Failed to create module');
		return data.data!;
	},

	async updateModule(
		courseId: string,
		moduleId: number,
		input: UpdateModuleRequest
	): Promise<CourseModule> {
		const res = await fetch(`/api/admin/courses/${courseId}/modules/${moduleId}`, {
			method: 'PUT',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify(input)
		});
		const data: ApiResponse<CourseModule> = await res.json();
		if (!data.success) throw new Error(data.error || 'Failed to update module');
		return data.data!;
	},

	async deleteModule(courseId: string, moduleId: number): Promise<void> {
		const res = await fetch(`/api/admin/courses/${courseId}/modules/${moduleId}`, {
			method: 'DELETE'
		});
		const data: ApiResponse<void> = await res.json();
		if (!data.success) throw new Error(data.error || 'Failed to delete module');
	},

	async reorderModules(courseId: string, items: ReorderRequest): Promise<void> {
		const res = await fetch(`/api/admin/courses/${courseId}/modules/reorder`, {
			method: 'PUT',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify(items)
		});
		const data: ApiResponse<void> = await res.json();
		if (!data.success) throw new Error(data.error || 'Failed to reorder modules');
	},

	// Lessons
	async listLessons(courseId: string): Promise<LessonListItem[]> {
		const res = await fetch(`/api/admin/courses/${courseId}/lessons`);
		const data: ApiResponse<LessonListItem[]> = await res.json();
		if (!data.success) throw new Error(data.error || 'Failed to fetch lessons');
		return data.data!;
	},

	async getLesson(
		courseId: string,
		lessonId: string
	): Promise<{ lesson: Lesson; downloads: CourseDownload[] }> {
		const res = await fetch(`/api/admin/courses/${courseId}/lessons/${lessonId}`);
		const data: ApiResponse<{ lesson: Lesson; downloads: CourseDownload[] }> = await res.json();
		if (!data.success) throw new Error(data.error || 'Failed to fetch lesson');
		return data.data!;
	},

	async createLesson(courseId: string, input: CreateLessonRequest): Promise<Lesson> {
		const res = await fetch(`/api/admin/courses/${courseId}/lessons`, {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify(input)
		});
		const data: ApiResponse<Lesson> = await res.json();
		if (!data.success) throw new Error(data.error || 'Failed to create lesson');
		return data.data!;
	},

	async updateLesson(
		courseId: string,
		lessonId: string,
		input: UpdateLessonRequest
	): Promise<Lesson> {
		const res = await fetch(`/api/admin/courses/${courseId}/lessons/${lessonId}`, {
			method: 'PUT',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify(input)
		});
		const data: ApiResponse<Lesson> = await res.json();
		if (!data.success) throw new Error(data.error || 'Failed to update lesson');
		return data.data!;
	},

	async deleteLesson(courseId: string, lessonId: string): Promise<void> {
		const res = await fetch(`/api/admin/courses/${courseId}/lessons/${lessonId}`, {
			method: 'DELETE'
		});
		const data: ApiResponse<void> = await res.json();
		if (!data.success) throw new Error(data.error || 'Failed to delete lesson');
	},

	async reorderLessons(courseId: string, items: ReorderRequest): Promise<void> {
		const res = await fetch(`/api/admin/courses/${courseId}/lessons/reorder`, {
			method: 'PUT',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify(items)
		});
		const data: ApiResponse<void> = await res.json();
		if (!data.success) throw new Error(data.error || 'Failed to reorder lessons');
	},

	// Downloads
	async listDownloads(courseId: string): Promise<CourseDownload[]> {
		const res = await fetch(`/api/admin/courses/${courseId}/downloads`);
		const data: ApiResponse<CourseDownload[]> = await res.json();
		if (!data.success) throw new Error(data.error || 'Failed to fetch downloads');
		return data.data!;
	},

	async createDownload(courseId: string, input: CreateDownloadRequest): Promise<CourseDownload> {
		const res = await fetch(`/api/admin/courses/${courseId}/downloads`, {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify(input)
		});
		const data: ApiResponse<CourseDownload> = await res.json();
		if (!data.success) throw new Error(data.error || 'Failed to create download');
		return data.data!;
	},

	async updateDownload(
		courseId: string,
		downloadId: number,
		input: UpdateDownloadRequest
	): Promise<CourseDownload> {
		const res = await fetch(`/api/admin/courses/${courseId}/downloads/${downloadId}`, {
			method: 'PUT',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify(input)
		});
		const data: ApiResponse<CourseDownload> = await res.json();
		if (!data.success) throw new Error(data.error || 'Failed to update download');
		return data.data!;
	},

	async deleteDownload(courseId: string, downloadId: number): Promise<void> {
		const res = await fetch(`/api/admin/courses/${courseId}/downloads/${downloadId}`, {
			method: 'DELETE'
		});
		const data: ApiResponse<void> = await res.json();
		if (!data.success) throw new Error(data.error || 'Failed to delete download');
	},

	async getUploadUrl(
		courseId: string,
		fileName: string,
		fileType: string
	): Promise<{
		upload_url: string;
		download_url: string;
		file_path: string;
		storage_zone: string;
		headers: Record<string, string>;
	}> {
		const res = await fetch(`/api/admin/courses/${courseId}/upload-url`, {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({ file_name: fileName, file_type: fileType })
		});
		const data = await res.json();
		if (!data.success) throw new Error(data.error || 'Failed to get upload URL');
		return data.data;
	}
};

// ═══════════════════════════════════════════════════════════════════════════════════
// PUBLIC COURSES API
// ═══════════════════════════════════════════════════════════════════════════════════

export const coursesApi = {
	async list(params?: CourseQueryParams): Promise<PaginatedCourses> {
		const searchParams = new URLSearchParams();
		if (params) {
			Object.entries(params).forEach(([key, value]) => {
				if (value !== undefined) searchParams.set(key, String(value));
			});
		}
		const res = await fetch(`/api/courses?${searchParams}`);
		const data: ApiResponse<PaginatedCourses> = await res.json();
		if (!data.success) throw new Error(data.error || 'Failed to fetch courses');
		return data.data!;
	},

	async get(slug: string): Promise<{ course: Course; modules: ModuleWithLessons[] }> {
		const res = await fetch(`/api/courses/${slug}`);
		const data: ApiResponse<{ course: Course; modules: ModuleWithLessons[] }> = await res.json();
		if (!data.success) throw new Error(data.error || 'Failed to fetch course');
		return data.data!;
	}
};

// ═══════════════════════════════════════════════════════════════════════════════════
// MY COURSES API (Authenticated)
// ═══════════════════════════════════════════════════════════════════════════════════

export const myCoursesApi = {
	async list(): Promise<EnrollmentWithCourse[]> {
		const res = await fetch('/api/my/courses');
		const data: ApiResponse<EnrollmentWithCourse[]> = await res.json();
		if (!data.success) throw new Error(data.error || 'Failed to fetch enrollments');
		return data.data!;
	},

	async getPlayer(slug: string): Promise<CoursePlayerData> {
		const res = await fetch(`/api/my/courses/${slug}/player`);
		const data: ApiResponse<CoursePlayerData> = await res.json();
		if (!data.success) throw new Error(data.error || 'Failed to fetch player data');
		return data.data!;
	},

	async updateProgress(slug: string, input: UpdateProgressRequest): Promise<UserLessonProgress> {
		const res = await fetch(`/api/my/courses/${slug}/progress`, {
			method: 'PUT',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify(input)
		});
		const data: ApiResponse<UserLessonProgress> = await res.json();
		if (!data.success) throw new Error(data.error || 'Failed to update progress');
		return data.data!;
	},

	async getDownloads(slug: string): Promise<{ downloads: CourseDownload[]; is_enrolled: boolean }> {
		const res = await fetch(`/api/my/courses/${slug}/downloads`);
		const data: ApiResponse<{ downloads: CourseDownload[]; is_enrolled: boolean }> =
			await res.json();
		if (!data.success) throw new Error(data.error || 'Failed to fetch downloads');
		return data.data!;
	}
};

/**
 * Learning Center Types
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Type definitions for the Learning Center feature, including courses,
 * modules, lessons, and progress tracking.
 *
 * @version 1.0.0 (December 2025)
 */

// ═══════════════════════════════════════════════════════════════════════════
// LESSON
// ═══════════════════════════════════════════════════════════════════════════

export interface LessonResource {
	id: string;
	name: string;
	url: string;
	type: 'pdf' | 'video' | 'link' | 'download';
	size?: string;
}

export interface Lesson {
	id: number;
	title: string;
	description: string;
	duration: string;
	durationSeconds: number;
	videoUrl: string;
	thumbnailUrl?: string;
	completed: boolean;
	locked: boolean;
	moduleId: number;
	moduleName: string;
	order: number;
	notes?: string;
	resources?: LessonResource[];
	watchedSeconds?: number;
	completedAt?: string;
}

// ═══════════════════════════════════════════════════════════════════════════
// MODULE
// ═══════════════════════════════════════════════════════════════════════════

export interface CourseModule {
	id: number;
	title: string;
	description: string;
	category: string;
	lessons: Lesson[];
	totalDuration: string;
	progress: number;
	order: number;
	locked: boolean;
}

// ═══════════════════════════════════════════════════════════════════════════
// COURSE
// ═══════════════════════════════════════════════════════════════════════════

export interface Course {
	id: string;
	slug: string;
	title: string;
	description: string;
	thumbnail?: string;
	modules: CourseModule[];
	totalLessons: number;
	totalDuration: string;
	progress: number;
	enrolledAt?: string;
	lastAccessedAt?: string;
}

// ═══════════════════════════════════════════════════════════════════════════
// PROGRESS
// ═══════════════════════════════════════════════════════════════════════════

export interface LessonProgress {
	lessonId: number;
	watchedSeconds: number;
	completed: boolean;
	completedAt?: string;
	lastWatchedAt: string;
}

export interface CourseProgress {
	courseId: string;
	lessonsCompleted: number;
	totalLessons: number;
	percentComplete: number;
	lastAccessedAt: string;
	lessonProgress: LessonProgress[];
}

// ═══════════════════════════════════════════════════════════════════════════
// VIDEO ARCHIVE
// ═══════════════════════════════════════════════════════════════════════════

export interface ArchivedVideo {
	id: number;
	title: string;
	description: string;
	duration: string;
	date: string;
	month: string;
	views: number;
	thumbnail?: string;
	videoUrl: string;
	category?: string;
	tags?: string[];
}

export interface VideoArchiveFilters {
	search: string;
	month: string;
	category: string;
}

// ═══════════════════════════════════════════════════════════════════════════
// CATEGORIES
// ═══════════════════════════════════════════════════════════════════════════

export interface LearningCategory {
	id: string;
	name: string;
	description?: string;
	icon?: string;
	courseCount?: number;
}

// ═══════════════════════════════════════════════════════════════════════════
// API RESPONSES
// ═══════════════════════════════════════════════════════════════════════════

export interface LearningCenterData {
	courses: Course[];
	categories: LearningCategory[];
	overallProgress: number;
	recentlyWatched: Lesson[];
}

export interface LessonData {
	lesson: Lesson;
	course: Course;
	module: CourseModule;
	previousLesson: Lesson | null;
	nextLesson: Lesson | null;
}

export interface VideoArchiveData {
	videos: ArchivedVideo[];
	totalCount: number;
	currentPage: number;
	totalPages: number;
	filters: VideoArchiveFilters;
}

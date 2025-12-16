/**
 * Learning Center Types
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Type definitions for the Learning Center system.
 * Each trading room/alert service has its own learning center with
 * room-specific educational content.
 *
 * @version 1.0.0 (December 2025)
 */

// ═══════════════════════════════════════════════════════════════════════════
// TRADING ROOM / SERVICE TYPES
// ═══════════════════════════════════════════════════════════════════════════

export type TradingRoomType = 'trading-room' | 'alert-service' | 'mastery' | 'course' | 'indicator';

export interface TradingRoom {
	id: string;
	slug: string;
	name: string;
	shortName?: string;
	description: string;
	type: TradingRoomType;
	icon: string;
	color?: string;
	tradingRoomUrl?: string;
	hasLearningCenter: boolean;
	hasArchive: boolean;
	hasDiscord: boolean;
	isActive: boolean;
	sortOrder: number;
	createdAt: string;
	updatedAt: string;
}

// ═══════════════════════════════════════════════════════════════════════════
// TRAINER TYPES
// ═══════════════════════════════════════════════════════════════════════════

export interface Trainer {
	id: string;
	slug: string;
	name: string;
	firstName: string;
	lastName: string;
	title: string;
	bio: string;
	shortBio?: string;
	imageUrl: string;
	thumbnailUrl?: string;
	socialLinks?: {
		twitter?: string;
		linkedin?: string;
		youtube?: string;
	};
	specialties: string[];
	isActive: boolean;
	sortOrder: number;
	createdAt: string;
	updatedAt: string;
}

// ═══════════════════════════════════════════════════════════════════════════
// CATEGORY TYPES
// ═══════════════════════════════════════════════════════════════════════════

export interface LessonCategory {
	id: string;
	slug: string;
	name: string;
	description?: string;
	icon?: string;
	color?: string;
	parentId?: string;
	sortOrder: number;
	isActive: boolean;
}

/**
 * Learning category used in store grouping
 */
export interface LearningCategory {
	id: string;
	name: string;
	slug?: string;
	description?: string;
	icon?: string;
	color?: string;
	sortOrder?: number;
}

// ═══════════════════════════════════════════════════════════════════════════
// LESSON TYPES
// ═══════════════════════════════════════════════════════════════════════════

export type LessonType = 'video' | 'article' | 'pdf' | 'quiz' | 'webinar-replay';
export type LessonStatus = 'draft' | 'published' | 'archived';
export type AccessLevel = 'free' | 'member' | 'premium';

export interface Lesson {
	id: string;
	slug: string;
	title: string;
	description: string;
	fullDescription?: string;

	// Content
	type: LessonType;
	videoUrl?: string;
	posterUrl?: string;
	thumbnailUrl?: string;
	articleContent?: string;
	pdfUrl?: string;
	duration?: string;

	// Relationships
	tradingRoomIds?: string[];
	trainerId?: string;
	categoryId?: string;
	tags?: string[];
	moduleId?: string;
	moduleOrder?: number;

	// Access & Status
	status?: LessonStatus;
	accessLevel?: AccessLevel;
	isFeatured?: boolean;
	isPinned?: boolean;

	// Ordering
	sortOrder?: number;

	// Progress tracking (merged from user progress)
	completed?: boolean;
	completedAt?: string;
	locked?: boolean;
	watchedSeconds?: number;
	notes?: string;
	progressPercent?: number;

	// Metadata
	publishedAt?: string;
	createdAt?: string;
	updatedAt?: string;

	// Analytics
	viewCount?: number;
	completionCount?: number;
	averageRating?: number;
}

// ═══════════════════════════════════════════════════════════════════════════
// MODULE TYPES (for organizing lessons into sections)
// ═══════════════════════════════════════════════════════════════════════════

export interface LessonModule {
	id: string;
	slug: string;
	tradingRoomId?: string;
	name: string;
	description?: string;
	sortOrder: number;
	isExpanded?: boolean;
	lessonCount?: number;
}

/**
 * Course module with lessons (used in store)
 */
export interface CourseModule extends LessonModule {
	lessons: Lesson[];
	category?: string;
	progress?: number;
}

// ═══════════════════════════════════════════════════════════════════════════
// COURSE TYPES
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Course with modules (used in learning center store)
 */
export interface Course {
	id: string;
	slug: string;
	name: string;
	shortName?: string;
	description: string;
	type?: TradingRoomType;
	icon?: string;
	color?: string;
	modules: CourseModule[];
	isActive?: boolean;
	sortOrder?: number;
	createdAt?: string;
	updatedAt?: string;
}

// ═══════════════════════════════════════════════════════════════════════════
// USER PROGRESS TYPES
// ═══════════════════════════════════════════════════════════════════════════

export interface UserLessonProgress {
	id: string;
	userId: string;
	lessonId: string;
	tradingRoomId: string;
	isCompleted: boolean;
	completedAt?: string;
	progressPercent: number;
	lastPosition?: number;
	isBookmarked: boolean;
	rating?: number;
	notes?: string;
	firstViewedAt: string;
	lastViewedAt: string;
}

export interface UserModuleProgress {
	moduleId: string;
	completedLessons: number;
	totalLessons: number;
	progressPercent: number;
}

export interface UserRoomProgress {
	tradingRoomId: string;
	completedLessons: number;
	totalLessons: number;
	progressPercent: number;
	moduleProgress: UserModuleProgress[];
}

export interface CourseProgress {
	courseId: string;
	userId: string;
	completedLessons: number;
	totalLessons: number;
	progressPercent: number;
	lastAccessedAt?: string;
	completedAt?: string;
}

export interface LessonProgress {
	lessonId: string;
	userId: string;
	isCompleted: boolean;
	progressPercent: number;
	lastPosition?: number;
	startedAt: string;
	completedAt?: string;
}

// ═══════════════════════════════════════════════════════════════════════════
// RESOURCE TYPES (downloadable files)
// ═══════════════════════════════════════════════════════════════════════════

export type ResourceType = 'pdf' | 'spreadsheet' | 'image' | 'video' | 'zip' | 'other';

export interface LessonResource {
	id: string;
	lessonId: string;
	name: string;
	description?: string;
	type: ResourceType;
	fileUrl: string;
	fileSize?: number;
	downloadCount?: number;
	sortOrder: number;
	createdAt: string;
}

// ═══════════════════════════════════════════════════════════════════════════
// ADMIN/UPLOAD TYPES
// ═══════════════════════════════════════════════════════════════════════════

export interface CreateLessonInput {
	title: string;
	description: string;
	fullDescription?: string;
	type: LessonType;
	videoUrl?: string;
	posterUrl?: string;
	thumbnailUrl?: string;
	duration?: string;
	tradingRoomIds: string[];
	trainerId: string;
	categoryId: string;
	tags?: string[];
	accessLevel?: AccessLevel;
	moduleId?: string;
	status?: LessonStatus;
}

export interface UpdateLessonInput extends Partial<CreateLessonInput> {
	id: string;
}

export interface LessonFilter {
	tradingRoomId?: string;
	trainerId?: string;
	categoryId?: string;
	moduleId?: string;
	type?: LessonType;
	status?: LessonStatus;
	accessLevel?: AccessLevel;
	search?: string;
	tags?: string[];
	isFeatured?: boolean;
}

// ═══════════════════════════════════════════════════════════════════════════
// API RESPONSE TYPES
// ═══════════════════════════════════════════════════════════════════════════

export interface LessonWithRelations extends Lesson {
	trainer?: Trainer;
	category?: LessonCategory;
	tradingRooms?: TradingRoom[];
	module?: LessonModule;
	resources?: LessonResource[];
	userProgress?: UserLessonProgress;
	// Navigation
	lesson?: Lesson;
	course?: Course;
	previousLesson?: Lesson;
	nextLesson?: Lesson;
}

/**
 * Learning center data response (used in store)
 */
export interface LearningCenterData {
	courses: Course[];
	categories: LearningCategory[];
	overallProgress: number;
	recentlyWatched: Lesson[];
	// Alternative shape for different API responses
	tradingRoom?: TradingRoom;
	modules?: LessonModule[];
	lessons?: LessonWithRelations[];
	trainers?: Trainer[];
	userProgress?: UserRoomProgress;
}

/**
 * Lesson data with full context (API response)
 */
export type LessonData = LessonWithRelations;

export interface PaginatedLessons {
	lessons: LessonWithRelations[];
	total: number;
	page: number;
	pageSize: number;
	totalPages: number;
}

// ═══════════════════════════════════════════════════════════════════════════
// VIDEO ARCHIVE TYPES
// ═══════════════════════════════════════════════════════════════════════════

export interface ArchivedVideo {
	id: string;
	title: string;
	description?: string;
	videoUrl: string;
	thumbnailUrl?: string;
	duration?: string;
	recordedAt: string;
	trainerId?: string;
	trainer?: Trainer;
	tags?: string[];
	views: number;
	viewCount?: number;
	createdAt: string;
}

export interface VideoArchiveData {
	videos: ArchivedVideo[];
	totalCount: number;
	currentPage: number;
	totalPages: number;
	total?: number;
	page?: number;
	pageSize?: number;
}

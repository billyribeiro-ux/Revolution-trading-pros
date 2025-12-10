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
	parentId?: string; // For nested categories
	sortOrder: number;
	isActive: boolean;
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
	duration?: string; // e.g., "45:00" for videos

	// Relationships
	tradingRoomIds: string[]; // Which rooms this lesson belongs to
	trainerId: string;
	categoryId: string;
	tags: string[];

	// Access & Status
	status: LessonStatus;
	accessLevel: AccessLevel;
	isFeatured: boolean;
	isPinned: boolean;

	// Ordering
	sortOrder: number;
	moduleId?: string; // For organizing into modules/sections
	moduleOrder?: number;

	// Metadata
	publishedAt?: string;
	createdAt: string;
	updatedAt: string;

	// Analytics (optional, populated separately)
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
	tradingRoomId: string;
	name: string;
	description?: string;
	sortOrder: number;
	isExpanded?: boolean; // UI state for accordion
	lessonCount?: number; // Computed
}

// ═══════════════════════════════════════════════════════════════════════════
// USER PROGRESS TYPES
// ═══════════════════════════════════════════════════════════════════════════

export interface UserLessonProgress {
	id: string;
	userId: string;
	lessonId: string;
	tradingRoomId: string;

	// Progress tracking
	isCompleted: boolean;
	completedAt?: string;
	progressPercent: number; // 0-100
	lastPosition?: number; // Video position in seconds

	// Engagement
	isBookmarked: boolean;
	rating?: number; // 1-5
	notes?: string;

	// Timestamps
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
	fileSize?: number; // bytes
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
}

export interface LearningCenterData {
	tradingRoom: TradingRoom;
	modules: LessonModule[];
	lessons: LessonWithRelations[];
	categories: LessonCategory[];
	trainers: Trainer[];
	userProgress?: UserRoomProgress;
}

export interface PaginatedLessons {
	lessons: LessonWithRelations[];
	total: number;
	page: number;
	pageSize: number;
	totalPages: number;
}

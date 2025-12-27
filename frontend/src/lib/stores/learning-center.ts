/**
 * Learning Center Store
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Svelte store for managing Learning Center state including courses,
 * lessons, progress tracking, and video archive.
 *
 * @version 1.0.0 (December 2025)
 */

import { writable, derived } from 'svelte/store';
import { learningCenterApi } from '$lib/api/learning-center';
import type {
	TradingRoom,
	LessonModule,
	Lesson,
	LessonWithRelations,
	LearningCenterData,
	LearningCategory,
	PaginatedLessons,
	Course,
	CourseModule
} from '$lib/types/learning-center';

// ═══════════════════════════════════════════════════════════════════════════
// STORE STATE
// ═══════════════════════════════════════════════════════════════════════════

interface LearningCenterState {
	// Course data
	courses: Course[];
	currentCourse: Course | null;
	categories: LearningCategory[];

	// Lesson data
	currentLesson: LessonWithRelations | null;
	currentModule: LessonModule | null;
	previousLesson: LessonWithRelations | null;
	nextLesson: LessonWithRelations | null;
	allLessons: LessonWithRelations[];

	// Video archive
	archivedVideos: LessonWithRelations[];
	archiveTotalCount: number;
	archiveCurrentPage: number;
	archiveTotalPages: number;

	// Progress
	overallProgress: number;
	recentlyWatched: LessonWithRelations[];

	// UI state
	isLoading: boolean;
	error: string | null;
	lastRefresh: Date | null;
	membershipSlug: string | null;
}

const initialState: LearningCenterState = {
	courses: [],
	currentCourse: null,
	categories: [],
	currentLesson: null,
	currentModule: null,
	previousLesson: null,
	nextLesson: null,
	allLessons: [],
	archivedVideos: [],
	archiveTotalCount: 0,
	archiveCurrentPage: 1,
	archiveTotalPages: 1,
	overallProgress: 0,
	recentlyWatched: [],
	isLoading: false,
	error: null,
	lastRefresh: null,
	membershipSlug: null
};

// ═══════════════════════════════════════════════════════════════════════════
// STORE FACTORY
// ═══════════════════════════════════════════════════════════════════════════

function createLearningCenterStore() {
	const { subscribe, set, update } = writable<LearningCenterState>(initialState);

	return {
		subscribe,

		// ═══════════════════════════════════════════════════════════════════════════
		// INITIALIZATION
		// ═══════════════════════════════════════════════════════════════════════════

		/**
		 * Initialize the store with membership data
		 */
		async init(membershipSlug: string) {
			update((state) => ({
				...state,
				isLoading: true,
				error: null,
				membershipSlug
			}));

			try {
				const data = await learningCenterApi.getCourses(membershipSlug);

				// LearningCenterData now has a flat structure with lessons already extracted
				const allLessons = data.lessons || [];

				update((state) => ({
					...state,
					courses: data.courses || [],
					categories: data.categories,
					overallProgress: data.userProgress?.progressPercent || 0,
					recentlyWatched: [], // TODO: Add recently watched logic if needed
					allLessons,
					isLoading: false,
					lastRefresh: new Date()
				}));
			} catch (error: any) {
				update((state) => ({
					...state,
					error: error.message || 'Failed to load learning center',
					isLoading: false
				}));
			}
		},

		// ═══════════════════════════════════════════════════════════════════════════
		// COURSES
		// ═══════════════════════════════════════════════════════════════════════════

		/**
		 * Load a specific course
		 */
		async loadCourse(courseId: string) {
			update((state) => ({ ...state, isLoading: true, error: null }));

			try {
				let membershipSlug: string | null = null;
				const unsubscribe = subscribe((state) => {
					membershipSlug = state.membershipSlug;
				});
				unsubscribe();

				if (!membershipSlug) {
					throw new Error('Membership not initialized');
				}

				const course = await learningCenterApi.getCourse(membershipSlug, courseId);

				update((state) => ({
					...state,
					currentCourse: course,
					isLoading: false
				}));
			} catch (error: any) {
				update((state) => ({
					...state,
					error: error.message || 'Failed to load course',
					isLoading: false
				}));
			}
		},

		// ═══════════════════════════════════════════════════════════════════════════
		// LESSONS
		// ═══════════════════════════════════════════════════════════════════════════

		/**
		 * Load a specific lesson with context
		 */
		async loadLesson(lessonId: string) {
			update((state) => ({ ...state, isLoading: true, error: null }));

			try {
				let membershipSlug: string | null = null;
				const unsubscribe = subscribe((state) => {
					membershipSlug = state.membershipSlug;
				});
				unsubscribe();

				if (!membershipSlug) {
					throw new Error('Membership not initialized');
				}

				const data = await learningCenterApi.getLesson(membershipSlug, lessonId);

				update((state) => ({
					...state,
					currentLesson: (data.lesson as LessonWithRelations) ?? null,
					currentCourse: (data.course as Course) ?? null,
					currentModule: (data.module as LessonModule) ?? null,
					previousLesson: (data.previousLesson as LessonWithRelations) ?? null,
					nextLesson: (data.nextLesson as LessonWithRelations) ?? null,
					isLoading: false
				}));
			} catch (error: any) {
				update((state) => ({
					...state,
					error: error.message || 'Failed to load lesson',
					isLoading: false
				}));
			}
		},

		/**
		 * Set the current lesson from local data (faster than API call)
		 */
		setCurrentLesson(lessonId: string) {
			update((state) => {
				const lesson = state.allLessons.find((l) => l.id === lessonId);
				if (!lesson) return state;

				const currentIndex = state.allLessons.findIndex((l) => l.id === lessonId);
				const previousLesson = currentIndex > 0 ? state.allLessons[currentIndex - 1] : null;
				const nextLesson =
					currentIndex < state.allLessons.length - 1 ? state.allLessons[currentIndex + 1] : null;

				// Module is already included in LessonWithRelations
				const currentModule = lesson.module || null;

				return {
					...state,
					currentLesson: lesson,
					currentModule,
					previousLesson,
					nextLesson
				};
			});
		},

		// ═══════════════════════════════════════════════════════════════════════════
		// PROGRESS
		// ═══════════════════════════════════════════════════════════════════════════

		/**
		 * Update lesson watch progress
		 */
		async updateProgress(lessonId: string, watchedSeconds: number) {
			let membershipSlug: string | null = null;
			const unsubscribe = subscribe((state) => {
				membershipSlug = state.membershipSlug;
			});
			unsubscribe();

			if (!membershipSlug) return;

			try {
				await learningCenterApi.updateLessonProgress(membershipSlug, lessonId, watchedSeconds);

				update((state) => ({
					...state,
					allLessons: state.allLessons.map((l) =>
						l.id === lessonId ? { ...l, watchedSeconds } : l
					),
					currentLesson:
						state.currentLesson?.id === lessonId
							? { ...state.currentLesson, watchedSeconds }
							: state.currentLesson
				}));
			} catch (error) {
				console.error('Failed to update progress:', error);
			}
		},

		/**
		 * Mark a lesson as complete
		 */
		async markComplete(lessonId: string) {
			let membershipSlug: string | null = null;
			const unsubscribe = subscribe((state) => {
				membershipSlug = state.membershipSlug;
			});
			unsubscribe();

			if (!membershipSlug) return;

			try {
				await learningCenterApi.markLessonComplete(membershipSlug, lessonId);

				update((state) => {
					// Update the lesson in allLessons
					const updatedLessons = state.allLessons.map((l) =>
						l.id === lessonId ? { ...l, completed: true, completedAt: new Date().toISOString() } : l
					);

					// Recalculate overall progress
					const completedCount = updatedLessons.filter((l) => l.completed).length;
					const overallProgress = Math.round((completedCount / updatedLessons.length) * 100);

					// Update module progress in courses
					const updatedCourses = state.courses.map((course) => ({
						...course,
						modules: course.modules.map((module) => {
							const moduleLessons = updatedLessons.filter((l) => l.moduleId === module.id);
							const moduleCompleted = moduleLessons.filter((l) => l.completed).length;
							return {
								...module,
								progress: Math.round((moduleCompleted / moduleLessons.length) * 100),
								lessons: module.lessons.map((l) =>
									l.id === lessonId ? { ...l, completed: true } : l
								)
							};
						})
					}));

					return {
						...state,
						allLessons: updatedLessons,
						courses: updatedCourses,
						overallProgress,
						currentLesson:
							state.currentLesson?.id === lessonId
								? { ...state.currentLesson, completed: true }
								: state.currentLesson
					};
				});
			} catch (error: any) {
				update((state) => ({
					...state,
					error: error.message || 'Failed to mark lesson complete'
				}));
			}
		},

		/**
		 * Mark a lesson as incomplete
		 */
		async markIncomplete(lessonId: string) {
			let membershipSlug: string | null = null;
			const unsubscribe = subscribe((state) => {
				membershipSlug = state.membershipSlug;
			});
			unsubscribe();

			if (!membershipSlug) return;

			try {
				await learningCenterApi.markLessonIncomplete(membershipSlug, lessonId);

				update((state) => {
					const updatedLessons = state.allLessons.map((l) =>
						l.id === lessonId ? { ...l, completed: false, completedAt: undefined } : l
					);

					const completedCount = updatedLessons.filter((l) => l.completed).length;
					const overallProgress = Math.round((completedCount / updatedLessons.length) * 100);

					return {
						...state,
						allLessons: updatedLessons,
						overallProgress,
						currentLesson:
							state.currentLesson?.id === lessonId
								? { ...state.currentLesson, completed: false }
								: state.currentLesson
					};
				});
			} catch (error: any) {
				update((state) => ({
					...state,
					error: error.message || 'Failed to mark lesson incomplete'
				}));
			}
		},

		// ═══════════════════════════════════════════════════════════════════════════
		// VIDEO ARCHIVE
		// ═══════════════════════════════════════════════════════════════════════════

		/**
		 * Load archived videos
		 */
		async loadArchive(options: {
			page?: number;
			limit?: number;
			search?: string;
			month?: string;
			category?: string;
		} = {}) {
			update((state) => ({ ...state, isLoading: true, error: null }));

			try {
				let membershipSlug: string | null = null;
				const unsubscribe = subscribe((state) => {
					membershipSlug = state.membershipSlug;
				});
				unsubscribe();

				if (!membershipSlug) {
					throw new Error('Membership not initialized');
				}

				const data = await learningCenterApi.getArchivedVideos(membershipSlug, options);

				update((state) => ({
					...state,
					archivedVideos: data.lessons,
					archiveTotalCount: data.total,
					archiveCurrentPage: data.page,
					archiveTotalPages: data.totalPages,
					isLoading: false
				}));
			} catch (error: any) {
				update((state) => ({
					...state,
					error: error.message || 'Failed to load video archive',
					isLoading: false
				}));
			}
		},

		/**
		 * Record a video view
		 */
		async recordView(videoId: string) {
			let membershipSlug: string | null = null;
			const unsubscribe = subscribe((state) => {
				membershipSlug = state.membershipSlug;
			});
			unsubscribe();

			if (!membershipSlug) return;

			try {
				await learningCenterApi.recordVideoView(membershipSlug, videoId);

				update((state) => ({
					...state,
					archivedVideos: state.archivedVideos.map((v) =>
						v.id === videoId ? { ...v, views: ((v as unknown as { views?: number }).views ?? 0) + 1 } : v
					)
				}));
			} catch (error) {
				console.error('Failed to record view:', error);
			}
		},

		// ═══════════════════════════════════════════════════════════════════════════
		// NOTES & BOOKMARKS
		// ═══════════════════════════════════════════════════════════════════════════

		/**
		 * Save notes for current lesson
		 */
		async saveNotes(lessonId: string, notes: string) {
			let membershipSlug: string | null = null;
			const unsubscribe = subscribe((state) => {
				membershipSlug = state.membershipSlug;
			});
			unsubscribe();

			if (!membershipSlug) return;

			try {
				await learningCenterApi.saveLessonNotes(membershipSlug, lessonId, notes);

				update((state) => ({
					...state,
					currentLesson:
						state.currentLesson?.id === lessonId
							? { ...state.currentLesson, notes }
							: state.currentLesson
				}));
			} catch (error: any) {
				update((state) => ({
					...state,
					error: error.message || 'Failed to save notes'
				}));
			}
		},

		// ═══════════════════════════════════════════════════════════════════════════
		// UTILITY
		// ═══════════════════════════════════════════════════════════════════════════

		/**
		 * Clear any errors
		 */
		clearError() {
			update((state) => ({ ...state, error: null }));
		},

		/**
		 * Reset the store to initial state
		 */
		reset() {
			set(initialState);
		}
	};
}

// ═══════════════════════════════════════════════════════════════════════════
// STORE INSTANCE
// ═══════════════════════════════════════════════════════════════════════════

export const learningCenterStore = createLearningCenterStore();

// ═══════════════════════════════════════════════════════════════════════════
// DERIVED STORES
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Get courses filtered by category
 */
export const coursesByCategory = derived(learningCenterStore, ($store) => {
	const grouped: Record<string, Course[]> = {};
	$store.courses.forEach((course) => {
		course.modules.forEach((module: CourseModule) => {
			const category = module.category ?? 'uncategorized';
			if (!grouped[category]) {
				grouped[category] = [];
			}
			if (!grouped[category].some((c) => c.id === course.id)) {
				grouped[category].push(course);
			}
		});
	});
	return grouped;
});

/**
 * Get incomplete lessons
 */
export const incompleteLessons = derived(learningCenterStore, ($store) =>
	$store.allLessons.filter((lesson) => !lesson.userProgress?.isCompleted)
);

/**
 * Get next lesson to continue
 */
export const nextLessonToContinue = derived(learningCenterStore, ($store) => {
	// First check recently watched
	if ($store.recentlyWatched.length > 0) {
		const recent = $store.recentlyWatched[0];
		if (!recent.userProgress?.isCompleted) return recent;
	}

	// Otherwise find first incomplete lesson
	return $store.allLessons.find((lesson) => !lesson.userProgress?.isCompleted) || null;
});

/**
 * Get module progress for current course
 */
export const moduleProgress = derived(learningCenterStore, ($store) => {
	if (!$store.currentModule) return 0;

	const moduleLessons = $store.allLessons.filter(
		(l) => l.moduleId === $store.currentModule?.id
	);
	const completed = moduleLessons.filter((l) => l.userProgress?.isCompleted).length;

	return moduleLessons.length > 0 ? Math.round((completed / moduleLessons.length) * 100) : 0;
});

/**
 * Page Builder Type Definitions
 * Apple Principal Engineer ICT 7 Grade - January 2026
 * 
 * Core types for the drag-and-drop course page builder system.
 * Designed for extensibility and type safety.
 */

// ═══════════════════════════════════════════════════════════════════════════════
// COMPONENT TYPES
// ═══════════════════════════════════════════════════════════════════════════════

export type ComponentType = 
	| 'course-header'
	| 'video-player'
	| 'video-stack'
	| 'class-downloads'
	| 'spacer'
	| 'divider';

// ═══════════════════════════════════════════════════════════════════════════════
// COMPONENT CONFIGURATIONS
// ═══════════════════════════════════════════════════════════════════════════════

export interface CourseHeaderConfig {
	title: string;
	subtitle?: string;
	description?: string;
	instructorName?: string;
	instructorTitle?: string;
	instructorAvatar?: string;
	showLoginButton?: boolean;
	loginButtonText?: string;
	loginButtonUrl?: string;
	backgroundColor?: string;
	textColor?: string;
}

export interface VideoPlayerConfig {
	videoId?: string;
	bunnyVideoGuid?: string;
	bunnyLibraryId?: number;
	title: string;
	subtitle?: string;
	thumbnailUrl?: string;
	duration?: number;
	autoplay?: boolean;
}

export interface VideoStackConfig {
	videos: VideoPlayerConfig[];
	showDates?: boolean;
	sortOrder?: 'newest' | 'oldest' | 'manual';
}

export interface ClassDownloadsConfig {
	courseId?: string;
	courseSlug?: string;
	title?: string;
	maxHeight?: string;
}

export interface SpacerConfig {
	height: number; // in pixels
}

export interface DividerConfig {
	style?: 'solid' | 'dashed' | 'dotted';
	color?: string;
	thickness?: number;
	marginTop?: number;
	marginBottom?: number;
}

// Union type for all configs
export type ComponentConfig = 
	| CourseHeaderConfig
	| VideoPlayerConfig
	| VideoStackConfig
	| ClassDownloadsConfig
	| SpacerConfig
	| DividerConfig;

// ═══════════════════════════════════════════════════════════════════════════════
// PAGE BUILDER BLOCK
// ═══════════════════════════════════════════════════════════════════════════════

export interface PageBlock {
	id: string;
	type: ComponentType;
	config: ComponentConfig;
	order: number;
}

// ═══════════════════════════════════════════════════════════════════════════════
// PAGE LAYOUT
// ═══════════════════════════════════════════════════════════════════════════════

export interface PageLayout {
	id?: string;
	courseId?: string;
	courseSlug?: string;
	title: string;
	blocks: PageBlock[];
	createdAt?: string;
	updatedAt?: string;
	publishedAt?: string;
	status: 'draft' | 'published' | 'archived';
}

// ═══════════════════════════════════════════════════════════════════════════════
// COMPONENT REGISTRY ENTRY
// ═══════════════════════════════════════════════════════════════════════════════

export interface ComponentRegistryEntry {
	type: ComponentType;
	name: string;
	description: string;
	icon: string;
	category: 'content' | 'media' | 'layout';
	defaultConfig: ComponentConfig;
}

// ═══════════════════════════════════════════════════════════════════════════════
// DRAG STATE
// ═══════════════════════════════════════════════════════════════════════════════

export interface DragState {
	isDragging: boolean;
	draggedType: ComponentType | null;
	draggedBlockId: string | null;
	dropTargetIndex: number | null;
}

// ═══════════════════════════════════════════════════════════════════════════════
// BUILDER STATE
// ═══════════════════════════════════════════════════════════════════════════════

export interface BuilderState {
	layout: PageLayout;
	selectedBlockId: string | null;
	dragState: DragState;
	isPreviewMode: boolean;
	isSaving: boolean;
	hasUnsavedChanges: boolean;
}

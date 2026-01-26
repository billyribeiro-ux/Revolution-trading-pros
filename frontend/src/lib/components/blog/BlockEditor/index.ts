/**
 * Block Editor - Enterprise-Grade Blog Editor
 * ===========================================
 * A comprehensive block-based content editor with
 * advanced features rivaling WordPress Elementor Pro.
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 *
 * Features:
 * - 30+ block types (text, media, layout, trading, AI)
 * - Drag & drop reordering
 * - Undo/Redo history
 * - Device preview (desktop/tablet/mobile)
 * - AI-powered content generation
 * - Real-time SEO analysis
 * - Version control with revision history
 * - Full keyboard shortcuts
 * - Media library with editing
 * - Collaboration features
 */

// Main Editor Component
export { default as BlockEditor } from './BlockEditor.svelte';

// Sub-components
export { default as BlockInserter } from './BlockInserter.svelte';
export { default as BlockRenderer } from './BlockRenderer.svelte';
// BlockToolbar retired 2026-01-26 - zero imports found
export { default as BlockSettingsPanel } from './BlockSettingsPanel.svelte';

// AI & Content
export { default as AIAssistant } from './AIAssistant.svelte';

// SEO & Analytics
export { default as SEOAnalyzer } from './SEOAnalyzer.svelte';

// Version Control
export { default as RevisionHistory } from './RevisionHistory.svelte';

// Media Management
// MediaLibrary retired 2026-01-26 - zero imports found

// Keyboard & Accessibility
export { default as KeyboardShortcuts } from './KeyboardShortcuts.svelte';

// Collaboration
// CollaborationPanel retired 2026-01-26 - zero imports found

// Type Exports
export type {
	Block,
	BlockType,
	BlockContent,
	BlockSettings,
	BlockMetadata,
	EditorState,
	SEOAnalysis,
	SEOIssue,
	AIWritingRequest,
	AIWritingResponse,
	Revision,
	RevisionDiff
} from './types';

// Constants Export
export { BLOCK_CATEGORIES, BLOCK_DEFINITIONS } from './types';

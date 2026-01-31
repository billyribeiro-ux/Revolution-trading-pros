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

// Error Boundary
export { default as BlockErrorBoundary } from './BlockErrorBoundary.svelte';

// Virtual Scrolling for Large Documents
export { default as VirtualBlockList } from './VirtualBlockList.svelte';
export type {
	VirtualBlockListProps,
	VisibleRange,
	BlockMeasurement,
	PerformanceMetrics
} from './VirtualBlockList.svelte';

// AI & Content
export { default as AIAssistant } from './AIAssistant.svelte';

// SEO & Analytics
export { default as SEOAnalyzer } from './SEOAnalyzer.svelte';

// Version Control
export { default as RevisionHistory } from './RevisionHistory.svelte';

// Media Management
// MediaLibrary retired 2026-01-26 - zero imports found
export { default as ImageUploader } from './ImageUploader.svelte';

// Image Upload Utilities
export {
	uploadImage,
	uploadImages,
	UploadController,
	formatFileSize,
	isValidImageType,
	generateUniqueFilename
} from './upload/uploader';

export type {
	UploadOptions,
	UploadResult,
	UploadProgress,
	BatchUploadResult
} from './upload/uploader';

// Image Processing Utilities
export {
	processImage,
	createThumbnail,
	canProcessImage,
	getImageDimensions,
	getOptimalQuality,
	extractDominantColor
} from './upload/image-processor';

export type {
	ProcessOptions,
	ProcessedImage,
	ImageDimensions
} from './upload/image-processor';

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

// Error Handling Utilities
export {
	// Error Types
	ErrorType,
	ErrorSeverity,
	// Core Functions
	captureBlockError,
	recoverBlock,
	serializeError,
	isRecoverable,
	getErrorMessage,
	classifyError,
	determineErrorSeverity,
	// Store Management
	errorTracking,
	getBlockErrors,
	markErrorRecovered,
	clearBlockErrors,
	clearAllErrors,
	// Utilities
	createDefaultBlock,
	isBlockStateValid
} from './error-handling';

export type {
	ErrorTypeValue,
	ErrorSeverityValue,
	SerializedError,
	BlockErrorContext,
	BlockErrorRecord,
	ErrorTrackingState
} from './error-handling';

// Performance Monitoring
export { default as PerformanceOverlay } from './PerformanceOverlay.svelte';

// Performance Metrics
export {
	initializeMetrics,
	measureBlockRender,
	measureOperation,
	measureAsync,
	recordDragDropLatency,
	recordKeystrokeLatency,
	recordSaveOperation,
	recordAIResponse,
	setBlockCount,
	getMetrics,
	getPercentiles,
	getBlockRenderStats,
	subscribeToMetrics,
	resetMetrics,
	destroyMetrics,
	getWebVitalRating,
	getWebVitalThresholds,
	EDITOR_METRIC_NAMES,
	WEB_VITAL_THRESHOLDS
} from './performance/metrics';

export type {
	WebVitalName,
	WebVitalMetric,
	EditorMetric,
	BlockRenderMetric,
	OperationMetric,
	MemoryMetrics,
	PercentileResult,
	MetricsSnapshot
} from './performance/metrics';

// Performance Reporter
export {
	initializeReporter,
	setReporterUserContext,
	setReporterPostContext,
	flushMetrics,
	updateReporterConfig,
	destroyReporter
} from './performance/reporter';

export type {
	ReporterConfig,
	MetricsPayload,
	WebVitalsPayload,
	EditorMetricsPayload,
	BlockRenderStatsPayload,
	OperationStatsPayload,
	MemoryUsagePayload,
	ConnectionInfo,
	PrivacySettings
} from './performance/reporter';

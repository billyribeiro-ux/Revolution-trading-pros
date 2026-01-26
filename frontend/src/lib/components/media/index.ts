/**
 * Media Components Index
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Export all media-related components for the Image Optimization Engine.
 *
 * New v2.0 Components:
 * - OptimizedImage: High-performance image with BlurHash/LQIP placeholders
 * - DropZone: Drag-and-drop file upload zone
 * - UploadProgress: Visual upload progress panel
 * - OptimizationStats: Display optimization statistics
 * - ImageCropModal: Image cropping and editing modal
 * - ResponsivePreview: Preview responsive image variants
 * - MediaSkeleton: Content-shaped skeleton loaders
 */

// Original components - retired 2026-01-26 (zero imports):
// MediaUpload, MediaGrid, MediaPreview, MediaAnalytics

// New v2.0 components
export { default as OptimizedImage } from './OptimizedImage.svelte';
export { default as DropZone } from './DropZone.svelte';
export { default as UploadProgress } from './UploadProgress.svelte';
// OptimizationStats retired 2026-01-26 - zero imports found
export { default as ImageCropModal } from './ImageCropModal.svelte';
export { default as ResponsivePreview } from './ResponsivePreview.svelte';
export { default as MediaSkeleton } from './MediaSkeleton.svelte';

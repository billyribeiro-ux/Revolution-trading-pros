/**
 * Banner Template System Exports
 *
 * Complete template system for consent banners with:
 * - 20 pre-built templates
 * - Custom template creation
 * - Live preview and editing
 * - Mobile/tablet responsive
 * - Backend sync support
 *
 * @module consent/templates
 * @version 1.0.0
 */

// Types
export type {
	TemplatePosition,
	TemplateStyle,
	ButtonVariant,
	AnimationType,
	ColorScheme,
	Typography,
	Spacing,
	MobileConfig,
	TabletConfig,
	TemplateCopy,
	BannerTemplate,
	TemplateCustomization,
	ActiveTemplateConfig,
	TemplatePreview
} from './types';

// Registry
export {
	BANNER_TEMPLATES,
	getTemplate,
	getTemplatesByCategory,
	getTemplateCategories,
	getTemplatePreviews,
	DEFAULT_TEMPLATE_ID
} from './registry';

// Store
export {
	activeTemplate,
	allTemplates,
	previewTemplate,
	isPreviewMode,
	initializeTemplateStore,
	setActiveTemplate,
	updateCustomization,
	clearCustomization,
	saveAsCustomTemplate,
	updateCustomTemplate,
	deleteCustomTemplate,
	enterPreviewMode,
	exitPreviewMode,
	applyPreview,
	getActiveTemplateConfig,
	getCurrentTemplate,
	exportTemplateConfig,
	importTemplateConfig
} from './store';

// Components
export { default as BannerRenderer } from './BannerRenderer.svelte';
export { default as TemplatePreviewCard } from './TemplatePreviewCard.svelte';
export { default as TemplateEditor } from './TemplateEditor.svelte';

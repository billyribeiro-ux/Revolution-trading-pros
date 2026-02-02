/**
 * Block Hooks Index
 * ═══════════════════════════════════════════════════════════════════════════
 * Export all composable hooks for block components
 */

// Media Controls Hook
export { useMediaControls, type MediaControlsOptions } from './useMediaControls.svelte';

// AI Generation Hook
export {
	useAIGeneration,
	type AIGenerationOptions,
	type AIGenerationType,
	type AIGenerationConfig,
	type TranslationConfig
} from './useAIGeneration.svelte';

// Block Validation Hook
export {
	useBlockValidation,
	PRESET_RULES,
	BLOCK_VALIDATION_RULES,
	type ValidationRule,
	type ValidationError,
	type ValidationResult,
	type ValidationOptions
} from './useBlockValidation.svelte';

// Image Upload Hook
export {
	useImageUpload,
	createPresignedUploader,
	dataUrlToFile,
	compressImage,
	type ImageUploadOptions,
	type ImageMetadata,
	type UploadState
} from './useImageUpload.svelte';

// Keyboard Shortcuts Hook
export {
	useKeyboardShortcuts,
	EDITOR_SHORTCUTS,
	NAVIGATION_SHORTCUTS,
	GLOBAL_SHORTCUTS,
	generateShortcutsHelp,
	type Shortcut,
	type ShortcutGroup,
	type KeyboardShortcutsOptions
} from './useKeyboardShortcuts.svelte';

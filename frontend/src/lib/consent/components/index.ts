/**
 * Consent Components Export
 *
 * Re-exports all consent-related Svelte components.
 *
 * @module consent/components
 */

export { default as ConsentBanner } from './ConsentBanner.svelte';
export { default as ConsentPreferencesModal } from './ConsentPreferencesModal.svelte';
export { default as ConsentSettingsButton } from './ConsentSettingsButton.svelte';
export { default as ContentPlaceholder } from './ContentPlaceholder.svelte';

// Template-based components
export { default as BannerRenderer } from '../templates/BannerRenderer.svelte';
export { default as TemplatePreviewCard } from '../templates/TemplatePreviewCard.svelte';
export { default as TemplateEditor } from '../templates/TemplateEditor.svelte';

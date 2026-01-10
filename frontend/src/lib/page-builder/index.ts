/**
 * Page Builder Module Exports
 * Apple Principal Engineer ICT 11 Grade - January 2026
 */

// Types
export * from './types';

// Registry
export { componentRegistry, getComponentByType, getComponentsByCategory, createDefaultConfig } from './registry';

// Store
export { createBuilderStore, type BuilderStore } from './store.svelte';

// Components
export { default as DynamicRenderer } from './components/DynamicRenderer.svelte';

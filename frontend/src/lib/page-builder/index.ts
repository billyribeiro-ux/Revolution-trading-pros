/**
 * Page Builder Module Index
 * Apple Principal Engineer ICT 7 Grade - January 2026
 * 
 * Central export point for all page builder functionality.
 */

// Types
export * from './types';

// Registry
export { componentRegistry, getComponentByType, getComponentsByCategory, createDefaultConfig } from './registry';

// Store
export { createBuilderStore, type BuilderStore } from './store.svelte';

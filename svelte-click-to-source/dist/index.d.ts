import { Plugin } from 'vite';
import { PreprocessorGroup } from 'svelte/compiler';

/**
 * Svelte Preprocessor for click-to-source
 * Adds data-source="filepath:line:col" to HTML elements in the markup section only.
 *
 * Skips:
 * - <script> and <style> blocks (avoids breaking TypeScript generics)
 * - Special elements: <title>, <script>, <style>, <svelte:*>, <slot>
 * - Self-closing tags inside excluded zones
 */

interface PreprocessorOptions {
    enabled?: boolean;
}
declare function createPreprocessor(options?: PreprocessorOptions): PreprocessorGroup;

/**
 * svelte-click-to-source
 *
 * Alt+Click any element in dev mode → Opens source in your editor
 *
 * Architecture:
 * 1. Preprocessor adds data-source="file:line:col" to HTML elements
 * 2. Runtime script highlights elements on hotkey+hover, sends fetch to dev server on click
 * 3. Vite middleware receives the request and opens the file via CLI command
 */

interface ClickToSourceOptions {
    hotkey?: 'alt' | 'ctrl' | 'meta' | 'shift';
    editor?: 'vscode' | 'cursor' | 'windsurf' | 'webstorm' | 'vim' | 'neovim';
    highlight?: boolean;
    highlightColor?: string;
    enabled?: boolean;
}
declare function clickToSource(options?: ClickToSourceOptions): Plugin;

export { type ClickToSourceOptions, clickToSource, createPreprocessor, clickToSource as default };

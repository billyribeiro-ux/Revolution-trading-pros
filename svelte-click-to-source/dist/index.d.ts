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

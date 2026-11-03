import { Plugin } from "vite";
import { PreprocessorGroup } from "svelte/compiler";
interface PreprocessorOptions {
  enabled?: boolean;
}
export declare function createPreprocessor(options?: PreprocessorOptions): PreprocessorGroup;
export interface ClickToSourceOptions {
  hotkey?: 'alt' | 'ctrl' | 'meta' | 'shift';
  editor?: 'vscode' | 'cursor' | 'windsurf' | 'webstorm' | 'vim' | 'neovim';
  highlight?: boolean;
  highlightColor?: string;
  enabled?: boolean;
}
export declare function clickToSource(options?: ClickToSourceOptions): Plugin;
export { clickToSource as default };
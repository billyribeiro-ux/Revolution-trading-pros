import { Plugin } from "vite";
import { PreprocessorGroup } from "svelte/compiler";
interface PreprocessorOptions {
  enabled?: boolean;
  root?: string;
}
export declare function createPreprocessor(options?: PreprocessorOptions): PreprocessorGroup;
export interface ClickToSourceOptions {
  hotkey?: 'alt' | 'ctrl' | 'meta' | 'shift';
  editor?: 'vscode' | 'cursor' | 'windsurf' | 'webstorm' | 'vim' | 'neovim';
  highlight?: boolean;
  highlightColor?: string;
  enabled?: boolean;
  /** Permit requests from non-loopback clients. Keep false unless the dev server is trusted. */
  allowRemote?: boolean;
  /** Optional trusted executable path or command name for a GUI editor. */
  editorCommand?: string;
}
export declare function resolveSourceFile(root: string, file: string): string | null;
export declare function clickToSource(options?: ClickToSourceOptions): Plugin;
export { clickToSource as default };
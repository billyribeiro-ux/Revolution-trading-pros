/**
 * svelte-click-to-source
 * 
 * Alt+Click any element in dev mode → Opens source in your editor
 */

import type { Plugin } from 'vite';
import { RUNTIME_SCRIPT } from './runtime';

export interface ClickToSourceOptions {
  hotkey?: 'alt' | 'ctrl' | 'meta' | 'shift';
  editor?: 'vscode' | 'cursor' | 'windsurf' | 'webstorm' | 'vim' | 'neovim';
  highlight?: boolean;
  highlightColor?: string;
  enabled?: boolean;
}

const VIRTUAL_MODULE_ID = 'virtual:click-to-source';
const RESOLVED_VIRTUAL_MODULE_ID = '\0' + VIRTUAL_MODULE_ID;

function detectEditor(): string {
  const termProgram = process.env.TERM_PROGRAM?.toLowerCase() || '';
  const editor = process.env.EDITOR?.toLowerCase() || '';
  const visual = process.env.VISUAL?.toLowerCase() || '';
  
  if (termProgram.includes('cursor') || editor.includes('cursor')) return 'cursor';
  if (termProgram.includes('windsurf') || editor.includes('windsurf')) return 'windsurf';
  if (editor.includes('nvim') || visual.includes('nvim')) return 'neovim';
  if (editor.includes('vim') || visual.includes('vim')) return 'vim';
  if (editor.includes('webstorm') || visual.includes('webstorm')) return 'webstorm';
  
  return 'vscode';
}

function getEditorProtocol(editor: string): string {
  const protocols: Record<string, string> = {
    vscode: 'vscode://file',
    cursor: 'cursor://file',
    windsurf: 'windsurf://file',
    webstorm: 'webstorm://open?file=',
    vim: 'mvim://open?url=file://',
    neovim: 'nvim://open?url=file://',
  };
  return protocols[editor] || protocols.vscode;
}

export function clickToSource(options: ClickToSourceOptions = {}): Plugin {
  const {
    hotkey = 'alt',
    editor = detectEditor(),
    highlight = true,
    highlightColor = '#3b82f6',
    enabled = true,
  } = options;

  let isDev = false;
  let projectRoot = '';

  return {
    name: 'vite-plugin-click-to-source',
    enforce: 'pre',

    configResolved(config) {
      isDev = config.command === 'serve';
      projectRoot = config.root;
      
      if (isDev && enabled) {
        console.log(`[click-to-source] Active. Editor: ${editor}. Hotkey: ${hotkey.toUpperCase()}+Click`);
      }
    },

    resolveId(id) {
      if (id === VIRTUAL_MODULE_ID) {
        return RESOLVED_VIRTUAL_MODULE_ID;
      }
      return undefined;
    },

    load(id) {
      if (id === RESOLVED_VIRTUAL_MODULE_ID) {
        if (!isDev || !enabled) {
          return 'export default {}';
        }
        
        const protocol = getEditorProtocol(editor);
        
        return RUNTIME_SCRIPT
          .replace(/__HOTKEY__/g, hotkey)
          .replace(/__EDITOR_PROTOCOL__/g, protocol)
          .replace(/__PROJECT_ROOT__/g, projectRoot)
          .replace(/__HIGHLIGHT_ENABLED__/g, String(highlight))
          .replace(/__HIGHLIGHT_COLOR__/g, highlightColor);
      }
      return undefined;
    },

    transformIndexHtml() {
      if (!isDev || !enabled) return [];
      
      return [
        {
          tag: 'script',
          attrs: { type: 'module' },
          children: `import '${VIRTUAL_MODULE_ID}'`,
          injectTo: 'body',
        },
      ];
    },
  };
}

export { createPreprocessor } from './preprocessor';
export default clickToSource;

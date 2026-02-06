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

import type { Plugin } from 'vite';
import { exec } from 'child_process';
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
const OPEN_ENDPOINT = '/__click-to-source-open';

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

function getEditorCommand(editor: string): string {
  const commands: Record<string, string> = {
    vscode: 'code',
    cursor: 'cursor',
    windsurf: 'windsurf',
    webstorm: 'webstorm',
    vim: 'vim',
    neovim: 'nvim',
  };
  return commands[editor] || commands.vscode;
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
  const editorCmd = getEditorCommand(editor);

  return {
    name: 'vite-plugin-click-to-source',
    enforce: 'pre',

    configResolved(config) {
      isDev = config.command === 'serve';
      projectRoot = config.root;
      
      if (isDev && enabled) {
        console.log(`[click-to-source] Active. Editor: ${editor} (${editorCmd}). Hotkey: ${hotkey.toUpperCase()}+Click`);
      }
    },

    configureServer(server) {
      if (!enabled) return;

      server.middlewares.use((req, res, next) => {
        if (!req.url?.startsWith(OPEN_ENDPOINT)) {
          return next();
        }

        const url = new URL(req.url, 'http://localhost');
        const file = url.searchParams.get('file');
        const line = url.searchParams.get('line') || '1';
        const col = url.searchParams.get('col') || '1';

        if (!file) {
          res.statusCode = 400;
          res.end('Missing file parameter');
          return;
        }

        const absolutePath = file.startsWith('/') ? file : `${projectRoot}/${file}`;
        const cmd = `${editorCmd} --goto "${absolutePath}:${line}:${col}"`;

        console.log(`[click-to-source] Opening: ${absolutePath}:${line}:${col}`);

        exec(cmd, (error) => {
          if (error) {
            console.warn(`[click-to-source] Failed to open editor:`, error.message);
            // Fallback: try open command on Mac
            exec(`open -a "Windsurf" "${absolutePath}"`, (err2) => {
              if (err2) {
                console.error(`[click-to-source] Fallback also failed:`, err2.message);
              }
            });
          }
        });

        res.setHeader('Access-Control-Allow-Origin', '*');
        res.setHeader('Content-Type', 'application/json');
        res.statusCode = 200;
        res.end(JSON.stringify({ ok: true, file: absolutePath, line, col }));
      });
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
        
        return RUNTIME_SCRIPT
          .replace(/__HOTKEY__/g, hotkey)
          .replace(/__PROJECT_ROOT__/g, projectRoot)
          .replace(/__HIGHLIGHT_ENABLED__/g, String(highlight))
          .replace(/__HIGHLIGHT_COLOR__/g, highlightColor)
          .replace(/__OPEN_ENDPOINT__/g, OPEN_ENDPOINT);
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

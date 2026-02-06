// src/runtime.ts
var RUNTIME_SCRIPT = `
(function() {
  'use strict';

  const HOTKEY = '__HOTKEY__';
  const EDITOR_PROTOCOL = '__EDITOR_PROTOCOL__';
  const PROJECT_ROOT = '__PROJECT_ROOT__';
  const HIGHLIGHT_ENABLED = __HIGHLIGHT_ENABLED__;
  const HIGHLIGHT_COLOR = '__HIGHLIGHT_COLOR__';

  let isHotkeyPressed = false;
  let currentHighlight = null;
  let overlay = null;
  let tooltip = null;

  const hotkeyMap = { alt: 'altKey', ctrl: 'ctrlKey', meta: 'metaKey', shift: 'shiftKey' };
  const hotkeyProp = hotkeyMap[HOTKEY] || 'altKey';

  function createOverlay() {
    overlay = document.createElement('div');
    overlay.id = 'click-to-source-overlay';
    overlay.style.cssText = \`
      position: fixed;
      pointer-events: none;
      z-index: 999999;
      border: 2px solid \${HIGHLIGHT_COLOR};
      background: \${HIGHLIGHT_COLOR}20;
      border-radius: 4px;
      transition: all 0.1s ease-out;
      display: none;
    \`;
    document.body.appendChild(overlay);

    tooltip = document.createElement('div');
    tooltip.id = 'click-to-source-tooltip';
    tooltip.style.cssText = \`
      position: fixed;
      pointer-events: none;
      z-index: 999999;
      background: #1e1e1e;
      color: #fff;
      padding: 4px 8px;
      border-radius: 4px;
      font-family: ui-monospace, monospace;
      font-size: 12px;
      white-space: nowrap;
      display: none;
      box-shadow: 0 2px 8px rgba(0,0,0,0.3);
    \`;
    document.body.appendChild(tooltip);
  }

  function findSourceElement(target) {
    let el = target;
    while (el && el !== document.body) {
      if (el.dataset && el.dataset.source) {
        return el;
      }
      el = el.parentElement;
    }
    return null;
  }

  function updateHighlight(el) {
    if (!HIGHLIGHT_ENABLED || !overlay) return;

    if (!el) {
      overlay.style.display = 'none';
      tooltip.style.display = 'none';
      currentHighlight = null;
      return;
    }

    const rect = el.getBoundingClientRect();
    overlay.style.display = 'block';
    overlay.style.left = rect.left + 'px';
    overlay.style.top = rect.top + 'px';
    overlay.style.width = rect.width + 'px';
    overlay.style.height = rect.height + 'px';

    const source = el.dataset.source;
    if (source) {
      tooltip.textContent = source;
      tooltip.style.display = 'block';
      let tooltipTop = rect.top - 28;
      if (tooltipTop < 8) tooltipTop = rect.bottom + 8;
      tooltip.style.left = rect.left + 'px';
      tooltip.style.top = tooltipTop + 'px';
    }

    currentHighlight = el;
  }

  function openInEditor(source) {
    if (!source) return;

    const match = source.match(/^(.+):(\\\\d+):(\\\\d+)$/);
    if (!match) {
      console.warn('[click-to-source] Invalid source format:', source);
      return;
    }

    const [, filePath, line, col] = match;
    const absolutePath = PROJECT_ROOT + '/' + filePath;

    let url;
    if (EDITOR_PROTOCOL.includes('webstorm')) {
      url = EDITOR_PROTOCOL + absolutePath + '&line=' + line + '&column=' + col;
    } else if (EDITOR_PROTOCOL.includes('mvim') || EDITOR_PROTOCOL.includes('nvim')) {
      url = EDITOR_PROTOCOL + absolutePath + '&line=' + line;
    } else {
      url = EDITOR_PROTOCOL + absolutePath + ':' + line + ':' + col;
    }

    console.log('[click-to-source] Opening:', url);
    window.open(url, '_self');
  }

  function handleKeyDown(e) {
    if (e[hotkeyProp] && !isHotkeyPressed) {
      isHotkeyPressed = true;
      document.body.style.cursor = 'crosshair';
    }
  }

  function handleKeyUp(e) {
    if (!e[hotkeyProp] && isHotkeyPressed) {
      isHotkeyPressed = false;
      document.body.style.cursor = '';
      updateHighlight(null);
    }
  }

  function handleMouseMove(e) {
    if (!isHotkeyPressed) return;
    const sourceEl = findSourceElement(e.target);
    if (sourceEl !== currentHighlight) {
      updateHighlight(sourceEl);
    }
  }

  function handleClick(e) {
    if (!isHotkeyPressed) return;
    const sourceEl = findSourceElement(e.target);
    if (sourceEl && sourceEl.dataset.source) {
      e.preventDefault();
      e.stopPropagation();
      openInEditor(sourceEl.dataset.source);
    }
  }

  function init() {
    if (document.getElementById('click-to-source-overlay')) return;
    createOverlay();
    document.addEventListener('keydown', handleKeyDown, true);
    document.addEventListener('keyup', handleKeyUp, true);
    document.addEventListener('mousemove', handleMouseMove, true);
    document.addEventListener('click', handleClick, true);
    window.addEventListener('blur', () => {
      isHotkeyPressed = false;
      document.body.style.cursor = '';
      updateHighlight(null);
    });
    console.log('[click-to-source] Ready. Hold ' + HOTKEY.toUpperCase() + ' and click any element.');
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
  } else {
    init();
  }
})();
`;

// src/preprocessor.ts
import MagicString from "magic-string";
function createPreprocessor(options = {}) {
  const { enabled = process.env.NODE_ENV !== "production" } = options;
  return {
    name: "click-to-source-preprocessor",
    markup({ content, filename }) {
      if (!enabled || !filename) {
        return { code: content };
      }
      if (filename.includes("node_modules") || filename.includes(".svelte-kit")) {
        return { code: content };
      }
      try {
        const s = new MagicString(content);
        const insertions = [];
        const tagRegex = /<([a-z][a-z0-9]*)([\s>])/gi;
        let match;
        while ((match = tagRegex.exec(content)) !== null) {
          const tagStart = match.index;
          const afterTagName = match.index + match[0].length - 1;
          const beforeTag = content.slice(0, tagStart);
          const lines = beforeTag.split("\n");
          const line = lines.length;
          const column = lines[lines.length - 1].length + 1;
          const relativePath = filename.replace(process.cwd() + "/", "");
          const attr = ` data-source="${relativePath}:${line}:${column}"`;
          insertions.push({ pos: afterTagName, text: attr });
        }
        insertions.sort((a, b) => b.pos - a.pos).forEach(({ pos, text }) => {
          s.appendLeft(pos, text);
        });
        return {
          code: s.toString(),
          map: s.generateMap({ source: filename, hires: true })
        };
      } catch (error) {
        console.warn(`[click-to-source] Failed to process ${filename}:`, error);
        return { code: content };
      }
    }
  };
}

// src/index.ts
var VIRTUAL_MODULE_ID = "virtual:click-to-source";
var RESOLVED_VIRTUAL_MODULE_ID = "\0" + VIRTUAL_MODULE_ID;
function detectEditor() {
  const termProgram = process.env.TERM_PROGRAM?.toLowerCase() || "";
  const editor = process.env.EDITOR?.toLowerCase() || "";
  const visual = process.env.VISUAL?.toLowerCase() || "";
  if (termProgram.includes("cursor") || editor.includes("cursor")) return "cursor";
  if (termProgram.includes("windsurf") || editor.includes("windsurf")) return "windsurf";
  if (editor.includes("nvim") || visual.includes("nvim")) return "neovim";
  if (editor.includes("vim") || visual.includes("vim")) return "vim";
  if (editor.includes("webstorm") || visual.includes("webstorm")) return "webstorm";
  return "vscode";
}
function getEditorProtocol(editor) {
  const protocols = {
    vscode: "vscode://file",
    cursor: "cursor://file",
    windsurf: "windsurf://file",
    webstorm: "webstorm://open?file=",
    vim: "mvim://open?url=file://",
    neovim: "nvim://open?url=file://"
  };
  return protocols[editor] || protocols.vscode;
}
function clickToSource(options = {}) {
  const {
    hotkey = "alt",
    editor = detectEditor(),
    highlight = true,
    highlightColor = "#3b82f6",
    enabled = true
  } = options;
  let isDev = false;
  let projectRoot = "";
  return {
    name: "vite-plugin-click-to-source",
    enforce: "pre",
    configResolved(config) {
      isDev = config.command === "serve";
      projectRoot = config.root;
      if (isDev && enabled) {
        console.log(`[click-to-source] Active. Editor: ${editor}. Hotkey: ${hotkey.toUpperCase()}+Click`);
      }
    },
    resolveId(id) {
      if (id === VIRTUAL_MODULE_ID) {
        return RESOLVED_VIRTUAL_MODULE_ID;
      }
      return void 0;
    },
    load(id) {
      if (id === RESOLVED_VIRTUAL_MODULE_ID) {
        if (!isDev || !enabled) {
          return "export default {}";
        }
        const protocol = getEditorProtocol(editor);
        return RUNTIME_SCRIPT.replace(/__HOTKEY__/g, hotkey).replace(/__EDITOR_PROTOCOL__/g, protocol).replace(/__PROJECT_ROOT__/g, projectRoot).replace(/__HIGHLIGHT_ENABLED__/g, String(highlight)).replace(/__HIGHLIGHT_COLOR__/g, highlightColor);
      }
      return void 0;
    },
    transformIndexHtml() {
      if (!isDev || !enabled) return [];
      return [
        {
          tag: "script",
          attrs: { type: "module" },
          children: `import '${VIRTUAL_MODULE_ID}'`,
          injectTo: "body"
        }
      ];
    }
  };
}
var index_default = clickToSource;
export {
  clickToSource,
  createPreprocessor,
  index_default as default
};

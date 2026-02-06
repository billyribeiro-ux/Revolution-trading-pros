// src/index.ts
import { exec } from "child_process";
import { existsSync } from "fs";

// src/runtime.ts
var RUNTIME_SCRIPT = `
(function() {
  'use strict';

  var HOTKEY = '__HOTKEY__';
  var PROJECT_ROOT = '__PROJECT_ROOT__';
  var HIGHLIGHT_ENABLED = __HIGHLIGHT_ENABLED__;
  var HIGHLIGHT_COLOR = '__HIGHLIGHT_COLOR__';
  var OPEN_ENDPOINT = '__OPEN_ENDPOINT__';

  var isHotkeyPressed = false;
  var currentHighlight = null;
  var overlay = null;
  var tooltip = null;

  var hotkeyMap = { alt: 'altKey', ctrl: 'ctrlKey', meta: 'metaKey', shift: 'shiftKey' };
  var hotkeyProp = hotkeyMap[HOTKEY] || 'altKey';

  function createOverlay() {
    overlay = document.createElement('div');
    overlay.id = 'click-to-source-overlay';
    overlay.style.cssText = 'position:fixed;pointer-events:none;z-index:999999;border:2px solid ' + HIGHLIGHT_COLOR + ';background:' + HIGHLIGHT_COLOR + '20;border-radius:4px;transition:all 0.1s ease-out;display:none;';
    document.body.appendChild(overlay);

    tooltip = document.createElement('div');
    tooltip.id = 'click-to-source-tooltip';
    tooltip.style.cssText = 'position:fixed;pointer-events:none;z-index:999999;background:#1e1e1e;color:#fff;padding:4px 8px;border-radius:4px;font-family:ui-monospace,monospace;font-size:12px;white-space:nowrap;display:none;box-shadow:0 2px 8px rgba(0,0,0,0.3);';
    document.body.appendChild(tooltip);
  }

  function findSourceElement(target) {
    var el = target;
    while (el && el !== document.body) {
      if (el.dataset && el.dataset.source) return el;
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
    var rect = el.getBoundingClientRect();
    overlay.style.display = 'block';
    overlay.style.left = rect.left + 'px';
    overlay.style.top = rect.top + 'px';
    overlay.style.width = rect.width + 'px';
    overlay.style.height = rect.height + 'px';
    var source = el.dataset.source;
    if (source) {
      tooltip.textContent = source;
      tooltip.style.display = 'block';
      var tooltipTop = rect.top - 28;
      if (tooltipTop < 8) tooltipTop = rect.bottom + 8;
      tooltip.style.left = rect.left + 'px';
      tooltip.style.top = tooltipTop + 'px';
    }
    currentHighlight = el;
  }

  function openInEditor(source) {
    if (!source) return;
    var parts = source.split(':');
    if (parts.length < 3) {
      console.warn('[click-to-source] Invalid source format:', source);
      return;
    }
    var col = parts.pop();
    var line = parts.pop();
    var filePath = parts.join(':');
    var url = OPEN_ENDPOINT + '?file=' + encodeURIComponent(filePath) + '&line=' + line + '&col=' + col;
    console.log('[click-to-source] Opening:', filePath + ':' + line + ':' + col);
    fetch(url).then(function(r) {
      if (!r.ok) console.warn('[click-to-source] Server returned', r.status);
    }).catch(function(err) {
      console.error('[click-to-source] Failed to contact dev server:', err);
    });
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
    var sourceEl = findSourceElement(e.target);
    if (sourceEl !== currentHighlight) updateHighlight(sourceEl);
  }

  function handleClick(e) {
    if (!isHotkeyPressed) return;
    var sourceEl = findSourceElement(e.target);
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
    window.addEventListener('blur', function() {
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
var EXCLUDED_TAGS = /* @__PURE__ */ new Set([
  "script",
  "style",
  "title",
  "slot",
  "head",
  "svelte:head",
  "svelte:body",
  "svelte:window",
  "svelte:document",
  "svelte:component",
  "svelte:self",
  "svelte:fragment",
  "svelte:element",
  "svelte:options",
  "svelte:boundary"
]);
function findExcludedRanges(content) {
  const ranges = [];
  const blockRegex = /<(script|style)(\s[^>]*)?>[\s\S]*?<\/\1>/gi;
  let match;
  while ((match = blockRegex.exec(content)) !== null) {
    ranges.push({ start: match.index, end: match.index + match[0].length });
  }
  return ranges;
}
function isInsideExcludedRange(pos, ranges) {
  return ranges.some((r) => pos >= r.start && pos < r.end);
}
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
        const excludedRanges = findExcludedRanges(content);
        const tagRegex = /<([a-z][a-z0-9-]*)([\s>\/])/gi;
        let match;
        while ((match = tagRegex.exec(content)) !== null) {
          const tagStart = match.index;
          const tagName = match[1].toLowerCase();
          const afterTagName = match.index + match[0].length - 1;
          if (isInsideExcludedRange(tagStart, excludedRanges)) {
            continue;
          }
          if (EXCLUDED_TAGS.has(tagName)) {
            continue;
          }
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
var OPEN_ENDPOINT = "/__click-to-source-open";
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
function getEditorCommand(editor) {
  const editorPaths = {
    vscode: ["code", "/usr/local/bin/code", "/Applications/Visual Studio Code.app/Contents/Resources/app/bin/code"],
    cursor: ["cursor", "/usr/local/bin/cursor", "/Applications/Cursor.app/Contents/Resources/app/bin/cursor"],
    windsurf: ["windsurf", "/usr/local/bin/windsurf", "/Applications/Windsurf.app/Contents/Resources/app/bin/windsurf"],
    webstorm: ["webstorm", "/usr/local/bin/webstorm"],
    vim: ["vim"],
    neovim: ["nvim"]
  };
  const paths = editorPaths[editor] || editorPaths.vscode;
  for (const p of paths) {
    if (p.startsWith("/") && existsSync(p)) return p;
  }
  return paths[0];
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
  const editorCmd = getEditorCommand(editor);
  return {
    name: "vite-plugin-click-to-source",
    enforce: "pre",
    configResolved(config) {
      isDev = config.command === "serve";
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
        const queryString = req.url.split("?")[1] || "";
        const params = new URLSearchParams(queryString);
        const rawFile = queryString.match(/file=([^&]*)/)?.[1];
        const file = rawFile ? decodeURIComponent(rawFile.replace(/\+/g, "%2B")) : params.get("file");
        const line = params.get("line") || "1";
        const col = params.get("col") || "1";
        if (!file) {
          res.statusCode = 400;
          res.end("Missing file parameter");
          return;
        }
        const absolutePath = file.startsWith("/") ? file : `${projectRoot}/${file}`;
        const cmd = `${editorCmd} --goto "${absolutePath}:${line}:${col}"`;
        console.log(`[click-to-source] Opening: ${absolutePath}:${line}:${col}`);
        exec(cmd, (error) => {
          if (error) {
            console.warn(`[click-to-source] Primary command failed:`, error.message);
            const appName = editor.charAt(0).toUpperCase() + editor.slice(1);
            const fallbackCmd = `open -a "${appName}" "${absolutePath}"`;
            console.log(`[click-to-source] Trying fallback: ${fallbackCmd}`);
            exec(fallbackCmd, (err2) => {
              if (err2) {
                console.error(`[click-to-source] Fallback also failed:`, err2.message);
              }
            });
          }
        });
        res.setHeader("Access-Control-Allow-Origin", "*");
        res.setHeader("Content-Type", "application/json");
        res.statusCode = 200;
        res.end(JSON.stringify({ ok: true, file: absolutePath, line, col }));
      });
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
        return RUNTIME_SCRIPT.replace(/__HOTKEY__/g, hotkey).replace(/__PROJECT_ROOT__/g, projectRoot).replace(/__HIGHLIGHT_ENABLED__/g, String(highlight)).replace(/__HIGHLIGHT_COLOR__/g, highlightColor).replace(/__OPEN_ENDPOINT__/g, OPEN_ENDPOINT);
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

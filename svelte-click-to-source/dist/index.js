import { randomBytes } from "node:crypto";
import { spawn } from "node:child_process";
import { existsSync, realpathSync, statSync } from "node:fs";
import { isAbsolute, relative, resolve, sep } from "node:path";
import MagicString from "magic-string";
// src/runtime.ts
/**
* Runtime Inspector for click-to-source
*
* Injected into the browser in dev mode.
* On ALT+Click, sends a fetch request to the Vite dev server middleware
* which opens the file in the editor via CLI command.
*/
const RUNTIME_SCRIPT = `
(function() {
  'use strict';

  var HOTKEY = __HOTKEY__;
  var HIGHLIGHT_ENABLED = __HIGHLIGHT_ENABLED__;
  var HIGHLIGHT_COLOR = __HIGHLIGHT_COLOR__;
  var OPEN_ENDPOINT = __OPEN_ENDPOINT__;
  var REQUEST_TOKEN = __REQUEST_TOKEN__;

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
    fetch(url, { method: 'POST', headers: { 'X-Click-To-Source-Token': REQUEST_TOKEN } }).then(function(r) {
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
const EXCLUDED_TAGS = /* @__PURE__ */ new Set([
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
/**
* Find all ranges in the source that are inside <script> or <style> blocks.
* We must not inject data-source attributes inside these blocks.
*/
function findExcludedRanges(content) {
	const ranges = [];
	const blockRegex = /<(script|style)(\s[^>]*)?>[\s\S]*?<\/\1>/gi;
	let match;
	while ((match = blockRegex.exec(content)) !== null) ranges.push({
		start: match.index,
		end: match.index + match[0].length
	});
	return ranges;
}
function isInsideExcludedRange(pos, ranges) {
	return ranges.some((r) => pos >= r.start && pos < r.end);
}
function createPreprocessor(options = {}) {
	const { enabled = process.env.NODE_ENV !== "production", root = process.cwd() } = options;
	return {
		name: "click-to-source-preprocessor",
		markup({ content, filename }) {
			if (!enabled || !filename) return { code: content };
			if (filename.includes("node_modules") || filename.includes(".svelte-kit")) return { code: content };
			try {
				const s = new MagicString(content);
				const insertions = [];
				const excludedRanges = findExcludedRanges(content);
				const tagRegex = /<([a-z][a-z0-9-]*)([\s>/])/gi;
				let match;
				while ((match = tagRegex.exec(content)) !== null) {
					const tagStart = match.index;
					const tagName = match[1].toLowerCase();
					const afterTagName = match.index + match[0].length - 1;
					if (isInsideExcludedRange(tagStart, excludedRanges)) continue;
					if (EXCLUDED_TAGS.has(tagName)) continue;
					const lines = content.slice(0, tagStart).split("\n");
					const line = lines.length;
					const column = lines[lines.length - 1].length + 1;
					const relativePath = relative(root, filename);
					if (!relativePath || relativePath === ".." || relativePath.startsWith(`..${sep}`) || isAbsolute(relativePath)) continue;
					const attr = ` data-source="${relativePath.replaceAll("&", "&amp;").replaceAll("\"", "&quot;").replaceAll("<", "&lt;")}:${line}:${column}"`;
					insertions.push({
						pos: afterTagName,
						text: attr
					});
				}
				insertions.sort((a, b) => b.pos - a.pos).forEach(({ pos, text }) => {
					s.appendLeft(pos, text);
				});
				return {
					code: s.toString(),
					map: s.generateMap({
						source: filename,
						hires: true
					})
				};
			} catch (error) {
				console.warn(`[click-to-source] Failed to process ${filename}:`, error);
				return { code: content };
			}
		}
	};
}
// src/index.ts
const VIRTUAL_MODULE_ID = "virtual:click-to-source";
const RESOLVED_VIRTUAL_MODULE_ID = "\0" + VIRTUAL_MODULE_ID;
const OPEN_ENDPOINT = "/__click-to-source-open";
const MAX_POSITION = 1e6;
function detectEditor() {
	const values = [
		process.env.TERM_PROGRAM,
		process.env.EDITOR,
		process.env.VISUAL
	].filter((value) => Boolean(value)).map((value) => value.toLowerCase());
	for (const name of [
		"cursor",
		"windsurf",
		"nvim",
		"vim",
		"webstorm"
	]) if (values.some((value) => value.includes(name))) return name === "nvim" ? "neovim" : name;
	return "vscode";
}
function getEditorCommand(editor) {
	const candidates = {
		vscode: [
			"code",
			"/usr/local/bin/code",
			"/Applications/Visual Studio Code.app/Contents/Resources/app/bin/code"
		],
		cursor: [
			"cursor",
			"/usr/local/bin/cursor",
			"/Applications/Cursor.app/Contents/Resources/app/bin/cursor"
		],
		windsurf: [
			"windsurf",
			"/usr/local/bin/windsurf",
			"/Applications/Windsurf.app/Contents/Resources/app/bin/windsurf"
		],
		webstorm: ["webstorm", "/usr/local/bin/webstorm"],
		vim: ["vim"],
		neovim: ["nvim"]
	}[editor];
	return candidates.find((candidate) => candidate.startsWith("/") && existsSync(candidate)) ?? candidates[0];
}
function resolveSourceFile(root, file) {
	if (!file || file.includes("\0") || isAbsolute(file) || !file.endsWith(".svelte")) return null;
	try {
		const rootPath = realpathSync(root);
		const filePath = realpathSync(resolve(rootPath, file));
		const withinRoot = relative(rootPath, filePath);
		if (!withinRoot || withinRoot === ".." || withinRoot.startsWith(`..${sep}`) || isAbsolute(withinRoot)) return null;
		return statSync(filePath).isFile() ? filePath : null;
	} catch {
		return null;
	}
}
function validPosition(value) {
	if (!value || !/^[1-9]\d{0,6}$/.test(value)) return false;
	return Number(value) <= MAX_POSITION;
}
function isLoopback(address) {
	return address === "::1" || address?.startsWith("127.") === true || address?.startsWith("::ffff:127.") === true;
}
function clickToSource(options = {}) {
	const { hotkey = "alt", editor = detectEditor(), highlight = true, highlightColor = "#3b82f6", enabled = true, allowRemote = false, editorCommand } = options;
	const requestToken = randomBytes(32).toString("base64url");
	let isDev = false;
	let projectRoot = "";
	const editorCmd = editorCommand ?? getEditorCommand(editor);
	return {
		name: "vite-plugin-click-to-source",
		enforce: "pre",
		configResolved(config) {
			isDev = config.command === "serve";
			projectRoot = config.root;
			if (isDev && enabled) console.log(`[click-to-source] Active. Editor: ${editor}. Hotkey: ${hotkey.toUpperCase()}+Click`);
		},
		configureServer(server) {
			if (!enabled || !isDev) return;
			server.middlewares.use((req, res, next) => {
				let url;
				try {
					url = new URL(req.url ?? "/", "http://localhost");
				} catch {
					return next();
				}
				if (url.pathname !== OPEN_ENDPOINT) return next();
				res.setHeader("Cache-Control", "no-store");
				res.setHeader("Content-Type", "application/json");
				if (!allowRemote && !isLoopback(req.socket.remoteAddress)) {
					res.statusCode = 403;
					res.end(JSON.stringify({ error: "Remote requests are disabled" }));
					return;
				}
				if (req.method !== "POST") {
					res.statusCode = 405;
					res.setHeader("Allow", "POST");
					res.end(JSON.stringify({ error: "Method not allowed" }));
					return;
				}
				if (req.headers["x-click-to-source-token"] !== requestToken) {
					res.statusCode = 403;
					res.end(JSON.stringify({ error: "Forbidden" }));
					return;
				}
				if (req.headers.origin) try {
					if (new URL(req.headers.origin).host !== req.headers.host) {
						res.statusCode = 403;
						res.end(JSON.stringify({ error: "Forbidden" }));
						return;
					}
				} catch {
					res.statusCode = 403;
					res.end(JSON.stringify({ error: "Forbidden" }));
					return;
				}
				const file = url.searchParams.get("file");
				const line = url.searchParams.get("line");
				const col = url.searchParams.get("col");
				const sourcePath = file ? resolveSourceFile(projectRoot, file) : null;
				if (!sourcePath || !validPosition(line) || !validPosition(col)) {
					res.statusCode = 400;
					res.end(JSON.stringify({ error: "Invalid source location" }));
					return;
				}
				const args = editor === "vim" || editor === "neovim" ? [`+${line}`, sourcePath] : ["--goto", `${sourcePath}:${line}:${col}`];
				const child = spawn(editorCmd, args, {
					detached: true,
					stdio: "ignore",
					shell: false
				});
				child.once("error", () => {
					if (res.writableEnded) return;
					res.statusCode = 500;
					res.end(JSON.stringify({ error: "Editor could not be started" }));
				});
				child.once("spawn", () => {
					child.unref();
					if (res.writableEnded) return;
					res.statusCode = 200;
					res.end(JSON.stringify({ ok: true }));
				});
			});
		},
		resolveId(id) {
			return id === VIRTUAL_MODULE_ID ? RESOLVED_VIRTUAL_MODULE_ID : void 0;
		},
		load(id) {
			if (id !== RESOLVED_VIRTUAL_MODULE_ID) return void 0;
			if (!isDev || !enabled) return "export default {}";
			return RUNTIME_SCRIPT.replace("__HOTKEY__", JSON.stringify(hotkey)).replace("__HIGHLIGHT_ENABLED__", JSON.stringify(highlight)).replace("__HIGHLIGHT_COLOR__", JSON.stringify(highlightColor)).replace("__OPEN_ENDPOINT__", JSON.stringify(OPEN_ENDPOINT)).replace("__REQUEST_TOKEN__", JSON.stringify(requestToken));
		},
		transformIndexHtml() {
			if (!isDev || !enabled) return [];
			return [{
				tag: "script",
				attrs: { type: "module" },
				children: `import '${VIRTUAL_MODULE_ID}'`,
				injectTo: "body"
			}];
		}
	};
}
export { clickToSource, clickToSource as default, createPreprocessor, resolveSourceFile };

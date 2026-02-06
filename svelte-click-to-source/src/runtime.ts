/**
 * Runtime Inspector for click-to-source
 * Injected into the page in dev mode.
 */

export const RUNTIME_SCRIPT = `
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

export default RUNTIME_SCRIPT;

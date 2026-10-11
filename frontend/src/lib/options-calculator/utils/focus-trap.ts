// ============================================================
// FOCUS TRAP — keeps keyboard focus inside a modal dialog
// ============================================================

import { on } from 'svelte/events';
import type { Attachment } from 'svelte/attachments';

const TABBABLE = [
	'a[href]',
	'button:not([disabled])',
	'input:not([disabled]):not([type="hidden"])',
	'select:not([disabled])',
	'textarea:not([disabled])',
	'[contenteditable="true"]',
	'[tabindex]:not([tabindex="-1"])'
].join(', ');

/**
 * Wraps Tab / Shift+Tab between the first and last tabbable elements of an
 * `aria-modal` dialog, as the WAI-ARIA dialog pattern requires: the backdrop
 * blocks the pointer, and this keeps the keyboard from reaching the page behind.
 */
export const trapFocus: Attachment<HTMLElement> = (dialog) =>
	on(dialog, 'keydown', (event) => {
		if (event.key !== 'Tab') return;
		const tabbable = Array.from(dialog.querySelectorAll<HTMLElement>(TABBABLE)).filter(
			(element) => element.getClientRects().length > 0
		);
		const first = tabbable.at(0);
		const last = tabbable.at(-1);
		if (!first || !last) {
			event.preventDefault();
			dialog.focus();
			return;
		}
		const active = document.activeElement;
		if (event.shiftKey && (active === first || active === dialog)) {
			event.preventDefault();
			last.focus();
		} else if (!event.shiftKey && active === last) {
			event.preventDefault();
			first.focus();
		}
	});

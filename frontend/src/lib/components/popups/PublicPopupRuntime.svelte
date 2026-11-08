<script lang="ts">
	import { onMount, tick } from 'svelte';
	import type { Attachment } from 'svelte/attachments';
	import { normalizeForm } from '#lib/api/forms-contract.js';
	import type { Form } from '#lib/api/forms.js';
	import type FormRenderer from '#lib/components/forms/FormRenderer.svelte';

	type PublicPopup = {
		id: number;
		type: string;
		title: string | null;
		content: string | null;
		cta_text: string | null;
		cta_url: string | null;
		cta_new_tab: boolean;
		close_on_escape: boolean;
		close_on_overlay_click: boolean;
		show_close_button: boolean;
		auto_close_after: number | null;
		has_form: boolean;
		form_id: number | null;
		trigger_rules: { delay?: number; scroll_depth?: number; selector?: string };
		frequency_rules: { frequency: string; cooldown_hours?: number; max_impressions?: number };
	};

	let { pathname }: { pathname: string } = $props();
	let popup = $state<PublicPopup | null>(null);
	let form = $state<Form | null>(null);
	let LoadedFormRenderer = $state<typeof FormRenderer | null>(null);
	let visible = $state(false);
	let panel: HTMLElement | null = $state(null);
	let previousFocus: HTMLElement | null = null;
	let conversionRecorded = false;
	let cleanup: Array<() => void> = [];
	let disposed = false;
	const attachPanel: Attachment<HTMLElement> = (element) => {
		panel = element;
		return () => {
			if (panel === element) panel = null;
		};
	};

	function isPopup(value: unknown): value is PublicPopup {
		return (
			typeof value === 'object' &&
			value !== null &&
			'id' in value &&
			typeof value.id === 'number' &&
			'type' in value &&
			typeof value.type === 'string' &&
			'frequency_rules' in value &&
			typeof value.frequency_rules === 'object' &&
			value.frequency_rules !== null
		);
	}

	function shownRecently(candidate: PublicPopup): boolean {
		const key = `popup-frequency-${candidate.id}`;
		const frequency = candidate.frequency_rules.frequency;
		const storage = frequency === 'once_per_session' ? sessionStorage : localStorage;
		const history = readHistory(storage.getItem(key));
		if (!history.last) return false;
		if (candidate.frequency_rules.max_impressions && history.count >= candidate.frequency_rules.max_impressions) return true;
		const cooldown = candidate.frequency_rules.cooldown_hours;
		if (cooldown && Date.now() - history.last < cooldown * 3_600_000) return true;
		if (frequency === 'always') return false;
		if (frequency === 'once' || frequency === 'once_per_session') return true;
		const duration = frequency === 'weekly' ? 7 * 86_400_000 : 86_400_000;
		return Date.now() - history.last < duration;
	}

	function readHistory(value: string | null): { last: number; count: number } {
		if (!value) return { last: 0, count: 0 };
		try {
			const parsed: unknown = JSON.parse(value);
			if (typeof parsed === 'number' && Number.isFinite(parsed)) return { last: parsed, count: 1 };
			if (typeof parsed === 'object' && parsed !== null && 'last' in parsed && 'count' in parsed &&
				typeof parsed.last === 'number' && typeof parsed.count === 'number') {
				return { last: parsed.last, count: parsed.count };
			}
		} catch {
			const last = Number(value);
			if (Number.isFinite(last)) return { last, count: 1 };
		}
		return { last: 0, count: 0 };
	}

	function rememberShown(candidate: PublicPopup): void {
		const frequency = candidate.frequency_rules.frequency;
		const storage = frequency === 'once_per_session' ? sessionStorage : localStorage;
		const key = `popup-frequency-${candidate.id}`;
		const previous = readHistory(storage.getItem(key));
		storage.setItem(key, JSON.stringify({ last: Date.now(), count: previous.count + 1 }));
	}

	async function record(action: 'impression' | 'conversion'): Promise<void> {
		if (!popup) return;
		if (action === 'conversion') {
			if (conversionRecorded) return;
			conversionRecorded = true;
		}
		try {
			const response = await fetch(`/api/popups/${popup.id}/${action}`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ page_url: pathname, device_type: window.innerWidth < 768 ? 'mobile' : 'desktop' }),
				keepalive: true
			});
			if (!response.ok && action === 'conversion') conversionRecorded = false;
		} catch {
			if (action === 'conversion') conversionRecorded = false;
			// Display and navigation must still work if analytics is unavailable.
		}
	}

	async function open(): Promise<void> {
		if (!popup || visible || disposed || shownRecently(popup)) return;
		previousFocus = document.activeElement instanceof HTMLElement ? document.activeElement : null;
		visible = true;
		rememberShown(popup);
		void record('impression');
		await tick();
		panel?.querySelector<HTMLElement>('button, a, input')?.focus();
		if (popup.auto_close_after && popup.auto_close_after > 0) {
			const timer = setTimeout(close, Math.min(popup.auto_close_after, 3600) * 1000);
			cleanup.push(() => clearTimeout(timer));
		}
	}

	function close(): void {
		visible = false;
		previousFocus?.focus();
		previousFocus = null;
	}

	function schedule(candidate: PublicPopup): void {
		const rules = candidate.trigger_rules;
		if (candidate.type === 'exit_intent') {
			const listener = (event: MouseEvent) => {
				if (event.clientY <= 0) void open();
			};
			document.addEventListener('mouseout', listener);
			cleanup.push(() => document.removeEventListener('mouseout', listener));
		} else if (candidate.type === 'scroll') {
			const listener = () => {
				const available = document.documentElement.scrollHeight - window.innerHeight;
				const depth = available > 0 ? (window.scrollY / available) * 100 : 100;
				if (depth >= (rules.scroll_depth ?? 50)) void open();
			};
			window.addEventListener('scroll', listener, { passive: true });
			cleanup.push(() => window.removeEventListener('scroll', listener));
		} else if (candidate.type === 'click_trigger' && rules.selector) {
			const selector = rules.selector;
			const listener = (event: MouseEvent) => {
				try {
					if (event.target instanceof Element && event.target.closest(selector)) void open();
				} catch {
					// An invalid admin-supplied selector cannot break the page.
				}
			};
			document.addEventListener('click', listener);
			cleanup.push(() => document.removeEventListener('click', listener));
		} else {
			const delay = Math.min(Math.max(rules.delay ?? 0, 0), 60_000);
			const timer = setTimeout(() => void open(), delay);
			cleanup.push(() => clearTimeout(timer));
		}
	}

	async function load(): Promise<void> {
		try {
			const device = window.innerWidth < 768 ? 'mobile' : 'desktop';
			const response = await fetch(
				`/api/popups/active?page=${encodeURIComponent(pathname)}&device=${device}`
			);
			if (!response.ok) return;
			const body: unknown = await response.json();
			if (typeof body !== 'object' || body === null || !('popups' in body) || !Array.isArray(body.popups)) return;
			const candidate = body.popups.find((item: unknown) => isPopup(item) && !shownRecently(item));
			if (!isPopup(candidate) || disposed) return;
			popup = candidate;
			if (candidate.has_form && candidate.form_id) {
				const formResponse = await fetch(`/api/forms/${candidate.form_id}`);
				if (formResponse.ok && !disposed) {
					form = normalizeForm(await formResponse.json());
					LoadedFormRenderer = (await import('#lib/components/forms/FormRenderer.svelte')).default;
				}
			}
			if (!disposed) schedule(candidate);
		} catch {
			// A failed campaign lookup must not interfere with the page.
		}
	}

	function onKeydown(event: KeyboardEvent): void {
		if (event.key === 'Escape' && popup?.close_on_escape) {
			close();
		} else if (event.key === 'Tab' && panel) {
			const controls = [...panel.querySelectorAll<HTMLElement>('button, a[href], input, select, textarea, [tabindex]:not([tabindex="-1"])')];
			if (controls.length === 0) return;
			const first = controls[0];
			const last = controls[controls.length - 1];
			if (event.shiftKey && document.activeElement === first) {
				last?.focus();
				event.preventDefault();
			} else if (!event.shiftKey && document.activeElement === last) {
				first?.focus();
				event.preventDefault();
			}
		}
	}

	function safeLink(value: string | null): string | null {
		if (!value) return null;
		try {
			const url = new URL(value, window.location.origin);
			return ['http:', 'https:'].includes(url.protocol) ? url.href : null;
		} catch {
			return null;
		}
	}

	onMount(() => {
		void load();
		return () => {
			disposed = true;
			cleanup.forEach((stop) => stop());
			close();
		};
	});
</script>

{#if visible && popup}
	<div class="popup-backdrop">
		{#if popup.close_on_overlay_click}
			<button class="popup-overlay" type="button" aria-label="Close popup" onclick={close}></button>
		{/if}
		<div class="popup-panel" role="dialog" aria-modal="true" aria-label={popup.title || 'Website message'} tabindex="-1" {@attach attachPanel} onkeydown={onKeydown}>
			{#if popup.show_close_button}
				<button class="popup-close" type="button" aria-label="Close popup" onclick={close}>×</button>
			{/if}
			{#if popup.title}<h2>{popup.title}</h2>{/if}
			{#if popup.content}<p>{popup.content}</p>{/if}
			{#if form && LoadedFormRenderer}
				<LoadedFormRenderer {form} onSuccess={() => void record('conversion')} />
			{/if}
			{#if popup.cta_text && safeLink(popup.cta_url)}
				<a href={safeLink(popup.cta_url) ?? '#'} target={popup.cta_new_tab ? '_blank' : undefined} rel="noopener noreferrer" onclick={() => void record('conversion')}>{popup.cta_text}</a>
			{/if}
		</div>
	</div>
{/if}

<style>
	.popup-backdrop{position:fixed;inset:0;z-index:9998;display:grid;place-items:center;padding:1rem}
	.popup-overlay{position:absolute;inset:0;width:100%;height:100%;border:0;background:rgba(0,0,0,.7);cursor:pointer}
	.popup-panel{position:relative;z-index:1;width:min(100%,38rem);max-height:90vh;overflow:auto;padding:2rem;border-radius:1rem;background:#fff;color:#1f2937;box-shadow:0 24px 64px rgba(0,0,0,.35)}
	.popup-panel h2{margin-top:0}.popup-panel p{white-space:pre-wrap}.popup-panel a{display:inline-block;margin-top:1rem;padding:.75rem 1.25rem;border-radius:.5rem;background:#1d4ed8;color:#fff;text-decoration:none}
	.popup-close{position:absolute;top:.5rem;right:.75rem;border:0;background:none;color:inherit;font-size:1.7rem;cursor:pointer}
	:global(.popup-panel :focus-visible){outline:3px solid #1d4ed8;outline-offset:3px}
</style>

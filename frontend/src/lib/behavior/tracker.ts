/**
 * RevolutionBehavior-L8-System - Client-Side Tracker
 * Lightweight, high-performance behavior tracking engine
 */

import type { BehaviorEvent, BehaviorEventBatch, BehaviorTrackerConfig } from './types';
import { generateSessionId, generateVisitorId } from './utils.js';

const SAFE_TAGS = new Set([
	'a',
	'button',
	'div',
	'span',
	'img',
	'svg',
	'path',
	'li',
	'nav',
	'header',
	'footer',
	'section',
	'article',
	'p',
	'h1',
	'h2',
	'h3',
	'h4',
	'h5',
	'h6',
	'label',
	'form',
	'main',
	'summary',
	'details'
]);
const PRIVATE_PATH =
	/^\/(admin|account|auth|checkout|login|register|dashboard|my|profile|settings|billing|orders|member|members|rooms|classroom|reset-password|verify-email|forgot-password|unsubscribe)(\/|$)/i;

export class BehaviorTracker {
	private config: BehaviorTrackerConfig;
	private sessionId: string;
	private visitorId: string;
	private eventBuffer: BehaviorEvent[] = [];
	private bufferTimer?: number;
	private sessionStartTime: number;
	private sequenceNumber: number = 0;
	private enabled = false;
	private closed = false;
	private discarded = false;
	private listeners: (() => void)[] = [];
	private pendingBatches: BehaviorEventBatch[] = [];
	private delivery: Promise<void> | null = null;
	private deliveryController: AbortController | null = null;
	private hoverTimer?: number;

	// Tracking state
	private maxScrollDepth: number = 0;
	private clickPositions: Array<{ x: number; y: number; time: number }> = [];
	private hoverStartTime?: number;
	private hoverElement?: Element;
	private idleTimer?: number;
	private isIdle: boolean = false;
	private lastScrollY: number = 0;
	private lastScrollTime: number = 0;
	private lastSpeedScrollEvent = 0;
	private lastPagePath = '';

	constructor(config: Partial<BehaviorTrackerConfig> = {}) {
		this.config = {
			apiEndpoint: '/api/behavior/events',
			bufferSize: 20,
			bufferTimeout: 5000,
			trackScrollDepth: true,
			trackRageClicks: true,
			trackHoverIntent: false,
			trackFormBehavior: false,
			trackCursorMovement: false,
			trackIdleTime: false,
			rageClickThreshold: 3,
			rageClickWindow: 1000,
			hoverIntentDuration: 1500,
			idleTimeout: 30000,
			speedScrollThreshold: 3000,
			maskPII: true,
			respectDNT: true,
			anonymizeIP: true,
			sampleRate: 1,
			maxEventsPerSession: 200,
			...config
		};

		this.sessionId = generateSessionId();
		this.visitorId = '';
		this.sessionStartTime = Date.now();

		this.init();
	}

	private init() {
		// Check DNT
		if (
			this.config.respectDNT &&
			(navigator.doNotTrack === '1' ||
				(navigator as Navigator & { globalPrivacyControl?: boolean }).globalPrivacyControl === true)
		) {
			console.info('[BehaviorTracker] DNT enabled, tracking disabled');
			return;
		}

		// Sample rate check
		if (this.config.sampleRate <= 0 || Math.random() >= this.config.sampleRate) {
			console.info('[BehaviorTracker] Session not sampled');
			return;
		}

		this.visitorId = generateVisitorId();
		this.enabled = true;
		this.pendingBatches = this.loadFailedBatches();
		this.persistBatches();
		this.setupEventListeners();
		this.trackPageView();
	}

	private listen<E extends Event>(
		target: EventTarget,
		type: string,
		handler: (event: E) => void,
		options?: boolean | AddEventListenerOptions
	): void {
		const listener: EventListener = (event) => {
			if (this.enabled && !this.closed) handler(event as E);
		};
		target.addEventListener(type, listener, options);
		this.listeners.push(() => target.removeEventListener(type, listener, options));
	}

	private setupEventListeners() {
		// Page visibility
		this.listen(document, 'visibilitychange', this.handleVisibilityChange.bind(this));
		this.listen(window, 'beforeunload', this.handlePageExit.bind(this));

		// Scroll tracking
		if (this.config.trackScrollDepth) {
			this.listen(window, 'scroll', this.handleScroll.bind(this), { passive: true });
		}

		// Click tracking
		this.listen(document, 'click', this.handleClick.bind(this), true);

		// Hover tracking
		if (this.config.trackHoverIntent) {
			this.listen(document, 'mouseover', this.handleMouseOver.bind(this), true);
			this.listen(document, 'mouseout', this.handleMouseOut.bind(this), true);
		}

		// Form tracking
		if (this.config.trackFormBehavior) {
			this.listen(document, 'focus', this.handleFocus.bind(this), true);
			this.listen(document, 'blur', this.handleBlur.bind(this), true);
			this.listen(document, 'submit', this.handleFormSubmit.bind(this), true);
		}

		// Idle tracking
		if (this.config.trackIdleTime) {
			this.resetIdleTimer();
			['mousedown', 'mousemove', 'keypress', 'scroll', 'touchstart'].forEach((event) => {
				this.listen(document, event, this.resetIdleTimer.bind(this), { passive: true });
			});
		}

		// Exit intent
		this.listen(document, 'mouseout', this.handleExitIntent.bind(this));
	}

	private trackPageView() {
		this.lastPagePath = window.location.pathname;
		this.trackEvent({
			event_type: 'page_view',
			timestamp: Date.now(),
			page_url: window.location.href
		});
	}

	public trackNavigation(): void {
		if (!this.enabled || this.closed || this.lastPagePath === window.location.pathname) return;
		this.maxScrollDepth = 0;
		this.lastScrollY = 0;
		this.lastScrollTime = 0;
		this.trackPageView();
	}

	public isActive(): boolean {
		return this.enabled && !this.closed;
	}

	private handleScroll() {
		const scrollY = window.scrollY;
		const scrollHeight = document.documentElement.scrollHeight - window.innerHeight;
		const scrollPercent =
			scrollHeight > 0
				? Math.min(100, Math.max(0, Math.round((scrollY / scrollHeight) * 100)))
				: 100;

		// Track scroll depth milestones
		const milestone = [100, 90, 75, 50, 25].find(
			(depth) => scrollPercent >= depth && this.maxScrollDepth < depth
		);
		if (milestone !== undefined) {
			this.maxScrollDepth = milestone;
			const timeSincePageLoad = Date.now() - this.sessionStartTime;

			this.trackEvent({
				event_type: 'scroll_depth',
				timestamp: Date.now(),
				page_url: window.location.href,
				event_value: milestone,
				event_metadata: {
					depth_percent: scrollPercent,
					time_to_depth: timeSincePageLoad,
					scroll_speed: this.calculateScrollSpeed(scrollY)
				}
			});
		}

		// Detect speed scrolling
		const scrollSpeed = this.calculateScrollSpeed(scrollY);
		if (
			scrollSpeed > this.config.speedScrollThreshold &&
			Date.now() - this.lastSpeedScrollEvent >= 1000
		) {
			this.lastSpeedScrollEvent = Date.now();
			this.trackEvent({
				event_type: 'speed_scroll',
				timestamp: Date.now(),
				page_url: window.location.href,
				event_metadata: {
					speed: scrollSpeed,
					direction: scrollY > this.lastScrollY ? 'down' : 'up',
					scroll_percent: scrollPercent
				}
			});
		}

		// Detect backtracking
		if (scrollY < this.lastScrollY - 500) {
			this.trackEvent({
				event_type: 'scroll_backtrack',
				timestamp: Date.now(),
				page_url: window.location.href,
				event_metadata: {
					distance: this.lastScrollY - scrollY,
					from_percent: Math.round((this.lastScrollY / scrollHeight) * 100),
					to_percent: scrollPercent
				}
			});
		}

		this.lastScrollY = scrollY;
		this.lastScrollTime = Date.now();
	}

	private calculateScrollSpeed(currentY: number): number {
		const timeDiff = Date.now() - this.lastScrollTime;
		if (timeDiff === 0) return 0;
		return Math.abs(currentY - this.lastScrollY) / (timeDiff / 1000);
	}

	private handleClick(event: MouseEvent) {
		const target = event.target as Element;
		const now = Date.now();

		if (
			target.closest(
				'input, textarea, select, [contenteditable], [data-sensitive], [data-behavior-ignore]'
			)
		)
			return;
		// Track click
		this.trackEvent({
			event_type: 'click',
			timestamp: now,
			page_url: window.location.href,
			element: target.tagName,
			element_selector: this.getSelector(target),
			coordinates: {
				x: Math.max(0, Math.min(100, (event.clientX / window.innerWidth) * 100)),
				y: Math.max(0, Math.min(100, (event.clientY / window.innerHeight) * 100))
			},
			coordinate_space: 'viewport_percent'
		});

		// Rage click detection
		if (this.config.trackRageClicks) {
			this.clickPositions.push({ x: event.clientX, y: event.clientY, time: now });
			this.clickPositions = this.clickPositions.filter(
				(click) => now - click.time < this.config.rageClickWindow
			);

			if (this.clickPositions.length >= this.config.rageClickThreshold) {
				const isInSameArea = this.clickPositions.every(
					(click) =>
						Math.abs(click.x - event.clientX) < 50 && Math.abs(click.y - event.clientY) < 50
				);

				if (isInSameArea) {
					this.trackEvent({
						event_type: 'rage_click',
						timestamp: now,
						page_url: window.location.href,
						element: target.tagName,
						element_selector: this.getSelector(target),
						coordinates: {
							x: Math.max(0, Math.min(100, (event.clientX / window.innerWidth) * 100)),
							y: Math.max(0, Math.min(100, (event.clientY / window.innerHeight) * 100))
						},
						coordinate_space: 'viewport_percent',
						event_metadata: {
							click_count: this.clickPositions.length,
							time_window: this.config.rageClickWindow,
							frustration_score: Math.min(this.clickPositions.length * 10, 100),
							element_type: target.tagName
						}
					});
					this.clickPositions = [];
				}
			}
		}

		// CTA tracking
		if (target.matches('button, a, [role="button"]')) {
			const isCTA =
				target.classList.contains('cta') ||
				target.classList.contains('btn-primary') ||
				target.getAttribute('data-cta') === 'true';

			if (isCTA) {
				this.trackEvent({
					event_type: 'cta_click',
					timestamp: now,
					page_url: window.location.href,
					element: target.tagName,
					element_selector: this.getSelector(target)
				});
			}
		}
	}

	private handleMouseOver(event: MouseEvent) {
		const target = event.target as Element;
		this.hoverElement = target;
		this.hoverStartTime = Date.now();

		if (this.hoverTimer) clearTimeout(this.hoverTimer);
		this.hoverTimer = window.setTimeout(() => {
			if (!this.closed && this.hoverElement === target && this.hoverStartTime) {
				const hoverDuration = Date.now() - this.hoverStartTime;

				if (hoverDuration >= this.config.hoverIntentDuration) {
					const intentStrength =
						hoverDuration < 2500 ? 'weak' : hoverDuration < 4000 ? 'moderate' : 'strong';

					this.trackEvent({
						event_type: 'hover_intent',
						timestamp: Date.now(),
						page_url: window.location.href,
						element: target.tagName,
						element_selector: this.getSelector(target),
						event_metadata: {
							hover_duration: hoverDuration,
							cursor_stability: 0.8,
							element_type: target.tagName,
							intent_strength: intentStrength
						}
					});
				}
			}
		}, this.config.hoverIntentDuration);
	}

	private handleMouseOut() {
		if (this.hoverTimer) clearTimeout(this.hoverTimer);
		this.hoverElement = undefined;
		this.hoverStartTime = undefined;
	}

	private handleFocus(event: FocusEvent) {
		const target = event.target as HTMLElement;

		if (target.matches('input, textarea, select')) {
			this.trackEvent({
				event_type: 'form_focus',
				timestamp: Date.now(),
				page_url: window.location.href,
				element: target.tagName,
				element_selector: this.getSelector(target),
				event_metadata: {
					field_name: (target as HTMLInputElement).name,
					field_type: (target as HTMLInputElement).type,
					form_id: (target.closest('form') as HTMLFormElement)?.id
				}
			});
		}
	}

	private handleBlur(event: FocusEvent) {
		const target = event.target as HTMLInputElement;

		if (target.matches('input, textarea, select')) {
			this.trackEvent({
				event_type: 'form_blur',
				timestamp: Date.now(),
				page_url: window.location.href,
				element: target.tagName,
				element_selector: this.getSelector(target),
				event_metadata: {
					field_name: target.name,
					filled: !!target.value,
					value_length: target.value?.length || 0
				}
			});
		}
	}

	private handleFormSubmit(event: Event) {
		const form = event.target as HTMLFormElement;

		this.trackEvent(
			{
				event_type: 'form_submit',
				timestamp: Date.now(),
				page_url: window.location.href,
				element: 'FORM',
				element_selector: this.getSelector(form),
				event_metadata: {
					form_id: form.id,
					fields_count: form.elements.length,
					action: form.action
				}
			},
			true
		); // Immediate dispatch
	}

	private handleVisibilityChange() {
		if (document.hidden) {
			this.trackEvent({
				event_type: 'tab_blur',
				timestamp: Date.now(),
				page_url: window.location.href
			});
			void this.flush();
		} else {
			this.trackEvent({
				event_type: 'tab_focus',
				timestamp: Date.now(),
				page_url: window.location.href
			});
		}
	}

	private handlePageExit() {
		this.trackEvent(
			{
				event_type: 'page_exit',
				timestamp: Date.now(),
				page_url: window.location.href,
				event_metadata: {
					duration: Date.now() - this.sessionStartTime,
					max_scroll_depth: this.maxScrollDepth
				}
			},
			true
		); // Immediate dispatch

		void this.flush();
	}

	private handleExitIntent(event: MouseEvent) {
		if (event.clientY <= 0 && event.relatedTarget === null) {
			this.trackEvent({
				event_type: 'exit_intent',
				timestamp: Date.now(),
				page_url: window.location.href,
				event_metadata: {
					scroll_depth: this.maxScrollDepth,
					time_on_page: Date.now() - this.sessionStartTime
				}
			});
		}
	}

	private resetIdleTimer() {
		if (this.isIdle) {
			this.trackEvent({
				event_type: 'idle_end',
				timestamp: Date.now(),
				page_url: window.location.href
			});
			this.isIdle = false;
		}

		if (this.idleTimer) {
			clearTimeout(this.idleTimer);
		}

		this.idleTimer = window.setTimeout(() => {
			this.isIdle = true;
			this.trackEvent({
				event_type: 'idle_start',
				timestamp: Date.now(),
				page_url: window.location.href
			});
		}, this.config.idleTimeout);
	}

	private getSelector(element: Element): string {
		// IDs and classes often contain account data. Only retain the tag.
		return element.tagName.toLowerCase();
	}

	public trackEvent(event: BehaviorEvent, immediate: boolean = false) {
		if (!this.enabled || this.closed || this.sequenceNumber >= this.config.maxEventsPerSession) {
			return;
		}

		const safe = this.sanitizeEvent(event);
		if (!safe) return;
		this.eventBuffer.push(safe);
		this.sequenceNumber++;

		if (immediate || this.eventBuffer.length >= this.config.bufferSize) {
			void this.flush();
		} else if (!this.bufferTimer) {
			this.bufferTimer = window.setTimeout(() => {
				this.bufferTimer = undefined;
				void this.flush();
			}, this.config.bufferTimeout);
		}
	}

	private sanitizeEvent(event: BehaviorEvent): BehaviorEvent | null {
		let path: string;
		try {
			const url = new URL(event.page_url, window.location.origin);
			if (url.origin !== window.location.origin) return null;
			path = url.pathname;
		} catch {
			return null;
		}
		if (path.length > 500 || PRIVATE_PATH.test(path)) return null;
		const selector = event.element_selector?.toLowerCase();
		return {
			event_type: event.event_type,
			timestamp: event.timestamp,
			page_url: `${window.location.origin}${path}`,
			...(selector && SAFE_TAGS.has(selector) ? { element_selector: selector } : {}),
			...(typeof event.event_value === 'number' && Number.isFinite(event.event_value)
				? { event_value: event.event_value }
				: {}),
			...(event.coordinate_space === 'viewport_percent' &&
			event.coordinates &&
			Number.isFinite(event.coordinates.x) &&
			Number.isFinite(event.coordinates.y)
				? { coordinates: event.coordinates, coordinate_space: 'viewport_percent' as const }
				: {})
		};
	}

	public flush(): Promise<void> {
		if (!this.enabled || this.closed || this.discarded) return Promise.resolve();
		if (this.bufferTimer) {
			clearTimeout(this.bufferTimer);
			this.bufferTimer = undefined;
		}
		// Keep batches within the Rust event and proxy byte limits.
		while (this.eventBuffer.length) {
			let size = Math.min(100, this.eventBuffer.length);
			let batch = this.makeBatch(this.eventBuffer.slice(0, size));
			while (size > 1 && this.batchBytes(batch) > 60 * 1024) {
				size = Math.max(1, Math.floor(size / 2));
				batch = this.makeBatch(this.eventBuffer.slice(0, size));
			}
			this.eventBuffer.splice(0, size);
			if (this.batchBytes(batch) > 60 * 1024) {
				console.warn('[BehaviorTracker] An event exceeded the ingestion byte limit');
				continue;
			}
			this.pendingBatches.push(batch);
		}
		if (this.pendingBatches.length > 5) {
			console.warn('[BehaviorTracker] Retry capacity exceeded; retaining the latest five batches');
			this.pendingBatches = this.delivery
				? [this.pendingBatches[0] as BehaviorEventBatch, ...this.pendingBatches.slice(-4)]
				: this.pendingBatches.slice(-5);
		}
		this.persistBatches();
		if (this.delivery) return this.delivery;
		const delivery = this.deliverBatches();
		this.delivery = delivery;
		const complete = () => {
			if (this.delivery === delivery) this.delivery = null;
		};
		void delivery.then(complete, complete);
		return delivery;
	}

	private batchBytes(batch: BehaviorEventBatch): number {
		try {
			return new TextEncoder().encode(JSON.stringify(batch)).byteLength;
		} catch {
			return Infinity;
		}
	}

	private makeBatch(events: BehaviorEvent[]): BehaviorEventBatch {
		return {
			session_id: this.sessionId,
			visitor_id: this.visitorId,
			events,
			client_timestamp: Date.now()
		};
	}

	private loadFailedBatches(): BehaviorEventBatch[] {
		try {
			const stored: unknown = JSON.parse(localStorage.getItem('behavior_failed_batches') || '[]');
			if (!Array.isArray(stored)) return [];
			return stored
				.filter((value: unknown): value is BehaviorEventBatch => {
					if (
						!value ||
						typeof value !== 'object' ||
						!('session_id' in value) ||
						typeof value.session_id !== 'string' ||
						!('visitor_id' in value) ||
						typeof value.visitor_id !== 'string' ||
						!('events' in value) ||
						!Array.isArray(value.events) ||
						!value.events.length ||
						value.events.length > 100
					)
						return false;
					return value.events.every(
						(event: unknown) =>
							event &&
							typeof event === 'object' &&
							'event_type' in event &&
							typeof event.event_type === 'string' &&
							'page_url' in event &&
							typeof event.page_url === 'string' &&
							'timestamp' in event &&
							typeof event.timestamp === 'number' &&
							Number.isFinite(event.timestamp)
					);
				})
				.slice(-5)
				.map((batch) => ({
					...batch,
					events: batch.events
						.map((event) => this.sanitizeEvent(event))
						.filter((event): event is BehaviorEvent => event !== null)
				}))
				.filter((batch) => batch.events.length > 0);
		} catch {
			return [];
		}
	}

	private persistBatches(): void {
		if (this.discarded) return;
		try {
			if (this.pendingBatches.length)
				localStorage.setItem('behavior_failed_batches', JSON.stringify(this.pendingBatches));
			else localStorage.removeItem('behavior_failed_batches');
		} catch (error) {
			console.warn(
				'[BehaviorTracker] Browser storage unavailable; pending batches remain in memory',
				error
			);
		}
	}

	private async deliverBatches(): Promise<void> {
		while (this.pendingBatches.length && !this.discarded) {
			const batch = this.pendingBatches[0];
			if (!batch) return;
			const controller = new AbortController();
			this.deliveryController = controller;
			try {
				const response = await fetch(this.config.apiEndpoint, {
					method: 'POST',
					headers: { 'Content-Type': 'application/json' },
					body: JSON.stringify(batch),
					keepalive: true,
					signal: AbortSignal.any([controller.signal, AbortSignal.timeout(15000)])
				});
				if (this.discarded) return;
				if (response.status === 400 || response.status === 422 || response.status === 413) {
					console.warn(
						`[BehaviorTracker] Invalid or expired batch rejected (HTTP ${response.status})`
					);
					this.pendingBatches.shift();
					this.persistBatches();
					continue;
				}
				if (!response.ok) throw new Error(`Behavior storage unavailable (HTTP ${response.status})`);
				const receipt: unknown = await response.json();
				if (
					!receipt ||
					typeof receipt !== 'object' ||
					!('success' in receipt) ||
					receipt.success !== true ||
					!('received' in receipt) ||
					receipt.received !== batch.events.length ||
					!('persisted' in receipt) ||
					typeof receipt.persisted !== 'number' ||
					!Number.isInteger(receipt.persisted) ||
					receipt.persisted < 0 ||
					receipt.persisted > batch.events.length
				)
					throw new Error('Behavior storage did not acknowledge the complete batch');
				if (this.discarded) return;
				this.pendingBatches.shift();
				this.persistBatches();
			} catch (error) {
				if (!this.discarded) {
					this.persistBatches();
					if (import.meta.env.DEV && import.meta.env.VITE_DEBUG_BEHAVIOR)
						console.debug('[BehaviorTracker] Batch retained for retry:', error);
				}
				return;
			} finally {
				if (this.deliveryController === controller) this.deliveryController = null;
			}
		}
	}

	public setUserId(_userId: string) {
		// Compatibility shim: identity is never sent with behavior telemetry.
	}

	public destroy(options: { flush?: boolean } = {}): void {
		if (this.closed) return;
		if (options.flush === false) {
			this.discarded = true;
			this.deliveryController?.abort();
			this.eventBuffer = [];
			this.pendingBatches = [];
			try {
				localStorage.removeItem('behavior_failed_batches');
			} catch {
				/* Storage may be disabled. */
			}
		} else {
			void this.flush();
		}
		this.closed = true;
		this.enabled = false;
		for (const remove of this.listeners) remove();
		this.listeners = [];
		if (this.bufferTimer) clearTimeout(this.bufferTimer);
		if (this.idleTimer) clearTimeout(this.idleTimer);
		if (this.hoverTimer) clearTimeout(this.hoverTimer);
		this.hoverElement = undefined;
	}
}

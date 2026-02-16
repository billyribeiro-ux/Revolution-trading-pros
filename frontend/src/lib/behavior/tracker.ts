/**
 * RevolutionBehavior-L8-System - Client-Side Tracker
 * Lightweight, high-performance behavior tracking engine
 */

import type { BehaviorEvent, BehaviorEventBatch, BehaviorTrackerConfig } from './types';
import { generateSessionId, generateVisitorId } from './utils.js';
import { logger } from '$lib/utils/logger';

export class BehaviorTracker {
	private config: BehaviorTrackerConfig;
	private sessionId: string;
	private visitorId: string;
	private userId?: string;
	private eventBuffer: BehaviorEvent[] = [];
	private bufferTimer?: number;
	private sessionStartTime: number;
	private sequenceNumber: number = 0;

	// Tracking state
	private maxScrollDepth: number = 0;
	private clickPositions: Array<{ x: number; y: number; time: number }> = [];
	private hoverStartTime?: number;
	private hoverElement?: Element;
	private idleTimer?: number;
	private isIdle: boolean = false;
	private lastScrollY: number = 0;
	private lastScrollTime: number = 0;

	constructor(config: Partial<BehaviorTrackerConfig> = {}) {
		this.config = {
			apiEndpoint: '/api/behavior/events',
			bufferSize: 20,
			bufferTimeout: 5000,
			trackScrollDepth: true,
			trackRageClicks: true,
			trackHoverIntent: true,
			trackFormBehavior: true,
			trackCursorMovement: false,
			trackIdleTime: true,
			rageClickThreshold: 3,
			rageClickWindow: 1000,
			hoverIntentDuration: 1500,
			idleTimeout: 30000,
			speedScrollThreshold: 3000,
			maskPII: true,
			respectDNT: true,
			anonymizeIP: true,
			sampleRate: 1,
			maxEventsPerSession: 1000,
			...config
		};

		this.sessionId = generateSessionId();
		this.visitorId = generateVisitorId();
		this.sessionStartTime = Date.now();

		this.init();
	}

	private init() {
		// Check DNT
		if (this.config.respectDNT && navigator.doNotTrack === '1') {
			logger.info('[BehaviorTracker] DNT enabled, tracking disabled');
			return;
		}

		// Sample rate check
		if (Math.random() > this.config.sampleRate) {
			logger.info('[BehaviorTracker] Session not sampled');
			return;
		}

		this.setupEventListeners();
		this.trackPageView();
	}

	private setupEventListeners() {
		// Page visibility
		document.addEventListener('visibilitychange', this.handleVisibilityChange.bind(this));
		window.addEventListener('beforeunload', this.handlePageExit.bind(this));

		// Scroll tracking
		if (this.config.trackScrollDepth) {
			window.addEventListener('scroll', this.handleScroll.bind(this), { passive: true });
		}

		// Click tracking
		document.addEventListener('click', this.handleClick.bind(this), true);

		// Hover tracking
		if (this.config.trackHoverIntent) {
			document.addEventListener('mouseover', this.handleMouseOver.bind(this), true);
			document.addEventListener('mouseout', this.handleMouseOut.bind(this), true);
		}

		// Form tracking
		if (this.config.trackFormBehavior) {
			document.addEventListener('focus', this.handleFocus.bind(this), true);
			document.addEventListener('blur', this.handleBlur.bind(this), true);
			document.addEventListener('submit', this.handleFormSubmit.bind(this), true);
		}

		// Idle tracking
		if (this.config.trackIdleTime) {
			this.resetIdleTimer();
			['mousedown', 'mousemove', 'keypress', 'scroll', 'touchstart'].forEach((event) => {
				document.addEventListener(event, this.resetIdleTimer.bind(this), { passive: true });
			});
		}

		// Exit intent
		document.addEventListener('mouseout', this.handleExitIntent.bind(this));
	}

	private trackPageView() {
		this.trackEvent({
			event_type: 'page_view',
			timestamp: Date.now(),
			page_url: window.location.href,
			event_metadata: {
				referrer: document.referrer,
				entry_type: this.getEntryType(),
				viewport: {
					width: window.innerWidth,
					height: window.innerHeight
				},
				utm_source: new URLSearchParams(window.location.search).get('utm_source') || undefined,
				utm_campaign: new URLSearchParams(window.location.search).get('utm_campaign') || undefined,
				utm_medium: new URLSearchParams(window.location.search).get('utm_medium') || undefined
			}
		});
	}

	private handleScroll() {
		const scrollY = window.scrollY;
		const scrollHeight = document.documentElement.scrollHeight - window.innerHeight;
		const scrollPercent = Math.round((scrollY / scrollHeight) * 100);

		// Track scroll depth milestones
		if (scrollPercent > this.maxScrollDepth && [25, 50, 75, 90, 100].includes(scrollPercent)) {
			this.maxScrollDepth = scrollPercent;
			const timeSincePageLoad = Date.now() - this.sessionStartTime;

			this.trackEvent({
				event_type: 'scroll_depth',
				timestamp: Date.now(),
				page_url: window.location.href,
				event_value: scrollPercent,
				event_metadata: {
					depth_percent: scrollPercent,
					time_to_depth: timeSincePageLoad,
					scroll_speed: this.calculateScrollSpeed(scrollY)
				}
			});
		}

		// Detect speed scrolling
		const scrollSpeed = this.calculateScrollSpeed(scrollY);
		if (scrollSpeed > this.config.speedScrollThreshold) {
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

		// Track click
		this.trackEvent({
			event_type: 'click',
			timestamp: now,
			page_url: window.location.href,
			element: target.tagName,
			element_selector: this.getSelector(target),
			coordinates: { x: event.clientX, y: event.clientY }
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
						coordinates: { x: event.clientX, y: event.clientY },
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

		setTimeout(() => {
			if (this.hoverElement === target && this.hoverStartTime) {
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

		this.flush();
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

	private getEntryType(): 'direct' | 'referral' | 'search' | 'social' | 'campaign' {
		const referrer = document.referrer;
		const hasUTM = window.location.search.includes('utm_');

		if (hasUTM) return 'campaign';
		if (!referrer) return 'direct';
		if (referrer.includes('google') || referrer.includes('bing')) return 'search';
		if (
			referrer.includes('facebook') ||
			referrer.includes('twitter') ||
			referrer.includes('linkedin')
		)
			return 'social';
		return 'referral';
	}

	private getSelector(element: Element): string {
		if (element.id) return `#${element.id}`;
		// Handle SVG elements which have className as SVGAnimatedString, not string
		if (element.className && typeof element.className === 'string') {
			return `.${element.className.split(' ')[0]}`;
		}
		return element.tagName.toLowerCase();
	}

	public trackEvent(event: BehaviorEvent, immediate: boolean = false) {
		if (this.eventBuffer.length >= this.config.maxEventsPerSession) {
			return;
		}

		this.eventBuffer.push(event);
		this.sequenceNumber++;

		if (immediate || this.eventBuffer.length >= this.config.bufferSize) {
			this.flush();
		} else if (!this.bufferTimer) {
			this.bufferTimer = window.setTimeout(() => this.flush(), this.config.bufferTimeout);
		}
	}

	public async flush() {
		if (this.eventBuffer.length === 0) return;

		const batch: BehaviorEventBatch = {
			session_id: this.sessionId,
			visitor_id: this.visitorId,
			user_id: this.userId,
			events: [...this.eventBuffer],
			client_timestamp: Date.now()
		};

		this.eventBuffer = [];

		if (this.bufferTimer) {
			clearTimeout(this.bufferTimer);
			this.bufferTimer = undefined;
		}

		try {
			const response = await fetch(this.config.apiEndpoint, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify(batch),
				keepalive: true
			});

			// Silently handle non-OK responses (endpoint may not exist)
			if (!response.ok && response.status !== 404) {
				logger.debug('[BehaviorTracker] Server returned:', response.status);
			}
		} catch (error) {
			// Silently fail - behavior tracking is non-critical
			// Only log in debug mode to avoid console spam
			if (import.meta.env.DEV && import.meta.env.VITE_DEBUG_BEHAVIOR) {
				logger.debug('[BehaviorTracker] Failed to send events:', error);
			}
			// Store in localStorage for retry when endpoint becomes available
			this.storeFailedBatch(batch);
		}
	}

	private storeFailedBatch(batch: BehaviorEventBatch) {
		try {
			const stored = localStorage.getItem('behavior_failed_batches');
			const batches = stored ? JSON.parse(stored) : [];
			batches.push(batch);
			localStorage.setItem('behavior_failed_batches', JSON.stringify(batches.slice(-5)));
		} catch (_e) {
			// Ignore storage errors
		}
	}

	public setUserId(userId: string) {
		this.userId = userId;
	}

	public destroy() {
		this.flush();
		if (this.bufferTimer) clearTimeout(this.bufferTimer);
		if (this.idleTimer) clearTimeout(this.idleTimer);
	}
}

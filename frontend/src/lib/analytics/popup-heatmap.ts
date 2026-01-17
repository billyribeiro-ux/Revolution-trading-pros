/**
 * Popup Heatmap & Click Tracking Service
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Enterprise-grade click and interaction tracking for popups:
 * - Click position tracking within popup bounds
 * - Scroll depth within popup content
 * - Form field interaction analysis
 * - Mouse movement heatmap data
 * - Touch event tracking for mobile
 * - Attention/gaze estimation
 * - Element visibility tracking
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 * @license MIT
 */

import { browser } from '$app/environment';

// ═══════════════════════════════════════════════════════════════════════════
// Types
// ═══════════════════════════════════════════════════════════════════════════

export interface ClickData {
	x: number;
	y: number;
	relativeX: number; // 0-100% of popup width
	relativeY: number; // 0-100% of popup height
	timestamp: number;
	elementTag: string;
	elementId?: string;
	elementClass?: string;
	elementText?: string;
	isButton: boolean;
	isLink: boolean;
	isFormElement: boolean;
}

export interface ScrollData {
	depth: number; // 0-100%
	maxDepth: number;
	timestamp: number;
	direction: 'up' | 'down';
	velocity: number;
}

export interface FormInteraction {
	fieldName: string;
	fieldType: string;
	action: 'focus' | 'blur' | 'input' | 'change';
	timestamp: number;
	timeSpent?: number;
	hasValue: boolean;
}

export interface MouseMovement {
	x: number;
	y: number;
	timestamp: number;
}

export interface HeatmapData {
	popupId: string;
	sessionId: string;
	clicks: ClickData[];
	scrolls: ScrollData[];
	formInteractions: FormInteraction[];
	mouseMovements: MouseMovement[];
	totalTimeSpent: number;
	engagementScore: number;
	attentionZones: AttentionZone[];
}

export interface AttentionZone {
	x: number;
	y: number;
	width: number;
	height: number;
	timeSpent: number;
	interactions: number;
}

// ═══════════════════════════════════════════════════════════════════════════
// Popup Heatmap Tracker Class
// ═══════════════════════════════════════════════════════════════════════════

export class PopupHeatmapTracker {
	private popupId: string;
	private sessionId: string;
	private popupElement: HTMLElement | null = null;
	private startTime: number = 0;
	private clicks: ClickData[] = [];
	private scrolls: ScrollData[] = [];
	private formInteractions: FormInteraction[] = [];
	private mouseMovements: MouseMovement[] = [];
	private maxScrollDepth: number = 0;
	private lastScrollPosition: number = 0;
	private lastScrollTime: number = 0;
	private fieldFocusTimes: Map<string, number> = new Map();
	private isTracking: boolean = false;
	private mouseThrottleTimer: number | null = null;

	// Store bound event handlers for proper cleanup
	private boundHandleClick: (event: MouseEvent) => void;
	private boundHandleTouch: (event: TouchEvent) => void;
	private boundHandleScroll: (event: Event) => void;
	private boundHandleMouseMove: (event: MouseEvent) => void;

	constructor(popupId: string, sessionId?: string) {
		this.popupId = popupId;
		this.sessionId = sessionId || this.generateSessionId();

		// Bind event handlers once for proper cleanup
		this.boundHandleClick = this.handleClick.bind(this);
		this.boundHandleTouch = this.handleTouch.bind(this);
		this.boundHandleScroll = this.handleScroll.bind(this);
		this.boundHandleMouseMove = this.handleMouseMove.bind(this);
	}

	/**
	 * Start tracking interactions on a popup element.
	 */
	start(element: HTMLElement): void {
		if (!browser || this.isTracking) return;

		this.popupElement = element;
		this.startTime = Date.now();
		this.isTracking = true;

		// Attach event listeners
		this.attachClickListener();
		this.attachScrollListener();
		this.attachFormListeners();
		this.attachMouseMoveListener();

		console.debug('[HeatmapTracker] Started tracking popup:', this.popupId);
	}

	/**
	 * Stop tracking and return collected data.
	 */
	stop(): HeatmapData {
		if (!browser) {
			return this.getEmptyHeatmapData();
		}

		this.isTracking = false;
		this.removeEventListeners();

		const totalTimeSpent = Date.now() - this.startTime;
		const engagementScore = this.calculateEngagementScore(totalTimeSpent);
		const attentionZones = this.calculateAttentionZones();

		const data: HeatmapData = {
			popupId: this.popupId,
			sessionId: this.sessionId,
			clicks: this.clicks,
			scrolls: this.scrolls,
			formInteractions: this.formInteractions,
			mouseMovements: this.mouseMovements,
			totalTimeSpent,
			engagementScore,
			attentionZones
		};

		console.debug('[HeatmapTracker] Stopped tracking, data collected:', {
			clicks: this.clicks.length,
			scrolls: this.scrolls.length,
			formInteractions: this.formInteractions.length,
			mouseMovements: this.mouseMovements.length,
			engagementScore
		});

		return data;
	}

	/**
	 * Get current heatmap data without stopping.
	 */
	getCurrentData(): HeatmapData {
		const totalTimeSpent = Date.now() - this.startTime;
		const engagementScore = this.calculateEngagementScore(totalTimeSpent);
		const attentionZones = this.calculateAttentionZones();

		return {
			popupId: this.popupId,
			sessionId: this.sessionId,
			clicks: [...this.clicks],
			scrolls: [...this.scrolls],
			formInteractions: [...this.formInteractions],
			mouseMovements: [...this.mouseMovements],
			totalTimeSpent,
			engagementScore,
			attentionZones
		};
	}

	// ─────────────────────────────────────────────────────────────────────────
	// Event Listeners
	// ─────────────────────────────────────────────────────────────────────────

	private attachClickListener(): void {
		if (!this.popupElement) return;

		this.popupElement.addEventListener('click', this.boundHandleClick, true);
		this.popupElement.addEventListener('touchend', this.boundHandleTouch, true);
	}

	private attachScrollListener(): void {
		if (!this.popupElement) return;

		// Find scrollable content within popup
		const scrollContainer =
			this.popupElement.querySelector('[data-popup-content]') ||
			this.popupElement.querySelector('.popup-content') ||
			this.popupElement;

		scrollContainer.addEventListener('scroll', this.boundHandleScroll, { passive: true });
	}

	private attachFormListeners(): void {
		if (!this.popupElement) return;

		const formElements = this.popupElement.querySelectorAll('input, textarea, select');

		formElements.forEach((element) => {
			element.addEventListener('focus', this.handleFormFocus.bind(this) as EventListener);
			element.addEventListener('blur', this.handleFormBlur.bind(this) as EventListener);
			element.addEventListener('input', this.handleFormInput.bind(this) as EventListener);
			element.addEventListener('change', this.handleFormChange.bind(this) as EventListener);
		});
	}

	private attachMouseMoveListener(): void {
		if (!this.popupElement) return;

		this.popupElement.addEventListener('mousemove', this.boundHandleMouseMove, { passive: true });
	}

	private removeEventListeners(): void {
		if (!this.popupElement) return;

		this.popupElement.removeEventListener('click', this.boundHandleClick, true);
		this.popupElement.removeEventListener('touchend', this.boundHandleTouch, true);
		this.popupElement.removeEventListener('scroll', this.boundHandleScroll);
		this.popupElement.removeEventListener('mousemove', this.boundHandleMouseMove);

		// Remove form listeners
		const formElements = this.popupElement.querySelectorAll('input, textarea, select');
		formElements.forEach((element) => {
			element.removeEventListener('focus', this.handleFormFocus.bind(this) as EventListener);
			element.removeEventListener('blur', this.handleFormBlur.bind(this) as EventListener);
			element.removeEventListener('input', this.handleFormInput.bind(this) as EventListener);
			element.removeEventListener('change', this.handleFormChange.bind(this) as EventListener);
		});
	}

	// ─────────────────────────────────────────────────────────────────────────
	// Event Handlers
	// ─────────────────────────────────────────────────────────────────────────

	private handleClick(event: MouseEvent): void {
		if (!this.popupElement || !this.isTracking) return;

		const rect = this.popupElement.getBoundingClientRect();
		const target = event.target as HTMLElement;

		const elementId = target.id || undefined;
		const elementClass = target.className || undefined;
		const elementText = target.textContent?.substring(0, 50) || undefined;

		const clickData: ClickData = {
			x: event.clientX - rect.left,
			y: event.clientY - rect.top,
			relativeX: ((event.clientX - rect.left) / rect.width) * 100,
			relativeY: ((event.clientY - rect.top) / rect.height) * 100,
			timestamp: Date.now(),
			elementTag: target.tagName.toLowerCase(),
			...(elementId && { elementId }),
			...(elementClass && { elementClass }),
			...(elementText && { elementText }),
			isButton: target.tagName === 'BUTTON' || target.getAttribute('role') === 'button',
			isLink: target.tagName === 'A',
			isFormElement: ['INPUT', 'TEXTAREA', 'SELECT'].includes(target.tagName)
		};

		this.clicks.push(clickData);
	}

	private handleTouch(event: TouchEvent): void {
		if (!this.popupElement || !this.isTracking || !event.changedTouches[0]) return;

		const touch = event.changedTouches[0];
		const rect = this.popupElement.getBoundingClientRect();
		const target = event.target as HTMLElement;

		const elementId = target.id || undefined;
		const elementClass = target.className || undefined;
		const elementText = target.textContent?.substring(0, 50) || undefined;

		const clickData: ClickData = {
			x: touch.clientX - rect.left,
			y: touch.clientY - rect.top,
			relativeX: ((touch.clientX - rect.left) / rect.width) * 100,
			relativeY: ((touch.clientY - rect.top) / rect.height) * 100,
			timestamp: Date.now(),
			elementTag: target.tagName.toLowerCase(),
			...(elementId && { elementId }),
			...(elementClass && { elementClass }),
			...(elementText && { elementText }),
			isButton: target.tagName === 'BUTTON' || target.getAttribute('role') === 'button',
			isLink: target.tagName === 'A',
			isFormElement: ['INPUT', 'TEXTAREA', 'SELECT'].includes(target.tagName)
		};

		this.clicks.push(clickData);
	}

	private handleScroll(event: Event): void {
		if (!this.isTracking) return;

		const target = event.target as HTMLElement;
		const scrollTop = target.scrollTop;
		const scrollHeight = target.scrollHeight - target.clientHeight;
		const depth = scrollHeight > 0 ? (scrollTop / scrollHeight) * 100 : 0;

		const now = Date.now();
		const timeDiff = now - this.lastScrollTime;
		const scrollDiff = scrollTop - this.lastScrollPosition;
		const velocity = timeDiff > 0 ? Math.abs(scrollDiff) / timeDiff : 0;

		this.maxScrollDepth = Math.max(this.maxScrollDepth, depth);

		const scrollData: ScrollData = {
			depth,
			maxDepth: this.maxScrollDepth,
			timestamp: now,
			direction: scrollDiff > 0 ? 'down' : 'up',
			velocity
		};

		this.scrolls.push(scrollData);
		this.lastScrollPosition = scrollTop;
		this.lastScrollTime = now;
	}

	private handleFormFocus(event: FocusEvent): void {
		if (!this.isTracking) return;

		const target = event.target as HTMLInputElement;
		const fieldName = target.name || target.id || 'unknown';

		this.fieldFocusTimes.set(fieldName, Date.now());

		this.formInteractions.push({
			fieldName,
			fieldType: target.type || target.tagName.toLowerCase(),
			action: 'focus',
			timestamp: Date.now(),
			hasValue: Boolean(target.value)
		});
	}

	private handleFormBlur(event: FocusEvent): void {
		if (!this.isTracking) return;

		const target = event.target as HTMLInputElement;
		const fieldName = target.name || target.id || 'unknown';

		const focusTime = this.fieldFocusTimes.get(fieldName);
		const timeSpent = focusTime ? Date.now() - focusTime : undefined;

		this.formInteractions.push({
			fieldName,
			fieldType: target.type || target.tagName.toLowerCase(),
			action: 'blur',
			timestamp: Date.now(),
			...(timeSpent !== undefined && { timeSpent }),
			hasValue: Boolean(target.value)
		});
	}

	private handleFormInput(event: Event): void {
		if (!this.isTracking) return;

		const target = event.target as HTMLInputElement;
		const fieldName = target.name || target.id || 'unknown';

		this.formInteractions.push({
			fieldName,
			fieldType: target.type || target.tagName.toLowerCase(),
			action: 'input',
			timestamp: Date.now(),
			hasValue: Boolean(target.value)
		});
	}

	private handleFormChange(event: Event): void {
		if (!this.isTracking) return;

		const target = event.target as HTMLInputElement;
		const fieldName = target.name || target.id || 'unknown';

		this.formInteractions.push({
			fieldName,
			fieldType: target.type || target.tagName.toLowerCase(),
			action: 'change',
			timestamp: Date.now(),
			hasValue: Boolean(target.value)
		});
	}

	private handleMouseMove(event: MouseEvent): void {
		if (!this.popupElement || !this.isTracking) return;

		// Throttle mouse movements to every 100ms
		if (this.mouseThrottleTimer) return;

		this.mouseThrottleTimer = window.setTimeout(() => {
			this.mouseThrottleTimer = null;
		}, 100);

		const rect = this.popupElement.getBoundingClientRect();

		// Only store sampled data to avoid memory bloat
		if (this.mouseMovements.length < 1000) {
			this.mouseMovements.push({
				x: ((event.clientX - rect.left) / rect.width) * 100,
				y: ((event.clientY - rect.top) / rect.height) * 100,
				timestamp: Date.now()
			});
		}
	}

	// ─────────────────────────────────────────────────────────────────────────
	// Analysis Functions
	// ─────────────────────────────────────────────────────────────────────────

	private calculateEngagementScore(totalTimeSpent: number): number {
		let score = 0;

		// Time engagement (max 30 points)
		const timeScore = Math.min(30, (totalTimeSpent / 1000) * 2); // 2 points per second, max 15 seconds
		score += timeScore;

		// Click engagement (max 25 points)
		const clickScore = Math.min(25, this.clicks.length * 5);
		score += clickScore;

		// Scroll engagement (max 20 points)
		const scrollScore = (this.maxScrollDepth / 100) * 20;
		score += scrollScore;

		// Form engagement (max 25 points)
		const formScore = Math.min(25, this.formInteractions.length * 3);
		score += formScore;

		return Math.min(100, Math.round(score));
	}

	private calculateAttentionZones(): AttentionZone[] {
		// Divide popup into 3x3 grid and calculate attention per zone
		const zones: AttentionZone[] = [];
		const gridSize = 3;

		for (let row = 0; row < gridSize; row++) {
			for (let col = 0; col < gridSize; col++) {
				const xStart = (col / gridSize) * 100;
				const xEnd = ((col + 1) / gridSize) * 100;
				const yStart = (row / gridSize) * 100;
				const yEnd = ((row + 1) / gridSize) * 100;

				// Count interactions in this zone
				const clicksInZone = this.clicks.filter(
					(c) =>
						c.relativeX >= xStart &&
						c.relativeX < xEnd &&
						c.relativeY >= yStart &&
						c.relativeY < yEnd
				).length;

				const mouseTimeInZone =
					this.mouseMovements.filter(
						(m) => m.x >= xStart && m.x < xEnd && m.y >= yStart && m.y < yEnd
					).length * 100; // Approximate ms based on sample rate

				zones.push({
					x: xStart,
					y: yStart,
					width: 100 / gridSize,
					height: 100 / gridSize,
					timeSpent: mouseTimeInZone,
					interactions: clicksInZone
				});
			}
		}

		return zones;
	}

	private generateSessionId(): string {
		return `hm_${Date.now().toString(36)}_${Math.random().toString(36).substring(2, 8)}`;
	}

	private getEmptyHeatmapData(): HeatmapData {
		return {
			popupId: this.popupId,
			sessionId: this.sessionId,
			clicks: [],
			scrolls: [],
			formInteractions: [],
			mouseMovements: [],
			totalTimeSpent: 0,
			engagementScore: 0,
			attentionZones: []
		};
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// Factory Functions
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Create a new heatmap tracker for a popup.
 */
export function createPopupHeatmapTracker(
	popupId: string,
	sessionId?: string
): PopupHeatmapTracker {
	return new PopupHeatmapTracker(popupId, sessionId);
}

/**
 * Generate heatmap visualization data for rendering.
 */
export function generateHeatmapVisualization(
	data: HeatmapData,
	width: number,
	height: number
): { x: number; y: number; intensity: number }[] {
	const points: { x: number; y: number; intensity: number }[] = [];

	// Combine clicks and mouse movements with different weights
	data.clicks.forEach((click) => {
		points.push({
			x: (click.relativeX / 100) * width,
			y: (click.relativeY / 100) * height,
			intensity: 1.0 // Clicks are high intensity
		});
	});

	data.mouseMovements.forEach((move) => {
		points.push({
			x: (move.x / 100) * width,
			y: (move.y / 100) * height,
			intensity: 0.3 // Mouse movements are lower intensity
		});
	});

	return points;
}

export default PopupHeatmapTracker;

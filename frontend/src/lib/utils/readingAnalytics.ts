/**
 * Reading Analytics - Track user engagement with blog posts
 * Svelte 5 / SvelteKit Compatible
 *
 * @version 1.0.0 - December 2024
 *
 * Tracks:
 * - Scroll depth (25%, 50%, 75%, 100%)
 * - Read completion (based on time + scroll)
 * - Time on page
 * - Engagement score
 */

interface ReadingMetrics {
	postId: string | number;
	slug: string;
	scrollDepth: number;
	maxScrollDepth: number;
	timeOnPage: number;
	readCompletion: number;
	engagementScore: number;
	milestones: Set<number>;
}

interface ReadingAnalyticsOptions {
	/** Analytics endpoint URL */
	endpoint?: string;
	/** Minimum time (ms) before tracking starts */
	minTime?: number;
	/** Scroll depth milestones to track */
	milestones?: number[];
	/** Enable console logging */
	debug?: boolean;
	/** Callback for each milestone */
	onMilestone?: (depth: number, metrics: ReadingMetrics) => void;
	/** Callback on completion */
	onComplete?: (metrics: ReadingMetrics) => void;
}

const DEFAULT_OPTIONS: ReadingAnalyticsOptions = {
	endpoint: '/api/analytics/reading',
	minTime: 5000,
	milestones: [25, 50, 75, 100],
	debug: false,
};

/**
 * Calculate estimated reading time for content
 * @param text - Content text
 * @param wordsPerMinute - Reading speed (default 200 WPM)
 * @returns Reading time in minutes
 */
export function calculateReadingTime(text: string, wordsPerMinute = 200): number {
	const words = text.trim().split(/\s+/).length;
	return Math.ceil(words / wordsPerMinute);
}

/**
 * Format reading time for display
 */
export function formatReadingTime(minutes: number): string {
	if (minutes < 1) return 'Less than 1 min read';
	if (minutes === 1) return '1 min read';
	return `${minutes} min read`;
}

/**
 * Initialize reading analytics for a blog post
 *
 * @example
 * ```ts
 * import { initReadingAnalytics } from '$lib/utils/readingAnalytics';
 *
 * // In blog post component
 * $effect(() => {
 *   if (browser && post) {
 *     const cleanup = initReadingAnalytics({
 *       postId: post.id,
 *       slug: post.slug,
 *       contentSelector: '.post-content',
 *       options: { debug: import.meta.env.DEV }
 *     });
 *     return cleanup;
 *   }
 * });
 * ```
 */
export function initReadingAnalytics(config: {
	postId: string | number;
	slug: string;
	contentSelector?: string;
	options?: ReadingAnalyticsOptions;
}): () => void {
	if (typeof window === 'undefined') return () => {};

	const { postId, slug, contentSelector = '.post-body', options = {} } = config;
	const opts = { ...DEFAULT_OPTIONS, ...options };

	const metrics: ReadingMetrics = {
		postId,
		slug,
		scrollDepth: 0,
		maxScrollDepth: 0,
		timeOnPage: 0,
		readCompletion: 0,
		engagementScore: 0,
		milestones: new Set(),
	};

	const startTime = Date.now();
	let rafId: number | null = null;
	let reported = false;

	const contentElement = document.querySelector(contentSelector);
	if (!contentElement) {
		if (opts.debug) console.warn('[Reading Analytics] Content element not found:', contentSelector);
		return () => {};
	}

	/**
	 * Calculate current scroll depth percentage
	 */
	function getScrollDepth(): number {
		const rect = contentElement!.getBoundingClientRect();
		const viewportHeight = window.innerHeight;
		const contentTop = rect.top + window.scrollY;
		const contentHeight = rect.height;
		const scrollPosition = window.scrollY + viewportHeight;

		const visibleContent = Math.max(0, scrollPosition - contentTop);
		const depth = Math.min(100, (visibleContent / contentHeight) * 100);

		return Math.round(depth);
	}

	/**
	 * Calculate engagement score (0-100)
	 * Based on: scroll depth (40%), time spent (40%), interaction (20%)
	 */
	function calculateEngagementScore(): number {
		const timeScore = Math.min(100, (metrics.timeOnPage / 120000) * 100); // 2 min = 100%
		const scrollScore = metrics.maxScrollDepth;
		const interactionBonus = metrics.milestones.size * 5; // 5 points per milestone

		return Math.round(scrollScore * 0.4 + timeScore * 0.4 + interactionBonus * 0.2);
	}

	/**
	 * Calculate read completion (0-100)
	 * Requires both scroll depth AND time spent
	 */
	function calculateReadCompletion(): number {
		const expectedReadTime = 60000; // Assume 1 minute minimum for completion
		const timeProgress = Math.min(100, (metrics.timeOnPage / expectedReadTime) * 100);
		const scrollProgress = metrics.maxScrollDepth;

		// Both factors contribute, but scroll is weighted more
		return Math.round(scrollProgress * 0.7 + timeProgress * 0.3);
	}

	/**
	 * Update metrics and check milestones
	 */
	function updateMetrics(): void {
		const currentDepth = getScrollDepth();
		metrics.scrollDepth = currentDepth;
		metrics.maxScrollDepth = Math.max(metrics.maxScrollDepth, currentDepth);
		metrics.timeOnPage = Date.now() - startTime;
		metrics.engagementScore = calculateEngagementScore();
		metrics.readCompletion = calculateReadCompletion();

		// Check milestones
		for (const milestone of opts.milestones!) {
			if (currentDepth >= milestone && !metrics.milestones.has(milestone)) {
				metrics.milestones.add(milestone);

				if (opts.debug) {
					console.log(`[Reading Analytics] Milestone reached: ${milestone}%`, {
						timeOnPage: `${(metrics.timeOnPage / 1000).toFixed(1)}s`,
						engagement: metrics.engagementScore,
					});
				}

				// Fire milestone callback
				opts.onMilestone?.(milestone, { ...metrics, milestones: new Set(metrics.milestones) });

				// Send milestone event
				sendEvent('milestone', { milestone });

				// Check for completion (100% scroll + minimum time)
				if (milestone === 100 && metrics.timeOnPage >= opts.minTime!) {
					handleCompletion();
				}
			}
		}
	}

	/**
	 * Handle read completion
	 */
	function handleCompletion(): void {
		if (reported) return;
		reported = true;

		if (opts.debug) {
			console.log('[Reading Analytics] Read completed!', {
				timeOnPage: `${(metrics.timeOnPage / 1000).toFixed(1)}s`,
				engagement: metrics.engagementScore,
				completion: metrics.readCompletion,
			});
		}

		opts.onComplete?.({ ...metrics, milestones: new Set(metrics.milestones) });
		sendEvent('completion', {
			timeOnPage: metrics.timeOnPage,
			engagementScore: metrics.engagementScore,
			readCompletion: metrics.readCompletion,
		});
	}

	/**
	 * Send analytics event
	 */
	function sendEvent(eventType: string, data: Record<string, unknown>): void {
		if (!opts.endpoint) return;

		const payload = {
			event: `reading_${eventType}`,
			postId: metrics.postId,
			slug: metrics.slug,
			...data,
			timestamp: Date.now(),
			url: window.location.href,
		};

		// Use sendBeacon for reliability
		if (navigator.sendBeacon) {
			navigator.sendBeacon(opts.endpoint, JSON.stringify(payload));
		} else {
			fetch(opts.endpoint, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify(payload),
				keepalive: true,
			}).catch(() => {});
		}
	}

	/**
	 * Throttled scroll handler
	 */
	function handleScroll(): void {
		if (rafId !== null) return;

		rafId = requestAnimationFrame(() => {
			updateMetrics();
			rafId = null;
		});
	}

	/**
	 * Handle page visibility change
	 */
	function handleVisibilityChange(): void {
		if (document.visibilityState === 'hidden') {
			// Send final metrics when user leaves
			sendEvent('leave', {
				maxScrollDepth: metrics.maxScrollDepth,
				timeOnPage: metrics.timeOnPage,
				engagementScore: metrics.engagementScore,
				readCompletion: metrics.readCompletion,
				milestonesReached: Array.from(metrics.milestones),
			});
		}
	}

	// Attach event listeners
	window.addEventListener('scroll', handleScroll, { passive: true });
	document.addEventListener('visibilitychange', handleVisibilityChange);

	// Initial update
	updateMetrics();

	if (opts.debug) {
		console.log('[Reading Analytics] Initialized for:', slug);
	}

	// Return cleanup function
	return () => {
		window.removeEventListener('scroll', handleScroll);
		document.removeEventListener('visibilitychange', handleVisibilityChange);
		if (rafId !== null) {
			cancelAnimationFrame(rafId);
		}

		// Send final metrics on cleanup
		if (!reported && metrics.timeOnPage >= opts.minTime!) {
			sendEvent('leave', {
				maxScrollDepth: metrics.maxScrollDepth,
				timeOnPage: metrics.timeOnPage,
				engagementScore: metrics.engagementScore,
				readCompletion: metrics.readCompletion,
				milestonesReached: Array.from(metrics.milestones),
			});
		}
	};
}

export type { ReadingMetrics, ReadingAnalyticsOptions };

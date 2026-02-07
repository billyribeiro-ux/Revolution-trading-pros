<script lang="ts">
	/**
	 * Social Share Component - Svelte 5
	 * Share buttons with tracking for blog posts
	 *
	 * @version 2.0.0 - January 2026
	 * Updated: CSS layers, oklch colors, container queries, modern patterns
	 */

	interface Props {
		/** URL to share (defaults to current page) */
		url?: string;
		/** Title for sharing */
		title: string;
		/** Description/excerpt for sharing */
		description?: string;
		/** Image URL for social cards */
		image?: string;
		/** Hashtags for Twitter (comma-separated) */
		hashtags?: string;
		/** Twitter handle without @ */
		via?: string;
		/** Layout: 'horizontal' or 'vertical' */
		layout?: 'horizontal' | 'vertical';
		/** Button size: 'small', 'medium', 'large' */
		size?: 'small' | 'medium' | 'large';
		/** Show share count (requires backend) */
		showCount?: boolean;
		/** Analytics tracking callback */
		onShare?: (platform: string) => void;
	}

	let props: Props = $props();

	// Destructure with defaults for local use
	const url = $derived(props.url ?? (typeof window !== 'undefined' ? window.location.href : ''));
	const title = $derived(props.title);
	const description = $derived(props.description ?? '');
	// @ts-ignore write-only state
	const _image = $derived(props.image ?? '');
	const hashtags = $derived(props.hashtags ?? '');
	const via = $derived(props.via ?? 'RevTradingPros');
	const layout = $derived(props.layout ?? 'horizontal');
	const size = $derived(props.size ?? 'medium');
	const showCount = $derived(props.showCount ?? false);
	const onShare = $derived(props.onShare);

	// Encode values for URLs
	let encodedUrl = $derived(encodeURIComponent(url));
	let encodedTitle = $derived(encodeURIComponent(title));
	let encodedDescription = $derived(encodeURIComponent(description));

	// Share URLs
	let shareUrls = $derived({
		twitter: `https://twitter.com/intent/tweet?url=${encodedUrl}&text=${encodedTitle}${hashtags ? `&hashtags=${encodeURIComponent(hashtags)}` : ''}${via ? `&via=${via}` : ''}`,
		facebook: `https://www.facebook.com/sharer/sharer.php?u=${encodedUrl}`,
		linkedin: `https://www.linkedin.com/sharing/share-offsite/?url=${encodedUrl}`,
		reddit: `https://reddit.com/submit?url=${encodedUrl}&title=${encodedTitle}`,
		email: `mailto:?subject=${encodedTitle}&body=${encodedDescription}%0A%0A${encodedUrl}`,
		copy: url
	});

	let copied = $state(false);

	function handleShare(platform: string, shareUrl: string) {
		// Track the share
		onShare?.(platform);
		trackShare(platform);

		if (platform === 'copy') {
			copyToClipboard();
			return;
		}

		// Open share window
		const width = 600;
		const height = 400;
		const left = (window.innerWidth - width) / 2;
		const top = (window.innerHeight - height) / 2;

		window.open(
			shareUrl,
			`share-${platform}`,
			`width=${width},height=${height},left=${left},top=${top},toolbar=0,menubar=0`
		);
	}

	async function copyToClipboard() {
		try {
			await navigator.clipboard.writeText(url);
			copied = true;
			setTimeout(() => {
				copied = false;
			}, 2000);
		} catch {
			// Fallback for older browsers
			const textarea = document.createElement('textarea');
			textarea.value = url;
			document.body.appendChild(textarea);
			textarea.select();
			document.execCommand('copy');
			document.body.removeChild(textarea);
			copied = true;
			setTimeout(() => {
				copied = false;
			}, 2000);
		}
	}

	function trackShare(platform: string) {
		// Send to analytics endpoint
		// ICT 11+: Use text/plain Blob to avoid CORS preflight
		if (typeof navigator !== 'undefined' && navigator.sendBeacon) {
			const blob = new Blob(
				[
					JSON.stringify({
						event: 'social_share',
						platform,
						url,
						title,
						timestamp: Date.now()
					})
				],
				{ type: 'text/plain' }
			);
			navigator.sendBeacon('/api/analytics/track', blob);
		}
	}

	// Size classes
	let sizeClass = $derived(
		size === 'small' ? 'size-small' : size === 'large' ? 'size-large' : 'size-medium'
	);
</script>

<div class="social-share {layout} {sizeClass}">
	<span class="share-label">Share:</span>

	<div class="share-buttons">
		<!-- Twitter/X -->
		<button
			class="share-btn twitter"
			onclick={() => handleShare('twitter', shareUrls.twitter)}
			aria-label="Share on Twitter"
			title="Share on Twitter"
		>
			<svg viewBox="0 0 24 24" fill="currentColor">
				<path
					d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z"
				/>
			</svg>
			{#if showCount}<span class="count">0</span>{/if}
		</button>

		<!-- Facebook -->
		<button
			class="share-btn facebook"
			onclick={() => handleShare('facebook', shareUrls.facebook)}
			aria-label="Share on Facebook"
			title="Share on Facebook"
		>
			<svg viewBox="0 0 24 24" fill="currentColor">
				<path
					d="M24 12.073c0-6.627-5.373-12-12-12s-12 5.373-12 12c0 5.99 4.388 10.954 10.125 11.854v-8.385H7.078v-3.47h3.047V9.43c0-3.007 1.792-4.669 4.533-4.669 1.312 0 2.686.235 2.686.235v2.953H15.83c-1.491 0-1.956.925-1.956 1.874v2.25h3.328l-.532 3.47h-2.796v8.385C19.612 23.027 24 18.062 24 12.073z"
				/>
			</svg>
			{#if showCount}<span class="count">0</span>{/if}
		</button>

		<!-- LinkedIn -->
		<button
			class="share-btn linkedin"
			onclick={() => handleShare('linkedin', shareUrls.linkedin)}
			aria-label="Share on LinkedIn"
			title="Share on LinkedIn"
		>
			<svg viewBox="0 0 24 24" fill="currentColor">
				<path
					d="M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433c-1.144 0-2.063-.926-2.063-2.065 0-1.138.92-2.063 2.063-2.063 1.14 0 2.064.925 2.064 2.063 0 1.139-.925 2.065-2.064 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z"
				/>
			</svg>
			{#if showCount}<span class="count">0</span>{/if}
		</button>

		<!-- Reddit -->
		<button
			class="share-btn reddit"
			onclick={() => handleShare('reddit', shareUrls.reddit)}
			aria-label="Share on Reddit"
			title="Share on Reddit"
		>
			<svg viewBox="0 0 24 24" fill="currentColor">
				<path
					d="M12 0A12 12 0 0 0 0 12a12 12 0 0 0 12 12 12 12 0 0 0 12-12A12 12 0 0 0 12 0zm5.01 4.744c.688 0 1.25.561 1.25 1.249a1.25 1.25 0 0 1-2.498.056l-2.597-.547-.8 3.747c1.824.07 3.48.632 4.674 1.488.308-.309.73-.491 1.207-.491.968 0 1.754.786 1.754 1.754 0 .716-.435 1.333-1.01 1.614a3.111 3.111 0 0 1 .042.52c0 2.694-3.13 4.87-7.004 4.87-3.874 0-7.004-2.176-7.004-4.87 0-.183.015-.366.043-.534A1.748 1.748 0 0 1 4.028 12c0-.968.786-1.754 1.754-1.754.463 0 .898.196 1.207.49 1.207-.883 2.878-1.43 4.744-1.487l.885-4.182a.342.342 0 0 1 .14-.197.35.35 0 0 1 .238-.042l2.906.617a1.214 1.214 0 0 1 1.108-.701zM9.25 12C8.561 12 8 12.562 8 13.25c0 .687.561 1.248 1.25 1.248.687 0 1.248-.561 1.248-1.249 0-.688-.561-1.249-1.249-1.249zm5.5 0c-.687 0-1.248.561-1.248 1.25 0 .687.561 1.248 1.249 1.248.688 0 1.249-.561 1.249-1.249 0-.687-.562-1.249-1.25-1.249zm-5.466 3.99a.327.327 0 0 0-.231.094.33.33 0 0 0 0 .463c.842.842 2.484.913 2.961.913.477 0 2.105-.056 2.961-.913a.361.361 0 0 0 .029-.463.33.33 0 0 0-.464 0c-.547.533-1.684.73-2.512.73-.828 0-1.979-.196-2.512-.73a.326.326 0 0 0-.232-.095z"
				/>
			</svg>
			{#if showCount}<span class="count">0</span>{/if}
		</button>

		<!-- Email -->
		<button
			class="share-btn email"
			onclick={() => handleShare('email', shareUrls.email)}
			aria-label="Share via Email"
			title="Share via Email"
		>
			<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
				<path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z" />
				<polyline points="22,6 12,13 2,6" />
			</svg>
		</button>

		<!-- Copy Link -->
		<button
			class="share-btn copy"
			class:copied
			onclick={() => handleShare('copy', shareUrls.copy)}
			aria-label={copied ? 'Copied!' : 'Copy link'}
			title={copied ? 'Copied!' : 'Copy link'}
		>
			{#if copied}
				<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<polyline points="20 6 9 17 4 12" />
				</svg>
			{:else}
				<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<rect x="9" y="9" width="13" height="13" rx="2" ry="2" />
					<path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" />
				</svg>
			{/if}
		</button>
	</div>
</div>

<style>
	/* 2026 CSS Standards: CSS Layers, oklch colors, container queries */
	@layer components {
		.social-share {
			--share-text-muted: oklch(0.65 0.02 260);
			--share-btn-bg: oklch(0.65 0.02 260 / 0.1);
			--share-twitter: oklch(0.65 0.15 220);
			--share-facebook: oklch(0.55 0.18 250);
			--share-linkedin: oklch(0.5 0.15 240);
			--share-reddit: oklch(0.6 0.22 30);
			--share-email: oklch(0.7 0.15 240);
			--share-copy: oklch(0.65 0.18 290);
			--share-success: oklch(0.6 0.18 145);

			display: flex;
			align-items: center;
			gap: 0.75rem;
			container-type: inline-size;
		}

		.social-share.vertical {
			flex-direction: column;
			align-items: flex-start;
		}

		.share-label {
			font-size: 0.875rem;
			font-weight: 600;
			color: var(--share-text-muted);
		}

		.share-buttons {
			display: flex;
			gap: 0.5rem;
			flex-wrap: wrap;
		}

		.vertical .share-buttons {
			flex-direction: column;
		}

		.share-btn {
			display: flex;
			align-items: center;
			justify-content: center;
			gap: 0.25rem;
			border: none;
			border-radius: 8px;
			cursor: pointer;
			transition: all 0.2s ease;
			background: var(--share-btn-bg);
			color: var(--share-text-muted);
		}

		.share-btn:focus-visible {
			outline: 2px solid var(--share-email);
			outline-offset: 2px;
		}

		/* Size variants */
		.size-small .share-btn {
			width: 32px;
			height: 32px;
		}

		.size-small .share-btn svg {
			width: 14px;
			height: 14px;
		}

		.size-medium .share-btn {
			width: 40px;
			height: 40px;
		}

		.size-medium .share-btn svg {
			width: 18px;
			height: 18px;
		}

		.size-large .share-btn {
			width: 48px;
			height: 48px;
		}

		.size-large .share-btn svg {
			width: 22px;
			height: 22px;
		}

		.share-btn:hover {
			transform: translateY(-2px);
		}

		/* Platform colors using oklch */
		.share-btn.twitter:hover {
			background: var(--share-twitter);
			color: white;
		}

		.share-btn.facebook:hover {
			background: var(--share-facebook);
			color: white;
		}

		.share-btn.linkedin:hover {
			background: var(--share-linkedin);
			color: white;
		}

		.share-btn.reddit:hover {
			background: var(--share-reddit);
			color: white;
		}

		.share-btn.email:hover {
			background: var(--share-email);
			color: white;
		}

		.share-btn.copy:hover {
			background: var(--share-copy);
			color: white;
		}

		.share-btn.copied {
			background: var(--share-success);
			color: white;
		}

		.count {
			font-size: 0.75rem;
			font-weight: 600;
		}

		/* Container query responsive adjustments */
		@container (max-width: 300px) {
			.social-share {
				flex-direction: column;
				align-items: flex-start;
			}

			.share-buttons {
				justify-content: flex-start;
			}
		}
	}
</style>

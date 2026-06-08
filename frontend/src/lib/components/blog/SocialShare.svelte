<script lang="ts">
	/**
	 * Social Share Component - Svelte 5
	 * Share buttons with tracking for blog posts
	 *
	 * @version 2.0.0 - January 2026
	 * Updated: CSS layers, oklch colors, container queries, modern patterns
	 */

	import { onDestroy } from 'svelte';
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

	import Icon from '$lib/components/Icon.svelte';

	let props: Props = $props();

	// Destructure with defaults for local use
	const url = $derived(props.url ?? (typeof window !== 'undefined' ? window.location.href : ''));
	const title = $derived(props.title);
	const description = $derived(props.description ?? '');
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
	let copyResetTimer: ReturnType<typeof setTimeout> | undefined;

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
		} catch {
			// Fallback for older browsers
			const textarea = document.createElement('textarea');
			textarea.value = url;
			document.body.appendChild(textarea);
			textarea.select();
			document.execCommand('copy');
			document.body.removeChild(textarea);
			copied = true;
		}

		if (copyResetTimer) {
			clearTimeout(copyResetTimer);
		}
		copyResetTimer = setTimeout(() => {
			copied = false;
		}, 2000);
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

	onDestroy(() => {
		if (copyResetTimer) {
			clearTimeout(copyResetTimer);
		}
	});
</script>

<div class={['social-share', layout, sizeClass]}>
	<span class="share-label">Share:</span>

	<div class="share-buttons">
		<!-- Twitter/X -->
		<button
			class="share-btn twitter"
			onclick={() => handleShare('twitter', shareUrls.twitter)}
			aria-label="Share on Twitter"
			title="Share on Twitter"
		>
			<Icon name="IconBrandX" />
			{#if showCount}<span class="count">0</span>{/if}
		</button>

		<!-- Facebook -->
		<button
			class="share-btn facebook"
			onclick={() => handleShare('facebook', shareUrls.facebook)}
			aria-label="Share on Facebook"
			title="Share on Facebook"
		>
			<Icon name="IconBrandFacebook" />
			{#if showCount}<span class="count">0</span>{/if}
		</button>

		<!-- LinkedIn -->
		<button
			class="share-btn linkedin"
			onclick={() => handleShare('linkedin', shareUrls.linkedin)}
			aria-label="Share on LinkedIn"
			title="Share on LinkedIn"
		>
			<Icon name="IconBrandLinkedin" />
			{#if showCount}<span class="count">0</span>{/if}
		</button>

		<!-- Reddit -->
		<button
			class="share-btn reddit"
			onclick={() => handleShare('reddit', shareUrls.reddit)}
			aria-label="Share on Reddit"
			title="Share on Reddit"
		>
			<Icon name="IconBrandReddit" />
			{#if showCount}<span class="count">0</span>{/if}
		</button>

		<!-- Email -->
		<button
			class="share-btn email"
			onclick={() => handleShare('email', shareUrls.email)}
			aria-label="Share via Email"
			title="Share via Email"
		>
			<Icon name="IconMail" />
		</button>

		<!-- Copy Link -->
		<button
			class={['share-btn', 'copy', { copied }]}
			onclick={() => handleShare('copy', shareUrls.copy)}
			aria-label={copied ? 'Copied!' : 'Copy link'}
			title={copied ? 'Copied!' : 'Copy link'}
		>
			{#if copied}
				<Icon name="IconCheck" />
			{:else}
				<Icon name="IconCopy" />
			{/if}
		</button>
	</div>
</div>

<style>
	/* 2026 CSS Standards: CSS Layers, oklch colors, container queries */
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

	.size-medium .share-btn {
		width: 40px;
		height: 40px;
	}

	.size-large .share-btn {
		width: 48px;
		height: 48px;
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
</style>

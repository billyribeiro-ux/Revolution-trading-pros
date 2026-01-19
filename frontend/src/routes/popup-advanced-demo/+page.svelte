<script lang="ts">
	import { popupStore, type Popup } from '$lib/stores/popups.svelte';
	import { IconClock, IconVideo, IconRocket } from '$lib/icons';

	// Get date 24 hours from now for countdown demo
	const tomorrow = new Date();
	tomorrow.setDate(tomorrow.getDate() + 1);

	// Advanced popup demos with countdown and video
	const advancedDemos: Partial<Popup>[] = [
		{
			id: 'countdown-flash-sale',
			name: 'Flash Sale with Countdown',
			title: '⚡ FLASH SALE ENDING SOON!',
			isActive: true,
			content: {
				type: 'html',
				data: `
          <p style="font-size: 1.25rem; margin-bottom: 0.5rem; text-align: center;">
            <strong style="color: #ef4444;">LIMITED TIME OFFER</strong>
          </p>
          <p style="text-align: center; color: #94a3b8; margin-bottom: 1rem;">
            Get 50% OFF all trading plans before time runs out!
          </p>
        `
			},
			countdownTimer: {
				enabled: true,
				endDate: tomorrow.toISOString(),
				showDays: true,
				showHours: true,
				showMinutes: true,
				showSeconds: true,
				timerColor: '#ef4444',
				onExpire: 'show-message',
				expireMessage: 'Sale has ended! Check back for more deals.'
			},
			buttons: [
				{
					text: 'Grab This Deal',
					link: '/live-trading-rooms/day-trading',
					action: 'custom',
					style: 'primary',
					gradient: 'bg-gradient-to-r from-red-500 via-orange-500 to-yellow-500'
				}
			],
			closeButton: true,
			closeOnOverlayClick: true,
			design: {
				width: '650px',
				maxWidth: '95vw',
				height: 'auto',
				padding: '2.5rem',
				backgroundColor: '#0f172a',
				textColor: '#f1f5f9',
				borderRadius: '28px',
				borderWidth: '2px',
				borderColor: '#ef4444',
				borderStyle: 'solid',
				backdropBlur: '20px',
				boxShadow: '0 25px 50px rgba(239, 68, 68, 0.4)',
				backgroundGradient: 'linear-gradient(135deg, #1e293b 0%, #0f172a 100%)'
			},
			animation: {
				entrance: 'zoom',
				exit: 'fade',
				duration: 400,
				easing: 'ease-out'
			},
			overlayColor: '#000000',
			overlayOpacity: 0.8,
			displayRules: {
				delaySeconds: 1,
				showOnScroll: false,
				scrollPercentage: 0,
				showOnExit: false,
				frequency: 'once-per-session',
				pages: ['/*'],
				excludePages: [],
				deviceTargeting: 'all',
				newVisitorsOnly: false,
				returningVisitorsOnly: false,
				minPageViews: 0
			},
			impressions: 0,
			conversions: 0
		},
		{
			id: 'video-popup',
			name: 'Video Introduction Popup',
			title: '🎥 Watch How It Works',
			isActive: true,
			content: {
				type: 'html',
				data: `
          <p style="text-align: center; font-size: 1.125rem; margin-bottom: 1rem;">
            See our trading room in action and learn how we help traders succeed
          </p>
        `
			},
			videoEmbed: {
				enabled: true,
				url: 'https://www.youtube.com/watch?v=dQw4w9WgXcQ',
				autoplay: false,
				muted: false,
				controls: true,
				aspectRatio: '16:9'
			},
			buttons: [
				{
					text: 'Join Now',
					link: '/live-trading-rooms/day-trading',
					action: 'custom',
					style: 'primary',
					gradient: 'bg-gradient-to-r from-blue-500 to-purple-500'
				},
				{
					text: 'Learn More',
					link: '/about',
					action: 'custom',
					style: 'outline'
				}
			],
			closeButton: true,
			closeOnOverlayClick: true,
			design: {
				width: '800px',
				maxWidth: '95vw',
				height: 'auto',
				padding: '2.5rem',
				backgroundColor: '#1e293b',
				textColor: '#f1f5f9',
				borderRadius: '24px',
				borderWidth: '1px',
				borderColor: '#3b82f6',
				borderStyle: 'solid',
				backdropBlur: '16px',
				boxShadow: '0 25px 50px rgba(59, 130, 246, 0.3)'
			},
			animation: {
				entrance: 'slide-up',
				exit: 'fade',
				duration: 350,
				easing: 'ease-out'
			},
			overlayColor: '#000000',
			overlayOpacity: 0.85,
			displayRules: {
				delaySeconds: 2,
				showOnScroll: false,
				scrollPercentage: 0,
				showOnExit: false,
				frequency: 'once-per-session',
				pages: ['/*'],
				excludePages: [],
				deviceTargeting: 'all',
				newVisitorsOnly: false,
				returningVisitorsOnly: false,
				minPageViews: 0
			},
			impressions: 0,
			conversions: 0
		},
		{
			id: 'countdown-video-combo',
			name: 'Countdown + Video Combo',
			title: '🔥 Webinar Starting Soon!',
			isActive: true,
			content: {
				type: 'html',
				data: `
          <p style="font-size: 1.25rem; text-align: center; margin-bottom: 1.5rem;">
            Join our <strong style="color: #6366f1;">LIVE Trading Masterclass</strong>
          </p>
          <p style="text-align: center; color: #94a3b8;">
            Watch a preview of what you'll learn:
          </p>
        `
			},
			countdownTimer: {
				enabled: true,
				endDate: tomorrow.toISOString(),
				showDays: false,
				showHours: true,
				showMinutes: true,
				showSeconds: true,
				timerColor: '#6366f1',
				onExpire: 'hide'
			},
			videoEmbed: {
				enabled: true,
				url: 'https://www.youtube.com/watch?v=dQw4w9WgXcQ',
				autoplay: false,
				muted: true,
				controls: true,
				aspectRatio: '16:9'
			},
			buttons: [
				{
					text: 'Reserve My Spot',
					link: '/live-trading-rooms/day-trading',
					action: 'custom',
					style: 'primary',
					gradient: 'bg-gradient-to-r from-indigo-500 via-purple-500 to-pink-500'
				}
			],
			closeButton: true,
			closeOnOverlayClick: false,
			design: {
				width: '850px',
				maxWidth: '95vw',
				height: 'auto',
				padding: '3rem',
				backgroundColor: '#0f172a',
				textColor: '#f1f5f9',
				borderRadius: '32px',
				borderWidth: '2px',
				borderColor: '#8b5cf6',
				borderStyle: 'solid',
				backdropBlur: '24px',
				boxShadow: '0 30px 60px rgba(139, 92, 246, 0.4)',
				backgroundGradient: 'linear-gradient(135deg, #1e293b 0%, #0f172a 100%)'
			},
			animation: {
				entrance: 'zoom',
				exit: 'zoom',
				duration: 500,
				easing: 'ease-out'
			},
			overlayColor: '#000000',
			overlayOpacity: 0.9,
			displayRules: {
				delaySeconds: 3,
				showOnScroll: false,
				scrollPercentage: 0,
				showOnExit: false,
				frequency: 'once-per-day',
				pages: ['/*'],
				excludePages: [],
				deviceTargeting: 'desktop',
				newVisitorsOnly: false,
				returningVisitorsOnly: false,
				minPageViews: 0
			},
			impressions: 0,
			conversions: 0
		}
	];

	function showPopup(popup: Partial<Popup>) {
		popupStore.show(popup as Popup);
	}

	function clearHistory() {
		popupStore.clearHistory();
		alert('Popup history cleared!');
	}
</script>

<svelte:head>
	<title>Advanced Popup Features Demo | Revolution Trading Pros</title>
</svelte:head>

<div class="demo-page">
	<div class="demo-container">
		<!-- Header -->
		<div class="demo-header">
			<h1 class="demo-title">
				<IconRocket size={48} class="title-icon" />
				Advanced Popup Features
			</h1>
			<p class="demo-subtitle">Countdown Timers, Video Embeds, and More</p>
		</div>

		<!-- Controls -->
		<div class="demo-controls">
			<button class="control-btn danger" onclick={clearHistory}> Clear History </button>
			<a href="/popup-demo" class="control-btn secondary"> View Basic Demos </a>
			<a href="/admin/popups/create" class="control-btn primary"> Create Custom Popup </a>
		</div>

		<!-- Demo Grid -->
		<div class="demo-grid">
			{#each advancedDemos as popup}
				<div class="demo-card">
					<div class="card-header">
						<h3 class="card-title">{popup.name}</h3>
						<div class="card-badges">
							{#if popup.countdownTimer?.enabled}
								<span class="badge blue">
									<IconClock size={14} />
									Countdown
								</span>
							{/if}
							{#if popup.videoEmbed?.enabled}
								<span class="badge purple">
									<IconVideo size={14} />
									Video
								</span>
							{/if}
						</div>
					</div>

					<div class="card-body">
						<!-- Features -->
						<div class="features">
							{#if popup.countdownTimer?.enabled}
								<div class="feature-item">
									<span class="feature-label">⏰ Timer:</span>
									<span class="feature-value">
										{popup.countdownTimer.showDays ? 'Days ' : ''}
										{popup.countdownTimer.showHours ? 'Hours ' : ''}
										{popup.countdownTimer.showMinutes ? 'Mins ' : ''}
										{popup.countdownTimer.showSeconds ? 'Secs' : ''}
									</span>
								</div>
							{/if}

							{#if popup.videoEmbed?.enabled}
								<div class="feature-item">
									<span class="feature-label">🎬 Video:</span>
									<span class="feature-value">{popup.videoEmbed.aspectRatio} aspect ratio</span>
								</div>
							{/if}

							<div class="feature-item">
								<span class="feature-label">📐 Size:</span>
								<span class="feature-value">{popup.design?.width}</span>
							</div>

							<div class="feature-item">
								<span class="feature-label">✨ Animation:</span>
								<span class="feature-value">{popup.animation?.entrance}</span>
							</div>
						</div>

						<!-- Description -->
						<div class="description">
							{#if popup.id === 'countdown-flash-sale'}
								<p>
									Demonstrates countdown timer with flash sale urgency. Timer shows days, hours,
									minutes, and seconds until offer expires.
								</p>
							{:else if popup.id === 'video-popup'}
								<p>
									Shows embedded YouTube video with custom controls. Perfect for product demos or
									testimonials.
								</p>
							{:else if popup.id === 'countdown-video-combo'}
								<p>
									Combines countdown timer with video embed for maximum engagement. Perfect for
									webinar registrations.
								</p>
							{/if}
						</div>
					</div>

					<button class="test-btn" onclick={() => showPopup(popup)}> Test This Popup </button>
				</div>
			{/each}
		</div>

		<!-- Feature Documentation -->
		<div class="documentation">
			<h2 class="section-title">🎯 Advanced Features Documentation</h2>

			<div class="doc-grid">
				<div class="doc-card">
					<h3>⏰ Countdown Timers</h3>
					<ul>
						<li>
							<strong>Flexible Display:</strong> Show/hide days, hours, minutes, seconds individually
						</li>
						<li><strong>Custom Colors:</strong> Match your brand with custom timer colors</li>
						<li>
							<strong>Expiry Actions:</strong> Hide popup, show message, or redirect on expiration
						</li>
						<li><strong>Real-Time:</strong> Updates every second for accurate countdown</li>
						<li><strong>Use Cases:</strong> Flash sales, webinar registration, limited offers</li>
					</ul>
				</div>

				<div class="doc-card">
					<h3>🎥 Video Embeds</h3>
					<ul>
						<li><strong>Multi-Platform:</strong> YouTube, Vimeo, or direct video URLs</li>
						<li><strong>Autoplay Control:</strong> Auto-play with optional mute</li>
						<li><strong>Aspect Ratios:</strong> 16:9, 4:3, 1:1, or custom ratios</li>
						<li><strong>Player Controls:</strong> Show/hide video controls</li>
						<li>
							<strong>Use Cases:</strong> Product demos, testimonials, tutorials, webinar previews
						</li>
					</ul>
				</div>

				<div class="doc-card">
					<h3>🎯 Advanced Targeting</h3>
					<ul>
						<li>
							<strong>UTM Parameters:</strong> Target based on campaign source and campaign name
						</li>
						<li><strong>Referrer Tracking:</strong> Show popups based on traffic source</li>
						<li><strong>Time on Page:</strong> Trigger after user spends X seconds on page</li>
						<li><strong>Page View Limits:</strong> Min/max page view requirements</li>
						<li><strong>Geo-Targeting:</strong> Country-based targeting (backend required)</li>
					</ul>
				</div>

				<div class="doc-card">
					<h3>📊 Analytics & Tracking</h3>
					<ul>
						<li><strong>Impression Tracking:</strong> Automatic tracking of popup views</li>
						<li><strong>Conversion Tracking:</strong> Track button clicks and form submissions</li>
						<li><strong>Conversion Rate:</strong> Automatic CVR calculation</li>
						<li><strong>Timeline Data:</strong> Historical performance metrics</li>
						<li><strong>API Integration:</strong> Send data to your analytics platform</li>
					</ul>
				</div>
			</div>
		</div>

		<!-- Implementation Guide -->
		<div class="guide">
			<h2 class="section-title">💡 Implementation Guide</h2>

			<div class="guide-section">
				<h3>Creating a Countdown Popup</h3>
				<pre class="code-block"><code
						>{`{
  countdownTimer: {
    enabled: true,
    endDate: '2025-12-31T23:59:59Z',
    showDays: true,
    showHours: true,
    showMinutes: true,
    showSeconds: true,
    timerColor: '#6366f1',
    onExpire: 'hide'
  }
}`}</code
					></pre>
			</div>

			<div class="guide-section">
				<h3>Embedding a Video</h3>
				<pre class="code-block"><code
						>{`{
  videoEmbed: {
    enabled: true,
    url: 'https://www.youtube.com/watch?v=VIDEO_ID',
    autoplay: false,
    muted: false,
    controls: true,
    aspectRatio: '16:9'
  }
}`}</code
					></pre>
			</div>

			<div class="guide-section">
				<h3>Advanced Targeting</h3>
				<pre class="code-block"><code
						>{`{
  displayRules: {
    timeOnPage: 30, // Show after 30 seconds
    utmSource: ['google', 'facebook'],
    utmCampaign: ['summer-sale'],
    referrer: ['google.com'],
    minPageViews: 3,
    maxPageViews: 10
  }
}`}</code
					></pre>
			</div>
		</div>
	</div>
</div>

<style>
	.demo-page {
		background: linear-gradient(135deg, #0f172a 0%, #1e293b 100%);
		padding: 4rem 2rem;
	}

	.demo-container {
		max-width: 1400px;
		margin: 0 auto;
	}

	.demo-header {
		text-align: center;
		margin-bottom: 3rem;
	}

	.demo-title {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 1rem;
		font-size: 3rem;
		font-weight: 700;
		background: linear-gradient(to right, #3b82f6, #8b5cf6, #ec4899);
		background-clip: text;
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		margin-bottom: 1rem;
	}

	:global(.title-icon) {
		color: #3b82f6;
		-webkit-text-fill-color: initial;
	}

	.demo-subtitle {
		font-size: 1.25rem;
		color: #94a3b8;
	}

	.demo-controls {
		display: flex;
		justify-content: center;
		gap: 1rem;
		margin-bottom: 3rem;
		flex-wrap: wrap;
	}

	.control-btn {
		padding: 0.875rem 1.75rem;
		font-weight: 600;
		border-radius: 12px;
		border: none;
		cursor: pointer;
		transition: all 0.3s;
		text-decoration: none;
		display: inline-block;
	}

	.control-btn.primary {
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		color: white;
	}

	.control-btn.secondary {
		background: rgba(99, 102, 241, 0.2);
		color: #a5b4fc;
		border: 1px solid rgba(99, 102, 241, 0.3);
	}

	.control-btn.danger {
		background: rgba(239, 68, 68, 0.2);
		color: #f87171;
		border: 1px solid #f87171;
	}

	.control-btn:hover {
		transform: translateY(-2px);
		box-shadow: 0 10px 25px rgba(99, 102, 241, 0.3);
	}

	.demo-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(380px, 1fr));
		gap: 2rem;
		margin-bottom: 4rem;
	}

	.demo-card {
		background: linear-gradient(135deg, #1e293b 0%, #0f172a 100%);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 20px;
		overflow: hidden;
		transition: all 0.3s;
	}

	.demo-card:hover {
		transform: translateY(-4px);
		box-shadow: 0 20px 40px rgba(99, 102, 241, 0.2);
		border-color: rgba(99, 102, 241, 0.4);
	}

	.card-header {
		padding: 1.5rem;
		border-bottom: 1px solid rgba(99, 102, 241, 0.1);
	}

	.card-title {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
		margin-bottom: 0.75rem;
	}

	.card-badges {
		display: flex;
		gap: 0.5rem;
	}

	.badge {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.375rem 0.875rem;
		border-radius: 8px;
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: uppercase;
	}

	.badge.blue {
		background: rgba(59, 130, 246, 0.2);
		color: #60a5fa;
	}

	.badge.purple {
		background: rgba(139, 92, 246, 0.2);
		color: #a78bfa;
	}

	.card-body {
		padding: 1.5rem;
	}

	.features {
		margin-bottom: 1.5rem;
	}

	.feature-item {
		display: flex;
		justify-content: space-between;
		padding: 0.75rem;
		margin-bottom: 0.5rem;
		background: rgba(99, 102, 241, 0.05);
		border-radius: 8px;
	}

	.feature-label {
		color: #94a3b8;
		font-weight: 600;
	}

	.feature-value {
		color: #e2e8f0;
	}

	.description {
		padding: 1rem;
		background: rgba(59, 130, 246, 0.05);
		border-left: 3px solid #3b82f6;
		border-radius: 8px;
		color: #cbd5e1;
		line-height: 1.6;
	}

	.test-btn {
		width: 100%;
		padding: 1rem;
		background: linear-gradient(135deg, #3b82f6, #8b5cf6);
		color: white;
		border: none;
		font-weight: 700;
		font-size: 1rem;
		cursor: pointer;
		transition: all 0.3s;
	}

	.test-btn:hover {
		background: linear-gradient(135deg, #2563eb, #7c3aed);
		box-shadow: 0 10px 25px rgba(59, 130, 246, 0.4);
	}

	.documentation,
	.guide {
		margin-bottom: 4rem;
	}

	.section-title {
		font-size: 2rem;
		font-weight: 700;
		color: #f1f5f9;
		margin-bottom: 2rem;
		text-align: center;
	}

	.doc-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
		gap: 1.5rem;
	}

	.doc-card {
		background: rgba(99, 102, 241, 0.05);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 16px;
		padding: 2rem;
	}

	.doc-card h3 {
		font-size: 1.5rem;
		font-weight: 700;
		color: #a5b4fc;
		margin-bottom: 1rem;
	}

	.doc-card ul {
		list-style: none;
		padding: 0;
	}

	.doc-card li {
		color: #cbd5e1;
		margin-bottom: 0.75rem;
		padding-left: 1.5rem;
		position: relative;
		line-height: 1.6;
	}

	.doc-card li::before {
		content: '→';
		position: absolute;
		left: 0;
		color: #6366f1;
	}

	.doc-card strong {
		color: #e2e8f0;
	}

	.guide-section {
		margin-bottom: 2rem;
		background: rgba(99, 102, 241, 0.05);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 16px;
		padding: 2rem;
	}

	.guide-section h3 {
		font-size: 1.25rem;
		font-weight: 700;
		color: #a5b4fc;
		margin-bottom: 1rem;
	}

	.code-block {
		background: #0f172a;
		border: 1px solid rgba(99, 102, 241, 0.3);
		border-radius: 12px;
		padding: 1.5rem;
		overflow-x: auto;
	}

	.code-block code {
		color: #94a3b8;
		font-family: 'Courier New', monospace;
		font-size: 0.875rem;
		line-height: 1.6;
	}

	@media (max-width: 768px) {
		.demo-title {
			font-size: 2rem;
		}

		.demo-grid {
			grid-template-columns: 1fr;
		}

		.doc-grid {
			grid-template-columns: 1fr;
		}
	}
</style>

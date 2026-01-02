<script lang="ts">
	import { popupStore, type Popup } from '$lib/stores/popups';
	import { IconRocket } from '$lib/icons';

	// Demo popup templates
	const demoPopups: Partial<Popup>[] = [
		{
			id: 'welcome-popup',
			name: 'Welcome Popup',
			title: '🎉 Welcome to Revolution Trading!',
			isActive: true,
			content: {
				type: 'html',
				data: `
          <p style="font-size: 1.125rem; margin-bottom: 1.5rem; line-height: 1.6;">
            Get <strong style="color: #6366f1;">20% OFF</strong> your first month when you sign up today!
          </p>
          <ul style="text-align: left; margin-bottom: 1.5rem; line-height: 1.8;">
            <li>✓ Live trading sessions daily</li>
            <li>✓ Real-time alerts & analysis</li>
            <li>✓ Community support</li>
          </ul>
        `
			},
			buttons: [
				{
					text: 'Claim Offer',
					link: '/live-trading-rooms/day-trading',
					action: 'custom',
					style: 'primary',
					gradient: 'bg-gradient-to-r from-indigo-500 via-purple-500 to-pink-500'
				},
				{
					text: 'Maybe Later',
					action: 'close',
					style: 'ghost'
				}
			],
			closeButton: true,
			closeOnOverlayClick: true,
			design: {
				width: '600px',
				maxWidth: '95vw',
				height: 'auto',
				padding: '3rem',
				backgroundColor: '#1e293b',
				textColor: '#f1f5f9',
				borderRadius: '32px',
				borderWidth: '2px',
				borderColor: '#6366f1',
				borderStyle: 'solid',
				backdropBlur: '20px',
				boxShadow: '0 25px 50px -12px rgba(99, 102, 241, 0.5)',
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
				delaySeconds: 2,
				showOnScroll: false,
				scrollPercentage: 0,
				showOnExit: false,
				frequency: 'once-per-session',
				pages: ['/*'],
				excludePages: ['/checkout', '/cart'],
				deviceTargeting: 'all',
				newVisitorsOnly: false,
				returningVisitorsOnly: false,
				minPageViews: 0
			},
			impressions: 0,
			conversions: 0
		},
		{
			id: 'exit-intent-popup',
			name: 'Exit Intent Popup',
			title: "⚠️ Wait! Don't Miss Out!",
			isActive: true,
			content: {
				type: 'html',
				data: `
          <p style="font-size: 1.25rem; margin-bottom: 1rem;">
            Before you go, grab this <span style="color: #ef4444; font-weight: 700;">exclusive offer</span>!
          </p>
          <p style="margin-bottom: 1.5rem; color: #94a3b8;">
            Join thousands of successful traders who trust our alerts and strategies.
          </p>
        `
			},
			buttons: [
				{
					text: 'Show Me The Offer',
					link: '/alerts/explosive-swings',
					action: 'custom',
					style: 'primary',
					gradient: 'bg-gradient-to-r from-red-500 to-orange-500'
				},
				{
					text: 'No Thanks',
					action: 'close',
					style: 'outline'
				}
			],
			closeButton: true,
			closeOnOverlayClick: true,
			design: {
				width: '500px',
				maxWidth: '95vw',
				height: 'auto',
				padding: '2.5rem',
				backgroundColor: '#1e293b',
				textColor: '#f1f5f9',
				borderRadius: '24px',
				borderWidth: '1px',
				borderColor: '#ef4444',
				borderStyle: 'solid',
				backdropBlur: '16px',
				boxShadow: '0 20px 40px rgba(239, 68, 68, 0.3)'
			},
			animation: {
				entrance: 'bounce',
				exit: 'slide-up',
				duration: 500,
				easing: 'ease-out'
			},
			overlayColor: '#000000',
			overlayOpacity: 0.75,
			displayRules: {
				delaySeconds: 0,
				showOnScroll: false,
				scrollPercentage: 0,
				showOnExit: true,
				frequency: 'once-per-day',
				pages: ['/*'],
				excludePages: ['/checkout'],
				deviceTargeting: 'all',
				newVisitorsOnly: false,
				returningVisitorsOnly: false,
				minPageViews: 0
			},
			impressions: 0,
			conversions: 0
		},
		{
			id: 'scroll-popup',
			name: 'Scroll Triggered Newsletter',
			title: '📬 Stay Updated',
			isActive: true,
			content: {
				type: 'html',
				data: `
          <p style="margin-bottom: 1.5rem; font-size: 1.125rem;">
            Get weekly trading insights, market analysis, and exclusive tips delivered to your inbox.
          </p>
        `
			},
			formFields: [
				{
					name: 'email',
					type: 'email',
					placeholder: 'Enter your email',
					required: true
				},
				{
					name: 'name',
					type: 'text',
					placeholder: 'Your name',
					required: true
				}
			],
			formAction: '/api/newsletter-signup',
			buttons: [
				{
					text: 'Subscribe Now',
					action: 'submit',
					style: 'primary',
					gradient: 'bg-gradient-to-r from-emerald-500 to-teal-500'
				}
			],
			closeButton: true,
			closeOnOverlayClick: true,
			design: {
				width: '550px',
				maxWidth: '95vw',
				height: 'auto',
				padding: '2.5rem',
				backgroundColor: '#0f172a',
				textColor: '#f1f5f9',
				borderRadius: '28px',
				borderWidth: '1px',
				borderColor: '#10b981',
				borderStyle: 'solid',
				backdropBlur: '24px',
				boxShadow: '0 25px 50px rgba(16, 185, 129, 0.3)'
			},
			animation: {
				entrance: 'slide-up',
				exit: 'fade',
				duration: 350,
				easing: 'ease-out'
			},
			overlayColor: '#000000',
			overlayOpacity: 0.7,
			displayRules: {
				delaySeconds: 0,
				showOnScroll: true,
				scrollPercentage: 50,
				showOnExit: false,
				frequency: 'once-per-week',
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
			id: 'small-popup',
			name: 'Small Notification',
			title: undefined,
			isActive: true,
			content: {
				type: 'html',
				data: `
          <div style="text-align: center;">
            <div style="font-size: 3rem; margin-bottom: 0.5rem;">🔥</div>
            <p style="font-weight: 700; font-size: 1.125rem; margin-bottom: 0.5rem;">Flash Sale!</p>
            <p style="color: #94a3b8; font-size: 0.875rem;">Limited time: 30% off all plans</p>
          </div>
        `
			},
			buttons: [
				{
					text: 'View Deals',
					link: '/live-trading-rooms/day-trading',
					action: 'custom',
					style: 'primary'
				}
			],
			closeButton: true,
			closeOnOverlayClick: true,
			design: {
				width: '320px',
				maxWidth: '95vw',
				height: 'auto',
				padding: '2rem',
				backgroundColor: '#1e293b',
				textColor: '#f1f5f9',
				borderRadius: '20px',
				borderWidth: '1px',
				borderColor: '#f59e0b',
				borderStyle: 'solid',
				backdropBlur: '16px',
				boxShadow: '0 20px 40px rgba(245, 158, 11, 0.3)'
			},
			animation: {
				entrance: 'slide-right',
				exit: 'slide-right',
				duration: 300,
				easing: 'ease-out'
			},
			overlayColor: '#000000',
			overlayOpacity: 0.6,
			displayRules: {
				delaySeconds: 5,
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
			id: 'large-popup',
			name: 'Large Featured Popup',
			title: '🚀 Massive Trading Event',
			isActive: true,
			content: {
				type: 'html',
				data: `
          <div style="text-align: center;">
            <p style="font-size: 1.5rem; font-weight: 700; margin-bottom: 1rem; background: linear-gradient(to right, #f59e0b, #f97316); -webkit-background-clip: text; -webkit-text-fill-color: transparent;">
              3-DAY LIVE TRADING BOOTCAMP
            </p>
            <p style="font-size: 1.125rem; margin-bottom: 2rem; color: #94a3b8;">
              Learn advanced strategies from professional traders
            </p>
            <div style="display: grid; grid-template-columns: repeat(3, 1fr); gap: 1.5rem; margin-bottom: 2rem;">
              <div>
                <div style="font-size: 2.5rem; font-weight: 700; color: #6366f1;">Day 1</div>
                <div style="font-size: 0.875rem; color: #94a3b8;">Market Analysis</div>
              </div>
              <div>
                <div style="font-size: 2.5rem; font-weight: 700; color: #8b5cf6;">Day 2</div>
                <div style="font-size: 0.875rem; color: #94a3b8;">Live Trading</div>
              </div>
              <div>
                <div style="font-size: 2.5rem; font-weight: 700; color: #ec4899;">Day 3</div>
                <div style="font-size: 0.875rem; color: #94a3b8;">Q&A Session</div>
              </div>
            </div>
          </div>
        `
			},
			buttons: [
				{
					text: 'Reserve My Spot',
					link: '/live-trading-rooms/day-trading',
					action: 'custom',
					style: 'primary',
					gradient: 'bg-gradient-to-r from-orange-500 via-red-500 to-pink-500'
				},
				{
					text: 'Learn More',
					link: '/about',
					action: 'custom',
					style: 'outline'
				}
			],
			closeButton: true,
			closeOnOverlayClick: false,
			design: {
				width: '800px',
				maxWidth: '95vw',
				height: 'auto',
				padding: '3rem',
				backgroundColor: '#0f172a',
				textColor: '#f1f5f9',
				borderRadius: '32px',
				borderWidth: '2px',
				borderColor: '#f59e0b',
				borderStyle: 'solid',
				backdropBlur: '24px',
				boxShadow: '0 30px 60px rgba(245, 158, 11, 0.4)',
				backgroundGradient: 'linear-gradient(135deg, #1e293b 0%, #0f172a 100%)'
			},
			animation: {
				entrance: 'flip',
				exit: 'zoom',
				duration: 600,
				easing: 'ease-out'
			},
			overlayColor: '#000000',
			overlayOpacity: 0.85,
			displayRules: {
				delaySeconds: 3,
				showOnScroll: false,
				scrollPercentage: 0,
				showOnExit: false,
				frequency: 'once-per-day',
				pages: ['/*'],
				excludePages: [],
				deviceTargeting: 'desktop',
				newVisitorsOnly: true,
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
		alert('Popup history cleared! You can now test frequency rules again.');
	}
</script>

<svelte:head>
	<title>Popup System Demo | Revolution Trading Pros</title>
</svelte:head>

<div class="demo-page">
	<div class="demo-container">
		<!-- Header -->
		<div class="demo-header">
			<h1 class="demo-title">
				<IconRocket size={48} class="title-icon" />
				Popup System Demo
			</h1>
			<p class="demo-subtitle">Test the most advanced, fully customizable popup system</p>
		</div>

		<!-- Controls -->
		<div class="demo-controls">
			<button class="control-btn danger" onclick={clearHistory}>
				Clear History (Reset Frequency)
			</button>
			<a href="/admin/popups/create" class="control-btn primary"> Create Custom Popup </a>
		</div>

		<!-- Demo Grid -->
		<div class="demo-grid">
			{#each demoPopups as popup}
				<div class="demo-card">
					<div class="card-header">
						<h3 class="card-title">{popup.name}</h3>
						<div class="card-badges">
							{#if popup.displayRules?.showOnExit}
								<span class="badge red">Exit Intent</span>
							{/if}
							{#if popup.displayRules?.showOnScroll}
								<span class="badge blue">Scroll: {popup.displayRules.scrollPercentage}%</span>
							{/if}
							{#if popup.displayRules?.delaySeconds && popup.displayRules.delaySeconds > 0}
								<span class="badge yellow">Delay: {popup.displayRules.delaySeconds}s</span>
							{/if}
						</div>
					</div>

					<div class="card-body">
						<!-- Size Info -->
						<div class="info-row">
							<span class="info-label">Size:</span>
							<span class="info-value">{popup.design?.width}</span>
						</div>

						<!-- Animation Info -->
						<div class="info-row">
							<span class="info-label">Animation:</span>
							<span class="info-value">{popup.animation?.entrance} → {popup.animation?.exit}</span>
						</div>

						<!-- Frequency Info -->
						<div class="info-row">
							<span class="info-label">Frequency:</span>
							<span class="info-value">{popup.displayRules?.frequency}</span>
						</div>

						<!-- Features -->
						<div class="features">
							{#if popup.buttons && popup.buttons.length > 0}
								<span class="feature"
									>✓ {popup.buttons.length} Button{popup.buttons.length > 1 ? 's' : ''}</span
								>
							{/if}
							{#if popup.formFields && popup.formFields.length > 0}
								<span class="feature">✓ Form ({popup.formFields.length} fields)</span>
							{/if}
							{#if popup.design?.backgroundGradient}
								<span class="feature">✓ Gradient BG</span>
							{/if}
						</div>
					</div>

					<button class="test-btn" onclick={() => showPopup(popup)}> Test This Popup </button>
				</div>
			{/each}
		</div>

		<!-- Feature List -->
		<div class="features-section">
			<h2 class="section-title">🎨 Customization Features</h2>
			<div class="features-grid">
				<div class="feature-card">
					<h3>📐 Sizes & Layout</h3>
					<ul>
						<li>Custom width/height (px, %, vh, vw)</li>
						<li>Max width for responsiveness</li>
						<li>Custom padding & margins</li>
						<li>Responsive breakpoints</li>
					</ul>
				</div>

				<div class="feature-card">
					<h3>🎬 Animations</h3>
					<ul>
						<li>8 entrance animations</li>
						<li>8 exit animations</li>
						<li>Custom duration & easing</li>
						<li>GSAP-powered smoothness</li>
					</ul>
				</div>

				<div class="feature-card">
					<h3>⏱️ Timing & Triggers</h3>
					<ul>
						<li>Delay (0-∞ seconds)</li>
						<li>Scroll percentage trigger</li>
						<li>Exit intent detection</li>
						<li>Page view count</li>
					</ul>
				</div>

				<div class="feature-card">
					<h3>🎯 Targeting</h3>
					<ul>
						<li>Page URL patterns (wildcards)</li>
						<li>Device targeting (desktop/mobile/tablet)</li>
						<li>New vs returning visitors</li>
						<li>Exclude specific pages</li>
					</ul>
				</div>

				<div class="feature-card">
					<h3>🔄 Frequency Control</h3>
					<ul>
						<li>Always show</li>
						<li>Once per session</li>
						<li>Once per day</li>
						<li>Once per week</li>
						<li>Once ever</li>
					</ul>
				</div>

				<div class="feature-card">
					<h3>🎨 Design Options</h3>
					<ul>
						<li>Custom colors & gradients</li>
						<li>Border radius, width, style</li>
						<li>Box shadows</li>
						<li>Backdrop blur effects</li>
						<li>Custom CSS support</li>
					</ul>
				</div>

				<div class="feature-card">
					<h3>📝 Content Types</h3>
					<ul>
						<li>HTML content</li>
						<li>Markdown support</li>
						<li>Custom components</li>
						<li>Form fields (all types)</li>
					</ul>
				</div>

				<div class="feature-card">
					<h3>📊 Analytics</h3>
					<ul>
						<li>Impression tracking</li>
						<li>Conversion tracking</li>
						<li>Conversion rate calculation</li>
						<li>Timeline analytics</li>
					</ul>
				</div>
			</div>
		</div>

		<!-- Instructions -->
		<div class="instructions">
			<h2 class="section-title">📖 Testing Instructions</h2>
			<ol>
				<li><strong>Welcome Popup:</strong> Shows after 2 seconds delay (once per session)</li>
				<li>
					<strong>Exit Intent:</strong> Move your mouse cursor to the top of the browser window to trigger
				</li>
				<li>
					<strong>Scroll Popup:</strong> Scroll down 50% of the page to trigger (once per week)
				</li>
				<li><strong>Small Notification:</strong> Appears after 5 seconds (compact size demo)</li>
				<li>
					<strong>Large Featured:</strong> Shows after 3 seconds, desktop only, new visitors only (once
					per day)
				</li>
			</ol>
			<p class="note">
				<strong>Note:</strong> If popups don't show due to frequency limits, click "Clear History" to
				reset.
			</p>
		</div>
	</div>
</div>

<style>
	.demo-page {
		min-height: 100vh;
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
		background: linear-gradient(to right, #6366f1, #8b5cf6, #ec4899);
		-webkit-background-clip: text;
		background-clip: text;
		-webkit-text-fill-color: transparent;
		margin-bottom: 1rem;
	}

	:global(.title-icon) {
		color: #6366f1;
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
		grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
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
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.badge {
		padding: 0.25rem 0.75rem;
		border-radius: 6px;
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.badge.red {
		background: rgba(239, 68, 68, 0.2);
		color: #f87171;
	}

	.badge.blue {
		background: rgba(59, 130, 246, 0.2);
		color: #60a5fa;
	}

	.badge.yellow {
		background: rgba(245, 158, 11, 0.2);
		color: #fbbf24;
	}

	.card-body {
		padding: 1.5rem;
	}

	.info-row {
		display: flex;
		justify-content: space-between;
		margin-bottom: 0.75rem;
		padding-bottom: 0.75rem;
		border-bottom: 1px solid rgba(99, 102, 241, 0.1);
	}

	.info-label {
		color: #94a3b8;
		font-weight: 600;
	}

	.info-value {
		color: #e2e8f0;
	}

	.features {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
		margin-top: 1rem;
	}

	.feature {
		padding: 0.375rem 0.75rem;
		background: rgba(34, 197, 94, 0.1);
		border: 1px solid rgba(34, 197, 94, 0.2);
		border-radius: 6px;
		font-size: 0.8125rem;
		color: #4ade80;
	}

	.test-btn {
		width: 100%;
		padding: 1rem;
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		color: white;
		border: none;
		font-weight: 700;
		font-size: 1rem;
		cursor: pointer;
		transition: all 0.3s;
	}

	.test-btn:hover {
		background: linear-gradient(135deg, #4f46e5, #7c3aed);
		box-shadow: 0 10px 25px rgba(99, 102, 241, 0.4);
	}

	.features-section {
		margin-bottom: 4rem;
	}

	.section-title {
		font-size: 2rem;
		font-weight: 700;
		color: #f1f5f9;
		margin-bottom: 2rem;
		text-align: center;
	}

	.features-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: 1.5rem;
	}

	.feature-card {
		background: rgba(99, 102, 241, 0.05);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 16px;
		padding: 1.5rem;
	}

	.feature-card h3 {
		font-size: 1.25rem;
		font-weight: 700;
		color: #a5b4fc;
		margin-bottom: 1rem;
	}

	.feature-card ul {
		list-style: none;
		padding: 0;
	}

	.feature-card li {
		color: #cbd5e1;
		margin-bottom: 0.5rem;
		padding-left: 1.5rem;
		position: relative;
	}

	.feature-card li::before {
		content: '→';
		position: absolute;
		left: 0;
		color: #6366f1;
	}

	.instructions {
		background: rgba(99, 102, 241, 0.05);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 16px;
		padding: 2rem;
	}

	.instructions ol {
		color: #e2e8f0;
		line-height: 2;
		margin-bottom: 1.5rem;
	}

	.instructions strong {
		color: #a5b4fc;
	}

	.note {
		background: rgba(245, 158, 11, 0.1);
		border: 1px solid rgba(245, 158, 11, 0.3);
		border-radius: 8px;
		padding: 1rem;
		color: #fbbf24;
	}

	@media (max-width: 768px) {
		.demo-title {
			font-size: 2rem;
		}

		.demo-grid {
			grid-template-columns: 1fr;
		}

		.features-grid {
			grid-template-columns: 1fr;
		}
	}
</style>

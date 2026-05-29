<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';

	// Animation states
	let mounted = $state(false);
	let showContent = $state(false);
	let tickerOffset = $state(0);

	// Trading data for background animation
	const tickers = [
		{ symbol: 'SPX', price: 4785.32, change: +1.24 },
		{ symbol: 'ES', price: 4782.5, change: +0.89 },
		{ symbol: 'NQ', price: 16845.2, change: -0.34 },
		{ symbol: 'RTY', price: 1985.45, change: +2.15 },
		{ symbol: 'CL', price: 72.45, change: -1.2 },
		{ symbol: 'GC', price: 2056.8, change: +0.56 },
		{ symbol: 'AAPL', price: 192.3, change: +1.45 },
		{ symbol: 'TSLA', price: 248.5, change: -2.3 },
		{ symbol: 'NVDA', price: 875.2, change: +3.45 },
		{ symbol: 'AMD', price: 178.9, change: +2.8 }
	];

	// Candlestick data for chart animation
	let candles = $state(
		Array.from({ length: 40 }, (_, i) => ({
			x: i * 30 + 50,
			op: 300 + Math.sin(i * 0.5) * 80 + Math.random() * 40,
			high: 320 + Math.sin(i * 0.5) * 80 + Math.random() * 60,
			low: 280 + Math.sin(i * 0.5) * 80 - Math.random() * 40,
			close: 310 + Math.sin(i * 0.5) * 80 + Math.random() * 30,
			bullish: Math.random() > 0.4
		}))
	);

	// Email capture state
	let email = $state('');
	let isSubmitting = $state(false);
	let isSubmitted = $state(false);
	let errorMessage = $state('');
	let submissionCount = $state(0);

	// Stats animation
	let stats = $state({
		students: 0,
		countries: 0,
		years: 0
	});

	// Animated grid lines
	let gridLines = $state(
		Array.from({ length: 20 }, (_, i) => ({
			y: i * 50,
			opacity: 0.05 + Math.random() * 0.1,
			speed: 0.5 + Math.random() * 1
		}))
	);

	onMount(() => {
		mounted = true;

		// Staggered content reveal
		setTimeout(() => (showContent = true), 300);

		// Animate stats
		const animateStats = () => {
			const targetStudents = 50000;
			const targetCountries = 127;
			const targetYears = 8;

			let step = 0;
			const interval = setInterval(() => {
				step++;
				stats.students = Math.min(Math.floor(targetStudents * (step / 60)), targetStudents);
				stats.countries = Math.min(Math.floor(targetCountries * (step / 60)), targetCountries);
				stats.years = Math.min(Math.floor(targetYears * (step / 60)), targetYears);

				if (step >= 60) clearInterval(interval);
			}, 30);
		};

		setTimeout(animateStats, 800);

		// Ticker animation
		const tickerInterval = setInterval(() => {
			tickerOffset = (tickerOffset + 0.5) % 100;
		}, 50);

		return () => clearInterval(tickerInterval);
	});

	const handleNotifyMe = async () => {
		const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
		if (!email || !emailRegex.test(email)) {
			errorMessage = 'Please enter a valid email address';
			return;
		}

		isSubmitting = true;
		errorMessage = '';

		try {
			// Call API endpoint
			const response = await fetch('/api/maintenance/notify', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ email })
			});

			if (!response.ok) throw new Error('Failed to submit');

			isSubmitted = true;
			submissionCount++;
			email = '';
		} catch (err) {
			errorMessage = 'Connection issue. Please try again.';
		} finally {
			isSubmitting = false;
		}
	};

	// Format number with commas
	const formatNumber = (num: number) => num.toLocaleString();
</script>

<!-- Full-screen maintenance mode - hides all website content -->
<div class="maintenance-overlay" class:mounted>
	<!-- Animated trading chart background -->
	<svg class="chart-bg" viewBox="0 0 1400 600" preserveAspectRatio="xMidYMid slice">
		<!-- Grid lines -->
		{#each gridLines as line, i}
			<line
				x1="0"
				y1={line.y}
				x2="1400"
				y2={line.y}
				stroke="rgba(59, 130, 246, {line.opacity})"
				stroke-width="1"
				class="grid-line"
				style="animation-delay: {i * 0.1}s"
			/>
		{/each}

		<!-- Candlesticks -->
		{#each candles as candle, i}
			<g class="candle" style="animation-delay: {i * 0.05}s">
				<!-- Wick -->
				<line
					x1={candle.x}
					y1={candle.high}
					x2={candle.x}
					y2={candle.low}
					stroke={candle.bullish ? '#10b981' : '#ef4444'}
					stroke-width="1"
					opacity="0.6"
				/>
				<!-- Body -->
				<rect
					x={candle.x - 8}
					y={Math.min(candle.open, candle.close)}
					width="16"
					height={Math.abs(candle.close - candle.open) + 1}
					fill={candle.bullish ? '#10b981' : '#ef4444'}
					opacity="0.8"
					rx="2"
				/>
			</g>
		{/each}

		<!-- Moving average line -->
		<polyline
			points={candles.map((c, i) => `${c.x},${(c.high + c.low) / 2}`).join(' ')}
			fill="none"
			stroke="rgba(99, 102, 241, 0.5)"
			stroke-width="2"
			class="ma-line"
		/>
	</svg>

	<!-- Gradient overlays -->
	<div class="gradient-overlay top"></div>
	<div class="gradient-overlay bottom"></div>
	<div class="vignette"></div>

	<!-- Animated ticker tape -->
	<div class="ticker-tape">
		<div
			class="ticker-content"
			style="transform: translateX(-{tickerOffset}%)"
			role="marquee"
			aria-label="Live market data"
		>
			{#each [...tickers, ...tickers, ...tickers] as ticker, i}
				<div class="ticker-item">
					<span class="ticker-symbol">{ticker.symbol}</span>
					<span class="ticker-price">${ticker.price.toFixed(2)}</span>
					<span
						class="ticker-change"
						class:positive={ticker.change > 0}
						class:negative={ticker.change < 0}
					>
						{ticker.change > 0 ? '+' : ''}{ticker.change.toFixed(2)}%
					</span>
				</div>
			{/each}
		</div>
	</div>

	<!-- Main content -->
	<main class="content" class:show={showContent}>
		<!-- Logo mark -->
		<div class="logo-section">
			<div class="logo-mark">
				<svg viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
					<path
						d="M6 36L18 24L26 32L42 12"
						stroke="url(#logo-gradient)"
						stroke-width="3"
						stroke-linecap="round"
						stroke-linejoin="round"
						class="logo-path"
					/>
					<circle cx="42" cy="12" r="4" fill="#10b981" class="logo-dot" />
					<defs>
						<linearGradient
							id="logo-gradient"
							x1="6"
							y1="36"
							x2="42"
							y2="12"
							gradientUnits="userSpaceOnUse"
						>
							<stop offset="0%" stop-color="#3b82f6" />
							<stop offset="100%" stop-color="#8b5cf6" />
						</linearGradient>
					</defs>
				</svg>
			</div>
			<div class="brand-name">Revolution Trading Pros</div>
		</div>

		<!-- Headline -->
		<h1 class="headline">
			<span class="headline-line">Building Something</span>
			<span class="headline-line highlight">Extraordinary</span>
		</h1>

		<!-- Exciting announcement -->
		<div class="announcement">
			<p class="lead-text">
				We're not just upgrading—we're <strong>raising the bar</strong> for stocks and options
				trading education. No fake "holy grail" indicators. Just
				<strong>real institutional tools</strong> and strategies.
			</p>

			<div class="features-grid">
				<div class="feature-card">
					<div class="feature-icon">
						<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
							<path
								d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"
							/>
						</svg>
					</div>
					<h3>Institutional-Grade Scanners</h3>
					<p>Pro-level scanning technology previously reserved for hedge funds</p>
				</div>

				<div class="feature-card">
					<div class="feature-icon">
						<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
							<path d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
						</svg>
					</div>
					<h3>Day Trading University</h3>
					<p>Comprehensive curriculum for intraday mastery</p>
				</div>

				<div class="feature-card">
					<div class="feature-icon">
						<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
							<path d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6" />
						</svg>
					</div>
					<h3>Swing Trading Academy</h3>
					<p>Multi-day position strategies for consistent profits</p>
				</div>
			</div>

			<p class="mission-text">
				Our commitment: <strong>leading stocks and options trading education</strong> by providing the
				unseen institutional tools and strategies that actually work. We're building something authentic—no
				gimmicks, just real trading excellence.
			</p>
		</div>

		<!-- Stats -->
		<div class="stats-bar">
			<div class="stat">
				<span class="stat-number">{formatNumber(stats.students)}+</span>
				<span class="stat-label">Traders Trained</span>
			</div>
			<div class="stat-divider"></div>
			<div class="stat">
				<span class="stat-number">{stats.countries}</span>
				<span class="stat-label">Countries</span>
			</div>
			<div class="stat-divider"></div>
			<div class="stat">
				<span class="stat-number">{stats.years}</span>
				<span class="stat-label">Years of Excellence</span>
			</div>
		</div>

		<!-- Progress indicator -->
		<div class="progress-section">
			<div class="progress-header">
				<span class="progress-label">System Upgrade in Progress</span>
				<span class="progress-value">Phase 2 of 3</span>
			</div>
			<div class="progress-bar">
				<div class="progress-fill"></div>
				<div class="progress-glow"></div>
			</div>
			<div class="progress-steps">
				<span class="step complete">Infrastructure</span>
				<span class="step active">Platform Expansion</span>
				<span class="step">Global Launch</span>
			</div>
		</div>

		<!-- Email capture -->
		<div class="cta-section">
			{#if !isSubmitted}
				<div class="cta-header">
					<span class="pulse-dot"></span>
					<p class="cta-text">Be the first to know when we launch</p>
				</div>

				<div class="email-form">
					<div class="input-group">
						<svg class="input-icon" viewBox="0 0 20 20" fill="currentColor">
							<path d="M2.003 5.884L10 9.882l7.997-3.998A2 2 0 0016 4H4a2 2 0 00-1.997 1.884z" />
							<path d="M18 8.118l-8 4-8-4V14a2 2 0 002 2h12a2 2 0 002-2V8.118z" />
						</svg>
						<input
							type="email"
							placeholder="Enter your email address"
							bind:value={email}
							onkeydown={(e) => e.key === 'Enter' && handleNotifyMe()}
							disabled={isSubmitting}
							class="email-input"
						/>
					</div>
					<button class="notify-button" onclick={handleNotifyMe} disabled={isSubmitting}>
						{#if isSubmitting}
							<span class="button-spinner"></span>
							<span>Joining...</span>
						{:else}
							<svg class="bell-icon" viewBox="0 0 20 20" fill="currentColor">
								<path
									d="M10 2a6 6 0 00-6 6v3.586l-.707.707A1 1 0 004 14h12a1 1 0 00.707-1.707L16 11.586V8a6 6 0 00-6-6zM10 18a3 3 0 01-3-3h6a3 3 0 01-3 3z"
								/>
							</svg>
							<span>Get Early Access</span>
						{/if}
					</button>
				</div>

				{#if errorMessage}
					<p class="error-message" role="alert">
						<svg viewBox="0 0 20 20" fill="currentColor">
							<path
								fill-rule="evenodd"
								d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z"
								clip-rule="evenodd"
							/>
						</svg>
						{errorMessage}
					</p>
				{/if}
			{:else}
				<div class="success-card" role="status">
					<div class="success-animation">
						<svg class="checkmark" viewBox="0 0 52 52">
							<circle class="checkmark-circle" cx="26" cy="26" r="25" fill="none" />
							<path class="checkmark-check" fill="none" d="M14.1 27.2l7.1 7.2 16.7-16.8" />
						</svg>
					</div>
					<h3 class="success-title">You're In!</h3>
					<p class="success-message">
						<strong>Check your email!</strong> We've sent you a confirmation. You'll be the first to know
						when we launch our new platform.
					</p>
					<div class="success-perks">
						<span class="perk">🎯 Early access to scanners</span>
						<span class="perk">📚 Free course preview</span>
						<span class="perk">💎 VIP launch pricing</span>
					</div>
				</div>
			{/if}
		</div>

		<!-- Trust indicators -->
		<div class="trust-section">
			<p class="trust-text">
				<span class="shield-icon">🛡️</span>
				Your email is secure. No spam, ever. Unsubscribe anytime.
			</p>
		</div>
	</main>

	<!-- Decorative elements -->
	<div class="corner-accent top-left"></div>
	<div class="corner-accent top-right"></div>
	<div class="corner-accent bottom-left"></div>
	<div class="corner-accent bottom-right"></div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   MAINTENANCE OVERLAY - Full-screen takeover
	   This hides ALL website content (no footer, no nav)
	   ═══════════════════════════════════════════════════════════════════════════ */
	.maintenance-overlay {
		position: fixed;
		inset: 0;
		z-index: 99999;
		background: linear-gradient(135deg, #0a0a0f 0%, #0f172a 50%, #1a0f2e 100%);
		display: flex;
		align-items: center;
		justify-content: center;
		overflow: hidden;
		opacity: 0;
		animation: fadeIn 0.8s ease-out forwards;
	}

	.maintenance-overlay.mounted {
		opacity: 1;
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}

	/* Trading Chart Background */
	.chart-bg {
		position: absolute;
		inset: 0;
		width: 100%;
		height: 100%;
		opacity: 0.4;
		pointer-events: none;
	}

	.grid-line {
		animation: gridPulse 3s ease-in-out infinite;
	}

	@keyframes gridPulse {
		0%,
		100% {
			opacity: 0.05;
		}
		50% {
			opacity: 0.15;
		}
	}

	.candle {
		animation: candleFade 0.5s ease-out forwards;
		opacity: 0;
		transform-origin: center;
	}

	@keyframes candleFade {
		from {
			opacity: 0;
			transform: scaleY(0);
		}
		to {
			opacity: 1;
			transform: scaleY(1);
		}
	}

	.ma-line {
		stroke-dasharray: 2000;
		stroke-dashoffset: 2000;
		animation: drawLine 4s ease-out forwards;
	}

	@keyframes drawLine {
		to {
			stroke-dashoffset: 0;
		}
	}

	/* Gradient Overlays */
	.gradient-overlay {
		position: absolute;
		left: 0;
		right: 0;
		height: 200px;
		pointer-events: none;
	}

	.gradient-overlay.top {
		top: 0;
		background: linear-gradient(to bottom, rgba(10, 10, 15, 0.9) 0%, transparent 100%);
	}

	.gradient-overlay.bottom {
		bottom: 0;
		background: linear-gradient(to top, rgba(10, 10, 15, 0.9) 0%, transparent 100%);
	}

	.vignette {
		position: absolute;
		inset: 0;
		background: radial-gradient(ellipse at center, transparent 0%, rgba(0, 0, 0, 0.4) 100%);
		pointer-events: none;
	}

	/* Ticker Tape */
	.ticker-tape {
		position: absolute;
		top: 2rem;
		left: 0;
		right: 0;
		overflow: hidden;
		padding: 0.75rem 0;
		background: rgba(0, 0, 0, 0.3);
		border-top: 1px solid rgba(59, 130, 246, 0.2);
		border-bottom: 1px solid rgba(59, 130, 246, 0.2);
		backdrop-filter: blur(10px);
	}

	.ticker-content {
		display: flex;
		gap: 3rem;
		white-space: nowrap;
		will-change: transform;
	}

	.ticker-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		font-family: 'SF Mono', monospace;
		font-size: 0.875rem;
	}

	.ticker-symbol {
		color: var(--rtp-text-soft);
		font-weight: 600;
	}

	.ticker-price {
		color: var(--rtp-text);
	}

	.ticker-change {
		font-size: 0.75rem;
		padding: 0.25rem 0.5rem;
		border-radius: 9999px;
		font-weight: 600;
	}

	.ticker-change.positive {
		background: rgba(16, 185, 129, 0.2);
		color: #10b981;
	}

	.ticker-change.negative {
		background: rgba(239, 68, 68, 0.2);
		color: #ef4444;
	}

	/* Main Content */
	.content {
		position: relative;
		z-index: 10;
		max-width: 800px;
		width: 100%;
		padding: 2rem;
		text-align: center;
		opacity: 0;
		transform: translateY(40px) scale(0.95);
		transition: all 1s cubic-bezier(0.16, 1, 0.3, 1);
	}

	.content.show {
		opacity: 1;
		transform: translateY(0) scale(1);
	}

	/* Logo */
	.logo-section {
		margin-bottom: 2rem;
	}

	.logo-mark {
		width: 64px;
		height: 64px;
		margin: 0 auto 1rem;
		animation: logoFloat 6s ease-in-out infinite;
	}

	@keyframes logoFloat {
		0%,
		100% {
			transform: translateY(0);
		}
		50% {
			transform: translateY(-10px);
		}
	}

	.logo-path {
		stroke-dasharray: 100;
		stroke-dashoffset: 100;
		animation: drawLogo 2s ease-out forwards;
	}

	@keyframes drawLogo {
		to {
			stroke-dashoffset: 0;
		}
	}

	.logo-dot {
		animation: pulseDot 2s ease-in-out infinite;
	}

	@keyframes pulseDot {
		0%,
		100% {
			opacity: 1;
			transform: scale(1);
		}
		50% {
			opacity: 0.7;
			transform: scale(1.2);
		}
	}

	.brand-name {
		font-size: 1rem;
		color: var(--rtp-text-muted);
		letter-spacing: 0.2em;
		text-transform: uppercase;
		font-weight: 500;
	}

	/* Headline */
	.headline {
		margin-bottom: 2rem;
	}

	.headline-line {
		display: block;
		font-size: clamp(2.5rem, 6vw, 4rem);
		font-weight: 800;
		line-height: 1.1;
		letter-spacing: -0.02em;
		color: var(--rtp-text);
	}

	.headline-line.highlight {
		background: linear-gradient(135deg, #3b82f6 0%, #8b5cf6 50%, #10b981 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
		animation: shimmerText 3s ease-in-out infinite;
		background-size: 200% 200%;
	}

	@keyframes shimmerText {
		0%,
		100% {
			background-position: 0% 50%;
		}
		50% {
			background-position: 100% 50%;
		}
	}

	/* Announcement */
	.announcement {
		margin-bottom: 2.5rem;
	}

	.lead-text {
		font-size: 1.25rem;
		color: var(--rtp-text-soft);
		line-height: 1.7;
		margin-bottom: 2rem;
	}

	.lead-text strong {
		color: var(--rtp-primary);
		font-weight: 600;
	}

	/* Features Grid */
	.features-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
		gap: 1.5rem;
		margin-bottom: 2rem;
	}

	.feature-card {
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 1rem;
		padding: 1.5rem;
		transition: all 0.3s ease;
		backdrop-filter: blur(10px);
	}

	.feature-card:hover {
		transform: translateY(-5px);
		background: rgba(255, 255, 255, 0.05);
		border-color: rgba(59, 130, 246, 0.3);
		box-shadow: 0 20px 40px rgba(0, 0, 0, 0.3);
	}

	.feature-icon {
		width: 48px;
		height: 48px;
		margin: 0 auto 1rem;
		color: var(--rtp-primary);
		background: linear-gradient(135deg, rgba(59, 130, 246, 0.2), rgba(139, 92, 246, 0.2));
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.feature-icon svg {
		width: 24px;
		height: 24px;
	}

	.feature-card h3 {
		font-size: 1rem;
		font-weight: 600;
		color: var(--rtp-text);
		margin-bottom: 0.5rem;
	}

	.feature-card p {
		font-size: 0.875rem;
		color: var(--rtp-text-muted);
		line-height: 1.5;
	}

	.mission-text {
		font-size: 1.125rem;
		color: var(--rtp-text-soft);
		line-height: 1.6;
		padding: 1.5rem;
		background: rgba(16, 185, 129, 0.1);
		border: 1px solid rgba(16, 185, 129, 0.2);
		border-radius: 1rem;
		margin-top: 1rem;
	}

	.mission-text strong {
		color: #10b981;
	}

	/* Stats Bar */
	.stats-bar {
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 2rem;
		margin-bottom: 2.5rem;
		padding: 1.5rem;
		background: rgba(0, 0, 0, 0.3);
		border-radius: 1rem;
		backdrop-filter: blur(10px);
	}

	.stat {
		text-align: center;
	}

	.stat-number {
		display: block;
		font-size: 1.75rem;
		font-weight: 800;
		background: linear-gradient(135deg, #3b82f6, #8b5cf6);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}

	.stat-label {
		font-size: 0.75rem;
		color: var(--rtp-text-muted);
		text-transform: uppercase;
		letter-spacing: 0.1em;
	}

	.stat-divider {
		width: 1px;
		height: 40px;
		background: rgba(255, 255, 255, 0.1);
	}

	/* Progress Section */
	.progress-section {
		margin-bottom: 2.5rem;
		padding: 1.5rem;
		background: rgba(255, 255, 255, 0.02);
		border-radius: 1rem;
		border: 1px solid rgba(255, 255, 255, 0.05);
	}

	.progress-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
		font-size: 0.875rem;
	}

	.progress-label {
		color: var(--rtp-text-muted);
	}

	.progress-value {
		color: var(--rtp-primary);
		font-weight: 600;
	}

	.progress-bar {
		position: relative;
		height: 8px;
		background: rgba(255, 255, 255, 0.05);
		border-radius: 9999px;
		overflow: hidden;
		margin-bottom: 1rem;
	}

	.progress-fill {
		position: absolute;
		height: 100%;
		width: 66%;
		background: linear-gradient(90deg, #3b82f6 0%, #8b5cf6 50%, #10b981 100%);
		border-radius: 9999px;
		animation: progressPulse 2s ease-in-out infinite;
	}

	.progress-glow {
		position: absolute;
		height: 100%;
		width: 66%;
		background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.3), transparent);
		border-radius: 9999px;
		animation: glowSlide 2s ease-in-out infinite;
	}

	@keyframes progressPulse {
		0%,
		100% {
			filter: brightness(1);
		}
		50% {
			filter: brightness(1.2);
		}
	}

	@keyframes glowSlide {
		0% {
			transform: translateX(-100%);
		}
		100% {
			transform: translateX(150%);
		}
	}

	.progress-steps {
		display: flex;
		justify-content: space-between;
		font-size: 0.75rem;
	}

	.step {
		color: var(--rtp-text-subtle);
		position: relative;
		padding-left: 1rem;
	}

	.step::before {
		content: '';
		position: absolute;
		left: 0;
		top: 50%;
		transform: translateY(-50%);
		width: 6px;
		height: 6px;
		border-radius: 50%;
		background: var(--rtp-text-subtle);
	}

	.step.complete {
		color: #10b981;
	}

	.step.complete::before {
		background: #10b981;
	}

	.step.active {
		color: #3b82f6;
		font-weight: 600;
	}

	.step.active::before {
		background: #3b82f6;
		box-shadow: 0 0 10px #3b82f6;
		animation: stepPulse 2s ease-in-out infinite;
	}

	@keyframes stepPulse {
		0%,
		100% {
			box-shadow: 0 0 5px #3b82f6;
		}
		50% {
			box-shadow: 0 0 15px #3b82f6;
		}
	}

	/* CTA Section */
	.cta-section {
		margin-bottom: 1.5rem;
	}

	.cta-header {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		margin-bottom: 1rem;
	}

	.pulse-dot {
		width: 8px;
		height: 8px;
		background: #10b981;
		border-radius: 50%;
		animation: pulseGreen 2s ease-in-out infinite;
	}

	@keyframes pulseGreen {
		0%,
		100% {
			opacity: 1;
			transform: scale(1);
		}
		50% {
			opacity: 0.5;
			transform: scale(1.5);
		}
	}

	.cta-text {
		font-size: 1rem;
		color: var(--rtp-text-soft);
		margin: 0;
	}

	/* Email Form */
	.email-form {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		max-width: 450px;
		margin: 0 auto;
	}

	@media (min-width: 480px) {
		.email-form {
			flex-direction: row;
		}
	}

	.input-group {
		position: relative;
		flex: 1;
	}

	.input-icon {
		position: absolute;
		left: 1rem;
		top: 50%;
		transform: translateY(-50%);
		width: 20px;
		height: 20px;
		color: var(--rtp-text-subtle);
		pointer-events: none;
	}

	.email-input {
		width: 100%;
		padding: 1rem 1rem 1rem 3rem;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.75rem;
		color: var(--rtp-text);
		font-size: 1rem;
		outline: none;
		transition: all 0.3s ease;
	}

	.email-input::placeholder {
		color: var(--rtp-text-subtle);
	}

	.email-input:focus {
		border-color: var(--rtp-primary);
		background: rgba(255, 255, 255, 0.08);
		box-shadow: 0 0 0 4px rgba(59, 130, 246, 0.15);
	}

	.email-input:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.notify-button {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 1rem 1.75rem;
		background: linear-gradient(135deg, #3b82f6 0%, #8b5cf6 100%);
		color: white;
		font-weight: 600;
		font-size: 1rem;
		border: none;
		border-radius: 0.75rem;
		cursor: pointer;
		transition: all 0.3s ease;
		white-space: nowrap;
		position: relative;
		overflow: hidden;
	}

	.notify-button::before {
		content: '';
		position: absolute;
		inset: 0;
		background: linear-gradient(135deg, rgba(255, 255, 255, 0.2) 0%, transparent 50%);
		opacity: 0;
		transition: opacity 0.3s;
	}

	.notify-button:hover::before {
		opacity: 1;
	}

	.notify-button:disabled {
		opacity: 0.7;
		cursor: not-allowed;
	}

	.notify-button:not(:disabled):hover {
		transform: translateY(-2px);
		box-shadow: 0 15px 30px rgba(59, 130, 246, 0.4);
	}

	.bell-icon {
		width: 20px;
		height: 20px;
	}

	.button-spinner {
		width: 18px;
		height: 18px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.error-message {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		margin-top: 0.75rem;
		font-size: 0.875rem;
		color: #f87171;
	}

	.error-message svg {
		width: 16px;
		height: 16px;
	}

	/* Success Card */
	.success-card {
		background: rgba(16, 185, 129, 0.1);
		border: 1px solid rgba(16, 185, 129, 0.2);
		border-radius: 1rem;
		padding: 2rem;
		animation: successPop 0.5s ease-out;
	}

	@keyframes successPop {
		0% {
			transform: scale(0.9);
			opacity: 0;
		}
		50% {
			transform: scale(1.02);
		}
		100% {
			transform: scale(1);
			opacity: 1;
		}
	}

	.success-animation {
		margin-bottom: 1rem;
	}

	.checkmark {
		width: 60px;
		height: 60px;
		margin: 0 auto;
	}

	.checkmark-circle {
		stroke: #10b981;
		stroke-width: 3;
		stroke-dasharray: 166;
		stroke-dashoffset: 166;
		animation: stroke 0.6s cubic-bezier(0.65, 0, 0.45, 1) forwards;
	}

	.checkmark-check {
		stroke: #10b981;
		stroke-width: 3;
		stroke-dasharray: 48;
		stroke-dashoffset: 48;
		animation: stroke 0.3s cubic-bezier(0.65, 0, 0.45, 1) 0.6s forwards;
	}

	@keyframes stroke {
		100% {
			stroke-dashoffset: 0;
		}
	}

	.success-title {
		font-size: 1.5rem;
		font-weight: 700;
		color: #10b981;
		margin-bottom: 0.5rem;
	}

	.success-message {
		color: var(--rtp-text-soft);
		margin-bottom: 1.5rem;
		line-height: 1.6;
	}

	.success-perks {
		display: flex;
		flex-wrap: wrap;
		justify-content: center;
		gap: 0.75rem;
	}

	.perk {
		font-size: 0.875rem;
		padding: 0.5rem 1rem;
		background: rgba(16, 185, 129, 0.2);
		border-radius: 9999px;
		color: var(--rtp-text);
	}

	/* Trust Section */
	.trust-section {
		margin-top: 1.5rem;
	}

	.trust-text {
		font-size: 0.8125rem;
		color: var(--rtp-text-subtle);
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
	}

	.shield-icon {
		font-size: 1rem;
	}

	/* Corner Accents */
	.corner-accent {
		position: absolute;
		width: 100px;
		height: 100px;
		border: 2px solid rgba(59, 130, 246, 0.2);
		pointer-events: none;
	}

	.corner-accent.top-left {
		top: 2rem;
		left: 2rem;
		border-right: none;
		border-bottom: none;
		border-top-left-radius: 1rem;
	}

	.corner-accent.top-right {
		top: 2rem;
		right: 2rem;
		border-left: none;
		border-bottom: none;
		border-top-right-radius: 1rem;
	}

	.corner-accent.bottom-left {
		bottom: 2rem;
		left: 2rem;
		border-right: none;
		border-top: none;
		border-bottom-left-radius: 1rem;
	}

	.corner-accent.bottom-right {
		bottom: 2rem;
		right: 2rem;
		border-left: none;
		border-top: none;
		border-bottom-right-radius: 1rem;
	}

	/* Responsive */
	@media (max-width: 768px) {
		.stats-bar {
			flex-direction: column;
			gap: 1rem;
		}

		.stat-divider {
			display: none;
		}

		.features-grid {
			grid-template-columns: 1fr;
		}

		.progress-steps {
			flex-direction: column;
			gap: 0.5rem;
			align-items: center;
		}

		.corner-accent {
			display: none;
		}
	}
</style>

<script lang="ts">
	import { onMount, tick } from 'svelte';
	import { browser } from '$app/environment';

	// ═══════════════════════════════════════════════════════════════════════════
	// CINEMATIC TRADING MAINTENANCE PAGE
	// Apple ICT 11+ / Netflix Level Design
	// Award-Winning Animation & Interaction Design
	// ═══════════════════════════════════════════════════════════════════════════

	// Animation states with $state runes (Svelte 5)
	let mounted = $state(false);
	let scrollY = $state(0);
	let mouseX = $state(0);
	let mouseY = $state(0);
	
	// Email capture state
	let email = $state('');
	let isSubmitting = $state(false);
	let isSubmitted = $state(false);
	let errorMessage = $state('');
	let isEmailFocused = $state(false);

	// Stats with animated counting
	let stats = $state({
		students: { value: 0, target: 50000, suffix: '+' },
		countries: { value: 0, target: 127, suffix: '' },
		years: { value: 0, target: 8, suffix: '' }
	});

	// Trading data
	const tickers = [
		{ symbol: 'SPX', price: 5892.33, change: +1.24, volume: '2.4B' },
		{ symbol: 'ES', price: 5890.50, change: +0.89, volume: '1.8M' },
		{ symbol: 'NQ', price: 20845.20, change: +2.34, volume: '892K' },
		{ symbol: 'RTY', price: 2385.45, change: +1.15, volume: '445K' },
		{ symbol: 'VIX', price: 12.45, change: -8.20, volume: '125K' },
		{ symbol: 'AAPL', price: 232.30, change: +1.45, volume: '45M' },
		{ symbol: 'TSLA', price: 348.50, change: +3.30, volume: '98M' },
		{ symbol: 'NVDA', price: 1475.20, change: +5.45, volume: '67M' },
		{ symbol: 'AMD', price: 178.90, change: +2.80, volume: '32M' },
		{ symbol: 'META', price: 585.20, change: +1.90, volume: '18M' },
		{ symbol: 'AMZN', price: 198.50, change: +0.75, volume: '28M' },
		{ symbol: 'GOOGL', price: 175.30, change: +0.45, volume: '15M' }
	];

	// Dynamic candlestick data
	let candles = $state(generateCandles());
	let activeCandle = $state(0);

	function generateCandles() {
		return Array.from({ length: 60 }, (_, i) => {
			const base = 300;
			const trend = Math.sin(i * 0.15) * 60;
			const noise = (Math.random() - 0.5) * 40;
			const close = base + trend + noise;
			const open = close + (Math.random() - 0.5) * 20;
			const high = Math.max(open, close) + Math.random() * 15 + 5;
			const low = Math.min(open, close) - Math.random() * 15 - 5;
			
			return {
				x: i * 22,
				op: open,
				hi: high,
				lo: low,
				cl: close,
				bullish: close >= open,
				volume: Math.floor(Math.random() * 1000000) + 500000
			};
		});
	}

	// Animated particles
	let particles = $state(
		Array.from({ length: 50 }, (_, i) => ({
			x: Math.random() * 100,
			y: Math.random() * 100,
			size: Math.random() * 3 + 1,
			speed: Math.random() * 0.8 + 0.2,
			opacity: Math.random() * 0.4 + 0.1,
			delay: Math.random() * 5,
			color: Math.random() > 0.5 ? '#3b82f6' : '#10b981'
		}))
	);

	// Grid lines for depth
	let gridLines = $state(
		Array.from({ length: 12 }, (_, i) => ({
			y: i * 8.33,
			opacity: 0.03 + (i % 3) * 0.02,
			offset: i * 100
		}))
	);

	onMount(async () => {
		mounted = true;
		await tick();

		// Animate stats with easing
		animateStats();

		// Start candlestick animation loop
		const candleInterval = setInterval(() => {
			activeCandle = (activeCandle + 1) % candles.length;
		}, 200);

		// Track scroll
		const handleScroll = () => {
			scrollY = window.scrollY;
		};

		// Track mouse for parallax
		const handleMouseMove = (e: MouseEvent) => {
			mouseX = (e.clientX / window.innerWidth - 0.5) * 2;
			mouseY = (e.clientY / window.innerHeight - 0.5) * 2;
		};

		if (browser) {
			window.addEventListener('scroll', handleScroll, { passive: true });
			window.addEventListener('mousemove', handleMouseMove, { passive: true });
		}

		return () => {
			clearInterval(candleInterval);
			if (browser) {
				window.removeEventListener('scroll', handleScroll);
				window.removeEventListener('mousemove', handleMouseMove);
			}
		};
	});

	function animateStats() {
		const duration = 2000;
		const steps = 60;
		const interval = duration / steps;
		let step = 0;

		const timer = setInterval(() => {
			step++;
			const progress = easeOutExpo(step / steps);
			
			stats.students.value = Math.floor(stats.students.target * progress);
			stats.countries.value = Math.floor(stats.countries.target * progress);
			stats.years.value = Math.floor(stats.years.target * progress);

			if (step >= steps) clearInterval(timer);
		}, interval);
	}

	function easeOutExpo(x: number): number {
		return x === 1 ? 1 : 1 - Math.pow(2, -10 * x);
	}

	function formatNumber(num: number): string {
		return num.toLocaleString();
	}

	async function handleNotifyMe() {
		const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
		if (!email || !emailRegex.test(email)) {
			errorMessage = 'Please enter a valid email address';
			return;
		}

		isSubmitting = true;
		errorMessage = '';

		try {
			const response = await fetch('/api/maintenance/notify', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ email })
			});

			if (!response.ok) throw new Error('Failed');

			isSubmitted = true;
			email = '';
		} catch {
			errorMessage = 'Connection error. Please try again.';
		} finally {
			isSubmitting = false;
		}
	}

	// Parallax calculations based on scroll and mouse
	const bgParallax = $derived(scrollY * 0.3);
	const contentParallax = $derived(scrollY * 0.1);
	const mouseParallaxX = $derived(mouseX * 20);
	const mouseParallaxY = $derived(mouseY * 20);
</script>

<!-- ═══════════════════════════════════════════════════════════════════════════
   CINEMATIC TRADING EXPERIENCE
   Full viewport takeover with award-winning design
   ═══════════════════════════════════════════════════════════════════════════ -->
<div class="experience-container" class:mounted>
	
	<!-- ═══════════════════════════════════════════════════════════════════════
	   LAYER 1: DYNAMIC TRADING CHART BACKGROUND
	   Real-time animated candlestick visualization
	   ═══════════════════════════════════════════════════════════════════════ -->
	<div class="chart-layer" style="transform: translateY({bgParallax}px)">
		<svg class="trading-chart" viewBox="0 0 1400 800" preserveAspectRatio="xMidYMid slice">
			<defs>
				<linearGradient id="bullGradient" x1="0%" y1="0%" x2="0%" y2="100%">
					<stop offset="0%" stop-color="#10b981" stop-opacity="0.9"/>
					<stop offset="100%" stop-color="#10b981" stop-opacity="0.4"/>
				</linearGradient>
				<linearGradient id="bearGradient" x1="0%" y1="0%" x2="0%" y2="100%">
					<stop offset="0%" stop-color="#ef4444" stop-opacity="0.9"/>
					<stop offset="100%" stop-color="#ef4444" stop-opacity="0.4"/>
				</linearGradient>
				<filter id="glow">
					<feGaussianBlur stdDeviation="3" result="coloredBlur"/>
					<feMerge>
						<feMergeNode in="coloredBlur"/>
						<feMergeNode in="SourceGraphic"/>
					</feMerge>
				</filter>
			</defs>

			<!-- Grid -->
			{#each gridLines as line, i}
				<line
					x1="0"
					y1="{line.y}%"
					x2="1400"
					y2="{line.y}%"
					stroke="rgba(59, 130, 246, {line.opacity})"
					stroke-width="1"
					class="grid-line"
					style="animation-delay: {line.offset}ms"
				/>
			{/each}

			<!-- Volume bars at bottom -->
			{#each candles as candle, i}
				<rect
					x={candle.x + 4}
					y={750 - (candle.volume / 2000000) * 80}
					width="10"
					height={(candle.volume / 2000000) * 80}
					fill={candle.bullish ? 'rgba(16, 185, 129, 0.15)' : 'rgba(239, 68, 68, 0.15)'}
					opacity={0.3 + (activeCandle === i ? 0.4 : 0)}
					class="volume-bar"
				/>
			{/each}

			<!-- Candlesticks -->
			{#each candles as candle, i}
				<g class="candle-group" class:active={activeCandle === i}>
					<!-- Wick -->
					<line
						x1={candle.x + 8}
						y1={candle.lo}
						x2={candle.x + 8}
						y2={candle.hi}
						stroke={candle.bullish ? '#10b981' : '#ef4444'}
						stroke-width="2"
						opacity={activeCandle === i ? 1 : 0.6}
					/>
					<!-- Body -->
					<rect
						x={candle.x}
						y={Math.min(candle.op, candle.cl)}
						width="16"
						height={Math.max(Math.abs(candle.cl - candle.op), 4)}
						rx="2"
						fill={candle.bullish ? 'url(#bullGradient)' : 'url(#bearGradient)'}
						class="candle-body"
						filter={activeCandle === i ? 'url(#glow)' : ''}
					/>
				</g>
			{/each}

			<!-- Moving averages -->
			<polyline
				points={candles.map((c, i) => `${c.x + 8},${(c.hi + c.lo) / 2}`).join(' ')}
				fill="none"
				stroke="rgba(139, 92, 246, 0.6)"
				stroke-width="3"
				stroke-linecap="round"
				stroke-linejoin="round"
				class="ma-line"
			/>
			<polyline
				points={candles.map((c, i) => `${c.x + 8},${c.cl}`).join(' ')}
				fill="none"
				stroke="rgba(59, 130, 246, 0.4)"
				stroke-width="2"
				stroke-dasharray="5,5"
				class="price-line"
			/>
		</svg>
	</div>

	<!-- ═══════════════════════════════════════════════════════════════════════
	   LAYER 2: PARTICLE FIELD
	   Floating data points with depth
	   ═══════════════════════════════════════════════════════════════════════ -->
	<div class="particle-field" style="transform: translate({mouseParallaxX}px, {mouseParallaxY}px)">
		{#each particles as particle, i}
			<div
				class="particle"
				style="
					left: {particle.x}%;
					top: {particle.y}%;
					width: {particle.size}px;
					height: {particle.size}px;
					background: {particle.color};
					opacity: {particle.opacity};
					animation-delay: {particle.delay}s;
					animation-duration: {15 / particle.speed}s;
				"
			></div>
		{/each}
	</div>

	<!-- ═══════════════════════════════════════════════════════════════════════
	   LAYER 3: VIGNETTE & OVERLAYS
	   Cinematic depth and atmosphere
	   ═══════════════════════════════════════════════════════════════════════ -->
	<div class="vignette-overlay"></div>
	<div class="gradient-fade top"></div>
	<div class="gradient-fade bottom"></div>
	<div class="ambient-glow" style="transform: translate({-mouseParallaxX * 0.5}px, {-mouseParallaxY * 0.5}px)"></div>

	<!-- ═══════════════════════════════════════════════════════════════════════
	   LAYER 4: SCROLLING TICKER TAPE
	   Real-time market data display
	   ═══════════════════════════════════════════════════════════════════════ -->
	<div class="ticker-container">
		<div class="ticker-band">
			<div class="ticker-content">
				{#each [...tickers, ...tickers, ...tickers] as ticker, i}
					<div class="ticker-item" class:up={ticker.change > 0} class:down={ticker.change < 0}>
						<span class="ticker-symbol">{ticker.symbol}</span>
						<span class="ticker-price">${ticker.price.toFixed(2)}</span>
						<span class="ticker-change">
							{ticker.change > 0 ? '+' : ''}{ticker.change.toFixed(2)}%
						</span>
						<span class="ticker-volume">{ticker.volume}</span>
					</div>
				{/each}
			</div>
		</div>
	</div>

	<!-- ═══════════════════════════════════════════════════════════════════════
	   MAIN CONTENT: CINEMATIC REVEAL
	   Award-winning typography and interaction design
	   ═══════════════════════════════════════════════════════════════════════ -->
	<main class="content-layer" style="transform: translateY({-contentParallax}px)">
		
		<!-- Logo Section with Animation -->
		<section class="hero-section">
			<div class="logo-container">
				<div class="logo-mark">
					<svg viewBox="0 0 64 64" fill="none" xmlns="http://www.w3.org/2000/svg">
						<circle cx="32" cy="32" r="30" stroke="url(#logoGrad)" stroke-width="2" class="logo-ring"/>
						<path 
							d="M16 40L28 28L36 36L48 20" 
							stroke="url(#logoGrad)" 
							stroke-width="3" 
							stroke-linecap="round" 
							stroke-linejoin="round"
							class="logo-chart"
						/>
						<circle cx="48" cy="20" r="5" fill="#10b981" class="logo-dot"/>
						<defs>
							<linearGradient id="logoGrad" x1="16" y1="40" x2="48" y2="20" gradientUnits="userSpaceOnUse">
								<stop offset="0%" stop-color="#3b82f6"/>
								<stop offset="50%" stop-color="#8b5cf6"/>
								<stop offset="100%" stop-color="#10b981"/>
							</linearGradient>
						</defs>
					</svg>
				</div>
				<div class="brand-text">
					<span class="brand-name">Revolution Trading Pros</span>
					<span class="brand-tagline">Institutional-Grade Trading Education</span>
				</div>
			</div>

			<!-- Cinematic Headline -->
			<h1 class="main-headline">
				<span class="headline-word" style="--delay: 0s">Building</span>
				<span class="headline-word" style="--delay: 0.1s">Something</span>
				<br />
				<span class="headline-word highlight" style="--delay: 0.2s">Extraordinary</span>
			</h1>

			<!-- Lead Text -->
			<p class="lead-statement">
				We're <strong>raising the bar</strong> for stocks and options trading education. 
				No fake "holy grail" indicators. Just <strong>real institutional tools</strong> 
				and strategies that professionals actually use.
			</p>
		</section>

		<!-- Feature Cards - Netflix Style Grid -->
		<section class="features-section">
			<div class="section-header">
				<span class="section-number">01</span>
				<h2>What's Coming</h2>
			</div>
			
			<div class="features-grid">
				<div class="feature-card" style="--index: 0">
					<div class="card-icon">
						<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
							<path d="M3 3v18h18"/>
							<path d="M18 9l-5 5-2-2-4 4"/>
						</svg>
					</div>
					<h3>Real-Time Scanners</h3>
					<p>Institutional-grade scanning technology previously reserved for hedge funds and prop firms</p>
					<div class="card-highlight"></div>
				</div>

				<div class="feature-card" style="--index: 1">
					<div class="card-icon">
						<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
							<circle cx="12" cy="12" r="10"/>
							<polyline points="12 6 12 12 16 14"/>
						</svg>
					</div>
					<h3>Day Trading Mastery</h3>
					<p>Complete intraday curriculum taught by professional traders with proven track records</p>
					<div class="card-highlight"></div>
				</div>

				<div class="feature-card" style="--index: 2">
					<div class="card-icon">
						<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
							<path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z"/>
						</svg>
					</div>
					<h3>Swing Strategies</h3>
					<p>Multi-day position strategies backed by quantitative analysis and market structure</p>
					<div class="card-highlight"></div>
				</div>
			</div>
		</section>

		<!-- Stats Section - Animated Counters -->
		<section class="stats-section">
			<div class="stats-container">
				<div class="stat-item">
					<div class="stat-number">
						<span class="counter">{formatNumber(stats.students.value)}</span>
						<span class="suffix">{stats.students.suffix}</span>
					</div>
					<div class="stat-label">Traders Trained</div>
					<div class="stat-bar" style="--progress: {stats.students.value / stats.students.target}"></div>
				</div>

				<div class="stat-divider"></div>

				<div class="stat-item">
					<div class="stat-number">
						<span class="counter">{stats.countries.value}</span>
						<span class="suffix">{stats.countries.suffix}</span>
					</div>
					<div class="stat-label">Countries</div>
					<div class="stat-bar" style="--progress: {stats.countries.value / stats.countries.target}"></div>
				</div>

				<div class="stat-divider"></div>

				<div class="stat-item">
					<div class="stat-number">
						<span class="counter">{stats.years.value}</span>
						<span class="suffix">{stats.years.suffix}</span>
					</div>
					<div class="stat-label">Years of Excellence</div>
					<div class="stat-bar" style="--progress: {stats.years.value / stats.years.target}"></div>
				</div>
			</div>
		</section>

		<!-- Commitment Statement -->
		<section class="commitment-section">
			<div class="commitment-card">
				<div class="commitment-icon">🎯</div>
				<p class="commitment-text">
					Our commitment: <strong>Leading stocks and options trading education</strong> 
					by providing the unseen institutional tools and strategies that professionals use. 
					No gimmicks. No false promises. Just authentic trading excellence.
				</p>
			</div>
		</section>

		<!-- Progress Indicator -->
		<section class="progress-section">
			<div class="progress-header">
				<span class="progress-title">System Upgrade</span>
				<span class="progress-phase">Phase 2 of 3</span>
			</div>
			<div class="progress-track">
				<div class="progress-fill"></div>
				<div class="progress-shimmer"></div>
			</div>
			<div class="progress-steps">
				<div class="step completed">
					<div class="step-dot"></div>
					<span>Infrastructure</span>
				</div>
				<div class="step active">
					<div class="step-dot"></div>
					<span>Platform Expansion</span>
				</div>
				<div class="step">
					<div class="step-dot"></div>
					<span>Global Launch</span>
				</div>
			</div>
		</section>

		<!-- Email Capture -->
		<section class="capture-section">
			{#if !isSubmitted}
				<div class="capture-header">
					<div class="pulse-indicator"></div>
					<p class="capture-title">Be First to Access</p>
				</div>

				<div class="email-form" class:focused={isEmailFocused}>
					<div class="input-container">
						<svg class="input-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
							<rect x="2" y="4" width="20" height="16" rx="2"/>
							<path d="M2 4l10 8 10-8"/>
						</svg>
						<input
							type="email"
							placeholder="Enter your email"
							bind:value={email}
							on:focus={() => isEmailFocused = true}
							on:blur={() => isEmailFocused = false}
							on:keydown={(e) => e.key === 'Enter' && handleNotifyMe()}
							disabled={isSubmitting}
							class="email-input"
						/>
					</div>
					<button
						class="submit-button"
						on:click={handleNotifyMe}
						disabled={isSubmitting || !email}
					>
						{#if isSubmitting}
							<div class="button-spinner"></div>
						{:else}
							<span class="button-text">Get Early Access</span>
							<svg class="button-arrow" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
								<path d="M5 12h14M12 5l7 7-7 7"/>
							</svg>
						{/if}
					</button>
				</div>

				{#if errorMessage}
					<div class="error-banner" role="alert">
						<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
							<circle cx="12" cy="12" r="10"/>
							<path d="M12 8v4M12 16h.01"/>
						</svg>
						{errorMessage}
					</div>
				{/if}
			{:else}
				<div class="success-state">
					<div class="success-animation">
						<svg class="checkmark" viewBox="0 0 52 52">
							<circle class="checkmark-circle" cx="26" cy="26" r="25"/>
							<path class="checkmark-check" d="M14.1 27.2l7.1 7.2 16.7-16.8"/>
						</svg>
					</div>
					<h3 class="success-title">You're In!</h3>
					<p class="success-message">
						Check your email for confirmation. You're now on the exclusive list for early access.
					</p>
					<div class="perks-list">
						<span class="perk">🎯 Early scanner access</span>
						<span class="perk">📚 Free preview content</span>
						<span class="perk">💎 VIP pricing</span>
					</div>
				</div>
			{/if}
		</section>

		<!-- Trust Footer -->
		<footer class="trust-footer">
			<div class="trust-content">
				<span class="trust-icon">🛡️</span>
				<p>Your email is secure. No spam. Unsubscribe anytime.</p>
			</div>
		</footer>

	</main>

	<!-- Corner Accents -->
	<div class="corner top-left"></div>
	<div class="corner top-right"></div>
	<div class="corner bottom-left"></div>
	<div class="corner bottom-right"></div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   BASE CONTAINER - Full viewport cinematic experience
	   ═══════════════════════════════════════════════════════════════════════════ */
	.experience-container {
		position: fixed;
		inset: 0;
		width: 100%;
		min-height: 100vh;
		background: linear-gradient(180deg, #050508 0%, #0a0a0f 50%, #0f0f14 100%);
		overflow-y: auto;
		overflow-x: hidden;
		z-index: 99999;
		opacity: 0;
		animation: containerFadeIn 1.2s cubic-bezier(0.16, 1, 0.3, 1) forwards;
	}

	.experience-container.mounted {
		opacity: 1;
	}

	@keyframes containerFadeIn {
		from { opacity: 0; }
		to { opacity: 1; }
	}

	/* Scrollbar styling */
	.experience-container::-webkit-scrollbar {
		width: 6px;
	}

	.experience-container::-webkit-scrollbar-track {
		background: transparent;
	}

	.experience-container::-webkit-scrollbar-thumb {
		background: rgba(59, 130, 246, 0.3);
		border-radius: 3px;
	}

	.experience-container::-webkit-scrollbar-thumb:hover {
		background: rgba(59, 130, 246, 0.5);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CHART BACKGROUND LAYER
	   ═══════════════════════════════════════════════════════════════════════════ */
	.chart-layer {
		position: fixed;
		inset: 0;
		z-index: 1;
		pointer-events: none;
	}

	.trading-chart {
		width: 100%;
		height: 100%;
		opacity: 0.6;
	}

	.grid-line {
		animation: gridPulse 4s ease-in-out infinite;
	}

	@keyframes gridPulse {
		0%, 100% { opacity: 0.03; }
		50% { opacity: 0.08; }
	}

	.candle-group {
		transition: all 0.3s ease;
	}

	.candle-group.active {
		filter: drop-shadow(0 0 8px currentColor);
	}

	.candle-body {
		animation: candleGrow 0.5s ease-out backwards;
	}

	@keyframes candleGrow {
		from { transform: scaleY(0); opacity: 0; }
		to { transform: scaleY(1); opacity: 1; }
	}

	.volume-bar {
		transition: opacity 0.3s ease;
	}

	.ma-line {
		stroke-dasharray: 2000;
		stroke-dashoffset: 2000;
		animation: drawLine 3s ease-out forwards;
	}

	@keyframes drawLine {
		to { stroke-dashoffset: 0; }
	}

	.price-line {
		animation: dashMove 20s linear infinite;
	}

	@keyframes dashMove {
		to { stroke-dashoffset: -100; }
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   PARTICLE FIELD
	   ═══════════════════════════════════════════════════════════════════════════ */
	.particle-field {
		position: fixed;
		inset: 0;
		z-index: 2;
		pointer-events: none;
		transition: transform 0.1s ease-out;
	}

	.particle {
		position: absolute;
		border-radius: 50%;
		animation: float 20s ease-in-out infinite;
		box-shadow: 0 0 10px currentColor;
	}

	@keyframes float {
		0%, 100% {
			transform: translateY(0) translateX(0);
			opacity: 0.1;
		}
		25% {
			transform: translateY(-30px) translateX(10px);
			opacity: 0.4;
		}
		50% {
			transform: translateY(-60px) translateX(-5px);
			opacity: 0.2;
		}
		75% {
			transform: translateY(-30px) translateX(15px);
			opacity: 0.3;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   VIGNETTE & AMBIENT EFFECTS
	   ═══════════════════════════════════════════════════════════════════════════ */
	.vignette-overlay {
		position: fixed;
		inset: 0;
		z-index: 3;
		background: radial-gradient(ellipse at center, transparent 0%, rgba(0, 0, 0, 0.6) 100%);
		pointer-events: none;
	}

	.gradient-fade {
		position: fixed;
		left: 0;
		right: 0;
		height: 200px;
		z-index: 4;
		pointer-events: none;
	}

	.gradient-fade.top {
		top: 0;
		background: linear-gradient(to bottom, rgba(5, 5, 8, 0.95) 0%, transparent 100%);
	}

	.gradient-fade.bottom {
		bottom: 0;
		background: linear-gradient(to top, rgba(5, 5, 8, 0.95) 0%, transparent 100%);
	}

	.ambient-glow {
		position: fixed;
		width: 600px;
		height: 600px;
		top: 50%;
		left: 50%;
		margin-left: -300px;
		margin-top: -300px;
		background: radial-gradient(circle, rgba(59, 130, 246, 0.15) 0%, transparent 70%);
		z-index: 2;
		pointer-events: none;
		transition: transform 0.3s ease-out;
		animation: glowPulse 8s ease-in-out infinite;
	}

	@keyframes glowPulse {
		0%, 100% { opacity: 0.5; transform: scale(1); }
		50% { opacity: 0.8; transform: scale(1.1); }
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   TICKER TAPE
	   ═══════════════════════════════════════════════════════════════════════════ */
	.ticker-container {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		z-index: 10;
		background: rgba(0, 0, 0, 0.4);
		backdrop-filter: blur(20px);
		border-bottom: 1px solid rgba(59, 130, 246, 0.2);
		padding: 12px 0;
		overflow: hidden;
	}

	.ticker-band {
		width: 100%;
		overflow: hidden;
	}

	.ticker-content {
		display: flex;
		gap: 48px;
		animation: tickerScroll 40s linear infinite;
		white-space: nowrap;
	}

	@keyframes tickerScroll {
		0% { transform: translateX(0); }
		100% { transform: translateX(-33.33%); }
	}

	.ticker-item {
		display: flex;
		align-items: center;
		gap: 12px;
		font-family: 'SF Mono', 'Monaco', monospace;
		font-size: 13px;
		font-weight: 500;
		padding: 4px 12px;
		border-radius: 6px;
		background: rgba(255, 255, 255, 0.03);
		transition: all 0.3s ease;
	}

	.ticker-item:hover {
		background: rgba(255, 255, 255, 0.08);
		transform: scale(1.05);
	}

	.ticker-symbol {
		color: #94a3b8;
		font-weight: 600;
		letter-spacing: 0.5px;
	}

	.ticker-price {
		color: #e2e8f0;
		font-weight: 500;
	}

	.ticker-change {
		padding: 2px 6px;
		border-radius: 4px;
		font-size: 11px;
		font-weight: 600;
	}

	.ticker-item.up .ticker-change {
		background: rgba(16, 185, 129, 0.2);
		color: #10b981;
	}

	.ticker-item.down .ticker-change {
		background: rgba(239, 68, 68, 0.2);
		color: #ef4444;
	}

	.ticker-volume {
		color: #64748b;
		font-size: 11px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MAIN CONTENT LAYER
	   ═══════════════════════════════════════════════════════════════════════════ */
	.content-layer {
		position: relative;
		z-index: 5;
		max-width: 900px;
		margin: 0 auto;
		padding: 140px 24px 80px;
		min-height: 100vh;
		display: flex;
		flex-direction: column;
		gap: 80px;
	}

	/* Hero Section */
	.hero-section {
		text-align: center;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 32px;
	}

	.logo-container {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 16px;
		animation: logoFloat 6s ease-in-out infinite;
	}

	@keyframes logoFloat {
		0%, 100% { transform: translateY(0); }
		50% { transform: translateY(-10px); }
	}

	.logo-mark {
		width: 80px;
		height: 80px;
	}

	.logo-ring {
		stroke-dasharray: 200;
		stroke-dashoffset: 200;
		animation: drawRing 2s ease-out forwards;
	}

	@keyframes drawRing {
		to { stroke-dashoffset: 0; }
	}

	.logo-chart {
		stroke-dasharray: 100;
		stroke-dashoffset: 100;
		animation: drawChart 1.5s ease-out 0.5s forwards;
	}

	@keyframes drawChart {
		to { stroke-dashoffset: 0; }
	}

	.logo-dot {
		animation: pulseDot 2s ease-in-out infinite;
	}

	@keyframes pulseDot {
		0%, 100% { transform: scale(1); opacity: 1; }
		50% { transform: scale(1.3); opacity: 0.7; }
	}

	.brand-text {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 4px;
	}

	.brand-name {
		font-size: 14px;
		font-weight: 600;
		letter-spacing: 0.3em;
		text-transform: uppercase;
		color: #94a3b8;
		animation: fadeInUp 0.8s ease-out 0.3s backwards;
	}

	.brand-tagline {
		font-size: 12px;
		color: #64748b;
		letter-spacing: 0.1em;
		animation: fadeInUp 0.8s ease-out 0.5s backwards;
	}

	@keyframes fadeInUp {
		from { opacity: 0; transform: translateY(20px); }
		to { opacity: 1; transform: translateY(0); }
	}

	/* Cinematic Headline */
	.main-headline {
		margin: 0;
		line-height: 1.1;
	}

	.headline-word {
		display: inline-block;
		font-size: clamp(2.5rem, 8vw, 5rem);
		font-weight: 800;
		color: #f8fafc;
		letter-spacing: -0.03em;
		opacity: 0;
		transform: translateY(40px);
		animation: wordReveal 1s cubic-bezier(0.16, 1, 0.3, 1) forwards;
		animation-delay: var(--delay);
	}

	@keyframes wordReveal {
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	.headline-word.highlight {
		background: linear-gradient(135deg, #3b82f6 0%, #8b5cf6 50%, #10b981 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
		background-size: 200% 200%;
		animation: 
			wordReveal 1s cubic-bezier(0.16, 1, 0.3, 1) forwards,
			gradientShift 4s ease-in-out infinite;
		animation-delay: var(--delay), 1.5s;
	}

	@keyframes gradientShift {
		0%, 100% { background-position: 0% 50%; }
		50% { background-position: 100% 50%; }
	}

	/* Lead Statement */
	.lead-statement {
		font-size: clamp(1.125rem, 2.5vw, 1.5rem);
		line-height: 1.8;
		color: #94a3b8;
		max-width: 700px;
		margin: 0;
		opacity: 0;
		animation: fadeInUp 1s ease-out 0.8s forwards;
	}

	.lead-statement strong {
		color: #e2e8f0;
		font-weight: 600;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   FEATURES SECTION - Netflix Style Cards
	   ═══════════════════════════════════════════════════════════════════════════ */
	.features-section {
		display: flex;
		flex-direction: column;
		gap: 24px;
	}

	.section-header {
		display: flex;
		align-items: center;
		gap: 16px;
	}

	.section-number {
		font-size: 12px;
		font-weight: 700;
		color: #3b82f6;
		letter-spacing: 0.2em;
		padding: 6px 12px;
		border: 1px solid rgba(59, 130, 246, 0.3);
		border-radius: 4px;
	}

	.features-section h2 {
		font-size: 14px;
		font-weight: 500;
		color: #64748b;
		letter-spacing: 0.15em;
		text-transform: uppercase;
		margin: 0;
	}

	.features-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
		gap: 20px;
	}

	.feature-card {
		position: relative;
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.08);
		border-radius: 16px;
		padding: 32px;
		overflow: hidden;
		transition: all 0.4s cubic-bezier(0.16, 1, 0.3, 1);
		cursor: pointer;
		opacity: 0;
		transform: translateY(30px);
		animation: cardReveal 0.8s cubic-bezier(0.16, 1, 0.3, 1) forwards;
		animation-delay: calc(var(--index) * 0.15s + 1s);
	}

	@keyframes cardReveal {
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	.feature-card:hover {
		transform: translateY(-8px) scale(1.02);
		border-color: rgba(59, 130, 246, 0.4);
		background: rgba(255, 255, 255, 0.06);
		box-shadow: 
			0 20px 40px rgba(0, 0, 0, 0.4),
			0 0 60px rgba(59, 130, 246, 0.15);
	}

	.card-highlight {
		position: absolute;
		inset: 0;
		background: linear-gradient(135deg, rgba(59, 130, 246, 0.1) 0%, transparent 50%);
		opacity: 0;
		transition: opacity 0.4s ease;
	}

	.feature-card:hover .card-highlight {
		opacity: 1;
	}

	.card-icon {
		width: 56px;
		height: 56px;
		background: linear-gradient(135deg, rgba(59, 130, 246, 0.2), rgba(139, 92, 246, 0.2));
		border-radius: 14px;
		display: flex;
		align-items: center;
		justify-content: center;
		margin-bottom: 20px;
		color: #60a5fa;
		transition: all 0.4s ease;
	}

	.feature-card:hover .card-icon {
		transform: scale(1.1);
		background: linear-gradient(135deg, rgba(59, 130, 246, 0.3), rgba(139, 92, 246, 0.3));
		box-shadow: 0 10px 30px rgba(59, 130, 246, 0.3);
	}

	.card-icon svg {
		width: 28px;
		height: 28px;
	}

	.feature-card h3 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 12px 0;
	}

	.feature-card p {
		font-size: 0.9375rem;
		color: #94a3b8;
		line-height: 1.6;
		margin: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   STATS SECTION - Animated Counters
	   ═══════════════════════════════════════════════════════════════════════════ */
	.stats-section {
		padding: 40px 0;
	}

	.stats-container {
		display: flex;
		justify-content: center;
		align-items: stretch;
		gap: 48px;
		padding: 40px;
		background: rgba(0, 0, 0, 0.3);
		border-radius: 24px;
		border: 1px solid rgba(255, 255, 255, 0.05);
		backdrop-filter: blur(20px);
	}

	.stat-item {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 8px;
		min-width: 140px;
	}

	.stat-number {
		display: flex;
		align-items: baseline;
		gap: 4px;
		font-size: 2.5rem;
		font-weight: 800;
		background: linear-gradient(135deg, #3b82f6, #8b5cf6, #10b981);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
		background-size: 200% 200%;
		animation: gradientShift 5s ease-in-out infinite;
	}

	.suffix {
		font-size: 1.5rem;
		font-weight: 600;
	}

	.stat-label {
		font-size: 0.75rem;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.15em;
		font-weight: 500;
	}

	.stat-bar {
		width: 100%;
		height: 3px;
		background: rgba(255, 255, 255, 0.1);
		border-radius: 2px;
		margin-top: 8px;
		position: relative;
		overflow: hidden;
	}

	.stat-bar::after {
		content: '';
		position: absolute;
		inset: 0;
		width: calc(var(--progress) * 100%);
		background: linear-gradient(90deg, #3b82f6, #10b981);
		border-radius: 2px;
		transition: width 0.3s ease;
	}

	.stat-divider {
		width: 1px;
		background: rgba(255, 255, 255, 0.1);
		align-self: stretch;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   COMMITMENT SECTION
	   ═══════════════════════════════════════════════════════════════════════════ */
	.commitment-section {
		display: flex;
		justify-content: center;
	}

	.commitment-card {
		max-width: 700px;
		padding: 40px;
		background: linear-gradient(135deg, rgba(16, 185, 129, 0.1) 0%, rgba(59, 130, 246, 0.05) 100%);
		border: 1px solid rgba(16, 185, 129, 0.2);
		border-radius: 20px;
		text-align: center;
		position: relative;
		overflow: hidden;
	}

	.commitment-card::before {
		content: '';
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		height: 2px;
		background: linear-gradient(90deg, transparent, #10b981, transparent);
	}

	.commitment-icon {
		font-size: 40px;
		margin-bottom: 20px;
	}

	.commitment-text {
		font-size: 1.125rem;
		line-height: 1.8;
		color: #cbd5e1;
		margin: 0;
	}

	.commitment-text strong {
		color: #f1f5f9;
		font-weight: 600;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   PROGRESS SECTION
	   ═══════════════════════════════════════════════════════════════════════════ */
	.progress-section {
		padding: 32px;
		background: rgba(255, 255, 255, 0.02);
		border-radius: 16px;
		border: 1px solid rgba(255, 255, 255, 0.05);
	}

	.progress-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 20px;
	}

	.progress-title {
		font-size: 0.875rem;
		color: #94a3b8;
		font-weight: 500;
	}

	.progress-phase {
		font-size: 0.875rem;
		color: #3b82f6;
		font-weight: 600;
		padding: 6px 12px;
		background: rgba(59, 130, 246, 0.1);
		border-radius: 20px;
	}

	.progress-track {
		position: relative;
		height: 6px;
		background: rgba(255, 255, 255, 0.05);
		border-radius: 3px;
		overflow: hidden;
		margin-bottom: 24px;
	}

	.progress-fill {
		position: absolute;
		height: 100%;
		width: 66%;
		background: linear-gradient(90deg, #3b82f6 0%, #8b5cf6 50%, #10b981 100%);
		border-radius: 3px;
		animation: progressGlow 2s ease-in-out infinite;
	}

	@keyframes progressGlow {
		0%, 100% { filter: brightness(1); box-shadow: 0 0 10px rgba(59, 130, 246, 0.5); }
		50% { filter: brightness(1.3); box-shadow: 0 0 20px rgba(59, 130, 246, 0.8); }
	}

	.progress-shimmer {
		position: absolute;
		height: 100%;
		width: 66%;
		background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.4), transparent);
		animation: shimmer 2s ease-in-out infinite;
	}

	@keyframes shimmer {
		0% { transform: translateX(-100%); }
		100% { transform: translateX(150%); }
	}

	.progress-steps {
		display: flex;
		justify-content: space-between;
	}

	.step {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 8px;
		font-size: 0.8125rem;
		color: #64748b;
		position: relative;
	}

	.step.completed {
		color: #10b981;
	}

	.step.active {
		color: #3b82f6;
		font-weight: 500;
	}

	.step-dot {
		width: 10px;
		height: 10px;
		border-radius: 50%;
		background: currentColor;
		box-shadow: 0 0 10px currentColor;
	}

	.step.active .step-dot {
		animation: stepPulse 2s ease-in-out infinite;
	}

	@keyframes stepPulse {
		0%, 100% { box-shadow: 0 0 5px currentColor; }
		50% { box-shadow: 0 0 15px currentColor, 0 0 30px currentColor; }
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   EMAIL CAPTURE SECTION
	   ═══════════════════════════════════════════════════════════════════════════ */
	.capture-section {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 20px;
		padding: 48px 32px;
		background: rgba(0, 0, 0, 0.3);
		border-radius: 24px;
		border: 1px solid rgba(255, 255, 255, 0.08);
		backdrop-filter: blur(20px);
	}

	.capture-header {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.pulse-indicator {
		width: 10px;
		height: 10px;
		background: #10b981;
		border-radius: 50%;
		animation: pulseGreen 2s ease-in-out infinite;
	}

	@keyframes pulseGreen {
		0%, 100% { opacity: 1; transform: scale(1); }
		50% { opacity: 0.4; transform: scale(1.5); }
	}

	.capture-title {
		font-size: 1.125rem;
		color: #e2e8f0;
		font-weight: 500;
		margin: 0;
	}

	.email-form {
		display: flex;
		gap: 12px;
		width: 100%;
		max-width: 480px;
		transition: all 0.3s ease;
	}

	@media (max-width: 640px) {
		.email-form {
			flex-direction: column;
		}
	}

	.input-container {
		position: relative;
		flex: 1;
	}

	.input-icon {
		position: absolute;
		left: 16px;
		top: 50%;
		transform: translateY(-50%);
		width: 20px;
		height: 20px;
		color: #64748b;
		transition: color 0.3s ease;
	}

	.email-form.focused .input-icon {
		color: #3b82f6;
	}

	.email-input {
		width: 100%;
		padding: 16px 16px 16px 48px;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 12px;
		color: #f1f5f9;
		font-size: 1rem;
		outline: none;
		transition: all 0.3s ease;
	}

	.email-input::placeholder {
		color: #64748b;
	}

	.email-input:focus {
		border-color: #3b82f6;
		background: rgba(255, 255, 255, 0.08);
		box-shadow: 0 0 0 4px rgba(59, 130, 246, 0.15);
	}

	.email-input:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.submit-button {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		padding: 16px 28px;
		background: linear-gradient(135deg, #3b82f6 0%, #8b5cf6 100%);
		border: none;
		border-radius: 12px;
		color: white;
		font-size: 1rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.3s cubic-bezier(0.16, 1, 0.3, 1);
		white-space: nowrap;
		position: relative;
		overflow: hidden;
	}

	.submit-button::before {
		content: '';
		position: absolute;
		inset: 0;
		background: linear-gradient(135deg, rgba(255,255,255,0.2) 0%, transparent 50%);
		opacity: 0;
		transition: opacity 0.3s ease;
	}

	.submit-button:hover::before {
		opacity: 1;
	}

	.submit-button:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 15px 40px rgba(59, 130, 246, 0.4);
	}

	.submit-button:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.button-spinner {
		width: 20px;
		height: 20px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	.button-arrow {
		width: 18px;
		height: 18px;
		transition: transform 0.3s ease;
	}

	.submit-button:hover .button-arrow {
		transform: translateX(4px);
	}

	.error-banner {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 12px 20px;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 10px;
		color: #f87171;
		font-size: 0.875rem;
		animation: shake 0.5s ease-in-out;
	}

	.error-banner svg {
		width: 18px;
		height: 18px;
		flex-shrink: 0;
	}

	@keyframes shake {
		0%, 100% { transform: translateX(0); }
		25% { transform: translateX(-5px); }
		75% { transform: translateX(5px); }
	}

	/* Success State */
	.success-state {
		text-align: center;
		animation: successPop 0.6s cubic-bezier(0.16, 1, 0.3, 1);
	}

	@keyframes successPop {
		0% { opacity: 0; transform: scale(0.9) translateY(20px); }
		50% { transform: scale(1.02); }
		100% { opacity: 1; transform: scale(1) translateY(0); }
	}

	.success-animation {
		margin-bottom: 24px;
	}

	.checkmark {
		width: 80px;
		height: 80px;
		margin: 0 auto;
	}

	.checkmark-circle {
		stroke: #10b981;
		stroke-width: 3;
		stroke-dasharray: 166;
		stroke-dashoffset: 166;
		fill: none;
		animation: stroke 0.6s cubic-bezier(0.65, 0, 0.45, 1) forwards;
	}

	.checkmark-check {
		stroke: #10b981;
		stroke-width: 3;
		stroke-dasharray: 48;
		stroke-dashoffset: 48;
		fill: none;
		animation: stroke 0.3s cubic-bezier(0.65, 0, 0.45, 1) 0.6s forwards;
	}

	@keyframes stroke {
		100% { stroke-dashoffset: 0; }
	}

	.success-title {
		font-size: 1.75rem;
		font-weight: 700;
		color: #10b981;
		margin: 0 0 12px 0;
	}

	.success-message {
		font-size: 1rem;
		color: #94a3b8;
		margin: 0 0 24px 0;
		line-height: 1.6;
	}

	.perks-list {
		display: flex;
		flex-wrap: wrap;
		justify-content: center;
		gap: 12px;
	}

	.perk {
		font-size: 0.875rem;
		padding: 10px 18px;
		background: rgba(16, 185, 129, 0.15);
		border: 1px solid rgba(16, 185, 129, 0.3);
		border-radius: 20px;
		color: #e2e8f0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   TRUST FOOTER
	   ═══════════════════════════════════════════════════════════════════════════ */
	.trust-footer {
		text-align: center;
		padding-top: 20px;
		border-top: 1px solid rgba(255, 255, 255, 0.05);
	}

	.trust-content {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		font-size: 0.8125rem;
		color: #64748b;
	}

	.trust-icon {
		font-size: 16px;
	}

	.trust-content p {
		margin: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CORNER ACCENTS
	   ═══════════════════════════════════════════════════════════════════════════ */
	.corner {
		position: fixed;
		width: 120px;
		height: 120px;
		border: 2px solid rgba(59, 130, 246, 0.15);
		pointer-events: none;
		z-index: 6;
	}

	.corner.top-left {
		top: 100px;
		left: 40px;
		border-right: none;
		border-bottom: none;
		border-top-left-radius: 16px;
	}

	.corner.top-right {
		top: 100px;
		right: 40px;
		border-left: none;
		border-bottom: none;
		border-top-right-radius: 16px;
	}

	.corner.bottom-left {
		bottom: 40px;
		left: 40px;
		border-right: none;
		border-top: none;
		border-bottom-left-radius: 16px;
	}

	.corner.bottom-right {
		bottom: 40px;
		right: 40px;
		border-left: none;
		border-top: none;
		border-bottom-right-radius: 16px;
	}

	/* Responsive Adjustments */
	@media (max-width: 768px) {
		.content-layer {
			padding: 120px 20px 60px;
			gap: 60px;
		}

		.stats-container {
			flex-direction: column;
			gap: 32px;
			padding: 32px 24px;
		}

		.stat-divider {
			display: none;
		}

		.corner {
			display: none;
		}

		.features-grid {
			grid-template-columns: 1fr;
		}

		.progress-steps {
			flex-direction: column;
			gap: 16px;
			align-items: center;
		}

		.ticker-item {
			font-size: 11px;
		}
	}

	@media (max-width: 480px) {
		.headline-word {
			font-size: 2rem;
		}

		.commitment-card {
			padding: 24px;
		}

		.capture-section {
			padding: 32px 20px;
		}
	}
</style>

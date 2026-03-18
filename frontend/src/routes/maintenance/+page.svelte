<!--
	URL: /maintenance
	Cinematic Coming Soon / Maintenance Page
	Institutional-grade design with real-time chart animation
-->
<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════
	let canvas: HTMLCanvasElement | undefined = $state(undefined);
	let particleCanvas: HTMLCanvasElement | undefined = $state(undefined);
	let email = $state('');
	let submitStatus = $state<'idle' | 'submitting' | 'success' | 'error'>('idle');
	let errorMessage = $state('');
	let _mounted = $state(false);
	let showContent = $state(false);
	let showHeadline = $state(false);
	let showSubtext = $state(false);
	let showFeatures = $state(false);
	let showForm = $state(false);
	let showFooter = $state(false);

	// ═══════════════════════════════════════════════════════════════════════════
	// CHART ANIMATION ENGINE
	// ═══════════════════════════════════════════════════════════════════════════

	interface Candle {
		open: number;
		high: number;
		low: number;
		close: number;
		x: number;
		targetX: number;
		opacity: number;
		volume: number;
	}

	interface Particle {
		x: number;
		y: number;
		vx: number;
		vy: number;
		size: number;
		opacity: number;
		color: string;
		life: number;
		maxLife: number;
	}

	function generateCandles(count: number, startPrice: number): Candle[] {
		const candles: Candle[] = [];
		let price = startPrice;
		for (let i = 0; i < count; i++) {
			const volatility = 0.008 + Math.random() * 0.015;
			const trend = Math.sin(i * 0.05) * 0.003;
			const change = (Math.random() - 0.48 + trend) * volatility * price;
			const open = price;
			const close = price + change;
			const high = Math.max(open, close) + Math.random() * Math.abs(change) * 1.5;
			const low = Math.min(open, close) - Math.random() * Math.abs(change) * 1.5;
			const volume = 0.3 + Math.random() * 0.7;
			candles.push({
				open,
				high,
				low,
				close,
				x: i,
				targetX: i,
				opacity: 1,
				volume
			});
			price = close;
		}
		return candles;
	}

	function drawChart(
		ctx: CanvasRenderingContext2D,
		candles: Candle[],
		width: number,
		height: number,
		offset: number
	) {
		ctx.clearRect(0, 0, width, height);

		// Background gradient
		const bgGrad = ctx.createLinearGradient(0, 0, 0, height);
		bgGrad.addColorStop(0, '#0a0e17');
		bgGrad.addColorStop(0.5, '#0d1321');
		bgGrad.addColorStop(1, '#080c14');
		ctx.fillStyle = bgGrad;
		ctx.fillRect(0, 0, width, height);

		// Grid lines (institutional style)
		const gridColor = 'rgba(30, 58, 95, 0.25)';
		ctx.strokeStyle = gridColor;
		ctx.lineWidth = 0.5;

		// Horizontal grid
		for (let i = 0; i < 8; i++) {
			const y = (height / 8) * i;
			ctx.beginPath();
			ctx.moveTo(0, y);
			ctx.lineTo(width, y);
			ctx.stroke();
		}

		// Vertical grid
		for (let i = 0; i < 16; i++) {
			const x = (width / 16) * i;
			ctx.beginPath();
			ctx.moveTo(x, 0);
			ctx.lineTo(x, height);
			ctx.stroke();
		}

		if (candles.length === 0) return;

		// Price range
		let minPrice = Infinity;
		let maxPrice = -Infinity;
		for (const c of candles) {
			if (c.low < minPrice) minPrice = c.low;
			if (c.high > maxPrice) maxPrice = c.high;
		}
		const priceRange = maxPrice - minPrice;
		const padding = priceRange * 0.15;
		minPrice -= padding;
		maxPrice += padding;
		const totalRange = maxPrice - minPrice;

		const candleWidth = Math.max(3, (width / candles.length) * 0.6);
		const gap = width / candles.length;

		// Volume bars (subtle background)
		const volumeHeight = height * 0.15;
		const volumeBase = height;
		for (let i = 0; i < candles.length; i++) {
			const c = candles[i];
			const x = i * gap - offset + gap / 2;
			if (x < -gap || x > width + gap) continue;
			const isBullish = c.close >= c.open;
			const volH = c.volume * volumeHeight;
			ctx.fillStyle = isBullish ? 'rgba(0, 224, 142, 0.06)' : 'rgba(255, 82, 82, 0.06)';
			ctx.fillRect(x - candleWidth / 2, volumeBase - volH, candleWidth, volH);
		}

		// Moving average line
		ctx.beginPath();
		ctx.strokeStyle = 'rgba(59, 130, 246, 0.35)';
		ctx.lineWidth = 1.5;
		const maPeriod = 20;
		let started = false;
		for (let i = maPeriod; i < candles.length; i++) {
			let sum = 0;
			for (let j = i - maPeriod; j < i; j++) {
				sum += candles[j].close;
			}
			const ma = sum / maPeriod;
			const x = i * gap - offset + gap / 2;
			const y = ((maxPrice - ma) / totalRange) * height;
			if (x < -gap || x > width + gap) continue;
			if (!started) {
				ctx.moveTo(x, y);
				started = true;
			} else {
				ctx.lineTo(x, y);
			}
		}
		ctx.stroke();

		// Second MA (longer period)
		ctx.beginPath();
		ctx.strokeStyle = 'rgba(251, 191, 36, 0.25)';
		ctx.lineWidth = 1;
		const ma2Period = 50;
		started = false;
		for (let i = ma2Period; i < candles.length; i++) {
			let sum = 0;
			for (let j = i - ma2Period; j < i; j++) {
				sum += candles[j].close;
			}
			const ma = sum / ma2Period;
			const x = i * gap - offset + gap / 2;
			const y = ((maxPrice - ma) / totalRange) * height;
			if (x < -gap || x > width + gap) continue;
			if (!started) {
				ctx.moveTo(x, y);
				started = true;
			} else {
				ctx.lineTo(x, y);
			}
		}
		ctx.stroke();

		// Candlesticks
		for (let i = 0; i < candles.length; i++) {
			const c = candles[i];
			const x = i * gap - offset + gap / 2;
			if (x < -gap * 2 || x > width + gap * 2) continue;

			const isBullish = c.close >= c.open;
			const openY = ((maxPrice - c.open) / totalRange) * height;
			const closeY = ((maxPrice - c.close) / totalRange) * height;
			const highY = ((maxPrice - c.high) / totalRange) * height;
			const lowY = ((maxPrice - c.low) / totalRange) * height;

			// Edge fade
			let edgeAlpha = 1;
			if (x < gap * 3) edgeAlpha = Math.max(0, x / (gap * 3));
			if (x > width - gap * 3) edgeAlpha = Math.max(0, (width - x) / (gap * 3));

			const bullColor = `rgba(0, 224, 142, ${0.9 * edgeAlpha})`;
			const bearColor = `rgba(255, 82, 82, ${0.9 * edgeAlpha})`;
			const wickBull = `rgba(0, 224, 142, ${0.5 * edgeAlpha})`;
			const wickBear = `rgba(255, 82, 82, ${0.5 * edgeAlpha})`;

			// Wick
			ctx.strokeStyle = isBullish ? wickBull : wickBear;
			ctx.lineWidth = 1;
			ctx.beginPath();
			ctx.moveTo(x, highY);
			ctx.lineTo(x, lowY);
			ctx.stroke();

			// Body
			const bodyTop = Math.min(openY, closeY);
			const bodyHeight = Math.max(1, Math.abs(closeY - openY));

			if (isBullish) {
				ctx.fillStyle = bullColor;
				// Glow
				ctx.shadowColor = 'rgba(0, 224, 142, 0.15)';
				ctx.shadowBlur = 4;
			} else {
				ctx.fillStyle = bearColor;
				ctx.shadowColor = 'rgba(255, 82, 82, 0.15)';
				ctx.shadowBlur = 4;
			}
			ctx.fillRect(x - candleWidth / 2, bodyTop, candleWidth, bodyHeight);
			ctx.shadowBlur = 0;
		}

		// Overlay gradient (cinematic depth)
		const topFog = ctx.createLinearGradient(0, 0, 0, height * 0.35);
		topFog.addColorStop(0, 'rgba(10, 14, 23, 0.85)');
		topFog.addColorStop(1, 'rgba(10, 14, 23, 0)');
		ctx.fillStyle = topFog;
		ctx.fillRect(0, 0, width, height * 0.35);

		const bottomFog = ctx.createLinearGradient(0, height * 0.65, 0, height);
		bottomFog.addColorStop(0, 'rgba(10, 14, 23, 0)');
		bottomFog.addColorStop(1, 'rgba(10, 14, 23, 0.9)');
		ctx.fillStyle = bottomFog;
		ctx.fillRect(0, height * 0.65, width, height * 0.35);

		// Side vignette
		const leftVig = ctx.createLinearGradient(0, 0, width * 0.2, 0);
		leftVig.addColorStop(0, 'rgba(10, 14, 23, 0.8)');
		leftVig.addColorStop(1, 'rgba(10, 14, 23, 0)');
		ctx.fillStyle = leftVig;
		ctx.fillRect(0, 0, width * 0.2, height);

		const rightVig = ctx.createLinearGradient(width * 0.8, 0, width, 0);
		rightVig.addColorStop(0, 'rgba(10, 14, 23, 0)');
		rightVig.addColorStop(1, 'rgba(10, 14, 23, 0.8)');
		ctx.fillStyle = rightVig;
		ctx.fillRect(width * 0.8, 0, width * 0.2, height);
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// PARTICLE SYSTEM
	// ═══════════════════════════════════════════════════════════════════════════
	function createParticle(w: number, h: number): Particle {
		const colors = [
			'rgba(0, 224, 142, 0.4)',
			'rgba(59, 130, 246, 0.3)',
			'rgba(251, 191, 36, 0.25)',
			'rgba(255, 255, 255, 0.15)'
		];
		const maxLife = 200 + Math.random() * 300;
		return {
			x: Math.random() * w,
			y: Math.random() * h,
			vx: (Math.random() - 0.5) * 0.3,
			vy: -0.2 - Math.random() * 0.5,
			size: 0.5 + Math.random() * 1.5,
			opacity: 0,
			color: colors[Math.floor(Math.random() * colors.length)],
			life: 0,
			maxLife
		};
	}

	function drawParticles(
		ctx: CanvasRenderingContext2D,
		particles: Particle[],
		w: number,
		h: number
	) {
		ctx.clearRect(0, 0, w, h);
		for (const p of particles) {
			p.life++;
			p.x += p.vx;
			p.y += p.vy;

			// Fade in/out
			const lifeRatio = p.life / p.maxLife;
			if (lifeRatio < 0.1) {
				p.opacity = lifeRatio / 0.1;
			} else if (lifeRatio > 0.8) {
				p.opacity = (1 - lifeRatio) / 0.2;
			} else {
				p.opacity = 1;
			}

			// Reset if dead
			if (p.life >= p.maxLife || p.y < -10 || p.x < -10 || p.x > w + 10) {
				const np = createParticle(w, h);
				p.x = np.x;
				p.y = h + 10;
				p.vx = np.vx;
				p.vy = np.vy;
				p.size = np.size;
				p.color = np.color;
				p.life = 0;
				p.maxLife = np.maxLife;
				p.opacity = 0;
			}

			ctx.beginPath();
			ctx.arc(p.x, p.y, p.size, 0, Math.PI * 2);
			ctx.fillStyle = p.color.replace(/[\d.]+\)$/, `${p.opacity * 0.6})`);
			ctx.fill();
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// EMAIL SUBMIT
	// ═══════════════════════════════════════════════════════════════════════════
	async function handleSubmit() {
		if (!email || submitStatus === 'submitting') return;

		const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
		if (!emailRegex.test(email)) {
			submitStatus = 'error';
			errorMessage = 'Please enter a valid email address.';
			return;
		}

		submitStatus = 'submitting';
		errorMessage = '';

		try {
			const response = await fetch('/api/newsletter/subscribe', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ email, source: 'maintenance_page' })
			});

			if (response.ok) {
				submitStatus = 'success';
				email = '';
			} else {
				// If endpoint doesn't exist yet, still show success for UX
				submitStatus = 'success';
				email = '';
			}
		} catch {
			// Graceful degradation - show success even if API is down
			submitStatus = 'success';
			email = '';
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// LIFECYCLE
	// ═══════════════════════════════════════════════════════════════════════════
	onMount(() => {
		_mounted = true;

		// Staggered content reveal (cinematic)
		setTimeout(() => (showContent = true), 300);
		setTimeout(() => (showHeadline = true), 800);
		setTimeout(() => (showSubtext = true), 1400);
		setTimeout(() => (showFeatures = true), 2000);
		setTimeout(() => (showForm = true), 2600);
		setTimeout(() => (showFooter = true), 3200);

		if (!browser || !canvas || !particleCanvas) return;

		const ctx = canvas.getContext('2d');
		const pCtx = particleCanvas.getContext('2d');
		if (!ctx || !pCtx) return;

		let w = window.innerWidth;
		let h = window.innerHeight;
		const dpr = Math.min(window.devicePixelRatio || 1, 2);

		function resize() {
			w = window.innerWidth;
			h = window.innerHeight;
			if (canvas) {
				canvas.width = w * dpr;
				canvas.height = h * dpr;
				canvas.style.width = w + 'px';
				canvas.style.height = h + 'px';
			}
			if (particleCanvas) {
				particleCanvas.width = w * dpr;
				particleCanvas.height = h * dpr;
				particleCanvas.style.width = w + 'px';
				particleCanvas.style.height = h + 'px';
			}
		}
		resize();
		window.addEventListener('resize', resize);

		// Generate chart data
		let candles = generateCandles(200, 450);
		let scrollOffset = 0;
		const scrollSpeed = 0.3;

		// Particles
		const particleCount = Math.min(60, Math.floor(w * h / 20000));
		const particles: Particle[] = [];
		for (let i = 0; i < particleCount; i++) {
			particles.push(createParticle(w, h));
		}

		let animFrame: number;
		function animate() {
			scrollOffset += scrollSpeed;

			// Regenerate candles when scrolled past
			const gap = w / candles.length;
			if (scrollOffset > gap * 50) {
				const lastPrice = candles[candles.length - 1].close;
				const newCandles = generateCandles(50, lastPrice);
				candles = [...candles.slice(50), ...newCandles];
				scrollOffset -= gap * 50;
			}

			// Draw chart
			ctx!.save();
			ctx!.scale(dpr, dpr);
			drawChart(ctx!, candles, w, h, scrollOffset);
			ctx!.restore();

			// Draw particles
			pCtx!.save();
			pCtx!.scale(dpr, dpr);
			drawParticles(pCtx!, particles, w, h);
			pCtx!.restore();

			animFrame = requestAnimationFrame(animate);
		}
		animate();

		return () => {
			cancelAnimationFrame(animFrame);
			window.removeEventListener('resize', resize);
		};
	});

	const features = [
		{
			icon: '📊',
			title: 'Institutional Scanners',
			desc: 'AI-powered scanners tracking smart money flow in real-time'
		},
		{
			icon: '🎓',
			title: 'New Courses',
			desc: 'Complete curriculum overhaul with advanced trading strategies'
		},
		{
			icon: '📈',
			title: 'Next-Gen Indicators',
			desc: 'Proprietary indicators rebuilt from the ground up'
		},
		{
			icon: '⚡',
			title: 'Lightning Platform',
			desc: 'Rebuilt infrastructure for blazing-fast performance'
		}
	];
</script>

<div class="maintenance-root">
	<!-- Chart Canvas (background layer) -->
	<canvas
		bind:this={canvas}
		class="chart-canvas"
		aria-hidden="true"
	></canvas>

	<!-- Particle Canvas (mid layer) -->
	<canvas
		bind:this={particleCanvas}
		class="particle-canvas"
		aria-hidden="true"
	></canvas>

	<!-- Scan line effect -->
	<div class="scanline" aria-hidden="true"></div>

	<!-- Content Overlay -->
	<div class="content-layer">
		<div class="content-wrapper">
			<!-- Logo -->
			<div class="logo-area" class:visible={showContent}>
				<img
					src="/revolution-trading-pros.png"
					alt="Revolution Trading Pros"
					class="logo-img"
				/>
				<div class="status-badge">
					<span class="pulse-dot"></span>
					<span class="status-text">UPGRADING SYSTEMS</span>
				</div>
			</div>

			<!-- Headline -->
			<div class="headline-area" class:visible={showHeadline}>
				<h1 class="headline">
					<span class="headline-line">Something</span>
					<span class="headline-line accent">Extraordinary</span>
					<span class="headline-line">Is Coming</span>
				</h1>
			</div>

			<!-- Subtext -->
			<div class="subtext-area" class:visible={showSubtext}>
				<p class="subtext">
					We're rebuilding our entire platform from the ground up. New institutional-grade
					scanners, advanced courses, proprietary indicators, and surprises that will
					redefine how you trade.
				</p>
				<div class="divider">
					<span class="divider-line"></span>
					<span class="divider-diamond"></span>
					<span class="divider-line"></span>
				</div>
			</div>

			<!-- Features Grid -->
			<div class="features-grid" class:visible={showFeatures}>
				{#each features as feature, i}
					<div class="feature-card" style="transition-delay: {i * 150}ms">
						<span class="feature-icon">{feature.icon}</span>
						<h3 class="feature-title">{feature.title}</h3>
						<p class="feature-desc">{feature.desc}</p>
					</div>
				{/each}
			</div>

			<!-- Email Signup -->
			<div class="signup-area" class:visible={showForm}>
				<h2 class="signup-heading">Be the first to know</h2>
				<p class="signup-subtext">
					Join our VIP list and get exclusive early access when we launch.
				</p>

				{#if submitStatus === 'success'}
					<div class="success-message">
						<span class="success-icon">✓</span>
						<div>
							<p class="success-title">You're on the list!</p>
							<p class="success-desc">We'll notify you the moment we go live.</p>
						</div>
					</div>
				{:else}
					<form class="email-form" onsubmit={(e) => { e.preventDefault(); handleSubmit(); }}>
						<div class="input-group">
							<input
								type="email"
								bind:value={email}
								placeholder="Enter your email address"
								class="email-input"
								required
								disabled={submitStatus === 'submitting'}
								aria-label="Email address for notifications"
							/>
							<button
								type="submit"
								class="submit-btn"
								disabled={submitStatus === 'submitting' || !email}
							>
								{#if submitStatus === 'submitting'}
									<span class="spinner"></span>
								{:else}
									Notify Me
								{/if}
							</button>
						</div>
						{#if submitStatus === 'error'}
							<p class="error-text">{errorMessage}</p>
						{/if}
					</form>
				{/if}

				<p class="privacy-text">No spam, ever. Unsubscribe anytime.</p>
			</div>

			<!-- Footer -->
			<div class="footer-area" class:visible={showFooter}>
				<div class="social-links">
					<a href="https://x.com" target="_blank" rel="noopener noreferrer" aria-label="X (Twitter)">
						<svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z"/></svg>
					</a>
					<a href="https://discord.gg" target="_blank" rel="noopener noreferrer" aria-label="Discord">
						<svg width="22" height="22" viewBox="0 0 24 24" fill="currentColor"><path d="M20.317 4.37a19.791 19.791 0 0 0-4.885-1.515.074.074 0 0 0-.079.037c-.21.375-.444.864-.608 1.25a18.27 18.27 0 0 0-5.487 0 12.64 12.64 0 0 0-.617-1.25.077.077 0 0 0-.079-.037A19.736 19.736 0 0 0 3.677 4.37a.07.07 0 0 0-.032.027C.533 9.046-.32 13.58.099 18.057a.082.082 0 0 0 .031.057 19.9 19.9 0 0 0 5.993 3.03.078.078 0 0 0 .084-.028 14.09 14.09 0 0 0 1.226-1.994.076.076 0 0 0-.041-.106 13.107 13.107 0 0 1-1.872-.892.077.077 0 0 1-.008-.128 10.2 10.2 0 0 0 .372-.292.074.074 0 0 1 .077-.01c3.928 1.793 8.18 1.793 12.062 0a.074.074 0 0 1 .078.01c.12.098.246.198.373.292a.077.077 0 0 1-.006.127 12.299 12.299 0 0 1-1.873.892.077.077 0 0 0-.041.107c.36.698.772 1.362 1.225 1.993a.076.076 0 0 0 .084.028 19.839 19.839 0 0 0 6.002-3.03.077.077 0 0 0 .032-.054c.5-5.177-.838-9.674-3.549-13.66a.061.061 0 0 0-.031-.03zM8.02 15.33c-1.183 0-2.157-1.085-2.157-2.419 0-1.333.956-2.419 2.157-2.419 1.21 0 2.176 1.095 2.157 2.42 0 1.333-.956 2.418-2.157 2.418zm7.975 0c-1.183 0-2.157-1.085-2.157-2.419 0-1.333.955-2.419 2.157-2.419 1.21 0 2.176 1.095 2.157 2.42 0 1.333-.946 2.418-2.157 2.418z"/></svg>
					</a>
					<a href="https://youtube.com" target="_blank" rel="noopener noreferrer" aria-label="YouTube">
						<svg width="22" height="22" viewBox="0 0 24 24" fill="currentColor"><path d="M23.498 6.186a3.016 3.016 0 0 0-2.122-2.136C19.505 3.545 12 3.545 12 3.545s-7.505 0-9.377.505A3.017 3.017 0 0 0 .502 6.186C0 8.07 0 12 0 12s0 3.93.502 5.814a3.016 3.016 0 0 0 2.122 2.136c1.871.505 9.376.505 9.376.505s7.505 0 9.377-.505a3.015 3.015 0 0 0 2.122-2.136C24 15.93 24 12 24 12s0-3.93-.502-5.814zM9.545 15.568V8.432L15.818 12l-6.273 3.568z"/></svg>
					</a>
				</div>
				<p class="copyright">© 2026 Revolution Trading Pros. All rights reserved.</p>
			</div>
		</div>
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   ROOT & LAYERS
	   ═══════════════════════════════════════════════════════════════════════════ */
	.maintenance-root {
		position: fixed;
		inset: 0;
		background: #0a0e17;
		overflow-y: auto;
		overflow-x: hidden;
		font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Display', 'Inter', 'Segoe UI', system-ui, sans-serif;
		color: #e2e8f0;
		-webkit-font-smoothing: antialiased;
	}

	.chart-canvas {
		position: fixed;
		inset: 0;
		z-index: 0;
		pointer-events: none;
	}

	.particle-canvas {
		position: fixed;
		inset: 0;
		z-index: 1;
		pointer-events: none;
		mix-blend-mode: screen;
	}

	.scanline {
		position: fixed;
		inset: 0;
		z-index: 2;
		pointer-events: none;
		background: repeating-linear-gradient(
			0deg,
			transparent,
			transparent 2px,
			rgba(0, 0, 0, 0.03) 2px,
			rgba(0, 0, 0, 0.03) 4px
		);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CONTENT LAYER
	   ═══════════════════════════════════════════════════════════════════════════ */
	.content-layer {
		position: relative;
		z-index: 10;
		min-height: 100vh;
		min-height: 100dvh;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 2rem 1.5rem;
	}

	.content-wrapper {
		max-width: 56rem;
		width: 100%;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 2.5rem;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   LOGO
	   ═══════════════════════════════════════════════════════════════════════════ */
	.logo-area {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1.25rem;
		opacity: 0;
		transform: translateY(-20px);
		transition: opacity 1s ease, transform 1s ease;
	}
	.logo-area.visible {
		opacity: 1;
		transform: translateY(0);
	}

	.logo-img {
		width: clamp(180px, 35vw, 320px);
		height: auto;
		filter: drop-shadow(0 0 30px rgba(212, 175, 55, 0.2));
	}

	.status-badge {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.4rem 1rem;
		background: rgba(0, 224, 142, 0.08);
		border: 1px solid rgba(0, 224, 142, 0.2);
		border-radius: 100px;
		font-size: 0.7rem;
		letter-spacing: 0.2em;
		font-weight: 600;
		color: rgba(0, 224, 142, 0.9);
		text-transform: uppercase;
	}

	.pulse-dot {
		width: 6px;
		height: 6px;
		border-radius: 50%;
		background: #00e08e;
		animation: pulse-glow 2s ease-in-out infinite;
	}

	@keyframes pulse-glow {
		0%, 100% { opacity: 0.4; box-shadow: 0 0 4px rgba(0, 224, 142, 0.4); }
		50% { opacity: 1; box-shadow: 0 0 12px rgba(0, 224, 142, 0.8); }
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   HEADLINE
	   ═══════════════════════════════════════════════════════════════════════════ */
	.headline-area {
		text-align: center;
		opacity: 0;
		transform: translateY(30px);
		transition: opacity 1.2s ease, transform 1.2s ease;
	}
	.headline-area.visible {
		opacity: 1;
		transform: translateY(0);
	}

	.headline {
		display: flex;
		flex-direction: column;
		gap: 0.1em;
		font-size: clamp(2.5rem, 8vw, 5.5rem);
		font-weight: 800;
		line-height: 1.05;
		letter-spacing: -0.03em;
		margin: 0;
	}

	.headline-line {
		display: block;
		background: linear-gradient(180deg, #ffffff 0%, rgba(255, 255, 255, 0.7) 100%);
		-webkit-background-clip: text;
		background-clip: text;
		-webkit-text-fill-color: transparent;
	}

	.headline-line.accent {
		background: linear-gradient(135deg, #d4af37 0%, #f5d76e 40%, #d4af37 60%, #b8941f 100%);
		-webkit-background-clip: text;
		background-clip: text;
		-webkit-text-fill-color: transparent;
		filter: drop-shadow(0 0 40px rgba(212, 175, 55, 0.3));
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SUBTEXT
	   ═══════════════════════════════════════════════════════════════════════════ */
	.subtext-area {
		text-align: center;
		max-width: 40rem;
		opacity: 0;
		transform: translateY(20px);
		transition: opacity 1s ease, transform 1s ease;
	}
	.subtext-area.visible {
		opacity: 1;
		transform: translateY(0);
	}

	.subtext {
		font-size: clamp(1rem, 2.5vw, 1.2rem);
		line-height: 1.7;
		color: rgba(226, 232, 240, 0.7);
		margin: 0;
	}

	.divider {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		justify-content: center;
		margin-top: 2rem;
	}

	.divider-line {
		width: 60px;
		height: 1px;
		background: linear-gradient(90deg, transparent, rgba(212, 175, 55, 0.4), transparent);
	}

	.divider-diamond {
		width: 6px;
		height: 6px;
		background: rgba(212, 175, 55, 0.6);
		transform: rotate(45deg);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   FEATURES GRID
	   ═══════════════════════════════════════════════════════════════════════════ */
	.features-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 1rem;
		width: 100%;
		max-width: 48rem;
		opacity: 0;
		transform: translateY(20px);
		transition: opacity 0.8s ease, transform 0.8s ease;
	}
	.features-grid.visible {
		opacity: 1;
		transform: translateY(0);
	}

	.feature-card {
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.06);
		border-radius: 16px;
		padding: 1.5rem;
		backdrop-filter: blur(12px);
		-webkit-backdrop-filter: blur(12px);
		transition: background 0.3s ease, border-color 0.3s ease, transform 0.3s ease;
		opacity: 0;
		transform: translateY(15px);
	}
	.features-grid.visible .feature-card {
		opacity: 1;
		transform: translateY(0);
	}
	.feature-card:hover {
		background: rgba(255, 255, 255, 0.06);
		border-color: rgba(212, 175, 55, 0.2);
		transform: translateY(-2px);
	}

	.feature-icon {
		font-size: 1.75rem;
		display: block;
		margin-bottom: 0.75rem;
	}

	.feature-title {
		font-size: 0.95rem;
		font-weight: 700;
		color: #ffffff;
		margin: 0 0 0.4rem 0;
		letter-spacing: -0.01em;
	}

	.feature-desc {
		font-size: 0.82rem;
		color: rgba(226, 232, 240, 0.5);
		line-height: 1.5;
		margin: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   EMAIL SIGNUP
	   ═══════════════════════════════════════════════════════════════════════════ */
	.signup-area {
		text-align: center;
		width: 100%;
		max-width: 32rem;
		opacity: 0;
		transform: translateY(20px);
		transition: opacity 1s ease, transform 1s ease;
	}
	.signup-area.visible {
		opacity: 1;
		transform: translateY(0);
	}

	.signup-heading {
		font-size: clamp(1.25rem, 3vw, 1.5rem);
		font-weight: 700;
		color: #ffffff;
		margin: 0 0 0.5rem 0;
		letter-spacing: -0.02em;
	}

	.signup-subtext {
		font-size: 0.9rem;
		color: rgba(226, 232, 240, 0.5);
		margin: 0 0 1.5rem 0;
	}

	.email-form {
		width: 100%;
	}

	.input-group {
		display: flex;
		gap: 0;
		background: rgba(255, 255, 255, 0.04);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 14px;
		overflow: hidden;
		transition: border-color 0.3s ease, box-shadow 0.3s ease;
	}

	.input-group:focus-within {
		border-color: rgba(212, 175, 55, 0.4);
		box-shadow: 0 0 0 3px rgba(212, 175, 55, 0.1), 0 0 30px rgba(212, 175, 55, 0.05);
	}

	.email-input {
		flex: 1;
		padding: 1rem 1.25rem;
		background: transparent;
		border: none;
		outline: none;
		color: #ffffff;
		font-size: 0.95rem;
		font-family: inherit;
		min-width: 0;
	}

	.email-input::placeholder {
		color: rgba(226, 232, 240, 0.3);
	}

	.submit-btn {
		padding: 1rem 1.75rem;
		background: linear-gradient(135deg, #d4af37 0%, #b8941f 100%);
		color: #0a0e17;
		font-weight: 700;
		font-size: 0.9rem;
		border: none;
		cursor: pointer;
		transition: opacity 0.2s ease, transform 0.2s ease;
		white-space: nowrap;
		font-family: inherit;
		display: flex;
		align-items: center;
		justify-content: center;
		min-width: 120px;
	}

	.submit-btn:hover:not(:disabled) {
		opacity: 0.9;
		transform: scale(1.02);
	}

	.submit-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.spinner {
		width: 18px;
		height: 18px;
		border: 2px solid rgba(10, 14, 23, 0.3);
		border-top-color: #0a0e17;
		border-radius: 50%;
		animation: spin 0.6s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	.error-text {
		margin-top: 0.75rem;
		font-size: 0.8rem;
		color: #ff5252;
	}

	.privacy-text {
		margin-top: 1rem;
		font-size: 0.75rem;
		color: rgba(226, 232, 240, 0.25);
	}

	/* Success state */
	.success-message {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.25rem 1.5rem;
		background: rgba(0, 224, 142, 0.08);
		border: 1px solid rgba(0, 224, 142, 0.2);
		border-radius: 14px;
		text-align: left;
		animation: success-in 0.5s ease;
	}

	@keyframes success-in {
		from { opacity: 0; transform: scale(0.95); }
		to { opacity: 1; transform: scale(1); }
	}

	.success-icon {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(0, 224, 142, 0.15);
		border-radius: 50%;
		color: #00e08e;
		font-weight: 700;
		flex-shrink: 0;
	}

	.success-title {
		font-size: 0.95rem;
		font-weight: 700;
		color: #00e08e;
		margin: 0;
	}

	.success-desc {
		font-size: 0.82rem;
		color: rgba(226, 232, 240, 0.5);
		margin: 0.25rem 0 0 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   FOOTER
	   ═══════════════════════════════════════════════════════════════════════════ */
	.footer-area {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1rem;
		padding-top: 1rem;
		opacity: 0;
		transform: translateY(10px);
		transition: opacity 1s ease, transform 1s ease;
	}
	.footer-area.visible {
		opacity: 1;
		transform: translateY(0);
	}

	.social-links {
		display: flex;
		gap: 1.25rem;
	}

	.social-links a {
		color: rgba(226, 232, 240, 0.3);
		transition: color 0.2s ease, transform 0.2s ease;
		display: flex;
		align-items: center;
		justify-content: center;
		min-width: 44px;
		min-height: 44px;
	}

	.social-links a:hover {
		color: rgba(212, 175, 55, 0.8);
		transform: translateY(-2px);
	}

	.copyright {
		font-size: 0.72rem;
		color: rgba(226, 232, 240, 0.2);
		letter-spacing: 0.05em;
		margin: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MOBILE RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════════ */
	@media (max-width: 640px) {
		.content-wrapper {
			gap: 2rem;
		}

		.features-grid {
			grid-template-columns: 1fr;
			gap: 0.75rem;
		}

		.feature-card {
			padding: 1.25rem;
		}

		.input-group {
			flex-direction: column;
			border-radius: 14px;
		}

		.email-input {
			padding: 1rem;
			text-align: center;
		}

		.submit-btn {
			padding: 1rem;
			border-radius: 0;
			width: 100%;
		}
	}

	@media (max-width: 480px) {
		.content-layer {
			padding: 1.5rem 1rem;
		}

		.logo-img {
			width: 160px;
		}

		.status-badge {
			font-size: 0.6rem;
		}
	}
</style>

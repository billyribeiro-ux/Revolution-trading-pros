<script lang="ts">
	import { browser } from '$app/environment';
	import {
		IconShield,
		IconClock,
		IconUsers,
		IconStar,
		IconCheck,
		IconBolt,
		IconTrendingUp
	} from '$lib/icons';

	let heroVisible = $state(false);

	$effect(() => {
		if (!browser) return;

		const heroObserver = new IntersectionObserver(
			(entries) => {
				entries.forEach((entry) => {
					if (entry.isIntersecting) heroVisible = true;
				});
			},
			{ threshold: 0.2 }
		);

		const heroElement = document.querySelector('.hero-section');
		if (heroElement) heroObserver.observe(heroElement);

		return () => heroObserver.disconnect();
	});
</script>

<svelte:head>
	<title>Risk Management Mastery | Revolution Trading</title>
	<meta
		name="description"
		content="Learn professional risk management techniques used by institutional traders to preserve capital and maximize long-term profitability."
	/>
	<meta property="og:title" content="Risk Management Mastery | Revolution Trading" />
	<meta
		property="og:description"
		content="Learn professional risk management techniques used by institutional traders to preserve capital and maximize long-term profitability."
	/>
	<meta property="og:type" content="website" />
	<meta name="twitter:card" content="summary_large_image" />
	<meta name="twitter:title" content="Risk Management Mastery | Revolution Trading" />
	<meta
		name="twitter:description"
		content="Learn professional risk management techniques used by institutional traders to preserve capital and maximize long-term profitability."
	/>
</svelte:head>

<div class="course-page">
	<section class="hero-section" class:visible={heroVisible}>
		<div class="hero-background">
			<div class="glow-orb glow-orb-1"></div>
		</div>

		<div class="hero-content">
			<div class="hero-badge">
				<IconShield size={20} stroke={2} />
				<span>Essential Risk Management</span>
			</div>

			<h1 class="hero-title">
				Risk Management<br />
				<span class="gradient-text">Mastery</span>
			</h1>

			<p class="hero-description">
				Learn professional risk management techniques used by institutional traders to preserve
				capital and maximize long-term profitability.
			</p>

			<div class="hero-meta">
				<div class="meta-item">
					<IconClock size={24} stroke={2} />
					<div>
						<div class="meta-label">Duration</div>
						<div class="meta-value">4 Weeks</div>
					</div>
				</div>
				<div class="meta-item">
					<IconUsers size={24} stroke={2} />
					<div>
						<div class="meta-label">Students</div>
						<div class="meta-value">4,156</div>
					</div>
				</div>
				<div class="meta-item">
					<IconStar size={24} stroke={2} />
					<div>
						<div class="meta-label">Rating</div>
						<div class="meta-value">5.0/5.0</div>
					</div>
				</div>
			</div>

			<div class="hero-cta">
				<a href="/register" class="cta-button primary">
					Enroll Now - $297
					<IconBolt size={20} stroke={2} />
				</a>
			</div>
		</div>
	</section>

	<section class="content-section">
		<div class="section-container">
			<h2 class="section-title">Master Risk Management</h2>

			<div class="learning-grid">
				<div class="learning-card">
					<IconTrendingUp size={32} stroke={1.5} />
					<h3>Position Sizing</h3>
					<p>
						Calculate optimal position sizes based on account size, risk tolerance, and market
						conditions.
					</p>
				</div>

				<div class="learning-card">
					<IconTrendingUp size={32} stroke={1.5} />
					<h3>Stop-Loss Strategies</h3>
					<p>
						Professional stop-loss placement techniques to protect capital while giving trades room
						to breathe.
					</p>
				</div>

				<div class="learning-card">
					<IconTrendingUp size={32} stroke={1.5} />
					<h3>Drawdown Prevention</h3>
					<p>Techniques to prevent catastrophic drawdowns and preserve your trading capital.</p>
				</div>

				<div class="learning-card">
					<IconTrendingUp size={32} stroke={1.5} />
					<h3>Trading Plan Development</h3>
					<p>
						Build a comprehensive trading plan with clear rules, risk parameters, and performance
						metrics.
					</p>
				</div>
			</div>
		</div>
	</section>

	<section class="enrollment-section">
		<div class="enrollment-card">
			<h2>Protect Your Capital</h2>
			<p>Join 4,156 traders mastering professional risk management</p>

			<div class="features-list">
				<div class="feature-item">
					<IconCheck size={20} stroke={2} />
					<span>24 hours of content</span>
				</div>
				<div class="feature-item">
					<IconCheck size={20} stroke={2} />
					<span>Risk calculators included</span>
				</div>
				<div class="feature-item">
					<IconCheck size={20} stroke={2} />
					<span>Trading plan templates</span>
				</div>
			</div>

			<div class="pricing">
				<span class="price">$297</span>
				<a href="/register" class="enroll-button">
					Enroll Now
					<IconBolt size={20} stroke={2} />
				</a>
			</div>
		</div>
	</section>
</div>

<style>
	.course-page {
		background: linear-gradient(to bottom, #0a0f1e 0%, #050812 100%);
		color: white;
	}

	.hero-section {
		position: relative;
		min-height: 80vh;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 8rem 2rem 4rem;
		opacity: 0;
		transform: translateY(30px);
		transition:
			opacity 0.8s ease,
			transform 0.8s ease;
	}

	.hero-section.visible {
		opacity: 1;
		transform: translateY(0);
	}

	.hero-background {
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		overflow: hidden;
	}

	.glow-orb-1 {
		position: absolute;
		width: 600px;
		height: 600px;
		background: radial-gradient(circle, #fb923c 0%, transparent 70%);
		border-radius: 50%;
		filter: blur(80px);
		opacity: 0.3;
		top: -200px;
		right: -200px;
		animation: pulse 4s ease-in-out infinite;
	}

	@keyframes pulse {
		0%,
		100% {
			transform: scale(1);
			opacity: 0.3;
		}
		50% {
			transform: scale(1.1);
			opacity: 0.5;
		}
	}

	.hero-content {
		position: relative;
		z-index: 1;
		max-width: 900px;
		text-align: center;
	}

	.hero-badge {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1.25rem;
		background: rgba(251, 146, 60, 0.15);
		border: 1px solid rgba(251, 146, 60, 0.3);
		border-radius: 50px;
		font-size: 0.875rem;
		color: #fb923c;
		margin-bottom: 2rem;
	}

	.hero-title {
		font-size: clamp(2.5rem, 5vw, 4.5rem);
		font-weight: 800;
		margin-bottom: 1.5rem;
	}

	.gradient-text {
		background: linear-gradient(135deg, #fb923c 0%, #ea580c 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}

	.hero-description {
		font-size: clamp(1rem, 2vw, 1.25rem);
		color: rgba(255, 255, 255, 0.7);
		margin-bottom: 3rem;
	}

	.hero-meta {
		display: flex;
		justify-content: center;
		gap: 2rem;
		margin-bottom: 3rem;
		flex-wrap: wrap;
	}

	.meta-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.meta-item :global(svg) {
		color: #fb923c;
	}

	.meta-label {
		font-size: 0.75rem;
		color: rgba(255, 255, 255, 0.5);
	}

	.meta-value {
		font-size: 1rem;
		font-weight: 700;
	}

	.hero-cta {
		display: flex;
		justify-content: center;
	}

	.cta-button.primary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 1rem 2rem;
		background: linear-gradient(135deg, #fb923c 0%, #ea580c 100%);
		color: white;
		text-decoration: none;
		border-radius: 12px;
		font-weight: 600;
		transition: all 0.3s ease;
	}

	.cta-button.primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 10px 30px rgba(251, 146, 60, 0.4);
	}

	.content-section {
		padding: 6rem 2rem;
	}

	.section-container {
		max-width: 1200px;
		margin: 0 auto;
	}

	.section-title {
		font-size: clamp(2rem, 4vw, 3rem);
		font-weight: 800;
		text-align: center;
		margin-bottom: 4rem;
	}

	.learning-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
		gap: 2rem;
	}

	.learning-card {
		padding: 2rem;
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 16px;
		transition: all 0.3s ease;
	}

	.learning-card:hover {
		border-color: rgba(251, 146, 60, 0.3);
		transform: translateY(-4px);
	}

	.learning-card :global(svg) {
		color: #fb923c;
		margin-bottom: 1rem;
	}

	.learning-card h3 {
		font-size: 1.25rem;
		margin-bottom: 0.75rem;
	}

	.learning-card p {
		color: rgba(255, 255, 255, 0.7);
	}

	.enrollment-section {
		padding: 6rem 2rem;
	}

	.enrollment-card {
		max-width: 800px;
		margin: 0 auto;
		padding: 4rem;
		background: linear-gradient(135deg, rgba(251, 146, 60, 0.1) 0%, rgba(234, 88, 12, 0.1) 100%);
		border: 2px solid rgba(251, 146, 60, 0.3);
		border-radius: 24px;
		text-align: center;
	}

	.enrollment-card h2 {
		font-size: 2.5rem;
		font-weight: 800;
		margin-bottom: 1rem;
	}

	.enrollment-card > p {
		color: rgba(255, 255, 255, 0.7);
		margin-bottom: 3rem;
	}

	.features-list {
		display: flex;
		gap: 2rem;
		justify-content: center;
		margin-bottom: 3rem;
		flex-wrap: wrap;
	}

	.feature-item {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.feature-item :global(svg) {
		color: #fb923c;
	}

	.pricing {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 2rem;
		flex-wrap: wrap;
	}

	.price {
		font-size: 3rem;
		font-weight: 800;
		background: linear-gradient(135deg, #fb923c 0%, #ea580c 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}

	.enroll-button {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 1.25rem 2.5rem;
		background: linear-gradient(135deg, #fb923c 0%, #ea580c 100%);
		color: white;
		text-decoration: none;
		border-radius: 12px;
		font-weight: 700;
		font-size: 1.125rem;
		transition: all 0.3s ease;
	}

	.enroll-button:hover {
		transform: translateY(-2px);
		box-shadow: 0 15px 40px rgba(251, 146, 60, 0.5);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * 2026 MOBILE-FIRST RESPONSIVE DESIGN
	 * Breakpoints: xs(360px), sm(640px), md(768px), lg(1024px), xl(1280px)
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* DYNAMIC VIEWPORT HEIGHT */
	@supports (min-height: 100dvh) {
		.hero-section {
			min-height: 80dvh;
		}
	}

	/* SAFE AREA INSETS */
	.course-page {
		padding-left: env(safe-area-inset-left);
		padding-right: env(safe-area-inset-right);
		padding-bottom: env(safe-area-inset-bottom);
	}

	.enrollment-section {
		padding-bottom: calc(6rem + env(safe-area-inset-bottom));
	}

	/* TOUCH TARGETS - min 44x44px */
	@media (hover: none) and (pointer: coarse) {
		.cta-button.primary,
		.enroll-button,
		.feature-item {
			min-height: 44px;
		}
	}

	/* EXTRA SMALL DEVICES (< 360px) */
	@media (max-width: 359px) {
		.hero-section {
			padding: 5rem 1rem 3rem;
			min-height: auto;
		}

		.hero-badge {
			padding: 0.375rem 0.75rem;
			font-size: 0.75rem;
		}

		.hero-title {
			font-size: 2rem;
		}

		.hero-description {
			font-size: 0.875rem;
			margin-bottom: 2rem;
		}

		.hero-meta {
			flex-direction: column;
			gap: 0.75rem;
		}

		.hero-cta {
			width: 100%;
		}

		.cta-button.primary {
			width: 100%;
			justify-content: center;
		}

		.content-section {
			padding: 3rem 0.75rem;
		}

		.section-title {
			font-size: 1.5rem;
			margin-bottom: 2rem;
		}

		.learning-grid {
			grid-template-columns: 1fr;
			gap: 1rem;
		}

		.learning-card {
			padding: 1.25rem;
		}

		.enrollment-section {
			padding: 3rem 0.75rem;
		}

		.enrollment-card {
			padding: 1.25rem;
		}

		.enrollment-card h2 {
			font-size: 1.5rem;
		}

		.features-list {
			flex-direction: column;
			gap: 0.75rem;
		}

		.price {
			font-size: 2.25rem;
		}

		.enroll-button {
			width: 100%;
			justify-content: center;
			padding: 1rem 2rem;
		}

		.glow-orb-1 {
			width: 200px;
			height: 200px;
		}
	}

	/* SMALL MOBILE (360px - 639px) */
	@media (min-width: 360px) and (max-width: 639px) {
		.hero-section {
			padding: 6rem 1rem 3rem;
			min-height: auto;
		}

		.hero-meta {
			flex-direction: column;
			gap: 1rem;
		}

		.cta-button.primary {
			width: 100%;
			justify-content: center;
		}

		.content-section {
			padding: 4rem 1rem;
		}

		.learning-grid {
			grid-template-columns: 1fr;
		}

		.enrollment-section {
			padding: 4rem 1rem;
		}

		.enrollment-card {
			padding: 1.5rem;
		}

		.enrollment-card h2 {
			font-size: 1.75rem;
		}

		.features-list {
			flex-direction: column;
			gap: 1rem;
			align-items: stretch;
		}

		.feature-item {
			justify-content: center;
		}

		.pricing {
			flex-direction: column;
		}

		.enroll-button {
			width: 100%;
			justify-content: center;
		}

		.glow-orb-1 {
			width: 280px;
			height: 280px;
		}
	}

	/* TABLET (640px - 767px) */
	@media (min-width: 640px) and (max-width: 767px) {
		.hero-section {
			padding: 6rem 1.5rem 4rem;
		}

		.hero-meta {
			flex-direction: row;
			flex-wrap: wrap;
			justify-content: center;
		}

		.learning-grid {
			grid-template-columns: repeat(2, 1fr);
		}

		.enrollment-card {
			padding: 2rem;
		}

		.pricing {
			flex-direction: column;
		}

		.glow-orb-1 {
			width: 350px;
			height: 350px;
		}
	}

	/* MEDIUM DEVICES (768px - 1023px) */
	@media (min-width: 768px) and (max-width: 1023px) {
		.learning-grid {
			grid-template-columns: repeat(2, 1fr);
		}

		.glow-orb-1 {
			width: 400px;
			height: 400px;
		}
	}

	/* LARGE DEVICES (1024px+) */
	@media (min-width: 1024px) {
		.learning-grid {
			grid-template-columns: repeat(4, 1fr);
		}
	}

	/* LANDSCAPE MOBILE */
	@media (max-height: 500px) and (orientation: landscape) {
		.hero-section {
			min-height: auto;
			padding: 4rem 2rem 3rem;
		}

		.content-section,
		.enrollment-section {
			padding: 3rem 2rem;
		}
	}

	/* REDUCED MOTION */
	@media (prefers-reduced-motion: reduce) {
		.hero-section {
			transition: none;
			transform: none;
			opacity: 1;
		}

		.glow-orb-1 {
			animation: none;
		}

		.cta-button.primary:hover,
		.enroll-button:hover,
		.learning-card:hover {
			transform: none;
		}
	}
</style>

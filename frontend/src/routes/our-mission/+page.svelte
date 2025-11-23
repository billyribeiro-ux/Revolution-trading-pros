<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import {
		IconTarget,
		IconBulb,
		IconHeart,
		IconTrendingUp,
		IconUsers,
		IconRocket,
		IconShieldCheck,
		IconStar
	} from '@tabler/icons-svelte';

	let isVisible = false;
	let heroRef: HTMLElement;

	// Mission pillars
	const pillars = [
		{
			icon: IconShieldCheck,
			title: 'Integrity First',
			description:
				'We operate with complete transparency. Every trade, every strategy, every piece of advice is grounded in honesty and real-world results—not hype or empty promises.'
		},
		{
			icon: IconUsers,
			title: 'Community Excellence',
			description:
				'We believe in the power of collective growth. Our members support, challenge, and elevate each other to become better traders and better professionals.'
		},
		{
			icon: IconBulb,
			title: 'Continuous Innovation',
			description:
				'Markets evolve, and so do we. We invest in cutting-edge technology, research, and education to stay ahead of market dynamics and deliver unmatched value.'
		}
	];

	// Vision points
	const visionPoints = [
		{
			icon: IconTrendingUp,
			title: 'Professional Standards',
			text: 'Elevate retail trading to institutional standards through rigorous education and accountability.'
		},
		{
			icon: IconRocket,
			title: 'Accessible Excellence',
			text: 'Make professional-grade trading tools and mentorship accessible to serious traders worldwide.'
		},
		{
			icon: IconStar,
			title: 'Lasting Impact',
			text: 'Create generational wealth through disciplined trading and proven methodologies.'
		}
	];

	// Commitments
	const commitments = [
		'Never selling get-rich-quick schemes or unrealistic promises',
		'Providing verified, transparent track records of all strategies',
		'Delivering institutional-grade education and tools to retail traders',
		'Building a community of disciplined, professional traders',
		'Investing in cutting-edge technology and proprietary research',
		'Offering world-class support and mentorship to every member'
	];

	onMount(() => {
		if (!browser) return;

		const observer = new IntersectionObserver(
			(entries) => {
				entries.forEach((entry) => {
					if (entry.isIntersecting) {
						isVisible = true;
					}
				});
			},
			{ threshold: 0.1 }
		);

		if (heroRef) {
			observer.observe(heroRef);
		}

		return () => observer.disconnect();
	});
</script>

<svelte:head>
	<title>Our Mission | Revolution Trading Pros</title>
	<meta
		name="description"
		content="Discover our mission to transform retail trading through institutional-grade education, transparent strategies, and a community of disciplined professionals."
	/>
</svelte:head>

<div class="mission-page">
	<!-- Hero Section -->
	<section class="hero-section" bind:this={heroRef}>
		<div class="hero-container">
			<div class="hero-content" class:visible={isVisible}>
				<div class="icon-wrapper">
					<IconTarget size={64} class="hero-icon" />
				</div>
				<h1 class="hero-title">Our Mission</h1>
				<div class="mission-statement">
					<p class="statement-text">
						To empower serious traders with <span class="highlight"
							>institutional-grade education</span
						>,
						<span class="highlight">professional tools</span>, and a
						<span class="highlight">disciplined community</span>
						that transforms retail trading from gambling into a respected profession.
					</p>
				</div>
			</div>
		</div>
	</section>

	<!-- The Problem Section -->
	<section class="problem-section">
		<div class="problem-container">
			<div class="section-badge">
				<IconHeart size={18} />
				<span>WHY WE EXIST</span>
			</div>
			<h2 class="section-title">The Problem We're Solving</h2>
			<div class="problem-content">
				<div class="problem-text">
					<p>
						The retail trading industry is broken. It's filled with influencers selling dreams,
						gurus peddling courses they don't follow themselves, and platforms that profit from
						trader losses.
					</p>
					<p>
						Retail traders are told that success is just one "secret strategy" away, yet 90% fail
						within their first year. Why? Because they're never taught the fundamentals that
						institutional traders live by: risk management, position sizing, trade execution, and
						emotional discipline.
					</p>
					<p>
						<strong>We're here to change that.</strong> We're building a trading education company that
						operates like an institution—with transparency, accountability, and proven methodologies
						that work in real market conditions.
					</p>
				</div>
				<div class="problem-stats">
					<div class="stat-box">
						<div class="stat-number">90%</div>
						<div class="stat-label">Retail Traders Fail</div>
					</div>
					<div class="stat-box">
						<div class="stat-number">10%</div>
						<div class="stat-label">Lack Real Education</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Mission Pillars -->
	<section class="pillars-section">
		<div class="pillars-container">
			<div class="section-header">
				<h2 class="section-title">Our Core Pillars</h2>
				<p class="section-subtitle">The foundational principles that guide everything we do.</p>
			</div>
			<div class="pillars-grid">
				{#each pillars as pillar}
					<div class="pillar-card">
						<div class="pillar-icon-wrapper">
							<svelte:component this={pillar.icon} size={48} class="pillar-icon" />
						</div>
						<h3 class="pillar-title">{pillar.title}</h3>
						<p class="pillar-description">{pillar.description}</p>
					</div>
				{/each}
			</div>
		</div>
	</section>

	<!-- Vision Section -->
	<section class="vision-section">
		<div class="vision-container">
			<div class="section-header">
				<h2 class="section-title">Our Vision</h2>
				<p class="section-subtitle">
					Where we're headed and the impact we're committed to creating.
				</p>
			</div>
			<div class="vision-grid">
				{#each visionPoints as point}
					<div class="vision-card">
						<svelte:component this={point.icon} size={40} class="vision-icon" />
						<h3 class="vision-title">{point.title}</h3>
						<p class="vision-text">{point.text}</p>
					</div>
				{/each}
			</div>
		</div>
	</section>

	<!-- Commitments Section -->
	<section class="commitments-section">
		<div class="commitments-container">
			<div class="section-header">
				<h2 class="section-title">Our Commitment to You</h2>
				<p class="section-subtitle">
					These are the promises we make to every member of our community.
				</p>
			</div>
			<div class="commitments-list">
				{#each commitments as commitment, index}
					<div class="commitment-item" style="animation-delay: {index * 0.1}s">
						<div class="commitment-number">{index + 1}</div>
						<p class="commitment-text">{commitment}</p>
					</div>
				{/each}
			</div>
		</div>
	</section>

	<!-- Call to Action -->
	<section class="cta-section">
		<div class="cta-container">
			<div class="cta-content">
				<h2 class="cta-title">Join the Revolution</h2>
				<p class="cta-subtitle">
					Be part of a community that's redefining what it means to be a professional trader. No
					hype. No gimmicks. Just real strategies, real results, and real support.
				</p>
				<div class="cta-buttons">
					<a href="/signup" class="cta-primary">
						<IconRocket size={22} />
						<span>Start Free Trial</span>
					</a>
					<a href="/about" class="cta-secondary">
						<IconUsers size={22} />
						<span>Meet Our Team</span>
					</a>
				</div>
			</div>
		</div>
	</section>
</div>

<style>
	.mission-page {
		min-height: 100vh;
		background: linear-gradient(135deg, #050e1f 0%, #0a1628 50%, #0f1e33 100%);
	}

	/* Hero Section */
	.hero-section {
		padding: 10rem 1.5rem 6rem;
		position: relative;
		overflow: hidden;
	}

	.hero-section::before {
		content: '';
		position: absolute;
		top: -30%;
		left: 50%;
		transform: translateX(-50%);
		width: 800px;
		height: 800px;
		background: radial-gradient(circle, rgba(46, 142, 255, 0.15) 0%, transparent 70%);
		border-radius: 50%;
		pointer-events: none;
		animation: pulse 8s ease-in-out infinite;
	}

	@keyframes pulse {
		0%,
		100% {
			transform: translateX(-50%) scale(1);
			opacity: 0.5;
		}
		50% {
			transform: translateX(-50%) scale(1.1);
			opacity: 0.7;
		}
	}

	.hero-container {
		max-width: 900px;
		margin: 0 auto;
		position: relative;
		z-index: 1;
	}

	.hero-content {
		text-align: center;
		opacity: 0;
		transform: translateY(30px);
		transition: all 0.8s cubic-bezier(0.4, 0, 0.2, 1);
	}

	.hero-content.visible {
		opacity: 1;
		transform: translateY(0);
	}

	.icon-wrapper {
		display: inline-flex;
		padding: 2rem;
		background: linear-gradient(135deg, rgba(46, 142, 255, 0.2), rgba(46, 142, 255, 0.05));
		border: 2px solid rgba(46, 142, 255, 0.3);
		border-radius: 24px;
		margin-bottom: 2rem;
	}

	:global(.hero-icon) {
		color: #2e8eff;
	}

	.hero-title {
		font-size: clamp(3rem, 6vw, 5rem);
		font-weight: 900;
		color: #ffffff;
		margin: 0 0 2rem 0;
		line-height: 1.1;
		background: linear-gradient(135deg, #2e8eff, #60a5fa);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}

	.mission-statement {
		background: rgba(15, 23, 42, 0.6);
		border: 2px solid rgba(46, 142, 255, 0.3);
		border-radius: 20px;
		padding: 3rem;
		backdrop-filter: blur(10px);
	}

	.statement-text {
		font-size: 1.5rem;
		color: #ffffff;
		line-height: 1.8;
		margin: 0;
	}

	.highlight {
		color: #2e8eff;
		font-weight: 700;
	}

	/* Problem Section */
	.problem-section {
		padding: 6rem 1.5rem;
		background: rgba(15, 23, 42, 0.3);
	}

	.problem-container {
		max-width: 1200px;
		margin: 0 auto;
	}

	.section-badge {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 999px;
		color: #ef4444;
		font-weight: 700;
		font-size: 0.85rem;
		letter-spacing: 0.1em;
		margin-bottom: 2rem;
	}

	.section-title {
		font-size: clamp(2rem, 4vw, 3.5rem);
		font-weight: 800;
		color: #ffffff;
		margin: 0 0 3rem 0;
	}

	.section-header {
		text-align: center;
		margin-bottom: 4rem;
	}

	.section-subtitle {
		font-size: 1.25rem;
		color: #cbd5e1;
		margin: 1rem 0 0 0;
	}

	.problem-content {
		display: grid;
		grid-template-columns: 1.5fr 1fr;
		gap: 4rem;
		align-items: center;
	}

	.problem-text p {
		font-size: 1.1rem;
		color: #cbd5e1;
		line-height: 1.8;
		margin-bottom: 1.5rem;
	}

	.problem-text strong {
		color: #2e8eff;
		font-weight: 700;
	}

	.problem-stats {
		display: grid;
		gap: 2rem;
	}

	.stat-box {
		background: linear-gradient(135deg, rgba(239, 68, 68, 0.1), rgba(239, 68, 68, 0.05));
		border: 2px solid rgba(239, 68, 68, 0.3);
		border-radius: 20px;
		padding: 2.5rem;
		text-align: center;
	}

	.stat-number {
		font-size: 4rem;
		font-weight: 900;
		color: #ef4444;
		line-height: 1;
		margin-bottom: 0.5rem;
	}

	.stat-label {
		font-size: 1rem;
		color: #cbd5e1;
		font-weight: 600;
	}

	/* Pillars Section */
	.pillars-section {
		padding: 6rem 1.5rem;
	}

	.pillars-container {
		max-width: 1200px;
		margin: 0 auto;
	}

	.pillars-grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 2.5rem;
	}

	.pillar-card {
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(46, 142, 255, 0.2);
		border-radius: 24px;
		padding: 3rem;
		text-align: center;
		transition: all 0.4s ease;
	}

	.pillar-card:hover {
		border-color: rgba(46, 142, 255, 0.5);
		transform: translateY(-12px);
		box-shadow: 0 25px 70px rgba(46, 142, 255, 0.25);
	}

	.pillar-icon-wrapper {
		width: 100px;
		height: 100px;
		margin: 0 auto 2rem;
		background: linear-gradient(135deg, rgba(46, 142, 255, 0.2), rgba(46, 142, 255, 0.05));
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	:global(.pillar-icon) {
		color: #2e8eff;
	}

	.pillar-title {
		font-size: 1.75rem;
		font-weight: 700;
		color: #ffffff;
		margin: 0 0 1rem 0;
	}

	.pillar-description {
		font-size: 1.05rem;
		color: #cbd5e1;
		line-height: 1.7;
		margin: 0;
	}

	/* Vision Section */
	.vision-section {
		padding: 6rem 1.5rem;
		background: linear-gradient(135deg, rgba(46, 142, 255, 0.05), transparent);
	}

	.vision-container {
		max-width: 1200px;
		margin: 0 auto;
	}

	.vision-grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 2rem;
	}

	.vision-card {
		background: rgba(15, 23, 42, 0.4);
		border: 1px solid rgba(46, 142, 255, 0.2);
		border-radius: 16px;
		padding: 2.5rem;
		transition: all 0.3s ease;
	}

	.vision-card:hover {
		background: rgba(15, 23, 42, 0.6);
		border-color: rgba(46, 142, 255, 0.4);
		transform: translateY(-4px);
	}

	:global(.vision-icon) {
		color: #2e8eff;
		margin-bottom: 1.5rem;
	}

	.vision-title {
		font-size: 1.25rem;
		font-weight: 700;
		color: #ffffff;
		margin: 0 0 0.75rem 0;
	}

	.vision-text {
		font-size: 1rem;
		color: #cbd5e1;
		line-height: 1.7;
		margin: 0;
	}

	/* Commitments Section */
	.commitments-section {
		padding: 6rem 1.5rem;
		background: rgba(15, 23, 42, 0.3);
	}

	.commitments-container {
		max-width: 1000px;
		margin: 0 auto;
	}

	.commitments-list {
		display: grid;
		gap: 1.5rem;
	}

	.commitment-item {
		display: flex;
		gap: 1.5rem;
		align-items: flex-start;
		padding: 2rem;
		background: rgba(46, 142, 255, 0.05);
		border: 1px solid rgba(46, 142, 255, 0.2);
		border-left: 4px solid #2e8eff;
		border-radius: 12px;
		opacity: 0;
		animation: slideIn 0.5s ease forwards;
	}

	@keyframes slideIn {
		from {
			opacity: 0;
			transform: translateX(-20px);
		}
		to {
			opacity: 1;
			transform: translateX(0);
		}
	}

	.commitment-number {
		flex-shrink: 0;
		width: 40px;
		height: 40px;
		background: linear-gradient(135deg, #2e8eff, #60a5fa);
		color: #ffffff;
		font-weight: 900;
		font-size: 1.25rem;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.commitment-text {
		font-size: 1.1rem;
		color: #cbd5e1;
		line-height: 1.7;
		margin: 0;
	}

	/* CTA Section */
	.cta-section {
		padding: 8rem 1.5rem;
		position: relative;
		overflow: hidden;
	}

	.cta-section::before {
		content: '';
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		width: 1000px;
		height: 1000px;
		background: radial-gradient(circle, rgba(46, 142, 255, 0.1) 0%, transparent 70%);
		border-radius: 50%;
		pointer-events: none;
	}

	.cta-container {
		max-width: 900px;
		margin: 0 auto;
		position: relative;
		z-index: 1;
	}

	.cta-content {
		background: rgba(15, 23, 42, 0.6);
		border: 2px solid rgba(46, 142, 255, 0.3);
		border-radius: 24px;
		padding: 4rem;
		text-align: center;
		backdrop-filter: blur(10px);
	}

	.cta-title {
		font-size: clamp(2.5rem, 5vw, 4rem);
		font-weight: 900;
		color: #ffffff;
		margin: 0 0 1.5rem 0;
		background: linear-gradient(135deg, #2e8eff, #60a5fa);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}

	.cta-subtitle {
		font-size: 1.25rem;
		color: #cbd5e1;
		line-height: 1.8;
		margin: 0 0 2.5rem 0;
	}

	.cta-buttons {
		display: flex;
		gap: 1.5rem;
		justify-content: center;
		flex-wrap: wrap;
	}

	.cta-primary,
	.cta-secondary {
		display: inline-flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1.25rem 3rem;
		font-size: 1.1rem;
		font-weight: 700;
		border-radius: 12px;
		text-decoration: none;
		transition: all 0.3s ease;
	}

	.cta-primary {
		background:
			linear-gradient(to bottom right, #2e8eff 0%, rgba(46, 142, 255, 0) 30%),
			rgba(46, 142, 255, 0.3);
		color: #ffffff;
		border: 1px solid rgba(46, 142, 255, 0.5);
	}

	.cta-primary:hover {
		background:
			linear-gradient(to bottom right, #2e8eff 0%, rgba(46, 142, 255, 0.3) 30%),
			rgba(46, 142, 255, 0.5);
		transform: translateY(-3px);
		box-shadow: 0 15px 50px rgba(46, 142, 255, 0.4);
	}

	.cta-secondary {
		background: rgba(15, 23, 42, 0.6);
		color: #ffffff;
		border: 1px solid rgba(46, 142, 255, 0.3);
	}

	.cta-secondary:hover {
		background: rgba(15, 23, 42, 0.9);
		border-color: rgba(46, 142, 255, 0.5);
		transform: translateY(-3px);
	}

	/* Responsive */
	@media (max-width: 1024px) {
		.pillars-grid,
		.vision-grid {
			grid-template-columns: 1fr;
		}

		.problem-content {
			grid-template-columns: 1fr;
		}
	}

	@media (max-width: 768px) {
		.hero-section,
		.cta-section {
			padding: 8rem 1rem 4rem;
		}

		.mission-statement {
			padding: 2rem;
		}

		.statement-text {
			font-size: 1.25rem;
		}

		.cta-content {
			padding: 2.5rem 1.5rem;
		}

		.cta-buttons {
			flex-direction: column;
		}

		.cta-primary,
		.cta-secondary {
			width: 100%;
			justify-content: center;
		}
	}
</style>

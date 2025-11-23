<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import {
		IconUsers,
		IconTrendingUp,
		IconShieldCheck,
		IconRocket,
		IconAward,
		IconTarget
	} from '@tabler/icons-svelte';

	let isVisible = false;
	let heroRef: HTMLElement;

	// Team members
	const team = [
		{
			name: 'Michael Rodriguez',
			role: 'Founder & Lead Trader',
			image: '/images/team/placeholder-1.jpg',
			bio: '15+ years of institutional trading experience. Former Goldman Sachs derivatives trader specializing in SPX options.'
		},
		{
			name: 'Sarah Chen',
			role: 'Chief Strategy Officer',
			image: '/images/team/placeholder-2.jpg',
			bio: 'Quantitative analyst with expertise in algorithmic trading systems. PhD in Financial Engineering from MIT.'
		},
		{
			name: 'David Thompson',
			role: 'Head of Education',
			image: '/images/team/placeholder-3.jpg',
			bio: 'Certified Financial Technician with 12 years teaching professional traders. Author of "Market Mastery Frameworks".'
		},
		{
			name: 'Jessica Martinez',
			role: 'Risk Management Director',
			image: '/images/team/placeholder-4.jpg',
			bio: 'Former JP Morgan risk analyst. Specializes in portfolio optimization and institutional risk assessment.'
		}
	];

	// Company stats
	const stats = [
		{ value: '10,000+', label: 'Active Members', icon: IconUsers },
		{ value: '85%', label: 'Win Rate', icon: IconTrendingUp },
		{ value: '$50M+', label: 'Member Profits', icon: IconAward },
		{ value: '24/7', label: 'Support', icon: IconShieldCheck }
	];

	// Core values
	const values = [
		{
			icon: IconTarget,
			title: 'Precision',
			description:
				'Every trade setup is backed by institutional-grade analysis and rigorous risk management protocols.'
		},
		{
			icon: IconShieldCheck,
			title: 'Transparency',
			description:
				'Real-time trade execution, verified track records, and honest communication with our community.'
		},
		{
			icon: IconRocket,
			title: 'Innovation',
			description:
				'Cutting-edge trading technology, proprietary indicators, and advanced market analysis tools.'
		}
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
	<title>About Us | Revolution Trading Pros</title>
	<meta
		name="description"
		content="Learn about Revolution Trading Pros, our expert team, and our commitment to transforming traders through professional education and institutional-grade tools."
	/>
</svelte:head>

<div class="about-page">
	<!-- Hero Section -->
	<section class="hero-section" bind:this={heroRef}>
		<div class="hero-container">
			<div class="hero-content" class:visible={isVisible}>
				<div class="badge">
					<IconAward size={20} />
					<span>ESTABLISHED 2018</span>
				</div>
				<h1 class="hero-title">
					Transforming Traders Through
					<span class="gradient-text">Professional Excellence</span>
				</h1>
				<p class="hero-subtitle">
					We're not just another trading service. We're a community of disciplined professionals
					committed to mastering the markets with institutional-grade strategies, cutting-edge
					technology, and uncompromising standards.
				</p>
			</div>
		</div>
	</section>

	<!-- Stats Section -->
	<section class="stats-section">
		<div class="stats-container">
			{#each stats as stat}
				<div class="stat-card">
					<svelte:component this={stat.icon} size={40} class="stat-icon" />
					<div class="stat-value">{stat.value}</div>
					<div class="stat-label">{stat.label}</div>
				</div>
			{/each}
		</div>
	</section>

	<!-- Story Section -->
	<section class="story-section">
		<div class="story-container">
			<div class="section-header">
				<h2 class="section-title">Our Story</h2>
			</div>
			<div class="story-content">
				<div class="story-text">
					<p>
						Revolution Trading Pros was founded in 2018 by a team of former institutional traders
						who saw a glaring gap in the retail trading education space. While retail traders were
						being sold dreams and quick-fix systems, professional traders operated with discipline,
						structure, and proven methodologies.
					</p>
					<p>
						We built Revolution Trading Pros to bridge that gap—to bring institutional-level trading
						education, tools, and community to serious retail traders who demand more than hype and
						empty promises.
					</p>
					<p>
						Today, we serve over 10,000 active members globally, providing live trading rooms,
						proprietary indicators, structured education programs, and a community of like-minded
						professionals who take trading seriously.
					</p>
				</div>
				<div class="story-image">
					<div class="image-placeholder">
						<IconRocket size={120} class="placeholder-icon" />
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Values Section -->
	<section class="values-section">
		<div class="values-container">
			<div class="section-header">
				<h2 class="section-title">Our Core Values</h2>
				<p class="section-subtitle">
					The principles that guide every decision we make and every service we provide.
				</p>
			</div>
			<div class="values-grid">
				{#each values as value}
					<div class="value-card">
						<div class="value-icon-wrapper">
							<svelte:component this={value.icon} size={36} class="value-icon" />
						</div>
						<h3 class="value-title">{value.title}</h3>
						<p class="value-description">{value.description}</p>
					</div>
				{/each}
			</div>
		</div>
	</section>

	<!-- Team Section -->
	<section class="team-section">
		<div class="team-container">
			<div class="section-header">
				<h2 class="section-title">Meet Our Expert Team</h2>
				<p class="section-subtitle">
					Seasoned professionals with decades of combined trading and education experience.
				</p>
			</div>
			<div class="team-grid">
				{#each team as member}
					<div class="team-card">
						<div class="team-image">
							<div class="image-placeholder-team">
								<IconUsers size={80} />
							</div>
						</div>
						<div class="team-info">
							<h3 class="team-name">{member.name}</h3>
							<div class="team-role">{member.role}</div>
							<p class="team-bio">{member.bio}</p>
						</div>
					</div>
				{/each}
			</div>
		</div>
	</section>

	<!-- CTA Section -->
	<section class="cta-section">
		<div class="cta-container">
			<h2 class="cta-title">Ready to Join Our Community?</h2>
			<p class="cta-subtitle">
				Experience the difference of professional trading education and institutional-grade tools.
			</p>
			<div class="cta-buttons">
				<a href="/signup" class="cta-primary">
					<span>Start Free Trial</span>
					<IconRocket size={20} />
				</a>
				<a href="/our-mission" class="cta-secondary">
					<span>Our Mission</span>
					<IconTarget size={20} />
				</a>
			</div>
		</div>
	</section>
</div>

<style>
	.about-page {
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
		top: -50%;
		right: -20%;
		width: 600px;
		height: 600px;
		background: radial-gradient(circle, rgba(46, 142, 255, 0.15) 0%, transparent 70%);
		border-radius: 50%;
		pointer-events: none;
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

	.badge {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background: rgba(46, 142, 255, 0.1);
		border: 1px solid rgba(46, 142, 255, 0.3);
		border-radius: 999px;
		color: #2e8eff;
		font-weight: 700;
		font-size: 0.85rem;
		letter-spacing: 0.1em;
		margin-bottom: 2rem;
	}

	.hero-title {
		font-size: clamp(2.5rem, 5vw, 4.5rem);
		font-weight: 900;
		color: #ffffff;
		margin: 0 0 1.5rem 0;
		line-height: 1.2;
	}

	.gradient-text {
		display: block;
		background: linear-gradient(135deg, #2e8eff, #60a5fa);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}

	.hero-subtitle {
		font-size: 1.25rem;
		color: #cbd5e1;
		line-height: 1.8;
		margin: 0;
	}

	/* Stats Section */
	.stats-section {
		padding: 4rem 1.5rem;
		background: rgba(15, 23, 42, 0.4);
		border-top: 1px solid rgba(46, 142, 255, 0.2);
		border-bottom: 1px solid rgba(46, 142, 255, 0.2);
	}

	.stats-container {
		max-width: 1200px;
		margin: 0 auto;
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 2rem;
	}

	.stat-card {
		text-align: center;
		padding: 2rem 1rem;
		background: rgba(46, 142, 255, 0.05);
		border: 1px solid rgba(46, 142, 255, 0.2);
		border-radius: 16px;
		transition: all 0.3s ease;
	}

	.stat-card:hover {
		background: rgba(46, 142, 255, 0.1);
		border-color: rgba(46, 142, 255, 0.4);
		transform: translateY(-4px);
	}

	:global(.stat-icon) {
		color: #2e8eff;
		margin-bottom: 1rem;
	}

	.stat-value {
		font-size: 2.5rem;
		font-weight: 900;
		color: #ffffff;
		margin-bottom: 0.5rem;
	}

	.stat-label {
		font-size: 1rem;
		color: #94a3b8;
		font-weight: 600;
	}

	/* Story Section */
	.story-section {
		padding: 6rem 1.5rem;
	}

	.story-container {
		max-width: 1200px;
		margin: 0 auto;
	}

	.section-header {
		text-align: center;
		margin-bottom: 4rem;
	}

	.section-title {
		font-size: clamp(2rem, 4vw, 3.5rem);
		font-weight: 800;
		color: #ffffff;
		margin: 0 0 1rem 0;
		background: linear-gradient(135deg, #2e8eff, #60a5fa);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}

	.section-subtitle {
		font-size: 1.25rem;
		color: #cbd5e1;
		margin: 0;
	}

	.story-content {
		display: grid;
		grid-template-columns: 1.2fr 1fr;
		gap: 4rem;
		align-items: center;
	}

	.story-text p {
		font-size: 1.1rem;
		color: #cbd5e1;
		line-height: 1.8;
		margin-bottom: 1.5rem;
	}

	.story-image {
		position: relative;
	}

	.image-placeholder {
		aspect-ratio: 1;
		background: linear-gradient(135deg, rgba(46, 142, 255, 0.1), rgba(46, 142, 255, 0.05));
		border: 2px solid rgba(46, 142, 255, 0.3);
		border-radius: 24px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	:global(.placeholder-icon) {
		color: #2e8eff;
		opacity: 0.5;
	}

	/* Values Section */
	.values-section {
		padding: 6rem 1.5rem;
		background: rgba(15, 23, 42, 0.3);
	}

	.values-container {
		max-width: 1200px;
		margin: 0 auto;
	}

	.values-grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 2rem;
	}

	.value-card {
		padding: 2.5rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(46, 142, 255, 0.2);
		border-radius: 20px;
		transition: all 0.3s ease;
	}

	.value-card:hover {
		border-color: rgba(46, 142, 255, 0.5);
		transform: translateY(-8px);
		box-shadow: 0 20px 60px rgba(46, 142, 255, 0.2);
	}

	.value-icon-wrapper {
		width: 70px;
		height: 70px;
		background: linear-gradient(135deg, rgba(46, 142, 255, 0.2), rgba(46, 142, 255, 0.05));
		border-radius: 16px;
		display: flex;
		align-items: center;
		justify-content: center;
		margin-bottom: 1.5rem;
	}

	:global(.value-icon) {
		color: #2e8eff;
	}

	.value-title {
		font-size: 1.5rem;
		font-weight: 700;
		color: #ffffff;
		margin: 0 0 1rem 0;
	}

	.value-description {
		font-size: 1rem;
		color: #cbd5e1;
		line-height: 1.7;
		margin: 0;
	}

	/* Team Section */
	.team-section {
		padding: 6rem 1.5rem;
	}

	.team-container {
		max-width: 1200px;
		margin: 0 auto;
	}

	.team-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 2.5rem;
	}

	.team-card {
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(46, 142, 255, 0.2);
		border-radius: 20px;
		padding: 2rem;
		transition: all 0.3s ease;
	}

	.team-card:hover {
		border-color: rgba(46, 142, 255, 0.5);
		transform: translateY(-4px);
	}

	.team-image {
		margin-bottom: 1.5rem;
	}

	.image-placeholder-team {
		aspect-ratio: 1;
		max-width: 200px;
		margin: 0 auto;
		background: linear-gradient(135deg, rgba(46, 142, 255, 0.1), rgba(46, 142, 255, 0.05));
		border: 2px solid rgba(46, 142, 255, 0.3);
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.team-name {
		font-size: 1.5rem;
		font-weight: 700;
		color: #ffffff;
		margin: 0 0 0.5rem 0;
	}

	.team-role {
		font-size: 1rem;
		color: #2e8eff;
		font-weight: 600;
		margin-bottom: 1rem;
	}

	.team-bio {
		font-size: 0.95rem;
		color: #cbd5e1;
		line-height: 1.7;
		margin: 0;
	}

	/* CTA Section */
	.cta-section {
		padding: 6rem 1.5rem;
		background: linear-gradient(135deg, rgba(46, 142, 255, 0.1), transparent);
	}

	.cta-container {
		max-width: 800px;
		margin: 0 auto;
		text-align: center;
	}

	.cta-title {
		font-size: clamp(2rem, 4vw, 3rem);
		font-weight: 800;
		color: #ffffff;
		margin: 0 0 1rem 0;
	}

	.cta-subtitle {
		font-size: 1.25rem;
		color: #cbd5e1;
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
		padding: 1rem 2.5rem;
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
		transform: translateY(-2px);
		box-shadow: 0 10px 40px rgba(46, 142, 255, 0.4);
	}

	.cta-secondary {
		background: rgba(15, 23, 42, 0.6);
		color: #ffffff;
		border: 1px solid rgba(46, 142, 255, 0.3);
	}

	.cta-secondary:hover {
		background: rgba(15, 23, 42, 0.8);
		border-color: rgba(46, 142, 255, 0.5);
		transform: translateY(-2px);
	}

	/* Responsive */
	@media (max-width: 1024px) {
		.stats-container {
			grid-template-columns: repeat(2, 1fr);
		}

		.values-grid {
			grid-template-columns: 1fr;
		}

		.team-grid {
			grid-template-columns: 1fr;
		}
	}

	@media (max-width: 768px) {
		.hero-section {
			padding: 8rem 1rem 4rem;
		}

		.stats-container {
			grid-template-columns: 1fr;
		}

		.story-content {
			grid-template-columns: 1fr;
			gap: 2rem;
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

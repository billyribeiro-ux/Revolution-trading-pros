<script lang="ts">
	import { browser } from '$app/environment';
	import {
		IconChartCandle,
		IconClock,
		IconUsers,
		IconStar,
		IconCertificate,
		IconTrendingUp,
		IconCheck,
		IconVideo,
		IconFileText,
		IconHeadset,
		IconBolt,
		IconTarget,
		IconChartLine,
		IconShield,
		IconTrophy
	} from '$lib/icons';

	interface Module {
		week: number;
		title: string;
		topics: string[];
		duration: string;
	}

	interface Instructor {
		name: string;
		title: string;
		experience: string;
		achievements: string[];
	}

	const modules: Module[] = [
		{
			week: 1,
			title: 'Day Trading Fundamentals',
			duration: '6 hours',
			topics: [
				'Market structure and mechanics',
				'Understanding price action and volume',
				'Types of day trading strategies',
				'Setting up your trading workspace',
				'Pre-market and post-market analysis'
			]
		},
		{
			week: 2,
			title: 'Chart Reading Mastery',
			duration: '7 hours',
			topics: [
				'Advanced candlestick patterns',
				'Support and resistance levels',
				'Trendlines and channels',
				'Chart patterns (flags, pennants, triangles)',
				'Multiple timeframe analysis'
			]
		},
		{
			week: 3,
			title: 'Technical Indicators & Tools',
			duration: '6 hours',
			topics: [
				'Moving averages and crossovers',
				'VWAP and VWMA strategies',
				'RSI and momentum indicators',
				'Volume analysis techniques',
				'Level 2 and time & sales'
			]
		},
		{
			week: 4,
			title: 'Entry & Exit Strategies',
			duration: '8 hours',
			topics: [
				'Identifying high-probability setups',
				'Precision entry techniques',
				'Stop-loss placement strategies',
				'Profit target calculation',
				'Scaling in and out of positions'
			]
		},
		{
			week: 5,
			title: 'Risk Management Excellence',
			duration: '6 hours',
			topics: [
				'Position sizing formulas',
				'Risk-reward ratio optimization',
				'Maximum daily loss limits',
				'Portfolio heat management',
				'Trading journal best practices'
			]
		},
		{
			week: 6,
			title: 'Advanced Day Trading Strategies',
			duration: '8 hours',
			topics: [
				'Momentum and breakout trading',
				'Gap trading strategies',
				'Reversal and pullback setups',
				'News-based trading',
				'Scalping techniques'
			]
		},
		{
			week: 7,
			title: 'Psychology & Discipline',
			duration: '5 hours',
			topics: [
				'Controlling emotions in trading',
				'Building a trading routine',
				'Overcoming fear and greed',
				'Dealing with losing streaks',
				'Developing mental resilience'
			]
		},
		{
			week: 8,
			title: 'Live Trading & Mastery',
			duration: '10 hours',
			topics: [
				'Live trading sessions with instructors',
				'Real-time market analysis',
				'Trade review and feedback',
				'Building your personalized strategy',
				'Graduation and next steps'
			]
		}
	];

	const instructor: Instructor = {
		name: 'Marcus Chen',
		title: 'Lead Day Trading Instructor',
		experience: '15+ Years Trading Experience',
		achievements: [
			'Former institutional trader at top prop firm',
			'Trained over 5,000 successful day traders',
			'Featured in Trading Magazine and Forbes',
			'Verified 78% win rate over 5 years'
		]
	};

	const features = [
		{
			icon: IconVideo,
			title: '56 hours of video content',
			description: 'Comprehensive video lessons'
		},
		{
			icon: IconFileText,
			title: 'Downloadable resources',
			description: 'Cheat sheets, templates, guides'
		},
		{
			icon: IconHeadset,
			title: 'Live trading sessions',
			description: 'Weekly live market analysis'
		},
		{
			icon: IconCertificate,
			title: 'Certificate of completion',
			description: 'Professional certification'
		},
		{
			icon: IconUsers,
			title: 'Private community access',
			description: 'Connect with fellow traders'
		},
		{ icon: IconBolt, title: 'Lifetime updates', description: 'Free course updates forever' }
	];

	let heroVisible = $state(false);
	let modulesVisible = $state<boolean[]>(new Array(modules.length).fill(false));

	$effect(() => {
		if (!browser) return;

		const heroObserver = new IntersectionObserver(
			(entries) => {
				entries.forEach((entry) => {
					if (entry.isIntersecting) {
						heroVisible = true;
					}
				});
			},
			{ threshold: 0.2 }
		);

		const moduleObserver = new IntersectionObserver(
			(entries) => {
				entries.forEach((entry) => {
					if (entry.isIntersecting) {
						const index = parseInt(entry.target.getAttribute('data-index') || '0');
						modulesVisible[index] = true;
					}
				});
			},
			{ threshold: 0.1 }
		);

		const heroElement = document.querySelector('.hero-section');
		if (heroElement) heroObserver.observe(heroElement);

		const moduleElements = document.querySelectorAll('.module-card');
		moduleElements.forEach((module) => moduleObserver.observe(module));

		return () => {
			heroObserver.disconnect();
			moduleObserver.disconnect();
		};
	});
</script>

<svelte:head>
	<title>Day Trading Masterclass | Revolution Trading</title>
	<meta
		name="description"
		content="Master day trading with institutional-grade strategies, real-time execution tactics, and professional risk management."
	/>
	<meta property="og:title" content="Day Trading Masterclass | Revolution Trading" />
	<meta
		property="og:description"
		content="Master day trading with institutional-grade strategies, real-time execution tactics, and professional risk management."
	/>
	<meta property="og:type" content="website" />
	<meta name="twitter:card" content="summary_large_image" />
	<meta name="twitter:title" content="Day Trading Masterclass | Revolution Trading" />
	<meta
		name="twitter:description"
		content="Master day trading with institutional-grade strategies, real-time execution tactics, and professional risk management."
	/>
</svelte:head>

<div class="course-page">
	<!-- Hero Section -->
	<section class="hero-section" class:visible={heroVisible}>
		<div class="hero-background">
			<div class="glow-orb glow-orb-1"></div>
			<div class="glow-orb glow-orb-2"></div>
			<div class="chart-pattern"></div>
		</div>

		<div class="hero-content">
			<div class="hero-badge">
				<IconChartCandle size={20} stroke={2} />
				<span>Professional Day Trading Course</span>
			</div>

			<h1 class="hero-title">
				Day Trading<br />
				<span class="gradient-text">Masterclass</span>
			</h1>

			<p class="hero-description">
				Transform into a consistently profitable day trader with institutional-grade strategies,
				real-time market analysis, and professional risk management techniques.
			</p>

			<div class="hero-meta">
				<div class="meta-item">
					<IconClock size={24} stroke={2} />
					<div>
						<div class="meta-label">Duration</div>
						<div class="meta-value">8 Weeks</div>
					</div>
				</div>
				<div class="meta-item">
					<IconUsers size={24} stroke={2} />
					<div>
						<div class="meta-label">Students</div>
						<div class="meta-value">2,847</div>
					</div>
				</div>
				<div class="meta-item">
					<IconStar size={24} stroke={2} />
					<div>
						<div class="meta-label">Rating</div>
						<div class="meta-value">4.9/5.0</div>
					</div>
				</div>
				<div class="meta-item">
					<IconTrendingUp size={24} stroke={2} />
					<div>
						<div class="meta-label">Level</div>
						<div class="meta-value">Intermediate</div>
					</div>
				</div>
			</div>

			<div class="hero-cta">
				<a href="/register" class="cta-button primary">
					Enroll Now - $497
					<IconBolt size={20} stroke={2} />
				</a>
				<a href="#curriculum" class="cta-button secondary"> View Curriculum </a>
			</div>
		</div>
	</section>

	<!-- What You'll Learn -->
	<section class="learning-section">
		<div class="section-container">
			<h2 class="section-title">What You'll Master</h2>
			<p class="section-description">
				This comprehensive program covers everything you need to become a professional day trader
			</p>

			<div class="learning-grid">
				<div class="learning-card">
					<div class="learning-icon">
						<IconTarget size={32} stroke={1.5} />
					</div>
					<h3>High-Probability Setups</h3>
					<p>
						Identify and execute trades with institutional-grade precision using proven chart
						patterns and technical analysis.
					</p>
				</div>

				<div class="learning-card">
					<div class="learning-icon">
						<IconChartLine size={32} stroke={1.5} />
					</div>
					<h3>Real-Time Execution</h3>
					<p>
						Master the art of timing your entries and exits in fast-moving markets with confidence
						and discipline.
					</p>
				</div>

				<div class="learning-card">
					<div class="learning-icon">
						<IconShield size={32} stroke={1.5} />
					</div>
					<h3>Professional Risk Management</h3>
					<p>
						Protect your capital with advanced position sizing, stop-loss strategies, and portfolio
						heat management.
					</p>
				</div>

				<div class="learning-card">
					<div class="learning-icon">
						<IconTrophy size={32} stroke={1.5} />
					</div>
					<h3>Consistent Profitability</h3>
					<p>
						Build a sustainable trading career with proven strategies that generate consistent
						returns over time.
					</p>
				</div>
			</div>
		</div>
	</section>

	<!-- Curriculum -->
	<section class="curriculum-section" id="curriculum">
		<div class="section-container">
			<h2 class="section-title">Course Curriculum</h2>
			<p class="section-description">
				8 comprehensive weeks designed to transform you into a professional day trader
			</p>

			<div class="modules-grid">
				{#each modules as module, index}
					<div
						class="module-card"
						class:visible={modulesVisible[index]}
						data-index={index}
						style="--delay: {index * 0.1}s"
					>
						<div class="module-header">
							<div class="module-number">Week {module.week}</div>
							<div class="module-duration">
								<IconClock size={16} stroke={2} />
								<span>{module.duration}</span>
							</div>
						</div>

						<h3 class="module-title">{module.title}</h3>

						<ul class="module-topics">
							{#each module.topics as topic}
								<li>
									<IconCheck size={18} stroke={2} />
									<span>{topic}</span>
								</li>
							{/each}
						</ul>
					</div>
				{/each}
			</div>
		</div>
	</section>

	<!-- Course Features -->
	<section class="features-section">
		<div class="section-container">
			<h2 class="section-title">What's Included</h2>

			<div class="features-grid">
				{#each features as feature}
					{@const FeatureIcon = feature.icon}
					<div class="feature-card">
						<div class="feature-icon">
							<FeatureIcon size={28} stroke={1.5} />
						</div>
						<div class="feature-content">
							<h4>{feature.title}</h4>
							<p>{feature.description}</p>
						</div>
					</div>
				{/each}
			</div>
		</div>
	</section>

	<!-- Instructor -->
	<section class="instructor-section">
		<div class="section-container">
			<h2 class="section-title">Your Instructor</h2>

			<div class="instructor-card">
				<div class="instructor-avatar">
					<IconChartCandle size={64} stroke={1.5} />
				</div>

				<div class="instructor-content">
					<h3 class="instructor-name">{instructor.name}</h3>
					<p class="instructor-title">{instructor.title}</p>
					<p class="instructor-experience">{instructor.experience}</p>

					<ul class="instructor-achievements">
						{#each instructor.achievements as achievement}
							<li>
								<IconTrophy size={18} stroke={2} />
								<span>{achievement}</span>
							</li>
						{/each}
					</ul>
				</div>
			</div>
		</div>
	</section>

	<!-- Enrollment CTA -->
	<section class="enrollment-section">
		<div class="enrollment-card">
			<div class="enrollment-content">
				<h2>Ready to Master Day Trading?</h2>
				<p>Join 2,847 students already transforming their trading careers</p>

				<div class="enrollment-features">
					<div class="enrollment-feature">
						<IconCheck size={20} stroke={2} />
						<span>56 hours of premium content</span>
					</div>
					<div class="enrollment-feature">
						<IconCheck size={20} stroke={2} />
						<span>Lifetime access & updates</span>
					</div>
					<div class="enrollment-feature">
						<IconCheck size={20} stroke={2} />
						<span>Private community access</span>
					</div>
					<div class="enrollment-feature">
						<IconCheck size={20} stroke={2} />
						<span>Certificate of completion</span>
					</div>
				</div>

				<div class="enrollment-pricing">
					<div class="price-container">
						<span class="price-label">One-time payment</span>
						<span class="price-value">$497</span>
					</div>

					<a href="/register" class="enrollment-button">
						Enroll Now
						<IconBolt size={20} stroke={2} />
					</a>
				</div>

				<p class="enrollment-guarantee">30-day money-back guarantee</p>
			</div>
		</div>
	</section>
</div>

<style>
	.course-page {
		background: linear-gradient(to bottom, #0a0f1e 0%, #050812 100%);
		color: white;
	}

	/* Hero Section */
	.hero-section {
		position: relative;
		min-height: 85vh;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 8rem 2rem 4rem;
		overflow: hidden;
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
		z-index: 0;
	}

	.glow-orb {
		position: absolute;
		border-radius: 50%;
		filter: blur(80px);
		opacity: 0.3;
		animation: pulse 4s ease-in-out infinite;
	}

	.glow-orb-1 {
		width: 600px;
		height: 600px;
		background: radial-gradient(circle, #2e8eff 0%, transparent 70%);
		top: -200px;
		right: -200px;
	}

	.glow-orb-2 {
		width: 500px;
		height: 500px;
		background: radial-gradient(circle, #34d399 0%, transparent 70%);
		bottom: -150px;
		left: -150px;
		animation-delay: 2s;
	}

	.chart-pattern {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		width: 800px;
		height: 400px;
		background:
			linear-gradient(90deg, transparent 49%, rgba(46, 142, 255, 0.1) 50%, transparent 51%),
			linear-gradient(0deg, transparent 49%, rgba(46, 142, 255, 0.1) 50%, transparent 51%);
		background-size: 40px 40px;
		opacity: 0.2;
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
		background: rgba(46, 142, 255, 0.15);
		border: 1px solid rgba(46, 142, 255, 0.3);
		border-radius: 50px;
		font-size: 0.875rem;
		font-weight: 500;
		color: #2e8eff;
		margin-bottom: 2rem;
		backdrop-filter: blur(10px);
	}

	.hero-title {
		font-size: clamp(2.5rem, 5vw, 4.5rem);
		font-weight: 800;
		line-height: 1.1;
		margin-bottom: 1.5rem;
		letter-spacing: -0.02em;
	}

	.gradient-text {
		background: linear-gradient(135deg, #2e8eff 0%, #34d399 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}

	.hero-description {
		font-size: clamp(1rem, 2vw, 1.25rem);
		line-height: 1.7;
		color: rgba(255, 255, 255, 0.7);
		max-width: 700px;
		margin: 0 auto 3rem;
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
		color: #2e8eff;
	}

	.meta-label {
		font-size: 0.75rem;
		color: rgba(255, 255, 255, 0.5);
	}

	.meta-value {
		font-size: 1rem;
		font-weight: 700;
		color: white;
	}

	.hero-cta {
		display: flex;
		gap: 1rem;
		justify-content: center;
		flex-wrap: wrap;
	}

	.cta-button {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 1rem 2rem;
		border-radius: 12px;
		font-weight: 600;
		font-size: 1rem;
		text-decoration: none;
		transition: all 0.3s ease;
		border: 2px solid transparent;
	}

	.cta-button.primary {
		background: linear-gradient(135deg, #2e8eff 0%, #1e5cb8 100%);
		color: white;
	}

	.cta-button.primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 10px 30px rgba(46, 142, 255, 0.4);
	}

	.cta-button.secondary {
		background: rgba(255, 255, 255, 0.05);
		color: white;
		border-color: rgba(255, 255, 255, 0.1);
		backdrop-filter: blur(10px);
	}

	.cta-button.secondary:hover {
		background: rgba(255, 255, 255, 0.1);
		border-color: rgba(255, 255, 255, 0.2);
	}

	/* Section Container */
	.section-container {
		max-width: 1200px;
		margin: 0 auto;
		padding: 0 2rem;
	}

	.section-title {
		font-size: clamp(2rem, 4vw, 3rem);
		font-weight: 800;
		text-align: center;
		margin-bottom: 1rem;
	}

	.section-description {
		font-size: 1.125rem;
		color: rgba(255, 255, 255, 0.7);
		text-align: center;
		max-width: 700px;
		margin: 0 auto 4rem;
	}

	/* Learning Section */
	.learning-section {
		padding: 6rem 0;
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
		background: rgba(255, 255, 255, 0.05);
		border-color: rgba(46, 142, 255, 0.3);
		transform: translateY(-4px);
	}

	.learning-icon {
		width: 64px;
		height: 64px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, #2e8eff 0%, #1e5cb8 100%);
		border-radius: 12px;
		margin-bottom: 1.5rem;
		color: white;
	}

	.learning-card h3 {
		font-size: 1.25rem;
		font-weight: 700;
		margin-bottom: 0.75rem;
	}

	.learning-card p {
		font-size: 0.9375rem;
		line-height: 1.6;
		color: rgba(255, 255, 255, 0.7);
	}

	/* Curriculum Section */
	.curriculum-section {
		padding: 6rem 0;
		background: rgba(255, 255, 255, 0.02);
	}

	.modules-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
		gap: 2rem;
	}

	.module-card {
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 16px;
		padding: 2rem;
		opacity: 0;
		transform: translateY(30px);
		transition: all 0.4s ease;
	}

	.module-card.visible {
		opacity: 1;
		transform: translateY(0);
		transition-delay: var(--delay);
	}

	.module-card:hover {
		border-color: rgba(46, 142, 255, 0.3);
		background: rgba(255, 255, 255, 0.05);
	}

	.module-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
	}

	.module-number {
		padding: 0.5rem 1rem;
		background: linear-gradient(135deg, #2e8eff 0%, #1e5cb8 100%);
		border-radius: 8px;
		font-size: 0.875rem;
		font-weight: 600;
	}

	.module-duration {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.875rem;
		color: rgba(255, 255, 255, 0.6);
	}

	.module-duration :global(svg) {
		color: #2e8eff;
	}

	.module-title {
		font-size: 1.5rem;
		font-weight: 700;
		margin-bottom: 1.5rem;
	}

	.module-topics {
		list-style: none;
		padding: 0;
		margin: 0;
	}

	.module-topics li {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
		padding: 0.75rem 0;
		font-size: 0.9375rem;
		color: rgba(255, 255, 255, 0.8);
		border-bottom: 1px solid rgba(255, 255, 255, 0.05);
	}

	.module-topics li:last-child {
		border-bottom: none;
	}

	.module-topics li :global(svg) {
		color: #34d399;
		flex-shrink: 0;
		margin-top: 2px;
	}

	/* Features Section */
	.features-section {
		padding: 6rem 0;
	}

	.features-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
		gap: 2rem;
	}

	.feature-card {
		display: flex;
		gap: 1.5rem;
		padding: 2rem;
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 16px;
		transition: all 0.3s ease;
	}

	.feature-card:hover {
		background: rgba(255, 255, 255, 0.05);
		border-color: rgba(46, 142, 255, 0.3);
		transform: translateY(-4px);
	}

	.feature-icon {
		width: 56px;
		height: 56px;
		flex-shrink: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, #2e8eff 0%, #1e5cb8 100%);
		border-radius: 12px;
		color: white;
	}

	.feature-content h4 {
		font-size: 1.125rem;
		font-weight: 700;
		margin-bottom: 0.5rem;
	}

	.feature-content p {
		font-size: 0.9375rem;
		color: rgba(255, 255, 255, 0.7);
	}

	/* Instructor Section */
	.instructor-section {
		padding: 6rem 0;
		background: rgba(255, 255, 255, 0.02);
	}

	.instructor-card {
		display: flex;
		gap: 3rem;
		max-width: 900px;
		margin: 0 auto;
		padding: 3rem;
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 20px;
	}

	.instructor-avatar {
		width: 180px;
		height: 180px;
		flex-shrink: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, #2e8eff 0%, #1e5cb8 100%);
		border-radius: 16px;
		color: white;
	}

	.instructor-name {
		font-size: 2rem;
		font-weight: 700;
		margin-bottom: 0.5rem;
	}

	.instructor-title {
		font-size: 1.125rem;
		color: #2e8eff;
		margin-bottom: 0.5rem;
	}

	.instructor-experience {
		font-size: 1rem;
		color: rgba(255, 255, 255, 0.7);
		margin-bottom: 1.5rem;
	}

	.instructor-achievements {
		list-style: none;
		padding: 0;
		margin: 0;
	}

	.instructor-achievements li {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem 0;
		font-size: 0.9375rem;
		color: rgba(255, 255, 255, 0.8);
	}

	.instructor-achievements li :global(svg) {
		color: #f59e0b;
		flex-shrink: 0;
	}

	/* Enrollment Section */
	.enrollment-section {
		padding: 6rem 2rem;
	}

	.enrollment-card {
		max-width: 800px;
		margin: 0 auto;
		padding: 4rem;
		background: linear-gradient(135deg, rgba(46, 142, 255, 0.1) 0%, rgba(52, 211, 153, 0.1) 100%);
		border: 2px solid rgba(46, 142, 255, 0.3);
		border-radius: 24px;
		text-align: center;
		backdrop-filter: blur(20px);
	}

	.enrollment-content h2 {
		font-size: clamp(2rem, 4vw, 2.5rem);
		font-weight: 800;
		margin-bottom: 1rem;
	}

	.enrollment-content > p {
		font-size: 1.125rem;
		color: rgba(255, 255, 255, 0.7);
		margin-bottom: 3rem;
	}

	.enrollment-features {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
		gap: 1rem;
		margin-bottom: 3rem;
	}

	.enrollment-feature {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem;
		background: rgba(255, 255, 255, 0.05);
		border-radius: 12px;
		font-size: 0.9375rem;
	}

	.enrollment-feature :global(svg) {
		color: #34d399;
		flex-shrink: 0;
	}

	.enrollment-pricing {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 2rem;
		margin-bottom: 2rem;
		flex-wrap: wrap;
	}

	.price-container {
		display: flex;
		flex-direction: column;
		align-items: flex-start;
	}

	.price-label {
		font-size: 0.875rem;
		color: rgba(255, 255, 255, 0.6);
		margin-bottom: 0.25rem;
	}

	.price-value {
		font-size: 3rem;
		font-weight: 800;
		background: linear-gradient(135deg, #2e8eff 0%, #34d399 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}

	.enrollment-button {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 1.25rem 2.5rem;
		background: linear-gradient(135deg, #2e8eff 0%, #1e5cb8 100%);
		color: white;
		text-decoration: none;
		border-radius: 12px;
		font-weight: 700;
		font-size: 1.125rem;
		transition: all 0.3s ease;
	}

	.enrollment-button:hover {
		transform: translateY(-2px);
		box-shadow: 0 15px 40px rgba(46, 142, 255, 0.5);
	}

	.enrollment-guarantee {
		font-size: 0.875rem;
		color: rgba(255, 255, 255, 0.6);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * 2026 MOBILE-FIRST RESPONSIVE DESIGN
	 * Breakpoints: xs(360px), sm(640px), md(768px), lg(1024px), xl(1280px)
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* DYNAMIC VIEWPORT HEIGHT */
	@supports (min-height: 100dvh) {
		.hero-section {
			min-height: 85dvh;
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
		.cta-button,
		.enrollment-button {
			min-height: 44px;
		}

		.module-topics li {
			min-height: 44px;
			display: flex;
			align-items: center;
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
		}

		.hero-meta {
			flex-direction: column;
			gap: 0.75rem;
		}

		.meta-item {
			justify-content: center;
		}

		.hero-cta {
			flex-direction: column;
			gap: 0.75rem;
		}

		.cta-button {
			width: 100%;
			justify-content: center;
			padding: 0.875rem 1.5rem;
		}

		.section-container {
			padding: 0 0.75rem;
		}

		.section-title {
			font-size: 1.5rem;
		}

		.section-description {
			font-size: 0.9375rem;
		}

		.learning-section,
		.curriculum-section,
		.features-section,
		.instructor-section {
			padding: 4rem 0;
		}

		.learning-card,
		.module-card {
			padding: 1.25rem;
		}

		.module-title {
			font-size: 1.25rem;
		}

		.enrollment-card {
			padding: 1.5rem;
		}

		.enrollment-content h2 {
			font-size: 1.5rem;
		}

		.price-value {
			font-size: 2.25rem;
		}

		.glow-orb-1,
		.glow-orb-2 {
			width: 200px;
			height: 200px;
		}

		.chart-pattern {
			width: 280px;
			height: 140px;
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

		.hero-cta {
			flex-direction: column;
			gap: 0.75rem;
		}

		.cta-button {
			width: 100%;
			justify-content: center;
		}

		.section-container {
			padding: 0 1rem;
		}

		.learning-grid,
		.features-grid,
		.modules-grid,
		.enrollment-features {
			grid-template-columns: 1fr;
		}

		.feature-card {
			flex-direction: column;
			text-align: center;
		}

		.feature-icon {
			margin: 0 auto;
		}

		.instructor-card {
			flex-direction: column;
			text-align: center;
			padding: 1.5rem;
		}

		.instructor-avatar {
			margin: 0 auto;
			width: 120px;
			height: 120px;
		}

		.enrollment-pricing {
			flex-direction: column;
			gap: 1.5rem;
		}

		.price-container {
			align-items: center;
		}

		.enrollment-button {
			width: 100%;
			justify-content: center;
		}

		.glow-orb-1 {
			width: 280px;
			height: 280px;
		}

		.glow-orb-2 {
			width: 220px;
			height: 220px;
		}

		.chart-pattern {
			width: 320px;
			height: 160px;
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
			gap: 1.5rem;
		}

		.learning-grid {
			grid-template-columns: repeat(2, 1fr);
		}

		.modules-grid,
		.features-grid {
			grid-template-columns: 1fr;
		}

		.instructor-card {
			flex-direction: column;
			text-align: center;
			padding: 2rem;
		}

		.instructor-avatar {
			margin: 0 auto;
		}
	}

	/* MEDIUM DEVICES (768px - 1023px) */
	@media (min-width: 768px) and (max-width: 1023px) {
		.modules-grid {
			grid-template-columns: 1fr;
		}

		.learning-grid,
		.features-grid {
			grid-template-columns: repeat(2, 1fr);
		}

		.glow-orb-1 {
			width: 400px;
			height: 400px;
		}

		.glow-orb-2 {
			width: 350px;
			height: 350px;
		}
	}

	/* LARGE DEVICES (1024px+) */
	@media (min-width: 1024px) {
		.modules-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	/* LANDSCAPE MOBILE */
	@media (max-height: 500px) and (orientation: landscape) {
		.hero-section {
			min-height: auto;
			padding: 4rem 2rem 3rem;
		}

		.learning-section,
		.curriculum-section,
		.features-section,
		.instructor-section {
			padding: 3rem 0;
		}

		.section-description {
			margin-bottom: 2rem;
		}
	}

	/* REDUCED MOTION */
	@media (prefers-reduced-motion: reduce) {
		.hero-section,
		.module-card {
			transition: none;
			transform: none;
			opacity: 1;
		}

		.glow-orb {
			animation: none;
		}

		.cta-button:hover,
		.enrollment-button:hover,
		.learning-card:hover,
		.module-card:hover,
		.feature-card:hover {
			transform: none;
		}
	}
</style>

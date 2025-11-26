<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import {
		IconSchool,
		IconTrendingUp,
		IconChartCandle,
		IconClock,
		IconUsers,
		IconCertificate,
		IconChartLine,
		IconBrain,
		IconShield,
		IconRocket,
		IconStar
	} from '@tabler/icons-svelte';
	import SEOHead from '$lib/components/SEOHead.svelte';

	interface Course {
		id: string;
		title: string;
		slug: string;
		description: string;
		level: string;
		duration: string;
		students: string;
		rating: number;
		price: string;
		icon: any;
		features: string[];
		gradient: string;
	}

	const courses: Course[] = [
		{
			id: '1',
			title: 'Day Trading Masterclass',
			slug: 'day-trading-masterclass',
			description:
				'Master the art of day trading with institutional-grade strategies, real-time execution tactics, and professional risk management.',
			level: 'Intermediate to Advanced',
			duration: '8 Weeks',
			students: '2,847',
			rating: 4.9,
			price: '$497',
			icon: IconChartCandle,
			features: [
				'Live trading sessions with pro traders',
				'Real-time market analysis and execution',
				'Advanced chart reading and pattern recognition',
				'Risk management and position sizing',
				'Psychology and discipline training'
			],
			gradient: 'linear-gradient(135deg, #2e8eff 0%, #1e5cb8 100%)'
		},
		{
			id: '2',
			title: 'Swing Trading Pro',
			slug: 'swing-trading-pro',
			description:
				'Learn professional swing trading strategies to capture multi-day moves with precision entries and optimal risk-reward setups.',
			level: 'Beginner to Intermediate',
			duration: '6 Weeks',
			students: '3,421',
			rating: 4.8,
			price: '$397',
			icon: IconChartLine,
			features: [
				'Weekly market analysis and trade setups',
				'Trend identification and momentum trading',
				'Support and resistance mastery',
				'Trade management and exit strategies',
				'Swing trading psychology'
			],
			gradient: 'linear-gradient(135deg, #34d399 0%, #059669 100%)'
		},
		{
			id: '3',
			title: 'Options Trading Fundamentals',
			slug: 'options-trading',
			description:
				'Unlock the power of options trading with comprehensive training on strategies, Greeks, volatility analysis, and risk management.',
			level: 'Intermediate',
			duration: '10 Weeks',
			students: '1,892',
			rating: 4.9,
			price: '$597',
			icon: IconBrain,
			features: [
				'Options fundamentals and terminology',
				'Understanding the Greeks (Delta, Gamma, Theta, Vega)',
				'Popular strategies (spreads, straddles, iron condors)',
				'Volatility analysis and implied volatility',
				'Advanced risk management for options'
			],
			gradient: 'linear-gradient(135deg, #a78bfa 0%, #7c3aed 100%)'
		},
		{
			id: '4',
			title: 'Risk Management Mastery',
			slug: 'risk-management',
			description:
				'Professional risk management techniques used by institutional traders to preserve capital and maximize long-term profitability.',
			level: 'All Levels',
			duration: '4 Weeks',
			students: '4,156',
			rating: 5.0,
			price: '$297',
			icon: IconShield,
			features: [
				'Position sizing and portfolio allocation',
				'Stop-loss strategies and trade management',
				'Maximum drawdown prevention',
				'Kelly Criterion and expectancy calculations',
				'Building a professional trading plan'
			],
			gradient: 'linear-gradient(135deg, #fb923c 0%, #ea580c 100%)'
		}
	];

	let heroVisible = false;
	let cardsVisible: boolean[] = new Array(courses.length).fill(false);

	// Course listing structured data for rich snippets
	const coursesSchema = [
		{
			'@context': 'https://schema.org',
			'@type': 'ItemList',
			'@id': 'https://revolutiontradingpros.com/courses/#courselist',
			name: 'Professional Trading Courses',
			description: 'Comprehensive trading education courses from Revolution Trading Pros',
			numberOfItems: courses.length,
			itemListElement: courses.map((course, index) => ({
				'@type': 'ListItem',
				position: index + 1,
				item: {
					'@type': 'Course',
					name: course.title,
					description: course.description,
					url: `https://revolutiontradingpros.com/courses/${course.slug}`,
					provider: {
						'@type': 'Organization',
						name: 'Revolution Trading Pros',
						url: 'https://revolutiontradingpros.com'
					},
					educationalLevel: course.level,
					timeRequired: course.duration,
					offers: {
						'@type': 'Offer',
						price: course.price.replace('$', ''),
						priceCurrency: 'USD',
						availability: 'https://schema.org/InStock'
					},
					aggregateRating: {
						'@type': 'AggregateRating',
						ratingValue: course.rating,
						reviewCount: parseInt(course.students.replace(',', ''))
					}
				}
			}))
		}
	];

	onMount(() => {
		if (!browser) return;

		// Hero animation
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

		const heroElement = document.querySelector('.hero-section');
		if (heroElement) heroObserver.observe(heroElement);

		// Card animations
		const cardObserver = new IntersectionObserver(
			(entries) => {
				entries.forEach((entry) => {
					if (entry.isIntersecting) {
						const index = parseInt(entry.target.getAttribute('data-index') || '0');
						cardsVisible[index] = true;
					}
				});
			},
			{ threshold: 0.1 }
		);

		const cardElements = document.querySelectorAll('.course-card');
		cardElements.forEach((card) => cardObserver.observe(card));

		return () => {
			heroObserver.disconnect();
			cardObserver.disconnect();
		};
	});
</script>

<SEOHead
	title="Professional Trading Courses"
	description="Master trading with our professional courses. Day trading, swing trading, options, and risk management from industry experts. 10,000+ students, 4.9 average rating."
	canonical="/courses"
	ogType="website"
	ogImage="/og-image.webp"
	ogImageAlt="Revolution Trading Pros Professional Trading Courses"
	keywords={[
		'trading courses',
		'day trading course',
		'swing trading course',
		'options trading course',
		'risk management course',
		'trading education',
		'learn to trade',
		'professional trading',
		'trading masterclass'
	]}
	schema={coursesSchema}
	schemaType="Course"
/>

<div class="courses-page">
	<!-- Hero Section -->
	<section class="hero-section" class:visible={heroVisible}>
		<div class="hero-background">
			<div class="glow-orb glow-orb-1"></div>
			<div class="glow-orb glow-orb-2"></div>
		</div>

		<div class="hero-content">
			<div class="hero-badge">
				<IconCertificate size={20} stroke={2} />
				<span>Professional Trading Education</span>
			</div>

			<h1 class="hero-title">
				Master Trading With<br />
				<span class="gradient-text">Institutional-Grade Courses</span>
			</h1>

			<p class="hero-description">
				Learn from professional traders with proven track records. Our comprehensive courses combine
				institutional strategies, real-time analysis, and hands-on training to transform you into a
				consistently profitable trader.
			</p>

			<div class="hero-stats">
				<div class="stat-item">
					<IconUsers size={32} stroke={1.5} class="stat-icon" />
					<div class="stat-content">
						<div class="stat-value">10,000+</div>
						<div class="stat-label">Active Students</div>
					</div>
				</div>
				<div class="stat-item">
					<IconStar size={32} stroke={1.5} class="stat-icon" />
					<div class="stat-content">
						<div class="stat-value">4.9/5.0</div>
						<div class="stat-label">Average Rating</div>
					</div>
				</div>
				<div class="stat-item">
					<IconRocket size={32} stroke={1.5} class="stat-icon" />
					<div class="stat-content">
						<div class="stat-value">85%</div>
						<div class="stat-label">Success Rate</div>
					</div>
				</div>
			</div>

			<div class="hero-cta">
				<a href="#courses" class="cta-button primary">
					Explore Courses
					<IconSchool size={20} stroke={2} />
				</a>
				<a href="/about" class="cta-button secondary"> Meet Our Instructors </a>
			</div>
		</div>
	</section>

	<!-- Courses Grid -->
	<section class="courses-section" id="courses">
		<div class="section-header">
			<h2 class="section-title">Our Professional Courses</h2>
			<p class="section-description">
				Choose from our comprehensive curriculum designed to take you from beginner to professional
				trader
			</p>
		</div>

		<div class="courses-grid">
			{#each courses as course, index}
				<article
					class="course-card"
					class:visible={cardsVisible[index]}
					data-index={index}
					style="--delay: {index * 0.1}s"
				>
					<div class="card-header" style="background: {course.gradient}">
						<div class="card-icon">
							<svelte:component this={course.icon} size={48} stroke={1.5} />
						</div>
						<div class="card-badge">{course.level}</div>
					</div>

					<div class="card-content">
						<h3 class="card-title">{course.title}</h3>
						<p class="card-description">{course.description}</p>

						<div class="card-meta">
							<div class="meta-item">
								<IconClock size={18} stroke={2} />
								<span>{course.duration}</span>
							</div>
							<div class="meta-item">
								<IconUsers size={18} stroke={2} />
								<span>{course.students} students</span>
							</div>
							<div class="meta-item">
								<IconStar size={18} stroke={2} />
								<span>{course.rating}/5.0</span>
							</div>
						</div>

						<ul class="card-features">
							{#each course.features as feature}
								<li>
									<IconTrendingUp size={16} stroke={2} />
									<span>{feature}</span>
								</li>
							{/each}
						</ul>

						<div class="card-footer">
							<div class="card-price">
								<span class="price-label">Course Price</span>
								<span class="price-value">{course.price}</span>
							</div>
							<a href="/courses/{course.slug}" class="card-button">
								View Course
								<IconSchool size={18} stroke={2} />
							</a>
						</div>
					</div>
				</article>
			{/each}
		</div>
	</section>

	<!-- Why Choose Section -->
	<section class="why-choose-section">
		<div class="why-choose-content">
			<h2 class="section-title">Why Choose Revolution Trading Academy</h2>

			<div class="benefits-grid">
				<div class="benefit-card">
					<div class="benefit-icon">
						<IconChartCandle size={32} stroke={1.5} />
					</div>
					<h3>Real-World Strategies</h3>
					<p>
						Learn strategies actively used by professional traders in live markets, not theoretical
						concepts.
					</p>
				</div>

				<div class="benefit-card">
					<div class="benefit-icon">
						<IconUsers size={32} stroke={1.5} />
					</div>
					<h3>Expert Instructors</h3>
					<p>
						Learn from traders with verified track records and decades of combined market
						experience.
					</p>
				</div>

				<div class="benefit-card">
					<div class="benefit-icon">
						<IconBrain size={32} stroke={1.5} />
					</div>
					<h3>Hands-On Training</h3>
					<p>Practice with real market data, live trading sessions, and personalized feedback.</p>
				</div>

				<div class="benefit-card">
					<div class="benefit-icon">
						<IconCertificate size={32} stroke={1.5} />
					</div>
					<h3>Lifetime Access</h3>
					<p>
						Get unlimited access to all course materials, updates, and our exclusive trading
						community.
					</p>
				</div>
			</div>
		</div>
	</section>
</div>

<style>
	.courses-page {
		min-height: 100vh;
		background: linear-gradient(to bottom, #0a0f1e 0%, #050812 100%);
		color: white;
	}

	/* Hero Section */
	.hero-section {
		position: relative;
		min-height: 90vh;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 8rem 2rem 6rem;
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
		max-width: 1000px;
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
		max-width: 800px;
		margin: 0 auto 3rem;
	}

	.hero-stats {
		display: flex;
		justify-content: center;
		gap: 3rem;
		margin-bottom: 3rem;
		flex-wrap: wrap;
	}

	.stat-item {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.stat-content {
		text-align: left;
	}

	.stat-value {
		font-size: 1.5rem;
		font-weight: 700;
		color: white;
	}

	.stat-label {
		font-size: 0.875rem;
		color: rgba(255, 255, 255, 0.6);
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

	/* Courses Section */
	.courses-section {
		padding: 6rem 2rem;
		max-width: 1400px;
		margin: 0 auto;
	}

	.section-header {
		text-align: center;
		margin-bottom: 4rem;
	}

	.section-title {
		font-size: clamp(2rem, 4vw, 3rem);
		font-weight: 800;
		margin-bottom: 1rem;
	}

	.section-description {
		font-size: 1.125rem;
		color: rgba(255, 255, 255, 0.7);
		max-width: 700px;
		margin: 0 auto;
	}

	.courses-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
		gap: 2rem;
	}

	.course-card {
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 20px;
		overflow: hidden;
		transition: all 0.4s ease;
		opacity: 0;
		transform: translateY(30px);
	}

	.course-card.visible {
		opacity: 1;
		transform: translateY(0);
		transition-delay: var(--delay);
	}

	.course-card:hover {
		transform: translateY(-8px);
		border-color: rgba(46, 142, 255, 0.3);
		box-shadow: 0 20px 50px rgba(46, 142, 255, 0.2);
	}

	.card-header {
		position: relative;
		padding: 3rem 2rem;
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.card-icon {
		color: white;
		filter: drop-shadow(0 0 10px rgba(255, 255, 255, 0.3));
	}

	.card-badge {
		position: absolute;
		top: 1rem;
		right: 1rem;
		padding: 0.4rem 1rem;
		background: rgba(0, 0, 0, 0.3);
		backdrop-filter: blur(10px);
		border-radius: 50px;
		font-size: 0.75rem;
		font-weight: 600;
		color: white;
	}

	.card-content {
		padding: 2rem;
	}

	.card-title {
		font-size: 1.5rem;
		font-weight: 700;
		margin-bottom: 1rem;
	}

	.card-description {
		font-size: 0.9375rem;
		line-height: 1.6;
		color: rgba(255, 255, 255, 0.7);
		margin-bottom: 1.5rem;
	}

	.card-meta {
		display: flex;
		gap: 1.5rem;
		margin-bottom: 1.5rem;
		flex-wrap: wrap;
	}

	.meta-item {
		display: flex;
		align-items: center;
		gap: 0.4rem;
		font-size: 0.875rem;
		color: rgba(255, 255, 255, 0.6);
	}

	.meta-item :global(svg) {
		color: #2e8eff;
	}

	.card-features {
		list-style: none;
		padding: 0;
		margin: 0 0 2rem 0;
	}

	.card-features li {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
		padding: 0.75rem 0;
		font-size: 0.875rem;
		color: rgba(255, 255, 255, 0.8);
		border-bottom: 1px solid rgba(255, 255, 255, 0.05);
	}

	.card-features li:last-child {
		border-bottom: none;
	}

	.card-features li :global(svg) {
		color: #34d399;
		flex-shrink: 0;
		margin-top: 2px;
	}

	.card-footer {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding-top: 1.5rem;
		border-top: 1px solid rgba(255, 255, 255, 0.1);
	}

	.card-price {
		display: flex;
		flex-direction: column;
	}

	.price-label {
		font-size: 0.75rem;
		color: rgba(255, 255, 255, 0.5);
		margin-bottom: 0.25rem;
	}

	.price-value {
		font-size: 1.75rem;
		font-weight: 700;
		background: linear-gradient(135deg, #2e8eff 0%, #34d399 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}

	.card-button {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.875rem 1.5rem;
		background: linear-gradient(135deg, #2e8eff 0%, #1e5cb8 100%);
		color: white;
		text-decoration: none;
		border-radius: 10px;
		font-weight: 600;
		font-size: 0.9375rem;
		transition: all 0.3s ease;
	}

	.card-button:hover {
		transform: translateX(4px);
		box-shadow: 0 5px 20px rgba(46, 142, 255, 0.4);
	}

	/* Why Choose Section */
	.why-choose-section {
		padding: 6rem 2rem;
		background: rgba(255, 255, 255, 0.02);
		border-top: 1px solid rgba(255, 255, 255, 0.05);
	}

	.why-choose-content {
		max-width: 1200px;
		margin: 0 auto;
	}

	.benefits-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
		gap: 2rem;
		margin-top: 3rem;
	}

	.benefit-card {
		padding: 2rem;
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 16px;
		transition: all 0.3s ease;
	}

	.benefit-card:hover {
		background: rgba(255, 255, 255, 0.05);
		border-color: rgba(46, 142, 255, 0.3);
		transform: translateY(-4px);
	}

	.benefit-icon {
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

	.benefit-card h3 {
		font-size: 1.25rem;
		font-weight: 700;
		margin-bottom: 0.75rem;
	}

	.benefit-card p {
		font-size: 0.9375rem;
		line-height: 1.6;
		color: rgba(255, 255, 255, 0.7);
	}

	/* Responsive */
	@media (max-width: 1200px) {
		.glow-orb-1 {
			width: 500px;
			height: 500px;
		}

		.glow-orb-2 {
			width: 450px;
			height: 450px;
		}
	}

	@media (max-width: 1024px) {
		.courses-grid {
			grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
		}

		.hero-stats {
			gap: 2rem;
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

	@media (max-width: 768px) {
		.hero-section {
			padding: 6rem 1.5rem 4rem;
			min-height: auto;
		}

		.courses-grid {
			grid-template-columns: 1fr;
		}

		.hero-stats {
			flex-direction: column;
			gap: 1.5rem;
		}

		.stat-item {
			justify-content: center;
		}

		.hero-cta {
			flex-direction: column;
		}

		.cta-button {
			width: 100%;
			justify-content: center;
		}

		.card-footer {
			flex-direction: column;
			gap: 1rem;
			align-items: flex-start;
		}

		.card-button {
			width: 100%;
			justify-content: center;
		}

		.glow-orb-1 {
			width: 300px;
			height: 300px;
		}

		.glow-orb-2 {
			width: 250px;
			height: 250px;
		}
	}

	@media (max-width: 640px) {
		.courses-section {
			padding: 4rem 1rem;
		}

		.why-choose-section {
			padding: 4rem 1rem;
		}

		.benefits-grid {
			grid-template-columns: 1fr;
		}
	}
</style>

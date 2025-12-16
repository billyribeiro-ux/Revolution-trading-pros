<script lang="ts">
	/**
	 * Macro Overview with Sam Shames
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * URL: /macro-overview-with-sam-shames
	 * Standalone landing page for Sam Shames' Macro Overview content.
	 * Matches Simpler Trading layout.
	 *
	 * @version 1.0.0 (December 2025)
	 */

	import {
		IconPlayerPlay,
		IconClock,
		IconUser,
		IconCheck,
		IconArrowRight,
		IconChevronRight,
		IconTrendingUp,
		IconChartBar,
		IconWorld,
		IconCurrencyDollar
	} from '$lib/icons';
	import '$lib/styles/st-icons.css';

	// Course data
	const course = {
		title: 'Macro Overview',
		subtitle: 'with Sam Shames',
		description: 'Understand the big picture of the markets with a comprehensive macro analysis from one of our expert traders.',
		duration: '1 hour 20 minutes',
		thumbnail: 'https://cdn.simplertrading.com/2023/09/19020326/8-1-1.png',
		videoUrl: 'https://simpler-options.s3.amazonaws.com/tutorials/MacroOverview-SS.mp4',
		posterUrl: 'https://cdn.simplertrading.com/2023/09/19020326/8-1-1.png'
	};

	const topics = [
		{
			icon: IconWorld,
			title: 'Global Economic Landscape',
			description: 'Understanding the interconnected nature of global markets'
		},
		{
			icon: IconTrendingUp,
			title: 'Interest Rates & Fed Policy',
			description: 'How central bank decisions impact your trading'
		},
		{
			icon: IconChartBar,
			title: 'Inflation Dynamics',
			description: 'Reading inflation data and its market implications'
		},
		{
			icon: IconCurrencyDollar,
			title: 'Sector Rotation',
			description: 'Identifying where money is flowing in the market'
		}
	];

	const modules = [
		{
			title: 'Current Economic Landscape',
			duration: '18 min',
			description: 'Overview of where we are in the economic cycle'
		},
		{
			title: 'Interest Rates Deep Dive',
			duration: '15 min',
			description: 'Understanding the Fed and rate impacts'
		},
		{
			title: 'Inflation Analysis',
			duration: '12 min',
			description: 'Reading and interpreting inflation data'
		},
		{
			title: 'Sector Rotation Strategies',
			duration: '15 min',
			description: 'Following money flow through sectors'
		},
		{
			title: 'Global Market Correlations',
			duration: '10 min',
			description: 'How international markets affect US equities'
		},
		{
			title: 'Key Economic Indicators',
			duration: '10 min',
			description: 'The data points every trader should watch'
		}
	];

	const benefits = [
		'Gain a comprehensive understanding of macro factors affecting markets',
		'Learn to interpret economic data releases',
		'Understand how the Fed influences market direction',
		'Identify sector rotation opportunities before they happen',
		'Incorporate macro analysis into your trading decisions'
	];

	const relatedLessons = [
		{
			title: 'Introduction to the Yield Curve',
			trainer: 'Sam Shames',
			duration: '1:05:00',
			slug: 'introduction-yield-curve',
			thumbnail: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/07/15174557/MemberWebinar-SamShames.jpg'
		},
		{
			title: 'Trading with Market Internals',
			trainer: 'TG Watkins',
			duration: '1:10:00',
			slug: 'trading-w-market-internals-breadth',
			thumbnail: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/03/29114004/DS_19-03-29.jpg'
		},
		{
			title: 'An Intro to Divergences',
			trainer: 'Sam Shames',
			duration: '52:00',
			slug: 'intro-divergences',
			thumbnail: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/07/15174557/MemberWebinar-SamShames.jpg'
		}
	];
</script>

<svelte:head>
	<title>Macro Overview with Sam Shames | Revolution Trading Pros</title>
	<meta name="description" content="Understand the big picture of the markets with Sam Shames' comprehensive macro analysis. Learn about interest rates, inflation, sector rotation, and more." />
	<meta property="og:title" content="Macro Overview with Sam Shames" />
	<meta property="og:description" content="Comprehensive macro analysis to help you understand the big picture of the markets." />
	<meta property="og:image" content={course.thumbnail} />
	<meta property="og:type" content="website" />
</svelte:head>

<div class="course-page">
	<!-- Hero Section -->
	<section class="hero">
		<div class="hero__inner">
			<div class="hero__content">
				<span class="hero__badge">Macro Analysis</span>
				<h1 class="hero__title">{course.title}</h1>
				<p class="hero__subtitle">{course.subtitle}</p>
				<p class="hero__description">{course.description}</p>

				<div class="hero__meta">
					<span class="hero__meta-item">
						<IconClock size={18} />
						{course.duration}
					</span>
					<span class="hero__meta-item">
						<IconUser size={18} />
						Sam Shames
					</span>
				</div>

				<a href="#watch" class="hero__cta">
					<IconPlayerPlay size={20} />
					Watch Now
				</a>
			</div>

			<div class="hero__image">
				<img src={course.thumbnail} alt="Sam Shames - Macro Overview" />
				<div class="hero__image-overlay">
					<a href="#watch" class="play-btn">
						<IconPlayerPlay size={36} />
					</a>
				</div>
			</div>
		</div>
	</section>

	<!-- Topics Grid -->
	<section class="topics-section">
		<div class="topics-section__inner">
			<div class="topics-grid">
				{#each topics as topic}
					<div class="topic-card">
						<div class="topic-icon">
							<topic.icon size={28} />
						</div>
						<h3 class="topic-title">{topic.title}</h3>
						<p class="topic-description">{topic.description}</p>
					</div>
				{/each}
			</div>
		</div>
	</section>

	<!-- Video Section -->
	<section id="watch" class="video-section">
		<div class="video-section__inner">
			<h2 class="section-title">Watch the Full Session</h2>
			<div class="video-wrapper">
				<video
					controls
					width="100%"
					poster={course.posterUrl}
					preload="metadata"
				>
					<source src={course.videoUrl} type="video/mp4" />
					Your browser does not support HTML5 video.
				</video>
			</div>
		</div>
	</section>

	<!-- What You'll Learn -->
	<section class="benefits-section">
		<div class="benefits-section__inner">
			<h2 class="section-title">What You'll Learn</h2>
			<div class="benefits-grid">
				{#each benefits as benefit}
					<div class="benefit-item">
						<IconCheck size={24} class="benefit-icon" />
						<span>{benefit}</span>
					</div>
				{/each}
			</div>
		</div>
	</section>

	<!-- Session Breakdown -->
	<section class="modules-section">
		<div class="modules-section__inner">
			<h2 class="section-title">Session Breakdown</h2>
			<div class="modules-list">
				{#each modules as module, index}
					<div class="module-card">
						<span class="module-number">{index + 1}</span>
						<div class="module-content">
							<h3 class="module-title">{module.title}</h3>
							<p class="module-description">{module.description}</p>
						</div>
						<span class="module-duration">{module.duration}</span>
					</div>
				{/each}
			</div>
		</div>
	</section>

	<!-- About Instructor -->
	<section class="instructor-section">
		<div class="instructor-section__inner">
			<div class="instructor-image">
				<img src="https://cdn.simplertrading.com/dev/wp-content/uploads/2019/07/15174557/MemberWebinar-SamShames.jpg" alt="Sam Shames" />
			</div>
			<div class="instructor-content">
				<h2 class="section-title">About Your Instructor</h2>
				<h3 class="instructor-name">Sam Shames</h3>
				<p class="instructor-role">Options & Macro Analyst</p>
				<p class="instructor-bio">
					Sam Shames brings a unique perspective to trading by combining deep macro
					analysis with technical trading strategies. His background in economics
					and finance gives him a comprehensive view of market dynamics.
				</p>
				<p class="instructor-bio">
					Known for his clear explanations of complex economic concepts, Sam helps
					traders understand how macro factors influence day-to-day price action
					and longer-term trends.
				</p>
				<a href="/learning-center?trainer=sam-shames" class="instructor-link">
					View More Lessons by Sam
					<IconArrowRight size={16} />
				</a>
			</div>
		</div>
	</section>

	<!-- Related Content -->
	<section class="related-section">
		<div class="related-section__inner">
			<h2 class="section-title">Continue Learning</h2>
			<p class="section-subtitle">Explore more macro and technical analysis content</p>
			<div class="related-grid">
				{#each relatedLessons as lesson}
					<a href="/learning-center/{lesson.slug}" class="related-card">
						<div class="related-thumbnail">
							<img src={lesson.thumbnail} alt={lesson.title} />
							<span class="related-duration">{lesson.duration}</span>
						</div>
						<div class="related-content">
							<h4 class="related-title">{lesson.title}</h4>
							<span class="related-trainer">{lesson.trainer}</span>
						</div>
					</a>
				{/each}
			</div>
			<a href="/learning-center" class="view-all-link">
				View All Lessons
				<IconChevronRight size={18} />
			</a>
		</div>
	</section>

	<!-- CTA Section -->
	<section class="cta-section">
		<div class="cta-section__inner">
			<h2 class="cta-title">Ready to See the Big Picture?</h2>
			<p class="cta-description">
				Understanding macro factors is essential for making informed trading decisions.
			</p>
			<a href="#watch" class="cta-btn">
				<IconPlayerPlay size={20} />
				Start Watching Now
			</a>
			<a href="/learning-center" class="cta-link">
				Explore More Content
				<IconChevronRight size={16} />
			</a>
		</div>
	</section>
</div>

<style>
	.course-page {
		background: #f8f9fa;
	}

	/* Hero */
	.hero {
		background: linear-gradient(135deg, #0a101c 0%, #1a2332 100%);
		padding: 80px 0;
	}

	.hero__inner {
		max-width: 1200px;
		margin: 0 auto;
		padding: 0 24px;
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 60px;
		align-items: center;
	}

	.hero__badge {
		display: inline-block;
		padding: 6px 14px;
		background: linear-gradient(135deg, #6366f1, #4f46e5);
		border-radius: 20px;
		font-size: 12px;
		font-weight: 700;
		color: #fff;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		margin-bottom: 20px;
	}

	.hero__title {
		font-size: 48px;
		font-weight: 800;
		color: #fff;
		line-height: 1.1;
		margin: 0 0 8px;
	}

	.hero__subtitle {
		font-size: 24px;
		color: #0ea5e9;
		margin: 0 0 20px;
		font-weight: 500;
	}

	.hero__description {
		font-size: 18px;
		color: rgba(255, 255, 255, 0.7);
		line-height: 1.6;
		margin: 0 0 24px;
	}

	.hero__meta {
		display: flex;
		gap: 24px;
		margin-bottom: 32px;
	}

	.hero__meta-item {
		display: flex;
		align-items: center;
		gap: 8px;
		font-size: 15px;
		color: rgba(255, 255, 255, 0.6);
	}

	.hero__cta {
		display: inline-flex;
		align-items: center;
		gap: 10px;
		padding: 16px 32px;
		background: linear-gradient(135deg, #f97316, #ea580c);
		border-radius: 8px;
		font-size: 16px;
		font-weight: 700;
		color: #fff;
		text-decoration: none;
		transition: all 0.2s ease;
	}

	.hero__cta:hover {
		transform: translateY(-2px);
		box-shadow: 0 10px 30px rgba(249, 115, 22, 0.3);
	}

	.hero__image {
		position: relative;
		border-radius: 16px;
		overflow: hidden;
	}

	.hero__image img {
		width: 100%;
		aspect-ratio: 16 / 9;
		object-fit: cover;
	}

	.hero__image-overlay {
		position: absolute;
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(0, 0, 0, 0.4);
	}

	.play-btn {
		width: 80px;
		height: 80px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(255, 255, 255, 0.95);
		border-radius: 50%;
		color: #0a101c;
		transition: all 0.2s ease;
	}

	.play-btn:hover {
		transform: scale(1.1);
		background: #fff;
	}

	/* Topics Grid */
	.topics-section {
		padding: 60px 0;
		background: #fff;
		border-bottom: 1px solid #e5e7eb;
	}

	.topics-section__inner {
		max-width: 1200px;
		margin: 0 auto;
		padding: 0 24px;
	}

	.topics-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 24px;
	}

	.topic-card {
		text-align: center;
		padding: 24px;
	}

	.topic-icon {
		width: 64px;
		height: 64px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, #f0f9ff, #e0f2fe);
		border-radius: 16px;
		color: #0369a1;
		margin: 0 auto 16px;
	}

	.topic-title {
		font-size: 16px;
		font-weight: 700;
		color: #1e293b;
		margin: 0 0 8px;
	}

	.topic-description {
		font-size: 14px;
		color: #64748b;
		margin: 0;
		line-height: 1.5;
	}

	/* Video Section */
	.video-section {
		padding: 80px 0;
		background: #f8f9fa;
	}

	.video-section__inner {
		max-width: 1000px;
		margin: 0 auto;
		padding: 0 24px;
	}

	.section-title {
		font-size: 32px;
		font-weight: 800;
		color: #1e293b;
		text-align: center;
		margin: 0 0 40px;
	}

	.section-subtitle {
		font-size: 16px;
		color: #64748b;
		text-align: center;
		margin: -30px 0 40px;
	}

	.video-wrapper {
		border-radius: 16px;
		overflow: hidden;
		box-shadow: 0 20px 60px rgba(0, 0, 0, 0.15);
	}

	.video-wrapper video {
		width: 100%;
		aspect-ratio: 16 / 9;
		background: #000;
	}

	/* Benefits */
	.benefits-section {
		padding: 80px 0;
		background: #fff;
	}

	.benefits-section__inner {
		max-width: 1000px;
		margin: 0 auto;
		padding: 0 24px;
	}

	.benefits-grid {
		display: flex;
		flex-direction: column;
		gap: 20px;
	}

	.benefit-item {
		display: flex;
		align-items: flex-start;
		gap: 16px;
		padding: 20px 24px;
		background: #f8f9fa;
		border-radius: 12px;
	}

	.benefit-item :global(.benefit-icon) {
		flex-shrink: 0;
		color: #10b981;
	}

	.benefit-item span {
		font-size: 16px;
		color: #475569;
		line-height: 1.5;
	}

	/* Modules */
	.modules-section {
		padding: 80px 0;
		background: #f8f9fa;
	}

	.modules-section__inner {
		max-width: 800px;
		margin: 0 auto;
		padding: 0 24px;
	}

	.modules-list {
		display: flex;
		flex-direction: column;
		gap: 16px;
	}

	.module-card {
		display: flex;
		align-items: center;
		gap: 20px;
		padding: 24px;
		background: #fff;
		border-radius: 12px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.06);
		transition: all 0.15s ease;
	}

	.module-card:hover {
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
	}

	.module-number {
		width: 48px;
		height: 48px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, #6366f1, #4f46e5);
		border-radius: 12px;
		font-size: 18px;
		font-weight: 700;
		color: #fff;
		flex-shrink: 0;
	}

	.module-content {
		flex: 1;
	}

	.module-title {
		font-size: 17px;
		font-weight: 700;
		color: #1e293b;
		margin: 0 0 4px;
	}

	.module-description {
		font-size: 14px;
		color: #64748b;
		margin: 0;
	}

	.module-duration {
		font-size: 14px;
		font-weight: 600;
		color: #64748b;
		flex-shrink: 0;
	}

	/* Instructor */
	.instructor-section {
		padding: 80px 0;
		background: #fff;
	}

	.instructor-section__inner {
		max-width: 1000px;
		margin: 0 auto;
		padding: 0 24px;
		display: grid;
		grid-template-columns: 300px 1fr;
		gap: 60px;
		align-items: start;
	}

	.instructor-image img {
		width: 100%;
		border-radius: 16px;
		box-shadow: 0 10px 40px rgba(0, 0, 0, 0.1);
	}

	.instructor-content .section-title {
		text-align: left;
		margin-bottom: 20px;
	}

	.instructor-name {
		font-size: 28px;
		font-weight: 800;
		color: #1e293b;
		margin: 0 0 4px;
	}

	.instructor-role {
		font-size: 16px;
		color: #6366f1;
		font-weight: 500;
		margin: 0 0 20px;
	}

	.instructor-bio {
		font-size: 16px;
		color: #475569;
		line-height: 1.7;
		margin: 0 0 16px;
	}

	.instructor-link {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		font-size: 15px;
		font-weight: 600;
		color: #6366f1;
		text-decoration: none;
		margin-top: 8px;
	}

	.instructor-link:hover {
		text-decoration: underline;
	}

	/* Related Content */
	.related-section {
		padding: 80px 0;
		background: #f8f9fa;
	}

	.related-section__inner {
		max-width: 1200px;
		margin: 0 auto;
		padding: 0 24px;
		text-align: center;
	}

	.related-grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 24px;
		margin-bottom: 32px;
	}

	.related-card {
		background: #fff;
		border-radius: 12px;
		overflow: hidden;
		text-decoration: none;
		transition: all 0.2s ease;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.06);
	}

	.related-card:hover {
		transform: translateY(-4px);
		box-shadow: 0 10px 30px rgba(0, 0, 0, 0.1);
	}

	.related-thumbnail {
		position: relative;
	}

	.related-thumbnail img {
		width: 100%;
		aspect-ratio: 16 / 9;
		object-fit: cover;
	}

	.related-duration {
		position: absolute;
		bottom: 8px;
		right: 8px;
		padding: 4px 8px;
		background: rgba(0, 0, 0, 0.8);
		border-radius: 4px;
		font-size: 11px;
		font-weight: 600;
		color: #fff;
	}

	.related-content {
		padding: 20px;
		text-align: left;
	}

	.related-title {
		font-size: 16px;
		font-weight: 700;
		color: #1e293b;
		margin: 0 0 4px;
	}

	.related-trainer {
		font-size: 13px;
		color: #64748b;
	}

	.view-all-link {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		font-size: 15px;
		font-weight: 600;
		color: #6366f1;
		text-decoration: none;
	}

	.view-all-link:hover {
		text-decoration: underline;
	}

	/* CTA */
	.cta-section {
		padding: 100px 0;
		background: linear-gradient(135deg, #0a101c 0%, #1a2332 100%);
		text-align: center;
	}

	.cta-section__inner {
		max-width: 600px;
		margin: 0 auto;
		padding: 0 24px;
	}

	.cta-title {
		font-size: 36px;
		font-weight: 800;
		color: #fff;
		margin: 0 0 16px;
	}

	.cta-description {
		font-size: 18px;
		color: rgba(255, 255, 255, 0.7);
		margin: 0 0 32px;
	}

	.cta-btn {
		display: inline-flex;
		align-items: center;
		gap: 10px;
		padding: 18px 36px;
		background: linear-gradient(135deg, #f97316, #ea580c);
		border-radius: 8px;
		font-size: 17px;
		font-weight: 700;
		color: #fff;
		text-decoration: none;
		transition: all 0.2s ease;
		margin-bottom: 20px;
	}

	.cta-btn:hover {
		transform: translateY(-2px);
		box-shadow: 0 10px 30px rgba(249, 115, 22, 0.3);
	}

	.cta-link {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		font-size: 15px;
		color: rgba(255, 255, 255, 0.6);
		text-decoration: none;
	}

	.cta-link:hover {
		color: #fff;
	}

	/* Responsive */
	@media (max-width: 1024px) {
		.hero__inner {
			grid-template-columns: 1fr;
			gap: 40px;
		}

		.topics-grid {
			grid-template-columns: repeat(2, 1fr);
		}

		.instructor-section__inner {
			grid-template-columns: 1fr;
			gap: 40px;
		}

		.instructor-image {
			max-width: 300px;
			margin: 0 auto;
		}

		.instructor-content .section-title {
			text-align: center;
		}

		.related-grid {
			grid-template-columns: 1fr;
			max-width: 400px;
			margin: 0 auto 32px;
		}
	}

	@media (max-width: 768px) {
		.hero {
			padding: 60px 0;
		}

		.hero__title {
			font-size: 32px;
		}

		.hero__subtitle {
			font-size: 20px;
		}

		.topics-grid {
			grid-template-columns: 1fr;
		}

		.section-title {
			font-size: 26px;
		}

		.cta-title {
			font-size: 28px;
		}
	}
</style>

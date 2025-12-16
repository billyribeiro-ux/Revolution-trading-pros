<script lang="ts">
	/**
	 * Basic Options Strategies with Danielle Shay
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * URL: /basic-options-strategies-with-danielle-shay
	 * Standalone landing page for Danielle Shay's Basic Options course.
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
		IconStar
	} from '$lib/icons';
	import '$lib/styles/st-icons.css';

	// Course data
	const course = {
		title: 'Basic Options Strategies',
		subtitle: 'with Danielle Shay',
		description: 'Master the fundamentals of options trading with one of the most respected educators in the industry.',
		duration: '55 minutes',
		thumbnail: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27120545/MemberWebinar-DanielleShay.jpg',
		videoUrl: 'https://simpler-options.s3.amazonaws.com/tutorials/BasicOptions-DS.mp4',
		posterUrl: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27120545/MemberWebinar-DanielleShay.jpg'
	};

	const modules = [
		{
			title: 'Understanding Calls and Puts',
			duration: '12 min',
			description: 'Learn the fundamental building blocks of options trading'
		},
		{
			title: 'When to Buy vs Sell Options',
			duration: '10 min',
			description: 'Strategic decision-making for option positions'
		},
		{
			title: 'Common Options Strategies',
			duration: '15 min',
			description: 'Covered calls, protective puts, and spreads explained'
		},
		{
			title: 'Risk and Reward Profiles',
			duration: '8 min',
			description: 'Understanding your potential gains and losses'
		},
		{
			title: 'Position Sizing for Options',
			duration: '5 min',
			description: 'Proper allocation for options trades'
		},
		{
			title: 'Live Chart Examples',
			duration: '5 min',
			description: 'Real-world application of strategies'
		}
	];

	const benefits = [
		'Understand the basics of options without the complexity',
		'Learn from a professional trader with over 15 years experience',
		'Get actionable strategies you can use immediately',
		'Avoid common mistakes that cost beginners money',
		'Build a foundation for more advanced options strategies'
	];

	const testimonials = [
		{
			name: 'Michael R.',
			role: 'Member since 2021',
			text: "Danielle's teaching style made options finally click for me. I went from completely confused to confidently placing trades.",
			rating: 5
		},
		{
			name: 'Sarah T.',
			role: 'Member since 2022',
			text: 'This course saved me from making expensive mistakes. The risk management section alone was worth it.',
			rating: 5
		},
		{
			name: 'James K.',
			role: 'Member since 2020',
			text: 'Clear, concise, and practical. Exactly what I needed to start trading options.',
			rating: 5
		}
	];
</script>

<svelte:head>
	<title>Basic Options Strategies with Danielle Shay | Revolution Trading Pros</title>
	<meta name="description" content="Master the fundamentals of options trading with Danielle Shay. Learn calls, puts, and proven strategies in this comprehensive beginner-friendly course." />
	<meta property="og:title" content="Basic Options Strategies with Danielle Shay" />
	<meta property="og:description" content="Master the fundamentals of options trading with one of the most respected educators in the industry." />
	<meta property="og:image" content={course.thumbnail} />
	<meta property="og:type" content="website" />
</svelte:head>

<div class="course-page">
	<!-- Hero Section -->
	<section class="hero">
		<div class="hero__inner">
			<div class="hero__content">
				<span class="hero__badge">Free Course</span>
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
						Danielle Shay
					</span>
				</div>

				<a href="#watch" class="hero__cta">
					<IconPlayerPlay size={20} />
					Watch Now
				</a>
			</div>

			<div class="hero__image">
				<img src={course.thumbnail} alt="Danielle Shay - Basic Options Strategies" />
				<div class="hero__image-overlay">
					<a href="#watch" class="play-btn">
						<IconPlayerPlay size={36} />
					</a>
				</div>
			</div>
		</div>
	</section>

	<!-- Video Section -->
	<section id="watch" class="video-section">
		<div class="video-section__inner">
			<h2 class="section-title">Watch the Full Course</h2>
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

	<!-- Course Modules -->
	<section class="modules-section">
		<div class="modules-section__inner">
			<h2 class="section-title">Course Modules</h2>
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
				<img src={course.thumbnail} alt="Danielle Shay" />
			</div>
			<div class="instructor-content">
				<h2 class="section-title">About Your Instructor</h2>
				<h3 class="instructor-name">Danielle Shay</h3>
				<p class="instructor-role">Director of Options at Simpler Trading</p>
				<p class="instructor-bio">
					Danielle Shay has been trading options professionally for over 15 years.
					As the Director of Options at Simpler Trading, she specializes in helping
					traders understand complex strategies in simple terms.
				</p>
				<p class="instructor-bio">
					Her unique ability to break down complicated concepts has helped thousands
					of traders successfully transition from stocks to options trading.
				</p>
				<a href="/learning-center?trainer=danielle-shay" class="instructor-link">
					View More Lessons by Danielle
					<IconArrowRight size={16} />
				</a>
			</div>
		</div>
	</section>

	<!-- Testimonials -->
	<section class="testimonials-section">
		<div class="testimonials-section__inner">
			<h2 class="section-title">What Students Say</h2>
			<div class="testimonials-grid">
				{#each testimonials as testimonial}
					<div class="testimonial-card">
						<div class="testimonial-rating">
							{#each Array(testimonial.rating) as _}
								<IconStar size={16} fill="#f59e0b" />
							{/each}
						</div>
						<p class="testimonial-text">"{testimonial.text}"</p>
						<div class="testimonial-author">
							<span class="testimonial-name">{testimonial.name}</span>
							<span class="testimonial-role">{testimonial.role}</span>
						</div>
					</div>
				{/each}
			</div>
		</div>
	</section>

	<!-- CTA Section -->
	<section class="cta-section">
		<div class="cta-section__inner">
			<h2 class="cta-title">Ready to Master Options Trading?</h2>
			<p class="cta-description">
				Start your options trading journey with this free comprehensive course.
			</p>
			<a href="#watch" class="cta-btn">
				<IconPlayerPlay size={20} />
				Start Learning Now
			</a>
			<a href="/learning-center" class="cta-link">
				Explore More Courses
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
		background: linear-gradient(135deg, #10b981, #059669);
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

	/* Video Section */
	.video-section {
		padding: 80px 0;
		background: #fff;
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
		background: #f8f9fa;
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
		background: #fff;
		border-radius: 12px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.06);
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
		background: #fff;
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
		background: #f8f9fa;
		border-radius: 12px;
		transition: all 0.15s ease;
	}

	.module-card:hover {
		background: #f1f5f9;
	}

	.module-number {
		width: 48px;
		height: 48px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #0ea5e9;
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
		background: #f8f9fa;
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
		color: #0ea5e9;
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
		color: #0ea5e9;
		text-decoration: none;
		margin-top: 8px;
	}

	.instructor-link:hover {
		text-decoration: underline;
	}

	/* Testimonials */
	.testimonials-section {
		padding: 80px 0;
		background: #fff;
	}

	.testimonials-section__inner {
		max-width: 1200px;
		margin: 0 auto;
		padding: 0 24px;
	}

	.testimonials-grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 24px;
	}

	.testimonial-card {
		padding: 32px;
		background: #f8f9fa;
		border-radius: 16px;
	}

	.testimonial-rating {
		display: flex;
		gap: 2px;
		margin-bottom: 16px;
		color: #f59e0b;
	}

	.testimonial-text {
		font-size: 15px;
		color: #475569;
		line-height: 1.7;
		margin: 0 0 20px;
		font-style: italic;
	}

	.testimonial-author {
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.testimonial-name {
		font-size: 15px;
		font-weight: 700;
		color: #1e293b;
	}

	.testimonial-role {
		font-size: 13px;
		color: #64748b;
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

		.section-title {
			font-size: 26px;
		}

		.testimonials-grid {
			grid-template-columns: 1fr;
		}

		.cta-title {
			font-size: 28px;
		}
	}
</style>

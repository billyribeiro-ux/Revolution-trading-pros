<script lang="ts">
	/**
	 * Learning Center - Main Listing Page
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * URL: /learning-center
	 * Shows all available educational content organized by category and trainer.
	 * Matches Simpler Trading's Learning Center layout.
	 *
	 * @version 1.0.0 (December 2025)
	 */

	import { page } from '$app/stores';
	import {
		IconPlayerPlay,
		IconClock,
		IconFilter,
		IconSearch,
		IconChevronRight,
		IconUser,
		IconBook,
		IconX
	} from '@tabler/icons-svelte';
	import '$lib/styles/st-icons.css';

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let searchQuery = $state('');
	let selectedCategory = $state('all');
	let selectedTrainer = $state('all');

	// ═══════════════════════════════════════════════════════════════════════════
	// CATEGORIES
	// ═══════════════════════════════════════════════════════════════════════════

	const categories = [
		{ id: 'all', name: 'All Content' },
		{ id: 'getting-started', name: 'Getting Started' },
		{ id: 'strategies', name: 'Trading Strategies' },
		{ id: 'technical', name: 'Technical Analysis' },
		{ id: 'psychology', name: 'Trading Psychology' },
		{ id: 'platform', name: 'Platform Tutorials' },
		{ id: 'indicators', name: 'Indicators' }
	];

	// ═══════════════════════════════════════════════════════════════════════════
	// TRAINERS (matching Simpler Trading)
	// ═══════════════════════════════════════════════════════════════════════════

	const trainers = [
		{ id: 'all', name: 'All Trainers' },
		{ id: 'allison-ostrander', name: 'Allison Ostrander', image: 'https://cdn.simplertrading.com/2023/06/07133302/Allison-SmallAccountMentorship.jpg' },
		{ id: 'chris-brecher', name: 'Chris Brecher', image: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/09/27154605/MemberWebinar-ChrisBrecher.jpg' },
		{ id: 'danielle-shay', name: 'Danielle Shay', image: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27120545/MemberWebinar-DanielleShay.jpg' },
		{ id: 'eric-purdy', name: 'Eric Purdy', image: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27120614/MemberWebinar-Generic1.jpg' },
		{ id: 'kody-ashmore', name: 'Kody Ashmore', image: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27120614/MemberWebinar-Generic1.jpg' },
		{ id: 'sam-shames', name: 'Sam Shames', image: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/07/15174557/MemberWebinar-SamShames.jpg' },
		{ id: 'tg-watkins', name: 'TG Watkins', image: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/03/29114004/DS_19-03-29.jpg' },
		{ id: 'jonathan-mckeever', name: 'Jonathan McKeever', image: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27120614/MemberWebinar-Generic1.jpg' }
	];

	// ═══════════════════════════════════════════════════════════════════════════
	// LESSONS DATA (matching Simpler Trading Learning Center content)
	// ═══════════════════════════════════════════════════════════════════════════

	interface Lesson {
		id: string;
		slug: string;
		title: string;
		description: string;
		trainer: string;
		trainerId: string;
		category: string;
		duration: string;
		thumbnail: string;
		videoUrl?: string;
		publishedDate: string;
	}

	const lessons: Lesson[] = [
		// Allison Ostrander
		{
			id: '1',
			slug: 'growing-a-small-account',
			title: 'Growing a Small Account',
			description: 'Allison Ostrander shares her strategies for growing a small trading account effectively.',
			trainer: 'Allison Ostrander',
			trainerId: 'allison-ostrander',
			category: 'strategies',
			duration: '45:00',
			thumbnail: 'https://cdn.simplertrading.com/2023/06/07133302/Allison-SmallAccountMentorship.jpg',
			publishedDate: '2022-11-14'
		},
		{
			id: '2',
			slug: 'fomo-to-jomo-with-ao',
			title: 'Psychology: Turning FOMO into JOMO',
			description: 'Learn how to transform Fear of Missing Out into Joy of Missing Out for better trading decisions.',
			trainer: 'Allison Ostrander',
			trainerId: 'allison-ostrander',
			category: 'psychology',
			duration: '38:00',
			thumbnail: 'https://cdn.simplertrading.com/2023/06/07133302/Allison-SmallAccountMentorship.jpg',
			videoUrl: 'https://simpler-options.s3.amazonaws.com/tutorials/FOMOtoJOMO-AO-11152022.mp4',
			publishedDate: '2022-11-15'
		},
		// Chris Brecher
		{
			id: '3',
			slug: 'chris-brecher-watchlist',
			title: 'How Chris Brecher Builds His Watchlist',
			description: 'Chris Brecher walks through his process for building and maintaining a trading watchlist.',
			trainer: 'Chris Brecher',
			trainerId: 'chris-brecher',
			category: 'strategies',
			duration: '42:00',
			thumbnail: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/09/27154605/MemberWebinar-ChrisBrecher.jpg',
			publishedDate: '2022-10-20'
		},
		// Danielle Shay
		{
			id: '4',
			slug: 'basic-options-strategies-with-danielle-shay',
			title: 'Basic Options Strategies',
			description: 'Danielle Shay introduces fundamental options trading strategies for beginners.',
			trainer: 'Danielle Shay',
			trainerId: 'danielle-shay',
			category: 'strategies',
			duration: '55:00',
			thumbnail: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27120545/MemberWebinar-DanielleShay.jpg',
			publishedDate: '2023-01-15'
		},
		// Eric Purdy
		{
			id: '5',
			slug: 'thinkorswim101-eric-purdy',
			title: 'ThinkorSwim (TOS) 101',
			description: 'Eric Purdy provides a comprehensive introduction to the ThinkorSwim trading platform.',
			trainer: 'Eric Purdy',
			trainerId: 'eric-purdy',
			category: 'platform',
			duration: '1:15:00',
			thumbnail: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27120614/MemberWebinar-Generic1.jpg',
			publishedDate: '2022-09-10'
		},
		// Kody Ashmore
		{
			id: '6',
			slug: 'quick-hits-review-advanced-version',
			title: 'Quick Hits Review — Advanced Version',
			description: 'Kody Ashmore reviews the advanced version of the Quick Hits strategy.',
			trainer: 'Kody Ashmore',
			trainerId: 'kody-ashmore',
			category: 'strategies',
			duration: '48:00',
			thumbnail: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27120614/MemberWebinar-Generic1.jpg',
			publishedDate: '2022-12-01'
		},
		// Sam Shames
		{
			id: '7',
			slug: 'intro-divergences',
			title: 'An Intro to Divergences',
			description: 'Sam Shames explains how to identify and trade divergences in the market.',
			trainer: 'Sam Shames',
			trainerId: 'sam-shames',
			category: 'technical',
			duration: '52:00',
			thumbnail: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/07/15174557/MemberWebinar-SamShames.jpg',
			publishedDate: '2022-11-01'
		},
		{
			id: '8',
			slug: 'quick-hits-review-james-version',
			title: 'Quick Hits Review — James Version',
			description: 'Sam Shames reviews the Quick Hits strategy and indicators based on the simpler version that John Carter taught his son.',
			trainer: 'Sam Shames',
			trainerId: 'sam-shames',
			category: 'strategies',
			duration: '35:00',
			thumbnail: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/07/15174557/MemberWebinar-SamShames.jpg',
			publishedDate: '2022-12-02'
		},
		{
			id: '9',
			slug: 'introduction-yield-curve',
			title: 'Introduction to the Yield Curve',
			description: 'Sam reviews the yield curve from a beginner, intermediate, and advanced level.',
			trainer: 'Sam Shames',
			trainerId: 'sam-shames',
			category: 'technical',
			duration: '1:05:00',
			thumbnail: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/07/15174557/MemberWebinar-SamShames.jpg',
			publishedDate: '2023-02-10'
		},
		{
			id: '10',
			slug: 'macro-overview-with-sam-shames',
			title: 'Macro Overview',
			description: 'Sam Shames provides a comprehensive macro overview of the markets.',
			trainer: 'Sam Shames',
			trainerId: 'sam-shames',
			category: 'technical',
			duration: '1:20:00',
			thumbnail: 'https://cdn.simplertrading.com/2023/09/19020326/8-1-1.png',
			publishedDate: '2023-09-22'
		},
		// TG Watkins
		{
			id: '11',
			slug: 'checklist-buy-stocks',
			title: 'Checklist for New Traders Looking to Buy Stocks',
			description: 'TG introduces his checklist for new traders looking to buy stocks and walks through setting up your trading account.',
			trainer: 'TG Watkins',
			trainerId: 'tg-watkins',
			category: 'getting-started',
			duration: '40:00',
			thumbnail: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/03/29114004/DS_19-03-29.jpg',
			publishedDate: '2023-01-20'
		},
		{
			id: '12',
			slug: 'tos-tips-tricks-tg',
			title: 'ThinkOrSwim: Tips & Tricks',
			description: 'TG Watkins shares some of his favorite features and tips & tricks when using the ThinkOrSwim platform.',
			trainer: 'TG Watkins',
			trainerId: 'tg-watkins',
			category: 'platform',
			duration: '55:00',
			thumbnail: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/03/29114004/DS_19-03-29.jpg',
			publishedDate: '2022-10-27'
		},
		{
			id: '13',
			slug: 'trading-w-market-internals-breadth',
			title: 'Trading with Market Internals and Breadth',
			description: 'TG Watkins explains how to use market internals and breadth indicators in your trading.',
			trainer: 'TG Watkins',
			trainerId: 'tg-watkins',
			category: 'technical',
			duration: '1:10:00',
			thumbnail: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/03/29114004/DS_19-03-29.jpg',
			publishedDate: '2023-03-02'
		},
		{
			id: '14',
			slug: 'stock-trader-tips-volatile-market',
			title: 'Stock Trader Tips For Volatile Market',
			description: 'Essential tips and strategies for navigating volatile market conditions.',
			trainer: 'TG Watkins',
			trainerId: 'tg-watkins',
			category: 'strategies',
			duration: '45:00',
			thumbnail: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/03/29114004/DS_19-03-29.jpg',
			publishedDate: '2023-02-15'
		},
		// Jonathan McKeever
		{
			id: '15',
			slug: 'intro-jonathan-mckeever-favorite-strategy',
			title: "An Introduction to Jonathan McKeever's Favorite Strategy",
			description: 'Jonathan McKeever introduces himself and his favorite strategy, explaining supply and demand and how to identify formations.',
			trainer: 'Jonathan McKeever',
			trainerId: 'jonathan-mckeever',
			category: 'strategies',
			duration: '58:00',
			thumbnail: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27120614/MemberWebinar-Generic1.jpg',
			publishedDate: '2022-11-14'
		}
	];

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE
	// ═══════════════════════════════════════════════════════════════════════════

	const filteredLessons = $derived.by(() => {
		return lessons.filter(lesson => {
			const matchesCategory = selectedCategory === 'all' || lesson.category === selectedCategory;
			const matchesTrainer = selectedTrainer === 'all' || lesson.trainerId === selectedTrainer;
			const matchesSearch = searchQuery === '' ||
				lesson.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
				lesson.description.toLowerCase().includes(searchQuery.toLowerCase()) ||
				lesson.trainer.toLowerCase().includes(searchQuery.toLowerCase());
			return matchesCategory && matchesTrainer && matchesSearch;
		});
	});

	const activeFiltersCount = $derived(
		(selectedCategory !== 'all' ? 1 : 0) +
		(selectedTrainer !== 'all' ? 1 : 0) +
		(searchQuery !== '' ? 1 : 0)
	);

	function clearFilters() {
		selectedCategory = 'all';
		selectedTrainer = 'all';
		searchQuery = '';
	}
</script>

<!-- ═══════════════════════════════════════════════════════════════════════════
     HEAD
     ═══════════════════════════════════════════════════════════════════════════ -->

<svelte:head>
	<title>Learning Center | Revolution Trading Pros</title>
	<meta name="description" content="Access our comprehensive library of trading education content. Learn strategies, technical analysis, and trading psychology from expert traders." />
	<meta property="og:title" content="Learning Center | Revolution Trading Pros" />
	<meta property="og:description" content="Access our comprehensive library of trading education content." />
	<meta property="og:type" content="website" />
</svelte:head>

<!-- ═══════════════════════════════════════════════════════════════════════════
     PAGE HEADER
     ═══════════════════════════════════════════════════════════════════════════ -->

<div class="learning-center">
	<header class="lc-header">
		<div class="lc-header__inner">
			<nav class="lc-breadcrumb" aria-label="Breadcrumb">
				<a href="/">Home</a>
				<span class="separator">/</span>
				<span class="current">Learning Center</span>
			</nav>
			<h1 class="lc-title">
				<span class="st-icon-learning-center"></span>
				Learning Center
			</h1>
			<p class="lc-subtitle">
				Comprehensive trading education from our expert team of traders
			</p>
		</div>
	</header>

	<!-- ═══════════════════════════════════════════════════════════════════════════
	     FILTERS
	     ═══════════════════════════════════════════════════════════════════════════ -->

	<div class="lc-filters">
		<div class="lc-filters__inner">
			<!-- Search -->
			<div class="lc-search">
				<IconSearch size={18} />
				<input
					type="text"
					placeholder="Search lessons..."
					bind:value={searchQuery}
					class="lc-search__input"
				/>
				{#if searchQuery}
					<button class="lc-search__clear" onclick={() => searchQuery = ''}>
						<IconX size={16} />
					</button>
				{/if}
			</div>

			<!-- Category Filter -->
			<div class="lc-filter-group">
				<label class="lc-filter-label">
					<IconFilter size={16} />
					Category
				</label>
				<div class="lc-filter-buttons">
					{#each categories as category}
						<button
							class="lc-filter-btn"
							class:active={selectedCategory === category.id}
							onclick={() => selectedCategory = category.id}
						>
							{category.name}
						</button>
					{/each}
				</div>
			</div>

			<!-- Trainer Filter -->
			<div class="lc-filter-group">
				<label class="lc-filter-label">
					<IconUser size={16} />
					Trainer
				</label>
				<select
					class="lc-filter-select"
					bind:value={selectedTrainer}
				>
					{#each trainers as trainer}
						<option value={trainer.id}>{trainer.name}</option>
					{/each}
				</select>
			</div>

			<!-- Active Filters -->
			{#if activeFiltersCount > 0}
				<button class="lc-clear-filters" onclick={clearFilters}>
					<IconX size={14} />
					Clear Filters ({activeFiltersCount})
				</button>
			{/if}
		</div>
	</div>

	<!-- ═══════════════════════════════════════════════════════════════════════════
	     LESSONS GRID
	     ═══════════════════════════════════════════════════════════════════════════ -->

	<div class="lc-content">
		<div class="lc-results-count">
			Showing {filteredLessons.length} of {lessons.length} lessons
		</div>

		<div class="lc-grid">
			{#each filteredLessons as lesson (lesson.id)}
				<a href="/learning-center/{lesson.slug}" class="lc-card">
					<div class="lc-card__thumbnail">
						<img src={lesson.thumbnail} alt={lesson.title} loading="lazy" />
						<div class="lc-card__play-overlay">
							<IconPlayerPlay size={48} />
						</div>
						<span class="lc-card__duration">
							<IconClock size={12} />
							{lesson.duration}
						</span>
					</div>
					<div class="lc-card__content">
						<span class="lc-card__category">{categories.find(c => c.id === lesson.category)?.name || lesson.category}</span>
						<h3 class="lc-card__title">{lesson.title}</h3>
						<p class="lc-card__description">{lesson.description}</p>
						<div class="lc-card__footer">
							<span class="lc-card__trainer">
								<IconUser size={14} />
								{lesson.trainer}
							</span>
							<span class="lc-card__arrow">
								<IconChevronRight size={18} />
							</span>
						</div>
					</div>
				</a>
			{/each}
		</div>

		{#if filteredLessons.length === 0}
			<div class="lc-empty">
				<IconBook size={64} />
				<h3>No lessons found</h3>
				<p>Try adjusting your search or filter criteria</p>
				<button class="lc-empty__btn" onclick={clearFilters}>
					Clear All Filters
				</button>
			</div>
		{/if}
	</div>
</div>

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	.learning-center {
		min-height: 100vh;
		background: #f8f9fa;
	}

	/* Header */
	.lc-header {
		background: linear-gradient(135deg, #0a101c 0%, #1a2332 100%);
		padding: 60px 0 40px;
	}

	.lc-header__inner {
		max-width: 1200px;
		margin: 0 auto;
		padding: 0 24px;
	}

	.lc-breadcrumb {
		font-size: 14px;
		color: rgba(255, 255, 255, 0.6);
		margin-bottom: 16px;
	}

	.lc-breadcrumb a {
		color: rgba(255, 255, 255, 0.6);
		text-decoration: none;
		transition: color 0.15s ease;
	}

	.lc-breadcrumb a:hover {
		color: #fff;
	}

	.lc-breadcrumb .separator {
		margin: 0 8px;
	}

	.lc-breadcrumb .current {
		color: #fff;
	}

	.lc-title {
		display: flex;
		align-items: center;
		gap: 16px;
		font-family: 'Inter', sans-serif;
		font-size: 42px;
		font-weight: 800;
		color: #fff;
		margin: 0 0 12px;
	}

	.lc-title .st-icon-learning-center {
		font-size: 42px;
		color: #0ea5e9;
	}

	.lc-subtitle {
		font-size: 18px;
		color: rgba(255, 255, 255, 0.7);
		margin: 0;
	}

	/* Filters */
	.lc-filters {
		background: #fff;
		border-bottom: 1px solid #e5e7eb;
		padding: 24px 0;
		position: sticky;
		top: 0;
		z-index: 100;
	}

	.lc-filters__inner {
		max-width: 1200px;
		margin: 0 auto;
		padding: 0 24px;
		display: flex;
		flex-wrap: wrap;
		gap: 20px;
		align-items: flex-end;
	}

	.lc-search {
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 12px 16px;
		background: #f8f9fa;
		border: 1px solid #e5e7eb;
		border-radius: 8px;
		flex: 1;
		min-width: 250px;
		max-width: 350px;
		color: #64748b;
	}

	.lc-search__input {
		flex: 1;
		border: none;
		outline: none;
		font-size: 14px;
		background: transparent;
		color: #1e293b;
	}

	.lc-search__input::placeholder {
		color: #94a3b8;
	}

	.lc-search__clear {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 24px;
		height: 24px;
		border: none;
		background: #e5e7eb;
		border-radius: 50%;
		cursor: pointer;
		color: #64748b;
		transition: all 0.15s ease;
	}

	.lc-search__clear:hover {
		background: #0ea5e9;
		color: #fff;
	}

	.lc-filter-group {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.lc-filter-label {
		display: flex;
		align-items: center;
		gap: 6px;
		font-size: 12px;
		font-weight: 600;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.lc-filter-buttons {
		display: flex;
		flex-wrap: wrap;
		gap: 8px;
	}

	.lc-filter-btn {
		padding: 8px 14px;
		background: #fff;
		border: 1px solid #e5e7eb;
		border-radius: 20px;
		font-size: 13px;
		font-weight: 500;
		color: #64748b;
		cursor: pointer;
		transition: all 0.15s ease;
		white-space: nowrap;
	}

	.lc-filter-btn:hover {
		border-color: #0ea5e9;
		color: #0ea5e9;
	}

	.lc-filter-btn.active {
		background: #0ea5e9;
		border-color: #0ea5e9;
		color: #fff;
	}

	.lc-filter-select {
		padding: 10px 14px;
		background: #fff;
		border: 1px solid #e5e7eb;
		border-radius: 8px;
		font-size: 14px;
		color: #1e293b;
		cursor: pointer;
		min-width: 180px;
	}

	.lc-clear-filters {
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 10px 16px;
		background: #fee2e2;
		border: none;
		border-radius: 8px;
		font-size: 13px;
		font-weight: 500;
		color: #dc2626;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.lc-clear-filters:hover {
		background: #fecaca;
	}

	/* Content */
	.lc-content {
		max-width: 1200px;
		margin: 0 auto;
		padding: 32px 24px 60px;
	}

	.lc-results-count {
		font-size: 14px;
		color: #64748b;
		margin-bottom: 24px;
	}

	/* Grid */
	.lc-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
		gap: 24px;
	}

	/* Card */
	.lc-card {
		background: #fff;
		border-radius: 12px;
		overflow: hidden;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.08);
		text-decoration: none;
		transition: all 0.2s ease;
	}

	.lc-card:hover {
		transform: translateY(-4px);
		box-shadow: 0 12px 24px rgba(0, 0, 0, 0.12);
	}

	.lc-card__thumbnail {
		position: relative;
		aspect-ratio: 16 / 9;
		overflow: hidden;
	}

	.lc-card__thumbnail img {
		width: 100%;
		height: 100%;
		object-fit: cover;
		transition: transform 0.3s ease;
	}

	.lc-card:hover .lc-card__thumbnail img {
		transform: scale(1.05);
	}

	.lc-card__play-overlay {
		position: absolute;
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(0, 0, 0, 0.4);
		opacity: 0;
		transition: opacity 0.2s ease;
		color: #fff;
	}

	.lc-card:hover .lc-card__play-overlay {
		opacity: 1;
	}

	.lc-card__duration {
		position: absolute;
		bottom: 12px;
		right: 12px;
		display: flex;
		align-items: center;
		gap: 4px;
		padding: 6px 10px;
		background: rgba(0, 0, 0, 0.8);
		border-radius: 4px;
		font-size: 12px;
		font-weight: 500;
		color: #fff;
	}

	.lc-card__content {
		padding: 20px;
	}

	.lc-card__category {
		display: inline-block;
		padding: 4px 10px;
		background: #f0f9ff;
		border-radius: 4px;
		font-size: 11px;
		font-weight: 600;
		color: #0369a1;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		margin-bottom: 12px;
	}

	.lc-card__title {
		font-size: 18px;
		font-weight: 700;
		color: #1e293b;
		margin: 0 0 8px;
		line-height: 1.3;
	}

	.lc-card__description {
		font-size: 14px;
		color: #64748b;
		margin: 0 0 16px;
		line-height: 1.5;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.lc-card__footer {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding-top: 16px;
		border-top: 1px solid #f1f5f9;
	}

	.lc-card__trainer {
		display: flex;
		align-items: center;
		gap: 6px;
		font-size: 13px;
		font-weight: 500;
		color: #64748b;
	}

	.lc-card__arrow {
		color: #cbd5e1;
		transition: all 0.15s ease;
	}

	.lc-card:hover .lc-card__arrow {
		color: #0ea5e9;
		transform: translateX(4px);
	}

	/* Empty State */
	.lc-empty {
		text-align: center;
		padding: 80px 20px;
		color: #64748b;
	}

	.lc-empty h3 {
		font-size: 20px;
		font-weight: 700;
		color: #1e293b;
		margin: 20px 0 8px;
	}

	.lc-empty p {
		font-size: 14px;
		margin: 0 0 24px;
	}

	.lc-empty__btn {
		padding: 12px 24px;
		background: #0ea5e9;
		border: none;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 600;
		color: #fff;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.lc-empty__btn:hover {
		background: #0284c7;
	}

	/* Responsive */
	@media (max-width: 768px) {
		.lc-header {
			padding: 40px 0 30px;
		}

		.lc-title {
			font-size: 28px;
		}

		.lc-title .st-icon-learning-center {
			font-size: 28px;
		}

		.lc-subtitle {
			font-size: 15px;
		}

		.lc-filters__inner {
			flex-direction: column;
			align-items: stretch;
		}

		.lc-search {
			max-width: none;
		}

		.lc-filter-buttons {
			overflow-x: auto;
			flex-wrap: nowrap;
			padding-bottom: 4px;
			-webkit-overflow-scrolling: touch;
		}

		.lc-grid {
			grid-template-columns: 1fr;
		}
	}
</style>

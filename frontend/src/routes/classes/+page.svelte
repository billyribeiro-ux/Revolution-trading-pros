<script lang="ts">
	/**
	 * Course Catalog Page
	 * Apple Principal Engineer ICT 7 Grade - January 2026
	 * Displays all published courses for members
	 */

	import { CourseCard } from '#lib/components/courses/index.js';
	import type { PageData } from './$types';

	interface Course {
		id: string;
		title: string;
		slug: string;
		card_image_url?: string;
		card_description?: string;
		card_badge?: string;
		card_badge_color?: string;
		instructor_name?: string;
		instructor_avatar_url?: string;
		level?: string;
		price_cents: number;
		is_free?: boolean;
		lesson_count?: number;
		total_duration_minutes?: number;
		avg_rating?: number;
		review_count?: number;
	}

	interface Props {
		data: PageData;
	}

	let props: Props = $props();

	let levelFilter = $state('all');
	let sortBy = $state('newest');

	const courses = $derived.by(() => {
		const initial: Course[] = props.data?.courses ?? [];
		const filtered =
			levelFilter === 'all' ? initial : initial.filter((c) => c.level === levelFilter);
		return [...filtered];
	});
	const loading = false;
</script>

<div class="courses-page">
	<header class="page-header">
		<div class="header-content">
			<h1>Trading Classes</h1>
			<p>Master the markets with our comprehensive trading courses</p>
		</div>
	</header>

	<div class="filters-bar">
		<div class="filter-group">
			<label for="level">Level</label>
			<select id="level" bind:value={levelFilter}>
				<option value="all">All Levels</option>
				<option value="Beginner">Beginner</option>
				<option value="Intermediate">Intermediate</option>
				<option value="Advanced">Advanced</option>
			</select>
		</div>
		<div class="filter-group">
			<label for="sort">Sort By</label>
			<select id="sort" bind:value={sortBy}>
				<option value="newest">Newest First</option>
				<option value="popular">Most Popular</option>
				<option value="rating">Highest Rated</option>
			</select>
		</div>
	</div>

	{#if loading}
		<div class="loading">
			<div class="spinner"></div>
			<p>Loading courses...</p>
		</div>
	{:else if courses.length === 0}
		<div class="empty-state">
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="64"
				height="64"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="1.5"
				><path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20" /></svg
			>
			<h2>Featured Trading Classes &amp; Curriculum</h2>
			<p>
				Master institutional-grade market structure, disciplined risk management, and repeatable
				execution workflows across day trading, swing trading, and SPX options strategies. Every
				class combines structured video modules, downloadable checklists, and real-market case
				studies led by our senior trading desk.
			</p>
			<div class="featured-classes-links">
				<a href="/classes/quickstart-precision-trading-c"
					>Quickstart Precision Trading Class — Price-Action &amp; Execution</a
				>
				<a href="/classes/tax-loss-harvest-c"
					>Strategic Tax-Loss Harvesting Class — Portfolio Optimization</a
				>
				<a href="/courses/day-trading-masterclass"
					>Day Trading Masterclass — Intraday Tape &amp; Order Flow</a
				>
				<a href="/courses/swing-trading-pro"
					>Swing Trading Pro — Multi-Day Momentum &amp; Position Sizing</a
				>
				<a href="/courses/options-trading">Options Trading Mastery — Greeks, Spreads &amp; Volatility</a>
				<a href="/courses/risk-management">Risk Management &amp; Capital Preservation Framework</a>
			</div>
		</div>
	{:else}
		<section class="catalog-section">
			<div class="catalog-intro">
				<h2>Curriculum Overview &amp; Active Trading Tracks</h2>
				<p>
					Every Revolution Trading Pros class is structured around institutional market structure,
					multi-timeframe order flow, disciplined position sizing, and repeatable execution
					checklists. Select a foundational or strategy-focused class below to review the syllabus,
					instructor breakdown, and on-demand video modules.
				</p>
			</div>
			<div class="courses-grid">
				{#each courses as course, index (course.id)}
					<CourseCard {course} loading={index < 3 ? 'eager' : 'lazy'} />
				{/each}
			</div>
		</section>
	{/if}
</div>

<style>
	.courses-page {
		background: #f9fafb;
	}

	.page-header {
		background: linear-gradient(135deg, #143e59 0%, #1e5a7a 100%);
		padding: 64px 24px;
		text-align: center;
		color: #fff;
	}

	.header-content {
		max-width: 800px;
		margin: 0 auto;
	}

	.page-header h1 {
		font-size: 40px;
		font-weight: 700;
		margin: 0 0 12px;
	}

	.page-header p {
		font-size: 18px;
		opacity: 0.9;
		margin: 0;
	}

	.filters-bar {
		display: flex;
		justify-content: center;
		gap: 24px;
		padding: 24px;
		background: #fff;
		border-bottom: 1px solid #e5e7eb;
	}

	.filter-group {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.filter-group label {
		font-size: 14px;
		font-weight: 500;
		color: #374151;
	}

	.filter-group select {
		padding: 8px 12px;
		border: 1px solid #d1d5db;
		border-radius: 6px;
		font-size: 14px;
		color: #111827;
		background: #fff;
		cursor: pointer;
	}

	.filter-group select:focus {
		outline: none;
		border-color: #143e59;
	}

	.loading,
	.empty-state {
		text-align: center;
		padding: 80px 24px;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid #e5e7eb;
		border-top-color: #143e59;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin: 0 auto 16px;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.loading p {
		color: #6b7280;
		font-size: 14px;
	}

	.empty-state svg {
		color: #d1d5db;
		margin-bottom: 16px;
	}

	.empty-state h2 {
		font-size: 20px;
		color: #1f2937;
		margin: 0 0 8px;
	}

	.empty-state p {
		color: #4b5563;
		max-width: 680px;
		margin: 0 auto 24px;
		line-height: 1.6;
	}

	.featured-classes-links {
		display: flex;
		flex-wrap: wrap;
		justify-content: center;
		gap: 12px;
		max-width: 840px;
		margin: 0 auto;
	}

	.featured-classes-links a {
		padding: 10px 16px;
		border-radius: 8px;
		background: #fff;
		border: 1px solid #d1d5db;
		color: #143e59;
		font-size: 14px;
		font-weight: 600;
		text-decoration: none;
	}

	.catalog-intro {
		max-width: 840px;
		margin: 32px auto 0;
		padding: 0 24px;
		text-align: center;
	}

	.catalog-intro h2 {
		font-size: 24px;
		font-weight: 700;
		color: #111827;
		margin: 0 0 8px;
	}

	.catalog-intro p {
		font-size: 16px;
		color: #4b5563;
		line-height: 1.6;
		margin: 0;
	}

	.courses-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
		gap: 24px;
		padding: 32px 24px;
		max-width: 1400px;
		margin: 0 auto;
	}

	@media (max-width: 768px) {
		.page-header {
			padding: 48px 16px;
		}

		.page-header h1 {
			font-size: 28px;
		}

		.page-header p {
			font-size: 16px;
		}

		.filters-bar {
			flex-direction: column;
			gap: 12px;
			align-items: stretch;
		}

		.filter-group {
			justify-content: space-between;
		}

		.courses-grid {
			grid-template-columns: 1fr;
			padding: 24px 16px;
		}
	}
</style>

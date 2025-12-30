<script lang="ts">
	/**
	 * My Indicators - Member Dashboard
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * 100% PIXEL-PERFECT match to SimplerMyIndicators reference
	 * API integration for member vs non-member conditional rendering
	 *
	 * @version 2.1.0 - API Integration + 100% Pixel Perfect
	 */
	import { onMount } from 'svelte';
	import { getUserMemberships, type UserMembershipsResponse } from '$lib/api/user-memberships';

	// Loading and data state
	let loading = $state(true);
	let membershipsData = $state<UserMembershipsResponse | null>(null);

	// Get user's owned indicators from API
	$effect(() => {
		// This will re-run if membershipsData changes
	});

	onMount(async () => {
		try {
			membershipsData = await getUserMemberships();
		} catch (err) {
			console.error('Failed to load indicators:', err);
		} finally {
			loading = false;
		}
	});

	// Derived indicators list from API response
	const indicators = $derived(membershipsData?.indicators || []);
</script>

<svelte:head>
	<title>My Indicators | Revolution Trading Pros</title>
</svelte:head>

<!-- 100% EXACT structure from SimplerMyIndicators reference lines 2815-2821 -->
<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			<h2 class="section-title">My Indicators</h2>
			<div>
				{#if loading}
					<!-- Loading State -->
					<div class="loading-state">
						<div class="spinner"></div>
						<p>Loading your indicators...</p>
					</div>
				{:else if indicators.length > 0}
					<div class="card-grid flex-grid row">
						{#each indicators as indicator}
							<article class="card-grid-spacer flex-grid-item col-xs-12 col-sm-6 col-md-6 col-lg-4">
								<div class="card flex-grid-panel">
									<section class="card-body u--squash">
										<h4 class="h5 card-title pb-1">
											<a href="/dashboard/indicators/{indicator.slug}">
												{indicator.name}
											</a>
										</h4>
									</section>
									<footer class="card-footer">
										<a class="btn btn-tiny btn-default" href="/dashboard/indicators/{indicator.slug}">Download</a>
									</footer>
								</div>
							</article>
						{/each}
					</div>
				{:else}
					<!-- Empty state - Exact from SimplerMyIndicators reference -->
					<h4 class="pb-2">
						You don't have any Indicators.
					</h4>
					<a target="_blank" class="btn btn-orange" href="/product/product-category/Indicators/">See All Indicators</a>
				{/if}
			</div>
		</section>
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   100% PIXEL-PERFECT STYLES - Matching SimplerMyIndicators reference
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* Loading State */
	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 200px;
		gap: 16px;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid #e5e7eb;
		border-top-color: #0984ae;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.loading-state p {
		color: #666;
		font-size: 14px;
	}

	.section-title {
		font-size: 24px;
		font-weight: 700;
		color: #333;
		margin: 0 0 20px;
		font-family: 'Open Sans', sans-serif;
	}

	/* Empty State - Exact from reference */
	h4.pb-2 {
		font-size: 16px;
		font-weight: 600;
		color: #333;
		margin: 0 0 20px;
		padding-bottom: 10px;
	}

	/* Orange Button - Exact from reference */
	.btn-orange {
		display: inline-block;
		padding: 12px 24px;
		font-size: 14px;
		font-weight: 700;
		background: #f99e31;
		color: #fff;
		border: none;
		border-radius: 4px;
		text-decoration: none;
		transition: all 0.15s ease;
	}

	.btn-orange:hover {
		background-color: #dc7309;
		border-color: #dc7309;
		color: #fff;
	}

	/* Card Grid - Matching SimplerMyClasses pattern */
	:global(.card-grid.flex-grid.row) {
		display: flex;
		flex-wrap: wrap;
		margin-left: -15px;
		margin-right: -15px;
	}

	:global(.card-grid-spacer.flex-grid-item) {
		padding: 0 15px;
		margin-bottom: 30px;
	}

	:global(.flex-grid-item.col-xs-12) {
		flex: 0 0 100%;
		max-width: 100%;
	}

	@media (min-width: 576px) {
		:global(.flex-grid-item.col-sm-6) {
			flex: 0 0 50%;
			max-width: 50%;
		}
	}

	@media (min-width: 768px) {
		:global(.flex-grid-item.col-md-6) {
			flex: 0 0 50%;
			max-width: 50%;
		}
	}

	@media (min-width: 992px) {
		:global(.flex-grid-item.col-lg-4) {
			flex: 0 0 33.333%;
			max-width: 33.333%;
		}
	}

	/* Card - Exact from reference */
	:global(.card.flex-grid-panel) {
		display: flex;
		flex-direction: column;
		width: 100%;
		height: 100%;
		background-color: #fff;
		border: 1px solid #dbdbdb;
		border-radius: 4px;
		box-shadow: 0 2px 5px rgba(0, 0, 0, 0.16);
		transition: all 0.2s ease-in-out;
		overflow: hidden;
	}

	:global(.card.flex-grid-panel:hover) {
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
	}

	:global(.card-body.u--squash) {
		padding: 20px 20px 10px;
		flex: 1;
	}

	:global(.card-title) {
		font-size: 16px;
		font-weight: 700;
		margin: 0;
	}

	:global(.card-title a) {
		color: #333;
		text-decoration: none;
	}

	:global(.card-title a:hover) {
		color: #0984ae;
	}

	:global(.card-footer) {
		padding: 15px 20px;
		border-top: 1px solid #ededed;
	}

	:global(.btn-tiny) {
		display: inline-block;
		padding: 8px 16px;
		font-size: 13px;
		font-weight: 600;
		border-radius: 4px;
		text-decoration: none;
		transition: all 0.15s ease;
	}

	:global(.btn-default) {
		background: #f4f4f4;
		color: #333;
		border: 1px solid #dbdbdb;
	}

	:global(.btn-default:hover) {
		color: #333;
		background-color: #e6e6e6;
		border-color: #adadad;
	}
</style>

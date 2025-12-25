<script lang="ts">
	/**
	 * Day Trading Room - Trading Room Archives
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * 100% PIXEL-PERFECT match to SimplerTradingRoomArchive reference
	 * Date-grouped sessions with card-grid layout
	 *
	 * @version 2.0.0 - 100% Pixel Perfect
	 */

	// Archive data grouped by date - matching SimplerTradingRoomArchive reference
	const archivesByDate = [
		{
			date: 'December 5, 2025',
			sessions: [
				{ id: 1, title: 'Nail the Open', trader: 'John F. Carter', slug: '12052025' },
				{ id: 2, title: 'Macro Moves', trader: 'Sam Shames', slug: '12052025' },
				{ id: 3, title: 'Mastery Live Trading', trader: 'Jonathan McKeever', slug: '12052025' },
				{ id: 4, title: 'BIAS Trading Session', trader: 'Bruce Marshall', slug: '12052025' },
				{ id: 5, title: 'Cash In On The Close', trader: 'Allison Ostrander', slug: '12052025' }
			]
		},
		{
			date: 'December 4, 2025',
			sessions: [
				{ id: 1, title: 'Nail the Open', trader: 'John F. Carter', slug: '12042025' },
				{ id: 2, title: 'Macro Moves', trader: 'Sam Shames', slug: '12042025' },
				{ id: 3, title: 'Simpler Tech', trader: 'Lorna St. George', slug: '12042025' },
				{ id: 4, title: 'Swing Trading Options', trader: 'Taylor Horton', slug: '12042025' },
				{ id: 5, title: 'Cash In On The Close', trader: 'Henry Gambell', slug: '12042025' }
			]
		},
		{
			date: 'December 3, 2025',
			sessions: [
				{ id: 1, title: 'Nail the Open', trader: 'John F. Carter', slug: '12032025' },
				{ id: 2, title: 'Macro Moves', trader: 'Sam Shames', slug: '12032025' },
				{ id: 3, title: 'Moxie', trader: 'TG Watkins', slug: '12032025' },
				{ id: 4, title: '5 Star Method for Options', trader: 'Danielle Shay', slug: '12032025' },
				{ id: 5, title: 'Cash In On The Close', trader: 'Henry Gambell', slug: '12032025' }
			]
		},
		{
			date: 'December 2, 2025',
			sessions: [
				{ id: 1, title: 'Nail the Open', trader: 'Henry Gambell', slug: '12022025' },
				{ id: 2, title: 'Futures Set Ups', trader: 'Raghee Horner', slug: '12022025' },
				{ id: 3, title: 'Elliott Wave', trader: 'David Starr', slug: '12022025' },
				{ id: 4, title: 'Swing Trading Options', trader: 'Taylor Horton', slug: '12022025' },
				{ id: 5, title: 'Cash In On The Close', trader: 'Danielle Shay', slug: '12022025' }
			]
		},
		{
			date: 'December 1, 2025',
			sessions: [
				{ id: 1, title: 'Nail the Open', trader: 'Taylor Horton', slug: '12012025' },
				{ id: 2, title: 'Macro Moves', trader: 'Sam Shames', slug: '12012025' },
				{ id: 3, title: 'Day Trading Options', trader: 'Neil Yeager', slug: '12012025' },
				{ id: 4, title: 'BIAS Trading Session', trader: 'Bruce Marshall', slug: '12012025' },
				{ id: 5, title: 'Cash In On The Close', trader: 'Henry Gambell', slug: '12012025' }
			]
		},
		{
			date: 'November 28, 2025',
			sessions: [
				{ id: 1, title: 'Nail the Open', trader: 'John F. Carter', slug: '11282025' },
				{ id: 2, title: 'Macro Moves', trader: 'Sam Shames', slug: '11282025' },
				{ id: 3, title: 'Day Trading Options', trader: 'Jonathan McKeever', slug: '11282025' }
			]
		},
		{
			date: 'November 26, 2025',
			sessions: [
				{ id: 1, title: 'Nail the Open', trader: 'Taylor Horton', slug: '11262025' },
				{ id: 2, title: 'Macro Moves', trader: 'Sam Shames', slug: '11262025' },
				{ id: 3, title: 'Moxie', trader: 'TG Watkins', slug: '11262025' },
				{ id: 4, title: '5 Star Method for Options', trader: 'Danielle Shay', slug: '11262025' },
				{ id: 5, title: 'Cash In On The Close', trader: 'John F. Carter', slug: '11262025' }
			]
		}
	];
</script>

<svelte:head>
	<title>Trading Room Archives - Day Trading Room | Revolution Trading Pros</title>
</svelte:head>

<!-- 100% EXACT structure from SimplerTradingRoomArchive reference -->
<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			<h2 class="section-title">Day Trading Room Trading Room Archives</h2>
			<p></p>
			<div id="response">
				{#each archivesByDate as dateGroup}
					<p class="u--margin-bottom-20"><strong>{dateGroup.date}</strong></p>
					<div class="card-grid flex-grid row training_room">
						{#each dateGroup.sessions as session, index}
							<article class="flex-grid-item col-xs-12 col-sm-6 col-md-4 col-xl-3 tr_archive_item u--margin-bottom-20">
								<div class="card flex-grid-panel">
									<section class="card-body flex-grid mod-flex-content-between flex-grid-column">
										<div>
											<h4 style="margin-bottom: 5px;"><strong>{session.title}</strong></h4>
											<p class="u--margin-bottom-20 u--font-size-sm"><i>With {session.trader}</i></p>
										</div>
										<h4 class="h5 card-title u--margin-bottom-0">
											<a data-index={index + 1} class="btn btn-tiny btn-default" href="/chatroom-archive/day-trading-room/{session.slug}">
												Watch Now
											</a>
										</h4>
									</section>
								</div>
							</article>
						{/each}
					</div>
				{/each}
			</div>
		</section>
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   100% PIXEL-PERFECT STYLES - Matching SimplerTradingRoomArchive reference
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* Date heading - Exact from reference line 3020 */
	:global(.u--margin-bottom-20) {
		margin-bottom: 20px;
	}

	/* Card Grid - Exact from reference line 3021 */
	:global(.card-grid.flex-grid.row.training_room) {
		display: flex;
		flex-wrap: wrap;
		margin-left: -15px;
		margin-right: -15px;
		margin-bottom: 10px;
	}

	/* Archive Item - Exact from reference line 3022 */
	:global(.tr_archive_item) {
		padding: 0 15px;
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
		:global(.flex-grid-item.col-md-4) {
			flex: 0 0 33.333%;
			max-width: 33.333%;
		}
	}

	@media (min-width: 1200px) {
		:global(.flex-grid-item.col-xl-3) {
			flex: 0 0 25%;
			max-width: 25%;
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
		border-radius: 8px;
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.1);
		transition: all 0.2s ease-in-out;
		overflow: hidden;
	}

	:global(.card.flex-grid-panel:hover) {
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.15);
		transform: translateY(-2px);
	}

	/* Card Body - Exact from reference line 3001 */
	:global(.card-body.flex-grid.mod-flex-content-between.flex-grid-column) {
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		flex: 1 1 auto;
		padding: 15px;
	}

	/* Session Title - Exact from reference line 3002 */
	:global(.card-body h4) {
		margin: 0;
		font-size: 16px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		color: #333;
	}

	/* Trader Name - Exact from reference */
	:global(.u--font-size-sm) {
		font-size: 13px;
		color: #666;
		margin: 0;
	}

	/* Watch Now Button Container */
	:global(.card-title.u--margin-bottom-0) {
		margin-bottom: 0 !important;
		margin-top: auto;
	}

	/* Empty paragraph hidden */
	p:empty {
		display: none;
	}
</style>

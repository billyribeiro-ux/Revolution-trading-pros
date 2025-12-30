<script lang="ts">
	/**
	 * SIMPLER TRADING DASHBOARD - ISOLATED TEST PAGE
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * This is a completely isolated test page to render the Simpler Trading
	 * dashboard EXACTLY as it appears on simplertrading.com/dashboard
	 *
	 * NO external CSS - all styles are inline/scoped
	 * Once this matches 100%, we'll apply the fixes to the main dashboard.
	 *
	 * URL: /simpler-trading-test
	 */

	// Sample data for testing
	const user = {
		name: 'John Trader',
		email: 'john@example.com'
	};

	const memberships = [
		{ id: 1, name: 'Mastering the Trade', slug: 'mastering-the-trade', type: 'trading-room' },
		{ id: 2, name: 'Options Day Trading Room', slug: 'options-day-trading', type: 'trading-room' },
		{ id: 3, name: 'Simpler Showcase', slug: 'simpler-showcase', type: 'showcase' }
	];

	const tradingRooms = [
		{ id: 1, name: 'Day Trading Room', roomLabel: 'Day Trading Room' },
		{ id: 2, name: 'Options Room', roomLabel: 'Options Day Trading Room' },
		{ id: 3, name: 'Swing Trading Room', roomLabel: 'Swing Trading Room' }
	];

	let dropdownOpen = $state(false);
	let mobileMenuOpen = $state(false);

	function toggleDropdown(e: MouseEvent) {
		e.preventDefault();
		e.stopPropagation();
		dropdownOpen = !dropdownOpen;
	}

	function closeDropdown() {
		dropdownOpen = false;
	}

	function toggleMobileMenu() {
		mobileMenuOpen = !mobileMenuOpen;
	}
</script>

<svelte:head>
	<title>Simpler Trading Dashboard Test | Isolated</title>
	<link href="https://fonts.googleapis.com/css2?family=Open+Sans:wght@300;400;600;700;800&family=Open+Sans+Condensed:wght@700&display=swap" rel="stylesheet">
</svelte:head>

<svelte:window onclick={closeDropdown} />

<!-- ISOLATED WRAPPER - Resets everything -->
<div class="st-isolated">

	<!-- BREADCRUMBS -->
	<nav class="st-breadcrumbs">
		<div class="st-container-fluid">
			<ul>
				<li><a href="/">Home</a></li>
				<li class="st-separator"> / </li>
				<li><strong>Member Dashboard</strong></li>
			</ul>
		</div>
	</nav>

	<!-- DASHBOARD WRAPPER -->
	<div class="st-dashboard" class:menu-open={mobileMenuOpen}>

		<!-- SIDEBAR -->
		<aside class="st-sidebar">
			<nav class="st-nav-primary">

				<!-- Profile -->
				<a href="/dashboard/account/" class="st-profile-item">
					<span class="st-profile-photo"></span>
					<span class="st-profile-name">{user.name}</span>
				</a>

				<!-- Main Links -->
				<ul class="st-nav-links">
					<li class="is-active">
						<a href="/dashboard/">
							<span class="st-nav-icon">
								<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
									<path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"></path>
									<polyline points="9 22 9 12 15 12 15 22"></polyline>
								</svg>
							</span>
							<span class="st-nav-text">Member Dashboard</span>
						</a>
					</li>
					<li>
						<a href="/dashboard/classes/">
							<span class="st-nav-icon">
								<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
									<path d="M22 10v6M2 10l10-5 10 5-10 5z"></path>
									<path d="M6 12v5c3 3 9 3 12 0v-5"></path>
								</svg>
							</span>
							<span class="st-nav-text st-nav-text--bold">My Classes</span>
						</a>
					</li>
					<li>
						<a href="/dashboard/indicators/">
							<span class="st-nav-icon">
								<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
									<path d="M9 5H2v7l6.29 6.29c.94.94 2.48.94 3.42 0l3.58-3.58c.94-.94.94-2.48 0-3.42L9 5Z"></path>
									<path d="M6 9.01V9"></path>
									<path d="m15 5 6.3 6.3a2.4 2.4 0 0 1 0 3.4L17 19"></path>
								</svg>
							</span>
							<span class="st-nav-text st-nav-text--bold">My Indicators</span>
						</a>
					</li>
				</ul>

				<!-- Memberships Category -->
				<p class="st-nav-category">memberships</p>
				<ul class="st-nav-links">
					<li>
						<a href="/dashboard/day-trading-room/">
							<span class="st-nav-icon">
								<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
									<path d="M9 5v4"></path>
									<rect width="4" height="6" x="7" y="9" rx="1"></rect>
									<path d="M9 15v2"></path>
									<path d="M17 3v2"></path>
									<rect width="4" height="8" x="15" y="5" rx="1"></rect>
									<path d="M17 13v3"></path>
									<path d="M3 3v18h18"></path>
								</svg>
							</span>
							<span class="st-nav-text">Day Trading Room</span>
						</a>
					</li>
					<li>
						<a href="/dashboard/options-day-trading/">
							<span class="st-nav-icon">
								<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
									<path d="M9 5v4"></path>
									<rect width="4" height="6" x="7" y="9" rx="1"></rect>
									<path d="M9 15v2"></path>
									<path d="M17 3v2"></path>
									<rect width="4" height="8" x="15" y="5" rx="1"></rect>
									<path d="M17 13v3"></path>
									<path d="M3 3v18h18"></path>
								</svg>
							</span>
							<span class="st-nav-text">Options Day Trading</span>
						</a>
					</li>
				</ul>

				<!-- Tools Category -->
				<p class="st-nav-category">tools</p>
				<ul class="st-nav-links">
					<li>
						<a href="/dashboard/ww/">
							<span class="st-nav-icon">
								<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
									<path d="M8 2v4"></path>
									<path d="M16 2v4"></path>
									<rect width="18" height="18" x="3" y="4" rx="2"></rect>
									<path d="M3 10h18"></path>
									<path d="M8 14h.01"></path>
									<path d="M12 14h.01"></path>
									<path d="M16 14h.01"></path>
									<path d="M8 18h.01"></path>
									<path d="M12 18h.01"></path>
									<path d="M16 18h.01"></path>
								</svg>
							</span>
							<span class="st-nav-text">Weekly Watchlist</span>
						</a>
					</li>
					<li>
						<a href="https://intercom.help/simpler-trading/en/" target="_blank">
							<span class="st-nav-icon">
								<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
									<path d="M3 11h3a2 2 0 0 1 2 2v3a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-5Zm0 0a9 9 0 1 1 18 0m0 0v5a2 2 0 0 1-2 2h-1a2 2 0 0 1-2-2v-3a2 2 0 0 1 2-2h3Z"></path>
								</svg>
							</span>
							<span class="st-nav-text">Support</span>
						</a>
					</li>
				</ul>

				<!-- Account Category -->
				<p class="st-nav-category">account</p>
				<ul class="st-nav-links">
					<li>
						<a href="/dashboard/account/">
							<span class="st-nav-icon">
								<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
									<path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"></path>
									<circle cx="12" cy="12" r="3"></circle>
								</svg>
							</span>
							<span class="st-nav-text">My Account</span>
						</a>
					</li>
				</ul>

			</nav>
		</aside>

		<!-- MAIN CONTENT -->
		<main class="st-main">

			<!-- HEADER -->
			<header class="st-header">
				<div class="st-header-left">
					<h1 class="st-page-title">Member Dashboard</h1>
				</div>
				<div class="st-header-right">
					<ul class="st-trading-rules">
						<li class="st-rules-link"><a href="https://cdn.simplertrading.com/2024/02/07192341/Simpler-Tradings-Rules-of-the-Room.pdf" target="_blank">Trading Room Rules</a></li>
						<li class="st-rules-disclaimer">By logging into any of our Live Trading Rooms, You are agreeing to our Rules of the Room.</li>
					</ul>

					<div class="st-dropdown" class:is-open={dropdownOpen}>
						<a href="#trading-rooms" class="st-dropdown-toggle" onclick={toggleDropdown}>
							<strong>Enter a Trading Room</strong>
						</a>

						{#if dropdownOpen}
							<nav class="st-dropdown-menu">
								<ul>
									{#each tradingRooms as room (room.id)}
										<li>
											<a href="/dashboard/{room.name.toLowerCase().replace(/\s+/g, '-')}/" target="_blank">
												<span class="st-dropdown-icon">
													<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
														<path d="M9 5v4"></path>
														<rect width="4" height="6" x="7" y="9" rx="1"></rect>
														<path d="M9 15v2"></path>
														<path d="M17 3v2"></path>
														<rect width="4" height="8" x="15" y="5" rx="1"></rect>
														<path d="M17 13v3"></path>
														<path d="M3 3v18h18"></path>
													</svg>
												</span>
												{room.roomLabel}
											</a>
										</li>
									{/each}
								</ul>
							</nav>
						{/if}
					</div>
				</div>
			</header>

			<!-- CONTENT -->
			<div class="st-content">
				<div class="st-content-main">

					<!-- MEMBERSHIPS SECTION -->
					<section class="st-section">
						<h2 class="st-section-title">Memberships</h2>
						<div class="st-membership-cards st-row">
							{#each memberships as membership (membership.id)}
								<div class="st-col st-col-sm-6 st-col-xl-4">
									<article class="st-membership-card">
										<a href="/dashboard/{membership.slug}/" class="st-card-header">
											<span class="st-card-icon-wrap">
												<span class="st-card-icon" class:st-simpler-showcase={membership.slug === 'simpler-showcase'}>
													<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
														<path d="M9 5v4"></path>
														<rect width="4" height="6" x="7" y="9" rx="1"></rect>
														<path d="M9 15v2"></path>
														<path d="M17 3v2"></path>
														<rect width="4" height="8" x="15" y="5" rx="1"></rect>
														<path d="M17 13v3"></path>
														<path d="M3 3v18h18"></path>
													</svg>
												</span>
											</span>
											<span class="st-card-name">{membership.name}</span>
										</a>
										<div class="st-card-actions">
											<a href="/dashboard/{membership.slug}/">Dashboard</a>
											<a href="/dashboard/{membership.slug}/" target="_blank">Trading Room</a>
										</div>
									</article>
								</div>
							{/each}
						</div>
					</section>

					<!-- TOOLS SECTION -->
					<section class="st-section">
						<h2 class="st-section-title">Tools</h2>
						<div class="st-membership-cards st-row">
							<div class="st-col st-col-sm-6 st-col-xl-4">
								<article class="st-membership-card">
									<a href="/dashboard/ww/" class="st-card-header">
										<span class="st-card-icon-wrap">
											<span class="st-card-icon">
												<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
													<path d="M8 2v4"></path>
													<path d="M16 2v4"></path>
													<rect width="18" height="18" x="3" y="4" rx="2"></rect>
													<path d="M3 10h18"></path>
												</svg>
											</span>
										</span>
										<span class="st-card-name">Weekly Watchlist</span>
									</a>
									<div class="st-card-actions">
										<a href="/dashboard/ww/">Dashboard</a>
									</div>
								</article>
							</div>
						</div>
					</section>

					<!-- WEEKLY WATCHLIST FEATURED -->
					<section class="st-section st-section--featured">
						<div class="st-row">
							<div class="st-col st-col-sm-6 st-col-lg-5">
								<h2 class="st-section-title-alt">Weekly Watchlist</h2>
								<h4 class="st-featured-title">Weekly Watchlist with TG Watkins</h4>
								<p class="st-featured-text">Week of December 22, 2025.</p>
								<a href="/watchlist/latest" class="st-btn st-btn-default">Watch Now</a>
							</div>
							<div class="st-col st-col-sm-6 st-col-lg-7 st-hide-mobile">
								<a href="/watchlist/latest">
									<img src="https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/TG-Watchlist-Rundown.jpg" alt="Weekly Watchlist" class="st-featured-image" />
								</a>
							</div>
						</div>
					</section>

				</div>

				<!-- CONTENT SIDEBAR (empty on main dashboard) -->
				<aside class="st-content-sidebar">
					<!-- Empty on main dashboard -->
				</aside>
			</div>

		</main>

	</div>

	<!-- MOBILE TOGGLE -->
	<footer class="st-mobile-toggle">
		<button type="button" class="st-toggle-btn" onclick={toggleMobileMenu}>
			<div class="st-toggle-icon">
				<span></span>
				<span></span>
				<span></span>
			</div>
			<span class="st-toggle-label">Dashboard Menu</span>
		</button>
	</footer>

	<!-- MOBILE OVERLAY -->
	{#if mobileMenuOpen}
		<div class="st-overlay" onclick={toggleMobileMenu} role="button" tabindex="-1" aria-label="Close menu"></div>
	{/if}

</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   ISOLATED RESET - Completely separate from all other styles
	   ═══════════════════════════════════════════════════════════════════════════ */
	.st-isolated {
		all: initial;
		display: block;
		font-family: "Open Sans", sans-serif;
		font-size: 16px;
		line-height: 1.6;
		color: #666666;
		background-color: #efefef;
		min-height: 100vh;
		-webkit-font-smoothing: antialiased;
		-moz-osx-font-smoothing: grayscale;
	}

	.st-isolated *,
	.st-isolated *::before,
	.st-isolated *::after {
		box-sizing: border-box;
	}

	.st-isolated a {
		color: #1e73be;
		text-decoration: none;
	}

	.st-isolated a:hover {
		color: #000000;
	}

	.st-isolated ul {
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.st-isolated p {
		margin: 0 0 1em;
	}

	.st-isolated h1, .st-isolated h2, .st-isolated h3, .st-isolated h4 {
		margin: 0;
		font-weight: 700;
		color: #333333;
	}

	.st-isolated img {
		max-width: 100%;
		height: auto;
		display: block;
	}

	.st-isolated button {
		font-family: inherit;
		cursor: pointer;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   BREADCRUMBS - Exact Simpler Trading
	   ═══════════════════════════════════════════════════════════════════════════ */
	.st-breadcrumbs {
		background-color: #f5f5f5;
		border-bottom: 1px solid #e5e5e5;
	}

	.st-container-fluid {
		max-width: 100%;
		padding: 10px 15px;
	}

	.st-breadcrumbs ul {
		display: flex;
		align-items: center;
		font-size: 13px;
	}

	.st-breadcrumbs a {
		color: #666;
		transition: color 0.15s ease-in-out;
	}

	.st-breadcrumbs a:hover {
		color: #0984ae;
		text-decoration: underline;
	}

	.st-separator {
		margin: 0 8px;
		color: #999;
	}

	.st-breadcrumbs strong {
		color: #333;
		font-weight: 600;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD LAYOUT
	   ═══════════════════════════════════════════════════════════════════════════ */
	.st-dashboard {
		display: flex;
		flex-flow: row nowrap;
		min-height: calc(100vh - 39px);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SIDEBAR - Exact Simpler Trading #0e2433
	   ═══════════════════════════════════════════════════════════════════════════ */
	.st-sidebar {
		width: 280px;
		flex: 0 0 280px;
		background-color: #0e2433;
		min-height: 100%;
	}

	.st-nav-primary {
		width: 280px;
		padding-bottom: 30px;
		font-size: 16px;
		line-height: 1;
	}

	/* Profile Item */
	.st-profile-item {
		display: block;
		height: auto;
		line-height: 1.4;
		padding: 32px 20px 28px 80px;
		position: relative;
		color: hsla(0, 0%, 100%, 0.5);
		transition: all 0.15s ease-in-out;
	}

	.st-profile-item:hover {
		background: rgba(255, 255, 255, 0.05);
		color: #fff;
	}

	.st-profile-photo {
		position: absolute;
		left: 20px;
		top: 50%;
		transform: translateY(-50%);
		width: 44px;
		height: 44px;
		border-radius: 50%;
		border: 2px solid hsla(0, 0%, 100%, 0.2);
		background-color: #0f2d41;
		background-image: url('https://www.gravatar.com/avatar/?d=mp&s=88');
		background-size: cover;
		transition: border-color 0.15s ease-in-out;
	}

	.st-profile-item:hover .st-profile-photo {
		border-color: #0984ae;
	}

	.st-profile-name {
		display: block;
		font-size: 16px;
		font-weight: 700;
		color: #fff;
	}

	/* Category Headers */
	.st-nav-category {
		color: hsla(0, 0%, 100%, 0.3);
		font-size: 11px;
		font-weight: 700;
		letter-spacing: 0.5px;
		line-height: 1;
		margin: 0;
		padding: 20px 20px 12px;
		text-transform: uppercase;
	}

	/* Navigation Links */
	.st-nav-links {
		margin: 0;
		padding: 0;
	}

	.st-nav-links li {
		position: relative;
	}

	.st-nav-links a {
		display: flex;
		align-items: center;
		height: 50px;
		padding: 0 20px 0 80px;
		position: relative;
		color: hsla(0, 0%, 100%, 0.5);
		transition: all 0.15s ease-in-out;
	}

	.st-nav-links a:hover {
		background-color: rgba(255, 255, 255, 0.05);
		color: #fff;
	}

	/* Active State - Blue bar on right */
	.st-nav-links li.is-active a::after {
		content: "";
		position: absolute;
		top: 0;
		right: 0;
		bottom: 0;
		width: 5px;
		background-color: #0984ae;
	}

	.st-nav-links li.is-active a {
		color: #fff;
	}

	/* Nav Icons */
	.st-nav-icon {
		position: absolute;
		top: 50%;
		left: 30px;
		margin-top: -14px;
		width: 28px;
		height: 28px;
		color: inherit;
	}

	.st-nav-icon svg {
		width: 28px;
		height: 28px;
	}

	/* Nav Text */
	.st-nav-text {
		font-size: 17px;
		font-weight: 400;
		color: hsla(0, 0%, 100%, 0.7);
	}

	.st-nav-text--bold {
		font-weight: 700;
		color: #fff;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MAIN CONTENT
	   ═══════════════════════════════════════════════════════════════════════════ */
	.st-main {
		flex: 1 1 auto;
		min-width: 0;
		display: flex;
		flex-direction: column;
	}

	/* Header */
	.st-header {
		background-color: #fff;
		border-bottom: 1px solid #dbdbdb;
		padding: 20px;
		display: flex;
		flex-wrap: wrap;
		justify-content: space-between;
		align-items: center;
	}

	@media (min-width: 1280px) {
		.st-header { padding: 30px; }
	}

	@media (min-width: 1440px) {
		.st-header { padding: 30px 40px; }
	}

	.st-page-title {
		font-size: 36px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		color: #333;
		line-height: 1.2;
	}

	.st-header-right {
		display: flex;
		align-items: center;
		gap: 10px;
	}

	/* Trading Rules */
	.st-trading-rules {
		text-align: right;
	}

	.st-rules-link a {
		font-weight: 700;
		font-size: 12px;
		color: #0984ae;
	}

	.st-rules-link a:hover {
		text-decoration: underline;
		color: #065a75;
	}

	.st-rules-disclaimer {
		font-size: 11px;
		color: #666;
	}

	/* Dropdown */
	.st-dropdown {
		position: relative;
		display: inline-block;
	}

	.st-dropdown-toggle {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		background: #F69532;
		color: #fff;
		border: none;
		border-radius: 4px;
		padding: 12px 18px;
		font-size: 14px;
		font-weight: 700;
		width: 280px;
		justify-content: center;
		box-shadow: 0 2px 5px rgba(0, 0, 0, 0.16);
		transition: all 0.2s ease-in-out;
	}

	.st-dropdown-toggle:hover {
		background: #dc7309;
		color: #fff;
	}

	.st-dropdown-toggle::after {
		content: "";
		display: inline-block;
		margin-left: 0.255em;
		border-top: 0.3em solid;
		border-right: 0.3em solid transparent;
		border-left: 0.3em solid transparent;
	}

	.st-dropdown-menu {
		position: absolute;
		top: 100%;
		right: 0;
		margin-top: 5px;
		background: #fff;
		border-radius: 4px;
		box-shadow: 0 10px 30px rgba(0, 0, 0, 0.15);
		min-width: 260px;
		padding: 20px;
		z-index: 1000;
	}

	.st-dropdown-menu ul {
		margin: -10px;
	}

	.st-dropdown-menu a {
		display: flex;
		align-items: center;
		padding: 10px 15px;
		color: #666;
		font-size: 14px;
		transition: all 0.15s ease-in-out;
	}

	.st-dropdown-menu a:hover {
		background-color: #f4f4f4;
		color: #0984ae;
	}

	.st-dropdown-icon {
		width: 24px;
		height: 24px;
		margin-right: 10px;
		color: #999;
		transition: color 0.15s ease-in-out;
	}

	.st-dropdown-menu a:hover .st-dropdown-icon {
		color: #0984ae;
	}

	.st-dropdown-icon svg {
		width: 20px;
		height: 20px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CONTENT AREA
	   ═══════════════════════════════════════════════════════════════════════════ */
	.st-content {
		display: flex;
		flex-flow: row nowrap;
		flex: 1;
	}

	.st-content-main {
		flex: 1 1 auto;
		min-width: 0;
		background-color: #efefef;
		border-right: 1px solid #dbdbdb;
	}

	.st-content-sidebar {
		display: none;
		width: 260px;
		flex: 0 0 260px;
		background: #fff;
	}

	/* Sections */
	.st-section {
		padding: 30px 40px;
		background-color: #fff;
		margin-bottom: 0;
	}

	.st-section + .st-section {
		border-top: 1px solid #dbdbdb;
	}

	.st-section-title {
		color: #333;
		font-size: 32px;
		font-weight: 700;
		margin: 0 0 30px;
		font-family: 'Open Sans', sans-serif;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   GRID SYSTEM
	   ═══════════════════════════════════════════════════════════════════════════ */
	.st-row {
		display: flex;
		flex-wrap: wrap;
		margin: -15px;
	}

	.st-col {
		padding: 15px;
		width: 100%;
	}

	@media (min-width: 576px) {
		.st-col-sm-6 { width: 50%; }
	}

	@media (min-width: 992px) {
		.st-col-lg-5 { width: 41.666667%; }
		.st-col-lg-7 { width: 58.333333%; }
	}

	@media (min-width: 1200px) {
		.st-col-xl-4 { width: 33.333333%; }
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MEMBERSHIP CARDS
	   ═══════════════════════════════════════════════════════════════════════════ */
	.st-membership-cards {
		margin-top: -15px;
	}

	.st-membership-card {
		background-color: #fff;
		border: 1px solid #dbdbdb;
		border-radius: 8px;
		box-shadow: 0 2px 5px rgba(0, 0, 0, 0.16);
		transition: all 0.2s ease-in-out;
		overflow: hidden;
	}

	.st-membership-card:hover {
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
		transform: translateY(-2px);
	}

	.st-card-header {
		display: flex;
		align-items: center;
		padding: 20px;
		color: #333;
		font-weight: 600;
		font-size: 15px;
		transition: all 0.15s ease-in-out;
	}

	.st-card-header:hover {
		background-color: #f4f4f4;
		color: #333;
	}

	.st-card-icon-wrap {
		flex-shrink: 0;
		margin-right: 12px;
	}

	.st-card-icon {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 50px;
		height: 50px;
		background-color: #0984ae;
		border-radius: 50%;
		color: #fff;
		box-shadow: 0 10px 20px rgba(9, 132, 174, 0.25);
	}

	.st-card-icon svg {
		width: 24px;
		height: 24px;
	}

	/* Simpler Showcase special styling */
	.st-simpler-showcase {
		background: black !important;
		color: orange !important;
	}

	.st-card-name {
		flex: 1;
		white-space: normal;
		line-height: 1.4;
	}

	.st-card-actions {
		display: flex;
		border-top: 1px solid #ededed;
	}

	.st-card-actions a {
		flex: 1 1 auto;
		padding: 15px 10px;
		color: #0984ae;
		font-size: 14px;
		text-align: center;
		transition: all 0.15s ease-in-out;
	}

	.st-card-actions a:hover {
		background-color: #f4f4f4;
	}

	.st-card-actions a + a {
		border-left: 1px solid #ededed;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   WEEKLY WATCHLIST FEATURED SECTION
	   ═══════════════════════════════════════════════════════════════════════════ */
	.st-section--featured {
		background-color: #fff;
	}

	.st-section-title-alt {
		color: #0984ae;
		font-size: 11px;
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 1px;
		margin: 0 0 15px;
		padding-bottom: 30px;
		position: relative;
	}

	.st-section-title-alt::after {
		content: "";
		position: absolute;
		bottom: 2px;
		left: 0;
		width: 50px;
		height: 2px;
		background-color: #e8e8e8;
	}

	.st-featured-title {
		font-size: 16px;
		font-weight: 700;
		margin: 0 0 10px;
	}

	.st-featured-text {
		font-size: 14px;
		color: #666;
		margin: 0 0 15px;
	}

	.st-featured-image {
		border-radius: 8px;
	}

	.st-hide-mobile {
		display: none;
	}

	@media (min-width: 992px) {
		.st-hide-mobile { display: block; }
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   BUTTONS
	   ═══════════════════════════════════════════════════════════════════════════ */
	.st-btn {
		display: inline-block;
		padding: 6px 12px;
		font-size: 14px;
		font-weight: 400;
		text-align: center;
		border: 1px solid transparent;
		border-radius: 4px;
		transition: all 0.15s ease-in-out;
	}

	.st-btn-default {
		background: transparent;
		color: #F3911B;
		font-size: 17px;
		font-weight: 700;
		padding-left: 0;
		border: none;
	}

	.st-btn-default:hover {
		background: #e7e7e7;
		padding-left: 8px;
		color: #F3911B;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MOBILE TOGGLE
	   ═══════════════════════════════════════════════════════════════════════════ */
	.st-mobile-toggle {
		display: none;
		position: fixed;
		bottom: 0;
		left: 0;
		right: 0;
		height: 50px;
		background-color: #0d2532;
		z-index: 100010;
	}

	.st-toggle-btn {
		display: flex;
		align-items: center;
		width: 100%;
		height: 50px;
		padding: 0 20px 0 60px;
		background: none;
		border: none;
		color: #fff;
		position: relative;
	}

	.st-toggle-icon {
		position: absolute;
		left: 20px;
		top: 50%;
		margin-top: -7px;
		width: 20px;
		height: 14px;
	}

	.st-toggle-icon span {
		position: absolute;
		left: 0;
		width: 20px;
		height: 2px;
		background-color: #fff;
		transition: all 0.2s ease-in-out;
	}

	.st-toggle-icon span:nth-child(1) { top: 0; }
	.st-toggle-icon span:nth-child(2) { top: 6px; }
	.st-toggle-icon span:nth-child(3) { top: 12px; }

	/* Hamburger to X animation */
	.st-dashboard.menu-open .st-toggle-icon span:nth-child(1) {
		top: -1px;
		left: 3px;
		transform: rotate(45deg);
	}

	.st-dashboard.menu-open .st-toggle-icon span:nth-child(2) {
		opacity: 0;
		width: 0;
	}

	.st-dashboard.menu-open .st-toggle-icon span:nth-child(3) {
		top: 13px;
		left: 3px;
		transform: rotate(-45deg);
	}

	.st-toggle-label {
		font-size: 12px;
		text-transform: uppercase;
	}

	/* Overlay */
	.st-overlay {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background-color: rgba(0, 0, 0, 0.65);
		z-index: 100009;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE - Mobile
	   ═══════════════════════════════════════════════════════════════════════════ */
	@media (max-width: 1279px) {
		.st-sidebar {
			position: fixed;
			top: 0;
			left: 0;
			bottom: 50px;
			z-index: 100010;
			opacity: 0;
			visibility: hidden;
			transform: translateX(-100%);
			transition: all 0.3s ease-in-out;
		}

		.st-dashboard.menu-open .st-sidebar {
			opacity: 1;
			visibility: visible;
			transform: translateX(0);
		}

		.st-mobile-toggle {
			display: block;
		}

		.st-isolated {
			padding-bottom: 50px;
		}

		.st-header {
			flex-direction: column;
			align-items: flex-start;
			gap: 15px;
		}

		.st-header-right {
			flex-direction: column;
			align-items: flex-start;
			width: 100%;
		}

		.st-trading-rules {
			text-align: left;
		}

		.st-dropdown-toggle {
			width: 100%;
		}
	}

	@media (max-width: 767px) {
		.st-section {
			padding: 20px;
		}

		.st-page-title {
			font-size: 28px;
		}

		.st-section-title {
			font-size: 24px;
		}
	}
</style>

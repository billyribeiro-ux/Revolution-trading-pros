<script lang="ts">
	/**
	 * Dashboard Layout - VISUAL SHELL ONLY
	 * No functionality, just structure matching WordPress
	 */
	import { NavBar } from '$lib/components/nav';
	import Footer from '$lib/components/sections/Footer.svelte';
	import type { Snippet } from 'svelte';

	let { children }: { children: Snippet } = $props();
</script>

<svelte:head>
	<title>Dashboard | Revolution Trading Pros</title>
</svelte:head>

<!-- PAGE WRAPPER -->
<div class="dashboard-page">

	<!-- NAVBAR -->
	<NavBar />

	<!-- BREADCRUMB -->
	<nav class="breadcrumbs">
		<div class="breadcrumbs__container">
			<span><a href="/">Home</a></span>
			<span class="separator">/</span>
			<span><strong>Member Dashboard</strong></span>
			<span style="color: red; margin-left: 20px; font-weight: bold;">*** NEW BUILD v2 ***</span>
		</div>
	</nav>

	<!-- DASHBOARD -->
	<div class="dashboard">

		<!-- SIDEBAR -->
		<aside class="dashboard__sidebar">
			<nav class="dashboard__nav-primary">

				<!-- Profile -->
				<a href="/dashboard/account" class="dashboard__profile">
					<span class="dashboard__profile-photo"></span>
					<span class="dashboard__profile-name">John Doe</span>
				</a>

				<!-- Main Links -->
				<ul class="dashboard__nav-list">
					<li class="is-active">
						<a href="/dashboard">
							<span class="dashboard__nav-icon">🏠</span>
							<span>Member Dashboard</span>
						</a>
					</li>
					<li>
						<a href="/dashboard/classes">
							<span class="dashboard__nav-icon">📹</span>
							<span>My Classes</span>
						</a>
					</li>
					<li>
						<a href="/dashboard/indicators">
							<span class="dashboard__nav-icon">📊</span>
							<span>My Indicators</span>
						</a>
					</li>
				</ul>

				<!-- Memberships -->
				<p class="dashboard__nav-category">MEMBERSHIPS</p>
				<ul class="dashboard__nav-list">
					<li>
						<a href="/dashboard/mastering-the-trade">
							<span class="dashboard__nav-icon">👥</span>
							<span>Mastering the Trade</span>
						</a>
					</li>
					<li>
						<a href="/dashboard/simpler-showcase">
							<span class="dashboard__nav-icon">👥</span>
							<span>Simpler Showcase</span>
						</a>
					</li>
				</ul>

				<!-- Tools -->
				<p class="dashboard__nav-category">TOOLS</p>
				<ul class="dashboard__nav-list">
					<li>
						<a href="/dashboard/watchlist">
							<span class="dashboard__nav-icon">📋</span>
							<span>Weekly Watchlist</span>
						</a>
					</li>
					<li>
						<a href="/dashboard/support">
							<span class="dashboard__nav-icon">🎧</span>
							<span>Support</span>
						</a>
					</li>
				</ul>

				<!-- Account -->
				<p class="dashboard__nav-category">ACCOUNT</p>
				<ul class="dashboard__nav-list">
					<li>
						<a href="/dashboard/account">
							<span class="dashboard__nav-icon">⚙️</span>
							<span>My Account</span>
						</a>
					</li>
				</ul>

			</nav>
		</aside>

		<!-- MAIN CONTENT -->
		<main class="dashboard__main">
			{@render children()}
		</main>

	</div>

	<!-- FOOTER -->
	<Footer />
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   PAGE WRAPPER
	   ═══════════════════════════════════════════════════════════════════════════ */
	.dashboard-page {
		display: flex;
		flex-direction: column;
		min-height: 100vh;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   BREADCRUMBS
	   ═══════════════════════════════════════════════════════════════════════════ */
	.breadcrumbs {
		background: #f8f9fa;
		border-bottom: 1px solid #e5e7eb;
		padding: 12px 30px;
		font-size: 13px;
		font-family: 'Open Sans', sans-serif;
	}

	.breadcrumbs__container {
		max-width: 1700px;
		margin: 0 auto;
	}

	.breadcrumbs a {
		color: #6b7280;
		text-decoration: none;
	}

	.breadcrumbs a:hover {
		color: #0984ae;
		text-decoration: underline;
	}

	.breadcrumbs .separator {
		margin: 0 8px;
		color: #9ca3af;
	}

	.breadcrumbs strong {
		color: #333;
		font-weight: 600;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD CONTAINER
	   ═══════════════════════════════════════════════════════════════════════════ */
	.dashboard {
		display: flex;
		flex: 1;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SIDEBAR
	   ═══════════════════════════════════════════════════════════════════════════ */
	.dashboard__sidebar {
		width: 280px;
		flex-shrink: 0;
		background: #0f2d41;
	}

	.dashboard__nav-primary {
		padding-bottom: 30px;
	}

	/* Profile Section */
	.dashboard__profile {
		display: block;
		padding: 32px 20px 28px 80px;
		position: relative;
		text-decoration: none;
		transition: background 0.15s;
	}

	.dashboard__profile:hover {
		background: rgba(255,255,255,0.05);
	}

	.dashboard__profile-photo {
		position: absolute;
		left: 30px;
		top: 50%;
		margin-top: -17px;
		width: 34px;
		height: 34px;
		border: 2px solid #fff;
		border-radius: 50%;
		background: #1a3a4f;
	}

	.dashboard__profile-name {
		color: #fff;
		font-weight: 600;
		font-size: 16px;
	}

	/* Navigation List */
	.dashboard__nav-list {
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.dashboard__nav-list li a {
		display: flex;
		align-items: center;
		height: 50px;
		padding: 0 20px 0 80px;
		position: relative;
		color: hsla(0,0%,100%,0.5);
		text-decoration: none;
		font-size: 14px;
		font-weight: 300;
		transition: color 0.15s;
	}

	.dashboard__nav-list li a:hover {
		color: #fff;
	}

	.dashboard__nav-list li.is-active a {
		color: #fff;
	}

	.dashboard__nav-list li.is-active a::after {
		content: '';
		position: absolute;
		top: 0;
		right: 0;
		bottom: 0;
		width: 5px;
		background: #0984ae;
	}

	.dashboard__nav-icon {
		position: absolute;
		left: 30px;
		font-size: 20px;
	}

	/* Category Headers */
	.dashboard__nav-category {
		padding: 30px 30px 10px;
		color: hsla(0,0%,100%,0.7);
		text-transform: uppercase;
		font-size: 10px;
		font-weight: 600;
		letter-spacing: 0.5px;
		margin: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MAIN CONTENT
	   ═══════════════════════════════════════════════════════════════════════════ */
	.dashboard__main {
		flex: 1;
		background: #f4f4f4;
		min-height: 500px;
	}
</style>

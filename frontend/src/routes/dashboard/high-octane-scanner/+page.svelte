<!--
	High Octane Scanner Dashboard Page
	═══════════════════════════════════════════════════════════════════════════
	
	Displays the High Octane Scanner interface with breadcrumbs and navigation.
	
	Svelte 5 (Dec 2025):
	- $state() and $derived() runes
	- Proper SSR safety
	- Type-safe props
	
	@version 1.0.0
	@author Revolution Trading Pros
-->
<script lang="ts">
	// Breadcrumb data
	const breadcrumbs = [
		{ name: 'Dashboard', url: '/dashboard' },
		{ name: 'High Octane Scanner', url: '/dashboard/high-octane-scanner' }
	];
</script>

<svelte:head>
	<title>High Octane Scanner | Dashboard | Revolution Trading Pros</title>
	<meta name="description" content="Access your High Octane Scanner - Professional options scanning tool" />
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<div class="scanner-dashboard-page">
	<div class="container">
		<!-- Breadcrumb Navigation -->
		<nav aria-label="Breadcrumb" class="breadcrumb-nav">
			<ol>
				{#each breadcrumbs as crumb, index}
					<li>
						{#if index < breadcrumbs.length - 1}
							<a href={crumb.url}>{crumb.name}</a>
							<span class="separator" aria-hidden="true">›</span>
						{:else}
							<span aria-current="page">{crumb.name}</span>
						{/if}
					</li>
				{/each}
			</ol>
		</nav>

		<!-- Main Header -->
		<header class="page-header">
			<h1>High Octane Scanner</h1>
		</header>

		<!-- Scanner Content Area -->
		<section class="scanner-content">
			<div class="scanner-placeholder">
				<div class="placeholder-icon">
					<svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<path d="M3 3v18h18" />
						<path d="M18 17V9" />
						<path d="M13 17V5" />
						<path d="M8 17v-3" />
					</svg>
				</div>
				<h2>Scanner Interface Coming Soon</h2>
				<p>Your High Octane Scanner dashboard will be available here.</p>
				<div class="features-list">
					<ul>
						<li>✓ Real-time options scanning</li>
						<li>✓ Advanced filtering algorithms</li>
						<li>✓ Custom alert notifications</li>
						<li>✓ Historical data analysis</li>
					</ul>
				</div>
			</div>
		</section>
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * SCANNER DASHBOARD PAGE STYLES
	 * Apple ICT 11+ Standards - Modern CSS
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.scanner-dashboard-page {
		min-height: 100vh;
		background: linear-gradient(135deg, #f5f7fa 0%, #e8eef5 100%);
		padding: 40px 20px;
	}

	.container {
		max-width: 1200px;
		margin: 0 auto;
	}

	/* Breadcrumb Navigation */
	.breadcrumb-nav {
		margin-bottom: 32px;
	}

	.breadcrumb-nav ol {
		display: flex;
		flex-wrap: wrap;
		gap: 8px;
		list-style: none;
		padding: 0;
		margin: 0;
		font-size: 0.875rem;
		color: #64748b;
	}

	.breadcrumb-nav li {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.breadcrumb-nav a {
		color: #0984ae;
		text-decoration: none;
		transition: color 0.2s ease;
	}

	.breadcrumb-nav a:hover {
		color: #0a2335;
		text-decoration: underline;
	}

	.breadcrumb-nav .separator {
		color: #cbd5e1;
		font-weight: 300;
	}

	.breadcrumb-nav [aria-current="page"] {
		color: #0a2335;
		font-weight: 500;
	}

	/* Page Header */
	.page-header {
		margin-bottom: 48px;
	}

	.page-header h1 {
		font-size: 3rem;
		font-weight: 700;
		color: #0a2335;
		margin: 0;
		line-height: 1.2;
	}

	/* Scanner Content */
	.scanner-content {
		background: white;
		border-radius: 16px;
		box-shadow: 0 4px 6px rgba(0, 0, 0, 0.05), 0 10px 15px rgba(0, 0, 0, 0.1);
		padding: 60px 40px;
		min-height: 500px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.scanner-placeholder {
		text-align: center;
		max-width: 600px;
	}

	.placeholder-icon {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 120px;
		height: 120px;
		background: linear-gradient(135deg, #0984ae 0%, #0a2335 100%);
		border-radius: 50%;
		margin-bottom: 32px;
		color: white;
	}

	.scanner-placeholder h2 {
		font-size: 2rem;
		font-weight: 600;
		color: #0a2335;
		margin: 0 0 16px 0;
	}

	.scanner-placeholder p {
		font-size: 1.125rem;
		color: #64748b;
		margin: 0 0 32px 0;
	}

	.features-list {
		display: inline-block;
		text-align: left;
	}

	.features-list ul {
		list-style: none;
		padding: 0;
		margin: 0;
	}

	.features-list li {
		font-size: 1rem;
		color: #475569;
		padding: 8px 0;
		display: flex;
		align-items: center;
		gap: 8px;
	}

	/* Responsive Design */
	@media (width <= 768px) {
		.scanner-dashboard-page {
			padding: 20px 16px;
		}

		.page-header h1 {
			font-size: 2rem;
		}

		.scanner-content {
			padding: 40px 24px;
		}

		.scanner-placeholder h2 {
			font-size: 1.5rem;
		}

		.scanner-placeholder p {
			font-size: 1rem;
		}

		.placeholder-icon {
			width: 80px;
			height: 80px;
		}

		.placeholder-icon svg {
			width: 48px;
			height: 48px;
		}
	}

	/* Accessibility - Reduced Motion */
	@media (prefers-reduced-motion: reduce) {
		.breadcrumb-nav a {
			transition: none;
		}
	}

	/* Dark Mode Support */
	@media (prefers-color-scheme: dark) {
		.scanner-dashboard-page {
			background: linear-gradient(135deg, #1a1f2e 0%, #0f1419 100%);
		}

		.scanner-content {
			background: #1e293b;
			box-shadow: 0 4px 6px rgba(0, 0, 0, 0.3), 0 10px 15px rgba(0, 0, 0, 0.4);
		}

		.page-header h1 {
			color: #f1f5f9;
		}

		.scanner-placeholder h2 {
			color: #f1f5f9;
		}

		.scanner-placeholder p {
			color: #94a3b8;
		}

		.features-list li {
			color: #cbd5e1;
		}

		.breadcrumb-nav ol {
			color: #94a3b8;
		}

		.breadcrumb-nav [aria-current="page"] {
			color: #f1f5f9;
		}
	}
</style>

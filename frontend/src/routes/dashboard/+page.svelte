<!--
	Dashboard Home Page - Member Dashboard Landing
	Main dashboard page showing memberships and quick access

	@version 1.0.0
	@author Revolution Trading Pros
-->
<script lang="ts">
	// Dashboard home page content will be added incrementally
	// For now, this is a placeholder to test the sidebar layout
</script>

<div class="dashboard__content">
	<header class="dashboard__header">
		<h1 class="dashboard__page-title">Member Dashboard</h1>
	</header>

	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			<p class="dashboard__welcome">Welcome to your member dashboard.</p>
		</section>
	</div>
</div>

<style>
	/* Dashboard Content Container */
	.dashboard__content {
		padding: 30px;
		max-width: 1400px;
		margin: 0 auto;
	}

	/* Dashboard Header */
	.dashboard__header {
		margin-bottom: 30px;
		padding-bottom: 20px;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
	}

	/* Page Title */
	.dashboard__page-title {
		font-family: 'Montserrat', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
		font-size: 28px;
		font-weight: 700;
		color: #ffffff;
		margin: 0;
	}

	/* Main Content Area */
	.dashboard__content-main {
		display: grid;
		gap: 30px;
	}

	/* Content Section */
	.dashboard__content-section {
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.08);
		border-radius: 12px;
		padding: 30px;
	}

	/* Welcome Text */
	.dashboard__welcome {
		font-family: 'Open Sans', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
		font-size: 16px;
		color: rgba(255, 255, 255, 0.8);
		margin: 0;
	}

	/* Responsive */
	@media (max-width: 768px) {
		.dashboard__content {
			padding: 20px;
		}

		.dashboard__page-title {
			font-size: 24px;
		}

		.dashboard__content-section {
			padding: 20px;
		}
	}
</style>

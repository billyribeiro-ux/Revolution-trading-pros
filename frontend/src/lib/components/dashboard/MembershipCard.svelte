<script lang="ts">
	/**
	 * Membership Card Component
	 * Matches WordPress .membership-cards structure exactly
	 * Displays a user's active membership/subscription with Dashboard/Trading Room actions
	 */

	interface Props {
		id: string;
		name: string;
		type: 'trading-room' | 'alert-service' | 'course' | 'indicator';
		slug: string;
		icon?: string;
		iconClass?: string;
		dashboardUrl?: string;
		roomUrl?: string;
	}

	let { id, name, type, slug, icon, iconClass, dashboardUrl, roomUrl }: Props = $props();

	// Generate URLs based on type
	function getBasePath() {
		switch (type) {
			case 'trading-room': return '/live-trading-rooms';
			case 'alert-service': return '/alerts';
			case 'course': return '/dashboard/courses';
			case 'indicator': return '/dashboard/indicators';
			default: return '/dashboard';
		}
	}
	const basePath = getBasePath();
	
	const finalDashboardUrl = dashboardUrl || `${basePath}/${slug}/dashboard`;
	const finalRoomUrl = roomUrl || `${basePath}/${slug}`;

	// Get the second action label based on type
	function getSecondActionLabel() {
		switch (type) {
			case 'trading-room': return 'Trading Room';
			case 'alert-service': return 'Alerts';
			case 'course': return 'Course';
			case 'indicator': return 'Indicator';
			default: return 'View';
		}
	}

	// Get icon class based on slug
	function getIconClass() {
		if (iconClass) return iconClass;
		return `st-icon-${slug}`;
	}
</script>

<div class="membership-cards {slug}-card">
	<a href={finalRoomUrl} class="membership-card__header">
		<span class="mbrship-card_icn {getIconClass()}">
			{#if icon}
				<img src={icon} alt="" class="mbrship-card_icn-img" />
			{:else}
				<i class="fa fa-graduation-cap"></i>
			{/if}
		</span>
		<span class="membership-card__title">{name}</span>
	</a>
	<div class="membership-card__actions">
		<a href={finalDashboardUrl}>Dashboard</a>
		<a href={finalRoomUrl}>{getSecondActionLabel()}</a>
	</div>
</div>

<style>
	/* Membership Cards - Matches WordPress structure exactly */
	.membership-cards {
		margin-top: 30px;
		background: #fff;
		border-radius: 5px;
		box-shadow: 0 5px 30px rgb(0 0 0 / 10%);
	}

	.membership-cards .membership-card__header {
		display: flex;
		align-items: center;
		padding: 20px;
		color: #333;
		font-weight: 700;
		white-space: normal;
		transition: all 0.15s ease-in-out;
		text-decoration: none;
	}

	.membership-cards .membership-card__header:hover {
		color: #0e6ac4;
	}

	.membership-card__title {
		font-size: 17px;
		font-family: 'Open Sans', sans-serif;
	}

	/* Icon */
	.mbrship-card_icn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 50px;
		height: 50px;
		margin-right: 9px;
		line-height: 50px;
		color: #fff;
		text-align: center;
		border-radius: 50%;
		transition: all 0.15s ease-in-out;
		box-shadow: 0 10px 20px rgba(9, 132, 174, 0.25);
		background-color: #0984ae;
		flex-shrink: 0;
	}

	.mbrship-card_icn-img {
		width: 32px;
		height: 32px;
		object-fit: contain;
	}

	.mbrship-card_icn :global(i),
	.mbrship-card_icn :global(svg) {
		font-size: 24px;
		color: #fff;
	}

	.membership-cards .membership-card__header:hover .mbrship-card_icn {
		background-color: #076787;
	}

	/* Day Trading specific */
	:global(.day-trading-card) .mbrship-card_icn {
		box-shadow: 0 10px 20px rgba(243, 110, 27, 0.25);
	}

	/* Small Accounts specific */
	:global(.small-accounts-card) .mbrship-card_icn {
		box-shadow: 0 10px 20px rgba(0, 171, 175, 0.25);
	}

	/* SPX Profit Pulse specific */
	:global(.spx-profit-pulse-card) .mbrship-card_icn {
		box-shadow: 0 10px 20px rgba(12, 36, 52, 0.25);
	}

	/* Actions */
	.membership-card__actions {
		display: flex;
		font-size: 16px;
		border-top: 1px solid #ededed;
		justify-content: center;
	}

	.membership-card__actions a {
		display: block;
		flex: 0 0 auto;
		flex-basis: 50%;
		width: 50%;
		height: 100%;
		padding: 15px;
		text-align: center;
		color: #1e73be;
		font-weight: 400;
		font-size: 14px;
		text-decoration: none;
		transition: all 0.15s ease-in-out;
	}

	.membership-card__actions a:hover {
		background-color: #f4f4f4;
		color: #0984ae;
	}

	.membership-card__actions a + a {
		border-left: 1px solid #ededed;
	}
</style>

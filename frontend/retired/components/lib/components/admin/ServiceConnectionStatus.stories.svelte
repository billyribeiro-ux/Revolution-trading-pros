<script context="module">
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import ServiceConnectionStatus from './ServiceConnectionStatus.svelte';

	const { Story } = defineMeta({
		component: ServiceConnectionStatus,
		title: 'Admin/Connections/ServiceConnectionStatus',
		tags: ['autodocs'],
		argTypes: {
			feature: {
				control: 'select',
				options: [
					'payment',
					'analytics',
					'email',
					'crm',
					'forms',
					'seo',
					'behavior',
					'social',
					'ai',
					'monitoring'
				],
				description: 'Feature category to check connection status for'
			},
			variant: {
				control: 'select',
				options: ['card', 'banner', 'inline', 'badge', 'minimal'],
				description: 'Display variant of the connection status component'
			},
			showFeatures: {
				control: 'boolean',
				description: 'Show feature list in card variant'
			}
		}
	});
</script>

<!-- Default: Payment Card with Features -->
<Story
	name="Default - Payment Card"
	args={{
		feature: 'payment',
		variant: 'card',
		showFeatures: true
	}}
/>

<!-- All Variants for Payment -->
<Story
	name="Payment - Card Variant"
	args={{
		feature: 'payment',
		variant: 'card',
		showFeatures: true
	}}
/>

<Story
	name="Payment - Banner Variant"
	args={{
		feature: 'payment',
		variant: 'banner'
	}}
/>

<Story
	name="Payment - Inline Variant"
	args={{
		feature: 'payment',
		variant: 'inline'
	}}
/>

<Story
	name="Payment - Badge Variant"
	args={{
		feature: 'payment',
		variant: 'badge'
	}}
/>

<Story
	name="Payment - Minimal Variant"
	args={{
		feature: 'payment',
		variant: 'minimal'
	}}
/>

<!-- Different Features -->
<Story
	name="Analytics - Card"
	args={{
		feature: 'analytics',
		variant: 'card',
		showFeatures: true
	}}
/>

<Story
	name="Email - Card"
	args={{
		feature: 'email',
		variant: 'card',
		showFeatures: true
	}}
/>

<Story
	name="CRM - Card"
	args={{
		feature: 'crm',
		variant: 'card',
		showFeatures: true
	}}
/>

<!-- Interactive Example with Svelte 5 Runes -->
<Story name="Interactive - Switch Variants">
	<script>
		let currentVariant = $state('card');
		let currentFeature = $state('payment');

		const variants = ['card', 'banner', 'inline', 'badge', 'minimal'];
		const features = ['payment', 'analytics', 'email', 'crm', 'forms'];
	</script>

	<div style="padding: 2rem; background: #0d1117;">
		<div style="margin-bottom: 1.5rem;">
			<h3 style="color: #f0f6fc; margin-bottom: 1rem;">Feature</h3>
			<div style="display: flex; gap: 0.5rem; flex-wrap: wrap;">
				{#each features as feature}
					<button
						onclick={() => (currentFeature = feature)}
						style="
							padding: 0.5rem 1rem; 
							background: {currentFeature === feature ? '#635BFF' : '#1c2128'}; 
							color: {currentFeature === feature ? '#fff' : '#8b949e'};
							border: 1px solid {currentFeature === feature ? '#635BFF' : '#30363d'};
							border-radius: 0.5rem;
							cursor: pointer;
							text-transform: capitalize;
						"
					>
						{feature}
					</button>
				{/each}
			</div>
		</div>

		<div style="margin-bottom: 1.5rem;">
			<h3 style="color: #f0f6fc; margin-bottom: 1rem;">Variant</h3>
			<div style="display: flex; gap: 0.5rem; flex-wrap: wrap;">
				{#each variants as variant}
					<button
						onclick={() => (currentVariant = variant)}
						style="
							padding: 0.5rem 1rem; 
							background: {currentVariant === variant ? '#e6b800' : '#1c2128'}; 
							color: {currentVariant === variant ? '#0d1117' : '#8b949e'};
							border: 1px solid {currentVariant === variant ? '#e6b800' : '#30363d'};
							border-radius: 0.5rem;
							cursor: pointer;
							text-transform: capitalize;
							font-weight: {currentVariant === variant ? '600' : '400'};
						"
					>
						{variant}
					</button>
				{/each}
			</div>
		</div>

		<div style="margin-top: 2rem;">
			<ServiceConnectionStatus
				feature={currentFeature}
				variant={currentVariant}
				showFeatures={currentVariant === 'card'}
			/>
		</div>
	</div>
</Story>

<!-- All Features Comparison -->
<Story name="All Features - Card View">
	<div
		style="padding: 2rem; background: #0d1117; display: grid; grid-template-columns: repeat(auto-fit, minmax(400px, 1fr)); gap: 2rem;"
	>
		<ServiceConnectionStatus feature="payment" variant="card" showFeatures={true} />
		<ServiceConnectionStatus feature="analytics" variant="card" showFeatures={true} />
		<ServiceConnectionStatus feature="email" variant="card" showFeatures={true} />
		<ServiceConnectionStatus feature="crm" variant="card" showFeatures={true} />
		<ServiceConnectionStatus feature="forms" variant="card" showFeatures={true} />
		<ServiceConnectionStatus feature="seo" variant="card" showFeatures={true} />
	</div>
</Story>

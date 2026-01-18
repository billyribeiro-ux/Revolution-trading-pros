<script context="module">
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import ConnectionGate from './ConnectionGate.svelte';

	const { Story } = defineMeta({
		component: ConnectionGate,
		title: 'Admin/Connections/ConnectionGate',
		tags: ['autodocs'],
		argTypes: {
			feature: {
				control: 'select',
				options: ['payment', 'analytics', 'email', 'crm', 'forms'],
				description: 'Feature to check connection for'
			},
			variant: {
				control: 'select',
				options: ['card', 'banner', 'inline'],
				description: 'Display variant when disconnected'
			}
		}
	});
</script>

<!-- Disconnected State (Default) -->
<Story 
	name="Disconnected - Shows Connection Prompt"
	args={{
		feature: 'payment',
		variant: 'card',
		showFeatures: true
	}}
>
	<ConnectionGate feature="payment" variant="card" showFeatures={true}>
		{#snippet children()}
			<div style="padding: 2rem; background: #1c2128; border: 1px solid #30363d; border-radius: 0.75rem;">
				<h2 style="color: #f0f6fc; margin: 0;">This content is hidden</h2>
				<p style="color: #8b949e; margin-top: 0.5rem;">
					You shouldn't see this because payment is not connected in Storybook
				</p>
			</div>
		{/snippet}
	</ConnectionGate>
</Story>

<!-- Connected State (Simulated) -->
<Story name="Connected - Shows Content">
	<div style="padding: 2rem; background: #0d1117;">
		<p style="color: #8b949e; margin-bottom: 1rem; font-size: 0.875rem;">
			Note: In Storybook, connections are mocked as disconnected. 
			In your app, when connected, the children content would display.
		</p>
		
		<!-- Simulated connected state -->
		<div style="padding: 2rem; background: #1c2128; border: 1px solid #30363d; border-radius: 0.75rem;">
			<div style="display: flex; align-items: center; gap: 0.5rem; margin-bottom: 1rem;">
				<span style="color: #3fb950; font-size: 1.5rem;">✓</span>
				<h2 style="color: #f0f6fc; margin: 0;">Payment Connected</h2>
			</div>
			<p style="color: #8b949e; margin: 0;">
				This is the content that displays when the feature is connected.
			</p>
			<div style="margin-top: 1.5rem; padding: 1rem; background: #0d1117; border-radius: 0.5rem;">
				<p style="color: #f0f6fc; margin: 0; font-family: monospace; font-size: 0.875rem;">
					// Your protected content here<br/>
					// Subscription management UI<br/>
					// Payment processing forms<br/>
					// etc.
				</p>
			</div>
		</div>
	</div>
</Story>

<!-- Different Variants -->
<Story 
	name="Banner Variant - Disconnected"
	args={{
		feature: 'analytics',
		variant: 'banner'
	}}
>
	<ConnectionGate feature="analytics" variant="banner">
		{#snippet children()}
			<div style="padding: 2rem; background: #1c2128; border-radius: 0.75rem;">
				<h3 style="color: #f0f6fc;">Analytics Dashboard</h3>
				<p style="color: #8b949e;">Protected content</p>
			</div>
		{/snippet}
	</ConnectionGate>
</Story>

<Story 
	name="Inline Variant - Disconnected"
	args={{
		feature: 'email',
		variant: 'inline'
	}}
>
	<div style="padding: 2rem; background: #0d1117;">
		<h2 style="color: #f0f6fc; margin-bottom: 1rem;">Email Campaign Settings</h2>
		<ConnectionGate feature="email" variant="inline">
			{#snippet children()}
				<div style="padding: 1rem; background: #1c2128; border-radius: 0.5rem;">
					<p style="color: #8b949e; margin: 0;">Email configuration form would be here</p>
				</div>
			{/snippet}
		</ConnectionGate>
	</div>
</Story>

<!-- Custom Disconnected Content -->
<Story name="Custom Disconnected Message">
	<ConnectionGate feature="crm">
		{#snippet disconnected()}
			<div style="padding: 2rem; background: #1c2128; border: 2px dashed #30363d; border-radius: 0.75rem; text-align: center;">
				<span style="font-size: 3rem;">🔌</span>
				<h3 style="color: #f0f6fc; margin-top: 1rem;">Custom CRM Connection Required</h3>
				<p style="color: #8b949e; max-width: 500px; margin: 1rem auto;">
					This is a custom disconnected message. You can provide your own UI instead of the default ServiceConnectionStatus component.
				</p>
				<button style="
					margin-top: 1rem;
					padding: 0.75rem 1.5rem;
					background: #635BFF;
					color: white;
					border: none;
					border-radius: 0.5rem;
					cursor: pointer;
					font-weight: 600;
				">
					Connect CRM Now
				</button>
			</div>
		{/snippet}
		{#snippet children()}
			<div style="padding: 2rem; background: #1c2128; border-radius: 0.75rem;">
				<h3 style="color: #f0f6fc;">CRM Dashboard</h3>
			</div>
		{/snippet}
	</ConnectionGate>
</Story>

<!-- Multiple Features -->
<Story name="Multiple Gates - Different Features">
	<div style="padding: 2rem; background: #0d1117; display: flex; flex-direction: column; gap: 2rem;">
		<ConnectionGate feature="payment" variant="inline">
			{#snippet children()}
				<div style="padding: 1rem; background: #1c2128; border-radius: 0.5rem;">
					<h4 style="color: #f0f6fc; margin: 0;">Payment Section</h4>
				</div>
			{/snippet}
		</ConnectionGate>
		
		<ConnectionGate feature="analytics" variant="inline">
			{#snippet children()}
				<div style="padding: 1rem; background: #1c2128; border-radius: 0.5rem;">
					<h4 style="color: #f0f6fc; margin: 0;">Analytics Section</h4>
				</div>
			{/snippet}
		</ConnectionGate>
		
		<ConnectionGate feature="email" variant="inline">
			{#snippet children()}
				<div style="padding: 1rem; background: #1c2128; border-radius: 0.5rem;">
					<h4 style="color: #f0f6fc; margin: 0;">Email Section</h4>
				</div>
			{/snippet}
		</ConnectionGate>
	</div>
</Story>

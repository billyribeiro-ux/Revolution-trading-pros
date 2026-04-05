<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { cubicOut } from 'svelte/easing';
	import {
		Icon,
		IconActivity,
		IconArrowRight,
		IconCheck,
		IconLockSquare,
		IconServer
	} from '$lib/icons';
	// --- Interaction Logic ---
	let containerRef = $state<HTMLElement | null>(null);
	let mouse = $state({ x: 0, y: 0 });
	// ICT11+ Fix: Start false, set true in onMount to trigger in: transitions
	let isVisible = $state(false);

	// Mouse tracking for subtle lighting effects
	const handleMouseMove = (e: MouseEvent) => {
		if (!containerRef) return;
		const rect = containerRef.getBoundingClientRect();
		mouse.x = e.clientX - rect.left;
		mouse.y = e.clientY - rect.top;
	};

	// Heavy, expensive-feeling transition
	function heavySlide(_node: Element, { delay = 0, duration = 1000 }) {
		return {
			delay,
			duration,
			css: (t: number) => {
				const eased = cubicOut(t);
				return `opacity: ${eased}; transform: translateY(${(1 - eased) * 20}px);`;
			}
		};
	}

	// Trigger entrance animations when section scrolls into viewport
	let observer: IntersectionObserver | null = null;

	onMount(() => {
		if (!browser) {
			isVisible = true;
			return;
		}

		queueMicrotask(() => {
			if (!containerRef) {
				isVisible = true;
				return;
			}

			observer = new IntersectionObserver(
				(entries) => {
					if (entries[0]?.isIntersecting) {
						isVisible = true;
						observer?.disconnect();
					}
				},
				{ threshold: 0.1, rootMargin: '50px' }
			);

			observer.observe(containerRef);
		});

		return () => observer?.disconnect();
	});

	// Mock data for the background "Depth of Market" animation
	const marketDepth = Array(20)
		.fill(0)
		.map((_, i) => ({
			price: (4200 + i * 0.25).toFixed(2),
			size: Math.floor(Math.random() * 500)
		}));
</script>

<section
	bind:this={containerRef}
	onmousemove={handleMouseMove}
	class="cta-section"
	aria-label="Account Creation Terminal"
>
	<div class="cta-depth-bg">
		<div class="cta-depth-col cta-depth-bid">
			{#each [...marketDepth, ...marketDepth] as tick}
				<div class="cta-tick">
					<span>{tick.price}</span>
					<span class="cta-tick-dim">{tick.size}</span>
				</div>
			{/each}
		</div>
		<div class="cta-depth-col cta-depth-ask">
			{#each [...marketDepth, ...marketDepth] as tick}
				<div class="cta-tick cta-tick-end">
					<span class="cta-tick-dim">{tick.size}</span>
					<span>{tick.price}</span>
				</div>
			{/each}
		</div>
	</div>

	<div
		class="cta-spotlight"
		style="background: radial-gradient(800px circle at var(--x) var(--y), oklch(1 0 0 / 0.03), transparent 60%);"
	></div>

	<div class="cta-container">
		<div class="cta-center">
			{#if isVisible}
				<div in:heavySlide={{ delay: 0, duration: 1000 }} class="cta-status-row">
					<span class="cta-ping-wrap">
						<span class="cta-ping"></span>
						<span class="cta-ping-dot"></span>
					</span>
					<span class="cta-status-label">
						Market Status: <span class="cta-status-open">Open</span>
					</span>
				</div>

				<h2 in:heavySlide={{ delay: 100 }} class="cta-heading">
					Professional <br />
					<span class="cta-heading-muted">Execution</span> Only.
				</h2>

				<p in:heavySlide={{ delay: 200 }} class="cta-subtext">
					This is not a game. It is a business. Authenticate now to access institutional-grade
					alerts, live mentorship, and proprietary indicators.
				</p>

				<div in:heavySlide={{ delay: 300 }} class="cta-form-wrap">
					<div class="cta-form-glow"></div>

					<div class="cta-form-card">
						<div class="cta-form-bar">
							<div class="cta-form-bar-start">
								<Icon icon={IconLockSquare} size={14} class="cta-icon-amber" />
								<span class="cta-form-bar-label">Secure_Enclave_v4.2</span>
							</div>
							<div class="cta-form-bar-tls">TLS 1.3 ENCRYPTED</div>
						</div>

						<div class="cta-form-body">
							<div class="cta-field">
								<label for="email-access" class="cta-label">Identity / Email</label>
								<div class="cta-input-wrap">
									<input
										type="email"
										id="email-access"
										name="email"
										autocomplete="email"
										placeholder="trader@fund.com"
										class="cta-input"
									/>
									<div class="cta-cursor"></div>
								</div>
							</div>

							<a href="/signup" class="cta-submit">
								<div class="cta-submit-text">
									<span class="cta-submit-label">Action</span>
									<span class="cta-submit-title">EXECUTE ORDER</span>
								</div>
								<Icon icon={IconArrowRight} size={24} class="cta-submit-arrow" />
							</a>
						</div>

						<div class="cta-form-footer">
							<span>Cost Basis: $0.00</span>
							<span class="cta-route">
								<span class="cta-route-dot"></span>
								Route: DIRECT
							</span>
						</div>
					</div>
				</div>

				<div in:heavySlide={{ delay: 400 }} class="cta-stats">
					<div class="cta-stat">
						<div class="cta-stat-header">
							<Icon icon={IconServer} size={16} />
							<span class="cta-stat-label">Network</span>
						</div>
						<div class="cta-stat-value">Global Edge</div>
					</div>

					<div class="cta-stat">
						<div class="cta-stat-header">
							<Icon icon={IconActivity} size={16} />
							<span class="cta-stat-label">Latency</span>
						</div>
						<div class="cta-stat-value">&lt; 20ms</div>
					</div>

					<div class="cta-stat">
						<div class="cta-stat-header">
							<Icon icon={IconLockSquare} size={16} />
							<span class="cta-stat-label">Security</span>
						</div>
						<div class="cta-stat-value">AES-256</div>
					</div>

					<div class="cta-stat">
						<div class="cta-stat-header">
							<Icon icon={IconCheck} size={16} />
							<span class="cta-stat-label">Commitment</span>
						</div>
						<div class="cta-stat-value">Cancel Anytime</div>
					</div>
				</div>
			{/if}
		</div>
	</div>
</section>

<style>
	/* ─── Section ─── */
	.cta-section {
		position: relative;
		padding-block: 8rem;
		padding-inline: 1.5rem;
		background-color: oklch(0.05 0 0);
		overflow: hidden;
		border-block-start: 1px solid oklch(1 0 0 / 0.1);
	}

	/* ─── Depth Background ─── */
	.cta-depth-bg {
		position: absolute;
		inset: 0;
		pointer-events: none;
		opacity: 0.03;
		overflow: hidden;
		display: flex;
		justify-content: space-between;
		padding-inline: 2.5rem;
	}

	.cta-depth-col {
		display: flex;
		flex-direction: column;
		font-size: 0.625rem;
		font-family: var(--font-mono, monospace);
	}

	.cta-depth-bid {
		color: oklch(0.7 0.17 160);
		animation: scroll-up 20s linear infinite;
	}

	.cta-depth-ask {
		color: oklch(0.7 0.15 70);
		text-align: end;
		animation: scroll-down 20s linear infinite;
	}

	.cta-tick {
		display: flex;
		gap: 2rem;
		margin-block: 0.25rem;
	}
	.cta-tick-end {
		justify-content: flex-end;
	}
	.cta-tick-dim {
		opacity: 0.5;
	}

	/* ─── Spotlight ─── */
	.cta-spotlight {
		position: absolute;
		inset: 0;
		pointer-events: none;
		opacity: 0.3;
	}

	/* ─── Container ─── */
	.cta-container {
		position: relative;
		max-inline-size: 56rem;
		margin-inline: auto;
		z-index: 10;
	}

	.cta-center {
		text-align: center;
	}

	/* ─── Status Row ─── */
	.cta-status-row {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 0.75rem;
		margin-block-end: 2.5rem;
	}

	.cta-ping-wrap {
		position: relative;
		display: flex;
		block-size: 0.5rem;
		inline-size: 0.5rem;
	}
	.cta-ping {
		position: absolute;
		display: inline-flex;
		block-size: 100%;
		inline-size: 100%;
		border-radius: 50%;
		background-color: oklch(0.7 0.17 160);
		opacity: 0.75;
		animation: ping 1s cubic-bezier(0, 0, 0.2, 1) infinite;
	}
	.cta-ping-dot {
		position: relative;
		display: inline-flex;
		border-radius: 50%;
		block-size: 0.5rem;
		inline-size: 0.5rem;
		background-color: oklch(0.7 0.17 160);
	}

	.cta-status-label {
		font-size: 0.625rem;
		font-family: var(--font-mono, monospace);
		text-transform: uppercase;
		letter-spacing: 0.2em;
		color: oklch(0.55 0.01 265);
	}

	.cta-status-open {
		color: oklch(0.7 0.17 160);
	}

	/* ─── Heading ─── */
	.cta-heading {
		font-size: clamp(3rem, 6vw, 4.5rem);
		font-family: var(--font-serif, serif);
		color: oklch(1 0 0);
		margin-block-end: 2rem;
		letter-spacing: -0.02em;
		line-height: 0.95;
	}

	.cta-heading-muted {
		color: oklch(0.35 0.01 265);
	}

	.cta-subtext {
		font-size: var(--text-lg);
		color: oklch(0.55 0.01 265);
		font-weight: 300;
		line-height: 1.7;
		max-inline-size: 32rem;
		margin-inline: auto;
		margin-block-end: 4rem;
	}

	/* ─── Form ─── */
	.cta-form-wrap {
		position: relative;
		max-inline-size: 36rem;
		margin-inline: auto;
		perspective: 1000px;

		&:hover .cta-form-glow {
			opacity: 1;
		}
	}

	.cta-form-glow {
		position: absolute;
		inset: -0.25rem;
		background: linear-gradient(to right, oklch(0.6 0.15 70 / 0.2), oklch(0.6 0.17 160 / 0.2));
		border-radius: var(--radius-sm);
		opacity: 0;
		filter: blur(16px);
		transition: opacity 700ms;
	}

	.cta-form-card {
		position: relative;
		background-color: oklch(0.08 0 0);
		border: 1px solid oklch(1 0 0 / 0.1);
		padding: 0.25rem;
		box-shadow: 0 25px 50px -12px oklch(0 0 0 / 0.5);
	}

	.cta-form-bar {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding-inline: 1rem;
		padding-block: 0.5rem;
		background-color: oklch(0.06 0 0);
		border-block-end: 1px solid oklch(1 0 0 / 0.05);
	}

	.cta-form-bar-start {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.cta-form-bar-label {
		font-size: 0.625rem;
		font-family: var(--font-mono, monospace);
		text-transform: uppercase;
		color: oklch(0.55 0.01 265);
		letter-spacing: 0.1em;
	}

	.cta-form-bar-tls {
		font-size: 0.625rem;
		font-family: var(--font-mono, monospace);
		color: oklch(0.7 0.17 160);
	}

	.cta-form-body {
		padding: 1.5rem;
		text-align: start;

		@media (min-width: 768px) {
			padding: 2rem;
		}
	}

	.cta-field {
		margin-block-end: 1.5rem;
	}

	.cta-label {
		display: block;
		font-size: 0.625rem;
		font-family: var(--font-mono, monospace);
		text-transform: uppercase;
		color: oklch(0.55 0.01 265);
		letter-spacing: 0.1em;
		margin-block-end: 0.5rem;
	}

	.cta-input-wrap {
		position: relative;
	}

	.cta-input {
		inline-size: 100%;
		background-color: oklch(0.05 0 0);
		border: 1px solid oklch(1 0 0 / 0.1);
		color: oklch(1 0 0);
		font-family: var(--font-mono, monospace);
		font-size: var(--text-sm);
		padding-inline: 1rem;
		padding-block: 0.75rem;
		transition:
			border-color 200ms,
			box-shadow 200ms;

		&:focus {
			outline: none;
			border-color: oklch(0.6 0.15 70);
			box-shadow: 0 0 0 1px oklch(0.6 0.15 70);
		}

		&::placeholder {
			color: oklch(0.35 0.01 265);
		}
	}

	.cta-cursor {
		position: absolute;
		inset-inline-end: 0.75rem;
		inset-block-start: 50%;
		transform: translateY(-50%);
		inline-size: 0.375rem;
		block-size: 1rem;
		background-color: oklch(0.6 0.15 70);
		animation: pulse 1s ease-in-out infinite;
		pointer-events: none;
	}

	.cta-submit {
		display: flex;
		align-items: center;
		justify-content: space-between;
		inline-size: 100%;
		background-color: oklch(0.45 0.12 70);
		color: oklch(1 0 0);
		padding-inline: 1.5rem;
		padding-block: 1rem;
		transition: background-color 200ms;
		box-shadow: 0 4px 20px oklch(0.45 0.12 70 / 0.2);
		text-decoration: none;

		&:hover {
			background-color: oklch(0.52 0.14 70);
		}
	}

	.cta-submit-text {
		display: flex;
		flex-direction: column;
	}

	.cta-submit-label {
		font-size: var(--text-xs);
		font-family: var(--font-mono, monospace);
		text-transform: uppercase;
		letter-spacing: 0.1em;
		color: oklch(0.85 0.08 70 / 0.8);
	}

	.cta-submit-title {
		font-size: var(--text-lg);
		font-weight: var(--weight-bold);
		letter-spacing: 0.05em;
	}

	.cta-form-footer {
		padding-inline: 1rem;
		padding-block: 0.75rem;
		background-color: oklch(0.06 0 0);
		border-block-start: 1px solid oklch(1 0 0 / 0.05);
		display: flex;
		justify-content: space-between;
		align-items: center;
		font-size: 0.625rem;
		font-family: var(--font-mono, monospace);
		color: oklch(0.4 0.01 265);
	}

	.cta-route {
		display: flex;
		align-items: center;
		gap: 0.25rem;
	}

	.cta-route-dot {
		inline-size: 0.375rem;
		block-size: 0.375rem;
		border-radius: 50%;
		background-color: oklch(0.7 0.17 160);
	}

	/* ─── Stats ─── */
	.cta-stats {
		margin-block-start: 4rem;
		padding-block-start: 2rem;
		border-block-start: 1px solid oklch(1 0 0 / 0.05);
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 2rem;

		@media (min-width: 768px) {
			grid-template-columns: repeat(4, 1fr);
		}
	}

	.cta-stat {
		text-align: center;
		@media (min-width: 768px) {
			text-align: start;
		}
	}

	.cta-stat-header {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		color: oklch(0.55 0.01 265);
		margin-block-end: 0.25rem;

		@media (min-width: 768px) {
			justify-content: flex-start;
		}
	}

	.cta-stat-label {
		font-size: 0.625rem;
		font-family: var(--font-mono, monospace);
		text-transform: uppercase;
		letter-spacing: 0.1em;
	}

	.cta-stat-value {
		color: oklch(1 0 0);
		font-family: var(--font-serif, serif);
		font-size: var(--text-lg);
	}

	/* ─── Icon helpers (child override) ─── */
	:global(.cta-icon-amber) {
		color: oklch(0.6 0.15 70);
	}
	:global(.cta-submit-arrow) {
		transition: transform 200ms;
	}
	.cta-submit:hover :global(.cta-submit-arrow) {
		transform: translateX(0.25rem);
	}

	/* ─── Keyframes ─── */
	@keyframes scroll-up {
		0% {
			transform: translateY(0);
		}
		100% {
			transform: translateY(-50%);
		}
	}
	@keyframes scroll-down {
		0% {
			transform: translateY(-50%);
		}
		100% {
			transform: translateY(0);
		}
	}
	@keyframes ping {
		75%,
		100% {
			transform: scale(2);
			opacity: 0;
		}
	}
	@keyframes pulse {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0.4;
		}
	}
</style>

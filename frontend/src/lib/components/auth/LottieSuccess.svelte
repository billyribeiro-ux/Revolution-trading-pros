<script lang="ts">
	/**
	 * LottieSuccess - Premium Success Animation
	 * Netflix L11+ Principal Engineer Grade
	 *
	 * Animated success checkmark using Lottie
	 * Plays once on mount, triggers callback on complete
	 *
	 * @version 1.0.0
	 */
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';

	// lottie-web loaded dynamically - no default export

	interface Props {
		onComplete?: () => void;
		size?: number;
	}

	let { onComplete, size = 120 }: Props = $props();

	let containerRef = $state<HTMLDivElement | null>(null);
	let animationInstance: any = null;

	// Premium success checkmark animation data
	const successAnimationData = {
		v: '5.7.4',
		fr: 60,
		ip: 0,
		op: 60,
		w: 200,
		h: 200,
		nm: 'Success Check',
		ddd: 0,
		assets: [],
		layers: [
			{
				ddd: 0,
				ind: 1,
				ty: 4,
				nm: 'Checkmark',
				sr: 1,
				ks: {
					o: { a: 0, k: 100 },
					r: { a: 0, k: 0 },
					p: { a: 0, k: [100, 100, 0] },
					a: { a: 0, k: [0, 0, 0] },
					s: {
						a: 1,
						k: [
							{ i: { x: [0.2], y: [1] }, o: { x: [0.8], y: [0] }, t: 20, s: [0, 0, 100] },
							{ t: 35, s: [100, 100, 100] }
						]
					}
				},
				ao: 0,
				shapes: [
					{
						ty: 'gr',
						it: [
							{
								ind: 0,
								ty: 'sh',
								ks: {
									a: 0,
									k: {
										i: [[0, 0], [0, 0], [0, 0]],
										o: [[0, 0], [0, 0], [0, 0]],
										v: [[-30, 0], [-10, 20], [30, -25]],
										c: false
									}
								},
								nm: 'Path'
							},
							{
								ty: 'st',
								c: { a: 0, k: [1, 1, 1, 1] },
								o: { a: 0, k: 100 },
								w: { a: 0, k: 8 },
								lc: 2,
								lj: 2,
								nm: 'Stroke'
							},
							{
								ty: 'tm',
								s: { a: 0, k: 0 },
								e: {
									a: 1,
									k: [
										{ i: { x: [0.2], y: [1] }, o: { x: [0.8], y: [0] }, t: 25, s: [0] },
										{ t: 45, s: [100] }
									]
								},
								o: { a: 0, k: 0 },
								m: 1,
								nm: 'Trim Paths'
							},
							{
								ty: 'tr',
								p: { a: 0, k: [0, 5] },
								a: { a: 0, k: [0, 0] },
								s: { a: 0, k: [100, 100] },
								r: { a: 0, k: 0 },
								o: { a: 0, k: 100 }
							}
						],
						nm: 'Check Group'
					}
				],
				ip: 0,
				op: 60,
				st: 0
			},
			{
				ddd: 0,
				ind: 2,
				ty: 4,
				nm: 'Circle',
				sr: 1,
				ks: {
					o: { a: 0, k: 100 },
					r: { a: 0, k: 0 },
					p: { a: 0, k: [100, 100, 0] },
					a: { a: 0, k: [0, 0, 0] },
					s: {
						a: 1,
						k: [
							{ i: { x: [0.2], y: [1] }, o: { x: [0.8], y: [0] }, t: 0, s: [0, 0, 100] },
							{ t: 20, s: [100, 100, 100] }
						]
					}
				},
				ao: 0,
				shapes: [
					{
						ty: 'gr',
						it: [
							{
								d: 1,
								ty: 'el',
								s: { a: 0, k: [120, 120] },
								p: { a: 0, k: [0, 0] },
								nm: 'Ellipse'
							},
							{
								ty: 'fl',
								c: { a: 0, k: [0.133, 0.773, 0.369, 1] },
								o: { a: 0, k: 100 },
								nm: 'Fill'
							},
							{
								ty: 'tr',
								p: { a: 0, k: [0, 0] },
								a: { a: 0, k: [0, 0] },
								s: { a: 0, k: [100, 100] },
								r: { a: 0, k: 0 },
								o: { a: 0, k: 100 }
							}
						],
						nm: 'Circle Group'
					}
				],
				ip: 0,
				op: 60,
				st: 0
			},
			{
				ddd: 0,
				ind: 3,
				ty: 4,
				nm: 'Ring',
				sr: 1,
				ks: {
					o: {
						a: 1,
						k: [
							{ i: { x: [0.2], y: [1] }, o: { x: [0.8], y: [0] }, t: 15, s: [100] },
							{ t: 40, s: [0] }
						]
					},
					r: { a: 0, k: 0 },
					p: { a: 0, k: [100, 100, 0] },
					a: { a: 0, k: [0, 0, 0] },
					s: {
						a: 1,
						k: [
							{ i: { x: [0.2], y: [1] }, o: { x: [0.8], y: [0] }, t: 15, s: [100, 100, 100] },
							{ t: 40, s: [150, 150, 100] }
						]
					}
				},
				ao: 0,
				shapes: [
					{
						ty: 'gr',
						it: [
							{
								d: 1,
								ty: 'el',
								s: { a: 0, k: [120, 120] },
								p: { a: 0, k: [0, 0] },
								nm: 'Ellipse'
							},
							{
								ty: 'st',
								c: { a: 0, k: [0.133, 0.773, 0.369, 1] },
								o: { a: 0, k: 100 },
								w: { a: 0, k: 4 },
								lc: 2,
								lj: 2,
								nm: 'Stroke'
							},
							{
								ty: 'tr',
								p: { a: 0, k: [0, 0] },
								a: { a: 0, k: [0, 0] },
								s: { a: 0, k: [100, 100] },
								r: { a: 0, k: 0 },
								o: { a: 0, k: 100 }
							}
						],
						nm: 'Ring Group'
					}
				],
				ip: 0,
				op: 60,
				st: 0
			}
		]
	};

	onMount(() => {
		if (!browser || !containerRef) return;

		// Use IIFE for async lottie import - Svelte 5 pattern
		(async () => {
			// lottie-web uses named export, not default
			const lottieModule = await import('lottie-web');
			const lottie = lottieModule.default || lottieModule;

			animationInstance = lottie.loadAnimation({
				container: containerRef!,
				renderer: 'svg',
				loop: false,
				autoplay: true,
				animationData: successAnimationData
			});

			animationInstance.addEventListener('complete', () => {
				onComplete?.();
			});
		})();

		return () => {
			if (animationInstance) {
				animationInstance.destroy();
			}
		};
	});
</script>

<div class="lottie-container" bind:this={containerRef} style="width: {size}px; height: {size}px;">
</div>

<style>
	.lottie-container {
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.lottie-container :global(svg) {
		filter: drop-shadow(0 4px 20px rgba(34, 197, 94, 0.4));
	}
</style>

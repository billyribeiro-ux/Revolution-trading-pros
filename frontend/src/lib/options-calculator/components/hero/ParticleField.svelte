<script lang="ts">
	import { on } from 'svelte/events';
	import { prefersReducedMotion } from 'svelte/motion';
	import { devicePixelRatio } from 'svelte/reactivity/window';
	import type { Attachment } from 'svelte/attachments';

	const PARTICLE_COUNT = 100;
	const CONNECTION_DISTANCE = 120;
	const CONNECTION_DISTANCE_SQ = CONNECTION_DISTANCE ** 2;
	const POINTER_RADIUS = 150;
	const POINTER_RADIUS_SQ = POINTER_RADIUS ** 2;
	const POINTER_FORCE = 0.02;
	const OFFSCREEN = -9999;
	const MAX_PIXEL_RATIO = 2;
	const TAU = Math.PI * 2;

	// Links fade out with distance. Their opacity is quantised into LINK_LEVELS steps
	// (0.0125 apart — below what the eye can tell on this background), so each step is one
	// path and one stroke() per frame rather than one stroke() per linked pair.
	const LINK_LEVELS = 12;
	const LINK_MAX_OPACITY = 0.15;
	const LINK_STYLES = Array.from(
		{ length: LINK_LEVELS },
		(_, level) =>
			`rgba(180, 210, 255, ${(((level + 0.5) / LINK_LEVELS) * LINK_MAX_OPACITY).toFixed(4)})`
	);

	interface Particle {
		x: number;
		y: number;
		vx: number;
		vy: number;
		baseVx: number;
		baseVy: number;
		radius: number;
		/** Halo gradient centred on the origin; drawn translated to the particle. */
		glow: CanvasGradient;
		/** Core fill colour (the particle's opacity is fixed, so this is built once). */
		core: string;
	}

	// The canvas's CSS size, kept current by the (ResizeObserver-backed) dimension bindings.
	let width = $state(0);
	let height = $state(0);

	const initField: Attachment<HTMLCanvasElement> = (canvas) => {
		const ctx = canvas.getContext('2d');
		if (!ctx) return;

		// Plain copies of the field size and pixel ratio for the frame loop.
		let fieldWidth = 0;
		let fieldHeight = 0;
		let pixelRatio = 1;
		let particles: Particle[] = [];
		let pointerX = OFFSCREEN;
		let pointerY = OFFSCREEN;

		const seed = (): Particle[] =>
			Array.from({ length: PARTICLE_COUNT }, () => {
				const vx = (Math.random() - 0.5) * 0.4;
				const vy = (Math.random() - 0.5) * 0.4;
				const radius = 1 + Math.random() * 1.5;
				const opacity = 0.15 + Math.random() * 0.45;
				const glow = ctx.createRadialGradient(0, 0, 0, 0, 0, radius * 3);
				glow.addColorStop(0, `rgba(180, 210, 255, ${opacity * 0.4})`);
				glow.addColorStop(1, 'rgba(180, 210, 255, 0)');
				return {
					x: Math.random() * fieldWidth,
					y: Math.random() * fieldHeight,
					vx,
					vy,
					baseVx: vx,
					baseVy: vy,
					radius,
					glow,
					core: `rgba(200, 220, 255, ${opacity})`
				};
			});

		const step = () => {
			for (const p of particles) {
				const dx = pointerX - p.x;
				const dy = pointerY - p.y;
				const distSq = dx * dx + dy * dy;
				if (distSq > 0 && distSq < POINTER_RADIUS_SQ) {
					const dist = Math.sqrt(distSq);
					const force = (1 - dist / POINTER_RADIUS) * POINTER_FORCE;
					p.vx += (dx / dist) * force;
					p.vy += (dy / dist) * force;
				}
				// Relax back towards the particle's own drift.
				p.vx += (p.baseVx - p.vx) * 0.01;
				p.vy += (p.baseVy - p.vy) * 0.01;
				p.x += p.vx;
				p.y += p.vy;
				if (p.x < -10) p.x = fieldWidth + 10;
				if (p.x > fieldWidth + 10) p.x = -10;
				if (p.y < -10) p.y = fieldHeight + 10;
				if (p.y > fieldHeight + 10) p.y = -10;
			}
		};

		const draw = () => {
			ctx.setTransform(pixelRatio, 0, 0, pixelRatio, 0, 0);
			ctx.clearRect(0, 0, fieldWidth, fieldHeight);

			const links = Array.from({ length: LINK_LEVELS }, () => new Path2D());
			for (let i = 0; i < particles.length; i++) {
				const a = particles[i];
				for (let j = i + 1; j < particles.length; j++) {
					const b = particles[j];
					const dx = a.x - b.x;
					const dy = a.y - b.y;
					const distSq = dx * dx + dy * dy;
					if (distSq >= CONNECTION_DISTANCE_SQ) continue;
					const strength = 1 - Math.sqrt(distSq) / CONNECTION_DISTANCE;
					const path = links[Math.min(LINK_LEVELS - 1, Math.floor(strength * LINK_LEVELS))];
					path.moveTo(a.x, a.y);
					path.lineTo(b.x, b.y);
				}
			}
			ctx.lineWidth = 0.5;
			for (let level = 0; level < LINK_LEVELS; level++) {
				ctx.strokeStyle = LINK_STYLES[level];
				ctx.stroke(links[level]);
			}

			// Each particle is drawn at the origin of a translated transform, so its cached
			// gradient (built at the origin) lands on it without a new gradient per frame.
			for (const p of particles) {
				ctx.setTransform(pixelRatio, 0, 0, pixelRatio, p.x * pixelRatio, p.y * pixelRatio);
				ctx.fillStyle = p.glow;
				ctx.beginPath();
				ctx.arc(0, 0, p.radius * 3, 0, TAU);
				ctx.fill();

				ctx.fillStyle = p.core;
				ctx.beginPath();
				ctx.arc(0, 0, p.radius, 0, TAU);
				ctx.fill();
			}
		};

		// ── Pointer attraction (follows mouse and touch; released on lift or cancel) ──
		const releasePointer = () => {
			pointerX = OFFSCREEN;
			pointerY = OFFSCREEN;
		};
		const stopListening = [
			on(
				window,
				'pointermove',
				(event) => {
					const rect = canvas.getBoundingClientRect();
					pointerX = event.clientX - rect.left;
					pointerY = event.clientY - rect.top;
				},
				{ passive: true }
			),
			on(window, 'pointerup', releasePointer, { passive: true }),
			// A touch that turns into a scroll ends with pointercancel, not pointerup.
			on(window, 'pointercancel', releasePointer, { passive: true }),
			on(document.documentElement, 'pointerleave', releasePointer)
		];

		// Whether the canvas is on screen. A callback can carry several queued entries for
		// this one canvas, oldest first: only the last one is the current state.
		let visible = $state(false);
		const visibility = new IntersectionObserver((entries) => {
			const latest = entries.at(-1);
			if (latest) visible = latest.isIntersecting;
		});
		visibility.observe(canvas);

		// Drawing buffer: follows the bound CSS size and the device pixel ratio. The field
		// is seeded on the first real size; later resizes keep particles inside it. Resizing
		// clears the canvas, so repaint the still frame at once — while paused there is no
		// next frame to do it.
		$effect(() => {
			if (width <= 0 || height <= 0) return;
			pixelRatio = Math.min(devicePixelRatio.current ?? 1, MAX_PIXEL_RATIO);
			fieldWidth = width;
			fieldHeight = height;
			canvas.width = Math.round(fieldWidth * pixelRatio);
			canvas.height = Math.round(fieldHeight * pixelRatio);
			if (particles.length === 0) {
				particles = seed();
			} else {
				for (const p of particles) {
					p.x = Math.min(p.x, fieldWidth);
					p.y = Math.min(p.y, fieldHeight);
				}
			}
			draw();
		});

		// Animate only while on screen and motion is allowed; otherwise the last frame stands.
		$effect(() => {
			if (!visible || prefersReducedMotion.current) return;
			let frameId = requestAnimationFrame(function frame() {
				step();
				draw();
				frameId = requestAnimationFrame(frame);
			});
			return () => cancelAnimationFrame(frameId);
		});

		// The nested effects above (and their rAF) are torn down before this runs.
		return () => {
			for (const stop of stopListening) stop();
			visibility.disconnect();
		};
	};
</script>

<canvas
	{@attach initField}
	bind:clientWidth={width}
	bind:clientHeight={height}
	class="absolute inset-0 h-full w-full"
	aria-hidden="true"
></canvas>

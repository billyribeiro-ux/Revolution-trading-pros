<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';

	let canvasEl: HTMLCanvasElement | undefined = $state();

	onMount(() => {
		if (!browser || !canvasEl) return;

		const prefersReducedMotion = window.matchMedia(
			'(prefers-reduced-motion: reduce)'
		).matches;

		const ctx = canvasEl.getContext('2d');
		if (!ctx) return;

		let disposed = false;
		let animId = 0;
		let mouseX = -9999;
		let mouseY = -9999;

		// --- Particle setup ---
		const PARTICLE_COUNT = 100;
		const CONNECTION_DISTANCE = 120;
		const MOUSE_RADIUS = 150;
		const MOUSE_FORCE = 0.02;

		interface Particle {
			x: number;
			y: number;
			vx: number;
			vy: number;
			baseVx: number;
			baseVy: number;
			radius: number;
			opacity: number;
		}

		let width = canvasEl.clientWidth;
		let height = canvasEl.clientHeight;

		function setCanvasSize() {
			if (!canvasEl) return;
			const dpr = Math.min(window.devicePixelRatio, 2);
			width = canvasEl.clientWidth;
			height = canvasEl.clientHeight;
			canvasEl.width = width * dpr;
			canvasEl.height = height * dpr;
			ctx!.setTransform(dpr, 0, 0, dpr, 0, 0);
		}
		setCanvasSize();

		const particles: Particle[] = [];
		for (let i = 0; i < PARTICLE_COUNT; i++) {
			const vx = (Math.random() - 0.5) * 0.4;
			const vy = (Math.random() - 0.5) * 0.4;
			particles.push({
				x: Math.random() * width,
				y: Math.random() * height,
				vx,
				vy,
				baseVx: vx,
				baseVy: vy,
				radius: 1 + Math.random() * 1.5,
				opacity: 0.15 + Math.random() * 0.45
			});
		}

		// --- Mouse tracking ---
		function onMouseMove(e: MouseEvent) {
			const rect = canvasEl!.getBoundingClientRect();
			mouseX = e.clientX - rect.left;
			mouseY = e.clientY - rect.top;
		}

		function onMouseLeave() {
			mouseX = -9999;
			mouseY = -9999;
		}

		function onTouchMove(e: TouchEvent) {
			if (e.touches.length > 0) {
				const rect = canvasEl!.getBoundingClientRect();
				mouseX = e.touches[0].clientX - rect.left;
				mouseY = e.touches[0].clientY - rect.top;
			}
		}

		function onTouchEnd() {
			mouseX = -9999;
			mouseY = -9999;
		}

		canvasEl.addEventListener('mousemove', onMouseMove);
		canvasEl.addEventListener('mouseleave', onMouseLeave);
		canvasEl.addEventListener('touchmove', onTouchMove, { passive: true });
		canvasEl.addEventListener('touchend', onTouchEnd);

		// --- Resize ---
		function onResize() {
			setCanvasSize();
			// Rebound particles within new bounds
			for (const p of particles) {
				if (p.x > width) p.x = width;
				if (p.y > height) p.y = height;
			}
		}
		window.addEventListener('resize', onResize);

		// --- Animation loop ---
		function animate() {
			if (disposed) return;
			animId = requestAnimationFrame(animate);

			ctx!.clearRect(0, 0, width, height);

			if (prefersReducedMotion) {
				// Static render only, no movement
				for (const p of particles) {
					ctx!.beginPath();
					ctx!.arc(p.x, p.y, p.radius, 0, Math.PI * 2);
					ctx!.fillStyle = `rgba(180, 210, 255, ${p.opacity})`;
					ctx!.fill();
				}
				return;
			}

			// Update particle positions
			for (const p of particles) {
				// Mouse attraction
				const dx = mouseX - p.x;
				const dy = mouseY - p.y;
				const dist = Math.sqrt(dx * dx + dy * dy);
				if (dist < MOUSE_RADIUS && dist > 0) {
					const force = (1 - dist / MOUSE_RADIUS) * MOUSE_FORCE;
					p.vx += (dx / dist) * force;
					p.vy += (dy / dist) * force;
				}

				// Dampen back to base velocity
				p.vx += (p.baseVx - p.vx) * 0.01;
				p.vy += (p.baseVy - p.vy) * 0.01;

				p.x += p.vx;
				p.y += p.vy;

				// Wrap around edges
				if (p.x < -10) p.x = width + 10;
				if (p.x > width + 10) p.x = -10;
				if (p.y < -10) p.y = height + 10;
				if (p.y > height + 10) p.y = -10;
			}

			// Draw connection lines
			for (let i = 0; i < particles.length; i++) {
				for (let j = i + 1; j < particles.length; j++) {
					const a = particles[i];
					const b = particles[j];
					const dx = a.x - b.x;
					const dy = a.y - b.y;
					const dist = Math.sqrt(dx * dx + dy * dy);

					if (dist < CONNECTION_DISTANCE) {
						const opacity = (1 - dist / CONNECTION_DISTANCE) * 0.15;
						ctx!.beginPath();
						ctx!.moveTo(a.x, a.y);
						ctx!.lineTo(b.x, b.y);
						ctx!.strokeStyle = `rgba(180, 210, 255, ${opacity})`;
						ctx!.lineWidth = 0.5;
						ctx!.stroke();
					}
				}
			}

			// Draw particles with glow
			for (const p of particles) {
				// Glow layer
				const gradient = ctx!.createRadialGradient(
					p.x, p.y, 0,
					p.x, p.y, p.radius * 3
				);
				gradient.addColorStop(0, `rgba(180, 210, 255, ${p.opacity * 0.4})`);
				gradient.addColorStop(1, 'rgba(180, 210, 255, 0)');
				ctx!.beginPath();
				ctx!.arc(p.x, p.y, p.radius * 3, 0, Math.PI * 2);
				ctx!.fillStyle = gradient;
				ctx!.fill();

				// Core dot
				ctx!.beginPath();
				ctx!.arc(p.x, p.y, p.radius, 0, Math.PI * 2);
				ctx!.fillStyle = `rgba(200, 220, 255, ${p.opacity})`;
				ctx!.fill();
			}
		}

		animate();

		return () => {
			disposed = true;
			cancelAnimationFrame(animId);
			window.removeEventListener('resize', onResize);
			canvasEl?.removeEventListener('mousemove', onMouseMove);
			canvasEl?.removeEventListener('mouseleave', onMouseLeave);
			canvasEl?.removeEventListener('touchmove', onTouchMove);
			canvasEl?.removeEventListener('touchend', onTouchEnd);
		};
	});
</script>

<canvas
	bind:this={canvasEl}
	class="absolute inset-0 h-full w-full"
	aria-hidden="true"
></canvas>

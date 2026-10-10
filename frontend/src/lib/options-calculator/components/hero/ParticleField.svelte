<script lang="ts">
	import { prefersReducedMotion } from 'svelte/motion';
	import type { Attachment } from 'svelte/attachments';

	const PARTICLE_COUNT = 100;
	const CONNECTION_DISTANCE = 120;
	const POINTER_RADIUS = 150;
	const POINTER_FORCE = 0.02;
	const OFFSCREEN = -9999;

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

	const initField: Attachment<HTMLCanvasElement> = (canvas) => {
		const ctx = canvas.getContext('2d');
		if (!ctx) return;

		let width = 0;
		let height = 0;
		let pointerX = OFFSCREEN;
		let pointerY = OFFSCREEN;
		let motion = true;
		let animId = 0;
		let visible = false;

		const resize = () => {
			const dpr = Math.min(window.devicePixelRatio, 2);
			width = canvas.clientWidth;
			height = canvas.clientHeight;
			canvas.width = Math.round(width * dpr);
			canvas.height = Math.round(height * dpr);
			ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
		};
		resize();

		const particles: Particle[] = Array.from({ length: PARTICLE_COUNT }, () => {
			const vx = (Math.random() - 0.5) * 0.4;
			const vy = (Math.random() - 0.5) * 0.4;
			return {
				x: Math.random() * width,
				y: Math.random() * height,
				vx,
				vy,
				baseVx: vx,
				baseVy: vy,
				radius: 1 + Math.random() * 1.5,
				opacity: 0.15 + Math.random() * 0.45
			};
		});

		const step = () => {
			for (const p of particles) {
				const dx = pointerX - p.x;
				const dy = pointerY - p.y;
				const dist = Math.hypot(dx, dy);
				if (dist > 0 && dist < POINTER_RADIUS) {
					const force = (1 - dist / POINTER_RADIUS) * POINTER_FORCE;
					p.vx += (dx / dist) * force;
					p.vy += (dy / dist) * force;
				}
				// Relax back towards the particle's own drift.
				p.vx += (p.baseVx - p.vx) * 0.01;
				p.vy += (p.baseVy - p.vy) * 0.01;
				p.x += p.vx;
				p.y += p.vy;
				if (p.x < -10) p.x = width + 10;
				if (p.x > width + 10) p.x = -10;
				if (p.y < -10) p.y = height + 10;
				if (p.y > height + 10) p.y = -10;
			}
		};

		const draw = () => {
			ctx.clearRect(0, 0, width, height);
			ctx.lineWidth = 0.5;
			for (let i = 0; i < particles.length; i++) {
				const a = particles[i];
				for (let j = i + 1; j < particles.length; j++) {
					const b = particles[j];
					const dist = Math.hypot(a.x - b.x, a.y - b.y);
					if (dist < CONNECTION_DISTANCE) {
						ctx.beginPath();
						ctx.moveTo(a.x, a.y);
						ctx.lineTo(b.x, b.y);
						ctx.strokeStyle = `rgba(180, 210, 255, ${(1 - dist / CONNECTION_DISTANCE) * 0.15})`;
						ctx.stroke();
					}
				}
			}
			for (const p of particles) {
				const glow = ctx.createRadialGradient(p.x, p.y, 0, p.x, p.y, p.radius * 3);
				glow.addColorStop(0, `rgba(180, 210, 255, ${p.opacity * 0.4})`);
				glow.addColorStop(1, 'rgba(180, 210, 255, 0)');
				ctx.beginPath();
				ctx.arc(p.x, p.y, p.radius * 3, 0, Math.PI * 2);
				ctx.fillStyle = glow;
				ctx.fill();

				ctx.beginPath();
				ctx.arc(p.x, p.y, p.radius, 0, Math.PI * 2);
				ctx.fillStyle = `rgba(200, 220, 255, ${p.opacity})`;
				ctx.fill();
			}
		};

		const frame = () => {
			step();
			draw();
			animId = requestAnimationFrame(frame);
		};

		/** Animate while visible and motion is allowed; otherwise hold one still frame. */
		const sync = () => {
			cancelAnimationFrame(animId);
			if (visible && motion) frame();
			else draw();
		};

		const toCanvas = (clientX: number, clientY: number) => {
			const rect = canvas.getBoundingClientRect();
			pointerX = clientX - rect.left;
			pointerY = clientY - rect.top;
		};
		const onPointerMove = (event: PointerEvent) => toCanvas(event.clientX, event.clientY);
		const onPointerEnd = () => {
			pointerX = OFFSCREEN;
			pointerY = OFFSCREEN;
		};
		window.addEventListener('pointermove', onPointerMove, { passive: true });
		window.addEventListener('pointerup', onPointerEnd, { passive: true });
		document.documentElement.addEventListener('pointerleave', onPointerEnd);

		const visibility = new IntersectionObserver(([entry]) => {
			visible = entry?.isIntersecting ?? false;
			sync();
		});
		visibility.observe(canvas);

		const sizeObserver = new ResizeObserver(() => {
			resize();
			for (const p of particles) {
				p.x = Math.min(p.x, width);
				p.y = Math.min(p.y, height);
			}
			if (!motion) draw();
		});
		sizeObserver.observe(canvas);

		$effect(() => {
			motion = !prefersReducedMotion.current;
			sync();
		});

		return () => {
			cancelAnimationFrame(animId);
			visibility.disconnect();
			sizeObserver.disconnect();
			window.removeEventListener('pointermove', onPointerMove);
			window.removeEventListener('pointerup', onPointerEnd);
			document.documentElement.removeEventListener('pointerleave', onPointerEnd);
		};
	};
</script>

<canvas {@attach initField} class="absolute inset-0 h-full w-full" aria-hidden="true"></canvas>

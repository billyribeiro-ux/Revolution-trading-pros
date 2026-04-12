// ============================================================
// CANVAS-BASED PARTICLE BURST SYSTEM
// ============================================================

interface Particle {
	x: number;
	y: number;
	vx: number;
	vy: number;
	life: number;
	maxLife: number;
	size: number;
	color: string;
	alpha: number;
}

export function createParticleSystem() {
	let canvas: HTMLCanvasElement | null = null;
	let ctx: CanvasRenderingContext2D | null = null;
	let particles: Particle[] = [];
	let animationId: number | null = null;

	function handleResize() {
		if (canvas) {
			canvas.width = window.innerWidth;
			canvas.height = window.innerHeight;
		}
	}

	function init(container: HTMLElement) {
		canvas = document.createElement('canvas');
		canvas.style.cssText = 'position:fixed;inset:0;z-index:9999;pointer-events:none;';
		canvas.width = window.innerWidth;
		canvas.height = window.innerHeight;
		container.appendChild(canvas);
		ctx = canvas.getContext('2d');

		window.addEventListener('resize', handleResize);
	}

	function burst(
		x: number,
		y: number,
		count = 30,
		colors = ['#f69532', '#6366f1', '#10b981', '#3b82f6']
	) {
		for (let i = 0; i < count; i++) {
			const angle = (Math.PI * 2 * i) / count + (Math.random() - 0.5) * 0.5;
			const speed = 2 + Math.random() * 4;
			particles.push({
				x,
				y,
				vx: Math.cos(angle) * speed,
				vy: Math.sin(angle) * speed,
				life: 1,
				maxLife: 0.8 + Math.random() * 0.5,
				size: 2 + Math.random() * 3,
				color: colors[Math.floor(Math.random() * colors.length)],
				alpha: 1
			});
		}
		if (!animationId) animate();
	}

	function animate() {
		if (!ctx || !canvas) return;
		ctx.clearRect(0, 0, canvas.width, canvas.height);

		particles = particles.filter((p) => {
			p.x += p.vx;
			p.y += p.vy;
			p.vy += 0.08; // gravity
			p.vx *= 0.99; // drag
			p.life -= 0.016;
			p.alpha = Math.max(0, p.life / p.maxLife);

			ctx!.beginPath();
			ctx!.arc(p.x, p.y, p.size * p.alpha, 0, Math.PI * 2);
			ctx!.fillStyle = p.color;
			ctx!.globalAlpha = p.alpha * 0.8;
			ctx!.fill();
			ctx!.globalAlpha = 1;

			return p.life > 0;
		});

		if (particles.length > 0) {
			animationId = requestAnimationFrame(animate);
		} else {
			animationId = null;
		}
	}

	function destroy() {
		if (animationId) cancelAnimationFrame(animationId);
		window.removeEventListener('resize', handleResize);
		canvas?.remove();
		canvas = null;
		ctx = null;
		particles = [];
	}

	return { init, burst, destroy };
}

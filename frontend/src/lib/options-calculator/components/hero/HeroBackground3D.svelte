<script lang="ts">
	import { prefersReducedMotion } from 'svelte/motion';
	import { PerspectiveCamera } from 'three/src/cameras/PerspectiveCamera.js';
	import { BufferAttribute } from 'three/src/core/BufferAttribute.js';
	import { BufferGeometry } from 'three/src/core/BufferGeometry.js';
	import { IcosahedronGeometry } from 'three/src/geometries/IcosahedronGeometry.js';
	import { OctahedronGeometry } from 'three/src/geometries/OctahedronGeometry.js';
	import { PlaneGeometry } from 'three/src/geometries/PlaneGeometry.js';
	import { TorusKnotGeometry } from 'three/src/geometries/TorusKnotGeometry.js';
	import { MeshBasicMaterial } from 'three/src/materials/MeshBasicMaterial.js';
	import { PointsMaterial } from 'three/src/materials/PointsMaterial.js';
	import { Mesh } from 'three/src/objects/Mesh.js';
	import { Points } from 'three/src/objects/Points.js';
	import { WebGLRenderer } from 'three/src/renderers/WebGLRenderer.js';
	import { FogExp2 } from 'three/src/scenes/FogExp2.js';
	import { Scene } from 'three/src/scenes/Scene.js';
	import type { Attachment } from 'svelte/attachments';

	const SHAPE_COUNT = 25;
	const PARTICLE_COUNT = 200;

	interface FloatingShape {
		mesh: Mesh;
		baseX: number;
		baseY: number;
		floatSpeed: number;
		floatAmp: number;
		spin: readonly [number, number, number];
	}

	const initScene: Attachment<HTMLCanvasElement> = (canvas) => {
		let renderer: WebGLRenderer;
		try {
			renderer = new WebGLRenderer({ canvas, antialias: true, alpha: true });
		} catch {
			// Decorative background: without WebGL the hero gradient stands alone.
			return;
		}
		renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
		renderer.setClearColor(0x000000, 0);

		const scene = new Scene();
		scene.fog = new FogExp2(0x0a0f1a, 0.035);

		const camera = new PerspectiveCamera(60, 16 / 9, 0.1, 200);
		camera.position.set(0, 0, 30);

		const resize = (width: number, height: number) => {
			if (width <= 0 || height <= 0) return;
			camera.aspect = width / height;
			camera.updateProjectionMatrix();
			renderer.setSize(width, height, false);
		};
		resize(canvas.clientWidth, canvas.clientHeight);

		// ── Floating wireframe shapes ──
		const materials = [
			new MeshBasicMaterial({ color: 0x143e59, wireframe: true, transparent: true, opacity: 0.25 }),
			new MeshBasicMaterial({ color: 0xf69532, wireframe: true, transparent: true, opacity: 0.15 }),
			new MeshBasicMaterial({ color: 0x6366f1, wireframe: true, transparent: true, opacity: 0.18 })
		];
		const geometries: BufferGeometry[] = [];
		const random = (spread: number) => (Math.random() - 0.5) * spread;

		const shapes: FloatingShape[] = Array.from({ length: SHAPE_COUNT }, () => {
			const size = 0.4 + Math.random() * 1.4;
			const kind = Math.floor(Math.random() * 3);
			const geometry =
				kind === 0
					? new IcosahedronGeometry(size, 1)
					: kind === 1
						? new OctahedronGeometry(size, 0)
						: new TorusKnotGeometry(size * 0.6, size * 0.2, 48, 8);
			geometries.push(geometry);
			const mesh = new Mesh(geometry, materials[Math.floor(Math.random() * materials.length)]);
			mesh.position.set(random(50), random(30), random(30) - 5);
			mesh.rotation.set(
				Math.random() * Math.PI * 2,
				Math.random() * Math.PI * 2,
				Math.random() * Math.PI * 2
			);
			scene.add(mesh);
			return {
				mesh,
				baseX: mesh.position.x,
				baseY: mesh.position.y,
				floatSpeed: 0.3 + Math.random() * 0.7,
				floatAmp: 0.5 + Math.random() * 1.5,
				spin: [random(0.01), random(0.01), random(0.005)] as const
			};
		});

		// ── Warped grid: a volatility-surface backdrop ──
		const gridGeometry = new PlaneGeometry(60, 40, 30, 20);
		geometries.push(gridGeometry);
		const gridMaterial = new MeshBasicMaterial({
			color: 0x143e59,
			wireframe: true,
			transparent: true,
			opacity: 0.08
		});
		const grid = new Mesh(gridGeometry, gridMaterial);
		grid.position.set(0, 0, -20);
		grid.rotation.x = -0.3;
		scene.add(grid);

		const gridPositions = gridGeometry.attributes.position;
		const gridBaseZ = new Float32Array(gridPositions.count);
		for (let i = 0; i < gridPositions.count; i++) {
			const x = gridPositions.getX(i);
			const y = gridPositions.getY(i);
			gridBaseZ[i] = Math.sin(x * 0.15) * Math.cos(y * 0.2) * 3 + Math.sin(x * 0.08 + y * 0.1) * 2;
			gridPositions.setZ(i, gridBaseZ[i]);
		}
		gridPositions.needsUpdate = true;

		// ── Drifting ambient particles ──
		const particlePositions = new Float32Array(PARTICLE_COUNT * 3);
		const particleSpeeds = new Float32Array(PARTICLE_COUNT);
		for (let i = 0; i < PARTICLE_COUNT; i++) {
			particlePositions[i * 3] = random(60);
			particlePositions[i * 3 + 1] = random(40);
			particlePositions[i * 3 + 2] = random(40) - 5;
			particleSpeeds[i] = 0.005 + Math.random() * 0.015;
		}
		const particleAttribute = new BufferAttribute(particlePositions, 3);
		const particleGeometry = new BufferGeometry();
		particleGeometry.setAttribute('position', particleAttribute);
		geometries.push(particleGeometry);
		const particleMaterial = new PointsMaterial({
			color: 0xaaccff,
			size: 0.08,
			transparent: true,
			opacity: 0.6,
			sizeAttenuation: true
		});
		scene.add(new Points(particleGeometry, particleMaterial));

		// ── Pointer parallax ──
		let pointerX = 0;
		let pointerY = 0;
		const onPointerMove = (event: PointerEvent) => {
			if (event.pointerType !== 'mouse') return;
			pointerX = (event.clientX / window.innerWidth) * 2 - 1;
			pointerY = -(event.clientY / window.innerHeight) * 2 + 1;
		};
		window.addEventListener('pointermove', onPointerMove, { passive: true });

		let motion = true;
		let animId = 0;
		const start = performance.now();

		const frame = () => {
			animId = requestAnimationFrame(frame);
			if (motion) {
				const t = (performance.now() - start) / 1000;
				for (const shape of shapes) {
					const { mesh } = shape;
					mesh.position.y = shape.baseY + Math.sin(t * shape.floatSpeed) * shape.floatAmp;
					// Ease towards a pointer-offset rest position (bounded, never drifts away).
					mesh.position.x += (shape.baseX + pointerX * 0.3 - mesh.position.x) * 0.02;
					mesh.rotation.x += shape.spin[0];
					mesh.rotation.y += shape.spin[1];
					mesh.rotation.z += shape.spin[2];
				}

				grid.rotation.z = Math.sin(t * 0.1) * 0.05;
				for (let i = 0; i < gridPositions.count; i++) {
					gridPositions.setZ(
						i,
						gridBaseZ[i] + Math.sin(t * 0.5 + gridPositions.getX(i) * 0.1) * 0.3
					);
				}
				gridPositions.needsUpdate = true;

				for (let i = 0; i < PARTICLE_COUNT; i++) {
					const y = i * 3 + 1;
					particlePositions[y] += particleSpeeds[i];
					if (particlePositions[y] > 20) particlePositions[y] = -20;
				}
				particleAttribute.needsUpdate = true;

				camera.position.x += (pointerX * 2 - camera.position.x) * 0.02;
				camera.position.y += (pointerY * 1.5 - camera.position.y) * 0.02;
				camera.lookAt(0, 0, 0);
			}
			renderer.render(scene, camera);
		};

		// Stop rendering once the hero has scrolled out of view.
		const visibility = new IntersectionObserver(([entry]) => {
			cancelAnimationFrame(animId);
			if (entry?.isIntersecting) frame();
		});
		visibility.observe(canvas);

		const sizeObserver = new ResizeObserver(([entry]) => {
			if (entry) resize(entry.contentRect.width, entry.contentRect.height);
		});
		sizeObserver.observe(canvas);

		$effect(() => {
			motion = !prefersReducedMotion.current;
		});

		return () => {
			cancelAnimationFrame(animId);
			visibility.disconnect();
			sizeObserver.disconnect();
			window.removeEventListener('pointermove', onPointerMove);
			for (const geometry of geometries) geometry.dispose();
			for (const material of materials) material.dispose();
			gridMaterial.dispose();
			particleMaterial.dispose();
			renderer.dispose();
			renderer.forceContextLoss();
		};
	};
</script>

<canvas {@attach initScene} class="absolute inset-0 h-full w-full" aria-hidden="true"></canvas>

<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';

	let canvasEl: HTMLCanvasElement | undefined = $state();
	let prefersReducedMotion = $state(false);

	onMount(() => {
		if (!browser || !canvasEl) return;

		const motionQuery = window.matchMedia('(prefers-reduced-motion: reduce)');
		prefersReducedMotion = motionQuery.matches;
		function onMotionChange(e: MediaQueryListEvent) {
			prefersReducedMotion = e.matches;
		}
		motionQuery.addEventListener('change', onMotionChange);

		let disposed = false;
		let animId = 0;

		// Track mouse position for parallax
		let mouseX = 0;
		let mouseY = 0;
		function onMouseMove(e: MouseEvent) {
			mouseX = (e.clientX / window.innerWidth) * 2 - 1;
			mouseY = -(e.clientY / window.innerHeight) * 2 + 1;
		}
		window.addEventListener('mousemove', onMouseMove);

		async function init() {
			const THREE = await import('three');
			if (disposed || !canvasEl) return;

			// --- Scene setup ---
			const scene = new THREE.Scene();
			scene.fog = new THREE.FogExp2(0x0a0f1a, 0.035);

			const camera = new THREE.PerspectiveCamera(
				60,
				canvasEl.clientWidth / canvasEl.clientHeight,
				0.1,
				200
			);
			camera.position.set(0, 0, 30);

			const renderer = new THREE.WebGLRenderer({
				canvas: canvasEl,
				antialias: true,
				alpha: true
			});
			renderer.setSize(canvasEl.clientWidth, canvasEl.clientHeight);
			renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
			renderer.setClearColor(0x000000, 0);

			// --- Floating geometric shapes ---
			const shapeMeshes: any[] = [];
			const shapeData: {
				mesh: any;
				baseY: number;
				floatSpeed: number;
				floatAmp: number;
				rotSpeed: any;
			}[] = [];

			const blueMat = new THREE.MeshBasicMaterial({
				color: 0x143e59,
				wireframe: true,
				transparent: true,
				opacity: 0.25
			});
			const goldMat = new THREE.MeshBasicMaterial({
				color: 0xf69532,
				wireframe: true,
				transparent: true,
				opacity: 0.15
			});
			const indigoMat = new THREE.MeshBasicMaterial({
				color: 0x6366f1,
				wireframe: true,
				transparent: true,
				opacity: 0.18
			});

			const materials = [blueMat, goldMat, indigoMat];

			const geometries: any[] = [];

			function createGeometry(type: number, size: number): any {
				let geo: any;
				if (type === 0) {
					geo = new THREE.IcosahedronGeometry(size, 1);
				} else if (type === 1) {
					geo = new THREE.OctahedronGeometry(size, 0);
				} else {
					geo = new THREE.TorusKnotGeometry(size * 0.6, size * 0.2, 48, 8);
				}
				geometries.push(geo);
				return geo;
			}

			const shapeCount = 25;
			for (let i = 0; i < shapeCount; i++) {
				const geoType = Math.floor(Math.random() * 3);
				const size = 0.4 + Math.random() * 1.4;
				const geo = createGeometry(geoType, size);
				const mat = materials[Math.floor(Math.random() * materials.length)];
				const mesh = new THREE.Mesh(geo, mat);

				mesh.position.set(
					(Math.random() - 0.5) * 50,
					(Math.random() - 0.5) * 30,
					(Math.random() - 0.5) * 30 - 5
				);

				mesh.rotation.set(
					Math.random() * Math.PI * 2,
					Math.random() * Math.PI * 2,
					Math.random() * Math.PI * 2
				);

				scene.add(mesh);
				shapeMeshes.push(mesh);
				shapeData.push({
					mesh,
					baseY: mesh.position.y,
					floatSpeed: 0.3 + Math.random() * 0.7,
					floatAmp: 0.5 + Math.random() * 1.5,
					rotSpeed: new THREE.Vector3(
						(Math.random() - 0.5) * 0.01,
						(Math.random() - 0.5) * 0.01,
						(Math.random() - 0.5) * 0.005
					)
				});
			}

			// --- Volatility surface grid in background ---
			const gridGeo = new THREE.PlaneGeometry(60, 40, 30, 20);
			const gridMat = new THREE.MeshBasicMaterial({
				color: 0x143e59,
				wireframe: true,
				transparent: true,
				opacity: 0.08
			});
			geometries.push(gridGeo);
			const gridMesh = new THREE.Mesh(gridGeo, gridMat);
			gridMesh.position.set(0, 0, -20);
			gridMesh.rotation.x = -0.3;
			scene.add(gridMesh);

			// Warp the grid vertices for a volatility-surface feel
			const gridPositions = gridGeo.attributes.position;
			const gridBaseZ: number[] = [];
			for (let i = 0; i < gridPositions.count; i++) {
				const x = gridPositions.getX(i);
				const y = gridPositions.getY(i);
				const z =
					Math.sin(x * 0.15) * Math.cos(y * 0.2) * 3 +
					Math.sin(x * 0.08 + y * 0.1) * 2;
				gridPositions.setZ(i, z);
				gridBaseZ.push(z);
			}
			gridGeo.attributes.position.needsUpdate = true;
			gridGeo.computeVertexNormals();

			// --- Ambient particles ---
			const particleCount = 200;
			const particleGeo = new THREE.BufferGeometry();
			const particlePositions = new Float32Array(particleCount * 3);
			const particleSpeeds = new Float32Array(particleCount);

			for (let i = 0; i < particleCount; i++) {
				particlePositions[i * 3] = (Math.random() - 0.5) * 60;
				particlePositions[i * 3 + 1] = (Math.random() - 0.5) * 40;
				particlePositions[i * 3 + 2] = (Math.random() - 0.5) * 40 - 5;
				particleSpeeds[i] = 0.005 + Math.random() * 0.015;
			}

			particleGeo.setAttribute('position', new THREE.BufferAttribute(particlePositions, 3));
			geometries.push(particleGeo);

			const particleMat = new THREE.PointsMaterial({
				color: 0xaaccff,
				size: 0.08,
				transparent: true,
				opacity: 0.6,
				sizeAttenuation: true
			});

			const particles = new THREE.Points(particleGeo, particleMat);
			scene.add(particles);

			// --- Resize handler ---
			function onResize() {
				if (!canvasEl || disposed) return;
				const w = canvasEl.clientWidth;
				const h = canvasEl.clientHeight;
				camera.aspect = w / h;
				camera.updateProjectionMatrix();
				renderer.setSize(w, h);
			}
			window.addEventListener('resize', onResize);

			// --- Animation loop ---
			const clock = new THREE.Clock();

			function animate() {
				if (disposed) return;
				animId = requestAnimationFrame(animate);

				const elapsed = clock.getElapsedTime();

				if (!prefersReducedMotion) {
					// Animate floating shapes
					for (const sd of shapeData) {
						sd.mesh.position.y =
							sd.baseY + Math.sin(elapsed * sd.floatSpeed) * sd.floatAmp;
						sd.mesh.rotation.x += sd.rotSpeed.x;
						sd.mesh.rotation.y += sd.rotSpeed.y;
						sd.mesh.rotation.z += sd.rotSpeed.z;

						// Mouse parallax on shapes
						sd.mesh.position.x += (mouseX * 0.3 - sd.mesh.position.x * 0.001) * 0.02;
					}

					// Rotate the grid slowly
					gridMesh.rotation.z = Math.sin(elapsed * 0.1) * 0.05;

					// Animate grid vertices for a breathing effect
					for (let i = 0; i < gridPositions.count; i++) {
						const baseZ = gridBaseZ[i];
						const x = gridPositions.getX(i);
						gridPositions.setZ(
							i,
							baseZ + Math.sin(elapsed * 0.5 + x * 0.1) * 0.3
						);
					}
					gridGeo.attributes.position.needsUpdate = true;

					// Drift particles
					for (let i = 0; i < particleCount; i++) {
						particlePositions[i * 3 + 1] += particleSpeeds[i];
						if (particlePositions[i * 3 + 1] > 20) {
							particlePositions[i * 3 + 1] = -20;
						}
					}
					particleGeo.attributes.position.needsUpdate = true;

					// Camera subtle parallax based on mouse
					camera.position.x += (mouseX * 2 - camera.position.x) * 0.02;
					camera.position.y += (mouseY * 1.5 - camera.position.y) * 0.02;
					camera.lookAt(0, 0, 0);
				}

				renderer.render(scene, camera);
			}

			animate();

			// Return cleanup function for the async init
			return () => {
				disposed = true;
				cancelAnimationFrame(animId);
				window.removeEventListener('resize', onResize);

				// Dispose all geometries
				for (const geo of geometries) {
					geo.dispose();
				}
				particleGeo.dispose();

				// Dispose materials
				blueMat.dispose();
				goldMat.dispose();
				indigoMat.dispose();
				gridMat.dispose();
				particleMat.dispose();

				renderer.dispose();
			};
		}

		let cleanupFn: (() => void) | undefined;
		init().then((cleanup) => {
			if (cleanup) cleanupFn = cleanup;
		});

		return () => {
			window.removeEventListener('mousemove', onMouseMove);
			motionQuery.removeEventListener('change', onMotionChange);
			if (cleanupFn) cleanupFn();
		};
	});
</script>

<canvas
	bind:this={canvasEl}
	class="absolute inset-0 h-full w-full"
	aria-hidden="true"
></canvas>

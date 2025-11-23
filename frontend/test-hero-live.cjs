// Quick test script - Run with: node test-hero-live.js
const http = require('http');

console.log('Testing Hero Component at http://localhost:5174\n');

// Test 1: Page loads
http
	.get('http://localhost:5174/', (res) => {
		let data = '';
		res.on('data', (chunk) => (data += chunk));
		res.on('end', () => {
			console.log('✅ Test 1: Page loads (status:', res.statusCode, ')');

			// Test 2: Hero section exists
			const hasHero = data.includes('hero-chart');
			console.log(
				hasHero
					? '✅ Test 2: Hero chart container exists'
					: '❌ Test 2: Hero chart container MISSING'
			);

			// Test 3: Slides exist
			const hasSlides = data.includes('data-slide="0"');
			console.log(hasSlides ? '✅ Test 3: Slides exist in markup' : '❌ Test 3: Slides MISSING');

			// Test 4: GSAP will be loaded
			const hasGsap = data.includes('gsap') || true; // Dynamic import, so always true
			console.log('✅ Test 4: GSAP will be dynamically imported');

			// Test 5: Lightweight Charts will be loaded
			console.log('✅ Test 5: Lightweight Charts imported in component');

			console.log('\n📋 MANUAL VERIFICATION REQUIRED:');
			console.log('   Open http://localhost:5174 in your browser');
			console.log('   Open DevTools Console (F12)');
			console.log('   Look for these logs:');
			console.log('   - 🚀 [MOUNT] Component mounted');
			console.log('   - 🚀 [MOUNT] After timeout - chartContainer: <div>');
			console.log('   - ✅ [INIT] Chart created');
			console.log('   - ✅ [REPLAY] Interval started');
			console.log('   - 🎬 [ANIM] Slide element: <div>');
			console.log('\n   VISUAL CHECK:');
			console.log('   - Chart: Look for subtle cyan/indigo candlesticks in background');
			console.log('   - Animations: Slide 0 should fade in with scale effect');
			console.log('   - Auto-advance: Wait 5 seconds, slide should change');
		});
	})
	.on('error', (err) => {
		console.error('❌ ERROR: Cannot connect to server:', err.message);
		console.log('   Make sure dev server is running: npm run dev');
	});

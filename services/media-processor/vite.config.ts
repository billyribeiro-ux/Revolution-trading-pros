import { defineConfig } from 'vite-plus';
export default defineConfig({
	test: { name: 'media-processor', environment: 'node', include: ['test/**/*.test.ts'] }
});

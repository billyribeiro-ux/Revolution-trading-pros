import { defineConfig } from 'vite-plus';

export default defineConfig({
	pack: {
		entry: ['src/index.ts'],
		format: ['esm'],
		dts: {
			sourcemap: false
		},
		sourcemap: false,
		clean: true,
		fixedExtension: false
	}
});

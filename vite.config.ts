import { defineConfig } from 'vite-plus';

export default defineConfig({
	defaultPackage: {
		dev: './frontend',
		build: './frontend',
		preview: './frontend',
		pack: './svelte-click-to-source'
	},
	test: {
		projects: ['frontend']
	},
	staged: {
		'*': 'vp check --fix'
	},
	fmt: {
		useTabs: true,
		singleQuote: true,
		trailingComma: 'none',
		printWidth: 100,
		ignorePatterns: [
			'**/.svelte-kit/**',
			'**/build/**',
			'**/dist/**',
			'**/.netlify/**',
			'**/node_modules/**',
			'**/.cache/**',
			'**/.turbo/**',
			'**/test-results/**',
			'**/playwright-report/**',
			'**/coverage/**',
			'**/*.log',
			'**/.DS_Store',
			'package-lock.json',
			'pnpm-lock.yaml',
			'yarn.lock',
			'**/Implementation/**',
			'api/.sqlx/**',
			'frontend/src/lib/icons/generated/**'
		]
	},
	lint: {
		ignorePatterns: [
			'**/.svelte-kit/**',
			'**/build/**',
			'**/dist/**',
			'**/node_modules/**',
			'frontend/static/**',
			'**/coverage/**',
			'**/playwright-report/**',
			'**/test-results/**',
			'frontend/tests/**',
			'frontend/scripts/**',
			'frontend/src/routes/test-error/**',
			'frontend/src/lib/stores/theme.svelte.ts',
			'frontend/src/lib/icons/generated/**'
		],
		jsPlugins: [{ name: 'vite-plus', specifier: 'vite-plus/oxlint-plugin' }],
		rules: {
			'vite-plus/prefer-vite-plus-imports': 'error',
			'typescript/no-unused-vars': [
				'warn',
				{
					argsIgnorePattern: '^_',
					varsIgnorePattern: '^_',
					caughtErrorsIgnorePattern: '^_'
				}
			],
			'typescript/no-explicit-any': 'warn',
			'typescript/ban-ts-comment': 'off',
			'typescript/no-empty-object-type': 'off',
			'no-unused-vars': 'off',
			'no-undef': 'off',
			'no-empty': 'warn',
			'no-case-declarations': 'off',
			'no-useless-escape': 'warn',
			'prefer-const': 'warn'
		},
		options: {
			typeAware: true,
			typeCheck: true
		}
	}
});

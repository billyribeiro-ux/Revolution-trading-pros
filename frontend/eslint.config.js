import js from '@eslint/js';
import ts from '@typescript-eslint/eslint-plugin';
import tsParser from '@typescript-eslint/parser';
import svelte from 'eslint-plugin-svelte';
import prettier from 'eslint-config-prettier';
import globals from 'globals';

export default [
	js.configs.recommended,
	...svelte.configs['flat/recommended'],
	...svelte.configs['flat/prettier'],
	{
		files: ['**/*.{js,ts,svelte}'],
		languageOptions: {
			globals: {
				...globals.browser,
				...globals.node,
				gapi: 'readonly',
				ServiceWorkerGlobalScope: 'readonly',
				FetchEvent: 'readonly',
				Disposable: 'readonly',
				// SvelteKit ambient namespace declared in src/app.d.ts
				App: 'readonly',
				$state: 'readonly',
				$derived: 'readonly',
				$effect: 'readonly',
				$props: 'readonly',
				$bindable: 'readonly',
				$inspect: 'readonly',
				$host: 'readonly'
			}
		}
	},
	// TS rules with project-aware parser — only src/** is in tsconfig.json
	{
		files: ['src/**/*.ts'],
		languageOptions: {
			parser: tsParser,
			parserOptions: {
				project: './tsconfig.json',
				extraFileExtensions: ['.svelte']
			}
		},
		plugins: {
			'@typescript-eslint': ts
		},
		rules: {
			...ts.configs.strict.rules,
			'@typescript-eslint/no-unused-vars': [
				'warn',
				{ argsIgnorePattern: '^_', varsIgnorePattern: '^_', caughtErrorsIgnorePattern: '^_' }
			],
			// FIX-2026-04-26: was 'off' — surface existing any usage as warnings
			// '@typescript-eslint/no-explicit-any': 'off',
			'@typescript-eslint/no-explicit-any': 'warn',
			// FIX-2026-04-26: was 'off' — surface non-null assertions as warnings
			// '@typescript-eslint/no-non-null-assertion': 'off',
			'@typescript-eslint/no-non-null-assertion': 'warn',
			'@typescript-eslint/no-invalid-void-type': 'off',
			'@typescript-eslint/no-unused-expressions': 'off',
			// FIX-2026-04-26: was 'off' — surface ts-comment suppressions as warnings
			// '@typescript-eslint/ban-ts-comment': 'off',
			'@typescript-eslint/ban-ts-comment': 'warn',
			'@typescript-eslint/no-useless-constructor': 'warn',
			'@typescript-eslint/no-dynamic-delete': 'off',
			'@typescript-eslint/no-unsafe-function-type': 'warn',
			'@typescript-eslint/no-empty-object-type': 'off',
			'@typescript-eslint/unified-signatures': 'off',
			'@typescript-eslint/no-this-alias': 'warn',
			'@typescript-eslint/prefer-as-const': 'warn',
			'@typescript-eslint/explicit-function-return-type': 'off',
			'@typescript-eslint/explicit-module-boundary-types': 'off',
			'no-case-declarations': 'off',
			'no-useless-escape': 'warn',
			'no-import-assign': 'warn',
			'no-useless-catch': 'warn',
			'no-control-regex': 'off',
			'no-empty': 'warn',
			// FIX-2026-04-26: was 'off' — warn on bare console.log only, allow .error/.warn/.info/.debug
			// 'no-console': 'off',
			'no-console': ['warn', { allow: ['error', 'warn', 'info', 'debug'] }],
			// PHASE-1 (audit 2026-05-16): TypeScript's own compiler resolves
			// ambient `declare global` namespaces (e.g. SvelteKit's `App.Locals`);
			// the base `no-undef` rule cannot and false-positives on them
			// (auth.ts `App.Locals['user']`). typescript-eslint's own guidance
			// is to disable `no-undef` for TS files. This block is now
			// consistent with the Svelte block, which already does this (below).
			'no-undef': 'off'
		}
	},
	// TS rules without project for test/script files (not in tsconfig.json)
	{
		files: [
			'tests/**/*.{ts,js}',
			'e2e/**/*.{ts,js}',
			'scripts/**/*.{ts,js}',
			'playwright.config.ts'
		],
		languageOptions: {
			parser: tsParser,
			parserOptions: {
				// No `project` — these files are not in tsconfig.json
				extraFileExtensions: ['.svelte']
			}
		},
		plugins: {
			'@typescript-eslint': ts
		},
		rules: {
			// Disable base rule — TS version below handles this with proper ignore patterns
			'no-unused-vars': 'off',
			'@typescript-eslint/no-unused-vars': [
				'warn',
				{ argsIgnorePattern: '^_', varsIgnorePattern: '^_', caughtErrorsIgnorePattern: '^_' }
			],
			// FIX-2026-04-26: was 'off' — surface existing any usage as warnings
			// '@typescript-eslint/no-explicit-any': 'off',
			'@typescript-eslint/no-explicit-any': 'warn',
			// FIX-2026-04-26: was 'off' — surface non-null assertions as warnings
			// '@typescript-eslint/no-non-null-assertion': 'off',
			'@typescript-eslint/no-non-null-assertion': 'warn',
			// FIX-2026-04-26: was 'off' — surface ts-comment suppressions as warnings
			// '@typescript-eslint/ban-ts-comment': 'off',
			'@typescript-eslint/ban-ts-comment': 'warn',
			// FIX-2026-04-26: was 'off' — warn on bare console.log only, allow .error/.warn/.info/.debug
			// 'no-console': 'off',
			'no-console': ['warn', { allow: ['error', 'warn', 'info', 'debug'] }]
		}
	},
	{
		files: ['src/**/*.svelte', 'src/**/*.svelte.ts', 'src/**/*.svelte.js'],
		languageOptions: {
			parserOptions: {
				parser: tsParser,
				project: './tsconfig.json',
				extraFileExtensions: ['.svelte'],
				svelteFeatures: {
					experimentalGenerics: true
				}
			},
			globals: {
				$state: 'readonly',
				$derived: 'readonly',
				$effect: 'readonly',
				$props: 'readonly',
				$bindable: 'readonly',
				$inspect: 'readonly',
				$host: 'readonly'
			}
		},
		plugins: {
			'@typescript-eslint': ts
		},
		rules: {
			...ts.configs.strict.rules,
			'@typescript-eslint/no-unused-vars': [
				'warn',
				{ argsIgnorePattern: '^_', varsIgnorePattern: '^(\\$|_)', caughtErrorsIgnorePattern: '^_' }
			],
			// FIX-2026-04-26: was 'off' — surface existing any usage as warnings
			// '@typescript-eslint/no-explicit-any': 'off',
			'@typescript-eslint/no-explicit-any': 'warn',
			// FIX-2026-04-26: was 'off' — surface non-null assertions as warnings
			// '@typescript-eslint/no-non-null-assertion': 'off',
			'@typescript-eslint/no-non-null-assertion': 'warn',
			'@typescript-eslint/no-invalid-void-type': 'off',
			'@typescript-eslint/no-unused-expressions': 'off',
			// FIX-2026-04-26: was 'off' — surface ts-comment suppressions as warnings
			// '@typescript-eslint/ban-ts-comment': 'off',
			'@typescript-eslint/ban-ts-comment': 'warn',
			'@typescript-eslint/no-useless-constructor': 'warn',
			'@typescript-eslint/no-dynamic-delete': 'off',
			'@typescript-eslint/no-unsafe-function-type': 'warn',
			'@typescript-eslint/no-empty-object-type': 'off',
			'@typescript-eslint/unified-signatures': 'off',
			'@typescript-eslint/no-this-alias': 'warn',
			'@typescript-eslint/prefer-as-const': 'warn',
			'@typescript-eslint/explicit-function-return-type': 'off',
			'@typescript-eslint/explicit-module-boundary-types': 'off',
			'no-case-declarations': 'off',
			'no-useless-escape': 'warn',
			'no-import-assign': 'warn',
			'no-useless-catch': 'warn',
			'no-control-regex': 'off',
			'no-empty': 'warn',
			// FIX-2026-04-26: was 'off' — warn on bare console.log only, allow .error/.warn/.info/.debug
			// 'no-console': 'off',
			'no-console': ['warn', { allow: ['error', 'warn', 'info', 'debug'] }],
			'svelte/no-unused-svelte-ignore': 'off',
			'svelte/no-shorthand-style-property-overrides': 'warn',
			// FIX-2026-04-26: was 'off' — surface {@html ...} usage as warnings
			// 'svelte/no-at-html-tags': 'off',
			'svelte/no-at-html-tags': 'warn',
			// FIX-2026-04-26: was 'off' — surface {#each} without key as warnings
			// 'svelte/require-each-key': 'off',
			'svelte/require-each-key': 'warn',
			'svelte/no-navigation-without-resolve': 'off',
			'svelte/prefer-svelte-reactivity': 'off',
			'svelte/no-unused-props': 'off',
			'svelte/prefer-writable-derived': 'off',
			'svelte/no-dom-manipulating': 'off',
			'svelte/no-object-in-text-mustaches': 'off',
			'no-undef': 'off'
		}
	},
	prettier,
	{
		// Tests, e2e and one-off scripts are allowed to log freely. Production
		// `src/**` code still goes through `no-console: warn` above.
		files: [
			'tests/**/*.{ts,js}',
			'e2e/**/*.{ts,js}',
			'scripts/**/*.{ts,js}',
			'playwright.config.ts',
			'src/**/__tests__/**/*.{ts,js}',
			'src/**/*.test.ts',
			'src/**/*.spec.ts'
		],
		rules: {
			'no-console': 'off'
		}
	},
	{
		// FIX-2026-04-26: was also ignoring tests/** and scripts/** — narrowed to build
		// output only so that tests and scripts are lintable. The TS-project-aware
		// rule blocks above now use src/**-scoped globs, avoiding tsconfig mismatch
		// errors for files not in tsconfig.json.
		// Old ignores:
		// ignores: [
		//   '.svelte-kit/**',
		//   'build/**',
		//   'node_modules/**',
		//   'retired/**',
		//   'playwright-report/**',
		//   'e2e/**',
		//   'tests/**',
		//   'scripts/**',
		//   'test-icons-import.ts',
		//   'playwright.config.ts'
		// ]
		ignores: [
			'.svelte-kit/**',
			'build/**',
			'dist/**',
			'node_modules/**',
			'retired/**',
			'.cls-probe/**'
		]
	}
];

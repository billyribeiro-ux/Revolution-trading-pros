// For more info, see https://github.com/storybookjs/eslint-plugin-storybook#configuration-flat-config-format
import storybook from 'eslint-plugin-storybook';

import js from '@eslint/js';
import svelte from 'eslint-plugin-svelte';
import globals from 'globals';
import ts from 'typescript-eslint';

import prettier from 'eslint-config-prettier';

const SAFE_MODE = true;

export default ts.config(
	js.configs.recommended,
	...ts.configs.recommended,
	...svelte.configs['flat/recommended'],
	prettier,
	...svelte.configs['flat/prettier'],
	{
		languageOptions: {
			globals: {
				...globals.browser,
				...globals.node
			}
		}
	},
	{
		// Ensure Svelte + TS "module" files like `*.svelte.ts` are parsed as TypeScript
		// (they are not `.svelte` components, so they must NOT be handled by the Svelte parser)
		files: ['**/*.svelte.ts'],
		languageOptions: {
			parser: ts.parser
		}
	},
	{
		files: ['**/*.svelte'],
		languageOptions: {
			parserOptions: {
				parser: ts.parser
			}
		}
	},
	{
		files: ['**/*.cjs'],
		rules: {
			'@typescript-eslint/no-require-imports': 'off'
		}
	},
	{
		rules: {
			// SAFE MODE: avoid forcing broad refactors. We keep eslint running, but relax
			// rules that are opinionated or noisy during incremental modernization.

			// TypeScript ergonomics / migration noise
			'@typescript-eslint/no-unused-vars': 'off',
			'@typescript-eslint/no-explicit-any': 'off',
			'@typescript-eslint/no-empty-object-type': 'off',

			// TS comments: do not force @ts-expect-error conversion in safe mode
			'@typescript-eslint/ban-ts-comment': 'off',

			// Minor preference rules (no refactors in safe mode)
			'@typescript-eslint/prefer-as-const': 'off',

			// Svelte 5 / runes ergonomics (don't force $derived refactors in safe mode)
			'svelte/prefer-writable-derived': 'off',

			// Svelte comment / mustache strictness (often used intentionally during migrations)
			'svelte/no-unused-svelte-ignore': 'off',
			'svelte/no-useless-mustaches': 'off',
			'svelte/no-object-in-text-mustaches': 'off',

			// Existing project preferences
			'svelte/no-at-html-tags': 'off',
			'svelte/no-navigation-without-resolve': 'off',
			'svelte/require-each-key': 'off',
			'svelte/prefer-svelte-reactivity': 'off',
			'svelte/no-dom-manipulating': 'off',
			'svelte/no-immutable-reactive-statements': 'off',
			'no-useless-escape': 'off',
			'no-case-declarations': 'off',
			'svelte/infinite-reactive-loop': 'off',
			'svelte/no-shorthand-style-property-overrides': 'off',
			'@typescript-eslint/no-unused-expressions': 'off',

			// Keep standard JS correctness rules enabled
			...(SAFE_MODE
				? {
						'prefer-const': 'off',
						'prefer-spread': 'off',
						'no-undef': 'off'
					}
				: {})
		}
	},
	{
		ignores: [
			'build/',
			'.svelte-kit/',
			'dist/',
			'retired/**',
			'**/*.generated.*',
			'**/generated/**'
		]
	},
	storybook.configs['flat/recommended']
);

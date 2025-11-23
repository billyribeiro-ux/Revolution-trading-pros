import js from '@eslint/js';
import svelte from 'eslint-plugin-svelte';
import globals from 'globals';
import ts from 'typescript-eslint';

import prettier from 'eslint-config-prettier';

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
			'@typescript-eslint/no-unused-vars': 'off', // TODO: Enable and fix
			'@typescript-eslint/no-explicit-any': 'off', // TODO: Enable this later after fixing existing issues
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
			'@typescript-eslint/no-unused-expressions': 'off'
		}
	},
	{
		ignores: ['build/', '.svelte-kit/', 'dist/']
	}
);

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
  {
    files: ['**/*.ts'],
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
      '@typescript-eslint/no-unused-vars': ['warn', { argsIgnorePattern: '^_', varsIgnorePattern: '^_', caughtErrorsIgnorePattern: '^_' }],
      '@typescript-eslint/no-explicit-any': 'off',
      '@typescript-eslint/no-non-null-assertion': 'off',
      '@typescript-eslint/no-invalid-void-type': 'off',
      '@typescript-eslint/no-unused-expressions': 'off',
      '@typescript-eslint/ban-ts-comment': 'off',
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
      'no-console': 'off'
    }
  },
  {
    files: ['**/*.svelte', '**/*.svelte.ts', '**/*.svelte.js'],
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
      '@typescript-eslint/no-unused-vars': ['warn', { argsIgnorePattern: '^_', varsIgnorePattern: '^(\\$|_)', caughtErrorsIgnorePattern: '^_' }],
      '@typescript-eslint/no-explicit-any': 'off',
      '@typescript-eslint/no-non-null-assertion': 'off',
      '@typescript-eslint/no-invalid-void-type': 'off',
      '@typescript-eslint/no-unused-expressions': 'off',
      '@typescript-eslint/ban-ts-comment': 'off',
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
      'no-console': 'off',
      'svelte/no-unused-svelte-ignore': 'warn',
      'svelte/no-shorthand-style-property-overrides': 'warn',
      'svelte/no-at-html-tags': 'off',
      'svelte/require-each-key': 'off',
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
    ignores: [
      '.svelte-kit/**',
      'build/**',
      'node_modules/**',
      'retired/**',
      'playwright-report/**',
      'e2e/**',
      'tests/**',
      'scripts/**',
      'test-icons-import.ts',
      'playwright.config.ts'
    ]
  }
];

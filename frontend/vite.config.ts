import { sveltekit } from '@sveltejs/kit/vite';
import { svelteInspector } from '@sveltejs/vite-plugin-svelte-inspector';
import { defineConfig } from 'vitest/config';
import devtoolsJson from 'vite-plugin-devtools-json';
import tailwindcss from '@tailwindcss/vite';
import { clickToSource } from 'svelte-click-to-source';
import type { Plugin } from 'vite';

export default defineConfig({
  plugins: [
    tailwindcss(),
    clickToSource({ editor: 'windsurf' }) as unknown as Plugin,
    sveltekit(),
    svelteInspector({
      toggleKeyCombo: 'meta-shift',
      showToggleButton: 'always',
      toggleButtonPos: 'bottom-right'
    }),
    devtoolsJson()
  ],
  test: {
    globals: true,
    environment: 'jsdom',
    setupFiles: ['./src/test/setup.ts'],
    include: ['src/**/*.{test,spec}.{js,ts}'],
    coverage: {
      provider: 'v8',
      reporter: ['text', 'json', 'html', 'lcov'],
      include: ['src/lib/**/*.{js,ts,svelte}'],
      exclude: [
        '**/*.test.{js,ts}',
        '**/*.spec.{js,ts}',
        '**/types.ts',
        '**/__tests__/**'
      ],
      thresholds: {
        lines: 80,
        functions: 80,
        branches: 75,
        statements: 80
      }
    },
    mockReset: true,
    restoreMocks: true
  },
  resolve: {
    alias: {
      $lib: '/src/lib'
    }
  },
  build: {
    target: 'es2022',
    rollupOptions: {
      output: {
        manualChunks: {
          'svelte-vendor': ['svelte'],
          'blocks-content': [
            './src/lib/components/cms/blocks/content/ParagraphBlock.svelte',
            './src/lib/components/cms/blocks/content/HeadingBlock.svelte',
            './src/lib/components/cms/blocks/content/QuoteBlock.svelte',
            './src/lib/components/cms/blocks/content/CodeBlock.svelte',
            './src/lib/components/cms/blocks/content/ListBlock.svelte'
          ],
          'blocks-media': [
            './src/lib/components/cms/blocks/media/ImageBlock.svelte',
            './src/lib/components/cms/blocks/media/VideoBlock.svelte',
            './src/lib/components/cms/blocks/media/AudioBlock.svelte',
            './src/lib/components/cms/blocks/media/GalleryBlock.svelte'
          ],
          'blocks-interactive': [
            './src/lib/components/cms/blocks/interactive/AccordionBlock.svelte',
            './src/lib/components/cms/blocks/interactive/TabsBlock.svelte',
            './src/lib/components/cms/blocks/interactive/ToggleBlock.svelte',
            './src/lib/components/cms/blocks/interactive/TocBlock.svelte'
          ],
          'blocks-layout': [
            './src/lib/components/cms/blocks/layout/ColumnsBlock.svelte',
            './src/lib/components/cms/blocks/layout/GroupBlock.svelte',
            './src/lib/components/cms/blocks/layout/DividerBlock.svelte'
          ],
          'blocks-advanced': [
            './src/lib/components/cms/blocks/advanced/CalloutBlock.svelte',
            './src/lib/components/cms/blocks/advanced/CtaBlock.svelte',
            './src/lib/components/cms/blocks/advanced/NewsletterBlock.svelte',
            './src/lib/components/cms/blocks/advanced/TestimonialBlock.svelte',
            './src/lib/components/cms/blocks/advanced/CountdownBlock.svelte',
            './src/lib/components/cms/blocks/advanced/SocialShareBlock.svelte',
            './src/lib/components/cms/blocks/advanced/AuthorBlock.svelte',
            './src/lib/components/cms/blocks/advanced/RelatedPostsBlock.svelte',
            './src/lib/components/cms/blocks/advanced/SpacerBlock.svelte',
            './src/lib/components/cms/blocks/advanced/ButtonBlock.svelte'
          ],
          'collaboration': [
            './src/lib/collaboration/yjs-provider.ts'
          ]
        }
      }
    },
    chunkSizeWarningLimit: 500
  }
});

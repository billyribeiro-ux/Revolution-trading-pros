import adapter from '@sveltejs/adapter-cloudflare';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),
  dynamicCompileOptions({ filename }) {
    if (filename.includes('node_modules')) {
      return { runes: false };
    }
    return { runes: true };
  },
  kit: {
    adapter: adapter({
      routes: {
        include: ['/*'],
        exclude: ['<all>']
      }
    }),
    alias: {
      $lib: 'src/lib',
      $components: 'src/lib/components',
      $stores: 'src/lib/stores',
      $utils: 'src/lib/utils',
      $api: 'src/lib/api'
    },
    prerender: {
      handleHttpError: ({ status, path }) => {
        if (status === 404) {
          return;
        }
        if (status === 500 && path.startsWith('/dashboard')) {
          return;
        }
        throw new Error(`${status} ${path}`);
      },
      handleMissingId: 'ignore',
      handleUnseenRoutes: 'ignore',
      concurrency: 8,
      crawl: true,
      entries: ['*', '/robots.txt']
    },
    inlineStyleThreshold: 0,
    csp: {
      mode: 'auto',
      directives: {
        'default-src': ['self'],
        'script-src': [
          'self',
          'unsafe-inline',
          'https://www.googletagmanager.com',
          'https://www.google-analytics.com',
          'https://static.cloudflareinsights.com',
          'https://apis.google.com'
        ],
        'style-src': ['self', 'unsafe-inline', 'fonts.googleapis.com'],
        'font-src': ['self', 'fonts.gstatic.com', 'data:'],
        'img-src': ['self', 'data:', 'https:', 'blob:'],
        'media-src': [
          'self',
          'https://*.mediadelivery.net',
          'https://vz-5a23b520-193.b-cdn.net',
          'https://*.b-cdn.net',
          'https://pub-2e5bd1b702b440bd888a0fc47f3493ae.r2.dev'
        ],
        'connect-src': [
          'self',
          'ws:',
          'wss:',
          'https:',
          'https://revolution-trading-pros-api.fly.dev',
          'https://revolution-trading-pros.pages.dev',
          'https://www.googleapis.com',
          'https://*.mediadelivery.net',
          'https://vz-5a23b520-193.b-cdn.net'
        ],
        'frame-src': ['self', 'https://iframe.mediadelivery.net', 'https://*.mediadelivery.net'],
        'frame-ancestors': ['none'],
        'base-uri': ['self'],
        'form-action': ['self']
      }
    },
    serviceWorker: {
      register: true
    },
    env: {
      publicPrefix: 'PUBLIC_'
    },
    output: {
      preloadStrategy: 'modulepreload'
    }
  },
  compilerOptions: {
  }
};

export default config;

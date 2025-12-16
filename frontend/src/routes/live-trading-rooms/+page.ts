/**
 * Live Trading Rooms - Page Configuration
 * 
 * Configures prerendering and SSG behavior for optimal performance:
 * - Static generation with server-side data loading
 * - SEO optimization for search engines
 * - Performance enhancements for edge deployment
 * 
 * @see https://kit.svelte.dev/docs/page-options
 * @module routes/live-trading-rooms/+page
 * @version 1.0.0
 */

import type { PageLoad } from './$types';

export const prerender = true;
export const trailingSlash = 'never';
export const ssr = true;
export const csr = true;

// Configure page metadata for SSG
export const config = {
    // Enable static generation with revalidation
    isr: {
        expiration: 300, // 5 minutes
        bypassToken: process.env.ISR_BYPASS_TOKEN
    },
    
    // SEO configuration
    seo: {
        indexable: true,
        followable: true,
        priority: 0.8,
        changeFrequency: 'weekly'
    },
    
    // Performance optimization
    performance: {
        enableCompression: true,
        enableCaching: true,
        enableStreaming: true
    }
};

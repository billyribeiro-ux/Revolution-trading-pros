/// <reference types="@sveltejs/kit" />

declare module '$env/dynamic/private' {
	export const env: {
		VITE_API_URL?: string;
		BACKEND_URL?: string;
		DATABASE_URL?: string;
		JWT_SECRET?: string;
		STRIPE_SECRET_KEY?: string;
		STRIPE_WEBHOOK_SECRET?: string;
		POSTMARK_API_TOKEN?: string;
		UPSTASH_REDIS_REST_URL?: string;
		UPSTASH_REDIS_REST_TOKEN?: string;
		CLOUDFLARE_R2_ACCESS_KEY_ID?: string;
		CLOUDFLARE_R2_SECRET_ACCESS_KEY?: string;
		CLOUDFLARE_R2_BUCKET?: string;
		MEILISEARCH_HOST?: string;
		MEILISEARCH_API_KEY?: string;
		[key: string]: string | undefined;
	};
}

declare module '$env/dynamic/public' {
	export const VITE_API_URL: string | undefined;
	export const VITE_STRIPE_PUBLISHABLE_KEY: string | undefined;
	export const VITE_POSTMARK_API_TOKEN: string | undefined;
	export const VITE_GOOGLE_ANALYTICS_ID: string | undefined;
	export const VITE_META_PIXEL_ID: string | undefined;
}

declare module '$env/static/private' {
	export const VITE_API_URL: string | undefined;
	export const BACKEND_URL: string | undefined;
	export const DATABASE_URL: string | undefined;
	export const JWT_SECRET: string | undefined;
	export const STRIPE_SECRET_KEY: string | undefined;
	export const STRIPE_WEBHOOK_SECRET: string | undefined;
	export const POSTMARK_API_TOKEN: string | undefined;
	export const UPSTASH_REDIS_REST_URL: string | undefined;
	export const UPSTASH_REDIS_REST_TOKEN: string | undefined;
	export const CLOUDFLARE_R2_ACCESS_KEY_ID: string | undefined;
	export const CLOUDFLARE_R2_SECRET_ACCESS_KEY: string | undefined;
	export const CLOUDFLARE_R2_BUCKET: string | undefined;
	export const MEILISEARCH_HOST: string | undefined;
	export const MEILISEARCH_API_KEY: string | undefined;
}

declare module '$env/static/public' {
	export const VITE_API_URL: string | undefined;
	export const VITE_STRIPE_PUBLISHABLE_KEY: string | undefined;
	export const PUBLIC_STRIPE_PUBLISHABLE_KEY: string | undefined;
	export const VITE_POSTMARK_API_TOKEN: string | undefined;
	export const VITE_GOOGLE_ANALYTICS_ID: string | undefined;
	export const VITE_META_PIXEL_ID: string | undefined;
}

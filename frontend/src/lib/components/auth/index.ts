/**
 * Auth Components - Premium Login/Registration UI
 * Netflix L11+ Principal Engineer Grade
 *
 * @version 2.0.0
 */

// Layout Components
export { default as LoginLayout } from './LoginLayout.svelte';
export { default as MobileBackground } from './MobileBackground.svelte';

// Hero Components
export { default as TradingHeroBackground } from './TradingHeroBackground.svelte';
export { default as TestimonialCarousel } from './TestimonialCarousel.svelte';

// 3D Components - DO NOT export statically, use dynamic imports instead
// TradingScene3D and Scene3D use @threlte/core which causes SSR issues
// Import them dynamically: const { default: TradingScene3D } = await import('./TradingScene3D.svelte');

// Form Components
export { default as LoginForm } from './LoginForm.svelte';
export { default as SocialLoginButtons } from './SocialLoginButtons.svelte';

// Animation Components
export { default as TypedHeadline } from './TypedHeadline.svelte';
export { default as LottieSuccess } from './LottieSuccess.svelte';

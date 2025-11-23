/// <reference types="svelte" />
// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
    namespace App {
        // interface Error {}
        // interface Locals {}
        // interface PageData {}
        // interface PageState {}
        // interface Platform {}
    }
    
    interface Window {
        YT?: {
            Player: any;
            PlayerState: {
                UNSTARTED: number;
                ENDED: number;
                PLAYING: number;
                PAUSED: number;
                BUFFERING: number;
                CUED: number;
            };
            ready: (callback: () => void) => void;
        };
        onYouTubeIframeAPIReady?: () => void;
    }
}

export { };

declare global {
    namespace svelteHTML {
        type HTMLAttributes<T> = any;
        type SVGAttributes<T> = any;
    }
}


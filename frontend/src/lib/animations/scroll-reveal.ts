import type { Attachment } from 'svelte/attachments';

/** Scoped route reveals: SSR copy remains readable even if motion cannot load. */
export const scrollReveal: Attachment<HTMLElement> = (root) => {
	let cancelled = false;
	let media: ReturnType<typeof import('gsap').gsap.matchMedia> | undefined;
	void (async () => {
		try {
			const [{ gsap }, { ScrollTrigger }] = await Promise.all([
				import('gsap'),
				import('gsap/ScrollTrigger')
			]);
			if (cancelled) return;
			gsap.registerPlugin(ScrollTrigger);
			media = gsap.matchMedia();
			media.add(
				'(prefers-reduced-motion: no-preference)',
				(context) => {
					const elements = root.querySelectorAll('[data-gsap]');
					for (const element of elements) {
						if (element.getBoundingClientRect().top >= window.innerHeight * 0.85) {
							gsap.set(element, { y: 30 });
						}
					}
					ScrollTrigger.batch(elements, {
						onEnter: (batch) => {
							if (cancelled) return;
							context.add(() =>
								gsap.to(batch, {
									y: 0,
									duration: 0.8,
									ease: 'power3.out',
									stagger: 0.1,
									overwrite: true
								})
							);
						},
						start: 'top 85%',
						once: true
					});
					ScrollTrigger.refresh();
				},
				root
			);
		} catch (error) {
			if (!cancelled) console.error('[Marketing] Scroll animation could not initialize', error);
		}
	})();
	return () => {
		cancelled = true;
		media?.revert();
	};
};

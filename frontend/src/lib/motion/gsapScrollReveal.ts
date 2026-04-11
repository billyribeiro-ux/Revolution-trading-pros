/**
 * Shared ScrollTrigger reveal for marketing pages using `[data-gsap]`.
 * On GSAP failure, forces elements visible so below-the-fold content is never stuck at opacity 0.
 */
export function revealAllDataGsapTargets(): void {
	if (typeof document === 'undefined') return;
	document.querySelectorAll('[data-gsap]').forEach((el) => {
		const node = el as HTMLElement;
		node.style.opacity = '1';
		node.style.transform = 'none';
	});
}

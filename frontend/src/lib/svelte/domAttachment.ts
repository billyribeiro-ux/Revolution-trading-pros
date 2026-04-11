/**
 * Svelte 5 {@attach} helper — preferred over bind:this for raw DOM element refs (5.29+).
 * Returns an attachment that sets the ref on mount and clears on teardown.
 */
export function domRef<T extends HTMLElement>(setter: (el: T | undefined) => void) {
	return (node: Element) => {
		setter(node as T);
		return () => setter(undefined);
	};
}

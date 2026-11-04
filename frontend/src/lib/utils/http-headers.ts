/** Merge every Fetch header representation, replacing names case-insensitively. */
export function mergeHeaders(...sources: (HeadersInit | undefined)[]): Headers {
	const merged = new Headers();
	for (const source of sources) {
		if (!source) continue;
		new Headers(source).forEach((value, name) => merged.set(name, value));
	}
	return merged;
}

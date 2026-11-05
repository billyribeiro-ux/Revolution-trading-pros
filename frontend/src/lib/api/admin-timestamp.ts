/** Rust's timezone-free timestamp aliases in these admin DTOs explicitly contain UTC. */
export function adminUtcTimestamp(value: string): string {
	return /^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?$/.test(value) ? `${value}Z` : value;
}

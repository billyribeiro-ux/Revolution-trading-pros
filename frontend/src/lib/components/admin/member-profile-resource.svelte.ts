import { memberIdentity, type MemberHistoryQuery } from '#lib/api/member-profile.js';
/** One resource per component/panel; no module-global member state or cached history. */
export class MemberProfileResource<T> {
	result = $state.raw<T | null>(null);
	loading = $state(false);
	error = $state('');
	query = $state<MemberHistoryQuery>({ page: 1, per_page: 25 });
	private current = '';
	private generation = 0;
	private controller: AbortController | null = null;
	private readonly loader: (
		member: string,
		query: MemberHistoryQuery,
		signal: AbortSignal
	) => Promise<T>;
	constructor(
		loader: (member: string, query: MemberHistoryQuery, signal: AbortSignal) => Promise<T>
	) {
		this.loader = loader;
	}
	reset() {
		++this.generation;
		this.controller?.abort();
		this.controller = null;
		this.current = '';
		this.result = null;
		this.error = '';
		this.loading = false;
		this.query = { page: 1, per_page: 25 };
	}
	seed(member: string | number, value: T, query: MemberHistoryQuery = { page: 1, per_page: 25 }) {
		this.reset();
		this.current = memberIdentity(member);
		this.result = value;
		this.query = { ...query };
	}
	async load(member: string | number, patch: MemberHistoryQuery = {}): Promise<T | undefined> {
		const generation = ++this.generation;
		this.controller?.abort();
		const controller = new AbortController();
		this.controller = controller;
		this.loading = true;
		this.error = '';
		this.result = null;
		try {
			const id = memberIdentity(member);
			this.query = { ...(this.current === id ? this.query : { page: 1, per_page: 25 }), ...patch };
			this.current = id;
			const result = await this.loader(id, { ...this.query }, controller.signal);
			if (generation !== this.generation || controller.signal.aborted) return;
			this.result = result;
			return result;
		} catch (cause) {
			if (generation === this.generation && !controller.signal.aborted)
				this.error =
					cause instanceof Error ? cause.message : 'The saved member history could not be loaded';
		} finally {
			if (generation === this.generation && !controller.signal.aborted) this.loading = false;
		}
		return undefined;
	}
}

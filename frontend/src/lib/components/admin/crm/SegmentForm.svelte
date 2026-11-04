<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { apiClient } from '#lib/api/client.svelte.js';
	import { apiErrorMessage } from '#lib/utils/api-error-message.js';
	import type { SegmentCondition } from '#lib/crm/types.js';
	let { id }: { id?: string } = $props();
	type Rule = { key: number; field: string; operator: SegmentCondition['operator']; value: string; logic?: 'and' | 'or' };
	let title = $state('');
	let description = $state('');
	let matchType = $state<'all' | 'any'>('all');
	let active = $state(true);
	let nextKey = 1;
	let rules = $state<Rule[]>([{ key: 0, field: 'status', operator: 'equals', value: 'lead' }]);
	let loading = $state(true);
	let saving = $state(false);
	let error = $state('');
	const fields = ['email', 'first_name', 'last_name', 'phone', 'company', 'job_title', 'status', 'source', 'score', 'created_at', 'updated_at', 'last_contacted_at', 'tags'];
	const textOperators: SegmentCondition['operator'][] = ['equals', 'not_equals', 'contains', 'not_contains', 'in', 'not_in', 'is_null', 'is_not_null'];
	const comparisonOperators: SegmentCondition['operator'][] = ['equals', 'not_equals', 'greater_than', 'less_than', 'between', 'in', 'not_in', 'is_null', 'is_not_null'];
	function operators(field: string) { return field === 'score' || field.endsWith('_at') ? comparisonOperators : textOperators; }
	function valueFor(rule: Rule): unknown {
		if (rule.operator === 'is_null' || rule.operator === 'is_not_null') return null;
		const values = ['in', 'not_in', 'between'].includes(rule.operator) ? rule.value.split(',').map((value) => value.trim()) : [rule.value];
		const parsed = rule.field === 'score' ? values.map((value) => {
			const number = Number(value);
			if (!value.trim() || !Number.isFinite(number)) throw new Error('Score rules require numeric values');
			return number;
		}) : values;
		return ['in', 'not_in', 'between'].includes(rule.operator) ? parsed : parsed[0];
	}
	async function load() {
		if (!id) { loading = false; return; }
		loading = true; error = '';
		try {
			const segment = await apiClient.get<{ title: string; description?: string; match_type: 'all' | 'any'; is_active: boolean; conditions: SegmentCondition[] }>(`/admin/crm/segments/${id}?definition=true`, { cache: { enabled: false } });
			title = segment.title; description = segment.description ?? ''; matchType = segment.match_type; active = segment.is_active;
			if (segment.conditions !== null && !Array.isArray(segment.conditions)) {
                rules = [];
                throw new Error('Stored rules have an unsupported format. Replace them with valid rules before saving.');
            }
            rules = (segment.conditions ?? []).map((rule) => ({ key: nextKey++, field: rule.field === 'lead_score' ? 'score' : rule.field === 'company_name' ? 'company' : rule.field, operator: rule.operator, value: Array.isArray(rule.value) ? rule.value.join(', ') : rule.value === null || rule.value === undefined ? '' : String(rule.value), logic: rule.logic }));
		} catch (cause) { error = apiErrorMessage(cause, 'Unable to load segment'); }
		finally { loading = false; }
	}
	async function save(event: SubmitEvent) {
		event.preventDefault(); if (saving) return;
		saving = true; error = '';
		try {
			const conditions: SegmentCondition[] = rules.map((rule) => ({ field: rule.field, operator: rule.operator, value: valueFor(rule), ...(rule.logic ? { logic: rule.logic } : {}) }));
			const payload = { title, description, conditions, match_type: matchType, is_active: active };
			if (id) await apiClient.put(`/admin/crm/segments/${id}`, payload); else await apiClient.post('/admin/crm/segments', payload);
			await goto('/admin/crm/segments', { invalidateAll: true });
		} catch (cause) { error = apiErrorMessage(cause, 'Unable to save segment'); }
		finally { saving = false; }
	}
	onMount(() => { void load(); });
</script>
<section>
	<a href="/admin/crm/segments">← Segments</a><h1>{id ? 'Edit Segment' : 'New Segment'}</h1>
	{#if error}<p role="alert">{error}</p>{#if id}<button onclick={() => void load()}>Retry loading</button>{/if}{/if}
	{#if loading}<p role="status">Loading segment…</p>{:else}
	<form onsubmit={(event) => void save(event)}>
		<label for="segment-title">Segment title</label><input id="segment-title" bind:value={title} required maxlength={255} />
		<label for="segment-description">Description</label><textarea id="segment-description" bind:value={description} maxlength={50000} rows={3}></textarea>
		<label for="segment-match">Match rules</label><select id="segment-match" bind:value={matchType}><option value="all">All rules</option><option value="any">Any rule</option></select>
		<p>Contacts are matched from their current saved values. Use commas for list and between values. Dates accept YYYY-MM-DD or an RFC3339 timestamp.</p>
		{#each rules as rule, index (rule.key)}
		<fieldset><legend>Rule {index + 1}</legend>
			<label for={`rule-field-${rule.key}`}>Contact field</label><select id={`rule-field-${rule.key}`} bind:value={rule.field} onchange={() => { rule.operator = 'equals'; rule.value = ''; }}>{#each fields as field (field)}<option value={field}>{field.replaceAll('_', ' ')}</option>{/each}</select>
			<label for={`rule-operator-${rule.key}`}>Operator</label><select id={`rule-operator-${rule.key}`} bind:value={rule.operator}>{#each operators(rule.field) as operator (operator)}<option value={operator}>{operator.replaceAll('_', ' ')}</option>{/each}</select>
			{#if rule.operator !== 'is_null' && rule.operator !== 'is_not_null'}<label for={`rule-value-${rule.key}`}>Value</label><input id={`rule-value-${rule.key}`} bind:value={rule.value} required maxlength={5000} />{/if}
			{#if index > 0}<label for={`rule-logic-${rule.key}`}>Connect to previous rules</label><select id={`rule-logic-${rule.key}`} value={rule.logic ?? 'default'} onchange={(event) => { const value = event.currentTarget.value; rule.logic = value === 'and' || value === 'or' ? value : undefined; }}><option value="default">Use match rules setting</option><option value="and">And</option><option value="or">Or</option></select>{/if}
			<button type="button" onclick={() => { rules = rules.filter((item) => item.key !== rule.key); }}>Remove Rule {index + 1}</button>
		</fieldset>{/each}
		{#if rules.length === 0}<p>{matchType === 'all' ? 'Without rules, all contacts match.' : 'Without rules, no contacts match.'}</p>{/if}
		<button type="button" disabled={rules.length >= 50} onclick={() => { rules = [...rules, { key: nextKey++, field: 'status', operator: 'equals', value: 'lead' }]; }}>Add Rule</button>
		<label class="check"><input type="checkbox" bind:checked={active} /> Active segment</label>
		<div><button type="submit" disabled={saving}>{saving ? 'Saving…' : 'Save Segment'}</button> <a href="/admin/crm/segments">Cancel</a></div>
	</form>{/if}
</section>
<style>section { max-width: 64rem; padding: 2rem; color: #e2e8f0; } form, fieldset { display: grid; gap: 0.75rem; } fieldset { padding: 1rem; border: 1px solid #475569; border-radius: 0.5rem; } input, select, textarea, button { padding: 0.75rem; border: 1px solid #475569; border-radius: 0.5rem; background: #0f172a; color: inherit; } a { color: #7dd3fc; } [role='alert'] { color: #fca5a5; } .check { display: flex; gap: 0.5rem; }</style>

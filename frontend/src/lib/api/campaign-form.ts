import { SvelteDate } from 'svelte/reactivity';
import type { Campaign, CreateCampaignData } from './campaigns.js';
export interface CampaignForm {
	name: string;
	subject: string;
	subjectB: string;
	useABTest: boolean;
	abTestSplit: number;
	scheduledFor: string;
	segmentId: string;
	templateId: string;
}
export function emptyCampaignForm(): CampaignForm {
	return {
		name: '',
		subject: '',
		subjectB: '',
		useABTest: false,
		abTestSplit: 50,
		scheduledFor: '',
		segmentId: '',
		templateId: ''
	};
}
function localDate(value: string | null | undefined): string {
	if (!value) return '';
	const date = new SvelteDate(value);
	if (!Number.isFinite(date.getTime())) throw new Error('Invalid requested campaign time');
	return new SvelteDate(date.getTime() - date.getTimezoneOffset() * 60000)
		.toISOString()
		.slice(0, -1);
}
export function campaignForm(campaign: Campaign): CampaignForm {
	return {
		name: campaign.name,
		subject: campaign.subject,
		subjectB: campaign.ab_test_config?.subject_b ?? '',
		useABTest: campaign.ab_test_config?.enabled ?? false,
		abTestSplit: campaign.ab_test_config?.split_percentage ?? 50,
		scheduledFor: localDate(campaign.planned_at ?? campaign.scheduled_at),
		segmentId: campaign.segment_id == null ? '' : String(campaign.segment_id),
		templateId: campaign.template_id == null ? '' : String(campaign.template_id)
	};
}
function reference(value: string): number | null {
	if (!value) return null;
	if (!/^[1-9]\d*$/.test(value) || !Number.isSafeInteger(Number(value)))
		throw new Error('Invalid campaign reference');
	return Number(value);
}
export function campaignWrite(form: CampaignForm): CreateCampaignData {
	const scheduled = form.scheduledFor ? Date.parse(form.scheduledFor) : null;
	if (scheduled !== null && !Number.isFinite(scheduled))
		throw new Error('Choose a valid requested time');
	return {
		name: form.name,
		subject: form.subject,
		template_id: reference(form.templateId),
		segment_id: reference(form.segmentId),
		scheduled_at: scheduled === null ? null : new SvelteDate(scheduled).toISOString(),
		ab_test_config: form.useABTest
			? { enabled: true, subject_b: form.subjectB, split_percentage: form.abTestSplit }
			: null
	};
}
/** Send only changed fields, preserving saved creative and started experiment cohorts. */
export function campaignPatch(original: Campaign, form: CampaignForm): Partial<CreateCampaignData> {
	const before = campaignWrite(campaignForm(original)),
		after = campaignWrite(form),
		patch: Partial<CreateCampaignData> = {};
	if (after.name !== before.name) patch.name = after.name;
	if (after.subject !== before.subject) patch.subject = after.subject;
	if (after.template_id !== before.template_id) patch.template_id = after.template_id;
	if (after.segment_id !== before.segment_id) patch.segment_id = after.segment_id;
	if (after.scheduled_at !== before.scheduled_at) patch.scheduled_at = after.scheduled_at;
	if (JSON.stringify(after.ab_test_config) !== JSON.stringify(before.ab_test_config))
		patch.ab_test_config = after.ab_test_config;
	return patch;
}

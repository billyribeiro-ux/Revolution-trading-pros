/**
 * RevolutionCRM-L8-System - API Client
 * FluentCRM Pro Feature Parity - December 2025
 */

import type {
	CampaignDelivery,
	Contact,
	Deal,
	DealWrite,
	DealSummary,
	Pipeline,
	Stage,
	ContactSegment,
	TimelineEvent,
	DealForecast,
	ContactFilters,
	DealFilters,
	// FluentCRM Pro - Email Sequences
	EmailSequence,
	SequenceMail,
	SequenceTracker,
	SequenceFilters,
	SequenceStats,
	// FluentCRM Pro - Recurring Campaigns
	RecurringCampaign,
	RecurringMail,
	RecurringCampaignFilters,
	RecurringCampaignStats,
	// FluentCRM Pro - Smart Links
	SmartLink,
	SmartLinkClick,
	SmartLinkFilters,
	SmartLinkStats,
	SmartLinkAnalytics,
	// FluentCRM Pro - Automation Funnels
	AutomationFunnel,
	FunnelAction,
	FunnelSubscriber,
	FunnelFilters,
	FunnelStats,
	// FluentCRM Pro - Contact Lists
	ContactList,
	// FluentCRM Pro - Contact Tags
	ContactTag,
	// FluentCRM Pro - CRM Companies
	CrmCompany,
	CompanyFilters,
	// FluentCRM Pro - Abandoned Carts
	AbandonedCart,
	AbandonedCartFilters,
	AbandonedCartSettings,
	AbandonedCartStats,
	// FluentCRM Pro - Email Templates
	EmailTemplate,
	TemplateCategory,
	// FluentCRM Pro - Webhooks
	Webhook,
	WebhookLog,
	// FluentCRM Pro - Import/Export
	ImportJob,
	ExportJob,
	// FluentCRM Pro - Manager Permissions
	ManagerRole,
	ManagerUser,
	PermissionModule,
	// FluentCRM Pro - Settings
	DoubleOptInSettings,
	EmailPreferencePage,
	CommunicationType,
	// FluentCRM Pro - System Logs
	SystemLog,
	SystemLogFilters,
	// FluentCRM Pro - Campaigns
	Campaign,
	CampaignStats,
	CampaignWrite,
	CampaignAudiencePreview
} from '#lib/crm/types.js';
import { apiClient, type RequestConfig } from './client.svelte';
import type { JsonValue, PaginatedResponse } from './_types';
import { sequenceId as checkedSequenceId, sequenceEmailId } from './email-sequence-catalog.js';
import {
	crmSequence,
	crmSequenceEnvelope,
	crmSequenceMail,
	crmSequenceTracker,
	sequencePage,
	sequenceAcknowledgement,
	sequenceDeletion,
	sequenceMailList,
	type SequenceWrite,
	type SequenceMailWrite
} from './crm-sequence-contract.js';
const sequenceWriteConfig: RequestConfig = { retry: { attempts: 1, condition: () => false } };

// Administrative records must reflect completed writes and explicit Refresh actions.
function freshGet<T>(endpoint: string, config?: RequestConfig): Promise<T> {
	return apiClient.get<T>(endpoint, { ...config, cache: { ...config?.cache, enabled: false } });
}

type CompanyWire = Omit<CrmCompany, 'id' | 'slug' | 'total_deal_value'> & {
	id: string | number;
	total_deal_value_cents: number;
};

function companyFromWire(company: CompanyWire): CrmCompany {
	return {
		...company,
		id: String(company.id),
		slug: company.name.toLowerCase().replace(/[^a-z0-9]+/g, '-'),
		total_deal_value: company.total_deal_value_cents / 100
	};
}

type SegmentWire = Omit<ContactSegment, 'id'> & { id: string | number };
function segmentFromWire(segment: SegmentWire): ContactSegment {
	return { ...segment, id: String(segment.id), name: segment.title };
}

export class CrmAPI {
	// Contacts
	async getContacts(filters?: ContactFilters): Promise<PaginatedResponse<Contact>> {
		return freshGet('/admin/crm/contacts', {
			preserveEnvelope: true,
			params: filters ?? undefined
		});
	}

	async getContact(id: string): Promise<Contact> {
		return freshGet(`/admin/crm/contacts/${id}`);
	}

	async createContact(data: Partial<Contact>): Promise<Contact> {
		return apiClient.post('/admin/crm/contacts', data);
	}

	async updateContact(id: string, data: Partial<Contact>): Promise<Contact> {
		return apiClient.put(`/admin/crm/contacts/${id}`, data);
	}

	async deleteContact(id: string): Promise<void> {
		return apiClient.delete(`/admin/crm/contacts/${id}`);
	}

	async getContactTimeline(id: string): Promise<TimelineEvent[]> {
		return freshGet(`/admin/crm/contacts/${id}/timeline`);
	}

	async recalculateContactScore(id: string): Promise<Contact> {
		return apiClient.post(`/admin/crm/contacts/${id}/recalculate-score`);
	}

	// Deals
	async getDeals(
		filters?: DealFilters
	): Promise<PaginatedResponse<Deal> & { summaries: DealSummary[] }> {
		return freshGet('/admin/crm/deals', {
			preserveEnvelope: true,
			params: filters ?? undefined
		});
	}

	async getDeal(id: string): Promise<Deal> {
		return freshGet(`/admin/crm/deals/${id}`);
	}

	async createDeal(data: DealWrite): Promise<Deal> {
		return apiClient.post('/admin/crm/deals', data);
	}

	async updateDeal(id: string, data: Partial<DealWrite>): Promise<Deal> {
		return apiClient.put(`/admin/crm/deals/${id}`, data);
	}

	async deleteDeal(id: string): Promise<void> {
		return apiClient.delete(`/admin/crm/deals/${id}`);
	}

	async updateDealStage(id: string, stageId: string, reason?: string): Promise<Deal> {
		return apiClient.patch(`/admin/crm/deals/${id}/stage`, { stage_id: Number(stageId), reason });
	}

	async winDeal(
		id: string,
		wonDetails?: string,
		closeDate?: string,
		stageId?: string
	): Promise<Deal> {
		return apiClient.post(`/admin/crm/deals/${id}/win`, {
			won_details: wonDetails,
			close_date: closeDate,
			stage_id: stageId ? Number(stageId) : undefined
		});
	}

	async loseDeal(id: string, lostReason: string, stageId?: string): Promise<Deal> {
		return apiClient.post(`/admin/crm/deals/${id}/lose`, {
			lost_reason: lostReason,
			stage_id: stageId ? Number(stageId) : undefined
		});
	}

	async getDealForecast(period: string = 'this_month'): Promise<DealForecast> {
		return freshGet('/admin/crm/deals/forecast', { params: { period } });
	}

	// Pipelines
	async getPipelines(): Promise<Pipeline[]> {
		return freshGet('/admin/crm/pipelines');
	}

	async getPipeline(id: string): Promise<Pipeline> {
		return freshGet(`/admin/crm/pipelines/${id}`);
	}

	async createPipeline(data: Partial<Pipeline>): Promise<Pipeline> {
		return apiClient.post('/admin/crm/pipelines', data);
	}

	async updatePipeline(id: string, data: Partial<Pipeline>): Promise<Pipeline> {
		return apiClient.put(`/admin/crm/pipelines/${id}`, data);
	}

	async deletePipeline(id: string): Promise<void> {
		return apiClient.delete(`/admin/crm/pipelines/${id}`);
	}

	async addStage(pipelineId: string, data: Partial<Stage>): Promise<Stage> {
		return apiClient.post(`/admin/crm/pipelines/${pipelineId}/stages`, data);
	}

	// =====================================================
	// FLUENTCRM PRO - EMAIL SEQUENCES (Drip Campaigns)
	// =====================================================

	async getSequences(filters?: SequenceFilters): Promise<PaginatedResponse<EmailSequence>> {
		return sequencePage(
			await freshGet('/admin/crm/sequences', {
				preserveEnvelope: true,
				params: filters ?? undefined
			}),
			crmSequence
		);
	}

	async getSequence(id: string): Promise<{ sequence: EmailSequence; stats: SequenceStats }> {
		id = checkedSequenceId(id);
		return crmSequenceEnvelope(await freshGet(`/admin/crm/sequences/${id}`), id);
	}

	async createSequence(data: SequenceWrite): Promise<EmailSequence> {
		return crmSequence(await apiClient.post('/admin/crm/sequences', data, sequenceWriteConfig));
	}

	async updateSequence(id: string, data: SequenceWrite): Promise<EmailSequence> {
		id = checkedSequenceId(id);
		return crmSequence(
			await apiClient.put(`/admin/crm/sequences/${id}`, data, sequenceWriteConfig),
			id
		);
	}

	async deleteSequence(id: string): Promise<void> {
		id = checkedSequenceId(id);
		sequenceDeletion(await apiClient.delete(`/admin/crm/sequences/${id}`, sequenceWriteConfig), id);
	}

	async duplicateSequence(id: string): Promise<EmailSequence> {
		id = checkedSequenceId(id);
		const copy = crmSequence(
			await apiClient.post(`/admin/crm/sequences/${id}/duplicate`, undefined, sequenceWriteConfig)
		);
		if (copy.id === id || copy.status !== 'draft')
			throw new Error('Sequence duplication was not confirmed');
		return copy;
	}

	async getSequenceEmails(sequenceId: string): Promise<SequenceMail[]> {
		sequenceId = checkedSequenceId(sequenceId);
		return sequenceMailList(
			await freshGet(`/admin/crm/sequences/${sequenceId}/emails`),
			sequenceId
		);
	}

	async createSequenceEmail(
		sequenceId: string,
		data: Partial<SequenceMailWrite>
	): Promise<SequenceMail> {
		sequenceId = checkedSequenceId(sequenceId);
		return crmSequenceMail(
			await apiClient.post(`/admin/crm/sequences/${sequenceId}/emails`, data, sequenceWriteConfig),
			sequenceId,
			data.id
		);
	}

	async updateSequenceEmail(
		sequenceId: string,
		emailId: string,
		data: Partial<SequenceMailWrite>
	): Promise<SequenceMail> {
		sequenceId = checkedSequenceId(sequenceId);
		emailId = sequenceEmailId(emailId);
		return crmSequenceMail(
			await apiClient.put(
				`/admin/crm/sequences/${sequenceId}/emails/${emailId}`,
				data,
				sequenceWriteConfig
			),
			sequenceId,
			emailId
		);
	}

	async deleteSequenceEmail(sequenceId: string, emailId: string): Promise<void> {
		sequenceId = checkedSequenceId(sequenceId);
		emailId = sequenceEmailId(emailId);
		sequenceDeletion(
			await apiClient.delete(
				`/admin/crm/sequences/${sequenceId}/emails/${emailId}`,
				sequenceWriteConfig
			),
			emailId,
			sequenceId
		);
	}

	async reorderSequenceEmails(sequenceId: string, emailIds: string[]): Promise<SequenceMail[]> {
		sequenceId = checkedSequenceId(sequenceId);
		emailIds = emailIds.map(sequenceEmailId);
		return sequenceMailList(
			await apiClient.post(
				`/admin/crm/sequences/${sequenceId}/emails/reorder`,
				{
					email_ids: emailIds
				},
				sequenceWriteConfig
			),
			sequenceId
		);
	}

	async getSequenceSubscribers(
		id: string,
		filters?: { status?: string; per_page?: number; page?: number; search?: string }
	): Promise<PaginatedResponse<SequenceTracker>> {
		id = checkedSequenceId(id);
		return sequencePage(
			await freshGet(`/admin/crm/sequences/${id}/subscribers`, {
				preserveEnvelope: true,
				params: filters ?? undefined
			}),
			(value) => crmSequenceTracker(value, id)
		);
	}

	async subscribeToSequence(
		id: string,
		contactIds: string[],
		startPosition?: number,
		expectedVersion?: number
	): Promise<{ subscribed_count: number }> {
		id = checkedSequenceId(id);
		const saved = await apiClient.post(
			`/admin/crm/sequences/${id}/subscribe`,
			{
				contact_ids: contactIds,
				start_position: startPosition,
				expected_version: expectedVersion
			},
			sequenceWriteConfig
		);
		const count = sequenceAcknowledgement(saved, id, 'subscribed_count');
		if (count > contactIds.length)
			throw new Error('Enrollment acknowledgement exceeded the requested contacts');
		return { subscribed_count: count };
	}

	async unsubscribeFromSequence(
		id: string,
		contactIds: string[],
		reason?: string,
		expectedVersion?: number
	): Promise<{ unsubscribed_count: number }> {
		id = checkedSequenceId(id);
		const saved = await apiClient.post(
			`/admin/crm/sequences/${id}/unsubscribe`,
			{
				contact_ids: contactIds,
				reason,
				expected_version: expectedVersion
			},
			sequenceWriteConfig
		);
		const count = sequenceAcknowledgement(saved, id, 'unsubscribed_count');
		if (count > contactIds.length)
			throw new Error('Removal acknowledgement exceeded the requested contacts');
		return { unsubscribed_count: count };
	}

	// =====================================================
	// FLUENTCRM PRO - RECURRING CAMPAIGNS
	// =====================================================

	async getRecurringCampaigns(
		filters?: RecurringCampaignFilters
	): Promise<PaginatedResponse<RecurringCampaign>> {
		return freshGet('/admin/crm/recurring-campaigns', {
			preserveEnvelope: true,
			params: filters
		});
	}

	async getRecurringCampaign(
		id: string
	): Promise<{ campaign: RecurringCampaign; stats: RecurringCampaignStats }> {
		return freshGet(`/admin/crm/recurring-campaigns/${id}`);
	}

	async createRecurringCampaign(data: Partial<RecurringCampaign>): Promise<RecurringCampaign> {
		return apiClient.post('/admin/crm/recurring-campaigns', data);
	}

	async updateRecurringCampaign(
		id: string,
		data: Partial<RecurringCampaign>
	): Promise<RecurringCampaign> {
		return apiClient.put(`/admin/crm/recurring-campaigns/${id}`, data);
	}

	async deleteRecurringCampaign(id: string): Promise<void> {
		return apiClient.delete(`/admin/crm/recurring-campaigns/${id}`);
	}

	async duplicateRecurringCampaign(id: string): Promise<RecurringCampaign> {
		return apiClient.post(`/admin/crm/recurring-campaigns/${id}/duplicate`);
	}

	async getRecurringCampaignEmails(
		campaignId: string,
		filters?: { status?: string; per_page?: number }
	): Promise<PaginatedResponse<RecurringMail>> {
		return freshGet(`/admin/crm/recurring-campaigns/${campaignId}/emails`, {
			preserveEnvelope: true,
			params: filters
		});
	}

	async triggerRecurringCampaign(id: string): Promise<RecurringMail> {
		return apiClient.post(`/admin/crm/recurring-campaigns/${id}/trigger`);
	}

	// =====================================================
	// FLUENTCRM PRO - SMART LINKS
	// =====================================================

	async getSmartLinks(filters?: SmartLinkFilters): Promise<PaginatedResponse<SmartLink>> {
		return freshGet('/admin/crm/smart-links', { preserveEnvelope: true, params: filters });
	}

	async getSmartLink(id: string): Promise<{ link: SmartLink; stats: SmartLinkStats }> {
		return freshGet(`/admin/crm/smart-links/${id}`);
	}

	async createSmartLink(data: Partial<SmartLink>): Promise<SmartLink> {
		return apiClient.post('/admin/crm/smart-links', data);
	}

	async updateSmartLink(id: string, data: Partial<SmartLink>): Promise<SmartLink> {
		return apiClient.put(`/admin/crm/smart-links/${id}`, data);
	}

	async deleteSmartLink(id: string): Promise<void> {
		return apiClient.delete(`/admin/crm/smart-links/${id}`);
	}

	async getSmartLinkClicks(
		id: string,
		filters?: { per_page?: number }
	): Promise<PaginatedResponse<SmartLinkClick>> {
		return freshGet(`/admin/crm/smart-links/${id}/clicks`, {
			preserveEnvelope: true,
			params: filters
		});
	}

	async getSmartLinkAnalytics(id: string): Promise<SmartLinkAnalytics> {
		return freshGet(`/admin/crm/smart-links/${id}/analytics`);
	}

	// =====================================================
	// FLUENTCRM PRO - AUTOMATION FUNNELS
	// =====================================================

	async getAutomationFunnels(
		filters?: FunnelFilters
	): Promise<PaginatedResponse<AutomationFunnel>> {
		return freshGet('/admin/crm/automations', { preserveEnvelope: true, params: filters });
	}

	async getAutomationFunnel(id: string): Promise<{
		funnel: AutomationFunnel;
		stats: FunnelStats;
		trigger_types: Record<string, string>;
		action_types: Record<string, string>;
	}> {
		return freshGet(`/admin/crm/automations/${id}`);
	}

	async createAutomationFunnel(data: Partial<AutomationFunnel>): Promise<AutomationFunnel> {
		return apiClient.post('/admin/crm/automations', data);
	}

	async updateAutomationFunnel(
		id: string,
		data: Partial<AutomationFunnel>
	): Promise<AutomationFunnel> {
		return apiClient.put(`/admin/crm/automations/${id}`, data);
	}

	async deleteAutomationFunnel(id: string): Promise<void> {
		return apiClient.delete(`/admin/crm/automations/${id}`);
	}

	async duplicateAutomationFunnel(id: string): Promise<AutomationFunnel> {
		return apiClient.post(`/admin/crm/automations/${id}/duplicate`);
	}

	async importAutomationFunnel(data: Record<string, JsonValue>): Promise<AutomationFunnel> {
		return apiClient.post('/admin/crm/automations/import', data);
	}

	async exportAutomationFunnel(id: string): Promise<Record<string, JsonValue>> {
		return freshGet(`/admin/crm/automations/${id}/export`);
	}

	async getTriggerTypes(): Promise<Record<string, string>> {
		return freshGet('/admin/crm/automations/trigger-types');
	}

	async getActionTypes(): Promise<Record<string, string>> {
		return freshGet('/admin/crm/automations/action-types');
	}

	async getFunnelActions(funnelId: string): Promise<FunnelAction[]> {
		return freshGet(`/admin/crm/automations/${funnelId}/actions`);
	}

	async createFunnelAction(funnelId: string, data: Partial<FunnelAction>): Promise<FunnelAction> {
		return apiClient.post(`/admin/crm/automations/${funnelId}/actions`, data);
	}

	async updateFunnelAction(
		funnelId: string,
		actionId: string,
		data: Partial<FunnelAction>
	): Promise<FunnelAction> {
		return apiClient.put(`/admin/crm/automations/${funnelId}/actions/${actionId}`, data);
	}

	async deleteFunnelAction(funnelId: string, actionId: string): Promise<void> {
		return apiClient.delete(`/admin/crm/automations/${funnelId}/actions/${actionId}`);
	}

	async reorderFunnelActions(funnelId: string, actionIds: string[]): Promise<FunnelAction[]> {
		return apiClient.post(`/admin/crm/automations/${funnelId}/actions/reorder`, {
			action_ids: actionIds
		});
	}

	async getFunnelSubscribers(
		funnelId: string,
		filters?: { status?: string; per_page?: number }
	): Promise<PaginatedResponse<FunnelSubscriber>> {
		return freshGet(`/admin/crm/automations/${funnelId}/subscribers`, {
			preserveEnvelope: true,
			params: filters
		});
	}

	async addToFunnel(funnelId: string, contactIds: string[]): Promise<{ added_count: number }> {
		return apiClient.post(`/admin/crm/automations/${funnelId}/add-contacts`, {
			contact_ids: contactIds
		});
	}

	async removeFromFunnel(
		funnelId: string,
		contactIds: string[]
	): Promise<{ removed_count: number }> {
		return apiClient.post(`/admin/crm/automations/${funnelId}/remove-contacts`, {
			contact_ids: contactIds
		});
	}

	// =====================================================
	// FLUENTCRM PRO - CONTACT LISTS
	// =====================================================

	async getContactLists(filters?: {
		page?: number;
		search?: string;
		is_public?: boolean;
		per_page?: number;
	}): Promise<PaginatedResponse<ContactList>> {
		return freshGet('/admin/crm/lists', {
			preserveEnvelope: true,
			cache: { enabled: false },
			params: filters
		});
	}

	async getContactList(id: string): Promise<{ list: ContactList; contacts_count: number }> {
		return freshGet(`/admin/crm/lists/${id}`, { cache: { enabled: false } });
	}

	async createContactList(data: Partial<ContactList>): Promise<ContactList> {
		return apiClient.post('/admin/crm/lists', data);
	}

	async updateContactList(id: string, data: Partial<ContactList>): Promise<ContactList> {
		return apiClient.put(`/admin/crm/lists/${id}`, data);
	}

	async deleteContactList(id: string): Promise<void> {
		return apiClient.delete(`/admin/crm/lists/${id}`, { cache: { enabled: false } });
	}

	async getListContacts(
		id: string,
		filters?: { page?: number; per_page?: number; search?: string }
	): Promise<PaginatedResponse<Contact>> {
		return freshGet(`/admin/crm/lists/${id}/contacts`, {
			preserveEnvelope: true,
			cache: { enabled: false },
			params: filters
		});
	}

	async addContactsToList(id: string, contactIds: string[]): Promise<{ added_count: number }> {
		return apiClient.post(`/admin/crm/lists/${id}/contacts`, { contact_ids: contactIds });
	}

	async removeContactsFromList(
		id: string,
		contactIds: string[]
	): Promise<{ removed_count: number }> {
		return apiClient.delete(`/admin/crm/lists/${id}/contacts`, {
			body: { contact_ids: contactIds }
		});
	}

	// =====================================================
	// FLUENTCRM PRO - CONTACT TAGS
	// =====================================================

	async getContactTags(filters?: {
		search?: string;
		per_page?: number;
	}): Promise<PaginatedResponse<ContactTag>> {
		return freshGet('/admin/crm/contact-tags', { preserveEnvelope: true, params: filters });
	}

	async getContactTag(id: string): Promise<{ tag: ContactTag; contacts_count: number }> {
		return freshGet(`/admin/crm/contact-tags/${id}`);
	}

	async createContactTag(data: Partial<ContactTag>): Promise<ContactTag> {
		return apiClient.post('/admin/crm/contact-tags', data);
	}

	async updateContactTag(id: string, data: Partial<ContactTag>): Promise<ContactTag> {
		return apiClient.put(`/admin/crm/contact-tags/${id}`, data);
	}

	async deleteContactTag(id: string): Promise<void> {
		return apiClient.delete(`/admin/crm/contact-tags/${id}`);
	}

	async getTagContacts(
		id: string,
		filters?: { per_page?: number; page?: number }
	): Promise<PaginatedResponse<Contact>> {
		return freshGet(`/admin/crm/contact-tags/${id}/contacts`, {
			preserveEnvelope: true,
			params: filters
		});
	}

	async applyTagToContacts(id: string, contactIds: string[]): Promise<{ applied_count: number }> {
		return apiClient.post(`/admin/crm/contact-tags/${id}/apply`, { contact_ids: contactIds });
	}

	async removeTagFromContacts(
		id: string,
		contactIds: string[]
	): Promise<{ removed_count: number }> {
		return apiClient.post(`/admin/crm/contact-tags/${id}/remove`, { contact_ids: contactIds });
	}

	async bulkApplyTags(tagIds: string[], contactIds: string[]): Promise<{ applied_count: number }> {
		return apiClient.post('/admin/crm/contact-tags/bulk-apply', {
			tag_ids: tagIds,
			contact_ids: contactIds
		});
	}

	// =====================================================
	// FLUENTCRM PRO - CRM COMPANIES (B2B)
	// =====================================================

	async getCompanies(filters?: CompanyFilters): Promise<PaginatedResponse<CrmCompany>> {
		const response = await freshGet<PaginatedResponse<CompanyWire>>('/admin/crm/companies', {
			params: filters,
			preserveEnvelope: true,
			cache: { enabled: false }
		});
		return { ...response, data: response.data.map(companyFromWire) };
	}

	async getCompany(id: string): Promise<CrmCompany> {
		return companyFromWire(
			await freshGet<CompanyWire>(`/admin/crm/companies/${encodeURIComponent(id)}`, {
				cache: { enabled: false }
			})
		);
	}

	async createCompany(data: Partial<CrmCompany>): Promise<CrmCompany> {
		return companyFromWire(await apiClient.post<CompanyWire>('/admin/crm/companies', data));
	}

	async updateCompany(id: string, data: Partial<CrmCompany>): Promise<CrmCompany> {
		return companyFromWire(
			await apiClient.put<CompanyWire>(`/admin/crm/companies/${encodeURIComponent(id)}`, data)
		);
	}

	async deleteCompany(id: string): Promise<void> {
		return apiClient.delete(`/admin/crm/companies/${id}`);
	}

	async duplicateCompany(id: string): Promise<CrmCompany> {
		return companyFromWire(
			await apiClient.post<CompanyWire>(`/admin/crm/companies/${encodeURIComponent(id)}/duplicate`)
		);
	}

	async getCompanyContacts(
		id: string,
		filters?: { per_page?: number }
	): Promise<PaginatedResponse<Contact>> {
		return freshGet(`/admin/crm/companies/${id}/contacts`, {
			preserveEnvelope: true,
			params: filters
		});
	}

	async getCompanyDeals(
		id: string,
		filters?: { status?: string; per_page?: number }
	): Promise<PaginatedResponse<Deal> & { summaries: DealSummary[] }> {
		return freshGet(`/admin/crm/companies/${id}/deals`, {
			preserveEnvelope: true,
			params: filters
		});
	}

	async getIndustries(): Promise<Record<string, string>> {
		return freshGet('/admin/crm/companies/industries');
	}

	async getCompanySizes(): Promise<Record<string, string>> {
		return freshGet('/admin/crm/companies/sizes');
	}

	// =====================================================
	// FLUENTCRM PRO - ABANDONED CARTS
	// =====================================================

	async getAbandonedCarts(
		filters?: AbandonedCartFilters
	): Promise<PaginatedResponse<AbandonedCart> & { haveAutomation: boolean }> {
		return freshGet('/admin/crm/abandoned-carts', { preserveEnvelope: true, params: filters });
	}

	async getAbandonedCart(id: string): Promise<AbandonedCart> {
		return freshGet(`/admin/crm/abandoned-carts/${id}`);
	}

	async deleteAbandonedCart(id: string): Promise<void> {
		return apiClient.delete(`/admin/crm/abandoned-carts/${id}`);
	}

	async bulkDeleteAbandonedCarts(cartIds: string[]): Promise<{ message: string }> {
		return apiClient.post('/admin/crm/abandoned-carts/bulk-delete', { cart_ids: cartIds });
	}

	async getAbandonedCartStats(dateRange?: string[]): Promise<{ widgets: AbandonedCartStats }> {
		return freshGet('/admin/crm/abandoned-carts/stats', { params: { date_range: dateRange } });
	}

	async getAbandonedCartSettings(): Promise<{
		settings: AbandonedCartSettings;
		wooOptions?: Record<string, JsonValue>;
	}> {
		return freshGet('/admin/crm/abandoned-carts/settings');
	}

	async saveAbandonedCartSettings(
		settings: Partial<AbandonedCartSettings>
	): Promise<{ message: string; reload: boolean }> {
		return apiClient.post('/admin/crm/abandoned-carts/settings', { settings });
	}

	// =====================================================
	// FLUENTCRM PRO - EMAIL TEMPLATES
	// =====================================================

	async getEmailTemplates(filters?: {
		search?: string;
		category?: string;
		page?: number;
		per_page?: number;
	}): Promise<PaginatedResponse<EmailTemplate>> {
		return freshGet('/admin/crm/templates', {
			preserveEnvelope: true,
			cache: { enabled: false },
			params: filters
		});
	}

	async getEmailTemplate(id: string): Promise<EmailTemplate> {
		return freshGet(`/admin/crm/templates/${id}`, { cache: { enabled: false } });
	}

	async createEmailTemplate(data: Partial<EmailTemplate>): Promise<EmailTemplate> {
		return apiClient.post('/admin/crm/templates', data);
	}

	async updateEmailTemplate(id: string, data: Partial<EmailTemplate>): Promise<EmailTemplate> {
		return apiClient.put(`/admin/crm/templates/${id}`, data);
	}

	async deleteEmailTemplate(id: string): Promise<void> {
		return apiClient.delete(`/admin/crm/templates/${id}`);
	}

	async duplicateEmailTemplate(id: string): Promise<EmailTemplate> {
		return apiClient.post(`/admin/crm/templates/${id}/duplicate`);
	}

	async getTemplateCategories(): Promise<TemplateCategory[]> {
		return freshGet('/admin/crm/templates/categories', { cache: { enabled: false } });
	}

	async getPrebuiltTemplates(): Promise<EmailTemplate[]> {
		return freshGet('/admin/crm/templates/prebuilt', { cache: { enabled: false } });
	}

	// =====================================================
	// FLUENTCRM PRO - WEBHOOKS
	// =====================================================

	async getWebhooks(filters?: {
		per_page?: number;
		page?: number;
		search?: string;
		status?: string;
	}): Promise<
		PaginatedResponse<Webhook> & {
			stats: { total: number; active: number; totalTriggers: number; totalFailures: number };
		}
	> {
		return freshGet('/admin/crm/webhooks', { preserveEnvelope: true, params: filters });
	}

	async getWebhook(id: string): Promise<Webhook> {
		return freshGet(`/admin/crm/webhooks/${id}`);
	}

	async createWebhook(data: Partial<Webhook>): Promise<Webhook> {
		return apiClient.post('/admin/crm/webhooks', data);
	}

	async updateWebhook(id: string, data: Partial<Webhook>): Promise<Webhook> {
		return apiClient.put(`/admin/crm/webhooks/${id}`, data);
	}

	async deleteWebhook(id: string): Promise<void> {
		return apiClient.delete(`/admin/crm/webhooks/${id}`);
	}

	async testWebhook(id: string): Promise<{
		delivery_id: string;
		status: string;
		success: boolean | null;
		response_code: number | null;
	}> {
		return apiClient.post(`/admin/crm/webhooks/${id}/test`);
	}

	async getWebhookLogs(
		id: string,
		filters?: { per_page?: number; page?: number }
	): Promise<PaginatedResponse<WebhookLog>> {
		return freshGet(`/admin/crm/webhooks/${id}/logs`, {
			preserveEnvelope: true,
			params: filters
		});
	}

	async getWebhookEvents(): Promise<Record<string, string>> {
		return freshGet('/admin/crm/webhooks/events');
	}

	// =====================================================
	// FLUENTCRM PRO - IMPORT/EXPORT
	// =====================================================

	async getImportJobs(filters?: {
		type?: string;
		status?: string;
		per_page?: number;
	}): Promise<PaginatedResponse<ImportJob>> {
		return freshGet('/admin/crm/imports', { preserveEnvelope: true, params: filters });
	}

	async getImportJob(id: string): Promise<ImportJob> {
		return freshGet(`/admin/crm/imports/${id}`);
	}

	async createImportJob(data: FormData): Promise<ImportJob> {
		return apiClient.post('/admin/crm/imports', data, {
			headers: { 'Content-Type': 'multipart/form-data' }
		});
	}

	async startImportJob(
		id: string,
		mapping: Record<string, string>,
		settings: Record<string, JsonValue>
	): Promise<ImportJob> {
		return apiClient.post(`/admin/crm/imports/${id}/start`, { mapping, settings });
	}

	async cancelImportJob(id: string): Promise<void> {
		return apiClient.post(`/admin/crm/imports/${id}/cancel`);
	}

	async getImportPreview(
		id: string
	): Promise<{ headers: string[]; rows: Array<Record<string, JsonValue>>; total_rows: number }> {
		return freshGet(`/admin/crm/imports/${id}/preview`);
	}

	async getExportJobs(filters?: {
		type?: string;
		per_page?: number;
	}): Promise<PaginatedResponse<ExportJob>> {
		return freshGet('/admin/crm/exports', { preserveEnvelope: true, params: filters });
	}

	async createExportJob(
		type: string,
		filters?: Record<string, JsonValue>,
		fields?: string[]
	): Promise<ExportJob> {
		return apiClient.post('/admin/crm/exports', { type, filters, fields });
	}

	async downloadExport(id: string): Promise<Blob> {
		return freshGet(`/admin/crm/exports/${id}/download`, { responseType: 'blob' });
	}

	async getExportableFields(type: string): Promise<{ field: string; label: string }[]> {
		return freshGet(`/admin/crm/exports/fields/${type}`);
	}

	// =====================================================
	// FLUENTCRM PRO - MANAGER PERMISSIONS
	// =====================================================

	async getManagerRoles(): Promise<{ data: ManagerRole[] }> {
		return freshGet('/admin/crm/manager-roles');
	}

	async getManagerRole(id: string): Promise<ManagerRole> {
		return freshGet(`/admin/crm/manager-roles/${id}`);
	}

	async createManagerRole(data: Partial<ManagerRole>): Promise<ManagerRole> {
		return apiClient.post('/admin/crm/manager-roles', data);
	}

	async updateManagerRole(id: string, data: Partial<ManagerRole>): Promise<ManagerRole> {
		return apiClient.put(`/admin/crm/manager-roles/${id}`, data);
	}

	async deleteManagerRole(id: string): Promise<void> {
		return apiClient.delete(`/admin/crm/manager-roles/${id}`);
	}

	async getManagerUsers(filters?: { per_page?: number }): Promise<PaginatedResponse<ManagerUser>> {
		return freshGet('/admin/crm/managers', { preserveEnvelope: true, params: filters });
	}

	async addManager(userId: string, roleId: string): Promise<ManagerUser> {
		return apiClient.post('/admin/crm/managers', { user_id: userId, role_id: roleId });
	}

	async updateManager(id: string, roleId: string): Promise<ManagerUser> {
		return apiClient.put(`/admin/crm/managers/${id}`, { role_id: roleId });
	}

	async removeManager(id: string): Promise<void> {
		return apiClient.delete(`/admin/crm/managers/${id}`);
	}

	async getPermissionModules(): Promise<PermissionModule[]> {
		return freshGet('/admin/crm/manager-roles/modules');
	}

	// =====================================================
	// FLUENTCRM PRO - SETTINGS
	// =====================================================

	async getDoubleOptInSettings(): Promise<DoubleOptInSettings> {
		return freshGet('/admin/crm/settings/double-optin');
	}

	async saveDoubleOptInSettings(
		settings: Partial<DoubleOptInSettings>
	): Promise<{ message: string }> {
		return apiClient.post('/admin/crm/settings/double-optin', settings);
	}

	async getEmailPreferenceSettings(): Promise<EmailPreferencePage> {
		return freshGet('/admin/crm/settings/email-preferences');
	}

	async saveEmailPreferenceSettings(
		settings: Partial<EmailPreferencePage>
	): Promise<{ message: string }> {
		return apiClient.post('/admin/crm/settings/email-preferences', settings);
	}

	async getCommunicationTypes(): Promise<CommunicationType[]> {
		return freshGet('/admin/crm/settings/communication-types');
	}

	async saveCommunicationType(data: Partial<CommunicationType>): Promise<CommunicationType> {
		return apiClient.post('/admin/crm/settings/communication-types', data);
	}

	async deleteCommunicationType(id: string): Promise<void> {
		return apiClient.delete(`/admin/crm/settings/communication-types/${id}`);
	}

	// =====================================================
	// FLUENTCRM PRO - SYSTEM LOGS
	// =====================================================

	async getSystemLogs(filters?: SystemLogFilters): Promise<PaginatedResponse<SystemLog>> {
		return freshGet('/admin/crm/logs', { preserveEnvelope: true, params: filters });
	}

	async getSystemLog(id: string): Promise<SystemLog> {
		return freshGet(`/admin/crm/logs/${id}`);
	}

	async clearSystemLogs(filters?: {
		search?: string;
		level?: string;
		category?: string;
		before_date?: string;
	}): Promise<{ deleted_count: number }> {
		return apiClient.post('/admin/crm/logs/clear', filters);
	}

	async getLogStats(): Promise<{
		total: number;
		by_level: Record<string, number>;
		by_category: Record<string, number>;
	}> {
		return freshGet('/admin/crm/logs/stats');
	}

	// =====================================================
	// FLUENTCRM PRO - CAMPAIGNS (ONE-TIME)
	// =====================================================

	async getCampaigns(filters?: {
		page?: number;
		status?: string;
		search?: string;
		per_page?: number;
	}): Promise<PaginatedResponse<Campaign> & { stats: CampaignStats }> {
		return freshGet('/admin/crm/campaigns', { preserveEnvelope: true, params: filters });
	}

	async getCampaign(id: string): Promise<Campaign> {
		return freshGet(`/admin/crm/campaigns/${id}`);
	}

	async createCampaign(data: CampaignWrite): Promise<Campaign> {
		return apiClient.post('/admin/crm/campaigns', data);
	}

	async updateCampaign(id: string, data: Partial<CampaignWrite>): Promise<Campaign> {
		return apiClient.put(`/admin/crm/campaigns/${id}`, data);
	}

	async deleteCampaign(id: string): Promise<void> {
		return apiClient.delete(`/admin/crm/campaigns/${id}`);
	}

	async duplicateCampaign(id: string): Promise<Campaign> {
		return apiClient.post(`/admin/crm/campaigns/${id}/duplicate`);
	}

	async scheduleCampaign(id: string, scheduledAt: string): Promise<Campaign> {
		return apiClient.post(`/admin/crm/campaigns/${id}/schedule`, { scheduled_at: scheduledAt });
	}

	async sendCampaignNow(id: string): Promise<Campaign> {
		return apiClient.post(`/admin/crm/campaigns/${id}/send`);
	}

	async pauseCampaign(id: string): Promise<Campaign> {
		return apiClient.post(`/admin/crm/campaigns/${id}/pause`);
	}

	async resumeCampaign(id: string): Promise<Campaign> {
		return apiClient.post(`/admin/crm/campaigns/${id}/resume`);
	}

	async getCampaignStats(): Promise<CampaignStats> {
		return freshGet('/admin/crm/campaigns/stats');
	}
	async getCampaignDeliveries(
		id: string,
		filters?: { page?: number; per_page?: number; status?: string }
	): Promise<{
		data: CampaignDelivery[];
		meta: { current_page: number; per_page: number; total: number; total_pages: number };
	}> {
		return freshGet(`/admin/crm/campaigns/${id}/deliveries`, {
			preserveEnvelope: true,
			params: filters
		});
	}

	async getCampaignRecipients(
		id: string,
		filters?: { per_page?: number; page?: number }
	): Promise<CampaignAudiencePreview> {
		return freshGet(`/admin/crm/campaigns/${id}/recipients`, {
			preserveEnvelope: true,
			params: filters
		});
	}

	// =====================================================
	// FLUENTCRM PRO - CONTACT SEGMENTS
	// =====================================================

	async getContactSegments(filters?: {
		search?: string;
		page?: number;
		is_active?: boolean;
		per_page?: number;
	}): Promise<PaginatedResponse<ContactSegment>> {
		const response = await freshGet<PaginatedResponse<SegmentWire>>('/admin/crm/segments', {
			preserveEnvelope: true,
			cache: { enabled: false },
			params: filters
		});
		return { ...response, data: response.data.map(segmentFromWire) };
	}

	async getContactSegment(
		id: string
	): Promise<{ segment: ContactSegment; contacts_count: number | null }> {
		const segment = segmentFromWire(
			await freshGet<SegmentWire>(`/admin/crm/segments/${id}`, { cache: { enabled: false } })
		);
		return { segment, contacts_count: segment.contacts_count };
	}

	async createContactSegment(data: Partial<ContactSegment>): Promise<ContactSegment> {
		return segmentFromWire(await apiClient.post<SegmentWire>('/admin/crm/segments', data));
	}

	async updateContactSegment(id: string, data: Partial<ContactSegment>): Promise<ContactSegment> {
		return segmentFromWire(await apiClient.put<SegmentWire>(`/admin/crm/segments/${id}`, data));
	}

	async deleteContactSegment(id: string): Promise<void> {
		return apiClient.delete(`/admin/crm/segments/${id}`);
	}

	async duplicateContactSegment(id: string): Promise<ContactSegment> {
		return segmentFromWire(
			await apiClient.post<SegmentWire>(`/admin/crm/segments/${id}/duplicate`)
		);
	}

	async syncContactSegment(id: string): Promise<{ contacts_count: number }> {
		return apiClient.post(`/admin/crm/segments/${id}/sync`);
	}

	async getSegmentContacts(
		id: string,
		filters?: { per_page?: number; page?: number }
	): Promise<PaginatedResponse<Contact>> {
		return freshGet(`/admin/crm/segments/${id}/contacts`, {
			preserveEnvelope: true,
			cache: { enabled: false },
			params: filters
		});
	}
}

export const crmAPI = new CrmAPI();

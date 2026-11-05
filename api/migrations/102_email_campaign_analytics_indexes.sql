-- Bound native campaign reporting by actual acceptance/event observation windows.
CREATE INDEX crm_campaign_acceptance_window ON crm_campaign_deliveries(accepted_at,campaign_id)
 WHERE accepted_at IS NOT NULL;
CREATE INDEX crm_campaign_engagement_window ON crm_campaign_events(occurred_at,delivery_id)
 WHERE record_type IN ('Open','Click');

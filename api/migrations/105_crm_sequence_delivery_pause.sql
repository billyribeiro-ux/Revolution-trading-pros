-- Provider recovery must not resume an operator-paused sequence.
ALTER TABLE crm_campaigns DROP CONSTRAINT crm_campaign_pause_reason_valid;
ALTER TABLE crm_campaigns ADD CONSTRAINT crm_campaign_pause_reason_valid
 CHECK(pause_reason IS NULL OR pause_reason IN
 ('manual','provider_unavailable','uncertain_submission','sequence_paused'));
CREATE INDEX crm_sequence_engine_source ON crm_sequence_engines(sequence_id,campaign_id);

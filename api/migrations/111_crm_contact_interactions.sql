-- Explicit CRM ownership is separate from the linked customer/member account.
ALTER TABLE contacts ADD COLUMN owner_id BIGINT REFERENCES users(id) ON DELETE SET NULL;
CREATE INDEX crm_contact_owner ON contacts(owner_id) WHERE owner_id IS NOT NULL;

CREATE TABLE crm_contact_notes (
 id BIGSERIAL PRIMARY KEY,
 contact_id BIGINT NOT NULL REFERENCES contacts(id) ON DELETE CASCADE,
 content TEXT NOT NULL CHECK(length(btrim(content)) BETWEEN 1 AND 10000),
 created_by BIGINT REFERENCES users(id) ON DELETE SET NULL,
 created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
);
CREATE INDEX crm_contact_note_history ON crm_contact_notes(contact_id,created_at DESC,id DESC);

-- The source contact may be deleted while an authorized submission is in
-- flight. Preserve the original identity and the one durable receipt.
CREATE TABLE crm_contact_mail_requests (
 request_id UUID PRIMARY KEY,
 contact_id BIGINT REFERENCES contacts(id) ON DELETE SET NULL,
 original_contact_id BIGINT NOT NULL CHECK(original_contact_id>0),
 created_by BIGINT REFERENCES users(id) ON DELETE SET NULL,
 recipient TEXT NOT NULL CHECK(recipient=lower(btrim(recipient)) AND length(recipient) BETWEEN 1 AND 255),
 subject TEXT NOT NULL,
 source_hash TEXT NOT NULL CHECK(length(source_hash)=64),
 content_snapshot JSONB NOT NULL CHECK(jsonb_typeof(content_snapshot)='object'),
 template_id BIGINT REFERENCES email_templates(id) ON DELETE SET NULL,
 original_template_id BIGINT,
 email_log_id BIGINT UNIQUE REFERENCES email_logs(id) ON DELETE SET NULL,
 created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
);
CREATE INDEX crm_contact_mail_history ON crm_contact_mail_requests(original_contact_id,created_at DESC,request_id);
CREATE UNIQUE INDEX crm_contact_mail_receipt ON email_logs((model->>'request_id'))
 WHERE template_type='crm_contact_mail' AND model->>'request_id' IS NOT NULL;

-- A recorded activity time is not a visitor-session or human-engagement claim.
CREATE FUNCTION crm_contact_last_activity(record_id BIGINT) RETURNS TIMESTAMPTZ
LANGUAGE SQL STABLE AS $$
 SELECT GREATEST(c.created_at AT TIME ZONE 'UTC',c.updated_at AT TIME ZONE 'UTC',
  c.last_contacted_at AT TIME ZONE 'UTC',
  (SELECT MAX(e.occurred_at) FROM crm_contact_events e WHERE e.contact_id=c.id),
  (SELECT MAX(n.created_at) FROM crm_contact_notes n WHERE n.contact_id=c.id),
  (SELECT MAX(a.created_at) FROM crm_deal_activities a JOIN crm_deals d ON d.id=a.deal_id WHERE d.contact_id=c.id),
  (SELECT MAX(h.created_at) FROM crm_deal_stage_history h JOIN crm_deals d ON d.id=h.deal_id WHERE d.contact_id=c.id),
  (SELECT MAX(GREATEST(d.created_at,d.accepted_at,d.opened_at,d.clicked_at,d.bounced_at,d.unsubscribed_at)) FROM crm_campaign_deliveries d WHERE d.contact_id=c.id),
  (SELECT MAX(GREATEST(r.created_at,e.sent_at)) FROM crm_contact_mail_requests r LEFT JOIN email_logs e ON e.id=r.email_log_id WHERE r.original_contact_id=c.id),
  (SELECT MAX(GREATEST(o.created_at,o.completed_at,o.refunded_at)) AT TIME ZONE 'UTC' FROM orders o WHERE o.user_id=c.user_id),
  (SELECT MAX(GREATEST(e.sent_at,e.queued_at,e.created_at AT TIME ZONE 'UTC')) FROM email_logs e WHERE e.user_id=c.user_id AND COALESCE(e.template_type,'') NOT IN ('crm_contact_mail','native_campaign_test','sequence_test')))
 FROM contacts c WHERE c.id=record_id
$$;

-- Preserve legacy steps/counters. New revisions and enrollments retain their
-- original creative and recipient identity while current authoring can evolve.
ALTER TABLE crm_sequences
 ADD COLUMN slug TEXT,
 ADD COLUMN design_template TEXT NOT NULL DEFAULT 'raw_html',
 ADD COLUMN settings JSONB NOT NULL DEFAULT '{}'::jsonb,
 ADD COLUMN subscriber_settings JSONB NOT NULL DEFAULT '{}'::jsonb,
 ADD COLUMN trigger_config JSONB NOT NULL DEFAULT '{}'::jsonb,
 ADD COLUMN authoring_version BIGINT NOT NULL DEFAULT 1 CHECK(authoring_version>0),
 ADD COLUMN created_by BIGINT REFERENCES users(id) ON DELETE SET NULL,
 ADD COLUMN deleted_at TIMESTAMPTZ,
 ADD COLUMN last_error TEXT;
UPDATE crm_sequences SET slug='sequence-'||id;
ALTER TABLE crm_sequences ALTER COLUMN slug SET NOT NULL;
CREATE UNIQUE INDEX crm_sequence_live_slug ON crm_sequences(LOWER(slug)) WHERE deleted_at IS NULL;
CREATE INDEX crm_sequence_catalog ON crm_sequences(status,created_at DESC,id DESC) WHERE deleted_at IS NULL;

CREATE TABLE crm_sequence_revisions (
 id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
 sequence_id BIGINT NOT NULL REFERENCES crm_sequences(id),
 authoring_version BIGINT NOT NULL CHECK(authoring_version>0),
 definition JSONB NOT NULL CHECK(jsonb_typeof(definition)='object'),
 created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
 UNIQUE(sequence_id,authoring_version),
 UNIQUE(id,sequence_id)
);
CREATE TABLE crm_sequence_trackers (
 id BIGSERIAL PRIMARY KEY,
 sequence_id BIGINT NOT NULL REFERENCES crm_sequences(id),
 revision_id UUID NOT NULL,
 contact_id BIGINT REFERENCES contacts(id) ON DELETE SET NULL,
 contact_email TEXT NOT NULL,
 status TEXT NOT NULL CHECK(status IN ('active','paused','completed','cancelled','failed')),
 last_sequence_mail_id UUID,
 next_sequence_mail_id UUID,
 next_execution_at TIMESTAMPTZ,
 started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
 last_executed_at TIMESTAMPTZ,
 completed_at TIMESTAMPTZ,
 notes JSONB NOT NULL DEFAULT '[]'::jsonb CHECK(jsonb_typeof(notes)='array'),
 last_error TEXT,
 completed_steps INTEGER NOT NULL DEFAULT 0 CHECK(completed_steps>=0),
 total_steps INTEGER NOT NULL CHECK(total_steps BETWEEN 1 AND 50),
 created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
 updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
 FOREIGN KEY(revision_id,sequence_id) REFERENCES crm_sequence_revisions(id,sequence_id),
 CHECK(completed_steps<=total_steps),
 UNIQUE(sequence_id,contact_id)
);
CREATE INDEX crm_sequence_due_tracker ON crm_sequence_trackers(next_execution_at,id) WHERE status='active';
CREATE INDEX crm_sequence_tracker_catalog ON crm_sequence_trackers(sequence_id,status,id);
CREATE UNIQUE INDEX crm_sequence_enrollment_email ON crm_sequence_trackers(sequence_id,LOWER(contact_email));
CREATE TABLE crm_sequence_engines (
 campaign_id BIGINT PRIMARY KEY REFERENCES crm_campaigns(id) ON DELETE CASCADE,
 sequence_id BIGINT NOT NULL REFERENCES crm_sequences(id),
 revision_id UUID NOT NULL,
 mail_id UUID NOT NULL,
 FOREIGN KEY(revision_id,sequence_id) REFERENCES crm_sequence_revisions(id,sequence_id),
 UNIQUE(revision_id,mail_id)
);
ALTER TABLE crm_campaign_deliveries
 ADD COLUMN sequence_tracker_id BIGINT REFERENCES crm_sequence_trackers(id),
 ADD COLUMN sequence_mail_id UUID,
 ADD CONSTRAINT crm_sequence_delivery_pair CHECK((sequence_tracker_id IS NULL)=(sequence_mail_id IS NULL));
CREATE UNIQUE INDEX crm_sequence_step_delivery ON crm_campaign_deliveries(sequence_tracker_id,sequence_mail_id)
 WHERE sequence_tracker_id IS NOT NULL;

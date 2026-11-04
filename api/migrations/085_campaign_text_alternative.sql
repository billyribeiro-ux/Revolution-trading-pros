-- Preserve the newsletter bulk contract's optional text alternative through
-- shared campaign editing, duplication, scheduling and frozen revisions.
ALTER TABLE crm_campaigns ADD COLUMN text_body TEXT;

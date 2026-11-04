CREATE TABLE crm_list_members (
    list_id BIGINT NOT NULL REFERENCES crm_lists(id) ON DELETE CASCADE,
    contact_id BIGINT NOT NULL REFERENCES contacts(id) ON DELETE CASCADE,
    subscribed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (list_id, contact_id)
);
CREATE INDEX crm_list_members_contact_idx ON crm_list_members(contact_id);

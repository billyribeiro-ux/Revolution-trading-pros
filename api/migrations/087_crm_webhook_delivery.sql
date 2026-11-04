ALTER TABLE crm_webhooks ADD COLUMN headers JSONB NOT NULL DEFAULT '{}'::jsonb
    CHECK (jsonb_typeof(headers)='object');
ALTER TABLE crm_webhooks ADD COLUMN created_by BIGINT REFERENCES users(id) ON DELETE SET NULL;
CREATE TABLE crm_webhook_deliveries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    webhook_id BIGINT NOT NULL REFERENCES crm_webhooks(id) ON DELETE CASCADE,
    event TEXT NOT NULL,
    payload JSONB NOT NULL CHECK (jsonb_typeof(payload)='object'),
    status TEXT NOT NULL DEFAULT 'queued' CHECK(status IN ('queued','processing','delivered','failed','cancelled')),
    attempts INTEGER NOT NULL DEFAULT 0 CHECK(attempts BETWEEN 0 AND 3),
    due_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    lease_token UUID,
    lease_expires_at TIMESTAMPTZ,
    response_code INTEGER,
    response_body TEXT,
    error_message TEXT,
    triggered_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    delivered_at TIMESTAMPTZ,
    CHECK(status<>'processing' OR (lease_token IS NOT NULL AND lease_expires_at IS NOT NULL))
);
CREATE INDEX crm_webhook_due ON crm_webhook_deliveries(due_at) WHERE status='queued';
CREATE INDEX crm_webhook_expired ON crm_webhook_deliveries(lease_expires_at) WHERE status='processing';
CREATE INDEX crm_webhook_history ON crm_webhook_deliveries(webhook_id,triggered_at DESC,id DESC);

-- Queue alongside the actual source mutation: rollbacks never leak notifications.
-- One stable event ID is shared by all subscribers and survives delivery retries.
CREATE FUNCTION crm_emit_webhook(event_name TEXT,event_data JSONB) RETURNS VOID
LANGUAGE plpgsql AS $$
DECLARE body JSONB;
BEGIN
    body:=jsonb_build_object('id',gen_random_uuid(),'event',event_name,'timestamp',clock_timestamp(),'data',event_data);
    INSERT INTO crm_webhook_deliveries(webhook_id,event,payload)
        SELECT id,event_name,body FROM crm_webhooks
        WHERE is_active=true AND events ? event_name;
END $$;

CREATE FUNCTION crm_contact_webhook_event() RETURNS TRIGGER LANGUAGE plpgsql AS $$
DECLARE value JSONB;
BEGIN
    IF TG_OP='DELETE' THEN
        PERFORM crm_emit_webhook('contact_deleted',jsonb_build_object('id',OLD.id::text,'email',OLD.email));
        RETURN OLD;
    END IF;
    IF TG_OP='UPDATE' AND to_jsonb(NEW)=to_jsonb(OLD) THEN RETURN NEW; END IF;
    PERFORM crm_emit_webhook(CASE WHEN TG_OP='INSERT' THEN 'contact_created' ELSE 'contact_updated' END,
        jsonb_build_object('id',NEW.id::text,'email',NEW.email,'first_name',NEW.first_name,'last_name',NEW.last_name,'status',NEW.status));
    IF TG_OP='UPDATE' AND NEW.status='unsubscribed' AND OLD.status IS DISTINCT FROM NEW.status THEN
        PERFORM crm_emit_webhook('contact_unsubscribed',jsonb_build_object('id',NEW.id::text,'email',NEW.email));
    END IF;
    IF TG_OP='UPDATE' AND jsonb_typeof(NEW.tags)='array' AND jsonb_typeof(COALESCE(OLD.tags,'[]'::jsonb))='array' THEN
        FOR value IN SELECT v FROM jsonb_array_elements(NEW.tags) v EXCEPT SELECT v FROM jsonb_array_elements(COALESCE(OLD.tags,'[]'::jsonb)) v LOOP
            PERFORM crm_emit_webhook('tag_applied',jsonb_build_object('contact_id',NEW.id::text,'tag',value));
        END LOOP;
        FOR value IN SELECT v FROM jsonb_array_elements(COALESCE(OLD.tags,'[]'::jsonb)) v EXCEPT SELECT v FROM jsonb_array_elements(NEW.tags) v LOOP
            PERFORM crm_emit_webhook('tag_removed',jsonb_build_object('contact_id',NEW.id::text,'tag',value));
        END LOOP;
    END IF;
    RETURN NEW;
END $$;
CREATE TRIGGER crm_contact_webhook AFTER INSERT OR UPDATE OR DELETE ON contacts FOR EACH ROW EXECUTE FUNCTION crm_contact_webhook_event();

CREATE FUNCTION crm_list_webhook_event() RETURNS TRIGGER LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP='DELETE' THEN
        PERFORM crm_emit_webhook('list_removed',jsonb_build_object('contact_id',OLD.contact_id::text,'list_id',OLD.list_id::text));
        RETURN OLD;
    END IF;
    PERFORM crm_emit_webhook('list_applied',jsonb_build_object('contact_id',NEW.contact_id::text,'list_id',NEW.list_id::text));
    RETURN NEW;
END $$;
CREATE TRIGGER crm_list_webhook AFTER INSERT OR DELETE ON crm_list_members FOR EACH ROW EXECUTE FUNCTION crm_list_webhook_event();

CREATE FUNCTION crm_deal_webhook_event() RETURNS TRIGGER LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP='INSERT' THEN
        PERFORM crm_emit_webhook('deal_created',jsonb_build_object('id',NEW.id::text,'name',NEW.name,'contact_id',NEW.contact_id::text,'amount',NEW.amount,'currency',NEW.currency));
    ELSIF NEW.status IN ('won','lost') AND OLD.status IS DISTINCT FROM NEW.status THEN
        PERFORM crm_emit_webhook('deal_'||NEW.status,jsonb_build_object('id',NEW.id::text,'name',NEW.name,'contact_id',NEW.contact_id::text,'amount',NEW.amount,'currency',NEW.currency));
    END IF;
    RETURN NEW;
END $$;
CREATE TRIGGER crm_deal_webhook AFTER INSERT OR UPDATE ON crm_deals FOR EACH ROW EXECUTE FUNCTION crm_deal_webhook_event();

CREATE FUNCTION crm_email_webhook_event() RETURNS TRIGGER LANGUAGE plpgsql AS $$
BEGIN
    IF NEW.record_type IN ('Open','Click') THEN
        PERFORM crm_emit_webhook(CASE WHEN NEW.record_type='Open' THEN 'email_opened' ELSE 'email_clicked' END,
            jsonb_build_object('delivery_id',NEW.delivery_id,'email',NEW.recipient_email,'occurred_at',NEW.occurred_at,'details',NEW.details));
    END IF;
    RETURN NEW;
END $$;
CREATE TRIGGER crm_email_webhook AFTER INSERT ON crm_campaign_events FOR EACH ROW EXECUTE FUNCTION crm_email_webhook_event();

CREATE FUNCTION crm_newsletter_webhook_event() RETURNS TRIGGER LANGUAGE plpgsql AS $$
BEGIN
    IF NEW.status='unsubscribed' AND OLD.status IS DISTINCT FROM NEW.status THEN
        PERFORM crm_emit_webhook('contact_unsubscribed',jsonb_build_object('email',NEW.email,'source','newsletter'));
    END IF;
    RETURN NEW;
END $$;
CREATE TRIGGER crm_newsletter_webhook AFTER UPDATE ON newsletter_subscribers FOR EACH ROW EXECUTE FUNCTION crm_newsletter_webhook_event();

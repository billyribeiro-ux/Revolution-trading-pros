-- Capture actual source transitions in the source transaction. Only sequences
-- active at the event capture participate; no historical events are backfilled.
CREATE TABLE crm_trigger_events (
 id BIGSERIAL PRIMARY KEY,
 source_key TEXT NOT NULL UNIQUE,
 kind TEXT NOT NULL CHECK(kind IN ('signup','purchase','tag_added','custom_event')),
 contact_id BIGINT REFERENCES contacts(id) ON DELETE SET NULL,
 email TEXT NOT NULL,
 payload JSONB NOT NULL CHECK(jsonb_typeof(payload)='object'),
 happened_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
 actor_id BIGINT REFERENCES users(id) ON DELETE SET NULL
);
CREATE TABLE crm_sequence_trigger_matches (
 id BIGSERIAL PRIMARY KEY,
 event_id BIGINT NOT NULL REFERENCES crm_trigger_events(id),
 sequence_id BIGINT NOT NULL REFERENCES crm_sequences(id),
 revision_id UUID NOT NULL,
 status TEXT NOT NULL DEFAULT 'pending' CHECK(status IN ('pending','enrolled','already_enrolled','skipped','cancelled','failed')),
 contact_id BIGINT REFERENCES contacts(id) ON DELETE SET NULL,
 tracker_id BIGINT REFERENCES crm_sequence_trackers(id),
 attempts INTEGER NOT NULL DEFAULT 0 CHECK(attempts>=0),
 available_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
 last_error TEXT,
 created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
 processed_at TIMESTAMPTZ,
 FOREIGN KEY(revision_id,sequence_id) REFERENCES crm_sequence_revisions(id,sequence_id),
 UNIQUE(event_id,sequence_id)
);
CREATE INDEX crm_sequence_trigger_due ON crm_sequence_trigger_matches(available_at,id) WHERE status='pending';
CREATE INDEX crm_sequence_trigger_history ON crm_sequence_trigger_matches(sequence_id,id DESC);

CREATE FUNCTION crm_capture_sequence_targets() RETURNS TRIGGER LANGUAGE plpgsql AS $$
BEGIN
 INSERT INTO crm_sequence_trigger_matches(event_id,sequence_id,revision_id)
 SELECT NEW.id,s.id,r.id FROM crm_sequences s
 JOIN crm_sequence_revisions r ON r.sequence_id=s.id AND r.authoring_version=s.authoring_version
 WHERE s.deleted_at IS NULL AND s.status='active' AND s.trigger_type=NEW.kind
 AND (NEW.kind='signup'
  OR (NEW.kind='tag_added' AND s.trigger_config->>'tag'=NEW.payload->>'tag')
  OR (NEW.kind='custom_event' AND s.trigger_config->>'event_name'=NEW.payload->>'event_name')
  OR (NEW.kind='purchase' AND (NOT(s.trigger_config ? 'product_id')
   OR EXISTS(SELECT 1 FROM jsonb_array_elements_text(COALESCE(NEW.payload->'product_ids','[]'::jsonb)) p(value)
      WHERE p.value=s.trigger_config->>'product_id'))))
 ON CONFLICT(event_id,sequence_id) DO NOTHING;
 RETURN NEW;
END $$;
CREATE TRIGGER crm_trigger_sequence_targets AFTER INSERT ON crm_trigger_events
 FOR EACH ROW EXECUTE FUNCTION crm_capture_sequence_targets();

CREATE FUNCTION crm_capture_signup_event() RETURNS TRIGGER LANGUAGE plpgsql AS $$
BEGIN
 IF TG_TABLE_NAME='users' THEN
  IF TG_OP='INSERT' THEN
   INSERT INTO crm_trigger_events(source_key,kind,email,payload)
   VALUES('signup:user:'||NEW.id,'signup',LOWER(NEW.email),jsonb_build_object('source','member','user_id',NEW.id))
   ON CONFLICT(source_key) DO NOTHING;
  END IF;
 ELSE
  IF NEW.status='confirmed' AND NEW.confirmed_at IS NOT NULL AND NEW.unsubscribed_at IS NULL THEN
   IF TG_OP='INSERT' OR OLD.status IS DISTINCT FROM 'confirmed' OR OLD.confirmed_at IS NULL OR OLD.unsubscribed_at IS NOT NULL THEN
    INSERT INTO crm_trigger_events(source_key,kind,email,payload)
    VALUES('signup:newsletter:'||NEW.id,'signup',LOWER(NEW.email),jsonb_build_object('source','newsletter','subscriber_id',NEW.id))
    ON CONFLICT(source_key) DO NOTHING;
   END IF;
  END IF;
 END IF;
 RETURN NEW;
END $$;
CREATE TRIGGER crm_user_signup_event AFTER INSERT ON users FOR EACH ROW EXECUTE FUNCTION crm_capture_signup_event();
CREATE TRIGGER crm_newsletter_signup_event AFTER INSERT OR UPDATE OF status,confirmed_at,unsubscribed_at ON newsletter_subscribers
 FOR EACH ROW EXECUTE FUNCTION crm_capture_signup_event();

CREATE FUNCTION crm_capture_contact_tags() RETURNS TRIGGER LANGUAGE plpgsql AS $$
DECLARE tag JSONB; previous JSONB;
BEGIN
 previous := CASE WHEN TG_OP='INSERT' THEN '[]'::jsonb ELSE COALESCE(OLD.tags,'[]'::jsonb) END;
 IF jsonb_typeof(NEW.tags)='array' THEN
  FOR tag IN SELECT DISTINCT value FROM jsonb_array_elements(NEW.tags) WHERE jsonb_typeof(value)='string' LOOP
   IF NOT previous @> jsonb_build_array(tag) THEN
    INSERT INTO crm_trigger_events(source_key,kind,contact_id,email,payload)
    VALUES('contact-tag:'||NEW.id||':'||gen_random_uuid(),'tag_added',NEW.id,LOWER(NEW.email),jsonb_build_object('tag',tag));
   END IF;
  END LOOP;
 END IF;
 RETURN NEW;
END $$;
CREATE TRIGGER crm_contact_tag_event AFTER INSERT OR UPDATE OF tags ON contacts
 FOR EACH ROW EXECUTE FUNCTION crm_capture_contact_tags();

CREATE FUNCTION crm_capture_purchase_event() RETURNS TRIGGER LANGUAGE plpgsql AS $$
DECLARE address TEXT; items JSONB;
BEGIN
 IF NEW.status='completed' AND (TG_OP='INSERT' OR OLD.status IS DISTINCT FROM 'completed') THEN
  -- Deferred execution reads the final transaction state and its actual items.
  SELECT LOWER(COALESCE(NULLIF(o.billing_email,''),u.email)) INTO address
   FROM orders o JOIN users u ON u.id=o.user_id WHERE o.id=NEW.id AND o.status='completed';
  IF address IS NOT NULL THEN
   SELECT COALESCE(jsonb_agg(DISTINCT product_id) FILTER(WHERE product_id IS NOT NULL),'[]'::jsonb)
    INTO items FROM order_items WHERE order_id=NEW.id;
   INSERT INTO crm_trigger_events(source_key,kind,email,payload)
   VALUES('purchase:order:'||NEW.id,'purchase',address,jsonb_build_object('order_id',NEW.id,'product_ids',items))
   ON CONFLICT(source_key) DO NOTHING;
  END IF;
 END IF;
 RETURN NEW;
END $$;
CREATE CONSTRAINT TRIGGER crm_order_purchase_event AFTER INSERT OR UPDATE ON orders
 DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION crm_capture_purchase_event();

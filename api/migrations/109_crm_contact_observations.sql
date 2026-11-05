-- Retain historical recorded scores without declaring them measured results.
ALTER TABLE contacts ADD COLUMN historical_score INTEGER;
ALTER TABLE contacts ADD COLUMN score_assessment JSONB
 CHECK(score_assessment IS NULL OR jsonb_typeof(score_assessment)='object');
ALTER TABLE contacts ADD COLUMN score_assessed_at TIMESTAMPTZ;
UPDATE contacts SET historical_score=score;
ALTER TABLE contacts ALTER COLUMN score DROP DEFAULT;

-- Field names, actor and capture time persist independently of operational-log
-- retention. No invented actor or historical change time is backfilled.
CREATE TABLE crm_contact_events (
 id BIGSERIAL PRIMARY KEY,
 contact_id BIGINT NOT NULL CHECK(contact_id>0),
 kind TEXT NOT NULL CHECK(kind IN ('created','updated','deleted','score_assessed')),
 metadata JSONB NOT NULL CHECK(jsonb_typeof(metadata)='object'),
 created_by BIGINT REFERENCES users(id) ON DELETE SET NULL,
 occurred_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
);
CREATE INDEX crm_contact_event_history ON crm_contact_events(contact_id,occurred_at DESC,id DESC);
CREATE INDEX crm_contact_delivery_history ON crm_campaign_deliveries(contact_id,accepted_at DESC,id)
 WHERE contact_id IS NOT NULL;

CREATE FUNCTION crm_invalidate_contact_score() RETURNS TRIGGER LANGUAGE plpgsql AS $$
BEGIN
 IF TG_OP='INSERT' THEN
  NEW.historical_score := COALESCE(NEW.historical_score,NEW.score);
  NEW.score_assessment := NULL;
  NEW.score_assessed_at := NULL;
 ELSIF current_setting('app.crm_score_assessment',true) IS DISTINCT FROM '1'
  AND (to_jsonb(NEW)-ARRAY['historical_score','score_assessment','score_assessed_at'])
      IS DISTINCT FROM (to_jsonb(OLD)-ARRAY['historical_score','score_assessment','score_assessed_at']) THEN
  NEW.score_assessment := CASE WHEN OLD.score_assessment IS NULL THEN NULL
   ELSE jsonb_set(OLD.score_assessment,'{status}','"stale"'::jsonb) END;
  NEW.score_assessed_at := OLD.score_assessed_at;
 END IF;
 RETURN NEW;
END $$;
CREATE TRIGGER crm_contact_score_invalidation BEFORE INSERT OR UPDATE ON contacts
 FOR EACH ROW EXECUTE FUNCTION crm_invalidate_contact_score();

CREATE FUNCTION crm_record_contact_event() RETURNS TRIGGER LANGUAGE plpgsql AS $$
DECLARE fields JSONB; actor BIGINT; actor_text TEXT; event_kind TEXT; record_id BIGINT; details JSONB;
BEGIN
 actor_text := current_setting('app.crm_actor_id',true);
 IF actor_text ~ '^[1-9][0-9]{0,18}$' THEN
  IF actor_text::numeric<=9223372036854775807 THEN
   SELECT id INTO actor FROM users WHERE id=actor_text::bigint;
  END IF;
 END IF;
 IF TG_OP='DELETE' THEN
  record_id := OLD.id; event_kind := 'deleted';
  details := jsonb_build_object('fields','[]'::jsonb);
 ELSIF TG_OP='INSERT' THEN
  record_id := NEW.id; event_kind := 'created';
  details := jsonb_build_object('source_created_at',NEW.created_at,'fields','[]'::jsonb);
 ELSE
  SELECT COALESCE(jsonb_agg(n.key ORDER BY n.key),'[]'::jsonb) INTO fields
   FROM jsonb_each(to_jsonb(NEW)) n WHERE n.value IS DISTINCT FROM to_jsonb(OLD)->n.key;
  IF fields='[]'::jsonb THEN RETURN NEW; END IF;
  record_id := NEW.id;
  event_kind := CASE WHEN current_setting('app.crm_score_assessment',true)='1' THEN 'score_assessed' ELSE 'updated' END;
  details := jsonb_build_object('fields',fields,'score_status',NEW.score_assessment->'status',
   'score',CASE WHEN NEW.score_assessment->>'status'='calculated' THEN NEW.score_assessment->'score' ELSE NULL END,
   'policy_hash',NEW.score_assessment->'policy_hash');
 END IF;
 INSERT INTO crm_contact_events(contact_id,kind,metadata,created_by) VALUES(record_id,event_kind,details,actor);
 IF TG_OP='DELETE' THEN RETURN OLD; END IF;
 RETURN NEW;
END $$;
CREATE TRIGGER crm_contact_record_event AFTER INSERT OR UPDATE OR DELETE ON contacts
 FOR EACH ROW EXECUTE FUNCTION crm_record_contact_event();

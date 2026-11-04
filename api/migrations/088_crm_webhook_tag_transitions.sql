-- Initial tags and clearing tags to NULL are real tag membership transitions.
CREATE OR REPLACE FUNCTION crm_contact_webhook_event() RETURNS TRIGGER LANGUAGE plpgsql AS $$
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
    IF TG_OP IN ('INSERT','UPDATE') THEN
        FOR value IN SELECT v FROM jsonb_array_elements(CASE WHEN jsonb_typeof(NEW.tags)='array' THEN NEW.tags ELSE '[]'::jsonb END) v EXCEPT SELECT v FROM jsonb_array_elements(CASE WHEN TG_OP='UPDATE' AND jsonb_typeof(OLD.tags)='array' THEN OLD.tags ELSE '[]'::jsonb END) v LOOP
            PERFORM crm_emit_webhook('tag_applied',jsonb_build_object('contact_id',NEW.id::text,'tag',value));
        END LOOP;
        FOR value IN SELECT v FROM jsonb_array_elements(CASE WHEN TG_OP='UPDATE' AND jsonb_typeof(OLD.tags)='array' THEN OLD.tags ELSE '[]'::jsonb END) v EXCEPT SELECT v FROM jsonb_array_elements(CASE WHEN jsonb_typeof(NEW.tags)='array' THEN NEW.tags ELSE '[]'::jsonb END) v LOOP
            PERFORM crm_emit_webhook('tag_removed',jsonb_build_object('contact_id',NEW.id::text,'tag',value));
        END LOOP;
    END IF;
    RETURN NEW;
END $$;

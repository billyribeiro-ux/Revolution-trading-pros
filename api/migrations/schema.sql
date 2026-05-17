-- ============================================================================
-- schema.sql -- reconstructed authoritative baseline for FRESH environments
-- Audit item G0.3  --  generated 2026-05-17
-- ============================================================================
--
-- WHAT THIS IS
--   The full net PostgreSQL schema produced by replaying ALL 60 historical
--   sqlx migrations (000..068) on an empty database, WITH a reconstructed
--   pre-sqlx (Laravel-era) baseline prepended. It is a `pg_dump
--   --schema-only --no-owner --no-privileges` of that replayed database.
--
-- WHY IT EXISTS
--   The historical migration chain is NOT reproducible from zero: it assumes
--   a handful of Laravel-era objects/columns that no migration creates, and
--   has three intra-chain ordering/SQL defects. A from-scratch
--   `sqlx migrate run` aborts mid-chain. This file lets CI DB-tests, local
--   dev, and disaster-recovery stand up the CURRENT schema in one shot.
--
-- PROD SAFETY  (read api/migrations/README.md before using this)
--   * PRODUCTION DOES NOT USE THIS FILE. Production keeps replaying the
--     committed 0*.sql chain unchanged (files byte-identical, sqlx checksums
--     intact). Nothing about how the app loads migrations changed.
--   * FRESH ENVIRONMENTS load this file, then seed _sqlx_migrations so
--     `sqlx::migrate!` treats the historical chain as already-applied.
--   * One owner-gated step remains: a maintenance-window cutover of the
--     PRODUCTION database onto this baseline. See README + the G0.3 audit
--     note for the exact pg_dump/diff runbook.
--
-- DO NOT hand-edit. Regenerate via the procedure in
-- docs/audits/G0_3_SCHEMA_BASELINE_2026-05-17.md if the chain changes.
-- ============================================================================

--
-- PostgreSQL database dump
--


-- Dumped from database version 16.13 (Ubuntu 16.13-0ubuntu0.24.04.1)
-- Dumped by pg_dump version 16.13 (Ubuntu 16.13-0ubuntu0.24.04.1)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'SQL_ASCII';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: btree_gin; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS btree_gin WITH SCHEMA public;


--
-- Name: EXTENSION btree_gin; Type: COMMENT; Schema: -; Owner: -
--

COMMENT ON EXTENSION btree_gin IS 'GIN support for scalar types';


--
-- Name: pg_trgm; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS pg_trgm WITH SCHEMA public;


--
-- Name: EXTENSION pg_trgm; Type: COMMENT; Schema: -; Owner: -
--

COMMENT ON EXTENSION pg_trgm IS 'Trigram similarity for fuzzy text matching';


--
-- Name: unaccent; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS unaccent WITH SCHEMA public;


--
-- Name: EXTENSION unaccent; Type: COMMENT; Schema: -; Owner: -
--

COMMENT ON EXTENSION unaccent IS 'Remove accents for search normalization';


--
-- Name: cms_ai_action; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.cms_ai_action AS ENUM (
    'improve',
    'shorten',
    'expand',
    'fix_grammar',
    'change_tone',
    'summarize',
    'generate_faq',
    'generate_meta',
    'generate_alt',
    'suggest_related',
    'custom'
);


--
-- Name: cms_alert_type; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.cms_alert_type AS ENUM (
    'intraday',
    'swing',
    'options',
    'futures'
);


--
-- Name: TYPE cms_alert_type; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TYPE public.cms_alert_type IS 'Categories for alert service offerings';


--
-- Name: cms_asset_type; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.cms_asset_type AS ENUM (
    'image',
    'video',
    'document',
    'audio'
);


--
-- Name: TYPE cms_asset_type; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TYPE public.cms_asset_type IS 'Classification of digital assets by media type';


--
-- Name: cms_block_type; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.cms_block_type AS ENUM (
    'hero-slider',
    'trading-rooms-grid',
    'alert-services-grid',
    'testimonials-masonry',
    'features-grid',
    'rich-text',
    'image',
    'video-embed',
    'spacer',
    'email-capture',
    'blog-feed',
    'indicators-showcase',
    'courses-grid',
    'faq-accordion',
    'pricing-table',
    'cta-banner',
    'stats-counter',
    'two-column-layout'
);


--
-- Name: TYPE cms_block_type; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TYPE public.cms_block_type IS 'All supported block types for the page builder';


--
-- Name: cms_content_mode; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.cms_content_mode AS ENUM (
    'richtext',
    'markdown',
    'html'
);


--
-- Name: TYPE cms_content_mode; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TYPE public.cms_content_mode IS 'Editing mode for content (rich text, markdown, or raw HTML)';


--
-- Name: cms_content_status; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.cms_content_status AS ENUM (
    'draft',
    'in_review',
    'approved',
    'scheduled',
    'published',
    'archived'
);


--
-- Name: TYPE cms_content_status; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TYPE public.cms_content_status IS 'Content workflow status';


--
-- Name: cms_content_type; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.cms_content_type AS ENUM (
    'page',
    'blog_post',
    'alert_service',
    'trading_room',
    'indicator',
    'course',
    'lesson',
    'testimonial',
    'faq',
    'author',
    'topic_cluster',
    'weekly_watchlist',
    'resource',
    'navigation_menu',
    'site_settings',
    'redirect'
);


--
-- Name: TYPE cms_content_type; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TYPE public.cms_content_type IS 'All supported content types in the custom CMS';


--
-- Name: cms_difficulty_level; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.cms_difficulty_level AS ENUM (
    'beginner',
    'intermediate',
    'advanced',
    'all_levels'
);


--
-- Name: TYPE cms_difficulty_level; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TYPE public.cms_difficulty_level IS 'Difficulty levels for courses and content';


--
-- Name: cms_preset_category; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.cms_preset_category AS ENUM (
    'default',
    'brand',
    'seasonal',
    'marketing',
    'trading',
    'custom'
);


--
-- Name: TYPE cms_preset_category; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TYPE public.cms_preset_category IS 'Categories for organizing block presets';


--
-- Name: cms_release_status; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.cms_release_status AS ENUM (
    'draft',
    'scheduled',
    'processing',
    'completed',
    'failed',
    'cancelled'
);


--
-- Name: cms_reusable_block_category; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.cms_reusable_block_category AS ENUM (
    'general',
    'trading',
    'layout',
    'callout',
    'marketing',
    'navigation',
    'media',
    'form'
);


--
-- Name: cms_room_type; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.cms_room_type AS ENUM (
    'day_trading',
    'swing_trading',
    'options',
    'futures',
    'small_accounts'
);


--
-- Name: TYPE cms_room_type; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TYPE public.cms_room_type IS 'Categories for live trading room offerings';


--
-- Name: cms_schedule_action; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.cms_schedule_action AS ENUM (
    'publish',
    'unpublish',
    'archive',
    'update'
);


--
-- Name: cms_schedule_status; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.cms_schedule_status AS ENUM (
    'pending',
    'processing',
    'completed',
    'failed',
    'cancelled'
);


--
-- Name: cms_sync_status; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.cms_sync_status AS ENUM (
    'pending',
    'syncing',
    'synced',
    'conflict',
    'failed'
);


--
-- Name: cms_user_role; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.cms_user_role AS ENUM (
    'super_admin',
    'marketing_manager',
    'content_editor',
    'weekly_editor',
    'developer',
    'viewer'
);


--
-- Name: TYPE cms_user_role; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TYPE public.cms_user_role IS 'CMS user roles with specific permissions';


--
-- Name: cms_webhook_delivery_status; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.cms_webhook_delivery_status AS ENUM (
    'pending',
    'retrying',
    'delivered',
    'failed'
);


--
-- Name: cms_workflow_action; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.cms_workflow_action AS ENUM (
    'submit_for_review',
    'approve',
    'request_changes',
    'reject',
    'schedule',
    'publish',
    'unpublish',
    'archive',
    'restore'
);


--
-- Name: TYPE cms_workflow_action; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TYPE public.cms_workflow_action IS 'Workflow transition actions for audit logging';


--
-- Name: cms_workflow_priority; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.cms_workflow_priority AS ENUM (
    'low',
    'normal',
    'high',
    'urgent'
);


--
-- Name: cms_workflow_stage; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.cms_workflow_stage AS ENUM (
    'draft',
    'in_review',
    'approved',
    'published',
    'archived'
);


--
-- Name: calculate_room_stats(character varying); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.calculate_room_stats(p_room_slug character varying) RETURNS void
    LANGUAGE plpgsql
    AS $_$
DECLARE
    v_wins INTEGER;
    v_losses INTEGER;
    v_total INTEGER;
    v_win_rate DECIMAL(5,2);
    v_active INTEGER;
    v_closed_week INTEGER;
    v_total_pnl DECIMAL(12,2);
    v_weekly_pnl DECIMAL(12,2);
    v_monthly_pnl DECIMAL(12,2);
    v_avg_win DECIMAL(12,2);
    v_avg_loss DECIMAL(12,2);
    v_profit_factor DECIMAL(6,2);
    v_avg_days DECIMAL(6,2);
    v_largest_win DECIMAL(12,2);
    v_largest_loss DECIMAL(12,2);
    v_streak INTEGER;
    v_daily_pnl JSONB;
BEGIN
    -- Count wins and losses
    SELECT 
        COALESCE(SUM(CASE WHEN result = 'WIN' THEN 1 ELSE 0 END), 0),
        COALESCE(SUM(CASE WHEN result = 'LOSS' THEN 1 ELSE 0 END), 0),
        COUNT(*)
    INTO v_wins, v_losses, v_total
    FROM room_trades 
    WHERE room_slug = p_room_slug AND status = 'closed' AND deleted_at IS NULL;
    
    -- Calculate win rate
    IF v_total > 0 THEN
        v_win_rate := (v_wins::DECIMAL / v_total::DECIMAL) * 100;
    ELSE
        v_win_rate := 0;
    END IF;
    
    -- Count active trades
    SELECT COUNT(*) INTO v_active
    FROM room_trades 
    WHERE room_slug = p_room_slug AND status = 'open' AND deleted_at IS NULL;
    
    -- Count closed this week
    SELECT COUNT(*) INTO v_closed_week
    FROM room_trades 
    WHERE room_slug = p_room_slug 
    AND status = 'closed' 
    AND deleted_at IS NULL
    AND exit_date >= CURRENT_DATE - INTERVAL '7 days';
    
    -- Calculate P&L metrics
    SELECT 
        COALESCE(SUM(pnl), 0),
        COALESCE(SUM(CASE WHEN exit_date >= CURRENT_DATE - INTERVAL '7 days' THEN pnl ELSE 0 END), 0),
        COALESCE(SUM(CASE WHEN exit_date >= CURRENT_DATE - INTERVAL '30 days' THEN pnl ELSE 0 END), 0),
        COALESCE(AVG(CASE WHEN result = 'WIN' THEN pnl END), 0),
        COALESCE(AVG(CASE WHEN result = 'LOSS' THEN ABS(pnl) END), 0),
        COALESCE(AVG(holding_days), 0),
        COALESCE(MAX(CASE WHEN pnl > 0 THEN pnl END), 0),
        COALESCE(MAX(CASE WHEN pnl < 0 THEN ABS(pnl) END), 0)
    INTO v_total_pnl, v_weekly_pnl, v_monthly_pnl, v_avg_win, v_avg_loss, v_avg_days, v_largest_win, v_largest_loss
    FROM room_trades 
    WHERE room_slug = p_room_slug AND status = 'closed' AND deleted_at IS NULL;
    
    -- Calculate profit factor
    IF v_avg_loss > 0 THEN
        v_profit_factor := (v_wins * v_avg_win) / NULLIF((v_losses * v_avg_loss), 0);
    ELSE
        v_profit_factor := v_avg_win;
    END IF;
    
    -- Calculate current streak (consecutive wins or losses)
    -- Fixed: avoid nested window functions by using subquery
    WITH recent_trades AS (
        SELECT result, ROW_NUMBER() OVER (ORDER BY exit_date DESC, id DESC) as rn
        FROM room_trades 
        WHERE room_slug = p_room_slug AND status = 'closed' AND deleted_at IS NULL
        ORDER BY exit_date DESC, id DESC
        LIMIT 20
    ),
    with_lag AS (
        SELECT result, rn, LAG(result) OVER (ORDER BY rn) as prev_result
        FROM recent_trades
    ),
    streak_calc AS (
        SELECT result, rn,
            SUM(CASE WHEN result != prev_result OR prev_result IS NULL THEN 1 ELSE 0 END) OVER (ORDER BY rn) as grp
        FROM with_lag
    )
    SELECT COUNT(*) INTO v_streak
    FROM streak_calc
    WHERE grp = 1;
    
    -- Calculate 30-day daily P&L for chart
    -- Fixed: compute cumulative in CTE first, then aggregate
    WITH daily_dates AS (
        SELECT generate_series(
            CURRENT_DATE - INTERVAL '29 days',
            CURRENT_DATE,
            INTERVAL '1 day'
        )::DATE as date
    ),
    daily_trades AS (
        SELECT exit_date, SUM(pnl) as daily_pnl
        FROM room_trades
        WHERE room_slug = p_room_slug AND status = 'closed' AND deleted_at IS NULL
        AND exit_date >= CURRENT_DATE - INTERVAL '30 days'
        GROUP BY exit_date
    ),
    daily_with_pnl AS (
        SELECT 
            d.date,
            COALESCE(t.daily_pnl, 0) as pnl,
            SUM(COALESCE(t.daily_pnl, 0)) OVER (ORDER BY d.date) as cumulative
        FROM daily_dates d
        LEFT JOIN daily_trades t ON d.date = t.exit_date
    )
    SELECT COALESCE(jsonb_agg(
        jsonb_build_object(
            'date', date::TEXT,
            'pnl', pnl,
            'cumulative', cumulative
        ) ORDER BY date
    ), '[]'::jsonb)
    INTO v_daily_pnl
    FROM daily_with_pnl;
    
    -- Upsert stats cache
    INSERT INTO room_stats_cache (
        room_slug, win_rate, weekly_profit, monthly_profit, active_trades,
        closed_this_week, total_trades, wins, losses, avg_win, avg_loss,
        profit_factor, avg_holding_days, largest_win, largest_loss, 
        current_streak, daily_pnl_30d, calculated_at
    ) VALUES (
        p_room_slug, v_win_rate, 
        CASE WHEN v_weekly_pnl >= 0 THEN '+$' || v_weekly_pnl::TEXT ELSE '-$' || ABS(v_weekly_pnl)::TEXT END,
        CASE WHEN v_monthly_pnl >= 0 THEN '+$' || v_monthly_pnl::TEXT ELSE '-$' || ABS(v_monthly_pnl)::TEXT END,
        v_active, v_closed_week, v_wins + v_losses, v_wins, v_losses,
        v_avg_win, v_avg_loss, COALESCE(v_profit_factor, 0), v_avg_days,
        v_largest_win, v_largest_loss, v_streak, v_daily_pnl, NOW()
    )
    ON CONFLICT (room_slug) DO UPDATE SET
        win_rate = EXCLUDED.win_rate,
        weekly_profit = EXCLUDED.weekly_profit,
        monthly_profit = EXCLUDED.monthly_profit,
        active_trades = EXCLUDED.active_trades,
        closed_this_week = EXCLUDED.closed_this_week,
        total_trades = EXCLUDED.total_trades,
        wins = EXCLUDED.wins,
        losses = EXCLUDED.losses,
        avg_win = EXCLUDED.avg_win,
        avg_loss = EXCLUDED.avg_loss,
        profit_factor = EXCLUDED.profit_factor,
        avg_holding_days = EXCLUDED.avg_holding_days,
        largest_win = EXCLUDED.largest_win,
        largest_loss = EXCLUDED.largest_loss,
        current_streak = EXCLUDED.current_streak,
        daily_pnl_30d = EXCLUDED.daily_pnl_30d,
        calculated_at = NOW();
END;
$_$;


--
-- Name: cleanup_expired_oauth_states(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cleanup_expired_oauth_states() RETURNS void
    LANGUAGE plpgsql
    AS $$
BEGIN
    DELETE FROM oauth_states WHERE expires_at < NOW();
END;
$$;


--
-- Name: cleanup_expired_rate_limits(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cleanup_expired_rate_limits() RETURNS integer
    LANGUAGE plpgsql
    AS $$
DECLARE
    deleted_count INTEGER;
BEGIN
    DELETE FROM login_rate_limits
    WHERE last_attempt_at < NOW() - INTERVAL '1 hour'
    AND (locked_until IS NULL OR locked_until < NOW());

    GET DIAGNOSTICS deleted_count = ROW_COUNT;
    RETURN deleted_count;
END;
$$;


--
-- Name: cleanup_old_mfa_attempts(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cleanup_old_mfa_attempts() RETURNS integer
    LANGUAGE plpgsql
    AS $$
DECLARE
    deleted_count INTEGER;
BEGIN
    DELETE FROM mfa_attempts
    WHERE created_at < NOW() - INTERVAL '24 hours';

    GET DIAGNOSTICS deleted_count = ROW_COUNT;
    RETURN deleted_count;
END;
$$;


--
-- Name: cleanup_old_resource_access(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cleanup_old_resource_access() RETURNS void
    LANGUAGE plpgsql
    AS $$
BEGIN
    DELETE FROM resource_access_log
    WHERE id IN (
        SELECT id FROM (
            SELECT id, ROW_NUMBER() OVER (PARTITION BY user_id ORDER BY accessed_at DESC) as rn
            FROM resource_access_log
        ) ranked
        WHERE rn > 100
    );
END;
$$;


--
-- Name: FUNCTION cleanup_old_resource_access(); Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON FUNCTION public.cleanup_old_resource_access() IS 'Keeps only the last 100 accessed resources per user';


--
-- Name: cms_auto_generate_structured_data(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_auto_generate_structured_data() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    IF NEW.status = 'published' AND NEW.structured_data IS NULL THEN
        NEW.structured_data := cms_generate_structured_data(NEW.id);
    END IF;
    RETURN NEW;
END;
$$;


--
-- Name: cms_cleanup_expired_preview_tokens(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_cleanup_expired_preview_tokens() RETURNS integer
    LANGUAGE plpgsql
    AS $$
DECLARE
    deleted_count INTEGER;
BEGIN
    DELETE FROM cms_preview_tokens
    WHERE expires_at < NOW()
       OR (max_views IS NOT NULL AND view_count >= max_views);
    
    GET DIAGNOSTICS deleted_count = ROW_COUNT;
    RETURN deleted_count;
END;
$$;


--
-- Name: cms_cleanup_old_schedules(integer); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_cleanup_old_schedules(p_days_old integer DEFAULT 90) RETURNS integer
    LANGUAGE plpgsql
    AS $$
DECLARE
    deleted_count INTEGER;
BEGIN
    WITH deleted AS (
        DELETE FROM cms_schedules
        WHERE status IN ('completed', 'failed', 'cancelled')
          AND executed_at < NOW() - (p_days_old || ' days')::INTERVAL
        RETURNING id
    )
    SELECT COUNT(*) INTO deleted_count FROM deleted;

    RETURN deleted_count;
END;
$$;


--
-- Name: cms_compute_revision_diff(uuid, integer, integer); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_compute_revision_diff(p_content_id uuid, p_version_from integer, p_version_to integer) RETURNS jsonb
    LANGUAGE plpgsql
    AS $$
DECLARE
    v_from_data JSONB;
    v_to_data JSONB;
    v_from_title TEXT;
    v_to_title TEXT;
    v_from_content TEXT;
    v_to_content TEXT;
    v_added_blocks INTEGER := 0;
    v_removed_blocks INTEGER := 0;
    v_modified_blocks INTEGER := 0;
    v_result JSONB;
BEGIN
    -- Get revision data
    SELECT data INTO v_from_data
    FROM cms_revisions
    WHERE content_id = p_content_id AND revision_number = p_version_from;

    SELECT data INTO v_to_data
    FROM cms_revisions
    WHERE content_id = p_content_id AND revision_number = p_version_to;

    IF v_from_data IS NULL OR v_to_data IS NULL THEN
        RETURN NULL;
    END IF;

    -- Extract titles
    v_from_title := v_from_data->>'title';
    v_to_title := v_to_data->>'title';

    -- Extract content
    v_from_content := v_from_data->>'content';
    v_to_content := v_to_data->>'content';

    -- Count block changes (simplified - frontend does detailed diff)
    v_added_blocks := COALESCE(jsonb_array_length(v_to_data->'content_blocks'), 0) -
                      COALESCE(jsonb_array_length(v_from_data->'content_blocks'), 0);
    IF v_added_blocks < 0 THEN
        v_removed_blocks := ABS(v_added_blocks);
        v_added_blocks := 0;
    END IF;

    -- Build result
    v_result := jsonb_build_object(
        'version_from', p_version_from,
        'version_to', p_version_to,
        'title_changed', v_from_title IS DISTINCT FROM v_to_title,
        'content_changed', v_from_content IS DISTINCT FROM v_to_content,
        'blocks_added', v_added_blocks,
        'blocks_removed', v_removed_blocks,
        'from_data', v_from_data,
        'to_data', v_to_data
    );

    RETURN v_result;
END;
$$;


--
-- Name: cms_content_webhook_trigger(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_content_webhook_trigger() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
DECLARE
    v_event TEXT;
    v_payload JSONB;
BEGIN
    -- Determine event type
    IF TG_OP = 'INSERT' THEN
        v_event := 'content.created';
    ELSIF TG_OP = 'UPDATE' THEN
        IF NEW.status = 'published' AND OLD.status != 'published' THEN
            v_event := 'content.published';
        ELSIF NEW.status != 'published' AND OLD.status = 'published' THEN
            v_event := 'content.unpublished';
        ELSIF NEW.deleted_at IS NOT NULL AND OLD.deleted_at IS NULL THEN
            v_event := 'content.deleted';
        ELSE
            v_event := 'content.updated';
        END IF;
    END IF;

    -- Build payload
    v_payload := jsonb_build_object(
        'contentId', NEW.id,
        'contentType', NEW.content_type,
        'slug', NEW.slug,
        'title', NEW.title,
        'status', NEW.status,
        'timestamp', NOW()
    );

    -- Queue webhooks
    PERFORM cms_queue_webhook_delivery(v_event, NEW.id, v_payload);

    RETURN NEW;
END;
$$;


--
-- Name: cms_create_revision(uuid, jsonb, uuid, character varying, text[]); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_create_revision(p_content_id uuid, p_data jsonb, p_created_by uuid, p_change_summary character varying DEFAULT NULL::character varying, p_changed_fields text[] DEFAULT NULL::text[]) RETURNS uuid
    LANGUAGE plpgsql
    AS $$
DECLARE
    v_revision_number INTEGER;
    v_new_id UUID;
BEGIN
    -- Get next revision number
    SELECT COALESCE(MAX(revision_number), 0) + 1 INTO v_revision_number
    FROM cms_revisions
    WHERE content_id = p_content_id;

    -- Mark previous revision as not current
    UPDATE cms_revisions
    SET is_current = false
    WHERE content_id = p_content_id;

    -- Insert new revision
    INSERT INTO cms_revisions (
        content_id, revision_number, is_current,
        data, change_summary, changed_fields, created_by
    ) VALUES (
        p_content_id, v_revision_number, true,
        p_data, p_change_summary, p_changed_fields, p_created_by
    ) RETURNING id INTO v_new_id;

    RETURN v_new_id;
END;
$$;


--
-- Name: FUNCTION cms_create_revision(p_content_id uuid, p_data jsonb, p_created_by uuid, p_change_summary character varying, p_changed_fields text[]); Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON FUNCTION public.cms_create_revision(p_content_id uuid, p_data jsonb, p_created_by uuid, p_change_summary character varying, p_changed_fields text[]) IS 'Create a new revision for content versioning';


--
-- Name: cms_create_workflow_status(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_create_workflow_status() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    INSERT INTO cms_workflow_status (content_id, current_stage)
    VALUES (NEW.id, 'draft'::cms_workflow_stage)
    ON CONFLICT (content_id) DO NOTHING;
    RETURN NEW;
END;
$$;


--
-- Name: cms_current_user_id(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_current_user_id() RETURNS uuid
    LANGUAGE plpgsql STABLE
    AS $$
BEGIN
    RETURN current_setting('cms.user_id', true)::uuid;
EXCEPTION WHEN OTHERS THEN
    RETURN NULL;
END;
$$;


--
-- Name: cms_current_user_role(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_current_user_role() RETURNS public.cms_user_role
    LANGUAGE plpgsql STABLE
    AS $$
DECLARE
    v_role cms_user_role;
BEGIN
    -- This will be set by the application via SET LOCAL
    v_role := current_setting('cms.user_role', true)::cms_user_role;
    RETURN COALESCE(v_role, 'viewer'::cms_user_role);
EXCEPTION WHEN OTHERS THEN
    RETURN 'viewer'::cms_user_role;
END;
$$;


--
-- Name: cms_datasource_entries_updated_at(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_datasource_entries_updated_at() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$;


--
-- Name: cms_datasource_entry_count(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_datasource_entry_count() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE cms_datasources SET entry_count = entry_count + 1 WHERE id = NEW.datasource_id;
        RETURN NEW;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE cms_datasources SET entry_count = entry_count - 1 WHERE id = OLD.datasource_id;
        RETURN OLD;
    END IF;
    RETURN NULL;
END;
$$;


--
-- Name: cms_datasources_updated_at(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_datasources_updated_at() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$;


--
-- Name: cms_generate_structured_data(uuid); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_generate_structured_data(p_content_id uuid) RETURNS jsonb
    LANGUAGE plpgsql STABLE
    AS $$
DECLARE
    v_content RECORD;
    v_author RECORD;
    v_site RECORD;
    v_image RECORD;
    v_result JSONB;
    v_base_url TEXT := 'https://revolution-trading-pros.com';
BEGIN
    -- Get content
    SELECT * INTO v_content FROM cms_content WHERE id = p_content_id;
    IF v_content IS NULL THEN RETURN NULL; END IF;

    -- Get author
    SELECT * INTO v_author FROM cms_users WHERE id = v_content.author_id;

    -- Get site settings
    SELECT * INTO v_site FROM cms_site_settings LIMIT 1;

    -- Get featured image
    SELECT * INTO v_image FROM cms_assets WHERE id = v_content.featured_image_id;

    -- Build base structured data
    CASE v_content.content_type
        WHEN 'blog_post' THEN
            v_result := jsonb_build_object(
                '@context', 'https://schema.org',
                '@type', 'BlogPosting',
                'headline', v_content.title,
                'description', COALESCE(v_content.meta_description, v_content.excerpt),
                'datePublished', v_content.published_at,
                'dateModified', v_content.updated_at,
                'mainEntityOfPage', jsonb_build_object(
                    '@type', 'WebPage',
                    '@id', v_base_url || '/blog/' || v_content.slug
                ),
                'publisher', jsonb_build_object(
                    '@type', 'Organization',
                    'name', COALESCE(v_site.site_name, 'Revolution Trading Pros'),
                    'url', v_base_url
                )
            );

        WHEN 'course' THEN
            v_result := jsonb_build_object(
                '@context', 'https://schema.org',
                '@type', 'Course',
                'name', v_content.title,
                'description', COALESCE(v_content.meta_description, v_content.excerpt),
                'provider', jsonb_build_object(
                    '@type', 'Organization',
                    'name', COALESCE(v_site.site_name, 'Revolution Trading Pros'),
                    'url', v_base_url
                ),
                'dateCreated', v_content.created_at,
                'dateModified', v_content.updated_at
            );

        WHEN 'faq' THEN
            v_result := jsonb_build_object(
                '@context', 'https://schema.org',
                '@type', 'FAQPage',
                'mainEntity', COALESCE(v_content.custom_fields->'faqs', '[]'::jsonb)
            );

        ELSE
            v_result := jsonb_build_object(
                '@context', 'https://schema.org',
                '@type', 'WebPage',
                'name', v_content.title,
                'description', COALESCE(v_content.meta_description, v_content.excerpt),
                'datePublished', v_content.published_at,
                'dateModified', v_content.updated_at
            );
    END CASE;

    -- Add author if exists
    IF v_author IS NOT NULL THEN
        v_result := v_result || jsonb_build_object(
            'author', jsonb_build_object(
                '@type', 'Person',
                'name', v_author.display_name,
                'url', v_base_url || '/authors/' || v_author.id
            )
        );
    END IF;

    -- Add image if exists
    IF v_image IS NOT NULL THEN
        v_result := v_result || jsonb_build_object(
            'image', jsonb_build_object(
                '@type', 'ImageObject',
                'url', v_image.cdn_url,
                'width', v_image.width,
                'height', v_image.height
            )
        );
    END IF;

    RETURN v_result;
END;
$$;


--
-- Name: FUNCTION cms_generate_structured_data(p_content_id uuid); Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON FUNCTION public.cms_generate_structured_data(p_content_id uuid) IS 'Generate JSON-LD structured data for SEO based on content type';


--
-- Name: cms_get_all_slugs(public.cms_content_type, character varying); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_get_all_slugs(p_content_type public.cms_content_type DEFAULT NULL::public.cms_content_type, p_locale character varying DEFAULT NULL::character varying) RETURNS TABLE(content_type public.cms_content_type, slug character varying, locale character varying, updated_at timestamp with time zone)
    LANGUAGE plpgsql STABLE
    AS $$
BEGIN
    RETURN QUERY
    SELECT c.content_type, c.slug, c.locale, c.updated_at
    FROM cms_content c
    WHERE c.status = 'published'
    AND c.deleted_at IS NULL
    AND (p_content_type IS NULL OR c.content_type = p_content_type)
    AND (p_locale IS NULL OR c.locale = p_locale)
    ORDER BY c.updated_at DESC;
END;
$$;


--
-- Name: FUNCTION cms_get_all_slugs(p_content_type public.cms_content_type, p_locale character varying); Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON FUNCTION public.cms_get_all_slugs(p_content_type public.cms_content_type, p_locale character varying) IS 'Get all published content slugs for static site generation';


--
-- Name: cms_get_block_types_with_presets(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_get_block_types_with_presets() RETURNS TABLE(block_type character varying, preset_count bigint, has_default boolean)
    LANGUAGE plpgsql STABLE
    AS $$
BEGIN
    RETURN QUERY
    SELECT
        p.block_type,
        COUNT(*)::BIGINT AS preset_count,
        BOOL_OR(p.is_default) AS has_default
    FROM cms_presets p
    WHERE p.deleted_at IS NULL
    AND p.is_global = true
    GROUP BY p.block_type
    ORDER BY p.block_type;
END;
$$;


--
-- Name: FUNCTION cms_get_block_types_with_presets(); Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON FUNCTION public.cms_get_block_types_with_presets() IS 'Get list of block types that have available presets';


--
-- Name: cms_get_content_full(public.cms_content_type, character varying, character varying, boolean); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_get_content_full(p_content_type public.cms_content_type, p_slug character varying, p_locale character varying DEFAULT 'en'::character varying, p_include_drafts boolean DEFAULT false) RETURNS jsonb
    LANGUAGE plpgsql STABLE SECURITY DEFINER
    AS $$
DECLARE
    v_result JSONB;
    v_content_id UUID;
BEGIN
    -- Get content ID first
    SELECT c.id INTO v_content_id
    FROM cms_content c
    WHERE c.content_type = p_content_type
    AND c.slug = p_slug
    AND c.locale = p_locale
    AND c.deleted_at IS NULL
    AND (p_include_drafts OR c.status = 'published');

    IF v_content_id IS NULL THEN
        RETURN NULL;
    END IF;

    -- Build full content response with relations
    SELECT jsonb_build_object(
        'id', c.id,
        'contentType', c.content_type,
        'slug', c.slug,
        'locale', c.locale,
        'title', c.title,
        'subtitle', c.subtitle,
        'excerpt', c.excerpt,
        'content', c.content,
        'contentMode', c.content_mode,
        'contentBlocks', c.content_blocks,
        'status', c.status,
        'version', c.version,
        'template', c.template,
        'customFields', c.custom_fields,

        -- SEO fields
        'seo', jsonb_build_object(
            'title', COALESCE(c.meta_title, c.title),
            'description', c.meta_description,
            'keywords', c.meta_keywords,
            'canonicalUrl', c.canonical_url,
            'robots', c.robots_directives,
            'structuredData', c.structured_data
        ),

        -- Featured image
        'featuredImage', CASE WHEN fi.id IS NOT NULL THEN jsonb_build_object(
            'id', fi.id,
            'url', fi.cdn_url,
            'width', fi.width,
            'height', fi.height,
            'alt', COALESCE(fi.alt_text, c.title),
            'blurhash', fi.blurhash,
            'variants', fi.variants
        ) END,

        -- OG image (fallback to featured)
        'ogImage', CASE WHEN oi.id IS NOT NULL THEN jsonb_build_object(
            'id', oi.id,
            'url', oi.cdn_url,
            'width', oi.width,
            'height', oi.height
        ) ELSE CASE WHEN fi.id IS NOT NULL THEN jsonb_build_object(
            'id', fi.id,
            'url', fi.cdn_url,
            'width', fi.width,
            'height', fi.height
        ) END END,

        -- Author
        'author', CASE WHEN u.id IS NOT NULL THEN jsonb_build_object(
            'id', u.id,
            'displayName', u.display_name,
            'avatarUrl', u.avatar_url,
            'bio', u.bio
        ) END,

        -- Tags
        'tags', COALESCE((
            SELECT jsonb_agg(jsonb_build_object(
                'id', t.id,
                'name', t.name,
                'slug', t.slug,
                'color', t.color
            ) ORDER BY ct.sort_order)
            FROM cms_content_tags ct
            JOIN cms_tags t ON ct.tag_id = t.id
            WHERE ct.content_id = c.id
        ), '[]'::jsonb),

        -- Related content
        'relatedContent', COALESCE((
            SELECT jsonb_agg(jsonb_build_object(
                'relationType', r.relation_type,
                'sortOrder', r.sort_order,
                'content', jsonb_build_object(
                    'id', rc.id,
                    'contentType', rc.content_type,
                    'slug', rc.slug,
                    'title', rc.title,
                    'excerpt', rc.excerpt,
                    'featuredImage', CASE WHEN rfi.id IS NOT NULL THEN jsonb_build_object(
                        'url', rfi.cdn_url,
                        'width', rfi.width,
                        'height', rfi.height,
                        'blurhash', rfi.blurhash
                    ) END
                )
            ) ORDER BY r.sort_order)
            FROM cms_content_relations r
            JOIN cms_content rc ON r.target_id = rc.id
            LEFT JOIN cms_assets rfi ON rc.featured_image_id = rfi.id
            WHERE r.source_id = c.id
            AND rc.deleted_at IS NULL
            AND (p_include_drafts OR rc.status = 'published')
        ), '[]'::jsonb),

        -- Timestamps
        'publishedAt', c.published_at,
        'createdAt', c.created_at,
        'updatedAt', c.updated_at
    ) INTO v_result
    FROM cms_content c
    LEFT JOIN cms_assets fi ON c.featured_image_id = fi.id
    LEFT JOIN cms_assets oi ON c.og_image_id = oi.id
    LEFT JOIN cms_users u ON c.author_id = u.id
    WHERE c.id = v_content_id;

    RETURN v_result;
END;
$$;


--
-- Name: FUNCTION cms_get_content_full(p_content_type public.cms_content_type, p_slug character varying, p_locale character varying, p_include_drafts boolean); Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON FUNCTION public.cms_get_content_full(p_content_type public.cms_content_type, p_slug character varying, p_locale character varying, p_include_drafts boolean) IS 'Retrieve content with all relations in a single optimized query';


--
-- Name: cms_get_content_stats(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_get_content_stats() RETURNS jsonb
    LANGUAGE plpgsql STABLE
    AS $$
DECLARE
    v_result JSONB;
BEGIN
    SELECT jsonb_build_object(
        'content', jsonb_build_object(
            'total', COUNT(*) FILTER (WHERE deleted_at IS NULL),
            'published', COUNT(*) FILTER (WHERE status = 'published' AND deleted_at IS NULL),
            'draft', COUNT(*) FILTER (WHERE status = 'draft' AND deleted_at IS NULL),
            'inReview', COUNT(*) FILTER (WHERE status = 'in_review' AND deleted_at IS NULL),
            'scheduled', COUNT(*) FILTER (WHERE status = 'scheduled' AND deleted_at IS NULL),
            'archived', COUNT(*) FILTER (WHERE status = 'archived' AND deleted_at IS NULL),
            'byType', (
                SELECT jsonb_object_agg(content_type, cnt)
                FROM (
                    SELECT content_type, COUNT(*) as cnt
                    FROM cms_content
                    WHERE deleted_at IS NULL
                    GROUP BY content_type
                ) t
            )
        ),
        'assets', jsonb_build_object(
            'total', (SELECT COUNT(*) FROM cms_assets WHERE deleted_at IS NULL),
            'images', (SELECT COUNT(*) FROM cms_assets WHERE asset_type = 'image' AND deleted_at IS NULL),
            'videos', (SELECT COUNT(*) FROM cms_assets WHERE asset_type = 'video' AND deleted_at IS NULL),
            'documents', (SELECT COUNT(*) FROM cms_assets WHERE asset_type = 'document' AND deleted_at IS NULL),
            'totalSize', (SELECT COALESCE(SUM(file_size), 0) FROM cms_assets WHERE deleted_at IS NULL)
        ),
        'tags', jsonb_build_object(
            'total', (SELECT COUNT(*) FROM cms_tags)
        ),
        'menus', jsonb_build_object(
            'total', (SELECT COUNT(*) FROM cms_navigation_menus),
            'active', (SELECT COUNT(*) FROM cms_navigation_menus WHERE is_active = true)
        ),
        'redirects', jsonb_build_object(
            'total', (SELECT COUNT(*) FROM cms_redirects),
            'active', (SELECT COUNT(*) FROM cms_redirects WHERE is_active = true)
        ),
        'comments', jsonb_build_object(
            'total', (SELECT COUNT(*) FROM cms_comments WHERE deleted_at IS NULL),
            'unresolved', (SELECT COUNT(*) FROM cms_comments WHERE is_resolved = false AND deleted_at IS NULL)
        ),
        'scheduledJobs', jsonb_build_object(
            'pending', (SELECT COUNT(*) FROM cms_scheduled_jobs WHERE status = 'pending'),
            'running', (SELECT COUNT(*) FROM cms_scheduled_jobs WHERE status = 'running'),
            'failed', (SELECT COUNT(*) FROM cms_scheduled_jobs WHERE status = 'failed')
        )
    ) INTO v_result
    FROM cms_content;

    RETURN v_result;
END;
$$;


--
-- Name: FUNCTION cms_get_content_stats(); Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON FUNCTION public.cms_get_content_stats() IS 'Get comprehensive CMS statistics for dashboard';


--
-- Name: cms_get_current_user_role(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_get_current_user_role() RETURNS public.cms_user_role
    LANGUAGE plpgsql SECURITY DEFINER
    AS $$
DECLARE
    v_role cms_user_role;
BEGIN
    SELECT cu.role INTO v_role
    FROM cms_users cu
    WHERE cu.user_id = (current_setting('request.jwt.claims', true)::jsonb->>'sub')::BIGINT
      AND cu.is_active = true;

    RETURN COALESCE(v_role, 'viewer'::cms_user_role);
END;
$$;


--
-- Name: cms_get_datasource_entries(character varying, character varying); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_get_datasource_entries(p_datasource_slug character varying, p_dimension character varying DEFAULT 'default'::character varying) RETURNS TABLE(id uuid, name character varying, value character varying, dimension character varying, sort_order integer, metadata jsonb)
    LANGUAGE plpgsql STABLE
    AS $$
BEGIN
    RETURN QUERY
    SELECT
        e.id,
        e.name,
        e.value,
        e.dimension,
        e.sort_order,
        e.metadata
    FROM cms_datasource_entries e
    JOIN cms_datasources d ON e.datasource_id = d.id
    WHERE d.slug = p_datasource_slug
    AND d.deleted_at IS NULL
    AND e.dimension = p_dimension
    ORDER BY e.sort_order, e.name;
END;
$$;


--
-- Name: FUNCTION cms_get_datasource_entries(p_datasource_slug character varying, p_dimension character varying); Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON FUNCTION public.cms_get_datasource_entries(p_datasource_slug character varying, p_dimension character varying) IS 'Get all entries for a datasource by slug and dimension';


--
-- Name: cms_get_folder_path(uuid); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_get_folder_path(p_folder_id uuid) RETURNS text
    LANGUAGE plpgsql
    AS $$
DECLARE
    v_path TEXT := '';
    v_current_id UUID := p_folder_id;
    v_slug VARCHAR;
    v_parent_id UUID;
BEGIN
    WHILE v_current_id IS NOT NULL LOOP
        SELECT slug, parent_id INTO v_slug, v_parent_id
        FROM cms_asset_folders
        WHERE id = v_current_id;

        IF v_slug IS NOT NULL THEN
            v_path := '/' || v_slug || v_path;
        END IF;

        v_current_id := v_parent_id;
    END LOOP;

    RETURN COALESCE(NULLIF(v_path, ''), '/');
END;
$$;


--
-- Name: FUNCTION cms_get_folder_path(p_folder_id uuid); Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON FUNCTION public.cms_get_folder_path(p_folder_id uuid) IS 'Calculate full path for a folder';


--
-- Name: cms_get_next_entry_sort_order(uuid, character varying); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_get_next_entry_sort_order(p_datasource_id uuid, p_dimension character varying DEFAULT 'default'::character varying) RETURNS integer
    LANGUAGE plpgsql STABLE
    AS $$
DECLARE
    max_sort INTEGER;
BEGIN
    SELECT COALESCE(MAX(sort_order), -1) + 1 INTO max_sort
    FROM cms_datasource_entries
    WHERE datasource_id = p_datasource_id AND dimension = p_dimension;
    RETURN max_sort;
END;
$$;


--
-- Name: FUNCTION cms_get_next_entry_sort_order(p_datasource_id uuid, p_dimension character varying); Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON FUNCTION public.cms_get_next_entry_sort_order(p_datasource_id uuid, p_dimension character varying) IS 'Get next available sort order for new entry';


--
-- Name: cms_get_pending_releases(integer); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_get_pending_releases(p_limit integer DEFAULT 10) RETURNS TABLE(release_id uuid, name character varying, scheduled_at timestamp with time zone, stop_on_error boolean)
    LANGUAGE plpgsql
    AS $$
BEGIN
    RETURN QUERY
    SELECT
        r.id,
        r.name,
        r.scheduled_at,
        r.stop_on_error
    FROM cms_releases r
    WHERE r.status = 'scheduled'
      AND r.scheduled_at <= NOW()
    ORDER BY r.scheduled_at ASC
    LIMIT p_limit
    FOR UPDATE SKIP LOCKED;
END;
$$;


--
-- Name: cms_get_pending_schedules(integer); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_get_pending_schedules(p_limit integer DEFAULT 100) RETURNS TABLE(schedule_id uuid, content_id uuid, action public.cms_schedule_action, scheduled_at timestamp with time zone, staged_data jsonb)
    LANGUAGE plpgsql
    AS $$
BEGIN
    RETURN QUERY
    SELECT
        s.id,
        s.content_id,
        s.action,
        s.scheduled_at,
        s.staged_data
    FROM cms_schedules s
    WHERE s.status = 'pending'
      AND s.scheduled_at <= NOW()
    ORDER BY s.scheduled_at ASC
    LIMIT p_limit
    FOR UPDATE SKIP LOCKED;
END;
$$;


--
-- Name: cms_get_presets_by_block_type(character varying, public.cms_preset_category, boolean, uuid); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_get_presets_by_block_type(p_block_type character varying, p_category public.cms_preset_category DEFAULT NULL::public.cms_preset_category, p_include_global boolean DEFAULT true, p_user_id uuid DEFAULT NULL::uuid) RETURNS TABLE(id uuid, block_type character varying, name character varying, slug character varying, description text, preset_data jsonb, thumbnail_url text, category public.cms_preset_category, tags text[], is_default boolean, usage_count integer)
    LANGUAGE plpgsql STABLE
    AS $$
BEGIN
    RETURN QUERY
    SELECT
        p.id,
        p.block_type,
        p.name,
        p.slug,
        p.description,
        p.preset_data,
        p.thumbnail_url,
        p.category,
        p.tags,
        p.is_default,
        p.usage_count
    FROM cms_presets p
    WHERE p.block_type = p_block_type
    AND p.deleted_at IS NULL
    AND (p_category IS NULL OR p.category = p_category)
    AND (p_include_global AND p.is_global = true OR p.created_by = p_user_id)
    ORDER BY
        p.is_default DESC,
        p.category,
        p.usage_count DESC,
        p.name;
END;
$$;


--
-- Name: FUNCTION cms_get_presets_by_block_type(p_block_type character varying, p_category public.cms_preset_category, p_include_global boolean, p_user_id uuid); Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON FUNCTION public.cms_get_presets_by_block_type(p_block_type character varying, p_category public.cms_preset_category, p_include_global boolean, p_user_id uuid) IS 'Get all presets for a specific block type with optional category filtering';


--
-- Name: cms_get_recent_content(public.cms_content_type, integer, integer, character varying); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_get_recent_content(p_content_type public.cms_content_type, p_limit integer DEFAULT 10, p_offset integer DEFAULT 0, p_locale character varying DEFAULT 'en'::character varying) RETURNS jsonb
    LANGUAGE plpgsql STABLE
    AS $$
BEGIN
    RETURN (
        SELECT jsonb_build_object(
            'data', COALESCE(jsonb_agg(item), '[]'::jsonb),
            'meta', jsonb_build_object(
                'total', (
                    SELECT COUNT(*) FROM cms_content
                    WHERE content_type = p_content_type
                    AND status = 'published'
                    AND deleted_at IS NULL
                    AND locale = p_locale
                ),
                'limit', p_limit,
                'offset', p_offset
            )
        )
        FROM (
            SELECT jsonb_build_object(
                'id', c.id,
                'slug', c.slug,
                'title', c.title,
                'excerpt', c.excerpt,
                'featuredImage', CASE WHEN a.id IS NOT NULL THEN jsonb_build_object(
                    'url', a.cdn_url,
                    'width', a.width,
                    'height', a.height,
                    'blurhash', a.blurhash
                ) END,
                'author', CASE WHEN u.id IS NOT NULL THEN jsonb_build_object(
                    'displayName', u.display_name,
                    'avatarUrl', u.avatar_url
                ) END,
                'publishedAt', c.published_at
            ) AS item
            FROM cms_content c
            LEFT JOIN cms_assets a ON c.featured_image_id = a.id
            LEFT JOIN cms_users u ON c.author_id = u.id
            WHERE c.content_type = p_content_type
            AND c.status = 'published'
            AND c.deleted_at IS NULL
            AND c.locale = p_locale
            ORDER BY c.published_at DESC
            LIMIT p_limit
            OFFSET p_offset
        ) subq
    );
END;
$$;


--
-- Name: FUNCTION cms_get_recent_content(p_content_type public.cms_content_type, p_limit integer, p_offset integer, p_locale character varying); Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON FUNCTION public.cms_get_recent_content(p_content_type public.cms_content_type, p_limit integer, p_offset integer, p_locale character varying) IS 'Get recent published content for delivery API';


--
-- Name: cms_get_schedule_calendar(timestamp with time zone, timestamp with time zone, uuid); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_get_schedule_calendar(p_start_date timestamp with time zone, p_end_date timestamp with time zone, p_content_type_id uuid DEFAULT NULL::uuid) RETURNS TABLE(id uuid, content_id uuid, content_title character varying, content_type character varying, action public.cms_schedule_action, scheduled_at timestamp with time zone, timezone character varying, status public.cms_schedule_status, is_release boolean, release_id uuid, release_name character varying)
    LANGUAGE plpgsql
    AS $$
BEGIN
    -- Individual schedules
    RETURN QUERY
    SELECT
        s.id,
        s.content_id,
        c.title,
        ct.name AS content_type,
        s.action,
        s.scheduled_at,
        s.timezone,
        s.status,
        FALSE AS is_release,
        NULL::UUID AS release_id,
        NULL::VARCHAR AS release_name
    FROM cms_schedules s
    JOIN cms_content c ON s.content_id = c.id
    LEFT JOIN cms_content_types ct ON c.content_type_id = ct.id
    WHERE s.scheduled_at BETWEEN p_start_date AND p_end_date
      AND s.status IN ('pending', 'processing', 'completed')
      AND (p_content_type_id IS NULL OR c.content_type_id = p_content_type_id)

    UNION ALL

    -- Release items
    SELECT
        ri.id,
        ri.content_id,
        c.title,
        ct.name AS content_type,
        ri.action,
        r.scheduled_at,
        r.timezone,
        ri.status,
        TRUE AS is_release,
        r.id AS release_id,
        r.name AS release_name
    FROM cms_release_items ri
    JOIN cms_releases r ON ri.release_id = r.id
    JOIN cms_content c ON ri.content_id = c.id
    LEFT JOIN cms_content_types ct ON c.content_type_id = ct.id
    WHERE r.scheduled_at BETWEEN p_start_date AND p_end_date
      AND r.status IN ('scheduled', 'processing', 'completed')
      AND (p_content_type_id IS NULL OR c.content_type_id = p_content_type_id)

    ORDER BY scheduled_at ASC;
END;
$$;


--
-- Name: cms_increment_preset_usage(uuid); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_increment_preset_usage(p_preset_id uuid) RETURNS void
    LANGUAGE plpgsql
    AS $$
BEGIN
    UPDATE cms_presets
    SET usage_count = usage_count + 1
    WHERE id = p_preset_id AND deleted_at IS NULL;
END;
$$;


--
-- Name: FUNCTION cms_increment_preset_usage(p_preset_id uuid); Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON FUNCTION public.cms_increment_preset_usage(p_preset_id uuid) IS 'Increment usage count when a preset is applied to a block';


--
-- Name: cms_log_audit(character varying, character varying, uuid, uuid, character varying, public.cms_user_role, jsonb, jsonb, jsonb, inet, text); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_log_audit(p_action character varying, p_entity_type character varying, p_entity_id uuid, p_user_id uuid, p_user_email character varying DEFAULT NULL::character varying, p_user_role public.cms_user_role DEFAULT NULL::public.cms_user_role, p_old_values jsonb DEFAULT NULL::jsonb, p_new_values jsonb DEFAULT NULL::jsonb, p_metadata jsonb DEFAULT NULL::jsonb, p_ip_address inet DEFAULT NULL::inet, p_user_agent text DEFAULT NULL::text) RETURNS uuid
    LANGUAGE plpgsql
    AS $$
DECLARE
    v_new_id UUID;
BEGIN
    INSERT INTO cms_audit_log (
        action, entity_type, entity_id, user_id, user_email, user_role,
        old_values, new_values, metadata, ip_address, user_agent
    ) VALUES (
        p_action, p_entity_type, p_entity_id, p_user_id, p_user_email, p_user_role,
        p_old_values, p_new_values, p_metadata, p_ip_address, p_user_agent
    ) RETURNING id INTO v_new_id;

    RETURN v_new_id;
END;
$$;


--
-- Name: FUNCTION cms_log_audit(p_action character varying, p_entity_type character varying, p_entity_id uuid, p_user_id uuid, p_user_email character varying, p_user_role public.cms_user_role, p_old_values jsonb, p_new_values jsonb, p_metadata jsonb, p_ip_address inet, p_user_agent text); Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON FUNCTION public.cms_log_audit(p_action character varying, p_entity_type character varying, p_entity_id uuid, p_user_id uuid, p_user_email character varying, p_user_role public.cms_user_role, p_old_values jsonb, p_new_values jsonb, p_metadata jsonb, p_ip_address inet, p_user_agent text) IS 'Log an action to the partitioned audit table';


--
-- Name: cms_log_workflow_transition(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_log_workflow_transition() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    IF OLD.current_stage IS DISTINCT FROM NEW.current_stage THEN
        INSERT INTO cms_workflow_history (
            workflow_id,
            content_id,
            from_stage,
            to_stage,
            comment,
            transitioned_by
        ) VALUES (
            NEW.id,
            NEW.content_id,
            OLD.current_stage,
            NEW.current_stage,
            NEW.transition_comment,
            (SELECT updated_by FROM cms_content WHERE id = NEW.content_id)
        );
    END IF;
    RETURN NEW;
END;
$$;


--
-- Name: cms_notify_comment_mention(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_notify_comment_mention() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
DECLARE
    v_mentioned_user UUID;
BEGIN
    -- Create notifications for mentioned users
    IF NEW.mentioned_users IS NOT NULL THEN
        FOREACH v_mentioned_user IN ARRAY NEW.mentioned_users
        LOOP
            INSERT INTO cms_comment_notifications (comment_id, user_id, notification_type)
            VALUES (NEW.id, v_mentioned_user, 'mention')
            ON CONFLICT (comment_id, user_id, notification_type) DO NOTHING;
        END LOOP;
    END IF;

    -- Notify thread participants of reply
    IF NEW.parent_id IS NOT NULL THEN
        INSERT INTO cms_comment_notifications (comment_id, user_id, notification_type)
        SELECT NEW.id, c.created_by, 'reply'
        FROM cms_comments c
        WHERE c.thread_id = NEW.thread_id
        AND c.created_by IS NOT NULL
        AND c.created_by != NEW.created_by
        ON CONFLICT (comment_id, user_id, notification_type) DO NOTHING;
    END IF;

    RETURN NEW;
END;
$$;


--
-- Name: cms_presets_updated_at(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_presets_updated_at() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$;


--
-- Name: cms_process_schedule(uuid, boolean, text); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_process_schedule(p_schedule_id uuid, p_success boolean, p_error_message text DEFAULT NULL::text) RETURNS boolean
    LANGUAGE plpgsql
    AS $$
DECLARE
    v_schedule cms_schedules%ROWTYPE;
BEGIN
    -- Get the schedule
    SELECT * INTO v_schedule
    FROM cms_schedules
    WHERE id = p_schedule_id
    FOR UPDATE;

    IF NOT FOUND THEN
        RETURN FALSE;
    END IF;

    IF p_success THEN
        UPDATE cms_schedules
        SET status = 'completed',
            executed_at = NOW()
        WHERE id = p_schedule_id;
    ELSE
        IF v_schedule.retry_count < v_schedule.max_retries THEN
            UPDATE cms_schedules
            SET status = 'pending',
                retry_count = retry_count + 1,
                error_message = p_error_message
            WHERE id = p_schedule_id;
        ELSE
            UPDATE cms_schedules
            SET status = 'failed',
                executed_at = NOW(),
                error_message = p_error_message
            WHERE id = p_schedule_id;
        END IF;
    END IF;

    RETURN TRUE;
END;
$$;


--
-- Name: cms_process_scheduled_jobs(integer); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_process_scheduled_jobs(p_batch_size integer DEFAULT 100) RETURNS integer
    LANGUAGE plpgsql
    AS $$
DECLARE
    v_job RECORD;
    v_count INTEGER := 0;
    v_now TIMESTAMPTZ := NOW();
BEGIN
    FOR v_job IN
        SELECT * FROM cms_scheduled_jobs
        WHERE status = 'pending'
        AND scheduled_at <= v_now
        AND (locked_at IS NULL OR locked_at < v_now - INTERVAL '5 minutes')
        ORDER BY scheduled_at
        LIMIT p_batch_size
        FOR UPDATE SKIP LOCKED
    LOOP
        -- Lock the job
        UPDATE cms_scheduled_jobs
        SET
            status = 'running',
            locked_by = pg_backend_pid()::text,
            locked_at = v_now,
            started_at = v_now,
            attempts = attempts + 1
        WHERE id = v_job.id;

        BEGIN
            -- Process based on job type
            CASE v_job.job_type
                WHEN 'publish' THEN
                    PERFORM cms_publish_content(v_job.content_id, v_job.created_by);

                WHEN 'unpublish' THEN
                    PERFORM cms_unpublish_content(v_job.content_id, v_job.created_by);

                WHEN 'archive' THEN
                    UPDATE cms_content
                    SET status = 'archived', updated_by = v_job.created_by
                    WHERE id = v_job.content_id;

                ELSE
                    RAISE EXCEPTION 'Unknown job type: %', v_job.job_type;
            END CASE;

            -- Mark as completed
            UPDATE cms_scheduled_jobs
            SET
                status = 'completed',
                completed_at = NOW(),
                result = jsonb_build_object('success', true),
                locked_by = NULL,
                locked_at = NULL
            WHERE id = v_job.id;

            v_count := v_count + 1;

        EXCEPTION WHEN OTHERS THEN
            -- Mark as failed or retry
            UPDATE cms_scheduled_jobs
            SET
                status = CASE
                    WHEN attempts >= max_attempts THEN 'failed'
                    ELSE 'pending'
                END,
                error_message = SQLERRM,
                locked_by = NULL,
                locked_at = NULL
            WHERE id = v_job.id;
        END;
    END LOOP;

    RETURN v_count;
END;
$$;


--
-- Name: FUNCTION cms_process_scheduled_jobs(p_batch_size integer); Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON FUNCTION public.cms_process_scheduled_jobs(p_batch_size integer) IS 'Process pending scheduled jobs with locking and retry logic';


--
-- Name: cms_process_sync_item(uuid, uuid); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_process_sync_item(p_sync_id uuid, p_user_id uuid) RETURNS jsonb
    LANGUAGE plpgsql
    AS $$
DECLARE
    v_sync RECORD;
    v_content RECORD;
    v_result JSONB;
BEGIN
    -- Get sync item
    SELECT * INTO v_sync
    FROM cms_offline_sync_queue
    WHERE id = p_sync_id AND user_id = p_user_id AND status = 'pending'
    FOR UPDATE SKIP LOCKED;

    IF v_sync IS NULL THEN
        RETURN jsonb_build_object('success', false, 'error', 'Sync item not found or already processed');
    END IF;

    -- Get current content version
    SELECT id, version INTO v_content
    FROM cms_content
    WHERE id = v_sync.content_id;

    IF v_content IS NULL THEN
        UPDATE cms_offline_sync_queue
        SET status = 'failed', error_message = 'Content not found'
        WHERE id = p_sync_id;
        RETURN jsonb_build_object('success', false, 'error', 'Content not found');
    END IF;

    -- Check for conflict
    IF v_content.version > v_sync.local_version THEN
        UPDATE cms_offline_sync_queue
        SET status = 'conflict',
            server_version = v_content.version,
            conflict_data = (SELECT row_to_json(c) FROM cms_content c WHERE c.id = v_sync.content_id)::JSONB
        WHERE id = p_sync_id;
        RETURN jsonb_build_object('success', false, 'conflict', true, 'server_version', v_content.version);
    END IF;

    -- Apply the sync (update content)
    UPDATE cms_content
    SET title = COALESCE(v_sync.payload->>'title', title),
        content = COALESCE(v_sync.payload->>'content', content),
        content_blocks = COALESCE(v_sync.payload->'content_blocks', content_blocks),
        version = version + 1,
        updated_at = NOW(),
        updated_by = p_user_id
    WHERE id = v_sync.content_id;

    -- Mark sync as complete
    UPDATE cms_offline_sync_queue
    SET status = 'synced',
        synced_at = NOW(),
        server_version = v_content.version + 1
    WHERE id = p_sync_id;

    RETURN jsonb_build_object('success', true, 'new_version', v_content.version + 1);
END;
$$;


--
-- Name: cms_publish_content(uuid, uuid); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_publish_content(p_content_id uuid, p_user_id uuid) RETURNS jsonb
    LANGUAGE plpgsql
    AS $$
DECLARE
    v_content RECORD;
    v_old_status cms_content_status;
    v_now TIMESTAMPTZ := NOW();
BEGIN
    -- Lock the content row for update
    SELECT * INTO v_content
    FROM cms_content
    WHERE id = p_content_id
    AND deleted_at IS NULL
    FOR UPDATE;

    IF v_content IS NULL THEN
        RETURN jsonb_build_object(
            'success', false,
            'error', 'Content not found or deleted'
        );
    END IF;

    v_old_status := v_content.status;

    -- Validate transition
    IF v_old_status NOT IN ('draft', 'in_review', 'approved', 'scheduled') THEN
        RETURN jsonb_build_object(
            'success', false,
            'error', format('Cannot publish content with status: %s', v_old_status)
        );
    END IF;

    -- Update to published
    UPDATE cms_content
    SET
        status = 'published',
        published_at = COALESCE(published_at, v_now),
        updated_at = v_now,
        updated_by = p_user_id,
        version = version + 1
    WHERE id = p_content_id;

    -- Log workflow transition
    INSERT INTO cms_workflow_log (
        content_id, from_status, to_status, transitioned_by, created_at
    ) VALUES (
        p_content_id, v_old_status, 'published', p_user_id, v_now
    );

    -- Create revision
    PERFORM cms_create_revision(
        p_content_id,
        (SELECT row_to_json(c)::jsonb FROM cms_content c WHERE c.id = p_content_id),
        p_user_id,
        'Published',
        ARRAY['status']
    );

    -- Log to audit
    INSERT INTO cms_audit_log (
        action, entity_type, entity_id, user_id,
        old_values, new_values, created_at
    ) VALUES (
        'content.published', 'cms_content', p_content_id, p_user_id,
        jsonb_build_object('status', v_old_status),
        jsonb_build_object('status', 'published', 'publishedAt', v_now),
        v_now
    );

    RETURN jsonb_build_object(
        'success', true,
        'contentId', p_content_id,
        'publishedAt', v_now,
        'previousStatus', v_old_status
    );
END;
$$;


--
-- Name: FUNCTION cms_publish_content(p_content_id uuid, p_user_id uuid); Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON FUNCTION public.cms_publish_content(p_content_id uuid, p_user_id uuid) IS 'Atomically publish content with workflow logging and audit trail';


--
-- Name: cms_queue_webhook_delivery(text, uuid, jsonb); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_queue_webhook_delivery(p_event_type text, p_content_id uuid DEFAULT NULL::uuid, p_payload jsonb DEFAULT '{}'::jsonb) RETURNS integer
    LANGUAGE plpgsql
    AS $$
DECLARE
    v_webhook RECORD;
    v_count INTEGER := 0;
BEGIN
    FOR v_webhook IN
        SELECT * FROM cms_webhooks
        WHERE is_active = true
        AND p_event_type = ANY(events)
        AND (content_types IS NULL OR (
            SELECT content_type FROM cms_content WHERE id = p_content_id
        ) = ANY(content_types))
    LOOP
        INSERT INTO cms_webhook_deliveries (
            webhook_id, event_type, content_id, payload, status
        ) VALUES (
            v_webhook.id, p_event_type, p_content_id, p_payload, 'pending'
        );
        v_count := v_count + 1;
    END LOOP;

    RETURN v_count;
END;
$$;


--
-- Name: FUNCTION cms_queue_webhook_delivery(p_event_type text, p_content_id uuid, p_payload jsonb); Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON FUNCTION public.cms_queue_webhook_delivery(p_event_type text, p_content_id uuid, p_payload jsonb) IS 'Queue webhook deliveries for matching webhooks';


--
-- Name: cms_reorder_datasource_entries(uuid[], integer[]); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_reorder_datasource_entries(p_entry_ids uuid[], p_sort_orders integer[]) RETURNS void
    LANGUAGE plpgsql
    AS $$
DECLARE
    i INTEGER;
BEGIN
    FOR i IN 1..array_length(p_entry_ids, 1) LOOP
        UPDATE cms_datasource_entries
        SET sort_order = p_sort_orders[i]
        WHERE id = p_entry_ids[i];
    END LOOP;
END;
$$;


--
-- Name: FUNCTION cms_reorder_datasource_entries(p_entry_ids uuid[], p_sort_orders integer[]); Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON FUNCTION public.cms_reorder_datasource_entries(p_entry_ids uuid[], p_sort_orders integer[]) IS 'Bulk update sort orders for drag-to-reorder';


--
-- Name: cms_schedules_update_timestamp(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_schedules_update_timestamp() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$;


--
-- Name: cms_search_content(text, public.cms_content_type[], uuid[], text, integer, integer, boolean); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_search_content(p_query text, p_content_types public.cms_content_type[] DEFAULT NULL::public.cms_content_type[], p_tag_ids uuid[] DEFAULT NULL::uuid[], p_locale text DEFAULT 'en'::text, p_limit integer DEFAULT 20, p_offset integer DEFAULT 0, p_published_only boolean DEFAULT true) RETURNS TABLE(id uuid, content_type public.cms_content_type, slug character varying, title character varying, excerpt text, featured_image_id uuid, author_id uuid, published_at timestamp with time zone, rank real, headline text)
    LANGUAGE plpgsql STABLE SECURITY DEFINER
    AS $$
DECLARE
    v_tsquery tsquery;
BEGIN
    -- Parse the search query
    v_tsquery := websearch_to_tsquery('english', p_query);

    RETURN QUERY
    WITH ranked_results AS (
        SELECT
            c.id,
            c.content_type,
            c.slug,
            c.title,
            c.excerpt,
            c.featured_image_id,
            c.author_id,
            c.published_at,
            -- Rank calculation with weighted fields
            ts_rank_cd(c.search_vector, v_tsquery, 32) +
            -- Boost for title similarity
            COALESCE(similarity(c.title, p_query) * 0.5, 0) +
            -- Boost for recent content (within 30 days)
            CASE WHEN c.published_at > NOW() - INTERVAL '30 days' THEN 0.1 ELSE 0 END
            AS rank,
            -- Generate headline with highlighted matches
            ts_headline('english',
                COALESCE(c.excerpt, LEFT(c.content, 300)),
                v_tsquery,
                'MaxFragments=2, MinWords=15, MaxWords=35, StartSel=<mark>, StopSel=</mark>'
            ) AS headline
        FROM cms_content c
        WHERE
            c.deleted_at IS NULL
            AND c.locale = p_locale
            AND (NOT p_published_only OR c.status = 'published')
            AND (p_content_types IS NULL OR c.content_type = ANY(p_content_types))
            AND (
                c.search_vector @@ v_tsquery
                OR c.title ILIKE '%' || p_query || '%'
                OR similarity(c.title, p_query) > 0.3
            )
    )
    SELECT r.*
    FROM ranked_results r
    WHERE
        p_tag_ids IS NULL
        OR EXISTS (
            SELECT 1 FROM cms_content_tags ct
            WHERE ct.content_id = r.id AND ct.tag_id = ANY(p_tag_ids)
        )
    ORDER BY r.rank DESC, r.published_at DESC NULLS LAST
    LIMIT p_limit
    OFFSET p_offset;
END;
$$;


--
-- Name: FUNCTION cms_search_content(p_query text, p_content_types public.cms_content_type[], p_tag_ids uuid[], p_locale text, p_limit integer, p_offset integer, p_published_only boolean); Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON FUNCTION public.cms_search_content(p_query text, p_content_types public.cms_content_type[], p_tag_ids uuid[], p_locale text, p_limit integer, p_offset integer, p_published_only boolean) IS 'Full-text search with ranking, fuzzy matching, and tag filtering';


--
-- Name: cms_set_updated_at(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_set_updated_at() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$;


--
-- Name: FUNCTION cms_set_updated_at(); Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON FUNCTION public.cms_set_updated_at() IS 'Standard updated_at trigger function';


--
-- Name: cms_unpublish_content(uuid, uuid, public.cms_content_status); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_unpublish_content(p_content_id uuid, p_user_id uuid, p_target_status public.cms_content_status DEFAULT 'draft'::public.cms_content_status) RETURNS jsonb
    LANGUAGE plpgsql
    AS $$
DECLARE
    v_content RECORD;
    v_old_status cms_content_status;
    v_now TIMESTAMPTZ := NOW();
BEGIN
    -- Lock the content row
    SELECT * INTO v_content
    FROM cms_content
    WHERE id = p_content_id
    AND deleted_at IS NULL
    FOR UPDATE;

    IF v_content IS NULL THEN
        RETURN jsonb_build_object(
            'success', false,
            'error', 'Content not found or deleted'
        );
    END IF;

    v_old_status := v_content.status;

    IF v_old_status != 'published' THEN
        RETURN jsonb_build_object(
            'success', false,
            'error', 'Content is not currently published'
        );
    END IF;

    -- Update status
    UPDATE cms_content
    SET
        status = p_target_status,
        updated_at = v_now,
        updated_by = p_user_id,
        version = version + 1
    WHERE id = p_content_id;

    -- Log workflow transition
    INSERT INTO cms_workflow_log (
        content_id, from_status, to_status, transitioned_by, created_at
    ) VALUES (
        p_content_id, v_old_status, p_target_status, p_user_id, v_now
    );

    -- Log to audit
    INSERT INTO cms_audit_log (
        action, entity_type, entity_id, user_id,
        old_values, new_values, created_at
    ) VALUES (
        'content.unpublished', 'cms_content', p_content_id, p_user_id,
        jsonb_build_object('status', v_old_status),
        jsonb_build_object('status', p_target_status),
        v_now
    );

    RETURN jsonb_build_object(
        'success', true,
        'contentId', p_content_id,
        'newStatus', p_target_status
    );
END;
$$;


--
-- Name: FUNCTION cms_unpublish_content(p_content_id uuid, p_user_id uuid, p_target_status public.cms_content_status); Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON FUNCTION public.cms_unpublish_content(p_content_id uuid, p_user_id uuid, p_target_status public.cms_content_status) IS 'Atomically unpublish content with workflow logging';


--
-- Name: cms_update_asset_usage(uuid, integer); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_update_asset_usage(p_asset_id uuid, p_increment integer) RETURNS void
    LANGUAGE plpgsql
    AS $$
BEGIN
    UPDATE cms_assets
    SET
        usage_count = GREATEST(usage_count + p_increment, 0),
        last_used_at = CASE WHEN p_increment > 0 THEN NOW() ELSE last_used_at END
    WHERE id = p_asset_id;
END;
$$;


--
-- Name: FUNCTION cms_update_asset_usage(p_asset_id uuid, p_increment integer); Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON FUNCTION public.cms_update_asset_usage(p_asset_id uuid, p_increment integer) IS 'Update asset usage statistics';


--
-- Name: cms_update_folder_path(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_update_folder_path() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.path := cms_get_folder_path(NEW.parent_id) || '/' || NEW.slug;
    NEW.depth := (SELECT COALESCE(depth, -1) + 1 FROM cms_asset_folders WHERE id = NEW.parent_id);
    IF NEW.depth IS NULL THEN NEW.depth := 0; END IF;
    RETURN NEW;
END;
$$;


--
-- Name: FUNCTION cms_update_folder_path(); Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON FUNCTION public.cms_update_folder_path() IS 'Trigger to maintain folder paths and depths';


--
-- Name: cms_update_release_item_count(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_update_release_item_count() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE cms_releases
        SET total_items = total_items + 1,
            updated_at = NOW()
        WHERE id = NEW.release_id;
        RETURN NEW;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE cms_releases
        SET total_items = GREATEST(0, total_items - 1),
            updated_at = NOW()
        WHERE id = OLD.release_id;
        RETURN OLD;
    END IF;
    RETURN NULL;
END;
$$;


--
-- Name: cms_update_release_progress(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_update_release_progress() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    IF NEW.status = 'completed' AND (OLD.status IS NULL OR OLD.status != 'completed') THEN
        UPDATE cms_releases
        SET completed_items = completed_items + 1,
            updated_at = NOW()
        WHERE id = NEW.release_id;
    ELSIF NEW.status = 'failed' AND (OLD.status IS NULL OR OLD.status != 'failed') THEN
        UPDATE cms_releases
        SET failed_items = failed_items + 1,
            updated_at = NOW()
        WHERE id = NEW.release_id;
    END IF;
    RETURN NEW;
END;
$$;


--
-- Name: cms_update_search_vector(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_update_search_vector() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.search_vector :=
        setweight(to_tsvector('english', COALESCE(NEW.title, '')), 'A') ||
        setweight(to_tsvector('english', COALESCE(NEW.excerpt, '')), 'B') ||
        setweight(to_tsvector('english', COALESCE(NEW.content, '')), 'C') ||
        setweight(to_tsvector('english', COALESCE(NEW.meta_description, '')), 'D');
    RETURN NEW;
END;
$$;


--
-- Name: cms_update_tag_usage(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_update_tag_usage() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE cms_tags SET usage_count = usage_count + 1 WHERE id = NEW.tag_id;
        RETURN NEW;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE cms_tags SET usage_count = GREATEST(usage_count - 1, 0) WHERE id = OLD.tag_id;
        RETURN OLD;
    END IF;
    RETURN NULL;
END;
$$;


--
-- Name: FUNCTION cms_update_tag_usage(); Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON FUNCTION public.cms_update_tag_usage() IS 'Trigger function to maintain tag usage counts';


--
-- Name: cms_update_timestamp(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.cms_update_timestamp() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$;


--
-- Name: create_content_version(character varying, bigint, jsonb, bigint, character varying, text[]); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.create_content_version(p_content_type character varying, p_content_id bigint, p_data jsonb, p_created_by bigint, p_change_summary character varying DEFAULT NULL::character varying, p_changed_fields text[] DEFAULT NULL::text[]) RETURNS bigint
    LANGUAGE plpgsql
    AS $$
DECLARE
    v_version_number INTEGER;
    v_new_id BIGINT;
BEGIN
    -- Get next version number
    SELECT COALESCE(MAX(version_number), 0) + 1 INTO v_version_number
    FROM content_versions
    WHERE content_type = p_content_type AND content_id = p_content_id;

    -- Mark previous version as not current
    UPDATE content_versions
    SET is_current = false
    WHERE content_type = p_content_type AND content_id = p_content_id;

    -- Insert new version
    INSERT INTO content_versions (
        content_type, content_id, version_number, is_current,
        data, change_summary, changed_fields, created_by
    ) VALUES (
        p_content_type, p_content_id, v_version_number, true,
        p_data, p_change_summary, p_changed_fields, p_created_by
    ) RETURNING id INTO v_new_id;

    RETURN v_new_id;
END;
$$;


--
-- Name: create_resource_version(bigint, character varying, bigint); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.create_resource_version(p_resource_id bigint, p_new_file_url character varying, p_new_file_size bigint DEFAULT NULL::bigint) RETURNS bigint
    LANGUAGE plpgsql
    AS $$
DECLARE
    v_old_resource room_resources%ROWTYPE;
    v_new_id BIGINT;
BEGIN
    -- Get the current resource
    SELECT * INTO v_old_resource FROM room_resources WHERE id = p_resource_id;

    IF NOT FOUND THEN
        RAISE EXCEPTION 'Resource not found: %', p_resource_id;
    END IF;

    -- Mark old version as not latest
    UPDATE room_resources SET is_latest_version = false WHERE id = p_resource_id;

    -- Create new version
    INSERT INTO room_resources (
        title, slug, description, trading_room_id, resource_type, content_type, section,
        file_url, file_size, mime_type, thumbnail_url, video_platform, bunny_video_guid, bunny_library_id,
        duration, width, height, is_featured, is_pinned, is_published, access_level,
        category, tags, difficulty_level, trader_id, course_id, lesson_id, course_order,
        version, previous_version_id, is_latest_version, resource_date, created_by
    )
    VALUES (
        v_old_resource.title, v_old_resource.slug || '-v' || (v_old_resource.version + 1),
        v_old_resource.description, v_old_resource.trading_room_id, v_old_resource.resource_type,
        v_old_resource.content_type, v_old_resource.section,
        p_new_file_url, COALESCE(p_new_file_size, v_old_resource.file_size), v_old_resource.mime_type,
        v_old_resource.thumbnail_url, v_old_resource.video_platform, v_old_resource.bunny_video_guid, v_old_resource.bunny_library_id,
        v_old_resource.duration, v_old_resource.width, v_old_resource.height,
        v_old_resource.is_featured, v_old_resource.is_pinned, v_old_resource.is_published, v_old_resource.access_level,
        v_old_resource.category, v_old_resource.tags, v_old_resource.difficulty_level,
        v_old_resource.trader_id, v_old_resource.course_id, v_old_resource.lesson_id, v_old_resource.course_order,
        v_old_resource.version + 1, p_resource_id, true, CURRENT_DATE, v_old_resource.created_by
    )
    RETURNING id INTO v_new_id;

    RETURN v_new_id;
END;
$$;


--
-- Name: generate_secure_download_token(bigint, integer); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.generate_secure_download_token(p_resource_id bigint, p_expires_in_hours integer DEFAULT 24) RETURNS character varying
    LANGUAGE plpgsql
    AS $$
DECLARE
    v_token VARCHAR(64);
BEGIN
    v_token := encode(gen_random_bytes(32), 'hex');

    UPDATE room_resources
    SET secure_token = v_token,
        secure_token_expires = NOW() + (p_expires_in_hours || ' hours')::INTERVAL
    WHERE id = p_resource_id;

    RETURN v_token;
END;
$$;


--
-- Name: get_equity_curve(text, date, date); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.get_equity_curve(p_room_slug text, p_from_date date DEFAULT NULL::date, p_to_date date DEFAULT NULL::date) RETURNS TABLE(trade_date date, cumulative_pnl numeric, trade_id bigint)
    LANGUAGE plpgsql STABLE
    AS $$
BEGIN
    RETURN QUERY
    SELECT
        t.exit_date::DATE as trade_date,
        SUM(t.pnl) OVER (ORDER BY t.exit_date) as cumulative_pnl,
        t.id as trade_id
    FROM room_trades t
    WHERE t.room_slug = p_room_slug
      AND t.status = 'closed'
      AND (p_from_date IS NULL OR t.exit_date >= p_from_date)
      AND (p_to_date IS NULL OR t.exit_date <= p_to_date)
    ORDER BY t.exit_date;
END;
$$;


--
-- Name: FUNCTION get_equity_curve(p_room_slug text, p_from_date date, p_to_date date); Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON FUNCTION public.get_equity_curve(p_room_slug text, p_from_date date, p_to_date date) IS 'Returns equity curve data with cumulative P&L for charting';


--
-- Name: get_monthly_returns(text, integer); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.get_monthly_returns(p_room_slug text, p_months integer DEFAULT 12) RETURNS TABLE(month text, pnl numeric, trades bigint, win_rate numeric)
    LANGUAGE plpgsql STABLE
    AS $$
BEGIN
    RETURN QUERY
    SELECT
        to_char(t.exit_date, 'YYYY-MM') as month,
        COALESCE(SUM(t.pnl), 0) as pnl,
        COUNT(*)::BIGINT as trades,
        ROUND(
            COUNT(*) FILTER (WHERE t.result = 'WIN')::NUMERIC /
            NULLIF(COUNT(*)::NUMERIC, 0) * 100, 2
        ) as win_rate
    FROM room_trades t
    WHERE t.room_slug = p_room_slug
      AND t.status = 'closed'
      AND t.exit_date >= (CURRENT_DATE - (p_months || ' months')::INTERVAL)
    GROUP BY to_char(t.exit_date, 'YYYY-MM')
    ORDER BY month DESC;
END;
$$;


--
-- Name: FUNCTION get_monthly_returns(p_room_slug text, p_months integer); Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON FUNCTION public.get_monthly_returns(p_room_slug text, p_months integer) IS 'Returns monthly P&L breakdown for historical performance analysis';


--
-- Name: get_popup_analytics(integer, integer); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.get_popup_analytics(p_popup_id integer, p_days integer DEFAULT 30) RETURNS TABLE(total_views bigint, total_conversions bigint, conversion_rate double precision, daily_avg_views double precision, daily_avg_conversions double precision, top_device character varying, top_page text)
    LANGUAGE plpgsql
    AS $$
BEGIN
    RETURN QUERY
    SELECT
        COUNT(*) FILTER (WHERE event_type = 'view') as total_views,
        COUNT(*) FILTER (WHERE event_type = 'conversion') as total_conversions,
        CASE
            WHEN COUNT(*) FILTER (WHERE event_type = 'view') > 0
            THEN (COUNT(*) FILTER (WHERE event_type = 'conversion')::float /
                  COUNT(*) FILTER (WHERE event_type = 'view')::float) * 100
            ELSE 0
        END as conversion_rate,
        COUNT(*) FILTER (WHERE event_type = 'view')::float / p_days as daily_avg_views,
        COUNT(*) FILTER (WHERE event_type = 'conversion')::float / p_days as daily_avg_conversions,
        (SELECT device_type FROM popup_events
         WHERE popup_id = p_popup_id AND device_type IS NOT NULL
         GROUP BY device_type ORDER BY COUNT(*) DESC LIMIT 1) as top_device,
        (SELECT page_url FROM popup_events
         WHERE popup_id = p_popup_id AND page_url IS NOT NULL
         GROUP BY page_url ORDER BY COUNT(*) DESC LIMIT 1) as top_page
    FROM popup_events
    WHERE popup_id = p_popup_id
      AND created_at >= NOW() - (p_days || ' days')::interval;
END;
$$;


--
-- Name: get_room_subscription_plans(text); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.get_room_subscription_plans(p_room_slug text) RETURNS TABLE(plan_id bigint, plan_slug character varying, display_name character varying, price numeric, billing_cycle character varying, interval_count integer, savings_percent integer, is_popular boolean, features jsonb, stripe_price_id character varying)
    LANGUAGE plpgsql
    AS $$
BEGIN
    RETURN QUERY
    SELECT 
        mp.id,
        mp.slug,
        mp.display_name,
        mp.price,
        mp.billing_cycle,
        mp.interval_count,
        mp.savings_percent,
        mp.is_popular,
        mp.features,
        mp.stripe_price_id
    FROM membership_plans mp
    JOIN trading_rooms tr ON tr.id = mp.room_id
    WHERE tr.slug = p_room_slug
      AND mp.is_active = true
    ORDER BY mp.sort_order;
END;
$$;


--
-- Name: get_room_ticker_suggestions(text, text, integer); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.get_room_ticker_suggestions(p_room_slug text, p_prefix text, p_limit integer DEFAULT 10) RETURNS TABLE(ticker text, occurrence_count bigint)
    LANGUAGE plpgsql STABLE
    AS $$
BEGIN
    RETURN QUERY
    WITH all_tickers AS (
        SELECT UPPER(a.ticker) as ticker FROM room_alerts a
        WHERE a.room_slug = p_room_slug AND a.deleted_at IS NULL
        UNION ALL
        SELECT UPPER(t.ticker) FROM room_trades t
        WHERE t.room_slug = p_room_slug AND t.deleted_at IS NULL
        UNION ALL
        SELECT UPPER(p.ticker) FROM room_trade_plans p
        WHERE p.room_slug = p_room_slug AND p.deleted_at IS NULL
    )
    SELECT at.ticker, COUNT(*) as occurrence_count
    FROM all_tickers at
    WHERE at.ticker LIKE UPPER(p_prefix) || '%'
    GROUP BY at.ticker
    ORDER BY occurrence_count DESC, at.ticker ASC
    LIMIT p_limit;
END;
$$;


--
-- Name: FUNCTION get_room_ticker_suggestions(p_room_slug text, p_prefix text, p_limit integer); Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON FUNCTION public.get_room_ticker_suggestions(p_room_slug text, p_prefix text, p_limit integer) IS 'Autocomplete suggestions for ticker symbols in a room';


--
-- Name: get_ticker_performance(text); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.get_ticker_performance(p_room_slug text) RETURNS TABLE(ticker character varying, total_trades bigint, wins bigint, losses bigint, win_rate numeric, total_pnl numeric, avg_pnl numeric)
    LANGUAGE plpgsql STABLE
    AS $$
BEGIN
    RETURN QUERY
    SELECT
        t.ticker,
        COUNT(*)::BIGINT as total_trades,
        COUNT(*) FILTER (WHERE t.result = 'WIN')::BIGINT as wins,
        COUNT(*) FILTER (WHERE t.result = 'LOSS')::BIGINT as losses,
        ROUND(
            COUNT(*) FILTER (WHERE t.result = 'WIN')::NUMERIC /
            NULLIF(COUNT(*)::NUMERIC, 0) * 100, 2
        ) as win_rate,
        COALESCE(SUM(t.pnl), 0) as total_pnl,
        ROUND(COALESCE(AVG(t.pnl), 0), 2) as avg_pnl
    FROM room_trades t
    WHERE t.room_slug = p_room_slug AND t.status = 'closed'
    GROUP BY t.ticker
    ORDER BY total_pnl DESC;
END;
$$;


--
-- Name: FUNCTION get_ticker_performance(p_room_slug text); Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON FUNCTION public.get_ticker_performance(p_room_slug text) IS 'Returns aggregated performance metrics by ticker symbol';


--
-- Name: get_upcoming_schedule_events(character varying, integer); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.get_upcoming_schedule_events(p_plan_slug character varying, p_days_ahead integer DEFAULT 7) RETURNS TABLE(id bigint, title character varying, trader_name character varying, event_date date, start_time time without time zone, end_time time without time zone, timezone character varying, is_cancelled boolean, exception_reason text)
    LANGUAGE plpgsql
    AS $$
BEGIN
    RETURN QUERY
    WITH date_series AS (
        SELECT generate_series(
            CURRENT_DATE,
            CURRENT_DATE + p_days_ahead,
            '1 day'::interval
        )::date AS event_date
    ),
    schedule_events AS (
        SELECT 
            s.id,
            s.title,
            s.trader_name,
            ds.event_date,
            COALESCE(e.new_start_time, s.start_time) AS start_time,
            COALESCE(e.new_end_time, s.end_time) AS end_time,
            s.timezone,
            CASE WHEN e.exception_type = 'cancelled' THEN true ELSE false END AS is_cancelled,
            e.reason AS exception_reason
        FROM trading_room_schedules s
        JOIN membership_plans p ON s.plan_id = p.id
        CROSS JOIN date_series ds
        LEFT JOIN schedule_exceptions e ON e.schedule_id = s.id AND e.exception_date = ds.event_date
        WHERE p.slug = p_plan_slug
          AND s.is_active = true
          AND EXTRACT(DOW FROM ds.event_date) = s.day_of_week
          AND (s.effective_from IS NULL OR ds.event_date >= s.effective_from)
          AND (s.effective_until IS NULL OR ds.event_date <= s.effective_until)
    )
    SELECT * FROM schedule_events
    WHERE NOT is_cancelled
    ORDER BY event_date, start_time;
END;
$$;


--
-- Name: get_video_completion_stats(bigint); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.get_video_completion_stats(p_video_id bigint) RETURNS TABLE(total_views bigint, unique_viewers bigint, avg_completion_percent numeric, completion_rate numeric, total_watch_time_seconds bigint)
    LANGUAGE plpgsql STABLE
    AS $$
BEGIN
    RETURN QUERY
    SELECT
        COUNT(*)::BIGINT as total_views,
        COUNT(DISTINCT user_id)::BIGINT as unique_viewers,
        ROUND(AVG(completion_percent)::NUMERIC, 2) as avg_completion_percent,
        ROUND((COUNT(*) FILTER (WHERE completed = TRUE)::NUMERIC / NULLIF(COUNT(*)::NUMERIC, 0)) * 100, 2) as completion_rate,
        SUM(current_time_seconds)::BIGINT as total_watch_time_seconds
    FROM video_watch_progress
    WHERE video_id = p_video_id;
END;
$$;


--
-- Name: FUNCTION get_video_completion_stats(p_video_id bigint); Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON FUNCTION public.get_video_completion_stats(p_video_id bigint) IS 'Returns completion statistics for a video';


--
-- Name: log_audit_event(character varying, character varying, bigint, bigint, character varying, jsonb, jsonb, jsonb, inet, text); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.log_audit_event(p_action character varying, p_entity_type character varying, p_entity_id bigint, p_user_id bigint, p_user_email character varying, p_old_values jsonb DEFAULT NULL::jsonb, p_new_values jsonb DEFAULT NULL::jsonb, p_metadata jsonb DEFAULT NULL::jsonb, p_ip_address inet DEFAULT NULL::inet, p_user_agent text DEFAULT NULL::text) RETURNS bigint
    LANGUAGE plpgsql
    AS $$
DECLARE
    v_new_id BIGINT;
BEGIN
    INSERT INTO audit_logs (
        action, entity_type, entity_id, user_id, user_email,
        old_values, new_values, metadata, ip_address, user_agent
    ) VALUES (
        p_action, p_entity_type, p_entity_id, p_user_id, p_user_email,
        p_old_values, p_new_values, p_metadata, p_ip_address, p_user_agent
    ) RETURNING id INTO v_new_id;

    RETURN v_new_id;
END;
$$;


--
-- Name: refresh_resource_analytics(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.refresh_resource_analytics() RETURNS void
    LANGUAGE plpgsql
    AS $$
BEGIN
    REFRESH MATERIALIZED VIEW CONCURRENTLY mv_resource_analytics;
END;
$$;


--
-- Name: restore_deleted_user(bigint); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.restore_deleted_user(p_user_id bigint) RETURNS boolean
    LANGUAGE plpgsql
    AS $$
BEGIN
    UPDATE users
    SET
        deleted_at = NULL,
        deleted_by = NULL,
        deletion_reason = NULL,
        updated_at = NOW()
    WHERE id = p_user_id AND deleted_at IS NOT NULL;

    RETURN FOUND;
END;
$$;


--
-- Name: search_room_content(text, text, text[], date, date, text, integer, integer); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.search_room_content(p_room_slug text, p_query text, p_content_types text[] DEFAULT ARRAY['alerts'::text, 'trades'::text, 'trade_plans'::text], p_from_date date DEFAULT NULL::date, p_to_date date DEFAULT NULL::date, p_ticker text DEFAULT NULL::text, p_limit integer DEFAULT 20, p_offset integer DEFAULT 0) RETURNS TABLE(content_type text, id bigint, ticker text, title text, content_preview text, relevance_score real, highlight text, created_at timestamp with time zone, extra_data jsonb)
    LANGUAGE plpgsql STABLE
    AS $$
DECLARE
    v_ts_query tsquery;
BEGIN
    -- Convert query to tsquery using plainto_tsquery for safety
    v_ts_query := plainto_tsquery('english', p_query);

    -- Return empty if query produces no terms
    IF v_ts_query IS NULL OR v_ts_query = ''::tsquery THEN
        RETURN;
    END IF;

    RETURN QUERY
    WITH ranked_results AS (
        -- Search Alerts
        SELECT
            'alert'::TEXT as content_type,
            a.id,
            a.ticker,
            a.title::TEXT as title,
            LEFT(a.message, 200) as content_preview,
            ts_rank_cd(
                to_tsvector('english', COALESCE(a.ticker, '') || ' ' || COALESCE(a.title, '') || ' ' || COALESCE(a.message, '') || ' ' || COALESCE(a.notes, '')),
                v_ts_query
            ) as relevance_score,
            ts_headline(
                'english',
                COALESCE(a.title, '') || ' - ' || COALESCE(a.message, ''),
                v_ts_query,
                'StartSel=<mark>, StopSel=</mark>, MaxWords=50, MinWords=20'
            ) as highlight,
            a.published_at as created_at,
            jsonb_build_object(
                'alert_type', a.alert_type,
                'is_new', a.is_new,
                'tos_string', a.tos_string
            ) as extra_data
        FROM room_alerts a
        WHERE a.room_slug = p_room_slug
        AND a.deleted_at IS NULL
        AND a.is_published = true
        AND 'alerts' = ANY(p_content_types)
        AND to_tsvector('english', COALESCE(a.ticker, '') || ' ' || COALESCE(a.title, '') || ' ' || COALESCE(a.message, '') || ' ' || COALESCE(a.notes, '')) @@ v_ts_query
        AND (p_from_date IS NULL OR a.published_at >= p_from_date)
        AND (p_to_date IS NULL OR a.published_at <= p_to_date)
        AND (p_ticker IS NULL OR UPPER(a.ticker) = UPPER(p_ticker))

        UNION ALL

        -- Search Trades
        SELECT
            'trade'::TEXT as content_type,
            t.id,
            t.ticker,
            (t.direction || ' ' || t.status)::TEXT as title,
            LEFT(COALESCE(t.notes, ''), 200) as content_preview,
            ts_rank_cd(
                to_tsvector('english', COALESCE(t.ticker, '') || ' ' || COALESCE(t.notes, '')),
                v_ts_query
            ) as relevance_score,
            ts_headline(
                'english',
                COALESCE(t.ticker, '') || ' - ' || COALESCE(t.notes, ''),
                v_ts_query,
                'StartSel=<mark>, StopSel=</mark>, MaxWords=50, MinWords=20'
            ) as highlight,
            t.created_at,
            jsonb_build_object(
                'status', t.status,
                'direction', t.direction,
                'entry_price', t.entry_price,
                'exit_price', t.exit_price,
                'pnl_percent', t.pnl_percent,
                'result', t.result
            ) as extra_data
        FROM room_trades t
        WHERE t.room_slug = p_room_slug
        AND t.deleted_at IS NULL
        AND 'trades' = ANY(p_content_types)
        AND to_tsvector('english', COALESCE(t.ticker, '') || ' ' || COALESCE(t.notes, '')) @@ v_ts_query
        AND (p_from_date IS NULL OR t.entry_date >= p_from_date)
        AND (p_to_date IS NULL OR t.entry_date <= p_to_date)
        AND (p_ticker IS NULL OR UPPER(t.ticker) = UPPER(p_ticker))

        UNION ALL

        -- Search Trade Plans
        SELECT
            'trade_plan'::TEXT as content_type,
            p.id,
            p.ticker,
            (p.bias || ' - ' || COALESCE(p.entry, 'N/A'))::TEXT as title,
            LEFT(COALESCE(p.notes, ''), 200) as content_preview,
            ts_rank_cd(
                to_tsvector('english', COALESCE(p.ticker, '') || ' ' || COALESCE(p.notes, '')),
                v_ts_query
            ) as relevance_score,
            ts_headline(
                'english',
                COALESCE(p.ticker, '') || ' - ' || COALESCE(p.notes, ''),
                v_ts_query,
                'StartSel=<mark>, StopSel=</mark>, MaxWords=50, MinWords=20'
            ) as highlight,
            p.created_at,
            jsonb_build_object(
                'bias', p.bias,
                'entry', p.entry,
                'target1', p.target1,
                'stop', p.stop,
                'week_of', p.week_of,
                'is_active', p.is_active
            ) as extra_data
        FROM room_trade_plans p
        WHERE p.room_slug = p_room_slug
        AND p.deleted_at IS NULL
        AND 'trade_plans' = ANY(p_content_types)
        AND to_tsvector('english', COALESCE(p.ticker, '') || ' ' || COALESCE(p.notes, '')) @@ v_ts_query
        AND (p_from_date IS NULL OR p.week_of >= p_from_date)
        AND (p_to_date IS NULL OR p.week_of <= p_to_date)
        AND (p_ticker IS NULL OR UPPER(p.ticker) = UPPER(p_ticker))
    )
    SELECT * FROM ranked_results
    ORDER BY relevance_score DESC, created_at DESC
    LIMIT p_limit
    OFFSET p_offset;
END;
$$;


--
-- Name: FUNCTION search_room_content(p_room_slug text, p_query text, p_content_types text[], p_from_date date, p_to_date date, p_ticker text, p_limit integer, p_offset integer); Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON FUNCTION public.search_room_content(p_room_slug text, p_query text, p_content_types text[], p_from_date date, p_to_date date, p_ticker text, p_limit integer, p_offset integer) IS 'Unified full-text search across room alerts, trades, and trade plans with relevance ranking';


--
-- Name: search_room_content_count(text, text, text[], date, date, text); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.search_room_content_count(p_room_slug text, p_query text, p_content_types text[] DEFAULT ARRAY['alerts'::text, 'trades'::text, 'trade_plans'::text], p_from_date date DEFAULT NULL::date, p_to_date date DEFAULT NULL::date, p_ticker text DEFAULT NULL::text) RETURNS bigint
    LANGUAGE plpgsql STABLE
    AS $$
DECLARE
    v_ts_query tsquery;
    v_count BIGINT := 0;
    v_alert_count BIGINT := 0;
    v_trade_count BIGINT := 0;
    v_plan_count BIGINT := 0;
BEGIN
    v_ts_query := plainto_tsquery('english', p_query);

    IF v_ts_query IS NULL OR v_ts_query = ''::tsquery THEN
        RETURN 0;
    END IF;

    -- Count alerts
    IF 'alerts' = ANY(p_content_types) THEN
        SELECT COUNT(*) INTO v_alert_count
        FROM room_alerts a
        WHERE a.room_slug = p_room_slug
        AND a.deleted_at IS NULL
        AND a.is_published = true
        AND to_tsvector('english', COALESCE(a.ticker, '') || ' ' || COALESCE(a.title, '') || ' ' || COALESCE(a.message, '') || ' ' || COALESCE(a.notes, '')) @@ v_ts_query
        AND (p_from_date IS NULL OR a.published_at >= p_from_date)
        AND (p_to_date IS NULL OR a.published_at <= p_to_date)
        AND (p_ticker IS NULL OR UPPER(a.ticker) = UPPER(p_ticker));
    END IF;

    -- Count trades
    IF 'trades' = ANY(p_content_types) THEN
        SELECT COUNT(*) INTO v_trade_count
        FROM room_trades t
        WHERE t.room_slug = p_room_slug
        AND t.deleted_at IS NULL
        AND to_tsvector('english', COALESCE(t.ticker, '') || ' ' || COALESCE(t.notes, '')) @@ v_ts_query
        AND (p_from_date IS NULL OR t.entry_date >= p_from_date)
        AND (p_to_date IS NULL OR t.entry_date <= p_to_date)
        AND (p_ticker IS NULL OR UPPER(t.ticker) = UPPER(p_ticker));
    END IF;

    -- Count trade plans
    IF 'trade_plans' = ANY(p_content_types) THEN
        SELECT COUNT(*) INTO v_plan_count
        FROM room_trade_plans p
        WHERE p.room_slug = p_room_slug
        AND p.deleted_at IS NULL
        AND to_tsvector('english', COALESCE(p.ticker, '') || ' ' || COALESCE(p.notes, '')) @@ v_ts_query
        AND (p_from_date IS NULL OR p.week_of >= p_from_date)
        AND (p_to_date IS NULL OR p.week_of <= p_to_date)
        AND (p_ticker IS NULL OR UPPER(p.ticker) = UPPER(p_ticker));
    END IF;

    RETURN v_alert_count + v_trade_count + v_plan_count;
END;
$$;


--
-- Name: FUNCTION search_room_content_count(p_room_slug text, p_query text, p_content_types text[], p_from_date date, p_to_date date, p_ticker text); Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON FUNCTION public.search_room_content_count(p_room_slug text, p_query text, p_content_types text[], p_from_date date, p_to_date date, p_ticker text) IS 'Count function for paginating search results';


--
-- Name: soft_delete_user(bigint, bigint, character varying); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.soft_delete_user(p_user_id bigint, p_deleted_by bigint, p_reason character varying DEFAULT NULL::character varying) RETURNS boolean
    LANGUAGE plpgsql
    AS $$
BEGIN
    UPDATE users
    SET
        deleted_at = NOW(),
        deleted_by = p_deleted_by,
        deletion_reason = p_reason,
        updated_at = NOW()
    WHERE id = p_user_id AND deleted_at IS NULL;

    RETURN FOUND;
END;
$$;


--
-- Name: trigger_recalculate_room_stats(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.trigger_recalculate_room_stats() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    -- Recalculate stats for the affected room
    PERFORM calculate_room_stats(COALESCE(NEW.room_slug, OLD.room_slug));
    RETURN NEW;
END;
$$;


--
-- Name: update_category_course_count(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.update_category_course_count() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE course_categories SET course_count = course_count + 1 WHERE id = NEW.category_id;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE course_categories SET course_count = GREATEST(course_count - 1, 0) WHERE id = OLD.category_id;
    END IF;
    RETURN NULL;
END;
$$;


--
-- Name: update_popup_conversion_rate(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.update_popup_conversion_rate() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    UPDATE popups
    SET conversion_rate = CASE
        WHEN total_views > 0 THEN (total_conversions::float / total_views::float) * 100
        ELSE 0
    END
    WHERE id = NEW.popup_id;
    RETURN NEW;
END;
$$;


--
-- Name: update_reusable_block_usage_count(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.update_reusable_block_usage_count() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE cms_reusable_blocks
        SET usage_count = usage_count + 1,
            updated_at = NOW()
        WHERE id = NEW.reusable_block_id;
        RETURN NEW;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE cms_reusable_blocks
        SET usage_count = GREATEST(0, usage_count - 1),
            updated_at = NOW()
        WHERE id = OLD.reusable_block_id;
        RETURN OLD;
    END IF;
    RETURN NULL;
END;
$$;


--
-- Name: update_schedule_timestamp(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.update_schedule_timestamp() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$;


--
-- Name: update_stock_lists_timestamp(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.update_stock_lists_timestamp() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$;


--
-- Name: update_tag_course_count(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.update_tag_course_count() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE course_tags SET course_count = course_count + 1 WHERE id = NEW.tag_id;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE course_tags SET course_count = GREATEST(course_count - 1, 0) WHERE id = OLD.tag_id;
    END IF;
    RETURN NULL;
END;
$$;


--
-- Name: update_updated_at_column(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.update_updated_at_column() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$;


--
-- Name: validate_secure_download_token(bigint, character varying); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.validate_secure_download_token(p_resource_id bigint, p_token character varying) RETURNS boolean
    LANGUAGE plpgsql
    AS $$
DECLARE
    v_valid BOOLEAN;
BEGIN
    SELECT EXISTS (
        SELECT 1 FROM room_resources
        WHERE id = p_resource_id
        AND secure_token = p_token
        AND secure_token_expires > NOW()
    ) INTO v_valid;

    RETURN v_valid;
END;
$$;


--
-- Name: cms_english; Type: TEXT SEARCH CONFIGURATION; Schema: public; Owner: -
--

CREATE TEXT SEARCH CONFIGURATION public.cms_english (
    PARSER = pg_catalog."default" );

ALTER TEXT SEARCH CONFIGURATION public.cms_english
    ADD MAPPING FOR asciiword WITH english_stem;

ALTER TEXT SEARCH CONFIGURATION public.cms_english
    ADD MAPPING FOR word WITH public.unaccent, english_stem;

ALTER TEXT SEARCH CONFIGURATION public.cms_english
    ADD MAPPING FOR numword WITH simple;

ALTER TEXT SEARCH CONFIGURATION public.cms_english
    ADD MAPPING FOR email WITH simple;

ALTER TEXT SEARCH CONFIGURATION public.cms_english
    ADD MAPPING FOR url WITH simple;

ALTER TEXT SEARCH CONFIGURATION public.cms_english
    ADD MAPPING FOR host WITH simple;

ALTER TEXT SEARCH CONFIGURATION public.cms_english
    ADD MAPPING FOR sfloat WITH simple;

ALTER TEXT SEARCH CONFIGURATION public.cms_english
    ADD MAPPING FOR version WITH simple;

ALTER TEXT SEARCH CONFIGURATION public.cms_english
    ADD MAPPING FOR hword_numpart WITH simple;

ALTER TEXT SEARCH CONFIGURATION public.cms_english
    ADD MAPPING FOR hword_part WITH public.unaccent, english_stem;

ALTER TEXT SEARCH CONFIGURATION public.cms_english
    ADD MAPPING FOR hword_asciipart WITH english_stem;

ALTER TEXT SEARCH CONFIGURATION public.cms_english
    ADD MAPPING FOR numhword WITH simple;

ALTER TEXT SEARCH CONFIGURATION public.cms_english
    ADD MAPPING FOR asciihword WITH english_stem;

ALTER TEXT SEARCH CONFIGURATION public.cms_english
    ADD MAPPING FOR hword WITH public.unaccent, english_stem;

ALTER TEXT SEARCH CONFIGURATION public.cms_english
    ADD MAPPING FOR url_path WITH simple;

ALTER TEXT SEARCH CONFIGURATION public.cms_english
    ADD MAPPING FOR file WITH simple;

ALTER TEXT SEARCH CONFIGURATION public.cms_english
    ADD MAPPING FOR "float" WITH simple;

ALTER TEXT SEARCH CONFIGURATION public.cms_english
    ADD MAPPING FOR "int" WITH simple;

ALTER TEXT SEARCH CONFIGURATION public.cms_english
    ADD MAPPING FOR uint WITH simple;


SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: admin_audit_logs; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.admin_audit_logs (
    id bigint NOT NULL,
    admin_id bigint NOT NULL,
    admin_email text,
    action text NOT NULL,
    entity_type text NOT NULL,
    entity_id bigint,
    old_value text,
    new_value text,
    metadata jsonb,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: TABLE admin_audit_logs; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.admin_audit_logs IS 'Append-only audit trail for admin-initiated mutations (connections, plans, etc).';


--
-- Name: admin_audit_logs_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.admin_audit_logs_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: admin_audit_logs_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.admin_audit_logs_id_seq OWNED BY public.admin_audit_logs.id;


--
-- Name: analytics_events; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.analytics_events (
    id bigint NOT NULL,
    user_id bigint,
    session_id character varying(255),
    event_type character varying(100) NOT NULL,
    event_name character varying(255) NOT NULL,
    page_url character varying(500),
    referrer character varying(500),
    user_agent text,
    ip_address character varying(45),
    properties jsonb,
    created_at timestamp without time zone DEFAULT now()
);


--
-- Name: analytics_events_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.analytics_events_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: analytics_events_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.analytics_events_id_seq OWNED BY public.analytics_events.id;


--
-- Name: application_settings; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.application_settings (
    id bigint NOT NULL,
    key character varying(255) NOT NULL,
    value jsonb,
    group_name character varying(100),
    description text,
    created_at timestamp without time zone DEFAULT now(),
    updated_at timestamp without time zone DEFAULT now()
);


--
-- Name: application_settings_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.application_settings_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: application_settings_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.application_settings_id_seq OWNED BY public.application_settings.id;


--
-- Name: audit_logs; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.audit_logs (
    id bigint NOT NULL,
    action character varying(50) NOT NULL,
    entity_type character varying(50) NOT NULL,
    entity_id bigint,
    user_id bigint,
    user_email character varying(255),
    user_role character varying(50),
    ip_address inet,
    user_agent text,
    request_method character varying(10),
    request_path character varying(500),
    old_values jsonb,
    new_values jsonb,
    metadata jsonb,
    status character varying(20) DEFAULT 'success'::character varying NOT NULL,
    error_message text,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: TABLE audit_logs; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.audit_logs IS 'Comprehensive audit trail for all system actions';


--
-- Name: audit_logs_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.audit_logs_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: audit_logs_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.audit_logs_id_seq OWNED BY public.audit_logs.id;


--
-- Name: bunny_uploads; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.bunny_uploads (
    id bigint NOT NULL,
    video_guid character varying(100) NOT NULL,
    library_id bigint NOT NULL,
    title character varying(500),
    status character varying(50) DEFAULT 'uploading'::character varying,
    file_size_bytes bigint,
    duration_seconds integer,
    thumbnail_url character varying(500),
    video_url character varying(500),
    uploaded_by bigint,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now(),
    room_slug character varying(100)
);


--
-- Name: COLUMN bunny_uploads.room_slug; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.bunny_uploads.room_slug IS 'Trading room slug (explosive-swings, spx-profit-pulse, etc.) for video organization';


--
-- Name: bunny_uploads_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.bunny_uploads_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: bunny_uploads_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.bunny_uploads_id_seq OWNED BY public.bunny_uploads.id;


--
-- Name: categories; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.categories (
    id bigint NOT NULL,
    name character varying(255) NOT NULL,
    slug character varying(255) NOT NULL,
    description text,
    parent_id bigint,
    created_at timestamp without time zone DEFAULT now(),
    updated_at timestamp without time zone DEFAULT now()
);


--
-- Name: categories_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.categories_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: categories_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.categories_id_seq OWNED BY public.categories.id;


--
-- Name: cms_ai_assist_history; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_ai_assist_history (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    user_id uuid NOT NULL,
    content_id uuid,
    block_id character varying(255),
    action public.cms_ai_action NOT NULL,
    input_text text NOT NULL,
    output_text text,
    options jsonb DEFAULT '{}'::jsonb,
    model_used character varying(100) DEFAULT 'claude-sonnet-4-20250514'::character varying,
    input_tokens integer,
    output_tokens integer,
    latency_ms integer,
    was_applied boolean DEFAULT false,
    applied_at timestamp with time zone,
    error_message text,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: cms_asset_folders; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_asset_folders (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    name character varying(255) NOT NULL,
    slug character varying(255) NOT NULL,
    parent_id uuid,
    path text DEFAULT '/'::text NOT NULL,
    depth integer DEFAULT 0 NOT NULL,
    description text,
    color character varying(7),
    icon character varying(50),
    is_public boolean DEFAULT false NOT NULL,
    allowed_roles public.cms_user_role[],
    sort_order integer DEFAULT 0 NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    created_by uuid,
    updated_by uuid,
    CONSTRAINT valid_folder_slug CHECK (((slug)::text ~ '^[a-z0-9]+(?:-[a-z0-9]+)*$'::text))
);


--
-- Name: TABLE cms_asset_folders; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.cms_asset_folders IS 'Hierarchical folder structure for organizing digital assets';


--
-- Name: COLUMN cms_asset_folders.path; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.cms_asset_folders.path IS 'Full path from root (e.g., /images/blog/2026)';


--
-- Name: COLUMN cms_asset_folders.depth; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.cms_asset_folders.depth IS 'Nesting level (0 = root level)';


--
-- Name: cms_assets; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_assets (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    folder_id uuid,
    filename character varying(255) NOT NULL,
    original_filename character varying(255) NOT NULL,
    mime_type character varying(100) NOT NULL,
    file_size bigint NOT NULL,
    file_extension character varying(20) NOT NULL,
    storage_provider character varying(50) DEFAULT 'r2'::character varying NOT NULL,
    storage_key character varying(500) NOT NULL,
    cdn_url character varying(500) NOT NULL,
    width integer,
    height integer,
    aspect_ratio numeric(10,6),
    blurhash character varying(100),
    dominant_color character varying(7),
    duration_seconds integer,
    video_codec character varying(50),
    audio_codec character varying(50),
    bunny_video_id character varying(100),
    bunny_library_id integer,
    thumbnail_url character varying(500),
    variants jsonb DEFAULT '[]'::jsonb,
    title character varying(255),
    alt_text character varying(500),
    caption text,
    description text,
    credits character varying(500),
    seo_title character varying(255),
    seo_description character varying(500),
    tags text[] DEFAULT ARRAY[]::text[],
    usage_count integer DEFAULT 0 NOT NULL,
    last_used_at timestamp with time zone,
    deleted_at timestamp with time zone,
    version integer DEFAULT 1 NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    created_by uuid,
    updated_by uuid,
    asset_type public.cms_asset_type NOT NULL
);


--
-- Name: TABLE cms_assets; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.cms_assets IS 'Digital Asset Management for all media files';


--
-- Name: COLUMN cms_assets.storage_key; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.cms_assets.storage_key IS 'Unique key in cloud storage (R2/S3)';


--
-- Name: COLUMN cms_assets.blurhash; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.cms_assets.blurhash IS 'BlurHash for progressive image loading';


--
-- Name: COLUMN cms_assets.variants; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.cms_assets.variants IS 'Array of responsive image sizes or video transcodes';


--
-- Name: cms_audit_log; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_audit_log (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    action character varying(100) NOT NULL,
    entity_type character varying(50) NOT NULL,
    entity_id uuid,
    user_id uuid,
    user_email character varying(255),
    user_role public.cms_user_role,
    ip_address inet,
    user_agent text,
    request_method character varying(10),
    request_path character varying(500),
    old_values jsonb,
    new_values jsonb,
    metadata jsonb,
    status character varying(20) DEFAULT 'success'::character varying NOT NULL,
    error_message text,
    created_at timestamp with time zone DEFAULT now() NOT NULL
)
PARTITION BY RANGE (created_at);


--
-- Name: TABLE cms_audit_log; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.cms_audit_log IS 'Partitioned audit log for all CMS actions (compliance-ready)';


--
-- Name: cms_audit_log_2026_05; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_audit_log_2026_05 (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    action character varying(100) NOT NULL,
    entity_type character varying(50) NOT NULL,
    entity_id uuid,
    user_id uuid,
    user_email character varying(255),
    user_role public.cms_user_role,
    ip_address inet,
    user_agent text,
    request_method character varying(10),
    request_path character varying(500),
    old_values jsonb,
    new_values jsonb,
    metadata jsonb,
    status character varying(20) DEFAULT 'success'::character varying NOT NULL,
    error_message text,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: cms_audit_log_2026_06; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_audit_log_2026_06 (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    action character varying(100) NOT NULL,
    entity_type character varying(50) NOT NULL,
    entity_id uuid,
    user_id uuid,
    user_email character varying(255),
    user_role public.cms_user_role,
    ip_address inet,
    user_agent text,
    request_method character varying(10),
    request_path character varying(500),
    old_values jsonb,
    new_values jsonb,
    metadata jsonb,
    status character varying(20) DEFAULT 'success'::character varying NOT NULL,
    error_message text,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: cms_audit_log_2026_07; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_audit_log_2026_07 (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    action character varying(100) NOT NULL,
    entity_type character varying(50) NOT NULL,
    entity_id uuid,
    user_id uuid,
    user_email character varying(255),
    user_role public.cms_user_role,
    ip_address inet,
    user_agent text,
    request_method character varying(10),
    request_path character varying(500),
    old_values jsonb,
    new_values jsonb,
    metadata jsonb,
    status character varying(20) DEFAULT 'success'::character varying NOT NULL,
    error_message text,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: cms_audit_log_2026_08; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_audit_log_2026_08 (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    action character varying(100) NOT NULL,
    entity_type character varying(50) NOT NULL,
    entity_id uuid,
    user_id uuid,
    user_email character varying(255),
    user_role public.cms_user_role,
    ip_address inet,
    user_agent text,
    request_method character varying(10),
    request_path character varying(500),
    old_values jsonb,
    new_values jsonb,
    metadata jsonb,
    status character varying(20) DEFAULT 'success'::character varying NOT NULL,
    error_message text,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: cms_audit_log_2026_09; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_audit_log_2026_09 (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    action character varying(100) NOT NULL,
    entity_type character varying(50) NOT NULL,
    entity_id uuid,
    user_id uuid,
    user_email character varying(255),
    user_role public.cms_user_role,
    ip_address inet,
    user_agent text,
    request_method character varying(10),
    request_path character varying(500),
    old_values jsonb,
    new_values jsonb,
    metadata jsonb,
    status character varying(20) DEFAULT 'success'::character varying NOT NULL,
    error_message text,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: cms_audit_log_2026_10; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_audit_log_2026_10 (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    action character varying(100) NOT NULL,
    entity_type character varying(50) NOT NULL,
    entity_id uuid,
    user_id uuid,
    user_email character varying(255),
    user_role public.cms_user_role,
    ip_address inet,
    user_agent text,
    request_method character varying(10),
    request_path character varying(500),
    old_values jsonb,
    new_values jsonb,
    metadata jsonb,
    status character varying(20) DEFAULT 'success'::character varying NOT NULL,
    error_message text,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: cms_audit_log_2026_11; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_audit_log_2026_11 (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    action character varying(100) NOT NULL,
    entity_type character varying(50) NOT NULL,
    entity_id uuid,
    user_id uuid,
    user_email character varying(255),
    user_role public.cms_user_role,
    ip_address inet,
    user_agent text,
    request_method character varying(10),
    request_path character varying(500),
    old_values jsonb,
    new_values jsonb,
    metadata jsonb,
    status character varying(20) DEFAULT 'success'::character varying NOT NULL,
    error_message text,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: cms_audit_log_2026_12; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_audit_log_2026_12 (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    action character varying(100) NOT NULL,
    entity_type character varying(50) NOT NULL,
    entity_id uuid,
    user_id uuid,
    user_email character varying(255),
    user_role public.cms_user_role,
    ip_address inet,
    user_agent text,
    request_method character varying(10),
    request_path character varying(500),
    old_values jsonb,
    new_values jsonb,
    metadata jsonb,
    status character varying(20) DEFAULT 'success'::character varying NOT NULL,
    error_message text,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: cms_audit_log_2027_01; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_audit_log_2027_01 (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    action character varying(100) NOT NULL,
    entity_type character varying(50) NOT NULL,
    entity_id uuid,
    user_id uuid,
    user_email character varying(255),
    user_role public.cms_user_role,
    ip_address inet,
    user_agent text,
    request_method character varying(10),
    request_path character varying(500),
    old_values jsonb,
    new_values jsonb,
    metadata jsonb,
    status character varying(20) DEFAULT 'success'::character varying NOT NULL,
    error_message text,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: cms_audit_log_2027_02; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_audit_log_2027_02 (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    action character varying(100) NOT NULL,
    entity_type character varying(50) NOT NULL,
    entity_id uuid,
    user_id uuid,
    user_email character varying(255),
    user_role public.cms_user_role,
    ip_address inet,
    user_agent text,
    request_method character varying(10),
    request_path character varying(500),
    old_values jsonb,
    new_values jsonb,
    metadata jsonb,
    status character varying(20) DEFAULT 'success'::character varying NOT NULL,
    error_message text,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: cms_audit_log_2027_03; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_audit_log_2027_03 (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    action character varying(100) NOT NULL,
    entity_type character varying(50) NOT NULL,
    entity_id uuid,
    user_id uuid,
    user_email character varying(255),
    user_role public.cms_user_role,
    ip_address inet,
    user_agent text,
    request_method character varying(10),
    request_path character varying(500),
    old_values jsonb,
    new_values jsonb,
    metadata jsonb,
    status character varying(20) DEFAULT 'success'::character varying NOT NULL,
    error_message text,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: cms_audit_log_2027_04; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_audit_log_2027_04 (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    action character varying(100) NOT NULL,
    entity_type character varying(50) NOT NULL,
    entity_id uuid,
    user_id uuid,
    user_email character varying(255),
    user_role public.cms_user_role,
    ip_address inet,
    user_agent text,
    request_method character varying(10),
    request_path character varying(500),
    old_values jsonb,
    new_values jsonb,
    metadata jsonb,
    status character varying(20) DEFAULT 'success'::character varying NOT NULL,
    error_message text,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: cms_audit_log_2027_05; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_audit_log_2027_05 (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    action character varying(100) NOT NULL,
    entity_type character varying(50) NOT NULL,
    entity_id uuid,
    user_id uuid,
    user_email character varying(255),
    user_role public.cms_user_role,
    ip_address inet,
    user_agent text,
    request_method character varying(10),
    request_path character varying(500),
    old_values jsonb,
    new_values jsonb,
    metadata jsonb,
    status character varying(20) DEFAULT 'success'::character varying NOT NULL,
    error_message text,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: cms_audit_logs; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_audit_logs (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    action character varying(100) NOT NULL,
    entity_type character varying(50) NOT NULL,
    entity_id uuid,
    user_id bigint,
    user_email character varying(255),
    old_values jsonb,
    new_values jsonb,
    metadata jsonb,
    ip_address inet,
    user_agent text,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: cms_comment_notifications; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_comment_notifications (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    comment_id uuid NOT NULL,
    user_id uuid NOT NULL,
    notification_type character varying(50) NOT NULL,
    is_read boolean DEFAULT false NOT NULL,
    read_at timestamp with time zone,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: cms_comments; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_comments (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    content_id uuid NOT NULL,
    parent_id uuid,
    thread_id uuid,
    body text NOT NULL,
    block_id character varying(100),
    selection_start integer,
    selection_end integer,
    is_resolved boolean DEFAULT false NOT NULL,
    resolved_by uuid,
    resolved_at timestamp with time zone,
    mentioned_users uuid[] DEFAULT ARRAY[]::uuid[],
    deleted_at timestamp with time zone,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    created_by uuid
);


--
-- Name: TABLE cms_comments; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.cms_comments IS 'Editorial collaboration comments on content items';


--
-- Name: COLUMN cms_comments.block_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.cms_comments.block_id IS 'Reference to specific block for inline comments';


--
-- Name: cms_content; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_content (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    content_type public.cms_content_type NOT NULL,
    slug character varying(500) NOT NULL,
    locale character varying(10) DEFAULT 'en'::character varying NOT NULL,
    is_primary_locale boolean DEFAULT true NOT NULL,
    parent_content_id uuid,
    title character varying(500) NOT NULL,
    subtitle character varying(500),
    excerpt text,
    content text,
    content_blocks jsonb DEFAULT '[]'::jsonb,
    featured_image_id uuid,
    og_image_id uuid,
    gallery_ids uuid[] DEFAULT ARRAY[]::uuid[],
    meta_title character varying(70),
    meta_description character varying(160),
    meta_keywords text[],
    canonical_url character varying(500),
    robots_directives character varying(100) DEFAULT 'index, follow'::character varying,
    structured_data jsonb,
    author_id uuid,
    contributors uuid[] DEFAULT ARRAY[]::uuid[],
    status public.cms_content_status DEFAULT 'draft'::public.cms_content_status NOT NULL,
    published_at timestamp with time zone,
    scheduled_publish_at timestamp with time zone,
    scheduled_unpublish_at timestamp with time zone,
    primary_category_id uuid,
    categories uuid[] DEFAULT ARRAY[]::uuid[],
    custom_fields jsonb DEFAULT '{}'::jsonb,
    template character varying(100),
    deleted_at timestamp with time zone,
    version integer DEFAULT 1 NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    created_by uuid,
    updated_by uuid,
    content_mode public.cms_content_mode DEFAULT 'richtext'::public.cms_content_mode,
    search_vector tsvector,
    CONSTRAINT valid_content_slug CHECK (((slug)::text ~ '^[a-z0-9]+(?:-[a-z0-9]+)*$'::text))
);


--
-- Name: TABLE cms_content; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.cms_content IS 'Unified content storage for all CMS content types';


--
-- Name: COLUMN cms_content.content_blocks; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.cms_content.content_blocks IS 'Array of content blocks for page builder';


--
-- Name: COLUMN cms_content.structured_data; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.cms_content.structured_data IS 'JSON-LD structured data for SEO';


--
-- Name: COLUMN cms_content.custom_fields; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.cms_content.custom_fields IS 'Content-type specific fields stored as JSON';


--
-- Name: cms_content_relations; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_content_relations (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    source_id uuid NOT NULL,
    target_id uuid NOT NULL,
    relation_type character varying(50) NOT NULL,
    metadata jsonb DEFAULT '{}'::jsonb,
    sort_order integer DEFAULT 0 NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    created_by uuid,
    CONSTRAINT no_self_reference CHECK ((source_id <> target_id))
);


--
-- Name: TABLE cms_content_relations; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.cms_content_relations IS 'Many-to-many relationships between content items';


--
-- Name: COLUMN cms_content_relations.relation_type; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.cms_content_relations.relation_type IS 'Type of relation (e.g., "related_posts", "prerequisites", "see_also")';


--
-- Name: cms_content_tags; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_content_tags (
    content_id uuid NOT NULL,
    tag_id uuid NOT NULL,
    sort_order integer DEFAULT 0 NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    created_by uuid
);


--
-- Name: TABLE cms_content_tags; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.cms_content_tags IS 'Junction table for content-tag relationships';


--
-- Name: cms_datasource_entries; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_datasource_entries (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    datasource_id uuid NOT NULL,
    name character varying(255) NOT NULL,
    value character varying(500) NOT NULL,
    dimension character varying(50) DEFAULT 'default'::character varying,
    sort_order integer DEFAULT 0 NOT NULL,
    metadata jsonb DEFAULT '{}'::jsonb,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    CONSTRAINT cms_datasource_entries_name_check CHECK ((length(TRIM(BOTH FROM name)) > 0)),
    CONSTRAINT cms_datasource_entries_value_check CHECK ((length(TRIM(BOTH FROM value)) > 0))
);


--
-- Name: TABLE cms_datasource_entries; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.cms_datasource_entries IS 'Key-value entries within a datasource';


--
-- Name: COLUMN cms_datasource_entries.name; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.cms_datasource_entries.name IS 'Display name (label) for the entry';


--
-- Name: COLUMN cms_datasource_entries.value; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.cms_datasource_entries.value IS 'Actual value used in forms/selects';


--
-- Name: COLUMN cms_datasource_entries.dimension; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.cms_datasource_entries.dimension IS 'Dimension for translations (e.g., en, de, fr, default)';


--
-- Name: COLUMN cms_datasource_entries.sort_order; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.cms_datasource_entries.sort_order IS 'Manual sort order for drag-to-reorder';


--
-- Name: COLUMN cms_datasource_entries.metadata; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.cms_datasource_entries.metadata IS 'Optional extra data (icon, color, disabled state, etc.)';


--
-- Name: cms_datasources; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_datasources (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    name character varying(100) NOT NULL,
    slug character varying(100) NOT NULL,
    description text,
    icon character varying(50),
    color character varying(20),
    entry_count integer DEFAULT 0 NOT NULL,
    is_system boolean DEFAULT false NOT NULL,
    is_locked boolean DEFAULT false NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    created_by uuid,
    updated_by uuid,
    deleted_at timestamp with time zone,
    CONSTRAINT cms_datasources_name_check CHECK ((length(TRIM(BOTH FROM name)) > 0)),
    CONSTRAINT cms_datasources_slug_check CHECK (((slug)::text ~ '^[a-z0-9-]+$'::text))
);


--
-- Name: TABLE cms_datasources; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.cms_datasources IS 'CMS Datasources System - Reusable option lists for dropdowns and selections';


--
-- Name: COLUMN cms_datasources.slug; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.cms_datasources.slug IS 'URL-friendly unique identifier for API access';


--
-- Name: COLUMN cms_datasources.is_system; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.cms_datasources.is_system IS 'System datasources cannot be deleted';


--
-- Name: COLUMN cms_datasources.is_locked; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.cms_datasources.is_locked IS 'Locked datasources cannot be edited by non-admins';


--
-- Name: cms_navigation_menus; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_navigation_menus (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    name character varying(100) NOT NULL,
    slug character varying(100) NOT NULL,
    location character varying(50),
    items jsonb DEFAULT '[]'::jsonb NOT NULL,
    is_active boolean DEFAULT true NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    created_by uuid,
    updated_by uuid,
    CONSTRAINT valid_menu_slug CHECK (((slug)::text ~ '^[a-z0-9]+(?:-[a-z0-9]+)*$'::text))
);


--
-- Name: TABLE cms_navigation_menus; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.cms_navigation_menus IS 'Navigation menu structures with hierarchical items';


--
-- Name: COLUMN cms_navigation_menus.location; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.cms_navigation_menus.location IS 'Menu location identifier (e.g., header, footer, sidebar)';


--
-- Name: COLUMN cms_navigation_menus.items; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.cms_navigation_menus.items IS 'JSON array of menu items with nested children';


--
-- Name: cms_offline_sync_queue; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_offline_sync_queue (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    user_id uuid NOT NULL,
    content_id uuid NOT NULL,
    client_id character varying(255) NOT NULL,
    operation character varying(50) NOT NULL,
    local_version integer NOT NULL,
    server_version integer,
    payload jsonb NOT NULL,
    status public.cms_sync_status DEFAULT 'pending'::public.cms_sync_status NOT NULL,
    conflict_data jsonb,
    resolution character varying(50),
    resolved_at timestamp with time zone,
    resolved_by uuid,
    attempts integer DEFAULT 0 NOT NULL,
    last_attempt_at timestamp with time zone,
    error_message text,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    synced_at timestamp with time zone
);


--
-- Name: cms_presets; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_presets (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    block_type character varying(50) NOT NULL,
    name character varying(100) NOT NULL,
    slug character varying(100) NOT NULL,
    description text,
    preset_data jsonb DEFAULT '{}'::jsonb NOT NULL,
    thumbnail_url text,
    thumbnail_blurhash character varying(100),
    category public.cms_preset_category DEFAULT 'custom'::public.cms_preset_category NOT NULL,
    tags text[] DEFAULT '{}'::text[],
    is_default boolean DEFAULT false NOT NULL,
    is_locked boolean DEFAULT false NOT NULL,
    is_global boolean DEFAULT true NOT NULL,
    usage_count integer DEFAULT 0 NOT NULL,
    version integer DEFAULT 1 NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    created_by uuid,
    updated_by uuid,
    deleted_at timestamp with time zone,
    CONSTRAINT cms_presets_name_check CHECK ((length(TRIM(BOTH FROM name)) > 0)),
    CONSTRAINT cms_presets_slug_check CHECK (((slug)::text ~ '^[a-z0-9-]+$'::text))
);


--
-- Name: TABLE cms_presets; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.cms_presets IS 'CMS Presets/Templates System - Component presets for the block editor';


--
-- Name: COLUMN cms_presets.block_type; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.cms_presets.block_type IS 'The block type this preset applies to (e.g., button, heading)';


--
-- Name: COLUMN cms_presets.preset_data; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.cms_presets.preset_data IS 'JSON containing content and settings for the preset';


--
-- Name: COLUMN cms_presets.is_default; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.cms_presets.is_default IS 'Whether this preset appears as the default for its block type';


--
-- Name: COLUMN cms_presets.is_locked; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.cms_presets.is_locked IS 'Prevents editing by non-admin users';


--
-- Name: COLUMN cms_presets.is_global; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.cms_presets.is_global IS 'Available to all users vs. creator only';


--
-- Name: cms_preview_tokens; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_preview_tokens (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    content_id uuid NOT NULL,
    token uuid DEFAULT gen_random_uuid() NOT NULL,
    max_views integer,
    view_count integer DEFAULT 0 NOT NULL,
    expires_at timestamp with time zone NOT NULL,
    created_by bigint,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    last_accessed_at timestamp with time zone
);


--
-- Name: cms_redirects; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_redirects (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    source_path character varying(500) NOT NULL,
    target_path character varying(500) NOT NULL,
    status_code integer DEFAULT 301 NOT NULL,
    is_regex boolean DEFAULT false NOT NULL,
    preserve_query_string boolean DEFAULT true NOT NULL,
    hit_count integer DEFAULT 0 NOT NULL,
    last_hit_at timestamp with time zone,
    is_active boolean DEFAULT true NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    created_by uuid,
    CONSTRAINT valid_status_code CHECK ((status_code = ANY (ARRAY[301, 302, 307, 308])))
);


--
-- Name: TABLE cms_redirects; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.cms_redirects IS 'URL redirect rules for SEO and content migration';


--
-- Name: cms_release_items; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_release_items (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    release_id uuid NOT NULL,
    content_id uuid NOT NULL,
    action public.cms_schedule_action DEFAULT 'publish'::public.cms_schedule_action NOT NULL,
    order_index integer DEFAULT 0 NOT NULL,
    staged_data jsonb,
    status public.cms_schedule_status DEFAULT 'pending'::public.cms_schedule_status NOT NULL,
    executed_at timestamp with time zone,
    error_message text,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: cms_releases; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_releases (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    name character varying(255) NOT NULL,
    description text,
    scheduled_at timestamp with time zone,
    timezone character varying(64) DEFAULT 'UTC'::character varying NOT NULL,
    status public.cms_release_status DEFAULT 'draft'::public.cms_release_status NOT NULL,
    executed_at timestamp with time zone,
    error_message text,
    total_items integer DEFAULT 0 NOT NULL,
    completed_items integer DEFAULT 0 NOT NULL,
    failed_items integer DEFAULT 0 NOT NULL,
    stop_on_error boolean DEFAULT false NOT NULL,
    notify_on_complete boolean DEFAULT true NOT NULL,
    created_by bigint,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    approved_by bigint,
    approved_at timestamp with time zone,
    cancelled_by bigint,
    cancelled_at timestamp with time zone
);


--
-- Name: cms_reusable_block_usage; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_reusable_block_usage (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    reusable_block_id uuid NOT NULL,
    content_id uuid NOT NULL,
    block_instance_id character varying(255) NOT NULL,
    is_synced boolean DEFAULT true NOT NULL,
    detached_at timestamp with time zone,
    detached_by uuid,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: cms_reusable_blocks; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_reusable_blocks (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    name character varying(255) NOT NULL,
    slug character varying(255) NOT NULL,
    description text,
    block_data jsonb NOT NULL,
    category public.cms_reusable_block_category DEFAULT 'general'::public.cms_reusable_block_category NOT NULL,
    tags text[] DEFAULT '{}'::text[],
    thumbnail_url text,
    preview_html text,
    usage_count integer DEFAULT 0 NOT NULL,
    is_global boolean DEFAULT true NOT NULL,
    is_locked boolean DEFAULT false NOT NULL,
    allowed_content_types public.cms_content_type[],
    version integer DEFAULT 1 NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    created_by uuid,
    updated_by uuid,
    deleted_at timestamp with time zone,
    cms_user_id uuid
);


--
-- Name: cms_revisions; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_revisions (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    content_id uuid NOT NULL,
    revision_number integer NOT NULL,
    is_current boolean DEFAULT false NOT NULL,
    data jsonb NOT NULL,
    change_summary character varying(500),
    changed_fields text[],
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    created_by uuid,
    change_type character varying(50) DEFAULT 'manual'::character varying,
    word_count integer,
    diff_stats jsonb
);


--
-- Name: TABLE cms_revisions; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.cms_revisions IS 'Version history for content with full rollback capability';


--
-- Name: cms_schedule_history; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_schedule_history (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    schedule_id uuid,
    release_id uuid,
    content_id uuid,
    event_type character varying(100) NOT NULL,
    previous_status character varying(50),
    new_status character varying(50),
    performed_by bigint,
    event_data jsonb,
    error_details text,
    user_id bigint,
    user_email character varying(255),
    ip_address inet,
    user_agent text,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    CONSTRAINT schedule_or_release_required CHECK (((schedule_id IS NOT NULL) OR (release_id IS NOT NULL)))
);


--
-- Name: cms_scheduled_jobs; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_scheduled_jobs (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    job_type character varying(50) NOT NULL,
    content_id uuid,
    scheduled_at timestamp with time zone NOT NULL,
    timezone character varying(50) DEFAULT 'UTC'::character varying,
    payload jsonb DEFAULT '{}'::jsonb,
    status character varying(20) DEFAULT 'pending'::character varying NOT NULL,
    attempts integer DEFAULT 0 NOT NULL,
    max_attempts integer DEFAULT 3 NOT NULL,
    started_at timestamp with time zone,
    completed_at timestamp with time zone,
    error_message text,
    result jsonb,
    locked_by character varying(100),
    locked_at timestamp with time zone,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    created_by uuid
);


--
-- Name: TABLE cms_scheduled_jobs; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.cms_scheduled_jobs IS 'Queue for scheduled CMS operations';


--
-- Name: cms_schedules; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_schedules (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    content_id uuid NOT NULL,
    action public.cms_schedule_action DEFAULT 'publish'::public.cms_schedule_action NOT NULL,
    scheduled_at timestamp with time zone NOT NULL,
    timezone character varying(64) DEFAULT 'UTC'::character varying NOT NULL,
    status public.cms_schedule_status DEFAULT 'pending'::public.cms_schedule_status NOT NULL,
    executed_at timestamp with time zone,
    error_message text,
    retry_count integer DEFAULT 0 NOT NULL,
    max_retries integer DEFAULT 3 NOT NULL,
    staged_data jsonb,
    notes text,
    created_by bigint,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    cancelled_by bigint,
    cancelled_at timestamp with time zone,
    CONSTRAINT valid_retry_count CHECK (((retry_count >= 0) AND (retry_count <= max_retries))),
    CONSTRAINT valid_scheduled_at CHECK (((scheduled_at > created_at) OR (status <> 'pending'::public.cms_schedule_status)))
);


--
-- Name: cms_site_settings; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_site_settings (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    site_name character varying(255) DEFAULT 'Revolution Trading Pros'::character varying NOT NULL,
    site_tagline character varying(500),
    site_description text,
    logo_light_id uuid,
    logo_dark_id uuid,
    favicon_id uuid,
    og_default_image_id uuid,
    contact_email character varying(255),
    support_email character varying(255),
    phone character varying(50),
    address text,
    social_links jsonb DEFAULT '{}'::jsonb,
    default_meta_title_suffix character varying(100) DEFAULT ' | Revolution Trading Pros'::character varying,
    default_robots character varying(100) DEFAULT 'index, follow'::character varying,
    google_analytics_id character varying(50),
    google_tag_manager_id character varying(50),
    maintenance_mode boolean DEFAULT false NOT NULL,
    maintenance_message text,
    head_scripts text,
    body_start_scripts text,
    body_end_scripts text,
    custom_css text,
    settings jsonb DEFAULT '{}'::jsonb,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_by uuid,
    CONSTRAINT single_settings CHECK ((id = '00000000-0000-0000-0000-000000000001'::uuid))
);


--
-- Name: TABLE cms_site_settings; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.cms_site_settings IS 'Global site settings (singleton table)';


--
-- Name: cms_tags; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_tags (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    name character varying(100) NOT NULL,
    slug character varying(100) NOT NULL,
    parent_id uuid,
    description text,
    color character varying(7),
    icon character varying(50),
    meta_title character varying(70),
    meta_description character varying(160),
    usage_count integer DEFAULT 0 NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    created_by uuid,
    CONSTRAINT valid_tag_slug CHECK (((slug)::text ~ '^[a-z0-9]+(?:-[a-z0-9]+)*$'::text))
);


--
-- Name: TABLE cms_tags; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.cms_tags IS 'Taxonomy tags for content organization';


--
-- Name: cms_user_editor_preferences; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_user_editor_preferences (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    user_id uuid NOT NULL,
    autosave_enabled boolean DEFAULT true NOT NULL,
    autosave_interval_seconds integer DEFAULT 10 NOT NULL,
    focus_mode_enabled boolean DEFAULT false NOT NULL,
    focus_dim_sidebar boolean DEFAULT true NOT NULL,
    focus_dim_toolbar boolean DEFAULT true NOT NULL,
    focus_dim_blocks boolean DEFAULT true NOT NULL,
    daily_word_goal integer,
    show_word_count boolean DEFAULT true NOT NULL,
    show_reading_time boolean DEFAULT true NOT NULL,
    show_character_count boolean DEFAULT false NOT NULL,
    custom_shortcuts jsonb DEFAULT '{}'::jsonb,
    sidebar_collapsed boolean DEFAULT false NOT NULL,
    show_block_handles boolean DEFAULT true NOT NULL,
    show_formatting_toolbar boolean DEFAULT true NOT NULL,
    default_block_type character varying(50) DEFAULT 'paragraph'::character varying,
    ai_enabled boolean DEFAULT true NOT NULL,
    ai_auto_suggest boolean DEFAULT false NOT NULL,
    editor_theme character varying(20) DEFAULT 'system'::character varying,
    font_size integer DEFAULT 16,
    line_height numeric(3,2) DEFAULT 1.75,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: cms_users; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_users (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    user_id bigint,
    display_name character varying(255) NOT NULL,
    avatar_url character varying(500),
    bio text,
    role public.cms_user_role DEFAULT 'viewer'::public.cms_user_role NOT NULL,
    permissions jsonb DEFAULT '[]'::jsonb,
    allowed_content_types public.cms_content_type[],
    preferences jsonb DEFAULT '{}'::jsonb,
    notification_settings jsonb DEFAULT '{"email": true, "in_app": true}'::jsonb,
    is_active boolean DEFAULT true NOT NULL,
    last_login_at timestamp with time zone,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    created_by uuid,
    updated_by uuid
);


--
-- Name: TABLE cms_users; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.cms_users IS 'CMS user profiles with roles and permissions';


--
-- Name: COLUMN cms_users.user_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.cms_users.user_id IS 'Foreign key to main users table';


--
-- Name: COLUMN cms_users.role; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.cms_users.role IS 'User role determining permissions';


--
-- Name: COLUMN cms_users.allowed_content_types; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.cms_users.allowed_content_types IS 'Content types this user can edit (for restricted roles)';


--
-- Name: cms_webhook_deliveries; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_webhook_deliveries (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    webhook_id uuid NOT NULL,
    event_type character varying(100) NOT NULL,
    content_id uuid,
    payload jsonb NOT NULL,
    status character varying(20) DEFAULT 'pending'::character varying NOT NULL,
    attempts integer DEFAULT 0 NOT NULL,
    response_status integer,
    response_body text,
    response_time_ms integer,
    error_message text,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    delivered_at timestamp with time zone,
    next_retry_at timestamp with time zone
);


--
-- Name: TABLE cms_webhook_deliveries; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.cms_webhook_deliveries IS 'Log of webhook delivery attempts';


--
-- Name: cms_webhooks; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_webhooks (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    name character varying(100) NOT NULL,
    description text,
    url character varying(500) NOT NULL,
    secret character varying(255),
    events text[] DEFAULT ARRAY['content.published'::text] NOT NULL,
    content_types public.cms_content_type[],
    is_active boolean DEFAULT true NOT NULL,
    retry_count integer DEFAULT 3 NOT NULL,
    timeout_seconds integer DEFAULT 30 NOT NULL,
    headers jsonb DEFAULT '{}'::jsonb,
    last_triggered_at timestamp with time zone,
    success_count integer DEFAULT 0 NOT NULL,
    failure_count integer DEFAULT 0 NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    created_by uuid
);


--
-- Name: TABLE cms_webhooks; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.cms_webhooks IS 'Webhook configurations for CMS event notifications';


--
-- Name: cms_workflow_history; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_workflow_history (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    workflow_id uuid NOT NULL,
    content_id uuid NOT NULL,
    from_stage public.cms_workflow_stage,
    to_stage public.cms_workflow_stage NOT NULL,
    comment text,
    transitioned_by bigint,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: cms_workflow_log; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_workflow_log (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    content_id uuid NOT NULL,
    from_status public.cms_content_status,
    to_status public.cms_content_status NOT NULL,
    transitioned_by uuid,
    comment text,
    metadata jsonb DEFAULT '{}'::jsonb,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: TABLE cms_workflow_log; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.cms_workflow_log IS 'Audit trail for all workflow status transitions';


--
-- Name: cms_workflow_status; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cms_workflow_status (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    content_id uuid NOT NULL,
    current_stage public.cms_workflow_stage DEFAULT 'draft'::public.cms_workflow_stage NOT NULL,
    previous_stage public.cms_workflow_stage,
    assigned_to bigint,
    assigned_by bigint,
    assigned_at timestamp with time zone,
    due_date timestamp with time zone,
    priority public.cms_workflow_priority DEFAULT 'normal'::public.cms_workflow_priority,
    notes text,
    transition_comment text,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: contacts; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.contacts (
    id bigint NOT NULL,
    user_id bigint,
    email character varying(255) NOT NULL,
    first_name character varying(255),
    last_name character varying(255),
    phone character varying(50),
    company character varying(255),
    job_title character varying(255),
    source character varying(100),
    status character varying(50) DEFAULT 'lead'::character varying,
    tags jsonb,
    custom_fields jsonb,
    last_contacted_at timestamp without time zone,
    created_at timestamp without time zone DEFAULT now(),
    updated_at timestamp without time zone DEFAULT now()
);


--
-- Name: contacts_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.contacts_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: contacts_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.contacts_id_seq OWNED BY public.contacts.id;


--
-- Name: content_translations; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.content_translations (
    id bigint NOT NULL,
    content_type character varying(50) NOT NULL,
    content_id bigint NOT NULL,
    locale character varying(10) NOT NULL,
    translations jsonb NOT NULL,
    status character varying(20) DEFAULT 'draft'::character varying NOT NULL,
    translated_by bigint,
    reviewed_by bigint,
    machine_translated boolean DEFAULT false NOT NULL,
    quality_score integer,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: TABLE content_translations; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.content_translations IS 'Translated content for i18n support';


--
-- Name: content_translations_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.content_translations_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: content_translations_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.content_translations_id_seq OWNED BY public.content_translations.id;


--
-- Name: content_versions; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.content_versions (
    id bigint NOT NULL,
    content_type character varying(50) NOT NULL,
    content_id bigint NOT NULL,
    version_number integer DEFAULT 1 NOT NULL,
    is_current boolean DEFAULT false NOT NULL,
    data jsonb NOT NULL,
    change_summary character varying(500),
    changed_fields text[],
    created_by bigint,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: TABLE content_versions; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.content_versions IS 'Stores all versions of content for history tracking and rollback';


--
-- Name: content_versions_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.content_versions_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: content_versions_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.content_versions_id_seq OWNED BY public.content_versions.id;


--
-- Name: content_workflow_status; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.content_workflow_status (
    id bigint NOT NULL,
    content_type character varying(50) NOT NULL,
    content_id bigint NOT NULL,
    workflow_id bigint,
    current_stage character varying(50) DEFAULT 'draft'::character varying NOT NULL,
    assigned_to bigint,
    assigned_by bigint,
    assigned_at timestamp with time zone,
    due_date timestamp with time zone,
    priority character varying(20) DEFAULT 'normal'::character varying,
    notes text,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: TABLE content_workflow_status; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.content_workflow_status IS 'Current workflow status for each piece of content';


--
-- Name: content_workflow_status_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.content_workflow_status_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: content_workflow_status_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.content_workflow_status_id_seq OWNED BY public.content_workflow_status.id;


--
-- Name: coupons; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.coupons (
    id bigint NOT NULL,
    code character varying(50) NOT NULL,
    description text,
    discount_type character varying(20) NOT NULL,
    discount_value numeric(10,2) NOT NULL,
    min_purchase numeric(10,2),
    max_discount numeric(10,2),
    usage_limit integer,
    usage_count integer DEFAULT 0,
    is_active boolean DEFAULT true,
    starts_at timestamp without time zone,
    expires_at timestamp without time zone,
    applicable_products jsonb,
    applicable_plans jsonb,
    created_at timestamp without time zone DEFAULT now(),
    updated_at timestamp without time zone DEFAULT now(),
    stripe_coupon_id text,
    duration text DEFAULT 'once'::text NOT NULL,
    duration_in_months integer,
    CONSTRAINT coupons_duration_in_months_required_for_repeating CHECK ((((duration = 'repeating'::text) AND (duration_in_months IS NOT NULL) AND (duration_in_months > 0)) OR ((duration <> 'repeating'::text) AND (duration_in_months IS NULL)))),
    CONSTRAINT coupons_duration_valid CHECK ((duration = ANY (ARRAY['once'::text, 'forever'::text, 'repeating'::text])))
);


--
-- Name: coupons_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.coupons_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: coupons_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.coupons_id_seq OWNED BY public.coupons.id;


--
-- Name: course_categories; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.course_categories (
    id bigint NOT NULL,
    name character varying(100) NOT NULL,
    slug character varying(100) NOT NULL,
    description text,
    icon character varying(100),
    color character varying(20),
    parent_id bigint,
    sort_order integer DEFAULT 0,
    is_featured boolean DEFAULT false,
    course_count integer DEFAULT 0,
    created_at timestamp without time zone DEFAULT now(),
    updated_at timestamp without time zone DEFAULT now()
);


--
-- Name: course_categories_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.course_categories_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: course_categories_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.course_categories_id_seq OWNED BY public.course_categories.id;


--
-- Name: course_category_mappings; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.course_category_mappings (
    course_id uuid NOT NULL,
    category_id bigint NOT NULL
);


--
-- Name: course_downloads; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.course_downloads (
    id bigint NOT NULL,
    course_id uuid,
    module_id bigint,
    lesson_id uuid,
    title character varying(255) NOT NULL,
    description text,
    file_name character varying(255) NOT NULL,
    file_path character varying(500) NOT NULL,
    file_size_bytes bigint,
    file_type character varying(50),
    mime_type character varying(100),
    bunny_file_id character varying(100),
    storage_zone character varying(100),
    download_url character varying(500),
    preview_url character varying(500),
    category character varying(50) DEFAULT 'resource'::character varying,
    sort_order integer DEFAULT 0,
    is_public boolean DEFAULT false,
    require_enrollment boolean DEFAULT true,
    require_lesson_complete boolean DEFAULT false,
    download_count integer DEFAULT 0,
    uploaded_by bigint,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now()
);


--
-- Name: course_downloads_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.course_downloads_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: course_downloads_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.course_downloads_id_seq OWNED BY public.course_downloads.id;


--
-- Name: course_lessons; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.course_lessons (
    id bigint NOT NULL,
    course_id bigint NOT NULL,
    section_id bigint NOT NULL,
    title character varying(255) NOT NULL,
    description text,
    content_html text,
    video_url character varying(500),
    video_platform character varying(50) DEFAULT 'bunny'::character varying,
    bunny_video_guid character varying(100),
    bunny_library_id bigint,
    duration_seconds integer,
    thumbnail_url character varying(500),
    sort_order integer DEFAULT 0 NOT NULL,
    lesson_type character varying(50) DEFAULT 'video'::character varying,
    is_preview boolean DEFAULT false,
    is_published boolean DEFAULT true,
    completion_type character varying(50) DEFAULT 'video_complete'::character varying,
    required_watch_percent integer DEFAULT 90,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now()
);


--
-- Name: course_lessons_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.course_lessons_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: course_lessons_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.course_lessons_id_seq OWNED BY public.course_lessons.id;


--
-- Name: course_live_sessions; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.course_live_sessions (
    id bigint NOT NULL,
    course_id bigint NOT NULL,
    section_id bigint,
    title character varying(255) NOT NULL,
    description text,
    session_date date NOT NULL,
    session_time time without time zone,
    video_url character varying(500),
    bunny_video_guid character varying(100),
    bunny_library_id bigint,
    duration_seconds integer,
    thumbnail_url character varying(500),
    replay_available_until timestamp with time zone,
    is_published boolean DEFAULT true,
    sort_order integer DEFAULT 0,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now()
);


--
-- Name: course_live_sessions_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.course_live_sessions_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: course_live_sessions_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.course_live_sessions_id_seq OWNED BY public.course_live_sessions.id;


--
-- Name: course_modules; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.course_modules (
    id bigint NOT NULL,
    course_id bigint NOT NULL,
    title character varying(255) NOT NULL,
    description text,
    sort_order integer DEFAULT 0,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now()
);


--
-- Name: course_modules_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.course_modules_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: course_modules_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.course_modules_id_seq OWNED BY public.course_modules.id;


--
-- Name: course_modules_v2; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.course_modules_v2 (
    id bigint NOT NULL,
    course_id uuid NOT NULL,
    title character varying(255) NOT NULL,
    description text,
    sort_order integer DEFAULT 0,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now(),
    drip_enabled boolean DEFAULT false,
    drip_days integer DEFAULT 0
);


--
-- Name: course_modules_v2_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.course_modules_v2_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: course_modules_v2_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.course_modules_v2_id_seq OWNED BY public.course_modules_v2.id;


--
-- Name: course_quizzes; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.course_quizzes (
    id bigint NOT NULL,
    course_id uuid NOT NULL,
    lesson_id uuid,
    module_id bigint,
    title character varying(255) NOT NULL,
    description text,
    quiz_type character varying(20) DEFAULT 'graded'::character varying,
    passing_score integer DEFAULT 70,
    time_limit_minutes integer,
    max_attempts integer,
    shuffle_questions boolean DEFAULT false,
    shuffle_answers boolean DEFAULT false,
    show_correct_answers boolean DEFAULT true,
    is_required boolean DEFAULT false,
    is_published boolean DEFAULT false,
    sort_order integer DEFAULT 0,
    created_at timestamp without time zone DEFAULT now(),
    updated_at timestamp without time zone DEFAULT now()
);


--
-- Name: course_quizzes_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.course_quizzes_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: course_quizzes_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.course_quizzes_id_seq OWNED BY public.course_quizzes.id;


--
-- Name: course_resources; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.course_resources (
    id bigint NOT NULL,
    course_id bigint NOT NULL,
    section_id bigint,
    lesson_id bigint,
    title character varying(255) NOT NULL,
    description text,
    file_url character varying(500) NOT NULL,
    file_name character varying(255) NOT NULL,
    file_type character varying(50),
    file_size_bytes bigint,
    sort_order integer DEFAULT 0,
    version character varying(50) DEFAULT '1.0'::character varying,
    download_count integer DEFAULT 0,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now()
);


--
-- Name: course_resources_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.course_resources_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: course_resources_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.course_resources_id_seq OWNED BY public.course_resources.id;


--
-- Name: course_reviews; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.course_reviews (
    id bigint NOT NULL,
    user_id bigint NOT NULL,
    course_id uuid NOT NULL,
    rating smallint NOT NULL,
    title character varying(255),
    content text,
    is_verified_purchase boolean DEFAULT false,
    is_approved boolean DEFAULT true,
    helpful_count integer DEFAULT 0,
    created_at timestamp without time zone DEFAULT now(),
    updated_at timestamp without time zone DEFAULT now(),
    CONSTRAINT course_reviews_rating_check CHECK (((rating >= 1) AND (rating <= 5)))
);


--
-- Name: course_reviews_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.course_reviews_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: course_reviews_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.course_reviews_id_seq OWNED BY public.course_reviews.id;


--
-- Name: course_sections; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.course_sections (
    id bigint NOT NULL,
    course_id bigint NOT NULL,
    title character varying(255) NOT NULL,
    description text,
    sort_order integer DEFAULT 0 NOT NULL,
    section_type character varying(50) DEFAULT 'module'::character varying,
    unlock_type character varying(50) DEFAULT 'sequential'::character varying,
    unlock_after_section_id bigint,
    unlock_date timestamp with time zone,
    is_published boolean DEFAULT true,
    estimated_duration_minutes integer DEFAULT 0,
    lesson_count integer DEFAULT 0,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now()
);


--
-- Name: course_sections_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.course_sections_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: course_sections_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.course_sections_id_seq OWNED BY public.course_sections.id;


--
-- Name: course_tag_mappings; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.course_tag_mappings (
    course_id uuid NOT NULL,
    tag_id bigint NOT NULL
);


--
-- Name: course_tags; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.course_tags (
    id bigint NOT NULL,
    name character varying(50) NOT NULL,
    slug character varying(50) NOT NULL,
    color character varying(20),
    course_count integer DEFAULT 0,
    created_at timestamp without time zone DEFAULT now()
);


--
-- Name: course_tags_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.course_tags_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: course_tags_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.course_tags_id_seq OWNED BY public.course_tags.id;


--
-- Name: courses; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.courses (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    title character varying(255) NOT NULL,
    slug character varying(255) NOT NULL,
    description text,
    price_cents bigint DEFAULT 0 NOT NULL,
    instructor_id bigint,
    is_published boolean DEFAULT false,
    thumbnail character varying(500),
    preview_video_url character varying(500),
    duration_minutes integer DEFAULT 0,
    level character varying(50) DEFAULT 'beginner'::character varying,
    metadata jsonb,
    created_at timestamp without time zone DEFAULT now(),
    updated_at timestamp without time zone DEFAULT now(),
    card_description text,
    card_image_url character varying(500),
    card_badge character varying(100),
    card_badge_color character varying(20),
    is_free boolean DEFAULT false,
    status character varying(50) DEFAULT 'draft'::character varying,
    instructor_name character varying(255),
    instructor_title character varying(255),
    instructor_avatar_url character varying(500),
    instructor_bio text,
    what_you_learn jsonb DEFAULT '[]'::jsonb,
    requirements jsonb DEFAULT '[]'::jsonb,
    tags jsonb DEFAULT '[]'::jsonb,
    module_count integer DEFAULT 0,
    lesson_count integer DEFAULT 0,
    total_duration_minutes integer DEFAULT 0,
    enrollment_count integer DEFAULT 0,
    avg_rating numeric(3,2) DEFAULT 0,
    review_count integer DEFAULT 0,
    bunny_library_id character varying(100),
    seo_title character varying(255),
    seo_description text,
    seo_keywords text,
    stripe_price_id text,
    stripe_product_id text
);


--
-- Name: COLUMN courses.stripe_price_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.courses.stripe_price_id IS 'Current active Stripe Price ID for this course (one-time purchase)';


--
-- Name: COLUMN courses.stripe_product_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.courses.stripe_product_id IS 'Stripe Product ID for this course';


--
-- Name: courses_enhanced; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.courses_enhanced (
    id bigint NOT NULL,
    title character varying(255) NOT NULL,
    slug character varying(255) NOT NULL,
    subtitle character varying(500),
    description text,
    description_html text,
    thumbnail_url character varying(500),
    trailer_video_url character varying(500),
    trailer_bunny_guid character varying(100),
    difficulty_level character varying(50) DEFAULT 'beginner'::character varying,
    category character varying(100),
    tags jsonb DEFAULT '[]'::jsonb,
    instructor_id bigint,
    estimated_duration_minutes integer DEFAULT 0,
    total_lessons integer DEFAULT 0,
    total_sections integer DEFAULT 0,
    is_published boolean DEFAULT false,
    is_featured boolean DEFAULT false,
    is_free boolean DEFAULT false,
    required_plan_id bigint,
    price_cents bigint,
    prerequisite_course_ids jsonb DEFAULT '[]'::jsonb,
    certificate_enabled boolean DEFAULT true,
    certificate_template character varying(100) DEFAULT 'default'::character varying,
    completion_threshold_percent integer DEFAULT 90,
    meta_title character varying(255),
    meta_description text,
    created_by bigint,
    updated_by bigint,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now(),
    published_at timestamp with time zone,
    deleted_at timestamp with time zone
);


--
-- Name: courses_enhanced_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.courses_enhanced_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: courses_enhanced_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.courses_enhanced_id_seq OWNED BY public.courses_enhanced.id;


--
-- Name: crm_abandoned_carts; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.crm_abandoned_carts (
    id bigint NOT NULL,
    contact_id bigint,
    email character varying(255) NOT NULL,
    cart_total numeric(15,2) DEFAULT 0.00,
    cart_items jsonb DEFAULT '[]'::jsonb,
    status character varying(20) DEFAULT 'abandoned'::character varying,
    recovery_email_sent boolean DEFAULT false,
    recovered boolean DEFAULT false,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now(),
    CONSTRAINT crm_abandoned_carts_status_check CHECK (((status)::text = ANY ((ARRAY['abandoned'::character varying, 'recovered'::character varying, 'expired'::character varying])::text[])))
);


--
-- Name: crm_abandoned_carts_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.crm_abandoned_carts_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: crm_abandoned_carts_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.crm_abandoned_carts_id_seq OWNED BY public.crm_abandoned_carts.id;


--
-- Name: crm_automations; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.crm_automations (
    id bigint NOT NULL,
    name character varying(255) NOT NULL,
    trigger_type character varying(100) NOT NULL,
    trigger_config jsonb DEFAULT '{}'::jsonb,
    actions jsonb DEFAULT '[]'::jsonb,
    is_active boolean DEFAULT true,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now(),
    title character varying(255),
    status character varying(20) DEFAULT 'draft'::character varying,
    trigger_name character varying(255),
    actions_count integer DEFAULT 0,
    subscribers_count integer DEFAULT 0,
    conversion_rate numeric(5,2) DEFAULT 0.00
);


--
-- Name: crm_automations_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.crm_automations_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: crm_automations_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.crm_automations_id_seq OWNED BY public.crm_automations.id;


--
-- Name: crm_campaigns; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.crm_campaigns (
    id bigint NOT NULL,
    name character varying(255) NOT NULL,
    subject character varying(500),
    content text,
    status character varying(50) DEFAULT 'draft'::character varying,
    scheduled_at timestamp with time zone,
    sent_at timestamp with time zone,
    stats jsonb DEFAULT '{}'::jsonb,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now(),
    title character varying(255),
    recipients_count integer DEFAULT 0,
    emails_sent integer DEFAULT 0,
    opens integer DEFAULT 0,
    clicks integer DEFAULT 0,
    open_rate numeric(5,2) DEFAULT 0.00,
    click_rate numeric(5,2) DEFAULT 0.00,
    template_id bigint
);


--
-- Name: crm_campaigns_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.crm_campaigns_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: crm_campaigns_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.crm_campaigns_id_seq OWNED BY public.crm_campaigns.id;


--
-- Name: crm_companies; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.crm_companies (
    id bigint NOT NULL,
    name character varying(255) NOT NULL,
    domain character varying(255),
    industry character varying(100),
    size character varying(50),
    website character varying(500),
    notes text,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now(),
    logo_url character varying(500),
    contacts_count integer DEFAULT 0,
    deals_count integer DEFAULT 0,
    total_deal_value numeric(15,2) DEFAULT 0.00
);


--
-- Name: crm_companies_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.crm_companies_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: crm_companies_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.crm_companies_id_seq OWNED BY public.crm_companies.id;


--
-- Name: crm_deal_activities; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.crm_deal_activities (
    id bigint NOT NULL,
    deal_id bigint NOT NULL,
    activity_type character varying(50) NOT NULL,
    title character varying(255),
    description text,
    due_date timestamp with time zone,
    completed_at timestamp with time zone,
    created_by bigint,
    assigned_to bigint,
    priority character varying(20) DEFAULT 'medium'::character varying,
    metadata jsonb DEFAULT '{}'::jsonb,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now()
);


--
-- Name: crm_deal_activities_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.crm_deal_activities_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: crm_deal_activities_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.crm_deal_activities_id_seq OWNED BY public.crm_deal_activities.id;


--
-- Name: crm_deal_stage_history; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.crm_deal_stage_history (
    id bigint NOT NULL,
    deal_id bigint NOT NULL,
    from_stage_id bigint,
    to_stage_id bigint NOT NULL,
    changed_by bigint,
    reason text,
    duration_in_stage integer,
    created_at timestamp with time zone DEFAULT now()
);


--
-- Name: crm_deal_stage_history_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.crm_deal_stage_history_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: crm_deal_stage_history_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.crm_deal_stage_history_id_seq OWNED BY public.crm_deal_stage_history.id;


--
-- Name: crm_deals; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.crm_deals (
    id bigint NOT NULL,
    name character varying(255) NOT NULL,
    contact_id bigint,
    company_id bigint,
    pipeline_id bigint NOT NULL,
    stage_id bigint NOT NULL,
    owner_id bigint,
    amount numeric(15,2) DEFAULT 0.00,
    currency character varying(3) DEFAULT 'USD'::character varying,
    probability integer DEFAULT 50,
    status character varying(20) DEFAULT 'open'::character varying,
    expected_close_date date,
    close_date date,
    lost_reason text,
    won_details text,
    priority character varying(20) DEFAULT 'medium'::character varying,
    source_channel character varying(100),
    source_campaign character varying(255),
    tags jsonb DEFAULT '[]'::jsonb,
    custom_fields jsonb DEFAULT '{}'::jsonb,
    stage_entered_at timestamp with time zone DEFAULT now(),
    closed_at timestamp with time zone,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now(),
    CONSTRAINT crm_deals_priority_check CHECK (((priority)::text = ANY ((ARRAY['low'::character varying, 'medium'::character varying, 'high'::character varying, 'urgent'::character varying])::text[]))),
    CONSTRAINT crm_deals_probability_check CHECK (((probability >= 0) AND (probability <= 100))),
    CONSTRAINT crm_deals_status_check CHECK (((status)::text = ANY ((ARRAY['open'::character varying, 'won'::character varying, 'lost'::character varying])::text[])))
);


--
-- Name: crm_deals_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.crm_deals_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: crm_deals_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.crm_deals_id_seq OWNED BY public.crm_deals.id;


--
-- Name: crm_lists; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.crm_lists (
    id bigint NOT NULL,
    name character varying(255) NOT NULL,
    description text,
    is_active boolean DEFAULT true,
    subscriber_count integer DEFAULT 0,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now(),
    title character varying(255),
    slug character varying(255),
    is_public boolean DEFAULT false,
    contacts_count integer DEFAULT 0
);


--
-- Name: crm_lists_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.crm_lists_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: crm_lists_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.crm_lists_id_seq OWNED BY public.crm_lists.id;


--
-- Name: crm_pipeline_stages; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.crm_pipeline_stages (
    id bigint NOT NULL,
    pipeline_id bigint NOT NULL,
    name character varying(255) NOT NULL,
    description text,
    "position" integer DEFAULT 0,
    probability integer DEFAULT 50,
    is_closed_won boolean DEFAULT false,
    is_closed_lost boolean DEFAULT false,
    auto_advance_after_days integer,
    required_activities jsonb DEFAULT '[]'::jsonb,
    color character varying(20) DEFAULT '#6366f1'::character varying,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now(),
    CONSTRAINT crm_pipeline_stages_probability_check CHECK (((probability >= 0) AND (probability <= 100)))
);


--
-- Name: crm_pipeline_stages_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.crm_pipeline_stages_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: crm_pipeline_stages_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.crm_pipeline_stages_id_seq OWNED BY public.crm_pipeline_stages.id;


--
-- Name: crm_pipelines; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.crm_pipelines (
    id bigint NOT NULL,
    name character varying(255) NOT NULL,
    description text,
    is_default boolean DEFAULT false,
    is_active boolean DEFAULT true,
    color character varying(20) DEFAULT '#6366f1'::character varying,
    icon character varying(50),
    "position" integer DEFAULT 0,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now()
);


--
-- Name: crm_pipelines_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.crm_pipelines_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: crm_pipelines_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.crm_pipelines_id_seq OWNED BY public.crm_pipelines.id;


--
-- Name: crm_recurring_campaigns; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.crm_recurring_campaigns (
    id bigint NOT NULL,
    campaign_id bigint,
    schedule jsonb NOT NULL,
    next_run_at timestamp with time zone,
    is_active boolean DEFAULT true,
    created_at timestamp with time zone DEFAULT now(),
    title character varying(255),
    status character varying(20) DEFAULT 'active'::character varying,
    scheduling_settings jsonb,
    total_campaigns_sent integer DEFAULT 0,
    total_emails_sent integer DEFAULT 0,
    total_revenue numeric(15,2) DEFAULT 0.00,
    last_sent_at timestamp with time zone,
    next_scheduled_at timestamp with time zone,
    updated_at timestamp with time zone DEFAULT now()
);


--
-- Name: crm_recurring_campaigns_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.crm_recurring_campaigns_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: crm_recurring_campaigns_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.crm_recurring_campaigns_id_seq OWNED BY public.crm_recurring_campaigns.id;


--
-- Name: crm_segments; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.crm_segments (
    id bigint NOT NULL,
    name character varying(255) NOT NULL,
    description text,
    conditions jsonb DEFAULT '[]'::jsonb,
    is_active boolean DEFAULT true,
    member_count integer DEFAULT 0,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now(),
    title character varying(255),
    slug character varying(255),
    match_type character varying(20) DEFAULT 'all'::character varying,
    contacts_count integer DEFAULT 0,
    last_synced_at timestamp with time zone
);


--
-- Name: crm_segments_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.crm_segments_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: crm_segments_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.crm_segments_id_seq OWNED BY public.crm_segments.id;


--
-- Name: crm_sequences; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.crm_sequences (
    id bigint NOT NULL,
    name character varying(255) NOT NULL,
    description text,
    steps jsonb DEFAULT '[]'::jsonb,
    is_active boolean DEFAULT true,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now(),
    title character varying(255),
    status character varying(20) DEFAULT 'draft'::character varying,
    trigger_type character varying(50),
    email_count integer DEFAULT 0,
    total_subscribers integer DEFAULT 0,
    emails_sent integer DEFAULT 0,
    open_rate numeric(5,2) DEFAULT 0.00,
    click_rate numeric(5,2) DEFAULT 0.00
);


--
-- Name: crm_sequences_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.crm_sequences_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: crm_sequences_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.crm_sequences_id_seq OWNED BY public.crm_sequences.id;


--
-- Name: crm_smart_links; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.crm_smart_links (
    id bigint NOT NULL,
    name character varying(255) NOT NULL,
    url character varying(500) NOT NULL,
    short_code character varying(50),
    click_count integer DEFAULT 0,
    created_at timestamp with time zone DEFAULT now(),
    title character varying(255),
    short character varying(50),
    target_url character varying(500),
    is_active boolean DEFAULT true,
    unique_clicks integer DEFAULT 0,
    actions jsonb DEFAULT '[]'::jsonb,
    updated_at timestamp with time zone DEFAULT now()
);


--
-- Name: crm_smart_links_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.crm_smart_links_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: crm_smart_links_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.crm_smart_links_id_seq OWNED BY public.crm_smart_links.id;


--
-- Name: crm_tags; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.crm_tags (
    id bigint NOT NULL,
    name character varying(100) NOT NULL,
    color character varying(7) DEFAULT '#6366f1'::character varying,
    description text,
    created_at timestamp with time zone DEFAULT now(),
    title character varying(255),
    slug character varying(255),
    contacts_count integer DEFAULT 0,
    updated_at timestamp with time zone DEFAULT now()
);


--
-- Name: crm_tags_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.crm_tags_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: crm_tags_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.crm_tags_id_seq OWNED BY public.crm_tags.id;


--
-- Name: crm_templates; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.crm_templates (
    id bigint NOT NULL,
    name character varying(255) NOT NULL,
    subject character varying(500),
    content text NOT NULL,
    category character varying(100),
    is_active boolean DEFAULT true,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now(),
    title character varying(255),
    body text,
    thumbnail character varying(500),
    is_global boolean DEFAULT false
);


--
-- Name: crm_templates_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.crm_templates_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: crm_templates_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.crm_templates_id_seq OWNED BY public.crm_templates.id;


--
-- Name: crm_webhooks; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.crm_webhooks (
    id bigint NOT NULL,
    name character varying(255) NOT NULL,
    url character varying(500) NOT NULL,
    events jsonb DEFAULT '[]'::jsonb,
    secret character varying(255),
    is_active boolean DEFAULT true,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now(),
    trigger_count integer DEFAULT 0,
    failure_count integer DEFAULT 0,
    last_triggered_at timestamp with time zone
);


--
-- Name: crm_webhooks_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.crm_webhooks_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: crm_webhooks_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.crm_webhooks_id_seq OWNED BY public.crm_webhooks.id;


--
-- Name: departments; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.departments (
    id bigint NOT NULL,
    name character varying(255) NOT NULL,
    slug character varying(255) NOT NULL,
    description text,
    parent_id bigint,
    color character varying(7) DEFAULT '#8b5cf6'::character varying,
    icon character varying(50),
    is_active boolean DEFAULT true,
    member_count integer DEFAULT 0,
    created_by bigint,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now()
);


--
-- Name: departments_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.departments_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: departments_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.departments_id_seq OWNED BY public.departments.id;


--
-- Name: email_campaigns; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.email_campaigns (
    id bigint NOT NULL,
    name character varying(255) NOT NULL,
    subject character varying(255) NOT NULL,
    template_id bigint,
    html_content text,
    text_content text,
    status character varying(20) DEFAULT 'draft'::character varying,
    segment_id bigint,
    recipient_count integer DEFAULT 0,
    sent_count integer DEFAULT 0,
    open_count integer DEFAULT 0,
    click_count integer DEFAULT 0,
    bounce_count integer DEFAULT 0,
    unsubscribe_count integer DEFAULT 0,
    scheduled_at timestamp without time zone,
    sent_at timestamp without time zone,
    created_at timestamp without time zone DEFAULT now(),
    updated_at timestamp without time zone DEFAULT now()
);


--
-- Name: email_campaigns_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.email_campaigns_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: email_campaigns_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.email_campaigns_id_seq OWNED BY public.email_campaigns.id;


--
-- Name: email_logs; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.email_logs (
    id bigint NOT NULL,
    campaign_id bigint,
    subscriber_id bigint,
    user_id bigint,
    email character varying(255) NOT NULL,
    subject character varying(255) NOT NULL,
    template_type character varying(50),
    status character varying(20) NOT NULL,
    provider character varying(50),
    provider_message_id character varying(255),
    opened_at timestamp without time zone,
    clicked_at timestamp without time zone,
    bounced_at timestamp without time zone,
    error_message text,
    metadata jsonb,
    created_at timestamp without time zone DEFAULT now(),
    to_email character varying(255),
    template_alias character varying(100),
    model jsonb,
    error text,
    queued_at timestamp with time zone,
    sent_at timestamp with time zone
);


--
-- Name: email_logs_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.email_logs_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: email_logs_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.email_logs_id_seq OWNED BY public.email_logs.id;


--
-- Name: email_templates; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.email_templates (
    id bigint NOT NULL,
    name character varying(255) NOT NULL,
    slug character varying(255) NOT NULL,
    subject character varying(255) NOT NULL,
    html_content text NOT NULL,
    text_content text,
    template_type character varying(50) DEFAULT 'transactional'::character varying,
    variables jsonb,
    is_active boolean DEFAULT true,
    created_at timestamp without time zone DEFAULT now(),
    updated_at timestamp without time zone DEFAULT now()
);


--
-- Name: email_templates_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.email_templates_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: email_templates_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.email_templates_id_seq OWNED BY public.email_templates.id;


--
-- Name: email_verification_tokens; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.email_verification_tokens (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    user_id bigint NOT NULL,
    token character varying(64) NOT NULL,
    expires_at timestamp with time zone NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: failed_jobs; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.failed_jobs (
    id bigint NOT NULL,
    queue character varying(255) NOT NULL,
    payload jsonb NOT NULL,
    exception text,
    failed_at timestamp without time zone DEFAULT now()
);


--
-- Name: failed_jobs_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.failed_jobs_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: failed_jobs_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.failed_jobs_id_seq OWNED BY public.failed_jobs.id;


--
-- Name: form_submissions; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.form_submissions (
    id bigint NOT NULL,
    form_id bigint NOT NULL,
    user_id bigint,
    data jsonb NOT NULL,
    ip_address inet,
    user_agent text,
    created_at timestamp with time zone DEFAULT now(),
    status character varying(20) DEFAULT 'unread'::character varying
);


--
-- Name: COLUMN form_submissions.status; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.form_submissions.status IS 'Submission status: unread, read, starred, archived, spam, complete, partial';


--
-- Name: form_submissions_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.form_submissions_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: form_submissions_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.form_submissions_id_seq OWNED BY public.form_submissions.id;


--
-- Name: forms; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.forms (
    id bigint NOT NULL,
    name character varying(255) NOT NULL,
    slug character varying(255) NOT NULL,
    description text,
    fields jsonb DEFAULT '[]'::jsonb NOT NULL,
    settings jsonb DEFAULT '{}'::jsonb,
    is_active boolean DEFAULT true,
    submission_count integer DEFAULT 0,
    created_by bigint,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now(),
    is_published boolean DEFAULT false
);


--
-- Name: forms_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.forms_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: forms_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.forms_id_seq OWNED BY public.forms.id;


--
-- Name: indicator_documentation; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.indicator_documentation (
    id bigint NOT NULL,
    indicator_id bigint NOT NULL,
    title character varying(255) NOT NULL,
    description text,
    content_type character varying(50) DEFAULT 'file'::character varying,
    content_html text,
    file_url character varying(500),
    external_url character varying(500),
    doc_type character varying(50) DEFAULT 'guide'::character varying,
    sort_order integer DEFAULT 0,
    view_count integer DEFAULT 0,
    is_published boolean DEFAULT true,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now()
);


--
-- Name: indicator_documentation_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.indicator_documentation_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: indicator_documentation_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.indicator_documentation_id_seq OWNED BY public.indicator_documentation.id;


--
-- Name: indicator_download_log; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.indicator_download_log (
    id bigint NOT NULL,
    indicator_id bigint NOT NULL,
    file_id bigint,
    user_id bigint,
    platform_slug character varying(100),
    file_name character varying(255),
    ip_address inet,
    user_agent text,
    downloaded_at timestamp with time zone DEFAULT now()
);


--
-- Name: indicator_download_log_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.indicator_download_log_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: indicator_download_log_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.indicator_download_log_id_seq OWNED BY public.indicator_download_log.id;


--
-- Name: indicator_downloads; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.indicator_downloads (
    id bigint NOT NULL,
    indicator_id bigint,
    user_id bigint,
    file_id bigint,
    downloaded_at timestamp with time zone DEFAULT now()
);


--
-- Name: indicator_downloads_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.indicator_downloads_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: indicator_downloads_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.indicator_downloads_id_seq OWNED BY public.indicator_downloads.id;


--
-- Name: indicator_files; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.indicator_files (
    id bigint NOT NULL,
    indicator_id bigint,
    platform character varying(50),
    file_url character varying(500),
    file_name character varying(255),
    version character varying(50),
    created_at timestamp with time zone DEFAULT now()
);


--
-- Name: indicator_files_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.indicator_files_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: indicator_files_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.indicator_files_id_seq OWNED BY public.indicator_files.id;


--
-- Name: indicator_platform_files; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.indicator_platform_files (
    id bigint NOT NULL,
    indicator_id bigint NOT NULL,
    platform_slug character varying(100) NOT NULL,
    file_name character varying(255) NOT NULL,
    file_display_name character varying(255),
    file_url character varying(500) NOT NULL,
    original_filename character varying(255),
    file_size_bytes bigint,
    file_type character varying(50),
    version character varying(50) DEFAULT '1.0'::character varying,
    installation_notes text,
    sort_order integer DEFAULT 0,
    download_count integer DEFAULT 0,
    is_active boolean DEFAULT true,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now()
);


--
-- Name: indicator_platform_files_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.indicator_platform_files_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: indicator_platform_files_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.indicator_platform_files_id_seq OWNED BY public.indicator_platform_files.id;


--
-- Name: indicator_platforms; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.indicator_platforms (
    id integer NOT NULL,
    name character varying(100) NOT NULL,
    slug character varying(100) NOT NULL,
    icon_url character varying(500),
    file_extensions jsonb DEFAULT '[]'::jsonb,
    installation_guide_url character varying(500),
    sort_order integer DEFAULT 0,
    is_active boolean DEFAULT true
);


--
-- Name: indicator_platforms_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.indicator_platforms_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: indicator_platforms_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.indicator_platforms_id_seq OWNED BY public.indicator_platforms.id;


--
-- Name: indicator_tradingview_access; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.indicator_tradingview_access (
    id bigint NOT NULL,
    indicator_id bigint NOT NULL,
    user_id bigint,
    tradingview_username character varying(255) NOT NULL,
    status character varying(50) DEFAULT 'pending'::character varying,
    granted_at timestamp with time zone,
    revoked_at timestamp with time zone,
    revoke_reason text,
    notification_sent boolean DEFAULT false,
    notification_sent_at timestamp with time zone,
    requested_at timestamp with time zone DEFAULT now(),
    requested_by bigint,
    admin_notes text
);


--
-- Name: indicator_tradingview_access_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.indicator_tradingview_access_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: indicator_tradingview_access_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.indicator_tradingview_access_id_seq OWNED BY public.indicator_tradingview_access.id;


--
-- Name: indicator_videos; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.indicator_videos (
    id bigint NOT NULL,
    indicator_id bigint NOT NULL,
    title character varying(255) NOT NULL,
    description text,
    video_url character varying(500),
    bunny_video_guid character varying(100),
    bunny_library_id bigint,
    duration_seconds integer,
    thumbnail_url character varying(500),
    video_type character varying(50) DEFAULT 'tutorial'::character varying,
    sort_order integer DEFAULT 0,
    is_published boolean DEFAULT true,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now()
);


--
-- Name: indicator_videos_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.indicator_videos_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: indicator_videos_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.indicator_videos_id_seq OWNED BY public.indicator_videos.id;


--
-- Name: indicators; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.indicators (
    id bigint NOT NULL,
    name character varying(255) NOT NULL,
    slug character varying(255) NOT NULL,
    description text,
    long_description text,
    is_active boolean DEFAULT true,
    platform character varying(50) NOT NULL,
    version character varying(20) DEFAULT '1.0.0'::character varying,
    download_url character varying(500),
    documentation_url character varying(500),
    thumbnail character varying(500),
    screenshots jsonb,
    features jsonb,
    requirements jsonb,
    meta_title character varying(255),
    meta_description text,
    created_at timestamp without time zone DEFAULT now(),
    updated_at timestamp without time zone DEFAULT now(),
    tagline character varying(500),
    price_cents bigint DEFAULT 0,
    is_free boolean DEFAULT false,
    sale_price_cents bigint,
    logo_url character varying(500),
    card_image_url character varying(500),
    status character varying(50) DEFAULT 'draft'::character varying,
    is_published boolean DEFAULT false,
    view_count integer DEFAULT 0,
    download_count integer DEFAULT 0,
    supported_platforms jsonb DEFAULT '[]'::jsonb,
    stripe_price_id text,
    stripe_product_id text
);


--
-- Name: COLUMN indicators.stripe_price_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.indicators.stripe_price_id IS 'Current active Stripe Price ID for this indicator (one-time purchase)';


--
-- Name: COLUMN indicators.stripe_product_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.indicators.stripe_product_id IS 'Stripe Product ID for this indicator';


--
-- Name: indicators_enhanced; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.indicators_enhanced (
    id bigint NOT NULL,
    title character varying(255) NOT NULL,
    slug character varying(255) NOT NULL,
    subtitle character varying(500),
    description text,
    description_html text,
    thumbnail_url character varying(500),
    banner_url character varying(500),
    category character varying(100),
    tags jsonb DEFAULT '[]'::jsonb,
    creator_id bigint,
    current_version character varying(50) DEFAULT '1.0.0'::character varying,
    changelog text,
    supported_platforms jsonb DEFAULT '[]'::jsonb,
    is_published boolean DEFAULT false,
    is_featured boolean DEFAULT false,
    is_free boolean DEFAULT false,
    required_plan_id bigint,
    price_cents bigint,
    download_count integer DEFAULT 0,
    view_count integer DEFAULT 0,
    meta_title character varying(255),
    meta_description text,
    created_by bigint,
    updated_by bigint,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now(),
    published_at timestamp with time zone,
    deleted_at timestamp with time zone
);


--
-- Name: indicators_enhanced_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.indicators_enhanced_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: indicators_enhanced_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.indicators_enhanced_id_seq OWNED BY public.indicators_enhanced.id;


--
-- Name: indicators_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.indicators_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: indicators_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.indicators_id_seq OWNED BY public.indicators.id;


--
-- Name: integration_webhooks; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.integration_webhooks (
    id bigint NOT NULL,
    connection_id bigint NOT NULL,
    name text NOT NULL,
    url text NOT NULL,
    secret text,
    events jsonb,
    is_active boolean DEFAULT true NOT NULL,
    last_triggered_at timestamp with time zone,
    last_status_code integer,
    failure_count integer DEFAULT 0 NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: integration_webhooks_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.integration_webhooks_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: integration_webhooks_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.integration_webhooks_id_seq OWNED BY public.integration_webhooks.id;


--
-- Name: invoices; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.invoices (
    id bigint NOT NULL,
    user_id bigint NOT NULL,
    subscription_id bigint,
    order_id bigint,
    invoice_number character varying(50) NOT NULL,
    status character varying(20) DEFAULT 'pending'::character varying,
    subtotal numeric(10,2) NOT NULL,
    discount numeric(10,2) DEFAULT 0,
    tax numeric(10,2) DEFAULT 0,
    total numeric(10,2) NOT NULL,
    currency character varying(3) DEFAULT 'USD'::character varying,
    stripe_invoice_id character varying(255),
    pdf_url character varying(500),
    due_date timestamp without time zone,
    paid_at timestamp without time zone,
    created_at timestamp without time zone DEFAULT now(),
    updated_at timestamp without time zone DEFAULT now()
);


--
-- Name: invoices_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.invoices_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: invoices_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.invoices_id_seq OWNED BY public.invoices.id;


--
-- Name: jobs; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.jobs (
    id uuid NOT NULL,
    queue character varying(100) DEFAULT 'default'::character varying NOT NULL,
    job_type character varying(100) NOT NULL,
    payload jsonb DEFAULT '{}'::jsonb NOT NULL,
    status character varying(50) DEFAULT 'pending'::character varying NOT NULL,
    attempts integer DEFAULT 0 NOT NULL,
    max_attempts integer DEFAULT 3 NOT NULL,
    error text,
    run_at timestamp with time zone DEFAULT now() NOT NULL,
    started_at timestamp with time zone,
    completed_at timestamp with time zone,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    available_at timestamp with time zone DEFAULT now() NOT NULL,
    reserved_at timestamp with time zone
);


--
-- Name: laravel_jobs_backup; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.laravel_jobs_backup (
    id bigint NOT NULL,
    queue character varying(255) DEFAULT 'default'::character varying NOT NULL,
    job_type character varying(100) DEFAULT 'default'::character varying NOT NULL,
    payload jsonb DEFAULT '{}'::jsonb NOT NULL,
    status character varying(50) DEFAULT 'pending'::character varying NOT NULL,
    attempts integer DEFAULT 0,
    max_attempts integer DEFAULT 3,
    error text,
    available_at timestamp without time zone DEFAULT now() NOT NULL,
    reserved_at timestamp without time zone,
    started_at timestamp without time zone,
    completed_at timestamp without time zone,
    created_at timestamp without time zone DEFAULT now()
);


--
-- Name: jobs_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.jobs_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: jobs_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.jobs_id_seq OWNED BY public.laravel_jobs_backup.id;


--
-- Name: lessons; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.lessons (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    course_id uuid NOT NULL,
    title character varying(255) NOT NULL,
    slug character varying(255) NOT NULL,
    description text,
    video_url character varying(500),
    duration_minutes integer DEFAULT 0,
    "position" integer DEFAULT 0 NOT NULL,
    is_free boolean DEFAULT false,
    content text,
    created_at timestamp without time zone DEFAULT now(),
    updated_at timestamp without time zone DEFAULT now(),
    module_id bigint,
    bunny_video_guid character varying(100),
    thumbnail_url character varying(500),
    content_html text,
    is_preview boolean DEFAULT false,
    is_published boolean DEFAULT true,
    sort_order integer DEFAULT 0,
    drip_days integer DEFAULT 0,
    prerequisite_lesson_ids jsonb
);


--
-- Name: locales; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.locales (
    id bigint NOT NULL,
    code character varying(10) NOT NULL,
    name character varying(100) NOT NULL,
    native_name character varying(100),
    is_default boolean DEFAULT false NOT NULL,
    is_active boolean DEFAULT true NOT NULL,
    rtl boolean DEFAULT false NOT NULL,
    fallback_locale character varying(10),
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    flag_emoji character varying(10),
    direction character varying(3) DEFAULT 'ltr'::character varying,
    date_format character varying(50) DEFAULT 'YYYY-MM-DD'::character varying,
    time_format character varying(50) DEFAULT 'HH:mm:ss'::character varying,
    currency_code character varying(3) DEFAULT 'USD'::character varying
);


--
-- Name: TABLE locales; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.locales IS 'Supported languages/locales';


--
-- Name: locales_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.locales_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: locales_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.locales_id_seq OWNED BY public.locales.id;


--
-- Name: media; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.media (
    id bigint NOT NULL,
    filename character varying(255) NOT NULL,
    original_filename character varying(255),
    mime_type character varying(100),
    size bigint,
    path text,
    url text,
    title character varying(500),
    alt_text character varying(500),
    caption text,
    description text,
    collection character varying(100),
    is_optimized boolean DEFAULT false,
    width integer,
    height integer,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now()
);


--
-- Name: media_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.media_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: media_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.media_id_seq OWNED BY public.media.id;


--
-- Name: member_audit_logs; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.member_audit_logs (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    user_id bigint,
    actor_id bigint,
    action character varying(100) NOT NULL,
    entity_type character varying(50) DEFAULT 'member'::character varying NOT NULL,
    entity_id bigint,
    old_values jsonb,
    new_values jsonb,
    metadata jsonb DEFAULT '{}'::jsonb,
    ip_address inet,
    user_agent text,
    created_at timestamp with time zone DEFAULT now()
);


--
-- Name: TABLE member_audit_logs; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.member_audit_logs IS 'ICT7: Complete audit trail for member system';


--
-- Name: member_emails; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.member_emails (
    id bigint NOT NULL,
    user_id bigint NOT NULL,
    subject character varying(500) NOT NULL,
    body text NOT NULL,
    template_id bigint,
    status character varying(50) DEFAULT 'sent'::character varying,
    sent_by bigint,
    sent_at timestamp with time zone DEFAULT now(),
    opened_at timestamp with time zone,
    clicked_at timestamp with time zone
);


--
-- Name: member_emails_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.member_emails_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: member_emails_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.member_emails_id_seq OWNED BY public.member_emails.id;


--
-- Name: member_filters; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.member_filters (
    id bigint NOT NULL,
    name character varying(255) NOT NULL,
    slug character varying(255) NOT NULL,
    description text,
    filter_config jsonb DEFAULT '{}'::jsonb NOT NULL,
    is_public boolean DEFAULT false,
    is_active boolean DEFAULT true,
    usage_count integer DEFAULT 0,
    created_by bigint,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now()
);


--
-- Name: member_filters_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.member_filters_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: member_filters_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.member_filters_id_seq OWNED BY public.member_filters.id;


--
-- Name: member_notes; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.member_notes (
    id bigint NOT NULL,
    user_id bigint NOT NULL,
    note text NOT NULL,
    note_type character varying(50) DEFAULT 'general'::character varying,
    is_pinned boolean DEFAULT false,
    created_by bigint,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now()
);


--
-- Name: member_notes_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.member_notes_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: member_notes_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.member_notes_id_seq OWNED BY public.member_notes.id;


--
-- Name: member_segments; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.member_segments (
    id bigint NOT NULL,
    name character varying(255) NOT NULL,
    slug character varying(255) NOT NULL,
    description text,
    color character varying(7) DEFAULT '#6366f1'::character varying,
    icon character varying(50),
    filter_criteria jsonb DEFAULT '{}'::jsonb,
    is_dynamic boolean DEFAULT false,
    is_active boolean DEFAULT true,
    member_count integer DEFAULT 0,
    created_by bigint,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now()
);


--
-- Name: member_segments_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.member_segments_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: member_segments_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.member_segments_id_seq OWNED BY public.member_segments.id;


--
-- Name: member_tags; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.member_tags (
    id bigint NOT NULL,
    name character varying(100) NOT NULL,
    slug character varying(100) NOT NULL,
    color character varying(7) DEFAULT '#10b981'::character varying,
    description text,
    is_active boolean DEFAULT true,
    usage_count integer DEFAULT 0,
    created_by bigint,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now()
);


--
-- Name: member_tags_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.member_tags_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: member_tags_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.member_tags_id_seq OWNED BY public.member_tags.id;


--
-- Name: membership_features; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.membership_features (
    id bigint NOT NULL,
    plan_id bigint NOT NULL,
    feature_code character varying(100) NOT NULL,
    feature_name character varying(255) NOT NULL,
    description text,
    created_at timestamp without time zone DEFAULT now()
);


--
-- Name: membership_features_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.membership_features_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: membership_features_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.membership_features_id_seq OWNED BY public.membership_features.id;


--
-- Name: membership_plan_price_history; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.membership_plan_price_history (
    id bigint NOT NULL,
    plan_id bigint NOT NULL,
    old_stripe_price_id text,
    new_stripe_price_id text NOT NULL,
    old_amount_cents bigint,
    new_amount_cents bigint NOT NULL,
    currency text DEFAULT 'usd'::text NOT NULL,
    billing_interval text NOT NULL,
    apply_to text NOT NULL,
    subscriptions_migrated integer DEFAULT 0 NOT NULL,
    subscriptions_failed integer DEFAULT 0 NOT NULL,
    failure_details jsonb,
    changed_by_user_id bigint,
    changed_at timestamp without time zone DEFAULT now() NOT NULL,
    CONSTRAINT membership_plan_price_history_apply_to_check CHECK ((apply_to = ANY (ARRAY['new_only'::text, 'next_renewal'::text, 'immediate_proration'::text]))),
    CONSTRAINT membership_plan_price_history_billing_interval_check CHECK ((billing_interval = ANY (ARRAY['month'::text, 'year'::text, 'one_time'::text])))
);


--
-- Name: TABLE membership_plan_price_history; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.membership_plan_price_history IS 'Append-only history of every Stripe Price change on a membership plan.';


--
-- Name: COLUMN membership_plan_price_history.apply_to; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.membership_plan_price_history.apply_to IS 'Rollout strategy: new_only|next_renewal|immediate_proration';


--
-- Name: membership_plan_price_history_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.membership_plan_price_history_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: membership_plan_price_history_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.membership_plan_price_history_id_seq OWNED BY public.membership_plan_price_history.id;


--
-- Name: membership_plans; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.membership_plans (
    id bigint NOT NULL,
    name character varying(255) NOT NULL,
    slug character varying(255) NOT NULL,
    description text,
    price numeric(10,2) NOT NULL,
    billing_cycle character varying(20) NOT NULL,
    is_active boolean DEFAULT true,
    metadata jsonb,
    stripe_price_id character varying(255),
    features jsonb,
    trial_days integer DEFAULT 0,
    created_at timestamp without time zone DEFAULT now(),
    updated_at timestamp without time zone DEFAULT now(),
    room_id bigint,
    interval_count integer DEFAULT 1,
    stripe_product_id character varying(255),
    display_name character varying(255),
    savings_percent integer DEFAULT 0,
    is_popular boolean DEFAULT false,
    sort_order integer DEFAULT 0,
    trial_period_days integer,
    trial_requires_payment_method boolean DEFAULT true NOT NULL,
    CONSTRAINT membership_plans_trial_period_days_check CHECK (((trial_period_days IS NULL) OR (trial_period_days = ANY (ARRAY[7, 14, 30]))))
);


--
-- Name: membership_plans_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.membership_plans_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: membership_plans_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.membership_plans_id_seq OWNED BY public.membership_plans.id;


--
-- Name: mfa_attempts; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.mfa_attempts (
    id bigint NOT NULL,
    user_id bigint NOT NULL,
    ip_address inet NOT NULL,
    success boolean DEFAULT false NOT NULL,
    attempt_type character varying(20) DEFAULT 'totp'::character varying NOT NULL,
    created_at timestamp with time zone DEFAULT now()
);


--
-- Name: TABLE mfa_attempts; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.mfa_attempts IS 'ICT7: MFA attempt tracking for rate limiting';


--
-- Name: mfa_attempts_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.mfa_attempts_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: mfa_attempts_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.mfa_attempts_id_seq OWNED BY public.mfa_attempts.id;


--
-- Name: resource_favorites; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.resource_favorites (
    id bigint NOT NULL,
    resource_id bigint NOT NULL,
    user_id bigint NOT NULL,
    created_at timestamp with time zone DEFAULT now()
);


--
-- Name: room_resources; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.room_resources (
    id bigint NOT NULL,
    title character varying(500) NOT NULL,
    slug character varying(500) NOT NULL,
    description text,
    room_id bigint NOT NULL,
    room_slug character varying(100) NOT NULL,
    resource_type character varying(50) DEFAULT 'video'::character varying NOT NULL,
    content_category character varying(100),
    file_url character varying(500),
    video_url character varying(500),
    video_platform character varying(50),
    bunny_video_guid character varying(100),
    bunny_library_id bigint,
    thumbnail_url character varying(500),
    duration_seconds integer,
    file_size_bytes bigint,
    mime_type character varying(100),
    is_featured boolean DEFAULT false,
    is_pinned boolean DEFAULT false,
    is_published boolean DEFAULT true,
    published_at timestamp with time zone,
    sort_order integer DEFAULT 0,
    view_count integer DEFAULT 0,
    download_count integer DEFAULT 0,
    tags jsonb DEFAULT '[]'::jsonb,
    meta_title character varying(255),
    meta_description text,
    created_by bigint,
    updated_by bigint,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now(),
    deleted_at timestamp with time zone,
    trading_room_id bigint,
    content_type character varying(50) DEFAULT 'other'::character varying,
    section character varying(50) DEFAULT 'latest_updates'::character varying,
    resource_date date DEFAULT CURRENT_DATE,
    video_id character varying(100),
    duration integer,
    width integer,
    height integer,
    file_path character varying(500),
    file_size bigint,
    thumbnail_path character varying(500),
    difficulty_level character varying(20) DEFAULT 'beginner'::character varying,
    views_count integer DEFAULT 0,
    downloads_count integer DEFAULT 0,
    likes_count integer DEFAULT 0,
    category character varying(100),
    metadata jsonb DEFAULT '{}'::jsonb,
    scheduled_at timestamp with time zone,
    trader_id bigint,
    access_level character varying(20) DEFAULT 'premium'::character varying,
    version integer DEFAULT 1,
    previous_version_id bigint,
    is_latest_version boolean DEFAULT true,
    course_id bigint,
    lesson_id bigint,
    course_order integer DEFAULT 0,
    secure_token character varying(64),
    secure_token_expires timestamp with time zone,
    file_hash character varying(64),
    storage_provider character varying(20) DEFAULT 'r2'::character varying
);


--
-- Name: TABLE room_resources; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.room_resources IS 'Unified resource storage for trading rooms - videos, PDFs, documents, images. ICT 7 Grade.';


--
-- Name: COLUMN room_resources.access_level; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.room_resources.access_level IS 'Access level: free, member, premium, vip';


--
-- Name: COLUMN room_resources.version; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.room_resources.version IS 'Version number, increments with each update';


--
-- Name: COLUMN room_resources.previous_version_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.room_resources.previous_version_id IS 'Reference to the previous version of this resource';


--
-- Name: COLUMN room_resources.is_latest_version; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.room_resources.is_latest_version IS 'Whether this is the latest version (for quick filtering)';


--
-- Name: COLUMN room_resources.course_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.room_resources.course_id IS 'Optional link to a course for course materials';


--
-- Name: COLUMN room_resources.lesson_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.room_resources.lesson_id IS 'Optional link to a specific lesson';


--
-- Name: COLUMN room_resources.course_order; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.room_resources.course_order IS 'Order within the course materials';


--
-- Name: COLUMN room_resources.secure_token; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.room_resources.secure_token IS 'Token for secure/signed URL downloads';


--
-- Name: COLUMN room_resources.secure_token_expires; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.room_resources.secure_token_expires IS 'Expiration time for secure download token';


--
-- Name: COLUMN room_resources.file_hash; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.room_resources.file_hash IS 'SHA-256 hash for deduplication and integrity';


--
-- Name: COLUMN room_resources.storage_provider; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.room_resources.storage_provider IS 'Storage backend: r2, bunny, s3, local';


--
-- Name: mv_resource_analytics; Type: MATERIALIZED VIEW; Schema: public; Owner: -
--

CREATE MATERIALIZED VIEW public.mv_resource_analytics AS
 SELECT r.trading_room_id,
    r.resource_type,
    COALESCE(r.access_level, 'premium'::character varying) AS access_level,
    count(*) AS resource_count,
    COALESCE(sum(r.views_count), (0)::bigint) AS total_views,
    COALESCE(sum(r.downloads_count), (0)::bigint) AS total_downloads,
    count(DISTINCT f.user_id) AS unique_favorites,
    max(r.created_at) AS latest_upload
   FROM (public.room_resources r
     LEFT JOIN public.resource_favorites f ON ((r.id = f.resource_id)))
  WHERE (r.deleted_at IS NULL)
  GROUP BY r.trading_room_id, r.resource_type, COALESCE(r.access_level, 'premium'::character varying)
  WITH NO DATA;


--
-- Name: newsletter_subscribers; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.newsletter_subscribers (
    id bigint NOT NULL,
    email character varying(255) NOT NULL,
    name character varying(255),
    status character varying(20) DEFAULT 'pending'::character varying,
    source character varying(100),
    ip_address character varying(45),
    user_agent text,
    tags jsonb,
    metadata jsonb,
    confirmed_at timestamp without time zone,
    unsubscribed_at timestamp without time zone,
    created_at timestamp without time zone DEFAULT now(),
    updated_at timestamp without time zone DEFAULT now()
);


--
-- Name: newsletter_subscribers_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.newsletter_subscribers_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: newsletter_subscribers_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.newsletter_subscribers_id_seq OWNED BY public.newsletter_subscribers.id;


--
-- Name: oauth_audit_log; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.oauth_audit_log (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    user_id bigint,
    provider character varying(50) NOT NULL,
    action character varying(50) NOT NULL,
    provider_user_id character varying(255),
    ip_address character varying(45),
    user_agent text,
    metadata jsonb DEFAULT '{}'::jsonb,
    created_at timestamp without time zone DEFAULT now()
);


--
-- Name: TABLE oauth_audit_log; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.oauth_audit_log IS 'Security audit trail for OAuth operations';


--
-- Name: oauth_states; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.oauth_states (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    state character varying(255) NOT NULL,
    provider character varying(50) NOT NULL,
    redirect_uri text,
    code_verifier character varying(128),
    nonce character varying(255),
    created_at timestamp without time zone DEFAULT now(),
    expires_at timestamp without time zone DEFAULT (now() + '00:10:00'::interval),
    used_at timestamp without time zone
);


--
-- Name: TABLE oauth_states; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.oauth_states IS 'CSRF protection states for OAuth flow - auto-expire after 10 minutes';


--
-- Name: order_items; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.order_items (
    id bigint NOT NULL,
    order_id bigint NOT NULL,
    product_id bigint,
    plan_id bigint,
    name character varying(255) NOT NULL,
    description text,
    quantity integer DEFAULT 1 NOT NULL,
    unit_price numeric(10,2) NOT NULL,
    total numeric(10,2) NOT NULL,
    metadata jsonb,
    created_at timestamp without time zone DEFAULT now()
);


--
-- Name: order_items_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.order_items_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: order_items_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.order_items_id_seq OWNED BY public.order_items.id;


--
-- Name: orders; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.orders (
    id bigint NOT NULL,
    user_id bigint NOT NULL,
    order_number character varying(50) NOT NULL,
    status character varying(20) DEFAULT 'pending'::character varying,
    subtotal numeric(10,2) DEFAULT 0 NOT NULL,
    discount numeric(10,2) DEFAULT 0 NOT NULL,
    tax numeric(10,2) DEFAULT 0 NOT NULL,
    total numeric(10,2) DEFAULT 0 NOT NULL,
    currency character varying(3) DEFAULT 'USD'::character varying,
    payment_provider character varying(50),
    payment_intent_id character varying(255),
    stripe_session_id character varying(255),
    coupon_id bigint,
    coupon_code character varying(50),
    billing_name character varying(255),
    billing_email character varying(255),
    billing_address jsonb,
    metadata jsonb,
    completed_at timestamp without time zone,
    refunded_at timestamp without time zone,
    created_at timestamp without time zone DEFAULT now(),
    updated_at timestamp without time zone DEFAULT now(),
    refund_status text,
    refund_amount numeric(10,2) DEFAULT 0 NOT NULL
);


--
-- Name: orders_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.orders_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: orders_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.orders_id_seq OWNED BY public.orders.id;


--
-- Name: page_layout_versions; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.page_layout_versions (
    id bigint NOT NULL,
    layout_id bigint NOT NULL,
    version integer NOT NULL,
    blocks jsonb NOT NULL,
    created_at timestamp with time zone DEFAULT now(),
    created_by bigint
);


--
-- Name: page_layout_versions_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.page_layout_versions_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: page_layout_versions_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.page_layout_versions_id_seq OWNED BY public.page_layout_versions.id;


--
-- Name: page_layouts; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.page_layouts (
    id bigint NOT NULL,
    title character varying(255) NOT NULL,
    slug character varying(255),
    description text,
    status character varying(50) DEFAULT 'draft'::character varying,
    version integer DEFAULT 1,
    blocks jsonb DEFAULT '[]'::jsonb NOT NULL,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now(),
    published_at timestamp with time zone,
    created_by bigint,
    updated_by bigint
);


--
-- Name: page_layouts_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.page_layouts_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: page_layouts_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.page_layouts_id_seq OWNED BY public.page_layouts.id;


--
-- Name: password_resets; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.password_resets (
    id bigint NOT NULL,
    email character varying(255) NOT NULL,
    token character varying(255) NOT NULL,
    expires_at timestamp without time zone NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL
);


--
-- Name: password_resets_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.password_resets_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: password_resets_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.password_resets_id_seq OWNED BY public.password_resets.id;


--
-- Name: payment_disputes; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.payment_disputes (
    id bigint NOT NULL,
    stripe_dispute_id text NOT NULL,
    stripe_charge_id text NOT NULL,
    reason text,
    status text NOT NULL,
    amount_cents bigint NOT NULL,
    currency text DEFAULT 'usd'::text NOT NULL,
    response_deadline timestamp with time zone,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: payment_disputes_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.payment_disputes_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: payment_disputes_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.payment_disputes_id_seq OWNED BY public.payment_disputes.id;


--
-- Name: popup_ab_tests; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.popup_ab_tests (
    id integer NOT NULL,
    name character varying(255) NOT NULL,
    status character varying(20) DEFAULT 'draft'::character varying NOT NULL,
    base_popup_id integer NOT NULL,
    winner_popup_id integer,
    confidence_threshold double precision DEFAULT 0.95 NOT NULL,
    start_date timestamp with time zone,
    end_date timestamp with time zone,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: TABLE popup_ab_tests; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.popup_ab_tests IS 'A/B test configurations for popup optimization';


--
-- Name: popup_ab_tests_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.popup_ab_tests_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: popup_ab_tests_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.popup_ab_tests_id_seq OWNED BY public.popup_ab_tests.id;


--
-- Name: popup_events; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.popup_events (
    id bigint NOT NULL,
    popup_id integer NOT NULL,
    event_type character varying(20) NOT NULL,
    session_id character varying(100),
    user_id integer,
    page_url text,
    device_type character varying(20),
    metadata jsonb,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: TABLE popup_events; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.popup_events IS 'Analytics events tracking for popups (views, conversions, etc.)';


--
-- Name: popup_events_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.popup_events_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: popup_events_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.popup_events_id_seq OWNED BY public.popup_events.id;


--
-- Name: popup_form_submissions; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.popup_form_submissions (
    id bigint NOT NULL,
    popup_id integer NOT NULL,
    form_id integer,
    data jsonb NOT NULL,
    session_id character varying(100),
    metadata jsonb,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: TABLE popup_form_submissions; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.popup_form_submissions IS 'Form submission data from popup forms';


--
-- Name: popup_form_submissions_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.popup_form_submissions_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: popup_form_submissions_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.popup_form_submissions_id_seq OWNED BY public.popup_form_submissions.id;


--
-- Name: popups; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.popups (
    id integer NOT NULL,
    name character varying(255) NOT NULL,
    type character varying(50) DEFAULT 'timed'::character varying NOT NULL,
    status character varying(20) DEFAULT 'draft'::character varying NOT NULL,
    priority integer DEFAULT 10 NOT NULL,
    title text,
    content text,
    cta_text character varying(255),
    cta_url text,
    cta_new_tab boolean DEFAULT false NOT NULL,
    "position" character varying(20) DEFAULT 'center'::character varying NOT NULL,
    size character varying(10) DEFAULT 'md'::character varying NOT NULL,
    animation character varying(20) DEFAULT 'zoom'::character varying NOT NULL,
    show_close_button boolean DEFAULT true NOT NULL,
    close_on_overlay_click boolean DEFAULT true NOT NULL,
    close_on_escape boolean DEFAULT true NOT NULL,
    auto_close_after integer,
    has_form boolean DEFAULT false NOT NULL,
    form_id integer,
    trigger_rules jsonb DEFAULT '{}'::jsonb NOT NULL,
    frequency_rules jsonb DEFAULT '{"frequency": "once_per_session"}'::jsonb NOT NULL,
    display_rules jsonb DEFAULT '{"devices": ["desktop", "tablet", "mobile"]}'::jsonb NOT NULL,
    design jsonb DEFAULT '{"text_color": "#4b5563", "title_color": "#1f2937", "button_color": "#3b82f6", "border_radius": 12, "overlay_color": "#000000", "overlay_opacity": 50, "background_color": "#ffffff", "button_text_color": "#ffffff", "button_border_radius": 8}'::jsonb NOT NULL,
    ab_test_id integer,
    variant_name character varying(50),
    traffic_allocation integer,
    start_date timestamp with time zone,
    end_date timestamp with time zone,
    total_views bigint DEFAULT 0 NOT NULL,
    total_conversions bigint DEFAULT 0 NOT NULL,
    conversion_rate double precision DEFAULT 0.0 NOT NULL,
    created_by integer,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: TABLE popups; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.popups IS 'Revolution Trading Pros - Complete popup/modal system with analytics and A/B testing';


--
-- Name: popups_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.popups_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: popups_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.popups_id_seq OWNED BY public.popups.id;


--
-- Name: post_categories; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.post_categories (
    post_id bigint NOT NULL,
    category_id bigint NOT NULL
);


--
-- Name: post_tags; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.post_tags (
    post_id bigint NOT NULL,
    tag_id bigint NOT NULL
);


--
-- Name: posts; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.posts (
    id bigint NOT NULL,
    author_id bigint NOT NULL,
    title character varying(255) NOT NULL,
    slug character varying(255) NOT NULL,
    excerpt text,
    content_blocks jsonb,
    featured_image character varying(500),
    status character varying(20) DEFAULT 'draft'::character varying,
    published_at timestamp without time zone,
    meta_title character varying(255),
    meta_description text,
    indexable boolean DEFAULT true,
    canonical_url character varying(500),
    schema_markup jsonb,
    created_at timestamp without time zone DEFAULT now(),
    updated_at timestamp without time zone DEFAULT now(),
    scheduled_publish_at timestamp with time zone,
    scheduled_unpublish_at timestamp with time zone,
    locale character varying(10) DEFAULT 'en'::character varying,
    is_primary_locale boolean DEFAULT true,
    parent_post_id bigint,
    featured_media_id bigint,
    featured_image_alt text,
    featured_image_caption text,
    featured_image_title character varying(255),
    featured_image_description text,
    meta_keywords text[],
    allow_comments boolean DEFAULT true NOT NULL
);


--
-- Name: COLUMN posts.featured_media_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.posts.featured_media_id IS 'Optional FK to a media table once one is wired; today this is a free integer for forward-compat.';


--
-- Name: COLUMN posts.featured_image_alt; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.posts.featured_image_alt IS 'Accessibility / SEO alt text for the featured image.';


--
-- Name: COLUMN posts.featured_image_caption; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.posts.featured_image_caption IS 'Caption shown below the featured image.';


--
-- Name: COLUMN posts.featured_image_title; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.posts.featured_image_title IS 'Display title overlay or hover-text for the featured image.';


--
-- Name: COLUMN posts.featured_image_description; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.posts.featured_image_description IS 'Long-form description; useful for image SEO.';


--
-- Name: COLUMN posts.meta_keywords; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.posts.meta_keywords IS 'Legacy SEO meta keywords (kept for completeness).';


--
-- Name: COLUMN posts.allow_comments; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.posts.allow_comments IS 'Whether to show comments on this post.';


--
-- Name: posts_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.posts_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: posts_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.posts_id_seq OWNED BY public.posts.id;


--
-- Name: preview_tokens; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.preview_tokens (
    id bigint NOT NULL,
    content_type character varying(50) NOT NULL,
    content_id bigint NOT NULL,
    token uuid DEFAULT gen_random_uuid() NOT NULL,
    expires_at timestamp with time zone NOT NULL,
    max_views integer,
    view_count integer DEFAULT 0 NOT NULL,
    password_hash character varying(255),
    allowed_emails text[],
    created_by bigint,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    last_accessed_at timestamp with time zone
);


--
-- Name: TABLE preview_tokens; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.preview_tokens IS 'Secure preview links for unpublished content';


--
-- Name: preview_tokens_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.preview_tokens_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: preview_tokens_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.preview_tokens_id_seq OWNED BY public.preview_tokens.id;


--
-- Name: products; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.products (
    id bigint NOT NULL,
    name character varying(255) NOT NULL,
    slug character varying(255) NOT NULL,
    type character varying(50) NOT NULL,
    description text,
    long_description text,
    price numeric(10,2) DEFAULT 0 NOT NULL,
    is_active boolean DEFAULT true,
    metadata jsonb,
    thumbnail character varying(500),
    meta_title character varying(255),
    meta_description text,
    indexable boolean DEFAULT true,
    canonical_url character varying(500),
    created_at timestamp without time zone DEFAULT now(),
    updated_at timestamp without time zone DEFAULT now(),
    scheduled_publish_at timestamp with time zone,
    scheduled_unpublish_at timestamp with time zone,
    locale character varying(10) DEFAULT 'en'::character varying,
    course_id uuid,
    indicator_id bigint
);


--
-- Name: products_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.products_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: products_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.products_id_seq OWNED BY public.products.id;


--
-- Name: quiz_answers; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.quiz_answers (
    id bigint NOT NULL,
    question_id bigint NOT NULL,
    answer_text text NOT NULL,
    is_correct boolean DEFAULT false,
    sort_order integer DEFAULT 0,
    feedback text
);


--
-- Name: quiz_answers_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.quiz_answers_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: quiz_answers_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.quiz_answers_id_seq OWNED BY public.quiz_answers.id;


--
-- Name: quiz_questions; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.quiz_questions (
    id bigint NOT NULL,
    quiz_id bigint NOT NULL,
    question_type character varying(30) DEFAULT 'multiple_choice'::character varying,
    question_text text NOT NULL,
    question_html text,
    explanation text,
    points integer DEFAULT 1,
    sort_order integer DEFAULT 0,
    is_required boolean DEFAULT true,
    media_url text,
    created_at timestamp without time zone DEFAULT now(),
    updated_at timestamp without time zone DEFAULT now()
);


--
-- Name: quiz_questions_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.quiz_questions_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: quiz_questions_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.quiz_questions_id_seq OWNED BY public.quiz_questions.id;


--
-- Name: reconciliation_log; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.reconciliation_log (
    id bigint NOT NULL,
    run_at timestamp without time zone DEFAULT now() NOT NULL,
    discrepancies_found integer DEFAULT 0 NOT NULL,
    log jsonb DEFAULT '[]'::jsonb NOT NULL
);


--
-- Name: TABLE reconciliation_log; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.reconciliation_log IS 'Append-only record of every Stripe reconciliation job run.';


--
-- Name: COLUMN reconciliation_log.log; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.reconciliation_log.log IS 'Array of discrepancy objects: {sub_id, kind, before, after, fixed}';


--
-- Name: reconciliation_log_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.reconciliation_log_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: reconciliation_log_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.reconciliation_log_id_seq OWNED BY public.reconciliation_log.id;


--
-- Name: redirects; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.redirects (
    id bigint NOT NULL,
    from_path character varying(500) NOT NULL,
    to_path character varying(500) NOT NULL,
    status_code integer DEFAULT 301 NOT NULL,
    redirect_type character varying(50) DEFAULT 'manual'::character varying NOT NULL,
    is_active boolean DEFAULT true NOT NULL,
    hit_count integer DEFAULT 0 NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: TABLE redirects; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.redirects IS 'URL redirect rules for SEO and site maintenance';


--
-- Name: COLUMN redirects.from_path; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.redirects.from_path IS 'Source URL path to redirect from';


--
-- Name: COLUMN redirects.to_path; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.redirects.to_path IS 'Destination URL path to redirect to';


--
-- Name: COLUMN redirects.status_code; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.redirects.status_code IS 'HTTP status code (301, 302, 307, 308)';


--
-- Name: COLUMN redirects.redirect_type; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.redirects.redirect_type IS 'Type of redirect (manual, automatic, migration)';


--
-- Name: COLUMN redirects.hit_count; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.redirects.hit_count IS 'Number of times this redirect was triggered';


--
-- Name: redirects_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.redirects_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: redirects_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.redirects_id_seq OWNED BY public.redirects.id;


--
-- Name: resource_access_log; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.resource_access_log (
    id bigint NOT NULL,
    user_id bigint NOT NULL,
    resource_id bigint NOT NULL,
    resource_type character varying(50) NOT NULL,
    resource_title character varying(500) NOT NULL,
    resource_thumbnail character varying(500),
    accessed_at timestamp without time zone DEFAULT now()
);


--
-- Name: TABLE resource_access_log; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.resource_access_log IS 'Tracks user resource access for recently accessed feature - ICT 7 Grade';


--
-- Name: resource_access_log_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.resource_access_log_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: resource_access_log_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.resource_access_log_id_seq OWNED BY public.resource_access_log.id;


--
-- Name: resource_download_logs; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.resource_download_logs (
    id bigint NOT NULL,
    resource_id bigint NOT NULL,
    user_id bigint,
    session_id character varying(64),
    ip_address inet,
    user_agent text,
    referer character varying(500),
    downloaded_at timestamp with time zone DEFAULT now()
);


--
-- Name: resource_download_logs_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.resource_download_logs_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: resource_download_logs_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.resource_download_logs_id_seq OWNED BY public.resource_download_logs.id;


--
-- Name: resource_favorites_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.resource_favorites_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: resource_favorites_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.resource_favorites_id_seq OWNED BY public.resource_favorites.id;


--
-- Name: resource_upload_limits; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.resource_upload_limits (
    id bigint NOT NULL,
    resource_type character varying(50) NOT NULL,
    max_file_size_bytes bigint NOT NULL,
    allowed_mime_types text[] DEFAULT '{}'::text[] NOT NULL,
    allowed_extensions text[] DEFAULT '{}'::text[] NOT NULL,
    requires_premium boolean DEFAULT false,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now()
);


--
-- Name: resource_upload_limits_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.resource_upload_limits_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: resource_upload_limits_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.resource_upload_limits_id_seq OWNED BY public.resource_upload_limits.id;


--
-- Name: review_helpful_votes; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.review_helpful_votes (
    id bigint NOT NULL,
    review_id bigint NOT NULL,
    user_id bigint NOT NULL,
    helpful boolean NOT NULL,
    created_at timestamp without time zone DEFAULT now()
);


--
-- Name: review_helpful_votes_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.review_helpful_votes_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: review_helpful_votes_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.review_helpful_votes_id_seq OWNED BY public.review_helpful_votes.id;


--
-- Name: room_alerts; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.room_alerts (
    id bigint NOT NULL,
    room_id bigint NOT NULL,
    room_slug character varying(100) NOT NULL,
    alert_type character varying(20) DEFAULT 'ENTRY'::character varying NOT NULL,
    ticker character varying(10) NOT NULL,
    message text NOT NULL,
    price character varying(20),
    is_active boolean DEFAULT true,
    created_by bigint,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now(),
    title character varying(500),
    notes text,
    trade_type character varying(20),
    action character varying(10),
    quantity integer,
    option_type character varying(10),
    strike numeric(10,2),
    expiration date,
    contract_type character varying(20),
    order_type character varying(10),
    limit_price numeric(10,2),
    fill_price numeric(10,2),
    tos_string text,
    entry_alert_id bigint,
    trade_plan_id bigint,
    is_new boolean DEFAULT true,
    is_published boolean DEFAULT true,
    is_pinned boolean DEFAULT false,
    published_at timestamp with time zone DEFAULT now(),
    deleted_at timestamp with time zone
);


--
-- Name: room_alerts_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.room_alerts_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: room_alerts_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.room_alerts_id_seq OWNED BY public.room_alerts.id;


--
-- Name: room_resources_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.room_resources_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: room_resources_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.room_resources_id_seq OWNED BY public.room_resources.id;


--
-- Name: room_sections; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.room_sections (
    id bigint NOT NULL,
    room_id bigint NOT NULL,
    room_slug character varying(100) NOT NULL,
    title character varying(255) NOT NULL,
    description text,
    sort_order integer DEFAULT 0,
    is_active boolean DEFAULT true,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now()
);


--
-- Name: room_sections_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.room_sections_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: room_sections_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.room_sections_id_seq OWNED BY public.room_sections.id;


--
-- Name: room_stats_cache; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.room_stats_cache (
    id bigint NOT NULL,
    room_id bigint,
    room_slug character varying(100) NOT NULL,
    total_videos integer DEFAULT 0,
    total_pdfs integer DEFAULT 0,
    total_documents integer DEFAULT 0,
    total_images integer DEFAULT 0,
    total_resources integer DEFAULT 0,
    last_video_at timestamp with time zone,
    last_resource_at timestamp with time zone,
    win_rate numeric(5,2),
    weekly_profit character varying(50),
    monthly_profit character varying(50),
    active_trades integer DEFAULT 0,
    closed_this_week integer DEFAULT 0,
    total_trades integer DEFAULT 0,
    wins integer DEFAULT 0,
    losses integer DEFAULT 0,
    avg_win numeric(12,2),
    avg_loss numeric(12,2),
    profit_factor numeric(6,2),
    avg_holding_days numeric(6,2),
    largest_win numeric(12,2),
    largest_loss numeric(12,2),
    current_streak integer DEFAULT 0,
    daily_pnl_30d jsonb DEFAULT '[]'::jsonb,
    calculated_at timestamp with time zone DEFAULT now()
);


--
-- Name: room_stats_cache_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.room_stats_cache_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: room_stats_cache_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.room_stats_cache_id_seq OWNED BY public.room_stats_cache.id;


--
-- Name: room_trade_plans; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.room_trade_plans (
    id bigint NOT NULL,
    room_id bigint NOT NULL,
    room_slug character varying(100) NOT NULL,
    week_of date DEFAULT CURRENT_DATE NOT NULL,
    ticker character varying(10) NOT NULL,
    bias character varying(20) NOT NULL,
    entry character varying(20),
    target1 character varying(20),
    target2 character varying(20),
    target3 character varying(20),
    runner character varying(20),
    stop character varying(20),
    options_strike character varying(50),
    options_exp date,
    notes text,
    sort_order integer DEFAULT 0,
    is_active boolean DEFAULT true,
    created_by bigint,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now(),
    deleted_at timestamp with time zone,
    runner_stop character varying(20)
);


--
-- Name: COLUMN room_trade_plans.runner_stop; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.room_trade_plans.runner_stop IS 'Stop loss price for runner position';


--
-- Name: room_trade_plans_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.room_trade_plans_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: room_trade_plans_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.room_trade_plans_id_seq OWNED BY public.room_trade_plans.id;


--
-- Name: room_traders; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.room_traders (
    id bigint NOT NULL,
    user_id bigint,
    name character varying(255) NOT NULL,
    slug character varying(255),
    title character varying(255),
    bio text,
    avatar_url character varying(500),
    cover_image_url character varying(500),
    twitter_url character varying(500),
    youtube_url character varying(500),
    instagram_url character varying(500),
    trading_style character varying(100),
    specialties jsonb DEFAULT '[]'::jsonb,
    years_experience integer,
    is_active boolean DEFAULT true,
    is_featured boolean DEFAULT false,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now()
);


--
-- Name: room_traders_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.room_traders_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: room_traders_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.room_traders_id_seq OWNED BY public.room_traders.id;


--
-- Name: room_trades; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.room_trades (
    id bigint NOT NULL,
    room_id bigint NOT NULL,
    room_slug character varying(100) NOT NULL,
    ticker character varying(10) NOT NULL,
    trade_type character varying(20) DEFAULT 'shares'::character varying NOT NULL,
    direction character varying(10) DEFAULT 'long'::character varying NOT NULL,
    quantity integer NOT NULL,
    option_type character varying(10),
    strike numeric(10,2),
    expiration date,
    contract_type character varying(20),
    entry_alert_id bigint,
    entry_price numeric(10,2) NOT NULL,
    entry_date date NOT NULL,
    entry_tos_string text,
    exit_alert_id bigint,
    exit_price numeric(10,2),
    exit_date date,
    exit_tos_string text,
    setup character varying(50),
    status character varying(20) DEFAULT 'open'::character varying NOT NULL,
    result character varying(10),
    pnl numeric(12,2),
    pnl_percent numeric(6,2),
    holding_days integer,
    notes text,
    created_by bigint,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now(),
    deleted_at timestamp with time zone,
    was_updated boolean DEFAULT false,
    invalidation_reason text
);


--
-- Name: COLUMN room_trades.status; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.room_trades.status IS 'Trade status: open, closed, partial, invalidated';


--
-- Name: room_trades_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.room_trades_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: room_trades_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.room_trades_id_seq OWNED BY public.room_trades.id;


--
-- Name: room_weekly_videos; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.room_weekly_videos (
    id bigint NOT NULL,
    room_id bigint NOT NULL,
    room_slug character varying(100) NOT NULL,
    week_of date NOT NULL,
    week_title character varying(255),
    video_title character varying(500),
    video_id bigint,
    title character varying(500),
    video_url text,
    video_platform character varying(50) DEFAULT 'bunny'::character varying,
    bunny_video_guid character varying(100),
    bunny_library_id bigint,
    thumbnail_url text,
    duration character varying(20),
    description text,
    is_current boolean DEFAULT true,
    is_featured boolean DEFAULT true,
    is_published boolean DEFAULT true,
    published_at timestamp with time zone DEFAULT now(),
    archived_at timestamp with time zone,
    created_by bigint,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now()
);


--
-- Name: room_weekly_videos_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.room_weekly_videos_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: room_weekly_videos_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.room_weekly_videos_id_seq OWNED BY public.room_weekly_videos.id;


--
-- Name: schedule_exceptions; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.schedule_exceptions (
    id bigint NOT NULL,
    schedule_id bigint NOT NULL,
    exception_date date NOT NULL,
    exception_type character varying(20) NOT NULL,
    new_start_time time without time zone,
    new_end_time time without time zone,
    new_trader_name character varying(255),
    reason text,
    created_by bigint,
    created_at timestamp without time zone DEFAULT now()
);


--
-- Name: schedule_exceptions_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.schedule_exceptions_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: schedule_exceptions_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.schedule_exceptions_id_seq OWNED BY public.schedule_exceptions.id;


--
-- Name: scheduled_content; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.scheduled_content (
    id bigint NOT NULL,
    content_type character varying(50) NOT NULL,
    content_id bigint NOT NULL,
    scheduled_action character varying(50) NOT NULL,
    scheduled_at timestamp with time zone NOT NULL,
    timezone character varying(50) DEFAULT 'UTC'::character varying,
    payload jsonb,
    status character varying(20) DEFAULT 'scheduled'::character varying NOT NULL,
    executed_at timestamp with time zone,
    error_message text,
    created_by bigint,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: TABLE scheduled_content; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.scheduled_content IS 'Content scheduled for future publication';


--
-- Name: scheduled_content_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.scheduled_content_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: scheduled_content_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.scheduled_content_id_seq OWNED BY public.scheduled_content.id;


--
-- Name: security_events; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.security_events (
    id bigint NOT NULL,
    user_id bigint,
    event_type character varying(100) NOT NULL,
    ip_address inet,
    user_agent text,
    details jsonb,
    created_at timestamp with time zone DEFAULT now(),
    event_category character varying(50) DEFAULT 'general'::character varying,
    severity character varying(20) DEFAULT 'info'::character varying
);


--
-- Name: security_events_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.security_events_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: security_events_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.security_events_id_seq OWNED BY public.security_events.id;


--
-- Name: service_connections; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.service_connections (
    id bigint NOT NULL,
    service_key text NOT NULL,
    name text NOT NULL,
    category text NOT NULL,
    description text,
    status text DEFAULT 'pending'::text NOT NULL,
    health_score integer DEFAULT 0 NOT NULL,
    health_status text,
    environment text,
    credentials_encrypted text,
    settings jsonb,
    webhook_url text,
    webhook_secret text,
    api_calls_today integer DEFAULT 0 NOT NULL,
    api_calls_total bigint DEFAULT 0 NOT NULL,
    last_error text,
    last_verified_at timestamp with time zone,
    connected_at timestamp with time zone,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    CONSTRAINT service_connections_status_check CHECK ((status = ANY (ARRAY['connected'::text, 'pending'::text, 'disconnected'::text, 'error'::text])))
);


--
-- Name: TABLE service_connections; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.service_connections IS 'Third-party service credentials, env-aware, resolved at request-time by CredentialResolver.';


--
-- Name: COLUMN service_connections.environment; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.service_connections.environment IS 'Optional environment scope (production|sandbox). One row per (service_key, environment).';


--
-- Name: COLUMN service_connections.webhook_secret; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.service_connections.webhook_secret IS 'Webhook signing secret for inbound events from this provider (e.g. Stripe whsec_).';


--
-- Name: service_connections_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.service_connections_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: service_connections_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.service_connections_id_seq OWNED BY public.service_connections.id;


--
-- Name: settings; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.settings (
    id bigint NOT NULL,
    key character varying(255) NOT NULL,
    value jsonb,
    description text,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now()
);


--
-- Name: settings_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.settings_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: settings_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.settings_id_seq OWNED BY public.settings.id;


--
-- Name: stock_lists; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.stock_lists (
    id bigint NOT NULL,
    name character varying(255) NOT NULL,
    slug character varying(255) NOT NULL,
    description text,
    list_type character varying(50) DEFAULT 'watchlist'::character varying NOT NULL,
    trading_room_id bigint NOT NULL,
    symbols jsonb DEFAULT '[]'::jsonb NOT NULL,
    is_active boolean DEFAULT true,
    is_featured boolean DEFAULT false,
    sort_order integer DEFAULT 0,
    week_of date,
    created_by bigint,
    created_at timestamp without time zone DEFAULT now(),
    updated_at timestamp without time zone DEFAULT now()
);


--
-- Name: TABLE stock_lists; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.stock_lists IS 'Stock, ETF, and watchlist management for trading rooms - ICT 7 Grade';


--
-- Name: COLUMN stock_lists.symbols; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.stock_lists.symbols IS 'JSON array with symbol details: {symbol, name, sector, notes, price_target, entry_price, stop_loss}';


--
-- Name: stock_lists_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.stock_lists_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: stock_lists_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.stock_lists_id_seq OWNED BY public.stock_lists.id;


--
-- Name: trading_rooms; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.trading_rooms (
    id bigint NOT NULL,
    name character varying(255) NOT NULL,
    slug character varying(255) NOT NULL,
    description text,
    short_description character varying(500),
    thumbnail_url character varying(500),
    banner_url character varying(500),
    room_type character varying(50) DEFAULT 'trading'::character varying,
    access_level character varying(50) DEFAULT 'member'::character varying,
    is_active boolean DEFAULT true,
    is_featured boolean DEFAULT false,
    is_public boolean DEFAULT true,
    sort_order integer DEFAULT 0,
    available_sections jsonb DEFAULT '[]'::jsonb,
    features jsonb DEFAULT '{}'::jsonb,
    icon character varying(50),
    color character varying(20),
    settings jsonb DEFAULT '{}'::jsonb,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now()
);


--
-- Name: subscription_plans_view; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.subscription_plans_view AS
 SELECT mp.id AS plan_id,
    mp.name AS plan_name,
    mp.slug AS plan_slug,
    mp.display_name,
    mp.description,
    mp.price,
    mp.billing_cycle,
    mp.interval_count,
    mp.savings_percent,
    mp.is_popular,
    mp.is_active,
    mp.features,
    mp.trial_days,
    mp.stripe_price_id,
    mp.stripe_product_id,
    mp.sort_order,
    tr.id AS room_id,
    tr.name AS room_name,
    tr.slug AS room_slug,
    tr.room_type,
    tr.icon AS room_icon,
    tr.color AS room_color,
    mp.created_at,
    mp.updated_at
   FROM (public.membership_plans mp
     LEFT JOIN public.trading_rooms tr ON ((tr.id = mp.room_id)))
  WHERE (mp.is_active = true)
  ORDER BY tr.sort_order, mp.sort_order;


--
-- Name: tags; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.tags (
    id bigint NOT NULL,
    name character varying(255) NOT NULL,
    slug character varying(255) NOT NULL,
    created_at timestamp without time zone DEFAULT now()
);


--
-- Name: tags_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.tags_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: tags_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.tags_id_seq OWNED BY public.tags.id;


--
-- Name: teams; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.teams (
    id bigint NOT NULL,
    name character varying(255) NOT NULL,
    slug character varying(255) NOT NULL,
    description text,
    color character varying(7) DEFAULT '#3b82f6'::character varying,
    icon character varying(50),
    is_active boolean DEFAULT true,
    member_count integer DEFAULT 0,
    created_by bigint,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now()
);


--
-- Name: teams_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.teams_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: teams_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.teams_id_seq OWNED BY public.teams.id;


--
-- Name: traders; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.traders (
    id bigint NOT NULL,
    name character varying(255),
    slug character varying(255),
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now()
);


--
-- Name: traders_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.traders_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: traders_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.traders_id_seq OWNED BY public.traders.id;


--
-- Name: trading_room_schedules; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.trading_room_schedules (
    id bigint NOT NULL,
    plan_id bigint NOT NULL,
    title character varying(255) NOT NULL,
    description text,
    trader_name character varying(255),
    trader_id bigint,
    day_of_week integer NOT NULL,
    start_time time without time zone NOT NULL,
    end_time time without time zone NOT NULL,
    timezone character varying(50) DEFAULT 'America/New_York'::character varying,
    is_recurring boolean DEFAULT true,
    effective_from date DEFAULT CURRENT_DATE,
    effective_until date,
    is_active boolean DEFAULT true,
    room_url character varying(500),
    room_type character varying(50) DEFAULT 'live'::character varying,
    metadata jsonb DEFAULT '{}'::jsonb,
    created_by bigint,
    updated_by bigint,
    created_at timestamp without time zone DEFAULT now(),
    updated_at timestamp without time zone DEFAULT now(),
    CONSTRAINT valid_day_of_week CHECK (((day_of_week >= 0) AND (day_of_week <= 6))),
    CONSTRAINT valid_time_range CHECK ((end_time > start_time))
);


--
-- Name: trading_room_schedules_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.trading_room_schedules_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: trading_room_schedules_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.trading_room_schedules_id_seq OWNED BY public.trading_room_schedules.id;


--
-- Name: trading_rooms_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.trading_rooms_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: trading_rooms_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.trading_rooms_id_seq OWNED BY public.trading_rooms.id;


--
-- Name: unified_videos; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.unified_videos (
    id bigint NOT NULL,
    title character varying(500) NOT NULL,
    slug character varying(500) NOT NULL,
    description text,
    video_url character varying(500),
    video_platform character varying(50) DEFAULT 'bunny'::character varying,
    bunny_video_guid character varying(100),
    bunny_library_id bigint,
    thumbnail_url character varying(500),
    duration integer,
    views_count integer DEFAULT 0,
    content_type character varying(50) DEFAULT 'daily_video'::character varying NOT NULL,
    room_id bigint,
    room_slug character varying(100),
    video_date date,
    week_of date,
    is_featured boolean DEFAULT false,
    is_published boolean DEFAULT true,
    published_at timestamp with time zone,
    created_by bigint,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now(),
    deleted_at timestamp with time zone,
    tags jsonb DEFAULT '[]'::jsonb
);


--
-- Name: unified_videos_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.unified_videos_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: unified_videos_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.unified_videos_id_seq OWNED BY public.unified_videos.id;


--
-- Name: user_activity_log; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.user_activity_log (
    id bigint NOT NULL,
    user_id bigint NOT NULL,
    action character varying(100) NOT NULL,
    entity_type character varying(50),
    entity_id bigint,
    details jsonb,
    ip_address inet,
    user_agent text,
    created_at timestamp with time zone DEFAULT now()
);


--
-- Name: user_activity_log_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.user_activity_log_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: user_activity_log_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.user_activity_log_id_seq OWNED BY public.user_activity_log.id;


--
-- Name: user_coupons; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.user_coupons (
    id bigint NOT NULL,
    user_id bigint NOT NULL,
    coupon_id bigint NOT NULL,
    used_at timestamp with time zone DEFAULT now(),
    order_id bigint
);


--
-- Name: user_coupons_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.user_coupons_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: user_coupons_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.user_coupons_id_seq OWNED BY public.user_coupons.id;


--
-- Name: user_course_enrollments; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.user_course_enrollments (
    id bigint NOT NULL,
    user_id bigint NOT NULL,
    course_id uuid NOT NULL,
    enrolled_at timestamp with time zone DEFAULT now(),
    enrollment_source character varying(50),
    access_expires_at timestamp with time zone,
    is_active boolean DEFAULT true,
    completed_lessons integer DEFAULT 0,
    total_lessons integer DEFAULT 0,
    progress_percent integer DEFAULT 0,
    is_completed boolean DEFAULT false,
    completed_at timestamp with time zone,
    certificate_issued boolean DEFAULT false,
    certificate_url character varying(500),
    last_lesson_id bigint,
    last_position_seconds integer DEFAULT 0,
    last_activity_at timestamp with time zone DEFAULT now(),
    certificate_issued_at timestamp without time zone
);


--
-- Name: user_course_enrollments_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.user_course_enrollments_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: user_course_enrollments_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.user_course_enrollments_id_seq OWNED BY public.user_course_enrollments.id;


--
-- Name: user_departments; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.user_departments (
    id bigint NOT NULL,
    user_id bigint NOT NULL,
    department_id bigint NOT NULL,
    role character varying(50) DEFAULT 'member'::character varying,
    joined_at timestamp with time zone DEFAULT now()
);


--
-- Name: user_departments_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.user_departments_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: user_departments_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.user_departments_id_seq OWNED BY public.user_departments.id;


--
-- Name: user_favorites; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.user_favorites (
    id bigint NOT NULL,
    user_id bigint NOT NULL,
    room_slug character varying(100) NOT NULL,
    item_type character varying(50) NOT NULL,
    item_id bigint NOT NULL,
    title character varying(500),
    excerpt text,
    href character varying(500),
    thumbnail_url character varying(500),
    created_at timestamp with time zone DEFAULT now()
);


--
-- Name: user_favorites_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.user_favorites_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: user_favorites_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.user_favorites_id_seq OWNED BY public.user_favorites.id;


--
-- Name: user_indicator_access; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.user_indicator_access (
    id bigint NOT NULL,
    user_id bigint NOT NULL,
    indicator_id bigint NOT NULL,
    granted_at timestamp with time zone DEFAULT now(),
    access_expires_at timestamp with time zone,
    is_active boolean DEFAULT true,
    access_source character varying(50),
    last_download_at timestamp with time zone,
    total_downloads integer DEFAULT 0
);


--
-- Name: user_indicator_access_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.user_indicator_access_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: user_indicator_access_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.user_indicator_access_id_seq OWNED BY public.user_indicator_access.id;


--
-- Name: user_indicator_ownership; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.user_indicator_ownership (
    id bigint NOT NULL,
    user_id bigint NOT NULL,
    indicator_id bigint NOT NULL,
    granted_at timestamp with time zone DEFAULT now(),
    expires_at timestamp with time zone,
    source character varying(50)
);


--
-- Name: user_indicator_ownership_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.user_indicator_ownership_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: user_indicator_ownership_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.user_indicator_ownership_id_seq OWNED BY public.user_indicator_ownership.id;


--
-- Name: user_indicators; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.user_indicators (
    id bigint NOT NULL,
    user_id bigint NOT NULL,
    indicator_id bigint NOT NULL,
    purchased_at timestamp without time zone DEFAULT now(),
    license_key character varying(255),
    expires_at timestamp without time zone,
    created_at timestamp without time zone DEFAULT now(),
    updated_at timestamp without time zone DEFAULT now()
);


--
-- Name: user_indicators_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.user_indicators_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: user_indicators_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.user_indicators_id_seq OWNED BY public.user_indicators.id;


--
-- Name: user_lesson_progress; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.user_lesson_progress (
    id bigint NOT NULL,
    user_id bigint NOT NULL,
    course_id bigint NOT NULL,
    lesson_id bigint NOT NULL,
    watch_position_seconds integer DEFAULT 0,
    watch_time_total_seconds integer DEFAULT 0,
    watch_percent integer DEFAULT 0,
    is_completed boolean DEFAULT false,
    completed_at timestamp with time zone,
    notes text,
    bookmarked boolean DEFAULT false,
    first_watched_at timestamp with time zone DEFAULT now(),
    last_watched_at timestamp with time zone DEFAULT now()
);


--
-- Name: user_lesson_progress_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.user_lesson_progress_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: user_lesson_progress_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.user_lesson_progress_id_seq OWNED BY public.user_lesson_progress.id;


--
-- Name: user_member_segments; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.user_member_segments (
    id bigint NOT NULL,
    user_id bigint NOT NULL,
    segment_id bigint NOT NULL,
    added_by bigint,
    added_at timestamp with time zone DEFAULT now()
);


--
-- Name: user_member_segments_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.user_member_segments_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: user_member_segments_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.user_member_segments_id_seq OWNED BY public.user_member_segments.id;


--
-- Name: user_member_tags; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.user_member_tags (
    id bigint NOT NULL,
    user_id bigint NOT NULL,
    tag_id bigint NOT NULL,
    added_by bigint,
    added_at timestamp with time zone DEFAULT now()
);


--
-- Name: user_member_tags_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.user_member_tags_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: user_member_tags_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.user_member_tags_id_seq OWNED BY public.user_member_tags.id;


--
-- Name: user_memberships; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.user_memberships (
    id bigint NOT NULL,
    user_id bigint NOT NULL,
    plan_id bigint NOT NULL,
    starts_at timestamp without time zone DEFAULT now() NOT NULL,
    expires_at timestamp without time zone,
    cancelled_at timestamp without time zone,
    status character varying(20) DEFAULT 'pending'::character varying,
    payment_provider character varying(50),
    stripe_subscription_id character varying(255),
    stripe_customer_id character varying(255),
    current_period_start timestamp without time zone,
    current_period_end timestamp without time zone,
    cancel_at_period_end boolean DEFAULT false,
    created_at timestamp without time zone DEFAULT now(),
    updated_at timestamp without time zone DEFAULT now(),
    deleted_at timestamp with time zone,
    deleted_by bigint,
    trial_ends_at timestamp without time zone,
    grace_period_end timestamp without time zone,
    renewal_reminder_sent_at timestamp with time zone,
    trial_ending_reminder_sent_at timestamp with time zone,
    payment_failure_count integer DEFAULT 0,
    last_payment_failure timestamp with time zone
);


--
-- Name: COLUMN user_memberships.renewal_reminder_sent_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.user_memberships.renewal_reminder_sent_at IS 'Timestamp when renewal reminder email was sent (prevents duplicate sends)';


--
-- Name: COLUMN user_memberships.trial_ending_reminder_sent_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.user_memberships.trial_ending_reminder_sent_at IS 'Timestamp when trial ending notification was sent (prevents duplicate sends)';


--
-- Name: COLUMN user_memberships.payment_failure_count; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.user_memberships.payment_failure_count IS 'Number of consecutive payment failures for this subscription';


--
-- Name: COLUMN user_memberships.last_payment_failure; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.user_memberships.last_payment_failure IS 'Timestamp of the most recent payment failure';


--
-- Name: user_memberships_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.user_memberships_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: user_memberships_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.user_memberships_id_seq OWNED BY public.user_memberships.id;


--
-- Name: user_mfa_secrets; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.user_mfa_secrets (
    id bigint NOT NULL,
    user_id bigint NOT NULL,
    totp_secret character varying(64) NOT NULL,
    totp_verified_at timestamp with time zone,
    backup_codes jsonb DEFAULT '[]'::jsonb,
    backup_codes_generated_at timestamp with time zone,
    backup_codes_used integer DEFAULT 0,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now()
);


--
-- Name: TABLE user_mfa_secrets; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.user_mfa_secrets IS 'ICT7: TOTP secrets and backup codes for MFA';


--
-- Name: user_mfa_secrets_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.user_mfa_secrets_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: user_mfa_secrets_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.user_mfa_secrets_id_seq OWNED BY public.user_mfa_secrets.id;


--
-- Name: user_products; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.user_products (
    id bigint NOT NULL,
    user_id bigint NOT NULL,
    product_id bigint NOT NULL,
    purchased_at timestamp without time zone DEFAULT now(),
    order_id bigint,
    created_at timestamp without time zone DEFAULT now(),
    updated_at timestamp without time zone DEFAULT now()
);


--
-- Name: user_products_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.user_products_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: user_products_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.user_products_id_seq OWNED BY public.user_products.id;


--
-- Name: user_quiz_attempts; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.user_quiz_attempts (
    id bigint NOT NULL,
    user_id bigint NOT NULL,
    quiz_id bigint NOT NULL,
    enrollment_id bigint NOT NULL,
    score integer,
    max_score integer NOT NULL,
    score_percent double precision,
    passed boolean,
    started_at timestamp without time zone DEFAULT now(),
    completed_at timestamp without time zone,
    time_spent_seconds integer,
    attempt_number integer DEFAULT 1,
    answers jsonb
);


--
-- Name: user_quiz_attempts_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.user_quiz_attempts_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: user_quiz_attempts_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.user_quiz_attempts_id_seq OWNED BY public.user_quiz_attempts.id;


--
-- Name: user_sessions; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.user_sessions (
    id bigint NOT NULL,
    user_id bigint,
    ip_address character varying(45),
    user_agent text,
    payload text,
    last_activity integer,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now(),
    device_fingerprint character varying(64),
    is_mfa_verified boolean DEFAULT false,
    mfa_verified_at timestamp with time zone
);


--
-- Name: COLUMN user_sessions.is_mfa_verified; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.user_sessions.is_mfa_verified IS 'ICT7: Session MFA verification status';


--
-- Name: user_sessions_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.user_sessions_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: user_sessions_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.user_sessions_id_seq OWNED BY public.user_sessions.id;


--
-- Name: user_status; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.user_status (
    id bigint NOT NULL,
    user_id bigint NOT NULL,
    status character varying(50) DEFAULT 'active'::character varying,
    status_reason text,
    banned_at timestamp with time zone,
    banned_by bigint,
    ban_reason text,
    ban_expires_at timestamp with time zone,
    last_login_at timestamp with time zone,
    last_activity_at timestamp with time zone,
    login_count integer DEFAULT 0,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now()
);


--
-- Name: user_status_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.user_status_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: user_status_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.user_status_id_seq OWNED BY public.user_status.id;


--
-- Name: user_teams; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.user_teams (
    id bigint NOT NULL,
    user_id bigint NOT NULL,
    team_id bigint NOT NULL,
    role character varying(50) DEFAULT 'member'::character varying,
    joined_at timestamp with time zone DEFAULT now()
);


--
-- Name: user_teams_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.user_teams_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: user_teams_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.user_teams_id_seq OWNED BY public.user_teams.id;


--
-- Name: users; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.users (
    id bigint NOT NULL,
    name character varying(255) NOT NULL,
    email character varying(255) NOT NULL,
    password_hash character varying(255) NOT NULL,
    role character varying(50) DEFAULT 'user'::character varying NOT NULL,
    email_verified_at timestamp without time zone,
    avatar_url character varying(500),
    mfa_enabled boolean DEFAULT false,
    remember_token character varying(100),
    stripe_customer_id character varying(255),
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone DEFAULT now() NOT NULL,
    is_active boolean DEFAULT true,
    last_login_at timestamp without time zone,
    last_login_ip character varying(45),
    timezone character varying(50) DEFAULT 'UTC'::character varying,
    locale character varying(10) DEFAULT 'en'::character varying,
    metadata jsonb,
    uuid_id uuid DEFAULT gen_random_uuid(),
    google_id character varying(255),
    apple_id character varying(255),
    oauth_provider character varying(50),
    oauth_linked_at timestamp without time zone,
    oauth_metadata jsonb DEFAULT '{}'::jsonb,
    deleted_at timestamp with time zone,
    deleted_by bigint,
    deletion_reason character varying(255)
);


--
-- Name: COLUMN users.google_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.users.google_id IS 'Google OAuth2 subject identifier (sub claim)';


--
-- Name: COLUMN users.apple_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.users.apple_id IS 'Apple Sign-In subject identifier (sub claim)';


--
-- Name: COLUMN users.oauth_provider; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.users.oauth_provider IS 'Primary OAuth provider used for registration: google, apple, or null for email';


--
-- Name: COLUMN users.oauth_linked_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.users.oauth_linked_at IS 'Timestamp when OAuth was first linked to this account';


--
-- Name: COLUMN users.oauth_metadata; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.users.oauth_metadata IS 'Additional OAuth provider data (profile picture URL, locale, etc.)';


--
-- Name: COLUMN users.deleted_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.users.deleted_at IS 'ICT7: Soft delete timestamp for compliance';


--
-- Name: users_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.users_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: users_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.users_id_seq OWNED BY public.users.id;


--
-- Name: video_chapters; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.video_chapters (
    id bigint NOT NULL,
    video_id bigint NOT NULL,
    title character varying(255) NOT NULL,
    description text,
    start_time_seconds integer DEFAULT 0 NOT NULL,
    end_time_seconds integer,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now()
);


--
-- Name: video_chapters_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.video_chapters_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: video_chapters_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.video_chapters_id_seq OWNED BY public.video_chapters.id;


--
-- Name: video_room_assignments; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.video_room_assignments (
    id bigint NOT NULL,
    video_id bigint NOT NULL,
    room_id bigint NOT NULL,
    room_slug character varying(100),
    is_primary boolean DEFAULT false,
    sort_order integer DEFAULT 0,
    assigned_at timestamp with time zone DEFAULT now()
);


--
-- Name: video_room_assignments_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.video_room_assignments_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: video_room_assignments_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.video_room_assignments_id_seq OWNED BY public.video_room_assignments.id;


--
-- Name: video_series; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.video_series (
    id bigint NOT NULL,
    title character varying(255) NOT NULL,
    slug character varying(255) NOT NULL,
    description text,
    is_published boolean DEFAULT false,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now()
);


--
-- Name: video_series_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.video_series_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: video_series_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.video_series_id_seq OWNED BY public.video_series.id;


--
-- Name: video_transcripts; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.video_transcripts (
    id bigint NOT NULL,
    video_id bigint NOT NULL,
    language_code character varying(10) DEFAULT 'en'::character varying NOT NULL,
    language_label character varying(100) DEFAULT 'English'::character varying NOT NULL,
    transcript_type character varying(50) DEFAULT 'subtitles'::character varying NOT NULL,
    vtt_url text,
    srt_url text,
    raw_text text,
    is_auto_generated boolean DEFAULT false NOT NULL,
    is_default boolean DEFAULT false NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone DEFAULT now() NOT NULL,
    CONSTRAINT video_transcripts_transcript_type_check CHECK (((transcript_type)::text = ANY ((ARRAY['subtitles'::character varying, 'captions'::character varying, 'descriptions'::character varying])::text[])))
);


--
-- Name: TABLE video_transcripts; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.video_transcripts IS 'Video transcripts and captions for accessibility';


--
-- Name: COLUMN video_transcripts.transcript_type; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.video_transcripts.transcript_type IS 'Type: subtitles, captions (for deaf), or descriptions (for blind)';


--
-- Name: video_transcripts_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.video_transcripts_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: video_transcripts_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.video_transcripts_id_seq OWNED BY public.video_transcripts.id;


--
-- Name: video_watch_progress; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.video_watch_progress (
    id bigint NOT NULL,
    user_id bigint NOT NULL,
    video_id bigint NOT NULL,
    current_time_seconds integer DEFAULT 0 NOT NULL,
    completion_percent integer DEFAULT 0 NOT NULL,
    completed boolean DEFAULT false NOT NULL,
    last_watched_at timestamp without time zone DEFAULT now() NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone DEFAULT now() NOT NULL,
    CONSTRAINT video_watch_progress_completion_percent_check CHECK (((completion_percent >= 0) AND (completion_percent <= 100)))
);


--
-- Name: TABLE video_watch_progress; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.video_watch_progress IS 'Tracks user video watch progress for resume and analytics';


--
-- Name: COLUMN video_watch_progress.current_time_seconds; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.video_watch_progress.current_time_seconds IS 'Current playback position in seconds';


--
-- Name: COLUMN video_watch_progress.completion_percent; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.video_watch_progress.completion_percent IS 'Percentage of video watched (0-100)';


--
-- Name: COLUMN video_watch_progress.completed; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.video_watch_progress.completed IS 'Whether user watched at least 90% of video';


--
-- Name: video_watch_progress_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.video_watch_progress_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: video_watch_progress_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.video_watch_progress_id_seq OWNED BY public.video_watch_progress.id;


--
-- Name: videos; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.videos (
    id bigint NOT NULL,
    title character varying(255) NOT NULL,
    slug character varying(255) NOT NULL,
    description text,
    video_url character varying(500) NOT NULL,
    thumbnail character varying(500),
    duration_seconds integer DEFAULT 0,
    is_public boolean DEFAULT true,
    views_count integer DEFAULT 0,
    created_at timestamp without time zone DEFAULT now(),
    updated_at timestamp without time zone DEFAULT now()
);


--
-- Name: videos_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.videos_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: videos_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.videos_id_seq OWNED BY public.videos.id;


--
-- Name: watchlist_entries; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.watchlist_entries (
    id bigint NOT NULL,
    room_id bigint NOT NULL,
    room_slug character varying(100) NOT NULL,
    week_of date DEFAULT CURRENT_DATE NOT NULL,
    ticker character varying(10) NOT NULL,
    company_name character varying(255),
    bias character varying(20) DEFAULT 'NEUTRAL'::character varying,
    entry_price character varying(20),
    target_price character varying(20),
    stop_price character varying(20),
    options_play text,
    notes text,
    chart_url character varying(500),
    sort_order integer DEFAULT 0,
    is_active boolean DEFAULT true,
    created_by bigint,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now()
);


--
-- Name: watchlist_entries_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.watchlist_entries_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: watchlist_entries_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.watchlist_entries_id_seq OWNED BY public.watchlist_entries.id;


--
-- Name: webhook_deliveries; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.webhook_deliveries (
    id bigint NOT NULL,
    webhook_id bigint,
    event_type character varying(100) NOT NULL,
    payload jsonb NOT NULL,
    status character varying(20) DEFAULT 'pending'::character varying NOT NULL,
    attempt_count integer DEFAULT 0 NOT NULL,
    response_status integer,
    response_body text,
    response_time_ms integer,
    error_message text,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    delivered_at timestamp with time zone,
    next_retry_at timestamp with time zone,
    attempts integer DEFAULT 0
);


--
-- Name: webhook_deliveries_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.webhook_deliveries_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: webhook_deliveries_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.webhook_deliveries_id_seq OWNED BY public.webhook_deliveries.id;


--
-- Name: webhook_events; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.webhook_events (
    event_id text NOT NULL,
    event_type text NOT NULL,
    received_at timestamp with time zone DEFAULT now() NOT NULL,
    processed_at timestamp with time zone,
    error text,
    payload jsonb NOT NULL
);


--
-- Name: webhooks; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.webhooks (
    id bigint NOT NULL,
    name character varying(100) NOT NULL,
    description text,
    url character varying(500) NOT NULL,
    secret character varying(255),
    events text[] DEFAULT ARRAY['content.created'::text, 'content.updated'::text, 'content.published'::text] NOT NULL,
    content_types text[],
    is_active boolean DEFAULT true NOT NULL,
    retry_count integer DEFAULT 3 NOT NULL,
    timeout_seconds integer DEFAULT 30 NOT NULL,
    headers jsonb DEFAULT '{}'::jsonb,
    created_by bigint,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: TABLE webhooks; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.webhooks IS 'Webhook configurations for external integrations';


--
-- Name: webhooks_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.webhooks_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: webhooks_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.webhooks_id_seq OWNED BY public.webhooks.id;


--
-- Name: workflow_definitions; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.workflow_definitions (
    id bigint NOT NULL,
    name character varying(100) NOT NULL,
    description text,
    stages jsonb DEFAULT '[]'::jsonb NOT NULL,
    is_active boolean DEFAULT true NOT NULL,
    applies_to text[] DEFAULT ARRAY['post'::text],
    created_by bigint,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: TABLE workflow_definitions; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.workflow_definitions IS 'Configurable workflow stages for content approval';


--
-- Name: workflow_definitions_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.workflow_definitions_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: workflow_definitions_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.workflow_definitions_id_seq OWNED BY public.workflow_definitions.id;


--
-- Name: workflow_transitions; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.workflow_transitions (
    id bigint NOT NULL,
    workflow_status_id bigint,
    from_stage character varying(50),
    to_stage character varying(50) NOT NULL,
    transitioned_by bigint,
    comment text,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: workflow_transitions_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.workflow_transitions_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: workflow_transitions_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.workflow_transitions_id_seq OWNED BY public.workflow_transitions.id;


--
-- Name: cms_audit_log_2026_05; Type: TABLE ATTACH; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_audit_log ATTACH PARTITION public.cms_audit_log_2026_05 FOR VALUES FROM ('2026-05-01 00:00:00+00') TO ('2026-06-01 00:00:00+00');


--
-- Name: cms_audit_log_2026_06; Type: TABLE ATTACH; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_audit_log ATTACH PARTITION public.cms_audit_log_2026_06 FOR VALUES FROM ('2026-06-01 00:00:00+00') TO ('2026-07-01 00:00:00+00');


--
-- Name: cms_audit_log_2026_07; Type: TABLE ATTACH; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_audit_log ATTACH PARTITION public.cms_audit_log_2026_07 FOR VALUES FROM ('2026-07-01 00:00:00+00') TO ('2026-08-01 00:00:00+00');


--
-- Name: cms_audit_log_2026_08; Type: TABLE ATTACH; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_audit_log ATTACH PARTITION public.cms_audit_log_2026_08 FOR VALUES FROM ('2026-08-01 00:00:00+00') TO ('2026-09-01 00:00:00+00');


--
-- Name: cms_audit_log_2026_09; Type: TABLE ATTACH; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_audit_log ATTACH PARTITION public.cms_audit_log_2026_09 FOR VALUES FROM ('2026-09-01 00:00:00+00') TO ('2026-10-01 00:00:00+00');


--
-- Name: cms_audit_log_2026_10; Type: TABLE ATTACH; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_audit_log ATTACH PARTITION public.cms_audit_log_2026_10 FOR VALUES FROM ('2026-10-01 00:00:00+00') TO ('2026-11-01 00:00:00+00');


--
-- Name: cms_audit_log_2026_11; Type: TABLE ATTACH; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_audit_log ATTACH PARTITION public.cms_audit_log_2026_11 FOR VALUES FROM ('2026-11-01 00:00:00+00') TO ('2026-12-01 00:00:00+00');


--
-- Name: cms_audit_log_2026_12; Type: TABLE ATTACH; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_audit_log ATTACH PARTITION public.cms_audit_log_2026_12 FOR VALUES FROM ('2026-12-01 00:00:00+00') TO ('2027-01-01 00:00:00+00');


--
-- Name: cms_audit_log_2027_01; Type: TABLE ATTACH; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_audit_log ATTACH PARTITION public.cms_audit_log_2027_01 FOR VALUES FROM ('2027-01-01 00:00:00+00') TO ('2027-02-01 00:00:00+00');


--
-- Name: cms_audit_log_2027_02; Type: TABLE ATTACH; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_audit_log ATTACH PARTITION public.cms_audit_log_2027_02 FOR VALUES FROM ('2027-02-01 00:00:00+00') TO ('2027-03-01 00:00:00+00');


--
-- Name: cms_audit_log_2027_03; Type: TABLE ATTACH; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_audit_log ATTACH PARTITION public.cms_audit_log_2027_03 FOR VALUES FROM ('2027-03-01 00:00:00+00') TO ('2027-04-01 00:00:00+00');


--
-- Name: cms_audit_log_2027_04; Type: TABLE ATTACH; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_audit_log ATTACH PARTITION public.cms_audit_log_2027_04 FOR VALUES FROM ('2027-04-01 00:00:00+00') TO ('2027-05-01 00:00:00+00');


--
-- Name: cms_audit_log_2027_05; Type: TABLE ATTACH; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_audit_log ATTACH PARTITION public.cms_audit_log_2027_05 FOR VALUES FROM ('2027-05-01 00:00:00+00') TO ('2027-06-01 00:00:00+00');


--
-- Name: admin_audit_logs id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.admin_audit_logs ALTER COLUMN id SET DEFAULT nextval('public.admin_audit_logs_id_seq'::regclass);


--
-- Name: analytics_events id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.analytics_events ALTER COLUMN id SET DEFAULT nextval('public.analytics_events_id_seq'::regclass);


--
-- Name: application_settings id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.application_settings ALTER COLUMN id SET DEFAULT nextval('public.application_settings_id_seq'::regclass);


--
-- Name: audit_logs id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.audit_logs ALTER COLUMN id SET DEFAULT nextval('public.audit_logs_id_seq'::regclass);


--
-- Name: bunny_uploads id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.bunny_uploads ALTER COLUMN id SET DEFAULT nextval('public.bunny_uploads_id_seq'::regclass);


--
-- Name: categories id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.categories ALTER COLUMN id SET DEFAULT nextval('public.categories_id_seq'::regclass);


--
-- Name: contacts id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.contacts ALTER COLUMN id SET DEFAULT nextval('public.contacts_id_seq'::regclass);


--
-- Name: content_translations id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.content_translations ALTER COLUMN id SET DEFAULT nextval('public.content_translations_id_seq'::regclass);


--
-- Name: content_versions id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.content_versions ALTER COLUMN id SET DEFAULT nextval('public.content_versions_id_seq'::regclass);


--
-- Name: content_workflow_status id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.content_workflow_status ALTER COLUMN id SET DEFAULT nextval('public.content_workflow_status_id_seq'::regclass);


--
-- Name: coupons id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.coupons ALTER COLUMN id SET DEFAULT nextval('public.coupons_id_seq'::regclass);


--
-- Name: course_categories id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_categories ALTER COLUMN id SET DEFAULT nextval('public.course_categories_id_seq'::regclass);


--
-- Name: course_downloads id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_downloads ALTER COLUMN id SET DEFAULT nextval('public.course_downloads_id_seq'::regclass);


--
-- Name: course_lessons id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_lessons ALTER COLUMN id SET DEFAULT nextval('public.course_lessons_id_seq'::regclass);


--
-- Name: course_live_sessions id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_live_sessions ALTER COLUMN id SET DEFAULT nextval('public.course_live_sessions_id_seq'::regclass);


--
-- Name: course_modules id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_modules ALTER COLUMN id SET DEFAULT nextval('public.course_modules_id_seq'::regclass);


--
-- Name: course_modules_v2 id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_modules_v2 ALTER COLUMN id SET DEFAULT nextval('public.course_modules_v2_id_seq'::regclass);


--
-- Name: course_quizzes id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_quizzes ALTER COLUMN id SET DEFAULT nextval('public.course_quizzes_id_seq'::regclass);


--
-- Name: course_resources id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_resources ALTER COLUMN id SET DEFAULT nextval('public.course_resources_id_seq'::regclass);


--
-- Name: course_reviews id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_reviews ALTER COLUMN id SET DEFAULT nextval('public.course_reviews_id_seq'::regclass);


--
-- Name: course_sections id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_sections ALTER COLUMN id SET DEFAULT nextval('public.course_sections_id_seq'::regclass);


--
-- Name: course_tags id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_tags ALTER COLUMN id SET DEFAULT nextval('public.course_tags_id_seq'::regclass);


--
-- Name: courses_enhanced id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.courses_enhanced ALTER COLUMN id SET DEFAULT nextval('public.courses_enhanced_id_seq'::regclass);


--
-- Name: crm_abandoned_carts id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_abandoned_carts ALTER COLUMN id SET DEFAULT nextval('public.crm_abandoned_carts_id_seq'::regclass);


--
-- Name: crm_automations id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_automations ALTER COLUMN id SET DEFAULT nextval('public.crm_automations_id_seq'::regclass);


--
-- Name: crm_campaigns id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_campaigns ALTER COLUMN id SET DEFAULT nextval('public.crm_campaigns_id_seq'::regclass);


--
-- Name: crm_companies id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_companies ALTER COLUMN id SET DEFAULT nextval('public.crm_companies_id_seq'::regclass);


--
-- Name: crm_deal_activities id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_deal_activities ALTER COLUMN id SET DEFAULT nextval('public.crm_deal_activities_id_seq'::regclass);


--
-- Name: crm_deal_stage_history id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_deal_stage_history ALTER COLUMN id SET DEFAULT nextval('public.crm_deal_stage_history_id_seq'::regclass);


--
-- Name: crm_deals id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_deals ALTER COLUMN id SET DEFAULT nextval('public.crm_deals_id_seq'::regclass);


--
-- Name: crm_lists id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_lists ALTER COLUMN id SET DEFAULT nextval('public.crm_lists_id_seq'::regclass);


--
-- Name: crm_pipeline_stages id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_pipeline_stages ALTER COLUMN id SET DEFAULT nextval('public.crm_pipeline_stages_id_seq'::regclass);


--
-- Name: crm_pipelines id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_pipelines ALTER COLUMN id SET DEFAULT nextval('public.crm_pipelines_id_seq'::regclass);


--
-- Name: crm_recurring_campaigns id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_recurring_campaigns ALTER COLUMN id SET DEFAULT nextval('public.crm_recurring_campaigns_id_seq'::regclass);


--
-- Name: crm_segments id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_segments ALTER COLUMN id SET DEFAULT nextval('public.crm_segments_id_seq'::regclass);


--
-- Name: crm_sequences id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_sequences ALTER COLUMN id SET DEFAULT nextval('public.crm_sequences_id_seq'::regclass);


--
-- Name: crm_smart_links id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_smart_links ALTER COLUMN id SET DEFAULT nextval('public.crm_smart_links_id_seq'::regclass);


--
-- Name: crm_tags id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_tags ALTER COLUMN id SET DEFAULT nextval('public.crm_tags_id_seq'::regclass);


--
-- Name: crm_templates id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_templates ALTER COLUMN id SET DEFAULT nextval('public.crm_templates_id_seq'::regclass);


--
-- Name: crm_webhooks id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_webhooks ALTER COLUMN id SET DEFAULT nextval('public.crm_webhooks_id_seq'::regclass);


--
-- Name: departments id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.departments ALTER COLUMN id SET DEFAULT nextval('public.departments_id_seq'::regclass);


--
-- Name: email_campaigns id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.email_campaigns ALTER COLUMN id SET DEFAULT nextval('public.email_campaigns_id_seq'::regclass);


--
-- Name: email_logs id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.email_logs ALTER COLUMN id SET DEFAULT nextval('public.email_logs_id_seq'::regclass);


--
-- Name: email_templates id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.email_templates ALTER COLUMN id SET DEFAULT nextval('public.email_templates_id_seq'::regclass);


--
-- Name: failed_jobs id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.failed_jobs ALTER COLUMN id SET DEFAULT nextval('public.failed_jobs_id_seq'::regclass);


--
-- Name: form_submissions id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.form_submissions ALTER COLUMN id SET DEFAULT nextval('public.form_submissions_id_seq'::regclass);


--
-- Name: forms id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.forms ALTER COLUMN id SET DEFAULT nextval('public.forms_id_seq'::regclass);


--
-- Name: indicator_documentation id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.indicator_documentation ALTER COLUMN id SET DEFAULT nextval('public.indicator_documentation_id_seq'::regclass);


--
-- Name: indicator_download_log id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.indicator_download_log ALTER COLUMN id SET DEFAULT nextval('public.indicator_download_log_id_seq'::regclass);


--
-- Name: indicator_downloads id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.indicator_downloads ALTER COLUMN id SET DEFAULT nextval('public.indicator_downloads_id_seq'::regclass);


--
-- Name: indicator_files id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.indicator_files ALTER COLUMN id SET DEFAULT nextval('public.indicator_files_id_seq'::regclass);


--
-- Name: indicator_platform_files id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.indicator_platform_files ALTER COLUMN id SET DEFAULT nextval('public.indicator_platform_files_id_seq'::regclass);


--
-- Name: indicator_platforms id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.indicator_platforms ALTER COLUMN id SET DEFAULT nextval('public.indicator_platforms_id_seq'::regclass);


--
-- Name: indicator_tradingview_access id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.indicator_tradingview_access ALTER COLUMN id SET DEFAULT nextval('public.indicator_tradingview_access_id_seq'::regclass);


--
-- Name: indicator_videos id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.indicator_videos ALTER COLUMN id SET DEFAULT nextval('public.indicator_videos_id_seq'::regclass);


--
-- Name: indicators id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.indicators ALTER COLUMN id SET DEFAULT nextval('public.indicators_id_seq'::regclass);


--
-- Name: indicators_enhanced id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.indicators_enhanced ALTER COLUMN id SET DEFAULT nextval('public.indicators_enhanced_id_seq'::regclass);


--
-- Name: integration_webhooks id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.integration_webhooks ALTER COLUMN id SET DEFAULT nextval('public.integration_webhooks_id_seq'::regclass);


--
-- Name: invoices id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.invoices ALTER COLUMN id SET DEFAULT nextval('public.invoices_id_seq'::regclass);


--
-- Name: laravel_jobs_backup id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.laravel_jobs_backup ALTER COLUMN id SET DEFAULT nextval('public.jobs_id_seq'::regclass);


--
-- Name: locales id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.locales ALTER COLUMN id SET DEFAULT nextval('public.locales_id_seq'::regclass);


--
-- Name: media id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.media ALTER COLUMN id SET DEFAULT nextval('public.media_id_seq'::regclass);


--
-- Name: member_emails id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.member_emails ALTER COLUMN id SET DEFAULT nextval('public.member_emails_id_seq'::regclass);


--
-- Name: member_filters id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.member_filters ALTER COLUMN id SET DEFAULT nextval('public.member_filters_id_seq'::regclass);


--
-- Name: member_notes id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.member_notes ALTER COLUMN id SET DEFAULT nextval('public.member_notes_id_seq'::regclass);


--
-- Name: member_segments id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.member_segments ALTER COLUMN id SET DEFAULT nextval('public.member_segments_id_seq'::regclass);


--
-- Name: member_tags id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.member_tags ALTER COLUMN id SET DEFAULT nextval('public.member_tags_id_seq'::regclass);


--
-- Name: membership_features id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.membership_features ALTER COLUMN id SET DEFAULT nextval('public.membership_features_id_seq'::regclass);


--
-- Name: membership_plan_price_history id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.membership_plan_price_history ALTER COLUMN id SET DEFAULT nextval('public.membership_plan_price_history_id_seq'::regclass);


--
-- Name: membership_plans id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.membership_plans ALTER COLUMN id SET DEFAULT nextval('public.membership_plans_id_seq'::regclass);


--
-- Name: mfa_attempts id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.mfa_attempts ALTER COLUMN id SET DEFAULT nextval('public.mfa_attempts_id_seq'::regclass);


--
-- Name: newsletter_subscribers id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.newsletter_subscribers ALTER COLUMN id SET DEFAULT nextval('public.newsletter_subscribers_id_seq'::regclass);


--
-- Name: order_items id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.order_items ALTER COLUMN id SET DEFAULT nextval('public.order_items_id_seq'::regclass);


--
-- Name: orders id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.orders ALTER COLUMN id SET DEFAULT nextval('public.orders_id_seq'::regclass);


--
-- Name: page_layout_versions id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.page_layout_versions ALTER COLUMN id SET DEFAULT nextval('public.page_layout_versions_id_seq'::regclass);


--
-- Name: page_layouts id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.page_layouts ALTER COLUMN id SET DEFAULT nextval('public.page_layouts_id_seq'::regclass);


--
-- Name: password_resets id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.password_resets ALTER COLUMN id SET DEFAULT nextval('public.password_resets_id_seq'::regclass);


--
-- Name: payment_disputes id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.payment_disputes ALTER COLUMN id SET DEFAULT nextval('public.payment_disputes_id_seq'::regclass);


--
-- Name: popup_ab_tests id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.popup_ab_tests ALTER COLUMN id SET DEFAULT nextval('public.popup_ab_tests_id_seq'::regclass);


--
-- Name: popup_events id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.popup_events ALTER COLUMN id SET DEFAULT nextval('public.popup_events_id_seq'::regclass);


--
-- Name: popup_form_submissions id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.popup_form_submissions ALTER COLUMN id SET DEFAULT nextval('public.popup_form_submissions_id_seq'::regclass);


--
-- Name: popups id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.popups ALTER COLUMN id SET DEFAULT nextval('public.popups_id_seq'::regclass);


--
-- Name: posts id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.posts ALTER COLUMN id SET DEFAULT nextval('public.posts_id_seq'::regclass);


--
-- Name: preview_tokens id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.preview_tokens ALTER COLUMN id SET DEFAULT nextval('public.preview_tokens_id_seq'::regclass);


--
-- Name: products id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.products ALTER COLUMN id SET DEFAULT nextval('public.products_id_seq'::regclass);


--
-- Name: quiz_answers id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.quiz_answers ALTER COLUMN id SET DEFAULT nextval('public.quiz_answers_id_seq'::regclass);


--
-- Name: quiz_questions id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.quiz_questions ALTER COLUMN id SET DEFAULT nextval('public.quiz_questions_id_seq'::regclass);


--
-- Name: reconciliation_log id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.reconciliation_log ALTER COLUMN id SET DEFAULT nextval('public.reconciliation_log_id_seq'::regclass);


--
-- Name: redirects id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.redirects ALTER COLUMN id SET DEFAULT nextval('public.redirects_id_seq'::regclass);


--
-- Name: resource_access_log id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.resource_access_log ALTER COLUMN id SET DEFAULT nextval('public.resource_access_log_id_seq'::regclass);


--
-- Name: resource_download_logs id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.resource_download_logs ALTER COLUMN id SET DEFAULT nextval('public.resource_download_logs_id_seq'::regclass);


--
-- Name: resource_favorites id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.resource_favorites ALTER COLUMN id SET DEFAULT nextval('public.resource_favorites_id_seq'::regclass);


--
-- Name: resource_upload_limits id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.resource_upload_limits ALTER COLUMN id SET DEFAULT nextval('public.resource_upload_limits_id_seq'::regclass);


--
-- Name: review_helpful_votes id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.review_helpful_votes ALTER COLUMN id SET DEFAULT nextval('public.review_helpful_votes_id_seq'::regclass);


--
-- Name: room_alerts id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.room_alerts ALTER COLUMN id SET DEFAULT nextval('public.room_alerts_id_seq'::regclass);


--
-- Name: room_resources id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.room_resources ALTER COLUMN id SET DEFAULT nextval('public.room_resources_id_seq'::regclass);


--
-- Name: room_sections id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.room_sections ALTER COLUMN id SET DEFAULT nextval('public.room_sections_id_seq'::regclass);


--
-- Name: room_stats_cache id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.room_stats_cache ALTER COLUMN id SET DEFAULT nextval('public.room_stats_cache_id_seq'::regclass);


--
-- Name: room_trade_plans id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.room_trade_plans ALTER COLUMN id SET DEFAULT nextval('public.room_trade_plans_id_seq'::regclass);


--
-- Name: room_traders id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.room_traders ALTER COLUMN id SET DEFAULT nextval('public.room_traders_id_seq'::regclass);


--
-- Name: room_trades id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.room_trades ALTER COLUMN id SET DEFAULT nextval('public.room_trades_id_seq'::regclass);


--
-- Name: room_weekly_videos id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.room_weekly_videos ALTER COLUMN id SET DEFAULT nextval('public.room_weekly_videos_id_seq'::regclass);


--
-- Name: schedule_exceptions id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.schedule_exceptions ALTER COLUMN id SET DEFAULT nextval('public.schedule_exceptions_id_seq'::regclass);


--
-- Name: scheduled_content id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.scheduled_content ALTER COLUMN id SET DEFAULT nextval('public.scheduled_content_id_seq'::regclass);


--
-- Name: security_events id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.security_events ALTER COLUMN id SET DEFAULT nextval('public.security_events_id_seq'::regclass);


--
-- Name: service_connections id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.service_connections ALTER COLUMN id SET DEFAULT nextval('public.service_connections_id_seq'::regclass);


--
-- Name: settings id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.settings ALTER COLUMN id SET DEFAULT nextval('public.settings_id_seq'::regclass);


--
-- Name: stock_lists id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.stock_lists ALTER COLUMN id SET DEFAULT nextval('public.stock_lists_id_seq'::regclass);


--
-- Name: tags id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.tags ALTER COLUMN id SET DEFAULT nextval('public.tags_id_seq'::regclass);


--
-- Name: teams id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.teams ALTER COLUMN id SET DEFAULT nextval('public.teams_id_seq'::regclass);


--
-- Name: traders id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.traders ALTER COLUMN id SET DEFAULT nextval('public.traders_id_seq'::regclass);


--
-- Name: trading_room_schedules id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.trading_room_schedules ALTER COLUMN id SET DEFAULT nextval('public.trading_room_schedules_id_seq'::regclass);


--
-- Name: trading_rooms id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.trading_rooms ALTER COLUMN id SET DEFAULT nextval('public.trading_rooms_id_seq'::regclass);


--
-- Name: unified_videos id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.unified_videos ALTER COLUMN id SET DEFAULT nextval('public.unified_videos_id_seq'::regclass);


--
-- Name: user_activity_log id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_activity_log ALTER COLUMN id SET DEFAULT nextval('public.user_activity_log_id_seq'::regclass);


--
-- Name: user_coupons id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_coupons ALTER COLUMN id SET DEFAULT nextval('public.user_coupons_id_seq'::regclass);


--
-- Name: user_course_enrollments id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_course_enrollments ALTER COLUMN id SET DEFAULT nextval('public.user_course_enrollments_id_seq'::regclass);


--
-- Name: user_departments id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_departments ALTER COLUMN id SET DEFAULT nextval('public.user_departments_id_seq'::regclass);


--
-- Name: user_favorites id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_favorites ALTER COLUMN id SET DEFAULT nextval('public.user_favorites_id_seq'::regclass);


--
-- Name: user_indicator_access id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_indicator_access ALTER COLUMN id SET DEFAULT nextval('public.user_indicator_access_id_seq'::regclass);


--
-- Name: user_indicator_ownership id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_indicator_ownership ALTER COLUMN id SET DEFAULT nextval('public.user_indicator_ownership_id_seq'::regclass);


--
-- Name: user_indicators id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_indicators ALTER COLUMN id SET DEFAULT nextval('public.user_indicators_id_seq'::regclass);


--
-- Name: user_lesson_progress id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_lesson_progress ALTER COLUMN id SET DEFAULT nextval('public.user_lesson_progress_id_seq'::regclass);


--
-- Name: user_member_segments id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_member_segments ALTER COLUMN id SET DEFAULT nextval('public.user_member_segments_id_seq'::regclass);


--
-- Name: user_member_tags id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_member_tags ALTER COLUMN id SET DEFAULT nextval('public.user_member_tags_id_seq'::regclass);


--
-- Name: user_memberships id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_memberships ALTER COLUMN id SET DEFAULT nextval('public.user_memberships_id_seq'::regclass);


--
-- Name: user_mfa_secrets id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_mfa_secrets ALTER COLUMN id SET DEFAULT nextval('public.user_mfa_secrets_id_seq'::regclass);


--
-- Name: user_products id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_products ALTER COLUMN id SET DEFAULT nextval('public.user_products_id_seq'::regclass);


--
-- Name: user_quiz_attempts id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_quiz_attempts ALTER COLUMN id SET DEFAULT nextval('public.user_quiz_attempts_id_seq'::regclass);


--
-- Name: user_sessions id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_sessions ALTER COLUMN id SET DEFAULT nextval('public.user_sessions_id_seq'::regclass);


--
-- Name: user_status id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_status ALTER COLUMN id SET DEFAULT nextval('public.user_status_id_seq'::regclass);


--
-- Name: user_teams id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_teams ALTER COLUMN id SET DEFAULT nextval('public.user_teams_id_seq'::regclass);


--
-- Name: users id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.users ALTER COLUMN id SET DEFAULT nextval('public.users_id_seq'::regclass);


--
-- Name: video_chapters id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.video_chapters ALTER COLUMN id SET DEFAULT nextval('public.video_chapters_id_seq'::regclass);


--
-- Name: video_room_assignments id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.video_room_assignments ALTER COLUMN id SET DEFAULT nextval('public.video_room_assignments_id_seq'::regclass);


--
-- Name: video_series id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.video_series ALTER COLUMN id SET DEFAULT nextval('public.video_series_id_seq'::regclass);


--
-- Name: video_transcripts id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.video_transcripts ALTER COLUMN id SET DEFAULT nextval('public.video_transcripts_id_seq'::regclass);


--
-- Name: video_watch_progress id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.video_watch_progress ALTER COLUMN id SET DEFAULT nextval('public.video_watch_progress_id_seq'::regclass);


--
-- Name: videos id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.videos ALTER COLUMN id SET DEFAULT nextval('public.videos_id_seq'::regclass);


--
-- Name: watchlist_entries id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.watchlist_entries ALTER COLUMN id SET DEFAULT nextval('public.watchlist_entries_id_seq'::regclass);


--
-- Name: webhook_deliveries id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.webhook_deliveries ALTER COLUMN id SET DEFAULT nextval('public.webhook_deliveries_id_seq'::regclass);


--
-- Name: webhooks id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.webhooks ALTER COLUMN id SET DEFAULT nextval('public.webhooks_id_seq'::regclass);


--
-- Name: workflow_definitions id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.workflow_definitions ALTER COLUMN id SET DEFAULT nextval('public.workflow_definitions_id_seq'::regclass);


--
-- Name: workflow_transitions id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.workflow_transitions ALTER COLUMN id SET DEFAULT nextval('public.workflow_transitions_id_seq'::regclass);


--
-- Name: admin_audit_logs admin_audit_logs_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.admin_audit_logs
    ADD CONSTRAINT admin_audit_logs_pkey PRIMARY KEY (id);


--
-- Name: analytics_events analytics_events_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.analytics_events
    ADD CONSTRAINT analytics_events_pkey PRIMARY KEY (id);


--
-- Name: application_settings application_settings_key_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.application_settings
    ADD CONSTRAINT application_settings_key_key UNIQUE (key);


--
-- Name: application_settings application_settings_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.application_settings
    ADD CONSTRAINT application_settings_pkey PRIMARY KEY (id);


--
-- Name: audit_logs audit_logs_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.audit_logs
    ADD CONSTRAINT audit_logs_pkey PRIMARY KEY (id);


--
-- Name: bunny_uploads bunny_uploads_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.bunny_uploads
    ADD CONSTRAINT bunny_uploads_pkey PRIMARY KEY (id);


--
-- Name: bunny_uploads bunny_uploads_video_guid_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.bunny_uploads
    ADD CONSTRAINT bunny_uploads_video_guid_key UNIQUE (video_guid);


--
-- Name: categories categories_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.categories
    ADD CONSTRAINT categories_pkey PRIMARY KEY (id);


--
-- Name: categories categories_slug_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.categories
    ADD CONSTRAINT categories_slug_key UNIQUE (slug);


--
-- Name: cms_ai_assist_history cms_ai_assist_history_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_ai_assist_history
    ADD CONSTRAINT cms_ai_assist_history_pkey PRIMARY KEY (id);


--
-- Name: cms_asset_folders cms_asset_folders_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_asset_folders
    ADD CONSTRAINT cms_asset_folders_pkey PRIMARY KEY (id);


--
-- Name: cms_assets cms_assets_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_assets
    ADD CONSTRAINT cms_assets_pkey PRIMARY KEY (id);


--
-- Name: cms_assets cms_assets_storage_key_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_assets
    ADD CONSTRAINT cms_assets_storage_key_key UNIQUE (storage_key);


--
-- Name: cms_audit_log cms_audit_log_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_audit_log
    ADD CONSTRAINT cms_audit_log_pkey PRIMARY KEY (id, created_at);


--
-- Name: cms_audit_log_2026_05 cms_audit_log_2026_05_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_audit_log_2026_05
    ADD CONSTRAINT cms_audit_log_2026_05_pkey PRIMARY KEY (id, created_at);


--
-- Name: cms_audit_log_2026_06 cms_audit_log_2026_06_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_audit_log_2026_06
    ADD CONSTRAINT cms_audit_log_2026_06_pkey PRIMARY KEY (id, created_at);


--
-- Name: cms_audit_log_2026_07 cms_audit_log_2026_07_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_audit_log_2026_07
    ADD CONSTRAINT cms_audit_log_2026_07_pkey PRIMARY KEY (id, created_at);


--
-- Name: cms_audit_log_2026_08 cms_audit_log_2026_08_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_audit_log_2026_08
    ADD CONSTRAINT cms_audit_log_2026_08_pkey PRIMARY KEY (id, created_at);


--
-- Name: cms_audit_log_2026_09 cms_audit_log_2026_09_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_audit_log_2026_09
    ADD CONSTRAINT cms_audit_log_2026_09_pkey PRIMARY KEY (id, created_at);


--
-- Name: cms_audit_log_2026_10 cms_audit_log_2026_10_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_audit_log_2026_10
    ADD CONSTRAINT cms_audit_log_2026_10_pkey PRIMARY KEY (id, created_at);


--
-- Name: cms_audit_log_2026_11 cms_audit_log_2026_11_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_audit_log_2026_11
    ADD CONSTRAINT cms_audit_log_2026_11_pkey PRIMARY KEY (id, created_at);


--
-- Name: cms_audit_log_2026_12 cms_audit_log_2026_12_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_audit_log_2026_12
    ADD CONSTRAINT cms_audit_log_2026_12_pkey PRIMARY KEY (id, created_at);


--
-- Name: cms_audit_log_2027_01 cms_audit_log_2027_01_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_audit_log_2027_01
    ADD CONSTRAINT cms_audit_log_2027_01_pkey PRIMARY KEY (id, created_at);


--
-- Name: cms_audit_log_2027_02 cms_audit_log_2027_02_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_audit_log_2027_02
    ADD CONSTRAINT cms_audit_log_2027_02_pkey PRIMARY KEY (id, created_at);


--
-- Name: cms_audit_log_2027_03 cms_audit_log_2027_03_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_audit_log_2027_03
    ADD CONSTRAINT cms_audit_log_2027_03_pkey PRIMARY KEY (id, created_at);


--
-- Name: cms_audit_log_2027_04 cms_audit_log_2027_04_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_audit_log_2027_04
    ADD CONSTRAINT cms_audit_log_2027_04_pkey PRIMARY KEY (id, created_at);


--
-- Name: cms_audit_log_2027_05 cms_audit_log_2027_05_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_audit_log_2027_05
    ADD CONSTRAINT cms_audit_log_2027_05_pkey PRIMARY KEY (id, created_at);


--
-- Name: cms_audit_logs cms_audit_logs_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_audit_logs
    ADD CONSTRAINT cms_audit_logs_pkey PRIMARY KEY (id);


--
-- Name: cms_comment_notifications cms_comment_notifications_comment_id_user_id_notification_t_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_comment_notifications
    ADD CONSTRAINT cms_comment_notifications_comment_id_user_id_notification_t_key UNIQUE (comment_id, user_id, notification_type);


--
-- Name: cms_comment_notifications cms_comment_notifications_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_comment_notifications
    ADD CONSTRAINT cms_comment_notifications_pkey PRIMARY KEY (id);


--
-- Name: cms_comments cms_comments_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_comments
    ADD CONSTRAINT cms_comments_pkey PRIMARY KEY (id);


--
-- Name: cms_content cms_content_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_content
    ADD CONSTRAINT cms_content_pkey PRIMARY KEY (id);


--
-- Name: cms_content_relations cms_content_relations_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_content_relations
    ADD CONSTRAINT cms_content_relations_pkey PRIMARY KEY (id);


--
-- Name: cms_content_tags cms_content_tags_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_content_tags
    ADD CONSTRAINT cms_content_tags_pkey PRIMARY KEY (content_id, tag_id);


--
-- Name: cms_datasource_entries cms_datasource_entries_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_datasource_entries
    ADD CONSTRAINT cms_datasource_entries_pkey PRIMARY KEY (id);


--
-- Name: cms_datasource_entries cms_datasource_entries_unique; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_datasource_entries
    ADD CONSTRAINT cms_datasource_entries_unique UNIQUE (datasource_id, value, dimension);


--
-- Name: cms_datasources cms_datasources_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_datasources
    ADD CONSTRAINT cms_datasources_pkey PRIMARY KEY (id);


--
-- Name: cms_datasources cms_datasources_slug_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_datasources
    ADD CONSTRAINT cms_datasources_slug_key UNIQUE (slug);


--
-- Name: cms_navigation_menus cms_navigation_menus_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_navigation_menus
    ADD CONSTRAINT cms_navigation_menus_pkey PRIMARY KEY (id);


--
-- Name: cms_navigation_menus cms_navigation_menus_slug_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_navigation_menus
    ADD CONSTRAINT cms_navigation_menus_slug_key UNIQUE (slug);


--
-- Name: cms_offline_sync_queue cms_offline_sync_queue_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_offline_sync_queue
    ADD CONSTRAINT cms_offline_sync_queue_pkey PRIMARY KEY (id);


--
-- Name: cms_presets cms_presets_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_presets
    ADD CONSTRAINT cms_presets_pkey PRIMARY KEY (id);


--
-- Name: cms_preview_tokens cms_preview_tokens_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_preview_tokens
    ADD CONSTRAINT cms_preview_tokens_pkey PRIMARY KEY (id);


--
-- Name: cms_preview_tokens cms_preview_tokens_token_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_preview_tokens
    ADD CONSTRAINT cms_preview_tokens_token_key UNIQUE (token);


--
-- Name: cms_redirects cms_redirects_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_redirects
    ADD CONSTRAINT cms_redirects_pkey PRIMARY KEY (id);


--
-- Name: cms_release_items cms_release_items_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_release_items
    ADD CONSTRAINT cms_release_items_pkey PRIMARY KEY (id);


--
-- Name: cms_release_items cms_release_items_release_id_content_id_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_release_items
    ADD CONSTRAINT cms_release_items_release_id_content_id_key UNIQUE (release_id, content_id);


--
-- Name: cms_releases cms_releases_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_releases
    ADD CONSTRAINT cms_releases_pkey PRIMARY KEY (id);


--
-- Name: cms_reusable_block_usage cms_reusable_block_usage_content_id_block_instance_id_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_reusable_block_usage
    ADD CONSTRAINT cms_reusable_block_usage_content_id_block_instance_id_key UNIQUE (content_id, block_instance_id);


--
-- Name: cms_reusable_block_usage cms_reusable_block_usage_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_reusable_block_usage
    ADD CONSTRAINT cms_reusable_block_usage_pkey PRIMARY KEY (id);


--
-- Name: cms_reusable_blocks cms_reusable_blocks_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_reusable_blocks
    ADD CONSTRAINT cms_reusable_blocks_pkey PRIMARY KEY (id);


--
-- Name: cms_reusable_blocks cms_reusable_blocks_slug_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_reusable_blocks
    ADD CONSTRAINT cms_reusable_blocks_slug_key UNIQUE (slug);


--
-- Name: cms_revisions cms_revisions_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_revisions
    ADD CONSTRAINT cms_revisions_pkey PRIMARY KEY (id);


--
-- Name: cms_schedule_history cms_schedule_history_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_schedule_history
    ADD CONSTRAINT cms_schedule_history_pkey PRIMARY KEY (id);


--
-- Name: cms_scheduled_jobs cms_scheduled_jobs_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_scheduled_jobs
    ADD CONSTRAINT cms_scheduled_jobs_pkey PRIMARY KEY (id);


--
-- Name: cms_schedules cms_schedules_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_schedules
    ADD CONSTRAINT cms_schedules_pkey PRIMARY KEY (id);


--
-- Name: cms_site_settings cms_site_settings_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_site_settings
    ADD CONSTRAINT cms_site_settings_pkey PRIMARY KEY (id);


--
-- Name: cms_tags cms_tags_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_tags
    ADD CONSTRAINT cms_tags_pkey PRIMARY KEY (id);


--
-- Name: cms_tags cms_tags_slug_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_tags
    ADD CONSTRAINT cms_tags_slug_key UNIQUE (slug);


--
-- Name: cms_user_editor_preferences cms_user_editor_preferences_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_user_editor_preferences
    ADD CONSTRAINT cms_user_editor_preferences_pkey PRIMARY KEY (id);


--
-- Name: cms_user_editor_preferences cms_user_editor_preferences_user_id_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_user_editor_preferences
    ADD CONSTRAINT cms_user_editor_preferences_user_id_key UNIQUE (user_id);


--
-- Name: cms_users cms_users_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_users
    ADD CONSTRAINT cms_users_pkey PRIMARY KEY (id);


--
-- Name: cms_users cms_users_user_id_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_users
    ADD CONSTRAINT cms_users_user_id_key UNIQUE (user_id);


--
-- Name: cms_webhook_deliveries cms_webhook_deliveries_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_webhook_deliveries
    ADD CONSTRAINT cms_webhook_deliveries_pkey PRIMARY KEY (id);


--
-- Name: cms_webhooks cms_webhooks_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_webhooks
    ADD CONSTRAINT cms_webhooks_pkey PRIMARY KEY (id);


--
-- Name: cms_workflow_history cms_workflow_history_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_workflow_history
    ADD CONSTRAINT cms_workflow_history_pkey PRIMARY KEY (id);


--
-- Name: cms_workflow_log cms_workflow_log_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_workflow_log
    ADD CONSTRAINT cms_workflow_log_pkey PRIMARY KEY (id);


--
-- Name: cms_workflow_status cms_workflow_status_content_id_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_workflow_status
    ADD CONSTRAINT cms_workflow_status_content_id_key UNIQUE (content_id);


--
-- Name: cms_workflow_status cms_workflow_status_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_workflow_status
    ADD CONSTRAINT cms_workflow_status_pkey PRIMARY KEY (id);


--
-- Name: contacts contacts_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.contacts
    ADD CONSTRAINT contacts_pkey PRIMARY KEY (id);


--
-- Name: content_translations content_translations_content_type_content_id_locale_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.content_translations
    ADD CONSTRAINT content_translations_content_type_content_id_locale_key UNIQUE (content_type, content_id, locale);


--
-- Name: content_translations content_translations_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.content_translations
    ADD CONSTRAINT content_translations_pkey PRIMARY KEY (id);


--
-- Name: content_versions content_versions_content_type_content_id_version_number_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.content_versions
    ADD CONSTRAINT content_versions_content_type_content_id_version_number_key UNIQUE (content_type, content_id, version_number);


--
-- Name: content_versions content_versions_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.content_versions
    ADD CONSTRAINT content_versions_pkey PRIMARY KEY (id);


--
-- Name: content_workflow_status content_workflow_status_content_type_content_id_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.content_workflow_status
    ADD CONSTRAINT content_workflow_status_content_type_content_id_key UNIQUE (content_type, content_id);


--
-- Name: content_workflow_status content_workflow_status_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.content_workflow_status
    ADD CONSTRAINT content_workflow_status_pkey PRIMARY KEY (id);


--
-- Name: coupons coupons_code_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.coupons
    ADD CONSTRAINT coupons_code_key UNIQUE (code);


--
-- Name: coupons coupons_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.coupons
    ADD CONSTRAINT coupons_pkey PRIMARY KEY (id);


--
-- Name: course_categories course_categories_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_categories
    ADD CONSTRAINT course_categories_pkey PRIMARY KEY (id);


--
-- Name: course_categories course_categories_slug_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_categories
    ADD CONSTRAINT course_categories_slug_key UNIQUE (slug);


--
-- Name: course_category_mappings course_category_mappings_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_category_mappings
    ADD CONSTRAINT course_category_mappings_pkey PRIMARY KEY (course_id, category_id);


--
-- Name: course_downloads course_downloads_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_downloads
    ADD CONSTRAINT course_downloads_pkey PRIMARY KEY (id);


--
-- Name: course_lessons course_lessons_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_lessons
    ADD CONSTRAINT course_lessons_pkey PRIMARY KEY (id);


--
-- Name: course_live_sessions course_live_sessions_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_live_sessions
    ADD CONSTRAINT course_live_sessions_pkey PRIMARY KEY (id);


--
-- Name: course_modules course_modules_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_modules
    ADD CONSTRAINT course_modules_pkey PRIMARY KEY (id);


--
-- Name: course_modules_v2 course_modules_v2_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_modules_v2
    ADD CONSTRAINT course_modules_v2_pkey PRIMARY KEY (id);


--
-- Name: course_quizzes course_quizzes_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_quizzes
    ADD CONSTRAINT course_quizzes_pkey PRIMARY KEY (id);


--
-- Name: course_resources course_resources_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_resources
    ADD CONSTRAINT course_resources_pkey PRIMARY KEY (id);


--
-- Name: course_reviews course_reviews_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_reviews
    ADD CONSTRAINT course_reviews_pkey PRIMARY KEY (id);


--
-- Name: course_reviews course_reviews_user_id_course_id_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_reviews
    ADD CONSTRAINT course_reviews_user_id_course_id_key UNIQUE (user_id, course_id);


--
-- Name: course_sections course_sections_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_sections
    ADD CONSTRAINT course_sections_pkey PRIMARY KEY (id);


--
-- Name: course_tag_mappings course_tag_mappings_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_tag_mappings
    ADD CONSTRAINT course_tag_mappings_pkey PRIMARY KEY (course_id, tag_id);


--
-- Name: course_tags course_tags_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_tags
    ADD CONSTRAINT course_tags_pkey PRIMARY KEY (id);


--
-- Name: course_tags course_tags_slug_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_tags
    ADD CONSTRAINT course_tags_slug_key UNIQUE (slug);


--
-- Name: courses_enhanced courses_enhanced_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.courses_enhanced
    ADD CONSTRAINT courses_enhanced_pkey PRIMARY KEY (id);


--
-- Name: courses_enhanced courses_enhanced_slug_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.courses_enhanced
    ADD CONSTRAINT courses_enhanced_slug_key UNIQUE (slug);


--
-- Name: courses courses_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.courses
    ADD CONSTRAINT courses_pkey PRIMARY KEY (id);


--
-- Name: courses courses_slug_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.courses
    ADD CONSTRAINT courses_slug_key UNIQUE (slug);


--
-- Name: crm_abandoned_carts crm_abandoned_carts_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_abandoned_carts
    ADD CONSTRAINT crm_abandoned_carts_pkey PRIMARY KEY (id);


--
-- Name: crm_automations crm_automations_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_automations
    ADD CONSTRAINT crm_automations_pkey PRIMARY KEY (id);


--
-- Name: crm_campaigns crm_campaigns_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_campaigns
    ADD CONSTRAINT crm_campaigns_pkey PRIMARY KEY (id);


--
-- Name: crm_companies crm_companies_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_companies
    ADD CONSTRAINT crm_companies_pkey PRIMARY KEY (id);


--
-- Name: crm_deal_activities crm_deal_activities_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_deal_activities
    ADD CONSTRAINT crm_deal_activities_pkey PRIMARY KEY (id);


--
-- Name: crm_deal_stage_history crm_deal_stage_history_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_deal_stage_history
    ADD CONSTRAINT crm_deal_stage_history_pkey PRIMARY KEY (id);


--
-- Name: crm_deals crm_deals_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_deals
    ADD CONSTRAINT crm_deals_pkey PRIMARY KEY (id);


--
-- Name: crm_lists crm_lists_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_lists
    ADD CONSTRAINT crm_lists_pkey PRIMARY KEY (id);


--
-- Name: crm_pipeline_stages crm_pipeline_stages_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_pipeline_stages
    ADD CONSTRAINT crm_pipeline_stages_pkey PRIMARY KEY (id);


--
-- Name: crm_pipelines crm_pipelines_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_pipelines
    ADD CONSTRAINT crm_pipelines_pkey PRIMARY KEY (id);


--
-- Name: crm_recurring_campaigns crm_recurring_campaigns_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_recurring_campaigns
    ADD CONSTRAINT crm_recurring_campaigns_pkey PRIMARY KEY (id);


--
-- Name: crm_segments crm_segments_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_segments
    ADD CONSTRAINT crm_segments_pkey PRIMARY KEY (id);


--
-- Name: crm_sequences crm_sequences_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_sequences
    ADD CONSTRAINT crm_sequences_pkey PRIMARY KEY (id);


--
-- Name: crm_smart_links crm_smart_links_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_smart_links
    ADD CONSTRAINT crm_smart_links_pkey PRIMARY KEY (id);


--
-- Name: crm_smart_links crm_smart_links_short_code_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_smart_links
    ADD CONSTRAINT crm_smart_links_short_code_key UNIQUE (short_code);


--
-- Name: crm_tags crm_tags_name_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_tags
    ADD CONSTRAINT crm_tags_name_key UNIQUE (name);


--
-- Name: crm_tags crm_tags_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_tags
    ADD CONSTRAINT crm_tags_pkey PRIMARY KEY (id);


--
-- Name: crm_templates crm_templates_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_templates
    ADD CONSTRAINT crm_templates_pkey PRIMARY KEY (id);


--
-- Name: crm_webhooks crm_webhooks_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_webhooks
    ADD CONSTRAINT crm_webhooks_pkey PRIMARY KEY (id);


--
-- Name: departments departments_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.departments
    ADD CONSTRAINT departments_pkey PRIMARY KEY (id);


--
-- Name: departments departments_slug_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.departments
    ADD CONSTRAINT departments_slug_key UNIQUE (slug);


--
-- Name: email_campaigns email_campaigns_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.email_campaigns
    ADD CONSTRAINT email_campaigns_pkey PRIMARY KEY (id);


--
-- Name: email_logs email_logs_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.email_logs
    ADD CONSTRAINT email_logs_pkey PRIMARY KEY (id);


--
-- Name: email_templates email_templates_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.email_templates
    ADD CONSTRAINT email_templates_pkey PRIMARY KEY (id);


--
-- Name: email_templates email_templates_slug_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.email_templates
    ADD CONSTRAINT email_templates_slug_key UNIQUE (slug);


--
-- Name: email_verification_tokens email_verification_tokens_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.email_verification_tokens
    ADD CONSTRAINT email_verification_tokens_pkey PRIMARY KEY (id);


--
-- Name: email_verification_tokens email_verification_tokens_token_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.email_verification_tokens
    ADD CONSTRAINT email_verification_tokens_token_key UNIQUE (token);


--
-- Name: failed_jobs failed_jobs_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.failed_jobs
    ADD CONSTRAINT failed_jobs_pkey PRIMARY KEY (id);


--
-- Name: form_submissions form_submissions_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.form_submissions
    ADD CONSTRAINT form_submissions_pkey PRIMARY KEY (id);


--
-- Name: forms forms_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.forms
    ADD CONSTRAINT forms_pkey PRIMARY KEY (id);


--
-- Name: forms forms_slug_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.forms
    ADD CONSTRAINT forms_slug_key UNIQUE (slug);


--
-- Name: indicator_documentation indicator_documentation_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.indicator_documentation
    ADD CONSTRAINT indicator_documentation_pkey PRIMARY KEY (id);


--
-- Name: indicator_download_log indicator_download_log_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.indicator_download_log
    ADD CONSTRAINT indicator_download_log_pkey PRIMARY KEY (id);


--
-- Name: indicator_downloads indicator_downloads_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.indicator_downloads
    ADD CONSTRAINT indicator_downloads_pkey PRIMARY KEY (id);


--
-- Name: indicator_files indicator_files_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.indicator_files
    ADD CONSTRAINT indicator_files_pkey PRIMARY KEY (id);


--
-- Name: indicator_platform_files indicator_platform_files_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.indicator_platform_files
    ADD CONSTRAINT indicator_platform_files_pkey PRIMARY KEY (id);


--
-- Name: indicator_platforms indicator_platforms_name_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.indicator_platforms
    ADD CONSTRAINT indicator_platforms_name_key UNIQUE (name);


--
-- Name: indicator_platforms indicator_platforms_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.indicator_platforms
    ADD CONSTRAINT indicator_platforms_pkey PRIMARY KEY (id);


--
-- Name: indicator_platforms indicator_platforms_slug_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.indicator_platforms
    ADD CONSTRAINT indicator_platforms_slug_key UNIQUE (slug);


--
-- Name: indicator_tradingview_access indicator_tradingview_access_indicator_id_tradingview_usern_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.indicator_tradingview_access
    ADD CONSTRAINT indicator_tradingview_access_indicator_id_tradingview_usern_key UNIQUE (indicator_id, tradingview_username);


--
-- Name: indicator_tradingview_access indicator_tradingview_access_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.indicator_tradingview_access
    ADD CONSTRAINT indicator_tradingview_access_pkey PRIMARY KEY (id);


--
-- Name: indicator_videos indicator_videos_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.indicator_videos
    ADD CONSTRAINT indicator_videos_pkey PRIMARY KEY (id);


--
-- Name: indicators_enhanced indicators_enhanced_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.indicators_enhanced
    ADD CONSTRAINT indicators_enhanced_pkey PRIMARY KEY (id);


--
-- Name: indicators_enhanced indicators_enhanced_slug_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.indicators_enhanced
    ADD CONSTRAINT indicators_enhanced_slug_key UNIQUE (slug);


--
-- Name: indicators indicators_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.indicators
    ADD CONSTRAINT indicators_pkey PRIMARY KEY (id);


--
-- Name: indicators indicators_slug_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.indicators
    ADD CONSTRAINT indicators_slug_key UNIQUE (slug);


--
-- Name: integration_webhooks integration_webhooks_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.integration_webhooks
    ADD CONSTRAINT integration_webhooks_pkey PRIMARY KEY (id);


--
-- Name: invoices invoices_invoice_number_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.invoices
    ADD CONSTRAINT invoices_invoice_number_key UNIQUE (invoice_number);


--
-- Name: invoices invoices_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.invoices
    ADD CONSTRAINT invoices_pkey PRIMARY KEY (id);


--
-- Name: laravel_jobs_backup jobs_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.laravel_jobs_backup
    ADD CONSTRAINT jobs_pkey PRIMARY KEY (id);


--
-- Name: jobs jobs_pkey1; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.jobs
    ADD CONSTRAINT jobs_pkey1 PRIMARY KEY (id);


--
-- Name: lessons lessons_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.lessons
    ADD CONSTRAINT lessons_pkey PRIMARY KEY (id);


--
-- Name: locales locales_code_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.locales
    ADD CONSTRAINT locales_code_key UNIQUE (code);


--
-- Name: locales locales_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.locales
    ADD CONSTRAINT locales_pkey PRIMARY KEY (id);


--
-- Name: media media_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.media
    ADD CONSTRAINT media_pkey PRIMARY KEY (id);


--
-- Name: member_audit_logs member_audit_logs_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.member_audit_logs
    ADD CONSTRAINT member_audit_logs_pkey PRIMARY KEY (id);


--
-- Name: member_emails member_emails_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.member_emails
    ADD CONSTRAINT member_emails_pkey PRIMARY KEY (id);


--
-- Name: member_filters member_filters_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.member_filters
    ADD CONSTRAINT member_filters_pkey PRIMARY KEY (id);


--
-- Name: member_filters member_filters_slug_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.member_filters
    ADD CONSTRAINT member_filters_slug_key UNIQUE (slug);


--
-- Name: member_notes member_notes_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.member_notes
    ADD CONSTRAINT member_notes_pkey PRIMARY KEY (id);


--
-- Name: member_segments member_segments_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.member_segments
    ADD CONSTRAINT member_segments_pkey PRIMARY KEY (id);


--
-- Name: member_segments member_segments_slug_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.member_segments
    ADD CONSTRAINT member_segments_slug_key UNIQUE (slug);


--
-- Name: member_tags member_tags_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.member_tags
    ADD CONSTRAINT member_tags_pkey PRIMARY KEY (id);


--
-- Name: member_tags member_tags_slug_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.member_tags
    ADD CONSTRAINT member_tags_slug_key UNIQUE (slug);


--
-- Name: membership_features membership_features_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.membership_features
    ADD CONSTRAINT membership_features_pkey PRIMARY KEY (id);


--
-- Name: membership_features membership_features_plan_id_feature_code_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.membership_features
    ADD CONSTRAINT membership_features_plan_id_feature_code_key UNIQUE (plan_id, feature_code);


--
-- Name: membership_plan_price_history membership_plan_price_history_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.membership_plan_price_history
    ADD CONSTRAINT membership_plan_price_history_pkey PRIMARY KEY (id);


--
-- Name: membership_plans membership_plans_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.membership_plans
    ADD CONSTRAINT membership_plans_pkey PRIMARY KEY (id);


--
-- Name: membership_plans membership_plans_slug_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.membership_plans
    ADD CONSTRAINT membership_plans_slug_key UNIQUE (slug);


--
-- Name: mfa_attempts mfa_attempts_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.mfa_attempts
    ADD CONSTRAINT mfa_attempts_pkey PRIMARY KEY (id);


--
-- Name: newsletter_subscribers newsletter_subscribers_email_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.newsletter_subscribers
    ADD CONSTRAINT newsletter_subscribers_email_key UNIQUE (email);


--
-- Name: newsletter_subscribers newsletter_subscribers_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.newsletter_subscribers
    ADD CONSTRAINT newsletter_subscribers_pkey PRIMARY KEY (id);


--
-- Name: oauth_audit_log oauth_audit_log_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.oauth_audit_log
    ADD CONSTRAINT oauth_audit_log_pkey PRIMARY KEY (id);


--
-- Name: oauth_states oauth_states_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.oauth_states
    ADD CONSTRAINT oauth_states_pkey PRIMARY KEY (id);


--
-- Name: oauth_states oauth_states_state_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.oauth_states
    ADD CONSTRAINT oauth_states_state_key UNIQUE (state);


--
-- Name: order_items order_items_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.order_items
    ADD CONSTRAINT order_items_pkey PRIMARY KEY (id);


--
-- Name: orders orders_order_number_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.orders
    ADD CONSTRAINT orders_order_number_key UNIQUE (order_number);


--
-- Name: orders orders_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.orders
    ADD CONSTRAINT orders_pkey PRIMARY KEY (id);


--
-- Name: page_layout_versions page_layout_versions_layout_id_version_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.page_layout_versions
    ADD CONSTRAINT page_layout_versions_layout_id_version_key UNIQUE (layout_id, version);


--
-- Name: page_layout_versions page_layout_versions_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.page_layout_versions
    ADD CONSTRAINT page_layout_versions_pkey PRIMARY KEY (id);


--
-- Name: page_layouts page_layouts_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.page_layouts
    ADD CONSTRAINT page_layouts_pkey PRIMARY KEY (id);


--
-- Name: page_layouts page_layouts_slug_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.page_layouts
    ADD CONSTRAINT page_layouts_slug_key UNIQUE (slug);


--
-- Name: password_resets password_resets_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.password_resets
    ADD CONSTRAINT password_resets_pkey PRIMARY KEY (id);


--
-- Name: payment_disputes payment_disputes_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.payment_disputes
    ADD CONSTRAINT payment_disputes_pkey PRIMARY KEY (id);


--
-- Name: payment_disputes payment_disputes_stripe_dispute_id_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.payment_disputes
    ADD CONSTRAINT payment_disputes_stripe_dispute_id_key UNIQUE (stripe_dispute_id);


--
-- Name: popup_ab_tests popup_ab_tests_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.popup_ab_tests
    ADD CONSTRAINT popup_ab_tests_pkey PRIMARY KEY (id);


--
-- Name: popup_events popup_events_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.popup_events
    ADD CONSTRAINT popup_events_pkey PRIMARY KEY (id);


--
-- Name: popup_form_submissions popup_form_submissions_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.popup_form_submissions
    ADD CONSTRAINT popup_form_submissions_pkey PRIMARY KEY (id);


--
-- Name: popups popups_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.popups
    ADD CONSTRAINT popups_pkey PRIMARY KEY (id);


--
-- Name: post_categories post_categories_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.post_categories
    ADD CONSTRAINT post_categories_pkey PRIMARY KEY (post_id, category_id);


--
-- Name: post_tags post_tags_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.post_tags
    ADD CONSTRAINT post_tags_pkey PRIMARY KEY (post_id, tag_id);


--
-- Name: posts posts_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.posts
    ADD CONSTRAINT posts_pkey PRIMARY KEY (id);


--
-- Name: posts posts_slug_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.posts
    ADD CONSTRAINT posts_slug_key UNIQUE (slug);


--
-- Name: preview_tokens preview_tokens_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.preview_tokens
    ADD CONSTRAINT preview_tokens_pkey PRIMARY KEY (id);


--
-- Name: preview_tokens preview_tokens_token_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.preview_tokens
    ADD CONSTRAINT preview_tokens_token_key UNIQUE (token);


--
-- Name: products products_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.products
    ADD CONSTRAINT products_pkey PRIMARY KEY (id);


--
-- Name: products products_slug_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.products
    ADD CONSTRAINT products_slug_key UNIQUE (slug);


--
-- Name: quiz_answers quiz_answers_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.quiz_answers
    ADD CONSTRAINT quiz_answers_pkey PRIMARY KEY (id);


--
-- Name: quiz_questions quiz_questions_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.quiz_questions
    ADD CONSTRAINT quiz_questions_pkey PRIMARY KEY (id);


--
-- Name: reconciliation_log reconciliation_log_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.reconciliation_log
    ADD CONSTRAINT reconciliation_log_pkey PRIMARY KEY (id);


--
-- Name: redirects redirects_from_path_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.redirects
    ADD CONSTRAINT redirects_from_path_key UNIQUE (from_path);


--
-- Name: redirects redirects_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.redirects
    ADD CONSTRAINT redirects_pkey PRIMARY KEY (id);


--
-- Name: resource_access_log resource_access_log_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.resource_access_log
    ADD CONSTRAINT resource_access_log_pkey PRIMARY KEY (id);


--
-- Name: resource_access_log resource_access_log_user_id_resource_id_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.resource_access_log
    ADD CONSTRAINT resource_access_log_user_id_resource_id_key UNIQUE (user_id, resource_id);


--
-- Name: resource_download_logs resource_download_logs_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.resource_download_logs
    ADD CONSTRAINT resource_download_logs_pkey PRIMARY KEY (id);


--
-- Name: resource_favorites resource_favorites_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.resource_favorites
    ADD CONSTRAINT resource_favorites_pkey PRIMARY KEY (id);


--
-- Name: resource_favorites resource_favorites_resource_id_user_id_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.resource_favorites
    ADD CONSTRAINT resource_favorites_resource_id_user_id_key UNIQUE (resource_id, user_id);


--
-- Name: resource_upload_limits resource_upload_limits_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.resource_upload_limits
    ADD CONSTRAINT resource_upload_limits_pkey PRIMARY KEY (id);


--
-- Name: resource_upload_limits resource_upload_limits_resource_type_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.resource_upload_limits
    ADD CONSTRAINT resource_upload_limits_resource_type_key UNIQUE (resource_type);


--
-- Name: review_helpful_votes review_helpful_votes_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.review_helpful_votes
    ADD CONSTRAINT review_helpful_votes_pkey PRIMARY KEY (id);


--
-- Name: review_helpful_votes review_helpful_votes_review_id_user_id_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.review_helpful_votes
    ADD CONSTRAINT review_helpful_votes_review_id_user_id_key UNIQUE (review_id, user_id);


--
-- Name: room_alerts room_alerts_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.room_alerts
    ADD CONSTRAINT room_alerts_pkey PRIMARY KEY (id);


--
-- Name: room_resources room_resources_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.room_resources
    ADD CONSTRAINT room_resources_pkey PRIMARY KEY (id);


--
-- Name: room_resources room_resources_room_slug_slug_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.room_resources
    ADD CONSTRAINT room_resources_room_slug_slug_key UNIQUE (room_slug, slug);


--
-- Name: room_sections room_sections_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.room_sections
    ADD CONSTRAINT room_sections_pkey PRIMARY KEY (id);


--
-- Name: room_stats_cache room_stats_cache_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.room_stats_cache
    ADD CONSTRAINT room_stats_cache_pkey PRIMARY KEY (id);


--
-- Name: room_stats_cache room_stats_cache_room_id_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.room_stats_cache
    ADD CONSTRAINT room_stats_cache_room_id_key UNIQUE (room_id);


--
-- Name: room_stats_cache room_stats_cache_room_slug_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.room_stats_cache
    ADD CONSTRAINT room_stats_cache_room_slug_key UNIQUE (room_slug);


--
-- Name: room_trade_plans room_trade_plans_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.room_trade_plans
    ADD CONSTRAINT room_trade_plans_pkey PRIMARY KEY (id);


--
-- Name: room_traders room_traders_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.room_traders
    ADD CONSTRAINT room_traders_pkey PRIMARY KEY (id);


--
-- Name: room_traders room_traders_slug_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.room_traders
    ADD CONSTRAINT room_traders_slug_key UNIQUE (slug);


--
-- Name: room_trades room_trades_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.room_trades
    ADD CONSTRAINT room_trades_pkey PRIMARY KEY (id);


--
-- Name: room_weekly_videos room_weekly_videos_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.room_weekly_videos
    ADD CONSTRAINT room_weekly_videos_pkey PRIMARY KEY (id);


--
-- Name: room_weekly_videos room_weekly_videos_room_slug_week_of_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.room_weekly_videos
    ADD CONSTRAINT room_weekly_videos_room_slug_week_of_key UNIQUE (room_slug, week_of);


--
-- Name: schedule_exceptions schedule_exceptions_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.schedule_exceptions
    ADD CONSTRAINT schedule_exceptions_pkey PRIMARY KEY (id);


--
-- Name: schedule_exceptions schedule_exceptions_schedule_id_exception_date_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.schedule_exceptions
    ADD CONSTRAINT schedule_exceptions_schedule_id_exception_date_key UNIQUE (schedule_id, exception_date);


--
-- Name: scheduled_content scheduled_content_content_type_content_id_scheduled_action__key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.scheduled_content
    ADD CONSTRAINT scheduled_content_content_type_content_id_scheduled_action__key UNIQUE (content_type, content_id, scheduled_action, status);


--
-- Name: scheduled_content scheduled_content_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.scheduled_content
    ADD CONSTRAINT scheduled_content_pkey PRIMARY KEY (id);


--
-- Name: security_events security_events_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.security_events
    ADD CONSTRAINT security_events_pkey PRIMARY KEY (id);


--
-- Name: service_connections service_connections_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.service_connections
    ADD CONSTRAINT service_connections_pkey PRIMARY KEY (id);


--
-- Name: settings settings_key_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.settings
    ADD CONSTRAINT settings_key_key UNIQUE (key);


--
-- Name: settings settings_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.settings
    ADD CONSTRAINT settings_pkey PRIMARY KEY (id);


--
-- Name: stock_lists stock_lists_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.stock_lists
    ADD CONSTRAINT stock_lists_pkey PRIMARY KEY (id);


--
-- Name: stock_lists stock_lists_slug_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.stock_lists
    ADD CONSTRAINT stock_lists_slug_key UNIQUE (slug);


--
-- Name: tags tags_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.tags
    ADD CONSTRAINT tags_pkey PRIMARY KEY (id);


--
-- Name: tags tags_slug_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.tags
    ADD CONSTRAINT tags_slug_key UNIQUE (slug);


--
-- Name: teams teams_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.teams
    ADD CONSTRAINT teams_pkey PRIMARY KEY (id);


--
-- Name: teams teams_slug_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.teams
    ADD CONSTRAINT teams_slug_key UNIQUE (slug);


--
-- Name: traders traders_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.traders
    ADD CONSTRAINT traders_pkey PRIMARY KEY (id);


--
-- Name: trading_room_schedules trading_room_schedules_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.trading_room_schedules
    ADD CONSTRAINT trading_room_schedules_pkey PRIMARY KEY (id);


--
-- Name: trading_rooms trading_rooms_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.trading_rooms
    ADD CONSTRAINT trading_rooms_pkey PRIMARY KEY (id);


--
-- Name: trading_rooms trading_rooms_slug_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.trading_rooms
    ADD CONSTRAINT trading_rooms_slug_key UNIQUE (slug);


--
-- Name: unified_videos unified_videos_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.unified_videos
    ADD CONSTRAINT unified_videos_pkey PRIMARY KEY (id);


--
-- Name: cms_content unique_content_type_slug_locale; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_content
    ADD CONSTRAINT unique_content_type_slug_locale UNIQUE (content_type, slug, locale);


--
-- Name: cms_asset_folders unique_folder_in_parent; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_asset_folders
    ADD CONSTRAINT unique_folder_in_parent UNIQUE (parent_id, slug);


--
-- Name: cms_content_relations unique_relation; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_content_relations
    ADD CONSTRAINT unique_relation UNIQUE (source_id, target_id, relation_type);


--
-- Name: cms_revisions unique_revision_number; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_revisions
    ADD CONSTRAINT unique_revision_number UNIQUE (content_id, revision_number);


--
-- Name: cms_redirects unique_source_path; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_redirects
    ADD CONSTRAINT unique_source_path UNIQUE (source_path);


--
-- Name: user_mfa_secrets uq_user_mfa_secrets_user; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_mfa_secrets
    ADD CONSTRAINT uq_user_mfa_secrets_user UNIQUE (user_id);


--
-- Name: video_watch_progress uq_user_video_progress; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.video_watch_progress
    ADD CONSTRAINT uq_user_video_progress UNIQUE (user_id, video_id);


--
-- Name: video_transcripts uq_video_transcript_lang; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.video_transcripts
    ADD CONSTRAINT uq_video_transcript_lang UNIQUE (video_id, language_code);


--
-- Name: user_activity_log user_activity_log_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_activity_log
    ADD CONSTRAINT user_activity_log_pkey PRIMARY KEY (id);


--
-- Name: user_coupons user_coupons_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_coupons
    ADD CONSTRAINT user_coupons_pkey PRIMARY KEY (id);


--
-- Name: user_coupons user_coupons_user_id_coupon_id_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_coupons
    ADD CONSTRAINT user_coupons_user_id_coupon_id_key UNIQUE (user_id, coupon_id);


--
-- Name: user_course_enrollments user_course_enrollments_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_course_enrollments
    ADD CONSTRAINT user_course_enrollments_pkey PRIMARY KEY (id);


--
-- Name: user_course_enrollments user_course_enrollments_user_id_course_id_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_course_enrollments
    ADD CONSTRAINT user_course_enrollments_user_id_course_id_key UNIQUE (user_id, course_id);


--
-- Name: user_departments user_departments_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_departments
    ADD CONSTRAINT user_departments_pkey PRIMARY KEY (id);


--
-- Name: user_departments user_departments_user_id_department_id_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_departments
    ADD CONSTRAINT user_departments_user_id_department_id_key UNIQUE (user_id, department_id);


--
-- Name: user_favorites user_favorites_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_favorites
    ADD CONSTRAINT user_favorites_pkey PRIMARY KEY (id);


--
-- Name: user_favorites user_favorites_user_id_item_type_item_id_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_favorites
    ADD CONSTRAINT user_favorites_user_id_item_type_item_id_key UNIQUE (user_id, item_type, item_id);


--
-- Name: user_indicator_access user_indicator_access_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_indicator_access
    ADD CONSTRAINT user_indicator_access_pkey PRIMARY KEY (id);


--
-- Name: user_indicator_access user_indicator_access_user_id_indicator_id_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_indicator_access
    ADD CONSTRAINT user_indicator_access_user_id_indicator_id_key UNIQUE (user_id, indicator_id);


--
-- Name: user_indicator_ownership user_indicator_ownership_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_indicator_ownership
    ADD CONSTRAINT user_indicator_ownership_pkey PRIMARY KEY (id);


--
-- Name: user_indicator_ownership user_indicator_ownership_user_id_indicator_id_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_indicator_ownership
    ADD CONSTRAINT user_indicator_ownership_user_id_indicator_id_key UNIQUE (user_id, indicator_id);


--
-- Name: user_indicators user_indicators_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_indicators
    ADD CONSTRAINT user_indicators_pkey PRIMARY KEY (id);


--
-- Name: user_indicators user_indicators_user_id_indicator_id_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_indicators
    ADD CONSTRAINT user_indicators_user_id_indicator_id_key UNIQUE (user_id, indicator_id);


--
-- Name: user_lesson_progress user_lesson_progress_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_lesson_progress
    ADD CONSTRAINT user_lesson_progress_pkey PRIMARY KEY (id);


--
-- Name: user_lesson_progress user_lesson_progress_user_id_lesson_id_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_lesson_progress
    ADD CONSTRAINT user_lesson_progress_user_id_lesson_id_key UNIQUE (user_id, lesson_id);


--
-- Name: user_member_segments user_member_segments_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_member_segments
    ADD CONSTRAINT user_member_segments_pkey PRIMARY KEY (id);


--
-- Name: user_member_segments user_member_segments_user_id_segment_id_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_member_segments
    ADD CONSTRAINT user_member_segments_user_id_segment_id_key UNIQUE (user_id, segment_id);


--
-- Name: user_member_tags user_member_tags_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_member_tags
    ADD CONSTRAINT user_member_tags_pkey PRIMARY KEY (id);


--
-- Name: user_member_tags user_member_tags_user_id_tag_id_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_member_tags
    ADD CONSTRAINT user_member_tags_user_id_tag_id_key UNIQUE (user_id, tag_id);


--
-- Name: user_memberships user_memberships_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_memberships
    ADD CONSTRAINT user_memberships_pkey PRIMARY KEY (id);


--
-- Name: user_mfa_secrets user_mfa_secrets_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_mfa_secrets
    ADD CONSTRAINT user_mfa_secrets_pkey PRIMARY KEY (id);


--
-- Name: user_products user_products_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_products
    ADD CONSTRAINT user_products_pkey PRIMARY KEY (id);


--
-- Name: user_products user_products_user_id_product_id_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_products
    ADD CONSTRAINT user_products_user_id_product_id_key UNIQUE (user_id, product_id);


--
-- Name: user_quiz_attempts user_quiz_attempts_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_quiz_attempts
    ADD CONSTRAINT user_quiz_attempts_pkey PRIMARY KEY (id);


--
-- Name: user_sessions user_sessions_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_sessions
    ADD CONSTRAINT user_sessions_pkey PRIMARY KEY (id);


--
-- Name: user_status user_status_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_status
    ADD CONSTRAINT user_status_pkey PRIMARY KEY (id);


--
-- Name: user_status user_status_user_id_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_status
    ADD CONSTRAINT user_status_user_id_key UNIQUE (user_id);


--
-- Name: user_teams user_teams_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_teams
    ADD CONSTRAINT user_teams_pkey PRIMARY KEY (id);


--
-- Name: user_teams user_teams_user_id_team_id_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_teams
    ADD CONSTRAINT user_teams_user_id_team_id_key UNIQUE (user_id, team_id);


--
-- Name: users users_apple_id_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_apple_id_key UNIQUE (apple_id);


--
-- Name: users users_email_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_email_key UNIQUE (email);


--
-- Name: users users_google_id_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_google_id_key UNIQUE (google_id);


--
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);


--
-- Name: video_chapters video_chapters_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.video_chapters
    ADD CONSTRAINT video_chapters_pkey PRIMARY KEY (id);


--
-- Name: video_room_assignments video_room_assignments_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.video_room_assignments
    ADD CONSTRAINT video_room_assignments_pkey PRIMARY KEY (id);


--
-- Name: video_room_assignments video_room_assignments_video_id_room_id_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.video_room_assignments
    ADD CONSTRAINT video_room_assignments_video_id_room_id_key UNIQUE (video_id, room_id);


--
-- Name: video_series video_series_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.video_series
    ADD CONSTRAINT video_series_pkey PRIMARY KEY (id);


--
-- Name: video_series video_series_slug_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.video_series
    ADD CONSTRAINT video_series_slug_key UNIQUE (slug);


--
-- Name: video_transcripts video_transcripts_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.video_transcripts
    ADD CONSTRAINT video_transcripts_pkey PRIMARY KEY (id);


--
-- Name: video_watch_progress video_watch_progress_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.video_watch_progress
    ADD CONSTRAINT video_watch_progress_pkey PRIMARY KEY (id);


--
-- Name: videos videos_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.videos
    ADD CONSTRAINT videos_pkey PRIMARY KEY (id);


--
-- Name: videos videos_slug_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.videos
    ADD CONSTRAINT videos_slug_key UNIQUE (slug);


--
-- Name: watchlist_entries watchlist_entries_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.watchlist_entries
    ADD CONSTRAINT watchlist_entries_pkey PRIMARY KEY (id);


--
-- Name: webhook_deliveries webhook_deliveries_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.webhook_deliveries
    ADD CONSTRAINT webhook_deliveries_pkey PRIMARY KEY (id);


--
-- Name: webhook_events webhook_events_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.webhook_events
    ADD CONSTRAINT webhook_events_pkey PRIMARY KEY (event_id);


--
-- Name: webhooks webhooks_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.webhooks
    ADD CONSTRAINT webhooks_pkey PRIMARY KEY (id);


--
-- Name: workflow_definitions workflow_definitions_name_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.workflow_definitions
    ADD CONSTRAINT workflow_definitions_name_key UNIQUE (name);


--
-- Name: workflow_definitions workflow_definitions_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.workflow_definitions
    ADD CONSTRAINT workflow_definitions_pkey PRIMARY KEY (id);


--
-- Name: workflow_transitions workflow_transitions_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.workflow_transitions
    ADD CONSTRAINT workflow_transitions_pkey PRIMARY KEY (id);


--
-- Name: idx_cms_audit_action; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_audit_action ON ONLY public.cms_audit_log USING btree (action);


--
-- Name: cms_audit_log_2026_05_action_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2026_05_action_idx ON public.cms_audit_log_2026_05 USING btree (action);


--
-- Name: idx_cms_audit_created; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_audit_created ON ONLY public.cms_audit_log USING btree (created_at DESC);


--
-- Name: cms_audit_log_2026_05_created_at_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2026_05_created_at_idx ON public.cms_audit_log_2026_05 USING btree (created_at DESC);


--
-- Name: idx_cms_audit_failed; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_audit_failed ON ONLY public.cms_audit_log USING btree (created_at DESC) WHERE ((status)::text = 'failed'::text);


--
-- Name: cms_audit_log_2026_05_created_at_idx1; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2026_05_created_at_idx1 ON public.cms_audit_log_2026_05 USING btree (created_at DESC) WHERE ((status)::text = 'failed'::text);


--
-- Name: idx_cms_audit_entity; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_audit_entity ON ONLY public.cms_audit_log USING btree (entity_type, entity_id);


--
-- Name: cms_audit_log_2026_05_entity_type_entity_id_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2026_05_entity_type_entity_id_idx ON public.cms_audit_log_2026_05 USING btree (entity_type, entity_id);


--
-- Name: idx_cms_audit_user; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_audit_user ON ONLY public.cms_audit_log USING btree (user_id);


--
-- Name: cms_audit_log_2026_05_user_id_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2026_05_user_id_idx ON public.cms_audit_log_2026_05 USING btree (user_id);


--
-- Name: cms_audit_log_2026_06_action_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2026_06_action_idx ON public.cms_audit_log_2026_06 USING btree (action);


--
-- Name: cms_audit_log_2026_06_created_at_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2026_06_created_at_idx ON public.cms_audit_log_2026_06 USING btree (created_at DESC);


--
-- Name: cms_audit_log_2026_06_created_at_idx1; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2026_06_created_at_idx1 ON public.cms_audit_log_2026_06 USING btree (created_at DESC) WHERE ((status)::text = 'failed'::text);


--
-- Name: cms_audit_log_2026_06_entity_type_entity_id_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2026_06_entity_type_entity_id_idx ON public.cms_audit_log_2026_06 USING btree (entity_type, entity_id);


--
-- Name: cms_audit_log_2026_06_user_id_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2026_06_user_id_idx ON public.cms_audit_log_2026_06 USING btree (user_id);


--
-- Name: cms_audit_log_2026_07_action_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2026_07_action_idx ON public.cms_audit_log_2026_07 USING btree (action);


--
-- Name: cms_audit_log_2026_07_created_at_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2026_07_created_at_idx ON public.cms_audit_log_2026_07 USING btree (created_at DESC);


--
-- Name: cms_audit_log_2026_07_created_at_idx1; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2026_07_created_at_idx1 ON public.cms_audit_log_2026_07 USING btree (created_at DESC) WHERE ((status)::text = 'failed'::text);


--
-- Name: cms_audit_log_2026_07_entity_type_entity_id_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2026_07_entity_type_entity_id_idx ON public.cms_audit_log_2026_07 USING btree (entity_type, entity_id);


--
-- Name: cms_audit_log_2026_07_user_id_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2026_07_user_id_idx ON public.cms_audit_log_2026_07 USING btree (user_id);


--
-- Name: cms_audit_log_2026_08_action_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2026_08_action_idx ON public.cms_audit_log_2026_08 USING btree (action);


--
-- Name: cms_audit_log_2026_08_created_at_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2026_08_created_at_idx ON public.cms_audit_log_2026_08 USING btree (created_at DESC);


--
-- Name: cms_audit_log_2026_08_created_at_idx1; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2026_08_created_at_idx1 ON public.cms_audit_log_2026_08 USING btree (created_at DESC) WHERE ((status)::text = 'failed'::text);


--
-- Name: cms_audit_log_2026_08_entity_type_entity_id_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2026_08_entity_type_entity_id_idx ON public.cms_audit_log_2026_08 USING btree (entity_type, entity_id);


--
-- Name: cms_audit_log_2026_08_user_id_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2026_08_user_id_idx ON public.cms_audit_log_2026_08 USING btree (user_id);


--
-- Name: cms_audit_log_2026_09_action_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2026_09_action_idx ON public.cms_audit_log_2026_09 USING btree (action);


--
-- Name: cms_audit_log_2026_09_created_at_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2026_09_created_at_idx ON public.cms_audit_log_2026_09 USING btree (created_at DESC);


--
-- Name: cms_audit_log_2026_09_created_at_idx1; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2026_09_created_at_idx1 ON public.cms_audit_log_2026_09 USING btree (created_at DESC) WHERE ((status)::text = 'failed'::text);


--
-- Name: cms_audit_log_2026_09_entity_type_entity_id_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2026_09_entity_type_entity_id_idx ON public.cms_audit_log_2026_09 USING btree (entity_type, entity_id);


--
-- Name: cms_audit_log_2026_09_user_id_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2026_09_user_id_idx ON public.cms_audit_log_2026_09 USING btree (user_id);


--
-- Name: cms_audit_log_2026_10_action_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2026_10_action_idx ON public.cms_audit_log_2026_10 USING btree (action);


--
-- Name: cms_audit_log_2026_10_created_at_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2026_10_created_at_idx ON public.cms_audit_log_2026_10 USING btree (created_at DESC);


--
-- Name: cms_audit_log_2026_10_created_at_idx1; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2026_10_created_at_idx1 ON public.cms_audit_log_2026_10 USING btree (created_at DESC) WHERE ((status)::text = 'failed'::text);


--
-- Name: cms_audit_log_2026_10_entity_type_entity_id_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2026_10_entity_type_entity_id_idx ON public.cms_audit_log_2026_10 USING btree (entity_type, entity_id);


--
-- Name: cms_audit_log_2026_10_user_id_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2026_10_user_id_idx ON public.cms_audit_log_2026_10 USING btree (user_id);


--
-- Name: cms_audit_log_2026_11_action_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2026_11_action_idx ON public.cms_audit_log_2026_11 USING btree (action);


--
-- Name: cms_audit_log_2026_11_created_at_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2026_11_created_at_idx ON public.cms_audit_log_2026_11 USING btree (created_at DESC);


--
-- Name: cms_audit_log_2026_11_created_at_idx1; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2026_11_created_at_idx1 ON public.cms_audit_log_2026_11 USING btree (created_at DESC) WHERE ((status)::text = 'failed'::text);


--
-- Name: cms_audit_log_2026_11_entity_type_entity_id_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2026_11_entity_type_entity_id_idx ON public.cms_audit_log_2026_11 USING btree (entity_type, entity_id);


--
-- Name: cms_audit_log_2026_11_user_id_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2026_11_user_id_idx ON public.cms_audit_log_2026_11 USING btree (user_id);


--
-- Name: cms_audit_log_2026_12_action_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2026_12_action_idx ON public.cms_audit_log_2026_12 USING btree (action);


--
-- Name: cms_audit_log_2026_12_created_at_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2026_12_created_at_idx ON public.cms_audit_log_2026_12 USING btree (created_at DESC);


--
-- Name: cms_audit_log_2026_12_created_at_idx1; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2026_12_created_at_idx1 ON public.cms_audit_log_2026_12 USING btree (created_at DESC) WHERE ((status)::text = 'failed'::text);


--
-- Name: cms_audit_log_2026_12_entity_type_entity_id_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2026_12_entity_type_entity_id_idx ON public.cms_audit_log_2026_12 USING btree (entity_type, entity_id);


--
-- Name: cms_audit_log_2026_12_user_id_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2026_12_user_id_idx ON public.cms_audit_log_2026_12 USING btree (user_id);


--
-- Name: cms_audit_log_2027_01_action_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2027_01_action_idx ON public.cms_audit_log_2027_01 USING btree (action);


--
-- Name: cms_audit_log_2027_01_created_at_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2027_01_created_at_idx ON public.cms_audit_log_2027_01 USING btree (created_at DESC);


--
-- Name: cms_audit_log_2027_01_created_at_idx1; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2027_01_created_at_idx1 ON public.cms_audit_log_2027_01 USING btree (created_at DESC) WHERE ((status)::text = 'failed'::text);


--
-- Name: cms_audit_log_2027_01_entity_type_entity_id_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2027_01_entity_type_entity_id_idx ON public.cms_audit_log_2027_01 USING btree (entity_type, entity_id);


--
-- Name: cms_audit_log_2027_01_user_id_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2027_01_user_id_idx ON public.cms_audit_log_2027_01 USING btree (user_id);


--
-- Name: cms_audit_log_2027_02_action_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2027_02_action_idx ON public.cms_audit_log_2027_02 USING btree (action);


--
-- Name: cms_audit_log_2027_02_created_at_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2027_02_created_at_idx ON public.cms_audit_log_2027_02 USING btree (created_at DESC);


--
-- Name: cms_audit_log_2027_02_created_at_idx1; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2027_02_created_at_idx1 ON public.cms_audit_log_2027_02 USING btree (created_at DESC) WHERE ((status)::text = 'failed'::text);


--
-- Name: cms_audit_log_2027_02_entity_type_entity_id_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2027_02_entity_type_entity_id_idx ON public.cms_audit_log_2027_02 USING btree (entity_type, entity_id);


--
-- Name: cms_audit_log_2027_02_user_id_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2027_02_user_id_idx ON public.cms_audit_log_2027_02 USING btree (user_id);


--
-- Name: cms_audit_log_2027_03_action_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2027_03_action_idx ON public.cms_audit_log_2027_03 USING btree (action);


--
-- Name: cms_audit_log_2027_03_created_at_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2027_03_created_at_idx ON public.cms_audit_log_2027_03 USING btree (created_at DESC);


--
-- Name: cms_audit_log_2027_03_created_at_idx1; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2027_03_created_at_idx1 ON public.cms_audit_log_2027_03 USING btree (created_at DESC) WHERE ((status)::text = 'failed'::text);


--
-- Name: cms_audit_log_2027_03_entity_type_entity_id_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2027_03_entity_type_entity_id_idx ON public.cms_audit_log_2027_03 USING btree (entity_type, entity_id);


--
-- Name: cms_audit_log_2027_03_user_id_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2027_03_user_id_idx ON public.cms_audit_log_2027_03 USING btree (user_id);


--
-- Name: cms_audit_log_2027_04_action_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2027_04_action_idx ON public.cms_audit_log_2027_04 USING btree (action);


--
-- Name: cms_audit_log_2027_04_created_at_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2027_04_created_at_idx ON public.cms_audit_log_2027_04 USING btree (created_at DESC);


--
-- Name: cms_audit_log_2027_04_created_at_idx1; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2027_04_created_at_idx1 ON public.cms_audit_log_2027_04 USING btree (created_at DESC) WHERE ((status)::text = 'failed'::text);


--
-- Name: cms_audit_log_2027_04_entity_type_entity_id_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2027_04_entity_type_entity_id_idx ON public.cms_audit_log_2027_04 USING btree (entity_type, entity_id);


--
-- Name: cms_audit_log_2027_04_user_id_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2027_04_user_id_idx ON public.cms_audit_log_2027_04 USING btree (user_id);


--
-- Name: cms_audit_log_2027_05_action_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2027_05_action_idx ON public.cms_audit_log_2027_05 USING btree (action);


--
-- Name: cms_audit_log_2027_05_created_at_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2027_05_created_at_idx ON public.cms_audit_log_2027_05 USING btree (created_at DESC);


--
-- Name: cms_audit_log_2027_05_created_at_idx1; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2027_05_created_at_idx1 ON public.cms_audit_log_2027_05 USING btree (created_at DESC) WHERE ((status)::text = 'failed'::text);


--
-- Name: cms_audit_log_2027_05_entity_type_entity_id_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2027_05_entity_type_entity_id_idx ON public.cms_audit_log_2027_05 USING btree (entity_type, entity_id);


--
-- Name: cms_audit_log_2027_05_user_id_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX cms_audit_log_2027_05_user_id_idx ON public.cms_audit_log_2027_05 USING btree (user_id);


--
-- Name: cms_presets_slug_unique; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX cms_presets_slug_unique ON public.cms_presets USING btree (block_type, slug) WHERE (deleted_at IS NULL);


--
-- Name: idx_activity_user; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_activity_user ON public.user_activity_log USING btree (user_id, created_at DESC);


--
-- Name: idx_admin_audit_logs_admin; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_admin_audit_logs_admin ON public.admin_audit_logs USING btree (admin_id, created_at DESC);


--
-- Name: idx_admin_audit_logs_created; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_admin_audit_logs_created ON public.admin_audit_logs USING btree (created_at DESC);


--
-- Name: idx_admin_audit_logs_entity; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_admin_audit_logs_entity ON public.admin_audit_logs USING btree (entity_type, entity_id);


--
-- Name: idx_ai_history_action; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_ai_history_action ON public.cms_ai_assist_history USING btree (action);


--
-- Name: idx_ai_history_content; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_ai_history_content ON public.cms_ai_assist_history USING btree (content_id);


--
-- Name: idx_ai_history_created; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_ai_history_created ON public.cms_ai_assist_history USING btree (created_at DESC);


--
-- Name: idx_ai_history_user; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_ai_history_user ON public.cms_ai_assist_history USING btree (user_id);


--
-- Name: idx_analytics_created; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_analytics_created ON public.analytics_events USING btree (created_at);


--
-- Name: idx_analytics_type; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_analytics_type ON public.analytics_events USING btree (event_type);


--
-- Name: idx_analytics_user; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_analytics_user ON public.analytics_events USING btree (user_id);


--
-- Name: idx_audit_logs_action; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_audit_logs_action ON public.audit_logs USING btree (action);


--
-- Name: idx_audit_logs_created_at; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_audit_logs_created_at ON public.audit_logs USING btree (created_at DESC);


--
-- Name: idx_audit_logs_entity; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_audit_logs_entity ON public.audit_logs USING btree (entity_type, entity_id);


--
-- Name: idx_audit_logs_failed; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_audit_logs_failed ON public.audit_logs USING btree (created_at DESC) WHERE ((status)::text = 'failed'::text);


--
-- Name: idx_audit_logs_ip; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_audit_logs_ip ON public.audit_logs USING btree (ip_address);


--
-- Name: idx_audit_logs_user; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_audit_logs_user ON public.audit_logs USING btree (user_id);


--
-- Name: idx_bunny_uploads_room_slug; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_bunny_uploads_room_slug ON public.bunny_uploads USING btree (room_slug);


--
-- Name: idx_bunny_uploads_room_status; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_bunny_uploads_room_status ON public.bunny_uploads USING btree (room_slug, status);


--
-- Name: idx_cms_asset_folders_parent; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_asset_folders_parent ON public.cms_asset_folders USING btree (parent_id);


--
-- Name: idx_cms_asset_folders_path; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_asset_folders_path ON public.cms_asset_folders USING btree (path);


--
-- Name: idx_cms_asset_folders_slug; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_asset_folders_slug ON public.cms_asset_folders USING btree (slug);


--
-- Name: idx_cms_assets_created_at; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_assets_created_at ON public.cms_assets USING btree (created_at DESC);


--
-- Name: idx_cms_assets_filename_trgm; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_assets_filename_trgm ON public.cms_assets USING gin (filename public.gin_trgm_ops) WHERE (deleted_at IS NULL);


--
-- Name: idx_cms_assets_folder; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_assets_folder ON public.cms_assets USING btree (folder_id);


--
-- Name: idx_cms_assets_mime_type; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_assets_mime_type ON public.cms_assets USING btree (mime_type);


--
-- Name: idx_cms_assets_not_deleted; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_assets_not_deleted ON public.cms_assets USING btree (id) WHERE (deleted_at IS NULL);


--
-- Name: idx_cms_assets_storage_key; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_assets_storage_key ON public.cms_assets USING btree (storage_key);


--
-- Name: idx_cms_assets_tags; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_assets_tags ON public.cms_assets USING gin (tags);


--
-- Name: idx_cms_assets_type; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_assets_type ON public.cms_assets USING btree (asset_type) WHERE (deleted_at IS NULL);


--
-- Name: idx_cms_assets_unused; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_assets_unused ON public.cms_assets USING btree (usage_count, created_at) WHERE ((usage_count = 0) AND (deleted_at IS NULL));


--
-- Name: idx_cms_assets_usage; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_assets_usage ON public.cms_assets USING btree (usage_count DESC, last_used_at DESC);


--
-- Name: idx_cms_audit_logs_action; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_audit_logs_action ON public.cms_audit_logs USING btree (action);


--
-- Name: idx_cms_audit_logs_created_at; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_audit_logs_created_at ON public.cms_audit_logs USING btree (created_at DESC);


--
-- Name: idx_cms_audit_logs_entity; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_audit_logs_entity ON public.cms_audit_logs USING btree (entity_type, entity_id);


--
-- Name: idx_cms_audit_logs_user; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_audit_logs_user ON public.cms_audit_logs USING btree (user_id);


--
-- Name: idx_cms_comments_by_content; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_comments_by_content ON public.cms_comments USING btree (content_id, created_at DESC) WHERE (deleted_at IS NULL);


--
-- Name: idx_cms_comments_content; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_comments_content ON public.cms_comments USING btree (content_id);


--
-- Name: idx_cms_comments_parent; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_comments_parent ON public.cms_comments USING btree (parent_id);


--
-- Name: idx_cms_comments_thread; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_comments_thread ON public.cms_comments USING btree (thread_id);


--
-- Name: idx_cms_comments_unresolved; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_comments_unresolved ON public.cms_comments USING btree (content_id) WHERE ((is_resolved = false) AND (deleted_at IS NULL));


--
-- Name: idx_cms_content_author; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_content_author ON public.cms_content USING btree (author_id);


--
-- Name: idx_cms_content_blocks; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_content_blocks ON public.cms_content USING gin (content_blocks);


--
-- Name: idx_cms_content_custom_fields; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_content_custom_fields ON public.cms_content USING gin (custom_fields);


--
-- Name: idx_cms_content_dashboard; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_content_dashboard ON public.cms_content USING btree (content_type, status, updated_at DESC) WHERE (deleted_at IS NULL);


--
-- Name: idx_cms_content_locale; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_content_locale ON public.cms_content USING btree (locale);


--
-- Name: idx_cms_content_not_deleted; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_content_not_deleted ON public.cms_content USING btree (id) WHERE (deleted_at IS NULL);


--
-- Name: idx_cms_content_published; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_content_published ON public.cms_content USING btree (published_at DESC) WHERE (status = 'published'::public.cms_content_status);


--
-- Name: idx_cms_content_recent_published; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_content_recent_published ON public.cms_content USING btree (published_at DESC) WHERE ((status = 'published'::public.cms_content_status) AND (deleted_at IS NULL));


--
-- Name: idx_cms_content_scheduled; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_content_scheduled ON public.cms_content USING btree (scheduled_publish_at) WHERE (scheduled_publish_at IS NOT NULL);


--
-- Name: idx_cms_content_search; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_content_search ON public.cms_content USING gin (search_vector);


--
-- Name: idx_cms_content_slug; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_content_slug ON public.cms_content USING btree (slug);


--
-- Name: idx_cms_content_status; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_content_status ON public.cms_content USING btree (status);


--
-- Name: idx_cms_content_tags_content; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_content_tags_content ON public.cms_content_tags USING btree (content_id);


--
-- Name: idx_cms_content_tags_tag; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_content_tags_tag ON public.cms_content_tags USING btree (tag_id);


--
-- Name: idx_cms_content_title_trgm; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_content_title_trgm ON public.cms_content USING gin (title public.gin_trgm_ops) WHERE (deleted_at IS NULL);


--
-- Name: idx_cms_content_type; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_content_type ON public.cms_content USING btree (content_type);


--
-- Name: idx_cms_content_type_status; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_content_type_status ON public.cms_content USING btree (content_type, status) WHERE (deleted_at IS NULL);


--
-- Name: idx_cms_datasource_entries_datasource; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_datasource_entries_datasource ON public.cms_datasource_entries USING btree (datasource_id);


--
-- Name: idx_cms_datasource_entries_dimension; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_datasource_entries_dimension ON public.cms_datasource_entries USING btree (datasource_id, dimension);


--
-- Name: idx_cms_datasource_entries_search; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_datasource_entries_search ON public.cms_datasource_entries USING gin (to_tsvector('english'::regconfig, (((COALESCE(name, ''::character varying))::text || ' '::text) || (COALESCE(value, ''::character varying))::text)));


--
-- Name: idx_cms_datasource_entries_sort; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_datasource_entries_sort ON public.cms_datasource_entries USING btree (datasource_id, dimension, sort_order);


--
-- Name: idx_cms_datasource_entries_value; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_datasource_entries_value ON public.cms_datasource_entries USING btree (datasource_id, value);


--
-- Name: idx_cms_datasources_created_by; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_datasources_created_by ON public.cms_datasources USING btree (created_by) WHERE (deleted_at IS NULL);


--
-- Name: idx_cms_datasources_deleted_at; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_datasources_deleted_at ON public.cms_datasources USING btree (deleted_at) WHERE (deleted_at IS NULL);


--
-- Name: idx_cms_datasources_name; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_datasources_name ON public.cms_datasources USING btree (name) WHERE (deleted_at IS NULL);


--
-- Name: idx_cms_datasources_search; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_datasources_search ON public.cms_datasources USING gin (to_tsvector('english'::regconfig, (((COALESCE(name, ''::character varying))::text || ' '::text) || COALESCE(description, ''::text)))) WHERE (deleted_at IS NULL);


--
-- Name: idx_cms_datasources_slug; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_datasources_slug ON public.cms_datasources USING btree (slug) WHERE (deleted_at IS NULL);


--
-- Name: idx_cms_jobs_content; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_jobs_content ON public.cms_scheduled_jobs USING btree (content_id);


--
-- Name: idx_cms_jobs_pending; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_jobs_pending ON public.cms_scheduled_jobs USING btree (scheduled_at) WHERE ((status)::text = 'pending'::text);


--
-- Name: idx_cms_jobs_status; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_jobs_status ON public.cms_scheduled_jobs USING btree (status);


--
-- Name: idx_cms_jobs_type; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_jobs_type ON public.cms_scheduled_jobs USING btree (job_type);


--
-- Name: idx_cms_navigation_active; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_navigation_active ON public.cms_navigation_menus USING btree (is_active) WHERE (is_active = true);


--
-- Name: idx_cms_navigation_location; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_navigation_location ON public.cms_navigation_menus USING btree (location);


--
-- Name: idx_cms_navigation_slug; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_navigation_slug ON public.cms_navigation_menus USING btree (slug);


--
-- Name: idx_cms_presets_block_type; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_presets_block_type ON public.cms_presets USING btree (block_type) WHERE (deleted_at IS NULL);


--
-- Name: idx_cms_presets_category; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_presets_category ON public.cms_presets USING btree (category) WHERE (deleted_at IS NULL);


--
-- Name: idx_cms_presets_created_by; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_presets_created_by ON public.cms_presets USING btree (created_by) WHERE (deleted_at IS NULL);


--
-- Name: idx_cms_presets_defaults; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_presets_defaults ON public.cms_presets USING btree (block_type, is_default) WHERE ((is_default = true) AND (deleted_at IS NULL));


--
-- Name: idx_cms_presets_deleted_at; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_presets_deleted_at ON public.cms_presets USING btree (deleted_at) WHERE (deleted_at IS NULL);


--
-- Name: idx_cms_presets_global; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_presets_global ON public.cms_presets USING btree (is_global, block_type, category) WHERE (deleted_at IS NULL);


--
-- Name: idx_cms_presets_popular; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_presets_popular ON public.cms_presets USING btree (usage_count DESC) WHERE (deleted_at IS NULL);


--
-- Name: idx_cms_presets_search; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_presets_search ON public.cms_presets USING gin (to_tsvector('english'::regconfig, (((COALESCE(name, ''::character varying))::text || ' '::text) || COALESCE(description, ''::text)))) WHERE (deleted_at IS NULL);


--
-- Name: idx_cms_presets_tags; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_presets_tags ON public.cms_presets USING gin (tags) WHERE (deleted_at IS NULL);


--
-- Name: idx_cms_preview_tokens_content; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_preview_tokens_content ON public.cms_preview_tokens USING btree (content_id);


--
-- Name: idx_cms_preview_tokens_expires; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_preview_tokens_expires ON public.cms_preview_tokens USING btree (expires_at);


--
-- Name: idx_cms_preview_tokens_token; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_preview_tokens_token ON public.cms_preview_tokens USING btree (token);


--
-- Name: idx_cms_redirects_active; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_redirects_active ON public.cms_redirects USING btree (is_active) WHERE (is_active = true);


--
-- Name: idx_cms_redirects_source; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_redirects_source ON public.cms_redirects USING btree (source_path);


--
-- Name: idx_cms_relations_source; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_relations_source ON public.cms_content_relations USING btree (source_id);


--
-- Name: idx_cms_relations_target; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_relations_target ON public.cms_content_relations USING btree (target_id);


--
-- Name: idx_cms_relations_type; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_relations_type ON public.cms_content_relations USING btree (relation_type);


--
-- Name: idx_cms_release_items_content; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_release_items_content ON public.cms_release_items USING btree (content_id);


--
-- Name: idx_cms_release_items_order; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_release_items_order ON public.cms_release_items USING btree (release_id, order_index);


--
-- Name: idx_cms_release_items_release; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_release_items_release ON public.cms_release_items USING btree (release_id);


--
-- Name: idx_cms_releases_created_by; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_releases_created_by ON public.cms_releases USING btree (created_by);


--
-- Name: idx_cms_releases_scheduled_at; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_releases_scheduled_at ON public.cms_releases USING btree (scheduled_at) WHERE (status = 'scheduled'::public.cms_release_status);


--
-- Name: idx_cms_releases_status; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_releases_status ON public.cms_releases USING btree (status);


--
-- Name: idx_cms_reusable_blocks_cms_user_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_reusable_blocks_cms_user_id ON public.cms_reusable_blocks USING btree (cms_user_id) WHERE (cms_user_id IS NOT NULL);


--
-- Name: idx_cms_reusable_blocks_deleted_at; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_reusable_blocks_deleted_at ON public.cms_reusable_blocks USING btree (deleted_at) WHERE (deleted_at IS NULL);


--
-- Name: idx_cms_revisions_content; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_revisions_content ON public.cms_revisions USING btree (content_id);


--
-- Name: idx_cms_revisions_created; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_revisions_created ON public.cms_revisions USING btree (created_at DESC);


--
-- Name: idx_cms_revisions_current; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_revisions_current ON public.cms_revisions USING btree (content_id) WHERE (is_current = true);


--
-- Name: idx_cms_schedule_history_content; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_schedule_history_content ON public.cms_schedule_history USING btree (content_id) WHERE (content_id IS NOT NULL);


--
-- Name: idx_cms_schedule_history_created_at; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_schedule_history_created_at ON public.cms_schedule_history USING btree (created_at DESC);


--
-- Name: idx_cms_schedule_history_event_type; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_schedule_history_event_type ON public.cms_schedule_history USING btree (event_type);


--
-- Name: idx_cms_schedule_history_release; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_schedule_history_release ON public.cms_schedule_history USING btree (release_id) WHERE (release_id IS NOT NULL);


--
-- Name: idx_cms_schedule_history_schedule; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_schedule_history_schedule ON public.cms_schedule_history USING btree (schedule_id) WHERE (schedule_id IS NOT NULL);


--
-- Name: idx_cms_schedules_content; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_schedules_content ON public.cms_schedules USING btree (content_id);


--
-- Name: idx_cms_schedules_created_by; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_schedules_created_by ON public.cms_schedules USING btree (created_by);


--
-- Name: idx_cms_schedules_scheduled_at; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_schedules_scheduled_at ON public.cms_schedules USING btree (scheduled_at) WHERE (status = 'pending'::public.cms_schedule_status);


--
-- Name: idx_cms_schedules_status; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_schedules_status ON public.cms_schedules USING btree (status) WHERE (status = ANY (ARRAY['pending'::public.cms_schedule_status, 'processing'::public.cms_schedule_status]));


--
-- Name: idx_cms_tags_name_trgm; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_tags_name_trgm ON public.cms_tags USING gin (name public.gin_trgm_ops);


--
-- Name: idx_cms_tags_parent; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_tags_parent ON public.cms_tags USING btree (parent_id);


--
-- Name: idx_cms_tags_slug; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_tags_slug ON public.cms_tags USING btree (slug);


--
-- Name: idx_cms_tags_usage; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_tags_usage ON public.cms_tags USING btree (usage_count DESC);


--
-- Name: idx_cms_users_active; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_users_active ON public.cms_users USING btree (is_active) WHERE (is_active = true);


--
-- Name: idx_cms_users_role; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_users_role ON public.cms_users USING btree (role);


--
-- Name: idx_cms_users_user_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_users_user_id ON public.cms_users USING btree (user_id);


--
-- Name: idx_cms_webhook_deliveries_created; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_webhook_deliveries_created ON public.cms_webhook_deliveries USING btree (created_at DESC);


--
-- Name: idx_cms_webhook_deliveries_next_retry; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_webhook_deliveries_next_retry ON public.cms_webhook_deliveries USING btree (next_retry_at) WHERE (next_retry_at IS NOT NULL);


--
-- Name: idx_cms_webhook_deliveries_retry; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_webhook_deliveries_retry ON public.cms_webhook_deliveries USING btree (next_retry_at) WHERE ((status)::text = 'pending'::text);


--
-- Name: idx_cms_webhook_deliveries_status; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_webhook_deliveries_status ON public.cms_webhook_deliveries USING btree (status);


--
-- Name: idx_cms_webhook_deliveries_webhook; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_webhook_deliveries_webhook ON public.cms_webhook_deliveries USING btree (webhook_id);


--
-- Name: idx_cms_webhooks_active; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_webhooks_active ON public.cms_webhooks USING btree (is_active) WHERE (is_active = true);


--
-- Name: idx_cms_webhooks_events; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_webhooks_events ON public.cms_webhooks USING gin (events);


--
-- Name: idx_cms_workflow_history_content; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_workflow_history_content ON public.cms_workflow_history USING btree (content_id);


--
-- Name: idx_cms_workflow_history_created_at; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_workflow_history_created_at ON public.cms_workflow_history USING btree (created_at DESC);


--
-- Name: idx_cms_workflow_history_workflow; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_workflow_history_workflow ON public.cms_workflow_history USING btree (workflow_id);


--
-- Name: idx_cms_workflow_log_actor; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_workflow_log_actor ON public.cms_workflow_log USING btree (transitioned_by);


--
-- Name: idx_cms_workflow_log_content; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_workflow_log_content ON public.cms_workflow_log USING btree (content_id);


--
-- Name: idx_cms_workflow_log_created; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_workflow_log_created ON public.cms_workflow_log USING btree (created_at DESC);


--
-- Name: idx_cms_workflow_status_assigned; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_workflow_status_assigned ON public.cms_workflow_status USING btree (assigned_to) WHERE (assigned_to IS NOT NULL);


--
-- Name: idx_cms_workflow_status_content; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_workflow_status_content ON public.cms_workflow_status USING btree (content_id);


--
-- Name: idx_cms_workflow_status_due_date; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_workflow_status_due_date ON public.cms_workflow_status USING btree (due_date) WHERE (due_date IS NOT NULL);


--
-- Name: idx_cms_workflow_status_stage; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_cms_workflow_status_stage ON public.cms_workflow_status USING btree (current_stage);


--
-- Name: idx_comment_notifications_user; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_comment_notifications_user ON public.cms_comment_notifications USING btree (user_id, is_read);


--
-- Name: idx_contacts_email; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_contacts_email ON public.contacts USING btree (email);


--
-- Name: idx_contacts_status; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_contacts_status ON public.contacts USING btree (status);


--
-- Name: idx_content_versions_created_at; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_content_versions_created_at ON public.content_versions USING btree (created_at DESC);


--
-- Name: idx_content_versions_created_by; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_content_versions_created_by ON public.content_versions USING btree (created_by);


--
-- Name: idx_content_versions_current; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_content_versions_current ON public.content_versions USING btree (content_type, content_id) WHERE (is_current = true);


--
-- Name: idx_content_versions_lookup; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_content_versions_lookup ON public.content_versions USING btree (content_type, content_id);


--
-- Name: idx_coupons_code; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_coupons_code ON public.coupons USING btree (code);


--
-- Name: idx_coupons_stripe_coupon_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_coupons_stripe_coupon_id ON public.coupons USING btree (stripe_coupon_id) WHERE (stripe_coupon_id IS NOT NULL);


--
-- Name: idx_course_categories_parent; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_course_categories_parent ON public.course_categories USING btree (parent_id);


--
-- Name: idx_course_categories_slug; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_course_categories_slug ON public.course_categories USING btree (slug);


--
-- Name: idx_course_downloads_course; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_course_downloads_course ON public.course_downloads USING btree (course_id);


--
-- Name: idx_course_downloads_lesson; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_course_downloads_lesson ON public.course_downloads USING btree (lesson_id);


--
-- Name: idx_course_lessons_section; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_course_lessons_section ON public.course_lessons USING btree (section_id, sort_order);


--
-- Name: idx_course_modules_v2_course; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_course_modules_v2_course ON public.course_modules_v2 USING btree (course_id, sort_order);


--
-- Name: idx_course_quizzes_course; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_course_quizzes_course ON public.course_quizzes USING btree (course_id);


--
-- Name: idx_course_quizzes_lesson; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_course_quizzes_lesson ON public.course_quizzes USING btree (lesson_id);


--
-- Name: idx_course_quizzes_module; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_course_quizzes_module ON public.course_quizzes USING btree (module_id);


--
-- Name: idx_course_reviews_course; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_course_reviews_course ON public.course_reviews USING btree (course_id);


--
-- Name: idx_course_reviews_rating; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_course_reviews_rating ON public.course_reviews USING btree (course_id, rating);


--
-- Name: idx_course_reviews_user; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_course_reviews_user ON public.course_reviews USING btree (user_id);


--
-- Name: idx_course_sections_course; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_course_sections_course ON public.course_sections USING btree (course_id, sort_order);


--
-- Name: idx_course_tags_slug; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_course_tags_slug ON public.course_tags USING btree (slug);


--
-- Name: idx_courses_enhanced_published; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_courses_enhanced_published ON public.courses_enhanced USING btree (is_published);


--
-- Name: idx_courses_enhanced_slug; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_courses_enhanced_slug ON public.courses_enhanced USING btree (slug);


--
-- Name: idx_courses_instructor; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_courses_instructor ON public.courses USING btree (instructor_id);


--
-- Name: idx_courses_published; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_courses_published ON public.courses USING btree (is_published) WHERE (is_published = true);


--
-- Name: idx_courses_slug; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_courses_slug ON public.courses USING btree (slug);


--
-- Name: idx_crm_abandoned_carts_contact; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_crm_abandoned_carts_contact ON public.crm_abandoned_carts USING btree (contact_id);


--
-- Name: idx_crm_abandoned_carts_status; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_crm_abandoned_carts_status ON public.crm_abandoned_carts USING btree (status, created_at DESC);


--
-- Name: idx_crm_deal_activities_deal; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_crm_deal_activities_deal ON public.crm_deal_activities USING btree (deal_id, created_at DESC);


--
-- Name: idx_crm_deal_history_deal; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_crm_deal_history_deal ON public.crm_deal_stage_history USING btree (deal_id, created_at DESC);


--
-- Name: idx_crm_deals_contact; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_crm_deals_contact ON public.crm_deals USING btree (contact_id);


--
-- Name: idx_crm_deals_expected_close; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_crm_deals_expected_close ON public.crm_deals USING btree (expected_close_date) WHERE ((status)::text = 'open'::text);


--
-- Name: idx_crm_deals_owner; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_crm_deals_owner ON public.crm_deals USING btree (owner_id);


--
-- Name: idx_crm_deals_pipeline; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_crm_deals_pipeline ON public.crm_deals USING btree (pipeline_id, stage_id);


--
-- Name: idx_crm_deals_status; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_crm_deals_status ON public.crm_deals USING btree (status, created_at DESC);


--
-- Name: idx_crm_pipelines_active; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_crm_pipelines_active ON public.crm_pipelines USING btree (is_active, "position");


--
-- Name: idx_crm_pipelines_default; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX idx_crm_pipelines_default ON public.crm_pipelines USING btree (is_default) WHERE (is_default = true);


--
-- Name: idx_crm_stages_pipeline; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_crm_stages_pipeline ON public.crm_pipeline_stages USING btree (pipeline_id, "position");


--
-- Name: idx_email_logs_queued_at_desc; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_email_logs_queued_at_desc ON public.email_logs USING btree (queued_at DESC);


--
-- Name: idx_email_logs_status_queued_at; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_email_logs_status_queued_at ON public.email_logs USING btree (status, queued_at DESC);


--
-- Name: idx_email_verification_expires; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_email_verification_expires ON public.email_verification_tokens USING btree (expires_at);


--
-- Name: idx_email_verification_token; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_email_verification_token ON public.email_verification_tokens USING btree (token);


--
-- Name: idx_email_verification_user; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_email_verification_user ON public.email_verification_tokens USING btree (user_id);


--
-- Name: idx_enrollments_course; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_enrollments_course ON public.user_course_enrollments USING btree (course_id);


--
-- Name: idx_enrollments_user; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_enrollments_user ON public.user_course_enrollments USING btree (user_id);


--
-- Name: idx_exceptions_date; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_exceptions_date ON public.schedule_exceptions USING btree (exception_date);


--
-- Name: idx_exceptions_schedule; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_exceptions_schedule ON public.schedule_exceptions USING btree (schedule_id);


--
-- Name: idx_exceptions_type; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_exceptions_type ON public.schedule_exceptions USING btree (exception_type);


--
-- Name: idx_form_submissions_form_created; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_form_submissions_form_created ON public.form_submissions USING btree (form_id, created_at DESC);


--
-- Name: idx_form_submissions_status; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_form_submissions_status ON public.form_submissions USING btree (status);


--
-- Name: idx_forms_published; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_forms_published ON public.forms USING btree (is_published) WHERE (is_published = true);


--
-- Name: idx_indicators_enhanced_slug; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_indicators_enhanced_slug ON public.indicators_enhanced USING btree (slug);


--
-- Name: idx_indicators_platform; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_indicators_platform ON public.indicators USING btree (platform);


--
-- Name: idx_indicators_slug; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_indicators_slug ON public.indicators USING btree (slug);


--
-- Name: idx_integration_webhooks_connection; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_integration_webhooks_connection ON public.integration_webhooks USING btree (connection_id);


--
-- Name: idx_jobs_pending; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_jobs_pending ON public.jobs USING btree (status, run_at) WHERE ((status)::text = 'pending'::text);


--
-- Name: idx_jobs_queue; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_jobs_queue ON public.jobs USING btree (queue);


--
-- Name: idx_jobs_run_at; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_jobs_run_at ON public.jobs USING btree (run_at);


--
-- Name: idx_jobs_status; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_jobs_status ON public.jobs USING btree (status);


--
-- Name: idx_jobs_type; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_jobs_type ON public.jobs USING btree (job_type);


--
-- Name: idx_lesson_progress_user_course; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_lesson_progress_user_course ON public.user_lesson_progress USING btree (user_id, course_id);


--
-- Name: idx_lessons_course; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_lessons_course ON public.lessons USING btree (course_id);


--
-- Name: idx_media_collection; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_media_collection ON public.media USING btree (collection);


--
-- Name: idx_media_filename; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_media_filename ON public.media USING btree (filename);


--
-- Name: idx_member_audit_logs_action; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_member_audit_logs_action ON public.member_audit_logs USING btree (action, created_at DESC);


--
-- Name: idx_member_audit_logs_actor; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_member_audit_logs_actor ON public.member_audit_logs USING btree (actor_id, created_at DESC);


--
-- Name: idx_member_audit_logs_entity; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_member_audit_logs_entity ON public.member_audit_logs USING btree (entity_type, entity_id);


--
-- Name: idx_member_audit_logs_user; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_member_audit_logs_user ON public.member_audit_logs USING btree (user_id, created_at DESC);


--
-- Name: idx_membership_plans_billing; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_membership_plans_billing ON public.membership_plans USING btree (billing_cycle);


--
-- Name: idx_membership_plans_room; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_membership_plans_room ON public.membership_plans USING btree (room_id);


--
-- Name: idx_mfa_attempts_ip_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_mfa_attempts_ip_time ON public.mfa_attempts USING btree (ip_address, created_at DESC);


--
-- Name: idx_mfa_attempts_user_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_mfa_attempts_user_time ON public.mfa_attempts USING btree (user_id, created_at DESC);


--
-- Name: idx_mpph_changed_at; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_mpph_changed_at ON public.membership_plan_price_history USING btree (changed_at DESC);


--
-- Name: idx_mpph_new_price; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_mpph_new_price ON public.membership_plan_price_history USING btree (new_stripe_price_id);


--
-- Name: idx_mpph_plan_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_mpph_plan_id ON public.membership_plan_price_history USING btree (plan_id);


--
-- Name: idx_mv_resource_analytics; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX idx_mv_resource_analytics ON public.mv_resource_analytics USING btree (trading_room_id, resource_type, access_level);


--
-- Name: idx_newsletter_email; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_newsletter_email ON public.newsletter_subscribers USING btree (email);


--
-- Name: idx_newsletter_status; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_newsletter_status ON public.newsletter_subscribers USING btree (status);


--
-- Name: idx_oauth_audit_log_created; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_oauth_audit_log_created ON public.oauth_audit_log USING btree (created_at);


--
-- Name: idx_oauth_audit_log_user; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_oauth_audit_log_user ON public.oauth_audit_log USING btree (user_id);


--
-- Name: idx_oauth_states_expires; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_oauth_states_expires ON public.oauth_states USING btree (expires_at);


--
-- Name: idx_oauth_states_state; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_oauth_states_state ON public.oauth_states USING btree (state);


--
-- Name: idx_orders_created_at; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_orders_created_at ON public.orders USING btree (created_at DESC);


--
-- Name: idx_orders_number; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_orders_number ON public.orders USING btree (order_number);


--
-- Name: idx_orders_status; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_orders_status ON public.orders USING btree (status);


--
-- Name: idx_orders_stripe_session; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_orders_stripe_session ON public.orders USING btree (stripe_session_id) WHERE (stripe_session_id IS NOT NULL);


--
-- Name: idx_orders_user; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_orders_user ON public.orders USING btree (user_id);


--
-- Name: idx_orders_user_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_orders_user_id ON public.orders USING btree (user_id);


--
-- Name: idx_password_resets_email; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_password_resets_email ON public.password_resets USING btree (email);


--
-- Name: idx_password_resets_expires; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_password_resets_expires ON public.password_resets USING btree (expires_at);


--
-- Name: idx_password_resets_token; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_password_resets_token ON public.password_resets USING btree (token);


--
-- Name: idx_payment_disputes_charge; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_payment_disputes_charge ON public.payment_disputes USING btree (stripe_charge_id);


--
-- Name: idx_popup_events_analytics; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_popup_events_analytics ON public.popup_events USING btree (popup_id, event_type, created_at);


--
-- Name: idx_popup_events_created; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_popup_events_created ON public.popup_events USING btree (created_at);


--
-- Name: idx_popup_events_device; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_popup_events_device ON public.popup_events USING btree (device_type) WHERE (device_type IS NOT NULL);


--
-- Name: idx_popup_events_popup; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_popup_events_popup ON public.popup_events USING btree (popup_id);


--
-- Name: idx_popup_events_session; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_popup_events_session ON public.popup_events USING btree (session_id) WHERE (session_id IS NOT NULL);


--
-- Name: idx_popup_events_type; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_popup_events_type ON public.popup_events USING btree (event_type);


--
-- Name: idx_popup_submissions_form; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_popup_submissions_form ON public.popup_form_submissions USING btree (form_id) WHERE (form_id IS NOT NULL);


--
-- Name: idx_popup_submissions_popup; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_popup_submissions_popup ON public.popup_form_submissions USING btree (popup_id);


--
-- Name: idx_popups_ab_test; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_popups_ab_test ON public.popups USING btree (ab_test_id) WHERE (ab_test_id IS NOT NULL);


--
-- Name: idx_popups_priority; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_popups_priority ON public.popups USING btree (priority DESC);


--
-- Name: idx_popups_schedule; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_popups_schedule ON public.popups USING btree (start_date, end_date) WHERE ((status)::text = 'published'::text);


--
-- Name: idx_popups_status; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_popups_status ON public.popups USING btree (status);


--
-- Name: idx_popups_type; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_popups_type ON public.popups USING btree (type);


--
-- Name: idx_posts_author; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_posts_author ON public.posts USING btree (author_id);


--
-- Name: idx_posts_published; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_posts_published ON public.posts USING btree (status, published_at);


--
-- Name: idx_posts_published_at; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_posts_published_at ON public.posts USING btree (published_at DESC) WHERE (published_at IS NOT NULL);


--
-- Name: idx_posts_scheduled_publish; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_posts_scheduled_publish ON public.posts USING btree (scheduled_publish_at) WHERE ((scheduled_publish_at IS NOT NULL) AND ((status)::text = 'scheduled'::text));


--
-- Name: idx_posts_slug; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_posts_slug ON public.posts USING btree (slug);


--
-- Name: idx_posts_status; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_posts_status ON public.posts USING btree (status);


--
-- Name: idx_preview_tokens_content; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_preview_tokens_content ON public.preview_tokens USING btree (content_type, content_id);


--
-- Name: idx_preview_tokens_expires; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_preview_tokens_expires ON public.preview_tokens USING btree (expires_at);


--
-- Name: idx_preview_tokens_token; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_preview_tokens_token ON public.preview_tokens USING btree (token);


--
-- Name: idx_products_active; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_products_active ON public.products USING btree (is_active);


--
-- Name: idx_products_active_type; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_products_active_type ON public.products USING btree (is_active, type) WHERE (is_active = true);


--
-- Name: idx_products_created_at; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_products_created_at ON public.products USING btree (created_at DESC);


--
-- Name: idx_products_name_trgm; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_products_name_trgm ON public.products USING gin (name public.gin_trgm_ops);


--
-- Name: idx_products_slug; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_products_slug ON public.products USING btree (slug);


--
-- Name: idx_products_type; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_products_type ON public.products USING btree (type);


--
-- Name: idx_quiz_answers_question; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_quiz_answers_question ON public.quiz_answers USING btree (question_id);


--
-- Name: idx_quiz_questions_quiz; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_quiz_questions_quiz ON public.quiz_questions USING btree (quiz_id);


--
-- Name: idx_reconciliation_log_run_at; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_reconciliation_log_run_at ON public.reconciliation_log USING btree (run_at DESC);


--
-- Name: idx_redirects_created_at; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_redirects_created_at ON public.redirects USING btree (created_at DESC);


--
-- Name: idx_redirects_from_path; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_redirects_from_path ON public.redirects USING btree (from_path);


--
-- Name: idx_redirects_is_active; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_redirects_is_active ON public.redirects USING btree (is_active);


--
-- Name: idx_resource_access_recent; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_resource_access_recent ON public.resource_access_log USING btree (user_id, accessed_at DESC);


--
-- Name: idx_resource_access_resource; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_resource_access_resource ON public.resource_access_log USING btree (resource_id);


--
-- Name: idx_resource_access_user; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_resource_access_user ON public.resource_access_log USING btree (user_id);


--
-- Name: idx_resource_downloads_date; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_resource_downloads_date ON public.resource_download_logs USING btree (downloaded_at DESC);


--
-- Name: idx_resource_downloads_resource; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_resource_downloads_resource ON public.resource_download_logs USING btree (resource_id);


--
-- Name: idx_resource_downloads_user; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_resource_downloads_user ON public.resource_download_logs USING btree (user_id) WHERE (user_id IS NOT NULL);


--
-- Name: idx_resource_favorites_user; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_resource_favorites_user ON public.resource_favorites USING btree (user_id);


--
-- Name: idx_reusable_block_usage_block; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_reusable_block_usage_block ON public.cms_reusable_block_usage USING btree (reusable_block_id);


--
-- Name: idx_reusable_block_usage_content; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_reusable_block_usage_content ON public.cms_reusable_block_usage USING btree (content_id);


--
-- Name: idx_reusable_blocks_category; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_reusable_blocks_category ON public.cms_reusable_blocks USING btree (category);


--
-- Name: idx_reusable_blocks_name_search; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_reusable_blocks_name_search ON public.cms_reusable_blocks USING gin (to_tsvector('english'::regconfig, (((name)::text || ' '::text) || COALESCE(description, ''::text))));


--
-- Name: idx_reusable_blocks_tags; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_reusable_blocks_tags ON public.cms_reusable_blocks USING gin (tags);


--
-- Name: idx_room_alerts_fts; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_room_alerts_fts ON public.room_alerts USING gin (to_tsvector('english'::regconfig, (((((((COALESCE(ticker, ''::character varying))::text || ' '::text) || (COALESCE(title, ''::character varying))::text) || ' '::text) || COALESCE(message, ''::text)) || ' '::text) || COALESCE(notes, ''::text))));


--
-- Name: INDEX idx_room_alerts_fts; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON INDEX public.idx_room_alerts_fts IS 'GIN index for full-text search on room alerts (ticker, title, message, notes)';


--
-- Name: idx_room_alerts_fts_date; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_room_alerts_fts_date ON public.room_alerts USING btree (room_slug, published_at DESC) WHERE ((deleted_at IS NULL) AND (is_published = true));


--
-- Name: idx_room_alerts_published; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_room_alerts_published ON public.room_alerts USING btree (is_published, published_at DESC);


--
-- Name: idx_room_alerts_room_published; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_room_alerts_room_published ON public.room_alerts USING btree (room_slug, published_at DESC) WHERE ((deleted_at IS NULL) AND (is_published = true));


--
-- Name: idx_room_alerts_room_slug; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_room_alerts_room_slug ON public.room_alerts USING btree (room_slug);


--
-- Name: idx_room_alerts_ticker; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_room_alerts_ticker ON public.room_alerts USING btree (ticker);


--
-- Name: idx_room_alerts_ticker_published; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_room_alerts_ticker_published ON public.room_alerts USING btree (room_slug, ticker, published_at DESC) WHERE ((deleted_at IS NULL) AND (is_published = true));


--
-- Name: idx_room_alerts_ticker_upper; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_room_alerts_ticker_upper ON public.room_alerts USING btree (upper((ticker)::text), room_slug) WHERE (deleted_at IS NULL);


--
-- Name: idx_room_alerts_type; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_room_alerts_type ON public.room_alerts USING btree (alert_type);


--
-- Name: idx_room_resources_access_level; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_room_resources_access_level ON public.room_resources USING btree (access_level);


--
-- Name: idx_room_resources_content_type; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_room_resources_content_type ON public.room_resources USING btree (content_type);


--
-- Name: idx_room_resources_course_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_room_resources_course_id ON public.room_resources USING btree (course_id) WHERE (course_id IS NOT NULL);


--
-- Name: idx_room_resources_lesson_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_room_resources_lesson_id ON public.room_resources USING btree (lesson_id) WHERE (lesson_id IS NOT NULL);


--
-- Name: idx_room_resources_resource_date; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_room_resources_resource_date ON public.room_resources USING btree (resource_date DESC);


--
-- Name: idx_room_resources_room; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_room_resources_room ON public.room_resources USING btree (room_id, resource_type);


--
-- Name: idx_room_resources_section; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_room_resources_section ON public.room_resources USING btree (section);


--
-- Name: idx_room_resources_trader; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_room_resources_trader ON public.room_resources USING btree (trader_id) WHERE (trader_id IS NOT NULL);


--
-- Name: idx_room_resources_trading_room_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_room_resources_trading_room_id ON public.room_resources USING btree (trading_room_id);


--
-- Name: idx_room_resources_version; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_room_resources_version ON public.room_resources USING btree (is_latest_version) WHERE (is_latest_version = true);


--
-- Name: idx_room_trade_plans_fts; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_room_trade_plans_fts ON public.room_trade_plans USING gin (to_tsvector('english'::regconfig, (((((COALESCE(ticker, ''::character varying))::text || ' '::text) || COALESCE(notes, ''::text)) || ' '::text) || (COALESCE(bias, ''::character varying))::text)));


--
-- Name: INDEX idx_room_trade_plans_fts; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON INDEX public.idx_room_trade_plans_fts IS 'GIN index for full-text search on room trade plans (ticker, notes, bias)';


--
-- Name: idx_room_trade_plans_fts_date; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_room_trade_plans_fts_date ON public.room_trade_plans USING btree (room_slug, week_of DESC) WHERE ((deleted_at IS NULL) AND (is_active = true));


--
-- Name: idx_room_trade_plans_room; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_room_trade_plans_room ON public.room_trade_plans USING btree (room_slug, week_of DESC);


--
-- Name: idx_room_trade_plans_room_week; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_room_trade_plans_room_week ON public.room_trade_plans USING btree (room_slug, week_of DESC) WHERE (deleted_at IS NULL);


--
-- Name: idx_room_trade_plans_ticker; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_room_trade_plans_ticker ON public.room_trade_plans USING btree (ticker);


--
-- Name: idx_room_trade_plans_ticker_upper; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_room_trade_plans_ticker_upper ON public.room_trade_plans USING btree (upper((ticker)::text), room_slug) WHERE (deleted_at IS NULL);


--
-- Name: idx_room_trade_plans_ticker_week; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_room_trade_plans_ticker_week ON public.room_trade_plans USING btree (room_slug, ticker, week_of DESC) WHERE (deleted_at IS NULL);


--
-- Name: idx_room_traders_slug; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_room_traders_slug ON public.room_traders USING btree (slug);


--
-- Name: idx_room_trades_entry_date; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_room_trades_entry_date ON public.room_trades USING btree (entry_date DESC);


--
-- Name: idx_room_trades_fts; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_room_trades_fts ON public.room_trades USING gin (to_tsvector('english'::regconfig, (((COALESCE(ticker, ''::character varying))::text || ' '::text) || COALESCE(notes, ''::text))));


--
-- Name: INDEX idx_room_trades_fts; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON INDEX public.idx_room_trades_fts IS 'GIN index for full-text search on room trades (ticker, notes)';


--
-- Name: idx_room_trades_fts_date; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_room_trades_fts_date ON public.room_trades USING btree (room_slug, entry_date DESC) WHERE (deleted_at IS NULL);


--
-- Name: idx_room_trades_invalidated; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_room_trades_invalidated ON public.room_trades USING btree (status) WHERE ((status)::text = 'invalidated'::text);


--
-- Name: idx_room_trades_room; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_room_trades_room ON public.room_trades USING btree (room_slug);


--
-- Name: idx_room_trades_room_entry; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_room_trades_room_entry ON public.room_trades USING btree (room_slug, entry_date DESC) WHERE (deleted_at IS NULL);


--
-- Name: idx_room_trades_room_status; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_room_trades_room_status ON public.room_trades USING btree (room_slug, status, entry_date DESC) WHERE (deleted_at IS NULL);


--
-- Name: idx_room_trades_status; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_room_trades_status ON public.room_trades USING btree (status);


--
-- Name: idx_room_trades_ticker; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_room_trades_ticker ON public.room_trades USING btree (ticker);


--
-- Name: idx_room_trades_ticker_entry; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_room_trades_ticker_entry ON public.room_trades USING btree (room_slug, ticker, entry_date DESC) WHERE (deleted_at IS NULL);


--
-- Name: idx_room_trades_ticker_upper; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_room_trades_ticker_upper ON public.room_trades USING btree (upper((ticker)::text), room_slug) WHERE (deleted_at IS NULL);


--
-- Name: idx_room_trades_was_updated; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_room_trades_was_updated ON public.room_trades USING btree (was_updated) WHERE (was_updated = true);


--
-- Name: idx_room_weekly_videos_room; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_room_weekly_videos_room ON public.room_weekly_videos USING btree (room_slug, is_current);


--
-- Name: idx_room_weekly_videos_week; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_room_weekly_videos_week ON public.room_weekly_videos USING btree (week_of DESC);


--
-- Name: idx_scheduled_content_lookup; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_scheduled_content_lookup ON public.scheduled_content USING btree (content_type, content_id);


--
-- Name: idx_scheduled_content_pending; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_scheduled_content_pending ON public.scheduled_content USING btree (scheduled_at) WHERE ((status)::text = 'scheduled'::text);


--
-- Name: idx_schedules_active; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_schedules_active ON public.trading_room_schedules USING btree (is_active);


--
-- Name: idx_schedules_day; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_schedules_day ON public.trading_room_schedules USING btree (day_of_week);


--
-- Name: idx_schedules_effective; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_schedules_effective ON public.trading_room_schedules USING btree (effective_from, effective_until);


--
-- Name: idx_schedules_plan; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_schedules_plan ON public.trading_room_schedules USING btree (plan_id);


--
-- Name: idx_security_events_category; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_security_events_category ON public.security_events USING btree (event_category, created_at DESC);


--
-- Name: idx_security_events_severity; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_security_events_severity ON public.security_events USING btree (severity, created_at DESC);


--
-- Name: idx_security_events_user; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_security_events_user ON public.security_events USING btree (user_id, created_at DESC);


--
-- Name: idx_service_connections_category; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_service_connections_category ON public.service_connections USING btree (category);


--
-- Name: idx_service_connections_status; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_service_connections_status ON public.service_connections USING btree (status);


--
-- Name: idx_stock_lists_active; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_stock_lists_active ON public.stock_lists USING btree (is_active) WHERE (is_active = true);


--
-- Name: idx_stock_lists_featured; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_stock_lists_featured ON public.stock_lists USING btree (is_featured) WHERE (is_featured = true);


--
-- Name: idx_stock_lists_room; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_stock_lists_room ON public.stock_lists USING btree (trading_room_id);


--
-- Name: idx_stock_lists_type; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_stock_lists_type ON public.stock_lists USING btree (list_type);


--
-- Name: idx_stock_lists_week; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_stock_lists_week ON public.stock_lists USING btree (week_of DESC) WHERE (week_of IS NOT NULL);


--
-- Name: idx_sync_queue_content; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_sync_queue_content ON public.cms_offline_sync_queue USING btree (content_id);


--
-- Name: idx_sync_queue_pending; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_sync_queue_pending ON public.cms_offline_sync_queue USING btree (status, created_at) WHERE (status = 'pending'::public.cms_sync_status);


--
-- Name: idx_sync_queue_status; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_sync_queue_status ON public.cms_offline_sync_queue USING btree (status);


--
-- Name: idx_sync_queue_user; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_sync_queue_user ON public.cms_offline_sync_queue USING btree (user_id);


--
-- Name: idx_trading_rooms_active; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_trading_rooms_active ON public.trading_rooms USING btree (is_active);


--
-- Name: idx_trading_rooms_slug; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_trading_rooms_slug ON public.trading_rooms USING btree (slug);


--
-- Name: idx_transcript_default; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_transcript_default ON public.video_transcripts USING btree (video_id, is_default) WHERE (is_default = true);


--
-- Name: idx_transcript_video; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_transcript_video ON public.video_transcripts USING btree (video_id);


--
-- Name: idx_translations_content; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_translations_content ON public.content_translations USING btree (content_type, content_id);


--
-- Name: idx_translations_locale; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_translations_locale ON public.content_translations USING btree (locale);


--
-- Name: idx_translations_status; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_translations_status ON public.content_translations USING btree (status);


--
-- Name: idx_unified_videos_date; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_unified_videos_date ON public.unified_videos USING btree (video_date DESC);


--
-- Name: idx_unified_videos_room; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_unified_videos_room ON public.unified_videos USING btree (room_id, content_type);


--
-- Name: idx_unified_videos_slug; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_unified_videos_slug ON public.unified_videos USING btree (slug);


--
-- Name: idx_user_enrollments_course; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_user_enrollments_course ON public.user_course_enrollments USING btree (course_id);


--
-- Name: idx_user_enrollments_user; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_user_enrollments_user ON public.user_course_enrollments USING btree (user_id);


--
-- Name: idx_user_favorites_item; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_user_favorites_item ON public.user_favorites USING btree (item_type, item_id);


--
-- Name: idx_user_favorites_user_room; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_user_favorites_user_room ON public.user_favorites USING btree (user_id, room_slug);


--
-- Name: idx_user_memberships_deleted; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_user_memberships_deleted ON public.user_memberships USING btree (deleted_at) WHERE (deleted_at IS NOT NULL);


--
-- Name: idx_user_memberships_expires_at; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_user_memberships_expires_at ON public.user_memberships USING btree (expires_at) WHERE (expires_at IS NOT NULL);


--
-- Name: idx_user_memberships_grace_period; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_user_memberships_grace_period ON public.user_memberships USING btree (grace_period_end) WHERE ((status)::text = 'past_due'::text);


--
-- Name: idx_user_memberships_renewal_reminder; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_user_memberships_renewal_reminder ON public.user_memberships USING btree (current_period_end, renewal_reminder_sent_at) WHERE (((status)::text = 'active'::text) AND (cancel_at_period_end = false));


--
-- Name: idx_user_memberships_status; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_user_memberships_status ON public.user_memberships USING btree (status);


--
-- Name: idx_user_memberships_trial_ending; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_user_memberships_trial_ending ON public.user_memberships USING btree (trial_ends_at, trial_ending_reminder_sent_at) WHERE ((status)::text = 'trial'::text);


--
-- Name: idx_user_memberships_user; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_user_memberships_user ON public.user_memberships USING btree (user_id);


--
-- Name: idx_user_memberships_user_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_user_memberships_user_id ON public.user_memberships USING btree (user_id);


--
-- Name: idx_user_memberships_user_status; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_user_memberships_user_status ON public.user_memberships USING btree (user_id, status);


--
-- Name: idx_user_quiz_attempts_enrollment; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_user_quiz_attempts_enrollment ON public.user_quiz_attempts USING btree (enrollment_id);


--
-- Name: idx_user_quiz_attempts_quiz; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_user_quiz_attempts_quiz ON public.user_quiz_attempts USING btree (quiz_id);


--
-- Name: idx_user_quiz_attempts_user; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_user_quiz_attempts_user ON public.user_quiz_attempts USING btree (user_id);


--
-- Name: idx_users_active; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_users_active ON public.users USING btree (id) WHERE (deleted_at IS NULL);


--
-- Name: idx_users_apple_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_users_apple_id ON public.users USING btree (apple_id) WHERE (apple_id IS NOT NULL);


--
-- Name: idx_users_created_at; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_users_created_at ON public.users USING btree (created_at DESC);


--
-- Name: idx_users_deleted_at; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_users_deleted_at ON public.users USING btree (deleted_at) WHERE (deleted_at IS NOT NULL);


--
-- Name: idx_users_email; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_users_email ON public.users USING btree (email);


--
-- Name: idx_users_email_verified; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_users_email_verified ON public.users USING btree (email_verified_at) WHERE (email_verified_at IS NOT NULL);


--
-- Name: idx_users_google_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_users_google_id ON public.users USING btree (google_id) WHERE (google_id IS NOT NULL);


--
-- Name: idx_users_mfa_enabled; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_users_mfa_enabled ON public.users USING btree (mfa_enabled) WHERE (mfa_enabled = true);


--
-- Name: idx_users_oauth_provider; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_users_oauth_provider ON public.users USING btree (oauth_provider) WHERE (oauth_provider IS NOT NULL);


--
-- Name: idx_users_role; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_users_role ON public.users USING btree (role);


--
-- Name: idx_videos_popular; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_videos_popular ON public.unified_videos USING btree (views_count DESC) WHERE ((is_published = true) AND (deleted_at IS NULL));


--
-- Name: idx_videos_related; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_videos_related ON public.unified_videos USING btree (content_type, video_date DESC) WHERE ((is_published = true) AND (deleted_at IS NULL));


--
-- Name: idx_videos_tags_gin; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_videos_tags_gin ON public.unified_videos USING gin (tags jsonb_path_ops) WHERE (deleted_at IS NULL);


--
-- Name: idx_videos_weekly_watchlist; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_videos_weekly_watchlist ON public.unified_videos USING btree (video_date DESC, created_at DESC) WHERE (((content_type)::text = 'weekly_watchlist'::text) AND (is_published = true) AND (deleted_at IS NULL));


--
-- Name: idx_watch_progress_incomplete; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_watch_progress_incomplete ON public.video_watch_progress USING btree (user_id, completed, last_watched_at DESC) WHERE (completed = false);


--
-- Name: idx_watch_progress_user_date; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_watch_progress_user_date ON public.video_watch_progress USING btree (user_id, last_watched_at DESC);


--
-- Name: idx_watch_progress_video; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_watch_progress_video ON public.video_watch_progress USING btree (video_id);


--
-- Name: idx_watchlist_room_week; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_watchlist_room_week ON public.watchlist_entries USING btree (room_slug, week_of);


--
-- Name: idx_webhook_deliveries_retry; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_webhook_deliveries_retry ON public.webhook_deliveries USING btree (next_retry_at) WHERE ((status)::text = 'pending'::text);


--
-- Name: idx_webhook_deliveries_status; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_webhook_deliveries_status ON public.webhook_deliveries USING btree (status);


--
-- Name: idx_webhook_deliveries_webhook; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_webhook_deliveries_webhook ON public.webhook_deliveries USING btree (webhook_id);


--
-- Name: idx_webhook_events_unprocessed; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_webhook_events_unprocessed ON public.webhook_events USING btree (received_at) WHERE (processed_at IS NULL);


--
-- Name: idx_webhooks_active; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_webhooks_active ON public.webhooks USING btree (is_active) WHERE (is_active = true);


--
-- Name: idx_workflow_status_assigned; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_workflow_status_assigned ON public.content_workflow_status USING btree (assigned_to);


--
-- Name: idx_workflow_status_content; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_workflow_status_content ON public.content_workflow_status USING btree (content_type, content_id);


--
-- Name: idx_workflow_status_stage; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_workflow_status_stage ON public.content_workflow_status USING btree (current_stage);


--
-- Name: idx_workflow_transitions_status; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_workflow_transitions_status ON public.workflow_transitions USING btree (workflow_status_id);


--
-- Name: uq_service_connections_key_env; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX uq_service_connections_key_env ON public.service_connections USING btree (service_key, COALESCE(environment, ''::text));


--
-- Name: uq_user_memberships_active_plan; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX uq_user_memberships_active_plan ON public.user_memberships USING btree (user_id, plan_id) WHERE ((status)::text = ANY ((ARRAY['active'::character varying, 'trialing'::character varying, 'past_due'::character varying])::text[]));


--
-- Name: uq_user_memberships_stripe_subscription; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX uq_user_memberships_stripe_subscription ON public.user_memberships USING btree (stripe_subscription_id) WHERE (stripe_subscription_id IS NOT NULL);


--
-- Name: cms_audit_log_2026_05_action_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_action ATTACH PARTITION public.cms_audit_log_2026_05_action_idx;


--
-- Name: cms_audit_log_2026_05_created_at_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_created ATTACH PARTITION public.cms_audit_log_2026_05_created_at_idx;


--
-- Name: cms_audit_log_2026_05_created_at_idx1; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_failed ATTACH PARTITION public.cms_audit_log_2026_05_created_at_idx1;


--
-- Name: cms_audit_log_2026_05_entity_type_entity_id_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_entity ATTACH PARTITION public.cms_audit_log_2026_05_entity_type_entity_id_idx;


--
-- Name: cms_audit_log_2026_05_pkey; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.cms_audit_log_pkey ATTACH PARTITION public.cms_audit_log_2026_05_pkey;


--
-- Name: cms_audit_log_2026_05_user_id_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_user ATTACH PARTITION public.cms_audit_log_2026_05_user_id_idx;


--
-- Name: cms_audit_log_2026_06_action_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_action ATTACH PARTITION public.cms_audit_log_2026_06_action_idx;


--
-- Name: cms_audit_log_2026_06_created_at_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_created ATTACH PARTITION public.cms_audit_log_2026_06_created_at_idx;


--
-- Name: cms_audit_log_2026_06_created_at_idx1; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_failed ATTACH PARTITION public.cms_audit_log_2026_06_created_at_idx1;


--
-- Name: cms_audit_log_2026_06_entity_type_entity_id_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_entity ATTACH PARTITION public.cms_audit_log_2026_06_entity_type_entity_id_idx;


--
-- Name: cms_audit_log_2026_06_pkey; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.cms_audit_log_pkey ATTACH PARTITION public.cms_audit_log_2026_06_pkey;


--
-- Name: cms_audit_log_2026_06_user_id_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_user ATTACH PARTITION public.cms_audit_log_2026_06_user_id_idx;


--
-- Name: cms_audit_log_2026_07_action_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_action ATTACH PARTITION public.cms_audit_log_2026_07_action_idx;


--
-- Name: cms_audit_log_2026_07_created_at_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_created ATTACH PARTITION public.cms_audit_log_2026_07_created_at_idx;


--
-- Name: cms_audit_log_2026_07_created_at_idx1; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_failed ATTACH PARTITION public.cms_audit_log_2026_07_created_at_idx1;


--
-- Name: cms_audit_log_2026_07_entity_type_entity_id_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_entity ATTACH PARTITION public.cms_audit_log_2026_07_entity_type_entity_id_idx;


--
-- Name: cms_audit_log_2026_07_pkey; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.cms_audit_log_pkey ATTACH PARTITION public.cms_audit_log_2026_07_pkey;


--
-- Name: cms_audit_log_2026_07_user_id_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_user ATTACH PARTITION public.cms_audit_log_2026_07_user_id_idx;


--
-- Name: cms_audit_log_2026_08_action_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_action ATTACH PARTITION public.cms_audit_log_2026_08_action_idx;


--
-- Name: cms_audit_log_2026_08_created_at_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_created ATTACH PARTITION public.cms_audit_log_2026_08_created_at_idx;


--
-- Name: cms_audit_log_2026_08_created_at_idx1; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_failed ATTACH PARTITION public.cms_audit_log_2026_08_created_at_idx1;


--
-- Name: cms_audit_log_2026_08_entity_type_entity_id_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_entity ATTACH PARTITION public.cms_audit_log_2026_08_entity_type_entity_id_idx;


--
-- Name: cms_audit_log_2026_08_pkey; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.cms_audit_log_pkey ATTACH PARTITION public.cms_audit_log_2026_08_pkey;


--
-- Name: cms_audit_log_2026_08_user_id_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_user ATTACH PARTITION public.cms_audit_log_2026_08_user_id_idx;


--
-- Name: cms_audit_log_2026_09_action_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_action ATTACH PARTITION public.cms_audit_log_2026_09_action_idx;


--
-- Name: cms_audit_log_2026_09_created_at_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_created ATTACH PARTITION public.cms_audit_log_2026_09_created_at_idx;


--
-- Name: cms_audit_log_2026_09_created_at_idx1; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_failed ATTACH PARTITION public.cms_audit_log_2026_09_created_at_idx1;


--
-- Name: cms_audit_log_2026_09_entity_type_entity_id_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_entity ATTACH PARTITION public.cms_audit_log_2026_09_entity_type_entity_id_idx;


--
-- Name: cms_audit_log_2026_09_pkey; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.cms_audit_log_pkey ATTACH PARTITION public.cms_audit_log_2026_09_pkey;


--
-- Name: cms_audit_log_2026_09_user_id_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_user ATTACH PARTITION public.cms_audit_log_2026_09_user_id_idx;


--
-- Name: cms_audit_log_2026_10_action_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_action ATTACH PARTITION public.cms_audit_log_2026_10_action_idx;


--
-- Name: cms_audit_log_2026_10_created_at_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_created ATTACH PARTITION public.cms_audit_log_2026_10_created_at_idx;


--
-- Name: cms_audit_log_2026_10_created_at_idx1; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_failed ATTACH PARTITION public.cms_audit_log_2026_10_created_at_idx1;


--
-- Name: cms_audit_log_2026_10_entity_type_entity_id_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_entity ATTACH PARTITION public.cms_audit_log_2026_10_entity_type_entity_id_idx;


--
-- Name: cms_audit_log_2026_10_pkey; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.cms_audit_log_pkey ATTACH PARTITION public.cms_audit_log_2026_10_pkey;


--
-- Name: cms_audit_log_2026_10_user_id_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_user ATTACH PARTITION public.cms_audit_log_2026_10_user_id_idx;


--
-- Name: cms_audit_log_2026_11_action_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_action ATTACH PARTITION public.cms_audit_log_2026_11_action_idx;


--
-- Name: cms_audit_log_2026_11_created_at_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_created ATTACH PARTITION public.cms_audit_log_2026_11_created_at_idx;


--
-- Name: cms_audit_log_2026_11_created_at_idx1; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_failed ATTACH PARTITION public.cms_audit_log_2026_11_created_at_idx1;


--
-- Name: cms_audit_log_2026_11_entity_type_entity_id_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_entity ATTACH PARTITION public.cms_audit_log_2026_11_entity_type_entity_id_idx;


--
-- Name: cms_audit_log_2026_11_pkey; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.cms_audit_log_pkey ATTACH PARTITION public.cms_audit_log_2026_11_pkey;


--
-- Name: cms_audit_log_2026_11_user_id_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_user ATTACH PARTITION public.cms_audit_log_2026_11_user_id_idx;


--
-- Name: cms_audit_log_2026_12_action_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_action ATTACH PARTITION public.cms_audit_log_2026_12_action_idx;


--
-- Name: cms_audit_log_2026_12_created_at_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_created ATTACH PARTITION public.cms_audit_log_2026_12_created_at_idx;


--
-- Name: cms_audit_log_2026_12_created_at_idx1; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_failed ATTACH PARTITION public.cms_audit_log_2026_12_created_at_idx1;


--
-- Name: cms_audit_log_2026_12_entity_type_entity_id_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_entity ATTACH PARTITION public.cms_audit_log_2026_12_entity_type_entity_id_idx;


--
-- Name: cms_audit_log_2026_12_pkey; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.cms_audit_log_pkey ATTACH PARTITION public.cms_audit_log_2026_12_pkey;


--
-- Name: cms_audit_log_2026_12_user_id_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_user ATTACH PARTITION public.cms_audit_log_2026_12_user_id_idx;


--
-- Name: cms_audit_log_2027_01_action_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_action ATTACH PARTITION public.cms_audit_log_2027_01_action_idx;


--
-- Name: cms_audit_log_2027_01_created_at_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_created ATTACH PARTITION public.cms_audit_log_2027_01_created_at_idx;


--
-- Name: cms_audit_log_2027_01_created_at_idx1; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_failed ATTACH PARTITION public.cms_audit_log_2027_01_created_at_idx1;


--
-- Name: cms_audit_log_2027_01_entity_type_entity_id_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_entity ATTACH PARTITION public.cms_audit_log_2027_01_entity_type_entity_id_idx;


--
-- Name: cms_audit_log_2027_01_pkey; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.cms_audit_log_pkey ATTACH PARTITION public.cms_audit_log_2027_01_pkey;


--
-- Name: cms_audit_log_2027_01_user_id_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_user ATTACH PARTITION public.cms_audit_log_2027_01_user_id_idx;


--
-- Name: cms_audit_log_2027_02_action_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_action ATTACH PARTITION public.cms_audit_log_2027_02_action_idx;


--
-- Name: cms_audit_log_2027_02_created_at_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_created ATTACH PARTITION public.cms_audit_log_2027_02_created_at_idx;


--
-- Name: cms_audit_log_2027_02_created_at_idx1; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_failed ATTACH PARTITION public.cms_audit_log_2027_02_created_at_idx1;


--
-- Name: cms_audit_log_2027_02_entity_type_entity_id_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_entity ATTACH PARTITION public.cms_audit_log_2027_02_entity_type_entity_id_idx;


--
-- Name: cms_audit_log_2027_02_pkey; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.cms_audit_log_pkey ATTACH PARTITION public.cms_audit_log_2027_02_pkey;


--
-- Name: cms_audit_log_2027_02_user_id_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_user ATTACH PARTITION public.cms_audit_log_2027_02_user_id_idx;


--
-- Name: cms_audit_log_2027_03_action_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_action ATTACH PARTITION public.cms_audit_log_2027_03_action_idx;


--
-- Name: cms_audit_log_2027_03_created_at_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_created ATTACH PARTITION public.cms_audit_log_2027_03_created_at_idx;


--
-- Name: cms_audit_log_2027_03_created_at_idx1; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_failed ATTACH PARTITION public.cms_audit_log_2027_03_created_at_idx1;


--
-- Name: cms_audit_log_2027_03_entity_type_entity_id_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_entity ATTACH PARTITION public.cms_audit_log_2027_03_entity_type_entity_id_idx;


--
-- Name: cms_audit_log_2027_03_pkey; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.cms_audit_log_pkey ATTACH PARTITION public.cms_audit_log_2027_03_pkey;


--
-- Name: cms_audit_log_2027_03_user_id_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_user ATTACH PARTITION public.cms_audit_log_2027_03_user_id_idx;


--
-- Name: cms_audit_log_2027_04_action_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_action ATTACH PARTITION public.cms_audit_log_2027_04_action_idx;


--
-- Name: cms_audit_log_2027_04_created_at_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_created ATTACH PARTITION public.cms_audit_log_2027_04_created_at_idx;


--
-- Name: cms_audit_log_2027_04_created_at_idx1; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_failed ATTACH PARTITION public.cms_audit_log_2027_04_created_at_idx1;


--
-- Name: cms_audit_log_2027_04_entity_type_entity_id_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_entity ATTACH PARTITION public.cms_audit_log_2027_04_entity_type_entity_id_idx;


--
-- Name: cms_audit_log_2027_04_pkey; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.cms_audit_log_pkey ATTACH PARTITION public.cms_audit_log_2027_04_pkey;


--
-- Name: cms_audit_log_2027_04_user_id_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_user ATTACH PARTITION public.cms_audit_log_2027_04_user_id_idx;


--
-- Name: cms_audit_log_2027_05_action_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_action ATTACH PARTITION public.cms_audit_log_2027_05_action_idx;


--
-- Name: cms_audit_log_2027_05_created_at_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_created ATTACH PARTITION public.cms_audit_log_2027_05_created_at_idx;


--
-- Name: cms_audit_log_2027_05_created_at_idx1; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_failed ATTACH PARTITION public.cms_audit_log_2027_05_created_at_idx1;


--
-- Name: cms_audit_log_2027_05_entity_type_entity_id_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_entity ATTACH PARTITION public.cms_audit_log_2027_05_entity_type_entity_id_idx;


--
-- Name: cms_audit_log_2027_05_pkey; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.cms_audit_log_pkey ATTACH PARTITION public.cms_audit_log_2027_05_pkey;


--
-- Name: cms_audit_log_2027_05_user_id_idx; Type: INDEX ATTACH; Schema: public; Owner: -
--

ALTER INDEX public.idx_cms_audit_user ATTACH PARTITION public.cms_audit_log_2027_05_user_id_idx;


--
-- Name: cms_content cms_content_search_trigger; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER cms_content_search_trigger BEFORE INSERT OR UPDATE OF title, excerpt, content, meta_description ON public.cms_content FOR EACH ROW EXECUTE FUNCTION public.cms_update_search_vector();


--
-- Name: cms_content cms_content_structured_data_trigger; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER cms_content_structured_data_trigger BEFORE UPDATE OF status ON public.cms_content FOR EACH ROW WHEN (((new.status = 'published'::public.cms_content_status) AND (old.status <> 'published'::public.cms_content_status))) EXECUTE FUNCTION public.cms_auto_generate_structured_data();


--
-- Name: cms_content_tags cms_content_tags_usage_trigger; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER cms_content_tags_usage_trigger AFTER INSERT OR DELETE ON public.cms_content_tags FOR EACH ROW EXECUTE FUNCTION public.cms_update_tag_usage();


--
-- Name: cms_content cms_content_webhook_trigger; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER cms_content_webhook_trigger AFTER INSERT OR UPDATE ON public.cms_content FOR EACH ROW EXECUTE FUNCTION public.cms_content_webhook_trigger();


--
-- Name: cms_datasource_entries cms_datasource_entries_updated_at_trigger; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER cms_datasource_entries_updated_at_trigger BEFORE UPDATE ON public.cms_datasource_entries FOR EACH ROW EXECUTE FUNCTION public.cms_datasource_entries_updated_at();


--
-- Name: cms_datasource_entries cms_datasource_entry_count_trigger; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER cms_datasource_entry_count_trigger AFTER INSERT OR DELETE ON public.cms_datasource_entries FOR EACH ROW EXECUTE FUNCTION public.cms_datasource_entry_count();


--
-- Name: cms_datasources cms_datasources_updated_at_trigger; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER cms_datasources_updated_at_trigger BEFORE UPDATE ON public.cms_datasources FOR EACH ROW EXECUTE FUNCTION public.cms_datasources_updated_at();


--
-- Name: cms_asset_folders cms_folder_path_trigger; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER cms_folder_path_trigger BEFORE INSERT OR UPDATE ON public.cms_asset_folders FOR EACH ROW EXECUTE FUNCTION public.cms_update_folder_path();


--
-- Name: cms_presets cms_presets_updated_at_trigger; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER cms_presets_updated_at_trigger BEFORE UPDATE ON public.cms_presets FOR EACH ROW EXECUTE FUNCTION public.cms_presets_updated_at();


--
-- Name: cms_asset_folders set_updated_at; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_updated_at BEFORE UPDATE ON public.cms_asset_folders FOR EACH ROW EXECUTE FUNCTION public.cms_set_updated_at();


--
-- Name: cms_assets set_updated_at; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_updated_at BEFORE UPDATE ON public.cms_assets FOR EACH ROW EXECUTE FUNCTION public.cms_set_updated_at();


--
-- Name: cms_comments set_updated_at; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_updated_at BEFORE UPDATE ON public.cms_comments FOR EACH ROW EXECUTE FUNCTION public.cms_set_updated_at();


--
-- Name: cms_content set_updated_at; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_updated_at BEFORE UPDATE ON public.cms_content FOR EACH ROW EXECUTE FUNCTION public.cms_set_updated_at();


--
-- Name: cms_navigation_menus set_updated_at; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_updated_at BEFORE UPDATE ON public.cms_navigation_menus FOR EACH ROW EXECUTE FUNCTION public.cms_set_updated_at();


--
-- Name: cms_redirects set_updated_at; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_updated_at BEFORE UPDATE ON public.cms_redirects FOR EACH ROW EXECUTE FUNCTION public.cms_set_updated_at();


--
-- Name: cms_scheduled_jobs set_updated_at; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_updated_at BEFORE UPDATE ON public.cms_scheduled_jobs FOR EACH ROW EXECUTE FUNCTION public.cms_set_updated_at();


--
-- Name: cms_tags set_updated_at; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_updated_at BEFORE UPDATE ON public.cms_tags FOR EACH ROW EXECUTE FUNCTION public.cms_set_updated_at();


--
-- Name: cms_users set_updated_at; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_updated_at BEFORE UPDATE ON public.cms_users FOR EACH ROW EXECUTE FUNCTION public.cms_set_updated_at();


--
-- Name: cms_webhooks set_updated_at; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_updated_at BEFORE UPDATE ON public.cms_webhooks FOR EACH ROW EXECUTE FUNCTION public.cms_set_updated_at();


--
-- Name: stock_lists tr_stock_lists_updated; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER tr_stock_lists_updated BEFORE UPDATE ON public.stock_lists FOR EACH ROW EXECUTE FUNCTION public.update_stock_lists_timestamp();


--
-- Name: cms_comments trg_comment_notifications; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER trg_comment_notifications AFTER INSERT ON public.cms_comments FOR EACH ROW EXECUTE FUNCTION public.cms_notify_comment_mention();


--
-- Name: popup_events trg_popup_conversion_rate; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER trg_popup_conversion_rate AFTER INSERT ON public.popup_events FOR EACH ROW EXECUTE FUNCTION public.update_popup_conversion_rate();


--
-- Name: room_trades trg_recalculate_stats; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER trg_recalculate_stats AFTER INSERT OR DELETE OR UPDATE ON public.room_trades FOR EACH ROW EXECUTE FUNCTION public.trigger_recalculate_room_stats();


--
-- Name: cms_reusable_block_usage trg_reusable_block_usage_count; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER trg_reusable_block_usage_count AFTER INSERT OR DELETE ON public.cms_reusable_block_usage FOR EACH ROW EXECUTE FUNCTION public.update_reusable_block_usage_count();


--
-- Name: cms_reusable_blocks trg_reusable_blocks_updated; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER trg_reusable_blocks_updated BEFORE UPDATE ON public.cms_reusable_blocks FOR EACH ROW EXECUTE FUNCTION public.cms_update_timestamp();


--
-- Name: cms_user_editor_preferences trg_user_editor_prefs_updated; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER trg_user_editor_prefs_updated BEFORE UPDATE ON public.cms_user_editor_preferences FOR EACH ROW EXECUTE FUNCTION public.cms_update_timestamp();


--
-- Name: cms_content trigger_cms_create_workflow_status; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER trigger_cms_create_workflow_status AFTER INSERT ON public.cms_content FOR EACH ROW EXECUTE FUNCTION public.cms_create_workflow_status();


--
-- Name: cms_workflow_status trigger_cms_log_workflow_transition; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER trigger_cms_log_workflow_transition AFTER UPDATE ON public.cms_workflow_status FOR EACH ROW EXECUTE FUNCTION public.cms_log_workflow_transition();


--
-- Name: cms_release_items trigger_cms_release_item_count; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER trigger_cms_release_item_count AFTER INSERT OR DELETE ON public.cms_release_items FOR EACH ROW EXECUTE FUNCTION public.cms_update_release_item_count();


--
-- Name: cms_release_items trigger_cms_release_items_updated_at; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER trigger_cms_release_items_updated_at BEFORE UPDATE ON public.cms_release_items FOR EACH ROW EXECUTE FUNCTION public.cms_schedules_update_timestamp();


--
-- Name: cms_release_items trigger_cms_release_progress; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER trigger_cms_release_progress AFTER UPDATE ON public.cms_release_items FOR EACH ROW EXECUTE FUNCTION public.cms_update_release_progress();


--
-- Name: cms_releases trigger_cms_releases_updated_at; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER trigger_cms_releases_updated_at BEFORE UPDATE ON public.cms_releases FOR EACH ROW EXECUTE FUNCTION public.cms_schedules_update_timestamp();


--
-- Name: cms_schedules trigger_cms_schedules_updated_at; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER trigger_cms_schedules_updated_at BEFORE UPDATE ON public.cms_schedules FOR EACH ROW EXECUTE FUNCTION public.cms_schedules_update_timestamp();


--
-- Name: video_transcripts trigger_transcript_updated; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER trigger_transcript_updated BEFORE UPDATE ON public.video_transcripts FOR EACH ROW EXECUTE FUNCTION public.update_updated_at_column();


--
-- Name: course_category_mappings trigger_update_category_count; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER trigger_update_category_count AFTER INSERT OR DELETE ON public.course_category_mappings FOR EACH ROW EXECUTE FUNCTION public.update_category_course_count();


--
-- Name: trading_room_schedules trigger_update_schedule_timestamp; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER trigger_update_schedule_timestamp BEFORE UPDATE ON public.trading_room_schedules FOR EACH ROW EXECUTE FUNCTION public.update_schedule_timestamp();


--
-- Name: course_tag_mappings trigger_update_tag_count; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER trigger_update_tag_count AFTER INSERT OR DELETE ON public.course_tag_mappings FOR EACH ROW EXECUTE FUNCTION public.update_tag_course_count();


--
-- Name: video_watch_progress trigger_watch_progress_updated; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER trigger_watch_progress_updated BEFORE UPDATE ON public.video_watch_progress FOR EACH ROW EXECUTE FUNCTION public.update_updated_at_column();


--
-- Name: course_downloads update_course_downloads_updated_at; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER update_course_downloads_updated_at BEFORE UPDATE ON public.course_downloads FOR EACH ROW EXECUTE FUNCTION public.update_updated_at_column();


--
-- Name: course_modules_v2 update_course_modules_v2_updated_at; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER update_course_modules_v2_updated_at BEFORE UPDATE ON public.course_modules_v2 FOR EACH ROW EXECUTE FUNCTION public.update_updated_at_column();


--
-- Name: courses update_courses_updated_at; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER update_courses_updated_at BEFORE UPDATE ON public.courses FOR EACH ROW EXECUTE FUNCTION public.update_updated_at_column();


--
-- Name: lessons update_lessons_updated_at; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER update_lessons_updated_at BEFORE UPDATE ON public.lessons FOR EACH ROW EXECUTE FUNCTION public.update_updated_at_column();


--
-- Name: analytics_events analytics_events_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.analytics_events
    ADD CONSTRAINT analytics_events_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id);


--
-- Name: audit_logs audit_logs_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.audit_logs
    ADD CONSTRAINT audit_logs_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE SET NULL;


--
-- Name: categories categories_parent_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.categories
    ADD CONSTRAINT categories_parent_id_fkey FOREIGN KEY (parent_id) REFERENCES public.categories(id);


--
-- Name: cms_ai_assist_history cms_ai_assist_history_content_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_ai_assist_history
    ADD CONSTRAINT cms_ai_assist_history_content_id_fkey FOREIGN KEY (content_id) REFERENCES public.cms_content(id) ON DELETE SET NULL;


--
-- Name: cms_ai_assist_history cms_ai_assist_history_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_ai_assist_history
    ADD CONSTRAINT cms_ai_assist_history_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.cms_users(id) ON DELETE CASCADE;


--
-- Name: cms_asset_folders cms_asset_folders_created_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_asset_folders
    ADD CONSTRAINT cms_asset_folders_created_by_fkey FOREIGN KEY (created_by) REFERENCES public.cms_users(id) ON DELETE SET NULL;


--
-- Name: cms_asset_folders cms_asset_folders_parent_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_asset_folders
    ADD CONSTRAINT cms_asset_folders_parent_id_fkey FOREIGN KEY (parent_id) REFERENCES public.cms_asset_folders(id) ON DELETE CASCADE;


--
-- Name: cms_asset_folders cms_asset_folders_updated_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_asset_folders
    ADD CONSTRAINT cms_asset_folders_updated_by_fkey FOREIGN KEY (updated_by) REFERENCES public.cms_users(id) ON DELETE SET NULL;


--
-- Name: cms_assets cms_assets_created_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_assets
    ADD CONSTRAINT cms_assets_created_by_fkey FOREIGN KEY (created_by) REFERENCES public.cms_users(id) ON DELETE SET NULL;


--
-- Name: cms_assets cms_assets_folder_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_assets
    ADD CONSTRAINT cms_assets_folder_id_fkey FOREIGN KEY (folder_id) REFERENCES public.cms_asset_folders(id) ON DELETE SET NULL;


--
-- Name: cms_assets cms_assets_updated_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_assets
    ADD CONSTRAINT cms_assets_updated_by_fkey FOREIGN KEY (updated_by) REFERENCES public.cms_users(id) ON DELETE SET NULL;


--
-- Name: cms_audit_log cms_audit_log_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE public.cms_audit_log
    ADD CONSTRAINT cms_audit_log_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.cms_users(id) ON DELETE SET NULL;


--
-- Name: cms_audit_logs cms_audit_logs_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_audit_logs
    ADD CONSTRAINT cms_audit_logs_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id);


--
-- Name: cms_comment_notifications cms_comment_notifications_comment_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_comment_notifications
    ADD CONSTRAINT cms_comment_notifications_comment_id_fkey FOREIGN KEY (comment_id) REFERENCES public.cms_comments(id) ON DELETE CASCADE;


--
-- Name: cms_comment_notifications cms_comment_notifications_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_comment_notifications
    ADD CONSTRAINT cms_comment_notifications_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.cms_users(id) ON DELETE CASCADE;


--
-- Name: cms_comments cms_comments_content_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_comments
    ADD CONSTRAINT cms_comments_content_id_fkey FOREIGN KEY (content_id) REFERENCES public.cms_content(id) ON DELETE CASCADE;


--
-- Name: cms_comments cms_comments_created_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_comments
    ADD CONSTRAINT cms_comments_created_by_fkey FOREIGN KEY (created_by) REFERENCES public.cms_users(id) ON DELETE SET NULL;


--
-- Name: cms_comments cms_comments_parent_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_comments
    ADD CONSTRAINT cms_comments_parent_id_fkey FOREIGN KEY (parent_id) REFERENCES public.cms_comments(id) ON DELETE CASCADE;


--
-- Name: cms_comments cms_comments_resolved_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_comments
    ADD CONSTRAINT cms_comments_resolved_by_fkey FOREIGN KEY (resolved_by) REFERENCES public.cms_users(id) ON DELETE SET NULL;


--
-- Name: cms_content cms_content_author_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_content
    ADD CONSTRAINT cms_content_author_id_fkey FOREIGN KEY (author_id) REFERENCES public.cms_users(id) ON DELETE SET NULL;


--
-- Name: cms_content cms_content_created_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_content
    ADD CONSTRAINT cms_content_created_by_fkey FOREIGN KEY (created_by) REFERENCES public.cms_users(id) ON DELETE SET NULL;


--
-- Name: cms_content cms_content_featured_image_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_content
    ADD CONSTRAINT cms_content_featured_image_id_fkey FOREIGN KEY (featured_image_id) REFERENCES public.cms_assets(id) ON DELETE SET NULL;


--
-- Name: cms_content cms_content_og_image_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_content
    ADD CONSTRAINT cms_content_og_image_id_fkey FOREIGN KEY (og_image_id) REFERENCES public.cms_assets(id) ON DELETE SET NULL;


--
-- Name: cms_content cms_content_parent_content_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_content
    ADD CONSTRAINT cms_content_parent_content_id_fkey FOREIGN KEY (parent_content_id) REFERENCES public.cms_content(id) ON DELETE SET NULL;


--
-- Name: cms_content_relations cms_content_relations_created_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_content_relations
    ADD CONSTRAINT cms_content_relations_created_by_fkey FOREIGN KEY (created_by) REFERENCES public.cms_users(id) ON DELETE SET NULL;


--
-- Name: cms_content_relations cms_content_relations_source_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_content_relations
    ADD CONSTRAINT cms_content_relations_source_id_fkey FOREIGN KEY (source_id) REFERENCES public.cms_content(id) ON DELETE CASCADE;


--
-- Name: cms_content_relations cms_content_relations_target_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_content_relations
    ADD CONSTRAINT cms_content_relations_target_id_fkey FOREIGN KEY (target_id) REFERENCES public.cms_content(id) ON DELETE CASCADE;


--
-- Name: cms_content_tags cms_content_tags_content_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_content_tags
    ADD CONSTRAINT cms_content_tags_content_id_fkey FOREIGN KEY (content_id) REFERENCES public.cms_content(id) ON DELETE CASCADE;


--
-- Name: cms_content_tags cms_content_tags_created_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_content_tags
    ADD CONSTRAINT cms_content_tags_created_by_fkey FOREIGN KEY (created_by) REFERENCES public.cms_users(id) ON DELETE SET NULL;


--
-- Name: cms_content_tags cms_content_tags_tag_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_content_tags
    ADD CONSTRAINT cms_content_tags_tag_id_fkey FOREIGN KEY (tag_id) REFERENCES public.cms_tags(id) ON DELETE CASCADE;


--
-- Name: cms_content cms_content_updated_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_content
    ADD CONSTRAINT cms_content_updated_by_fkey FOREIGN KEY (updated_by) REFERENCES public.cms_users(id) ON DELETE SET NULL;


--
-- Name: cms_datasource_entries cms_datasource_entries_datasource_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_datasource_entries
    ADD CONSTRAINT cms_datasource_entries_datasource_id_fkey FOREIGN KEY (datasource_id) REFERENCES public.cms_datasources(id) ON DELETE CASCADE;


--
-- Name: cms_datasources cms_datasources_created_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_datasources
    ADD CONSTRAINT cms_datasources_created_by_fkey FOREIGN KEY (created_by) REFERENCES public.cms_users(id) ON DELETE SET NULL;


--
-- Name: cms_datasources cms_datasources_updated_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_datasources
    ADD CONSTRAINT cms_datasources_updated_by_fkey FOREIGN KEY (updated_by) REFERENCES public.cms_users(id) ON DELETE SET NULL;


--
-- Name: cms_navigation_menus cms_navigation_menus_created_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_navigation_menus
    ADD CONSTRAINT cms_navigation_menus_created_by_fkey FOREIGN KEY (created_by) REFERENCES public.cms_users(id) ON DELETE SET NULL;


--
-- Name: cms_navigation_menus cms_navigation_menus_updated_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_navigation_menus
    ADD CONSTRAINT cms_navigation_menus_updated_by_fkey FOREIGN KEY (updated_by) REFERENCES public.cms_users(id) ON DELETE SET NULL;


--
-- Name: cms_offline_sync_queue cms_offline_sync_queue_content_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_offline_sync_queue
    ADD CONSTRAINT cms_offline_sync_queue_content_id_fkey FOREIGN KEY (content_id) REFERENCES public.cms_content(id) ON DELETE CASCADE;


--
-- Name: cms_offline_sync_queue cms_offline_sync_queue_resolved_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_offline_sync_queue
    ADD CONSTRAINT cms_offline_sync_queue_resolved_by_fkey FOREIGN KEY (resolved_by) REFERENCES public.cms_users(id) ON DELETE SET NULL;


--
-- Name: cms_offline_sync_queue cms_offline_sync_queue_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_offline_sync_queue
    ADD CONSTRAINT cms_offline_sync_queue_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.cms_users(id) ON DELETE CASCADE;


--
-- Name: cms_presets cms_presets_created_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_presets
    ADD CONSTRAINT cms_presets_created_by_fkey FOREIGN KEY (created_by) REFERENCES public.cms_users(id) ON DELETE SET NULL;


--
-- Name: cms_presets cms_presets_updated_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_presets
    ADD CONSTRAINT cms_presets_updated_by_fkey FOREIGN KEY (updated_by) REFERENCES public.cms_users(id) ON DELETE SET NULL;


--
-- Name: cms_preview_tokens cms_preview_tokens_content_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_preview_tokens
    ADD CONSTRAINT cms_preview_tokens_content_id_fkey FOREIGN KEY (content_id) REFERENCES public.cms_content(id) ON DELETE CASCADE;


--
-- Name: cms_preview_tokens cms_preview_tokens_created_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_preview_tokens
    ADD CONSTRAINT cms_preview_tokens_created_by_fkey FOREIGN KEY (created_by) REFERENCES public.users(id);


--
-- Name: cms_redirects cms_redirects_created_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_redirects
    ADD CONSTRAINT cms_redirects_created_by_fkey FOREIGN KEY (created_by) REFERENCES public.cms_users(id) ON DELETE SET NULL;


--
-- Name: cms_release_items cms_release_items_content_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_release_items
    ADD CONSTRAINT cms_release_items_content_id_fkey FOREIGN KEY (content_id) REFERENCES public.cms_content(id) ON DELETE CASCADE;


--
-- Name: cms_release_items cms_release_items_release_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_release_items
    ADD CONSTRAINT cms_release_items_release_id_fkey FOREIGN KEY (release_id) REFERENCES public.cms_releases(id) ON DELETE CASCADE;


--
-- Name: cms_releases cms_releases_approved_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_releases
    ADD CONSTRAINT cms_releases_approved_by_fkey FOREIGN KEY (approved_by) REFERENCES public.users(id);


--
-- Name: cms_releases cms_releases_cancelled_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_releases
    ADD CONSTRAINT cms_releases_cancelled_by_fkey FOREIGN KEY (cancelled_by) REFERENCES public.users(id);


--
-- Name: cms_releases cms_releases_created_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_releases
    ADD CONSTRAINT cms_releases_created_by_fkey FOREIGN KEY (created_by) REFERENCES public.users(id);


--
-- Name: cms_reusable_block_usage cms_reusable_block_usage_content_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_reusable_block_usage
    ADD CONSTRAINT cms_reusable_block_usage_content_id_fkey FOREIGN KEY (content_id) REFERENCES public.cms_content(id) ON DELETE CASCADE;


--
-- Name: cms_reusable_block_usage cms_reusable_block_usage_detached_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_reusable_block_usage
    ADD CONSTRAINT cms_reusable_block_usage_detached_by_fkey FOREIGN KEY (detached_by) REFERENCES public.cms_users(id) ON DELETE SET NULL;


--
-- Name: cms_reusable_block_usage cms_reusable_block_usage_reusable_block_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_reusable_block_usage
    ADD CONSTRAINT cms_reusable_block_usage_reusable_block_id_fkey FOREIGN KEY (reusable_block_id) REFERENCES public.cms_reusable_blocks(id) ON DELETE CASCADE;


--
-- Name: cms_reusable_blocks cms_reusable_blocks_cms_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_reusable_blocks
    ADD CONSTRAINT cms_reusable_blocks_cms_user_id_fkey FOREIGN KEY (cms_user_id) REFERENCES public.cms_users(id) ON DELETE SET NULL;


--
-- Name: cms_reusable_blocks cms_reusable_blocks_created_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_reusable_blocks
    ADD CONSTRAINT cms_reusable_blocks_created_by_fkey FOREIGN KEY (created_by) REFERENCES public.cms_users(id) ON DELETE SET NULL;


--
-- Name: cms_reusable_blocks cms_reusable_blocks_updated_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_reusable_blocks
    ADD CONSTRAINT cms_reusable_blocks_updated_by_fkey FOREIGN KEY (updated_by) REFERENCES public.cms_users(id) ON DELETE SET NULL;


--
-- Name: cms_revisions cms_revisions_content_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_revisions
    ADD CONSTRAINT cms_revisions_content_id_fkey FOREIGN KEY (content_id) REFERENCES public.cms_content(id) ON DELETE CASCADE;


--
-- Name: cms_revisions cms_revisions_created_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_revisions
    ADD CONSTRAINT cms_revisions_created_by_fkey FOREIGN KEY (created_by) REFERENCES public.cms_users(id) ON DELETE SET NULL;


--
-- Name: cms_schedule_history cms_schedule_history_content_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_schedule_history
    ADD CONSTRAINT cms_schedule_history_content_id_fkey FOREIGN KEY (content_id) REFERENCES public.cms_content(id) ON DELETE SET NULL;


--
-- Name: cms_schedule_history cms_schedule_history_release_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_schedule_history
    ADD CONSTRAINT cms_schedule_history_release_id_fkey FOREIGN KEY (release_id) REFERENCES public.cms_releases(id) ON DELETE SET NULL;


--
-- Name: cms_schedule_history cms_schedule_history_schedule_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_schedule_history
    ADD CONSTRAINT cms_schedule_history_schedule_id_fkey FOREIGN KEY (schedule_id) REFERENCES public.cms_schedules(id) ON DELETE SET NULL;


--
-- Name: cms_schedule_history cms_schedule_history_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_schedule_history
    ADD CONSTRAINT cms_schedule_history_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id);


--
-- Name: cms_scheduled_jobs cms_scheduled_jobs_content_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_scheduled_jobs
    ADD CONSTRAINT cms_scheduled_jobs_content_id_fkey FOREIGN KEY (content_id) REFERENCES public.cms_content(id) ON DELETE CASCADE;


--
-- Name: cms_scheduled_jobs cms_scheduled_jobs_created_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_scheduled_jobs
    ADD CONSTRAINT cms_scheduled_jobs_created_by_fkey FOREIGN KEY (created_by) REFERENCES public.cms_users(id) ON DELETE SET NULL;


--
-- Name: cms_schedules cms_schedules_cancelled_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_schedules
    ADD CONSTRAINT cms_schedules_cancelled_by_fkey FOREIGN KEY (cancelled_by) REFERENCES public.users(id);


--
-- Name: cms_schedules cms_schedules_content_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_schedules
    ADD CONSTRAINT cms_schedules_content_id_fkey FOREIGN KEY (content_id) REFERENCES public.cms_content(id) ON DELETE CASCADE;


--
-- Name: cms_schedules cms_schedules_created_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_schedules
    ADD CONSTRAINT cms_schedules_created_by_fkey FOREIGN KEY (created_by) REFERENCES public.users(id);


--
-- Name: cms_site_settings cms_site_settings_favicon_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_site_settings
    ADD CONSTRAINT cms_site_settings_favicon_id_fkey FOREIGN KEY (favicon_id) REFERENCES public.cms_assets(id) ON DELETE SET NULL;


--
-- Name: cms_site_settings cms_site_settings_logo_dark_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_site_settings
    ADD CONSTRAINT cms_site_settings_logo_dark_id_fkey FOREIGN KEY (logo_dark_id) REFERENCES public.cms_assets(id) ON DELETE SET NULL;


--
-- Name: cms_site_settings cms_site_settings_logo_light_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_site_settings
    ADD CONSTRAINT cms_site_settings_logo_light_id_fkey FOREIGN KEY (logo_light_id) REFERENCES public.cms_assets(id) ON DELETE SET NULL;


--
-- Name: cms_site_settings cms_site_settings_og_default_image_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_site_settings
    ADD CONSTRAINT cms_site_settings_og_default_image_id_fkey FOREIGN KEY (og_default_image_id) REFERENCES public.cms_assets(id) ON DELETE SET NULL;


--
-- Name: cms_site_settings cms_site_settings_updated_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_site_settings
    ADD CONSTRAINT cms_site_settings_updated_by_fkey FOREIGN KEY (updated_by) REFERENCES public.cms_users(id) ON DELETE SET NULL;


--
-- Name: cms_tags cms_tags_created_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_tags
    ADD CONSTRAINT cms_tags_created_by_fkey FOREIGN KEY (created_by) REFERENCES public.cms_users(id) ON DELETE SET NULL;


--
-- Name: cms_tags cms_tags_parent_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_tags
    ADD CONSTRAINT cms_tags_parent_id_fkey FOREIGN KEY (parent_id) REFERENCES public.cms_tags(id) ON DELETE SET NULL;


--
-- Name: cms_user_editor_preferences cms_user_editor_preferences_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_user_editor_preferences
    ADD CONSTRAINT cms_user_editor_preferences_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.cms_users(id) ON DELETE CASCADE;


--
-- Name: cms_users cms_users_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_users
    ADD CONSTRAINT cms_users_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: cms_webhook_deliveries cms_webhook_deliveries_content_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_webhook_deliveries
    ADD CONSTRAINT cms_webhook_deliveries_content_id_fkey FOREIGN KEY (content_id) REFERENCES public.cms_content(id) ON DELETE SET NULL;


--
-- Name: cms_webhook_deliveries cms_webhook_deliveries_webhook_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_webhook_deliveries
    ADD CONSTRAINT cms_webhook_deliveries_webhook_id_fkey FOREIGN KEY (webhook_id) REFERENCES public.cms_webhooks(id) ON DELETE CASCADE;


--
-- Name: cms_webhooks cms_webhooks_created_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_webhooks
    ADD CONSTRAINT cms_webhooks_created_by_fkey FOREIGN KEY (created_by) REFERENCES public.cms_users(id) ON DELETE SET NULL;


--
-- Name: cms_workflow_history cms_workflow_history_content_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_workflow_history
    ADD CONSTRAINT cms_workflow_history_content_id_fkey FOREIGN KEY (content_id) REFERENCES public.cms_content(id) ON DELETE CASCADE;


--
-- Name: cms_workflow_history cms_workflow_history_transitioned_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_workflow_history
    ADD CONSTRAINT cms_workflow_history_transitioned_by_fkey FOREIGN KEY (transitioned_by) REFERENCES public.users(id);


--
-- Name: cms_workflow_history cms_workflow_history_workflow_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_workflow_history
    ADD CONSTRAINT cms_workflow_history_workflow_id_fkey FOREIGN KEY (workflow_id) REFERENCES public.cms_workflow_status(id) ON DELETE CASCADE;


--
-- Name: cms_workflow_log cms_workflow_log_content_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_workflow_log
    ADD CONSTRAINT cms_workflow_log_content_id_fkey FOREIGN KEY (content_id) REFERENCES public.cms_content(id) ON DELETE CASCADE;


--
-- Name: cms_workflow_log cms_workflow_log_transitioned_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_workflow_log
    ADD CONSTRAINT cms_workflow_log_transitioned_by_fkey FOREIGN KEY (transitioned_by) REFERENCES public.cms_users(id) ON DELETE SET NULL;


--
-- Name: cms_workflow_status cms_workflow_status_assigned_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_workflow_status
    ADD CONSTRAINT cms_workflow_status_assigned_by_fkey FOREIGN KEY (assigned_by) REFERENCES public.users(id);


--
-- Name: cms_workflow_status cms_workflow_status_assigned_to_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_workflow_status
    ADD CONSTRAINT cms_workflow_status_assigned_to_fkey FOREIGN KEY (assigned_to) REFERENCES public.users(id);


--
-- Name: cms_workflow_status cms_workflow_status_content_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cms_workflow_status
    ADD CONSTRAINT cms_workflow_status_content_id_fkey FOREIGN KEY (content_id) REFERENCES public.cms_content(id) ON DELETE CASCADE;


--
-- Name: contacts contacts_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.contacts
    ADD CONSTRAINT contacts_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id);


--
-- Name: content_translations content_translations_locale_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.content_translations
    ADD CONSTRAINT content_translations_locale_fkey FOREIGN KEY (locale) REFERENCES public.locales(code) ON DELETE CASCADE;


--
-- Name: content_translations content_translations_reviewed_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.content_translations
    ADD CONSTRAINT content_translations_reviewed_by_fkey FOREIGN KEY (reviewed_by) REFERENCES public.users(id) ON DELETE SET NULL;


--
-- Name: content_translations content_translations_translated_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.content_translations
    ADD CONSTRAINT content_translations_translated_by_fkey FOREIGN KEY (translated_by) REFERENCES public.users(id) ON DELETE SET NULL;


--
-- Name: content_versions content_versions_created_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.content_versions
    ADD CONSTRAINT content_versions_created_by_fkey FOREIGN KEY (created_by) REFERENCES public.users(id) ON DELETE SET NULL;


--
-- Name: content_workflow_status content_workflow_status_assigned_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.content_workflow_status
    ADD CONSTRAINT content_workflow_status_assigned_by_fkey FOREIGN KEY (assigned_by) REFERENCES public.users(id) ON DELETE SET NULL;


--
-- Name: content_workflow_status content_workflow_status_assigned_to_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.content_workflow_status
    ADD CONSTRAINT content_workflow_status_assigned_to_fkey FOREIGN KEY (assigned_to) REFERENCES public.users(id) ON DELETE SET NULL;


--
-- Name: content_workflow_status content_workflow_status_workflow_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.content_workflow_status
    ADD CONSTRAINT content_workflow_status_workflow_id_fkey FOREIGN KEY (workflow_id) REFERENCES public.workflow_definitions(id) ON DELETE SET NULL;


--
-- Name: course_categories course_categories_parent_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_categories
    ADD CONSTRAINT course_categories_parent_id_fkey FOREIGN KEY (parent_id) REFERENCES public.course_categories(id) ON DELETE SET NULL;


--
-- Name: course_category_mappings course_category_mappings_category_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_category_mappings
    ADD CONSTRAINT course_category_mappings_category_id_fkey FOREIGN KEY (category_id) REFERENCES public.course_categories(id) ON DELETE CASCADE;


--
-- Name: course_category_mappings course_category_mappings_course_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_category_mappings
    ADD CONSTRAINT course_category_mappings_course_id_fkey FOREIGN KEY (course_id) REFERENCES public.courses(id) ON DELETE CASCADE;


--
-- Name: course_downloads course_downloads_course_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_downloads
    ADD CONSTRAINT course_downloads_course_id_fkey FOREIGN KEY (course_id) REFERENCES public.courses(id) ON DELETE CASCADE;


--
-- Name: course_downloads course_downloads_lesson_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_downloads
    ADD CONSTRAINT course_downloads_lesson_id_fkey FOREIGN KEY (lesson_id) REFERENCES public.lessons(id) ON DELETE SET NULL;


--
-- Name: course_downloads course_downloads_module_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_downloads
    ADD CONSTRAINT course_downloads_module_id_fkey FOREIGN KEY (module_id) REFERENCES public.course_modules_v2(id) ON DELETE SET NULL;


--
-- Name: course_downloads course_downloads_uploaded_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_downloads
    ADD CONSTRAINT course_downloads_uploaded_by_fkey FOREIGN KEY (uploaded_by) REFERENCES public.users(id) ON DELETE SET NULL;


--
-- Name: course_lessons course_lessons_course_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_lessons
    ADD CONSTRAINT course_lessons_course_id_fkey FOREIGN KEY (course_id) REFERENCES public.courses_enhanced(id) ON DELETE CASCADE;


--
-- Name: course_lessons course_lessons_section_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_lessons
    ADD CONSTRAINT course_lessons_section_id_fkey FOREIGN KEY (section_id) REFERENCES public.course_sections(id) ON DELETE CASCADE;


--
-- Name: course_live_sessions course_live_sessions_course_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_live_sessions
    ADD CONSTRAINT course_live_sessions_course_id_fkey FOREIGN KEY (course_id) REFERENCES public.courses_enhanced(id) ON DELETE CASCADE;


--
-- Name: course_live_sessions course_live_sessions_section_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_live_sessions
    ADD CONSTRAINT course_live_sessions_section_id_fkey FOREIGN KEY (section_id) REFERENCES public.course_sections(id) ON DELETE CASCADE;


--
-- Name: course_modules course_modules_course_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_modules
    ADD CONSTRAINT course_modules_course_id_fkey FOREIGN KEY (course_id) REFERENCES public.courses_enhanced(id) ON DELETE CASCADE;


--
-- Name: course_modules_v2 course_modules_v2_course_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_modules_v2
    ADD CONSTRAINT course_modules_v2_course_id_fkey FOREIGN KEY (course_id) REFERENCES public.courses(id) ON DELETE CASCADE;


--
-- Name: course_quizzes course_quizzes_course_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_quizzes
    ADD CONSTRAINT course_quizzes_course_id_fkey FOREIGN KEY (course_id) REFERENCES public.courses(id) ON DELETE CASCADE;


--
-- Name: course_quizzes course_quizzes_lesson_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_quizzes
    ADD CONSTRAINT course_quizzes_lesson_id_fkey FOREIGN KEY (lesson_id) REFERENCES public.lessons(id) ON DELETE SET NULL;


--
-- Name: course_quizzes course_quizzes_module_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_quizzes
    ADD CONSTRAINT course_quizzes_module_id_fkey FOREIGN KEY (module_id) REFERENCES public.course_modules_v2(id) ON DELETE SET NULL;


--
-- Name: course_resources course_resources_course_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_resources
    ADD CONSTRAINT course_resources_course_id_fkey FOREIGN KEY (course_id) REFERENCES public.courses_enhanced(id) ON DELETE CASCADE;


--
-- Name: course_resources course_resources_lesson_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_resources
    ADD CONSTRAINT course_resources_lesson_id_fkey FOREIGN KEY (lesson_id) REFERENCES public.course_lessons(id) ON DELETE CASCADE;


--
-- Name: course_resources course_resources_section_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_resources
    ADD CONSTRAINT course_resources_section_id_fkey FOREIGN KEY (section_id) REFERENCES public.course_sections(id) ON DELETE CASCADE;


--
-- Name: course_reviews course_reviews_course_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_reviews
    ADD CONSTRAINT course_reviews_course_id_fkey FOREIGN KEY (course_id) REFERENCES public.courses(id) ON DELETE CASCADE;


--
-- Name: course_reviews course_reviews_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_reviews
    ADD CONSTRAINT course_reviews_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: course_sections course_sections_course_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_sections
    ADD CONSTRAINT course_sections_course_id_fkey FOREIGN KEY (course_id) REFERENCES public.courses_enhanced(id) ON DELETE CASCADE;


--
-- Name: course_sections course_sections_unlock_after_section_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_sections
    ADD CONSTRAINT course_sections_unlock_after_section_id_fkey FOREIGN KEY (unlock_after_section_id) REFERENCES public.course_sections(id) ON DELETE SET NULL;


--
-- Name: course_tag_mappings course_tag_mappings_course_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_tag_mappings
    ADD CONSTRAINT course_tag_mappings_course_id_fkey FOREIGN KEY (course_id) REFERENCES public.courses(id) ON DELETE CASCADE;


--
-- Name: course_tag_mappings course_tag_mappings_tag_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.course_tag_mappings
    ADD CONSTRAINT course_tag_mappings_tag_id_fkey FOREIGN KEY (tag_id) REFERENCES public.course_tags(id) ON DELETE CASCADE;


--
-- Name: courses_enhanced courses_enhanced_instructor_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.courses_enhanced
    ADD CONSTRAINT courses_enhanced_instructor_id_fkey FOREIGN KEY (instructor_id) REFERENCES public.room_traders(id) ON DELETE SET NULL;


--
-- Name: courses courses_instructor_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.courses
    ADD CONSTRAINT courses_instructor_id_fkey FOREIGN KEY (instructor_id) REFERENCES public.users(id);


--
-- Name: crm_abandoned_carts crm_abandoned_carts_contact_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_abandoned_carts
    ADD CONSTRAINT crm_abandoned_carts_contact_id_fkey FOREIGN KEY (contact_id) REFERENCES public.contacts(id) ON DELETE SET NULL;


--
-- Name: crm_campaigns crm_campaigns_template_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_campaigns
    ADD CONSTRAINT crm_campaigns_template_id_fkey FOREIGN KEY (template_id) REFERENCES public.crm_templates(id) ON DELETE SET NULL;


--
-- Name: crm_deal_activities crm_deal_activities_assigned_to_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_deal_activities
    ADD CONSTRAINT crm_deal_activities_assigned_to_fkey FOREIGN KEY (assigned_to) REFERENCES public.users(id) ON DELETE SET NULL;


--
-- Name: crm_deal_activities crm_deal_activities_created_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_deal_activities
    ADD CONSTRAINT crm_deal_activities_created_by_fkey FOREIGN KEY (created_by) REFERENCES public.users(id) ON DELETE SET NULL;


--
-- Name: crm_deal_activities crm_deal_activities_deal_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_deal_activities
    ADD CONSTRAINT crm_deal_activities_deal_id_fkey FOREIGN KEY (deal_id) REFERENCES public.crm_deals(id) ON DELETE CASCADE;


--
-- Name: crm_deal_stage_history crm_deal_stage_history_changed_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_deal_stage_history
    ADD CONSTRAINT crm_deal_stage_history_changed_by_fkey FOREIGN KEY (changed_by) REFERENCES public.users(id) ON DELETE SET NULL;


--
-- Name: crm_deal_stage_history crm_deal_stage_history_deal_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_deal_stage_history
    ADD CONSTRAINT crm_deal_stage_history_deal_id_fkey FOREIGN KEY (deal_id) REFERENCES public.crm_deals(id) ON DELETE CASCADE;


--
-- Name: crm_deal_stage_history crm_deal_stage_history_from_stage_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_deal_stage_history
    ADD CONSTRAINT crm_deal_stage_history_from_stage_id_fkey FOREIGN KEY (from_stage_id) REFERENCES public.crm_pipeline_stages(id) ON DELETE SET NULL;


--
-- Name: crm_deal_stage_history crm_deal_stage_history_to_stage_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_deal_stage_history
    ADD CONSTRAINT crm_deal_stage_history_to_stage_id_fkey FOREIGN KEY (to_stage_id) REFERENCES public.crm_pipeline_stages(id) ON DELETE CASCADE;


--
-- Name: crm_deals crm_deals_company_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_deals
    ADD CONSTRAINT crm_deals_company_id_fkey FOREIGN KEY (company_id) REFERENCES public.crm_companies(id) ON DELETE SET NULL;


--
-- Name: crm_deals crm_deals_contact_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_deals
    ADD CONSTRAINT crm_deals_contact_id_fkey FOREIGN KEY (contact_id) REFERENCES public.contacts(id) ON DELETE SET NULL;


--
-- Name: crm_deals crm_deals_owner_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_deals
    ADD CONSTRAINT crm_deals_owner_id_fkey FOREIGN KEY (owner_id) REFERENCES public.users(id) ON DELETE SET NULL;


--
-- Name: crm_deals crm_deals_pipeline_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_deals
    ADD CONSTRAINT crm_deals_pipeline_id_fkey FOREIGN KEY (pipeline_id) REFERENCES public.crm_pipelines(id) ON DELETE CASCADE;


--
-- Name: crm_deals crm_deals_stage_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_deals
    ADD CONSTRAINT crm_deals_stage_id_fkey FOREIGN KEY (stage_id) REFERENCES public.crm_pipeline_stages(id) ON DELETE RESTRICT;


--
-- Name: crm_pipeline_stages crm_pipeline_stages_pipeline_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_pipeline_stages
    ADD CONSTRAINT crm_pipeline_stages_pipeline_id_fkey FOREIGN KEY (pipeline_id) REFERENCES public.crm_pipelines(id) ON DELETE CASCADE;


--
-- Name: crm_recurring_campaigns crm_recurring_campaigns_campaign_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.crm_recurring_campaigns
    ADD CONSTRAINT crm_recurring_campaigns_campaign_id_fkey FOREIGN KEY (campaign_id) REFERENCES public.crm_campaigns(id) ON DELETE CASCADE;


--
-- Name: departments departments_parent_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.departments
    ADD CONSTRAINT departments_parent_id_fkey FOREIGN KEY (parent_id) REFERENCES public.departments(id) ON DELETE SET NULL;


--
-- Name: email_campaigns email_campaigns_template_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.email_campaigns
    ADD CONSTRAINT email_campaigns_template_id_fkey FOREIGN KEY (template_id) REFERENCES public.email_templates(id);


--
-- Name: email_logs email_logs_campaign_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.email_logs
    ADD CONSTRAINT email_logs_campaign_id_fkey FOREIGN KEY (campaign_id) REFERENCES public.email_campaigns(id);


--
-- Name: email_logs email_logs_subscriber_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.email_logs
    ADD CONSTRAINT email_logs_subscriber_id_fkey FOREIGN KEY (subscriber_id) REFERENCES public.newsletter_subscribers(id);


--
-- Name: email_logs email_logs_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.email_logs
    ADD CONSTRAINT email_logs_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id);


--
-- Name: popups fk_popup_ab_test; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.popups
    ADD CONSTRAINT fk_popup_ab_test FOREIGN KEY (ab_test_id) REFERENCES public.popup_ab_tests(id) ON DELETE SET NULL;


--
-- Name: form_submissions form_submissions_form_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.form_submissions
    ADD CONSTRAINT form_submissions_form_id_fkey FOREIGN KEY (form_id) REFERENCES public.forms(id) ON DELETE CASCADE;


--
-- Name: form_submissions form_submissions_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.form_submissions
    ADD CONSTRAINT form_submissions_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE SET NULL;


--
-- Name: indicator_documentation indicator_documentation_indicator_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.indicator_documentation
    ADD CONSTRAINT indicator_documentation_indicator_id_fkey FOREIGN KEY (indicator_id) REFERENCES public.indicators_enhanced(id) ON DELETE CASCADE;


--
-- Name: indicator_download_log indicator_download_log_file_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.indicator_download_log
    ADD CONSTRAINT indicator_download_log_file_id_fkey FOREIGN KEY (file_id) REFERENCES public.indicator_platform_files(id) ON DELETE SET NULL;


--
-- Name: indicator_download_log indicator_download_log_indicator_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.indicator_download_log
    ADD CONSTRAINT indicator_download_log_indicator_id_fkey FOREIGN KEY (indicator_id) REFERENCES public.indicators_enhanced(id) ON DELETE CASCADE;


--
-- Name: indicator_download_log indicator_download_log_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.indicator_download_log
    ADD CONSTRAINT indicator_download_log_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE SET NULL;


--
-- Name: indicator_downloads indicator_downloads_indicator_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.indicator_downloads
    ADD CONSTRAINT indicator_downloads_indicator_id_fkey FOREIGN KEY (indicator_id) REFERENCES public.indicators(id) ON DELETE CASCADE;


--
-- Name: indicator_downloads indicator_downloads_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.indicator_downloads
    ADD CONSTRAINT indicator_downloads_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE SET NULL;


--
-- Name: indicator_files indicator_files_indicator_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.indicator_files
    ADD CONSTRAINT indicator_files_indicator_id_fkey FOREIGN KEY (indicator_id) REFERENCES public.indicators(id) ON DELETE CASCADE;


--
-- Name: indicator_platform_files indicator_platform_files_indicator_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.indicator_platform_files
    ADD CONSTRAINT indicator_platform_files_indicator_id_fkey FOREIGN KEY (indicator_id) REFERENCES public.indicators_enhanced(id) ON DELETE CASCADE;


--
-- Name: indicator_tradingview_access indicator_tradingview_access_indicator_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.indicator_tradingview_access
    ADD CONSTRAINT indicator_tradingview_access_indicator_id_fkey FOREIGN KEY (indicator_id) REFERENCES public.indicators_enhanced(id) ON DELETE CASCADE;


--
-- Name: indicator_tradingview_access indicator_tradingview_access_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.indicator_tradingview_access
    ADD CONSTRAINT indicator_tradingview_access_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: indicator_videos indicator_videos_indicator_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.indicator_videos
    ADD CONSTRAINT indicator_videos_indicator_id_fkey FOREIGN KEY (indicator_id) REFERENCES public.indicators_enhanced(id) ON DELETE CASCADE;


--
-- Name: indicators_enhanced indicators_enhanced_creator_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.indicators_enhanced
    ADD CONSTRAINT indicators_enhanced_creator_id_fkey FOREIGN KEY (creator_id) REFERENCES public.room_traders(id) ON DELETE SET NULL;


--
-- Name: integration_webhooks integration_webhooks_connection_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.integration_webhooks
    ADD CONSTRAINT integration_webhooks_connection_id_fkey FOREIGN KEY (connection_id) REFERENCES public.service_connections(id) ON DELETE CASCADE;


--
-- Name: invoices invoices_order_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.invoices
    ADD CONSTRAINT invoices_order_id_fkey FOREIGN KEY (order_id) REFERENCES public.orders(id);


--
-- Name: invoices invoices_subscription_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.invoices
    ADD CONSTRAINT invoices_subscription_id_fkey FOREIGN KEY (subscription_id) REFERENCES public.user_memberships(id);


--
-- Name: invoices invoices_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.invoices
    ADD CONSTRAINT invoices_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: lessons lessons_course_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.lessons
    ADD CONSTRAINT lessons_course_id_fkey FOREIGN KEY (course_id) REFERENCES public.courses(id) ON DELETE CASCADE;


--
-- Name: lessons lessons_module_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.lessons
    ADD CONSTRAINT lessons_module_id_fkey FOREIGN KEY (module_id) REFERENCES public.course_modules_v2(id) ON DELETE SET NULL;


--
-- Name: locales locales_fallback_locale_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.locales
    ADD CONSTRAINT locales_fallback_locale_fkey FOREIGN KEY (fallback_locale) REFERENCES public.locales(code) ON DELETE SET NULL;


--
-- Name: member_audit_logs member_audit_logs_actor_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.member_audit_logs
    ADD CONSTRAINT member_audit_logs_actor_id_fkey FOREIGN KEY (actor_id) REFERENCES public.users(id) ON DELETE SET NULL;


--
-- Name: member_audit_logs member_audit_logs_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.member_audit_logs
    ADD CONSTRAINT member_audit_logs_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE SET NULL;


--
-- Name: member_emails member_emails_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.member_emails
    ADD CONSTRAINT member_emails_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: member_notes member_notes_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.member_notes
    ADD CONSTRAINT member_notes_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: membership_features membership_features_plan_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.membership_features
    ADD CONSTRAINT membership_features_plan_id_fkey FOREIGN KEY (plan_id) REFERENCES public.membership_plans(id) ON DELETE CASCADE;


--
-- Name: membership_plan_price_history membership_plan_price_history_changed_by_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.membership_plan_price_history
    ADD CONSTRAINT membership_plan_price_history_changed_by_user_id_fkey FOREIGN KEY (changed_by_user_id) REFERENCES public.users(id) ON DELETE SET NULL;


--
-- Name: membership_plan_price_history membership_plan_price_history_plan_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.membership_plan_price_history
    ADD CONSTRAINT membership_plan_price_history_plan_id_fkey FOREIGN KEY (plan_id) REFERENCES public.membership_plans(id) ON DELETE CASCADE;


--
-- Name: membership_plans membership_plans_room_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.membership_plans
    ADD CONSTRAINT membership_plans_room_id_fkey FOREIGN KEY (room_id) REFERENCES public.trading_rooms(id);


--
-- Name: mfa_attempts mfa_attempts_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.mfa_attempts
    ADD CONSTRAINT mfa_attempts_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: oauth_audit_log oauth_audit_log_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.oauth_audit_log
    ADD CONSTRAINT oauth_audit_log_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE SET NULL;


--
-- Name: order_items order_items_order_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.order_items
    ADD CONSTRAINT order_items_order_id_fkey FOREIGN KEY (order_id) REFERENCES public.orders(id) ON DELETE CASCADE;


--
-- Name: order_items order_items_plan_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.order_items
    ADD CONSTRAINT order_items_plan_id_fkey FOREIGN KEY (plan_id) REFERENCES public.membership_plans(id);


--
-- Name: order_items order_items_product_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.order_items
    ADD CONSTRAINT order_items_product_id_fkey FOREIGN KEY (product_id) REFERENCES public.products(id);


--
-- Name: orders orders_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.orders
    ADD CONSTRAINT orders_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: page_layout_versions page_layout_versions_layout_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.page_layout_versions
    ADD CONSTRAINT page_layout_versions_layout_id_fkey FOREIGN KEY (layout_id) REFERENCES public.page_layouts(id) ON DELETE CASCADE;


--
-- Name: popup_ab_tests popup_ab_tests_base_popup_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.popup_ab_tests
    ADD CONSTRAINT popup_ab_tests_base_popup_id_fkey FOREIGN KEY (base_popup_id) REFERENCES public.popups(id) ON DELETE CASCADE;


--
-- Name: popup_ab_tests popup_ab_tests_winner_popup_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.popup_ab_tests
    ADD CONSTRAINT popup_ab_tests_winner_popup_id_fkey FOREIGN KEY (winner_popup_id) REFERENCES public.popups(id) ON DELETE SET NULL;


--
-- Name: popup_events popup_events_popup_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.popup_events
    ADD CONSTRAINT popup_events_popup_id_fkey FOREIGN KEY (popup_id) REFERENCES public.popups(id) ON DELETE CASCADE;


--
-- Name: popup_events popup_events_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.popup_events
    ADD CONSTRAINT popup_events_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE SET NULL;


--
-- Name: popup_form_submissions popup_form_submissions_form_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.popup_form_submissions
    ADD CONSTRAINT popup_form_submissions_form_id_fkey FOREIGN KEY (form_id) REFERENCES public.forms(id) ON DELETE SET NULL;


--
-- Name: popup_form_submissions popup_form_submissions_popup_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.popup_form_submissions
    ADD CONSTRAINT popup_form_submissions_popup_id_fkey FOREIGN KEY (popup_id) REFERENCES public.popups(id) ON DELETE CASCADE;


--
-- Name: popups popups_created_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.popups
    ADD CONSTRAINT popups_created_by_fkey FOREIGN KEY (created_by) REFERENCES public.users(id) ON DELETE SET NULL;


--
-- Name: popups popups_form_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.popups
    ADD CONSTRAINT popups_form_id_fkey FOREIGN KEY (form_id) REFERENCES public.forms(id) ON DELETE SET NULL;


--
-- Name: post_categories post_categories_category_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.post_categories
    ADD CONSTRAINT post_categories_category_id_fkey FOREIGN KEY (category_id) REFERENCES public.categories(id) ON DELETE CASCADE;


--
-- Name: post_categories post_categories_post_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.post_categories
    ADD CONSTRAINT post_categories_post_id_fkey FOREIGN KEY (post_id) REFERENCES public.posts(id) ON DELETE CASCADE;


--
-- Name: post_tags post_tags_post_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.post_tags
    ADD CONSTRAINT post_tags_post_id_fkey FOREIGN KEY (post_id) REFERENCES public.posts(id) ON DELETE CASCADE;


--
-- Name: post_tags post_tags_tag_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.post_tags
    ADD CONSTRAINT post_tags_tag_id_fkey FOREIGN KEY (tag_id) REFERENCES public.tags(id) ON DELETE CASCADE;


--
-- Name: posts posts_author_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.posts
    ADD CONSTRAINT posts_author_id_fkey FOREIGN KEY (author_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: posts posts_parent_post_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.posts
    ADD CONSTRAINT posts_parent_post_id_fkey FOREIGN KEY (parent_post_id) REFERENCES public.posts(id) ON DELETE SET NULL;


--
-- Name: preview_tokens preview_tokens_created_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.preview_tokens
    ADD CONSTRAINT preview_tokens_created_by_fkey FOREIGN KEY (created_by) REFERENCES public.users(id) ON DELETE SET NULL;


--
-- Name: quiz_answers quiz_answers_question_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.quiz_answers
    ADD CONSTRAINT quiz_answers_question_id_fkey FOREIGN KEY (question_id) REFERENCES public.quiz_questions(id) ON DELETE CASCADE;


--
-- Name: quiz_questions quiz_questions_quiz_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.quiz_questions
    ADD CONSTRAINT quiz_questions_quiz_id_fkey FOREIGN KEY (quiz_id) REFERENCES public.course_quizzes(id) ON DELETE CASCADE;


--
-- Name: resource_access_log resource_access_log_resource_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.resource_access_log
    ADD CONSTRAINT resource_access_log_resource_id_fkey FOREIGN KEY (resource_id) REFERENCES public.room_resources(id) ON DELETE CASCADE;


--
-- Name: resource_access_log resource_access_log_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.resource_access_log
    ADD CONSTRAINT resource_access_log_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: resource_download_logs resource_download_logs_resource_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.resource_download_logs
    ADD CONSTRAINT resource_download_logs_resource_id_fkey FOREIGN KEY (resource_id) REFERENCES public.room_resources(id);


--
-- Name: resource_download_logs resource_download_logs_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.resource_download_logs
    ADD CONSTRAINT resource_download_logs_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id);


--
-- Name: resource_favorites resource_favorites_resource_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.resource_favorites
    ADD CONSTRAINT resource_favorites_resource_id_fkey FOREIGN KEY (resource_id) REFERENCES public.room_resources(id) ON DELETE CASCADE;


--
-- Name: resource_favorites resource_favorites_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.resource_favorites
    ADD CONSTRAINT resource_favorites_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: review_helpful_votes review_helpful_votes_review_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.review_helpful_votes
    ADD CONSTRAINT review_helpful_votes_review_id_fkey FOREIGN KEY (review_id) REFERENCES public.course_reviews(id) ON DELETE CASCADE;


--
-- Name: review_helpful_votes review_helpful_votes_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.review_helpful_votes
    ADD CONSTRAINT review_helpful_votes_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: room_alerts room_alerts_entry_alert_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.room_alerts
    ADD CONSTRAINT room_alerts_entry_alert_id_fkey FOREIGN KEY (entry_alert_id) REFERENCES public.room_alerts(id);


--
-- Name: room_resources room_resources_previous_version_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.room_resources
    ADD CONSTRAINT room_resources_previous_version_id_fkey FOREIGN KEY (previous_version_id) REFERENCES public.room_resources(id);


--
-- Name: room_resources room_resources_trader_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.room_resources
    ADD CONSTRAINT room_resources_trader_id_fkey FOREIGN KEY (trader_id) REFERENCES public.traders(id);


--
-- Name: room_traders room_traders_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.room_traders
    ADD CONSTRAINT room_traders_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE SET NULL;


--
-- Name: room_trades room_trades_entry_alert_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.room_trades
    ADD CONSTRAINT room_trades_entry_alert_id_fkey FOREIGN KEY (entry_alert_id) REFERENCES public.room_alerts(id);


--
-- Name: room_trades room_trades_exit_alert_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.room_trades
    ADD CONSTRAINT room_trades_exit_alert_id_fkey FOREIGN KEY (exit_alert_id) REFERENCES public.room_alerts(id);


--
-- Name: schedule_exceptions schedule_exceptions_created_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.schedule_exceptions
    ADD CONSTRAINT schedule_exceptions_created_by_fkey FOREIGN KEY (created_by) REFERENCES public.users(id) ON DELETE SET NULL;


--
-- Name: schedule_exceptions schedule_exceptions_schedule_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.schedule_exceptions
    ADD CONSTRAINT schedule_exceptions_schedule_id_fkey FOREIGN KEY (schedule_id) REFERENCES public.trading_room_schedules(id) ON DELETE CASCADE;


--
-- Name: scheduled_content scheduled_content_created_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.scheduled_content
    ADD CONSTRAINT scheduled_content_created_by_fkey FOREIGN KEY (created_by) REFERENCES public.users(id) ON DELETE SET NULL;


--
-- Name: security_events security_events_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.security_events
    ADD CONSTRAINT security_events_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE SET NULL;


--
-- Name: stock_lists stock_lists_created_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.stock_lists
    ADD CONSTRAINT stock_lists_created_by_fkey FOREIGN KEY (created_by) REFERENCES public.users(id);


--
-- Name: trading_room_schedules trading_room_schedules_created_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.trading_room_schedules
    ADD CONSTRAINT trading_room_schedules_created_by_fkey FOREIGN KEY (created_by) REFERENCES public.users(id) ON DELETE SET NULL;


--
-- Name: trading_room_schedules trading_room_schedules_plan_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.trading_room_schedules
    ADD CONSTRAINT trading_room_schedules_plan_id_fkey FOREIGN KEY (plan_id) REFERENCES public.membership_plans(id) ON DELETE CASCADE;


--
-- Name: trading_room_schedules trading_room_schedules_trader_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.trading_room_schedules
    ADD CONSTRAINT trading_room_schedules_trader_id_fkey FOREIGN KEY (trader_id) REFERENCES public.users(id) ON DELETE SET NULL;


--
-- Name: trading_room_schedules trading_room_schedules_updated_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.trading_room_schedules
    ADD CONSTRAINT trading_room_schedules_updated_by_fkey FOREIGN KEY (updated_by) REFERENCES public.users(id) ON DELETE SET NULL;


--
-- Name: user_activity_log user_activity_log_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_activity_log
    ADD CONSTRAINT user_activity_log_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: user_coupons user_coupons_coupon_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_coupons
    ADD CONSTRAINT user_coupons_coupon_id_fkey FOREIGN KEY (coupon_id) REFERENCES public.coupons(id) ON DELETE CASCADE;


--
-- Name: user_coupons user_coupons_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_coupons
    ADD CONSTRAINT user_coupons_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: user_course_enrollments user_course_enrollments_course_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_course_enrollments
    ADD CONSTRAINT user_course_enrollments_course_id_fkey FOREIGN KEY (course_id) REFERENCES public.courses(id) ON DELETE CASCADE;


--
-- Name: user_course_enrollments user_course_enrollments_last_lesson_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_course_enrollments
    ADD CONSTRAINT user_course_enrollments_last_lesson_id_fkey FOREIGN KEY (last_lesson_id) REFERENCES public.course_lessons(id) ON DELETE SET NULL;


--
-- Name: user_course_enrollments user_course_enrollments_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_course_enrollments
    ADD CONSTRAINT user_course_enrollments_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: user_departments user_departments_department_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_departments
    ADD CONSTRAINT user_departments_department_id_fkey FOREIGN KEY (department_id) REFERENCES public.departments(id) ON DELETE CASCADE;


--
-- Name: user_departments user_departments_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_departments
    ADD CONSTRAINT user_departments_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: user_favorites user_favorites_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_favorites
    ADD CONSTRAINT user_favorites_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: user_indicator_access user_indicator_access_indicator_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_indicator_access
    ADD CONSTRAINT user_indicator_access_indicator_id_fkey FOREIGN KEY (indicator_id) REFERENCES public.indicators_enhanced(id) ON DELETE CASCADE;


--
-- Name: user_indicator_access user_indicator_access_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_indicator_access
    ADD CONSTRAINT user_indicator_access_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: user_indicator_ownership user_indicator_ownership_indicator_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_indicator_ownership
    ADD CONSTRAINT user_indicator_ownership_indicator_id_fkey FOREIGN KEY (indicator_id) REFERENCES public.indicators(id) ON DELETE CASCADE;


--
-- Name: user_indicator_ownership user_indicator_ownership_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_indicator_ownership
    ADD CONSTRAINT user_indicator_ownership_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: user_indicators user_indicators_indicator_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_indicators
    ADD CONSTRAINT user_indicators_indicator_id_fkey FOREIGN KEY (indicator_id) REFERENCES public.indicators(id) ON DELETE CASCADE;


--
-- Name: user_indicators user_indicators_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_indicators
    ADD CONSTRAINT user_indicators_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: user_lesson_progress user_lesson_progress_course_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_lesson_progress
    ADD CONSTRAINT user_lesson_progress_course_id_fkey FOREIGN KEY (course_id) REFERENCES public.courses_enhanced(id) ON DELETE CASCADE;


--
-- Name: user_lesson_progress user_lesson_progress_lesson_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_lesson_progress
    ADD CONSTRAINT user_lesson_progress_lesson_id_fkey FOREIGN KEY (lesson_id) REFERENCES public.course_lessons(id) ON DELETE CASCADE;


--
-- Name: user_lesson_progress user_lesson_progress_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_lesson_progress
    ADD CONSTRAINT user_lesson_progress_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: user_member_segments user_member_segments_segment_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_member_segments
    ADD CONSTRAINT user_member_segments_segment_id_fkey FOREIGN KEY (segment_id) REFERENCES public.member_segments(id) ON DELETE CASCADE;


--
-- Name: user_member_segments user_member_segments_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_member_segments
    ADD CONSTRAINT user_member_segments_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: user_member_tags user_member_tags_tag_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_member_tags
    ADD CONSTRAINT user_member_tags_tag_id_fkey FOREIGN KEY (tag_id) REFERENCES public.member_tags(id) ON DELETE CASCADE;


--
-- Name: user_member_tags user_member_tags_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_member_tags
    ADD CONSTRAINT user_member_tags_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: user_memberships user_memberships_plan_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_memberships
    ADD CONSTRAINT user_memberships_plan_id_fkey FOREIGN KEY (plan_id) REFERENCES public.membership_plans(id) ON DELETE CASCADE;


--
-- Name: user_memberships user_memberships_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_memberships
    ADD CONSTRAINT user_memberships_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: user_mfa_secrets user_mfa_secrets_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_mfa_secrets
    ADD CONSTRAINT user_mfa_secrets_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: user_products user_products_product_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_products
    ADD CONSTRAINT user_products_product_id_fkey FOREIGN KEY (product_id) REFERENCES public.products(id) ON DELETE CASCADE;


--
-- Name: user_products user_products_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_products
    ADD CONSTRAINT user_products_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: user_quiz_attempts user_quiz_attempts_enrollment_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_quiz_attempts
    ADD CONSTRAINT user_quiz_attempts_enrollment_id_fkey FOREIGN KEY (enrollment_id) REFERENCES public.user_course_enrollments(id) ON DELETE CASCADE;


--
-- Name: user_quiz_attempts user_quiz_attempts_quiz_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_quiz_attempts
    ADD CONSTRAINT user_quiz_attempts_quiz_id_fkey FOREIGN KEY (quiz_id) REFERENCES public.course_quizzes(id) ON DELETE CASCADE;


--
-- Name: user_quiz_attempts user_quiz_attempts_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_quiz_attempts
    ADD CONSTRAINT user_quiz_attempts_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: user_status user_status_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_status
    ADD CONSTRAINT user_status_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: user_teams user_teams_team_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_teams
    ADD CONSTRAINT user_teams_team_id_fkey FOREIGN KEY (team_id) REFERENCES public.teams(id) ON DELETE CASCADE;


--
-- Name: user_teams user_teams_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_teams
    ADD CONSTRAINT user_teams_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: users users_deleted_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_deleted_by_fkey FOREIGN KEY (deleted_by) REFERENCES public.users(id);


--
-- Name: video_chapters video_chapters_video_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.video_chapters
    ADD CONSTRAINT video_chapters_video_id_fkey FOREIGN KEY (video_id) REFERENCES public.unified_videos(id) ON DELETE CASCADE;


--
-- Name: video_room_assignments video_room_assignments_video_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.video_room_assignments
    ADD CONSTRAINT video_room_assignments_video_id_fkey FOREIGN KEY (video_id) REFERENCES public.unified_videos(id) ON DELETE CASCADE;


--
-- Name: video_transcripts video_transcripts_video_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.video_transcripts
    ADD CONSTRAINT video_transcripts_video_id_fkey FOREIGN KEY (video_id) REFERENCES public.unified_videos(id) ON DELETE CASCADE;


--
-- Name: video_watch_progress video_watch_progress_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.video_watch_progress
    ADD CONSTRAINT video_watch_progress_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: video_watch_progress video_watch_progress_video_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.video_watch_progress
    ADD CONSTRAINT video_watch_progress_video_id_fkey FOREIGN KEY (video_id) REFERENCES public.unified_videos(id) ON DELETE CASCADE;


--
-- Name: webhook_deliveries webhook_deliveries_webhook_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.webhook_deliveries
    ADD CONSTRAINT webhook_deliveries_webhook_id_fkey FOREIGN KEY (webhook_id) REFERENCES public.webhooks(id) ON DELETE CASCADE;


--
-- Name: webhooks webhooks_created_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.webhooks
    ADD CONSTRAINT webhooks_created_by_fkey FOREIGN KEY (created_by) REFERENCES public.users(id) ON DELETE SET NULL;


--
-- Name: workflow_definitions workflow_definitions_created_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.workflow_definitions
    ADD CONSTRAINT workflow_definitions_created_by_fkey FOREIGN KEY (created_by) REFERENCES public.users(id) ON DELETE SET NULL;


--
-- Name: workflow_transitions workflow_transitions_transitioned_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.workflow_transitions
    ADD CONSTRAINT workflow_transitions_transitioned_by_fkey FOREIGN KEY (transitioned_by) REFERENCES public.users(id) ON DELETE SET NULL;


--
-- Name: workflow_transitions workflow_transitions_workflow_status_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.workflow_transitions
    ADD CONSTRAINT workflow_transitions_workflow_status_id_fkey FOREIGN KEY (workflow_status_id) REFERENCES public.content_workflow_status(id) ON DELETE CASCADE;


--
-- Name: cms_ai_assist_history; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.cms_ai_assist_history ENABLE ROW LEVEL SECURITY;

--
-- Name: cms_ai_assist_history cms_ai_history_own; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_ai_history_own ON public.cms_ai_assist_history USING ((user_id = public.cms_current_user_id()));


--
-- Name: cms_assets; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.cms_assets ENABLE ROW LEVEL SECURITY;

--
-- Name: cms_assets cms_assets_delete; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_assets_delete ON public.cms_assets FOR DELETE USING ((public.cms_current_user_role() = ANY (ARRAY['super_admin'::public.cms_user_role, 'marketing_manager'::public.cms_user_role])));


--
-- Name: cms_assets cms_assets_read; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_assets_read ON public.cms_assets FOR SELECT USING (((deleted_at IS NULL) OR (public.cms_current_user_role() = ANY (ARRAY['super_admin'::public.cms_user_role, 'marketing_manager'::public.cms_user_role]))));


--
-- Name: cms_assets cms_assets_update; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_assets_update ON public.cms_assets FOR UPDATE USING ((public.cms_current_user_role() = ANY (ARRAY['super_admin'::public.cms_user_role, 'marketing_manager'::public.cms_user_role, 'content_editor'::public.cms_user_role])));


--
-- Name: cms_assets cms_assets_write; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_assets_write ON public.cms_assets FOR INSERT WITH CHECK ((public.cms_current_user_role() = ANY (ARRAY['super_admin'::public.cms_user_role, 'marketing_manager'::public.cms_user_role, 'content_editor'::public.cms_user_role, 'weekly_editor'::public.cms_user_role])));


--
-- Name: cms_comments; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.cms_comments ENABLE ROW LEVEL SECURITY;

--
-- Name: cms_comments cms_comments_read; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_comments_read ON public.cms_comments FOR SELECT USING (((deleted_at IS NULL) AND (public.cms_current_user_role() = ANY (ARRAY['super_admin'::public.cms_user_role, 'marketing_manager'::public.cms_user_role, 'content_editor'::public.cms_user_role, 'weekly_editor'::public.cms_user_role, 'developer'::public.cms_user_role]))));


--
-- Name: cms_comments cms_comments_update; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_comments_update ON public.cms_comments FOR UPDATE USING (((created_by = public.cms_current_user_id()) OR (public.cms_current_user_role() = ANY (ARRAY['super_admin'::public.cms_user_role, 'marketing_manager'::public.cms_user_role]))));


--
-- Name: cms_comments cms_comments_write; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_comments_write ON public.cms_comments FOR INSERT WITH CHECK ((public.cms_current_user_role() = ANY (ARRAY['super_admin'::public.cms_user_role, 'marketing_manager'::public.cms_user_role, 'content_editor'::public.cms_user_role, 'weekly_editor'::public.cms_user_role])));


--
-- Name: cms_content; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.cms_content ENABLE ROW LEVEL SECURITY;

--
-- Name: cms_content cms_content_admin_access; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_content_admin_access ON public.cms_content USING ((public.cms_current_user_role() = ANY (ARRAY['super_admin'::public.cms_user_role, 'marketing_manager'::public.cms_user_role]))) WITH CHECK ((public.cms_current_user_role() = ANY (ARRAY['super_admin'::public.cms_user_role, 'marketing_manager'::public.cms_user_role])));


--
-- Name: cms_content cms_content_editor_insert; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_content_editor_insert ON public.cms_content FOR INSERT WITH CHECK ((public.cms_current_user_role() = ANY (ARRAY['super_admin'::public.cms_user_role, 'marketing_manager'::public.cms_user_role, 'content_editor'::public.cms_user_role, 'weekly_editor'::public.cms_user_role])));


--
-- Name: cms_content cms_content_editor_read; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_content_editor_read ON public.cms_content FOR SELECT USING (((public.cms_current_user_role() = ANY (ARRAY['super_admin'::public.cms_user_role, 'marketing_manager'::public.cms_user_role, 'content_editor'::public.cms_user_role, 'weekly_editor'::public.cms_user_role, 'developer'::public.cms_user_role, 'viewer'::public.cms_user_role])) OR ((status = 'published'::public.cms_content_status) AND (deleted_at IS NULL))));


--
-- Name: cms_content cms_content_editor_update; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_content_editor_update ON public.cms_content FOR UPDATE USING (((public.cms_current_user_role() = ANY (ARRAY['super_admin'::public.cms_user_role, 'marketing_manager'::public.cms_user_role])) OR ((public.cms_current_user_role() = ANY (ARRAY['content_editor'::public.cms_user_role, 'weekly_editor'::public.cms_user_role])) AND ((created_by = public.cms_current_user_id()) OR (author_id = public.cms_current_user_id())) AND (status = ANY (ARRAY['draft'::public.cms_content_status, 'in_review'::public.cms_content_status])))));


--
-- Name: cms_content cms_content_public_read; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_content_public_read ON public.cms_content FOR SELECT USING (((status = 'published'::public.cms_content_status) AND (deleted_at IS NULL) AND ((scheduled_publish_at IS NULL) OR (scheduled_publish_at <= now())) AND ((scheduled_unpublish_at IS NULL) OR (scheduled_unpublish_at > now()))));


--
-- Name: cms_content cms_content_weekly_editor; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_content_weekly_editor ON public.cms_content USING (((public.cms_current_user_role() = 'weekly_editor'::public.cms_user_role) AND (content_type = 'weekly_watchlist'::public.cms_content_type))) WITH CHECK (((public.cms_current_user_role() = 'weekly_editor'::public.cms_user_role) AND (content_type = 'weekly_watchlist'::public.cms_content_type)));


--
-- Name: cms_datasource_entries; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.cms_datasource_entries ENABLE ROW LEVEL SECURITY;

--
-- Name: cms_datasource_entries cms_datasource_entries_delete; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_datasource_entries_delete ON public.cms_datasource_entries FOR DELETE USING ((public.cms_current_user_role() = ANY (ARRAY['super_admin'::public.cms_user_role, 'marketing_manager'::public.cms_user_role, 'content_editor'::public.cms_user_role])));


--
-- Name: cms_datasource_entries cms_datasource_entries_insert; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_datasource_entries_insert ON public.cms_datasource_entries FOR INSERT WITH CHECK ((public.cms_current_user_role() = ANY (ARRAY['super_admin'::public.cms_user_role, 'marketing_manager'::public.cms_user_role, 'content_editor'::public.cms_user_role])));


--
-- Name: cms_datasource_entries cms_datasource_entries_read; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_datasource_entries_read ON public.cms_datasource_entries FOR SELECT USING (true);


--
-- Name: cms_datasource_entries cms_datasource_entries_update; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_datasource_entries_update ON public.cms_datasource_entries FOR UPDATE USING ((public.cms_current_user_role() = ANY (ARRAY['super_admin'::public.cms_user_role, 'marketing_manager'::public.cms_user_role, 'content_editor'::public.cms_user_role])));


--
-- Name: cms_datasources; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.cms_datasources ENABLE ROW LEVEL SECURITY;

--
-- Name: cms_datasources cms_datasources_delete; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_datasources_delete ON public.cms_datasources FOR DELETE USING (((public.cms_current_user_role() = 'super_admin'::public.cms_user_role) AND (is_system = false)));


--
-- Name: cms_datasources cms_datasources_insert; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_datasources_insert ON public.cms_datasources FOR INSERT WITH CHECK ((public.cms_current_user_role() = ANY (ARRAY['super_admin'::public.cms_user_role, 'marketing_manager'::public.cms_user_role, 'content_editor'::public.cms_user_role])));


--
-- Name: cms_datasources cms_datasources_read; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_datasources_read ON public.cms_datasources FOR SELECT USING ((deleted_at IS NULL));


--
-- Name: cms_datasources cms_datasources_update; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_datasources_update ON public.cms_datasources FOR UPDATE USING (((public.cms_current_user_role() = ANY (ARRAY['super_admin'::public.cms_user_role, 'marketing_manager'::public.cms_user_role])) OR ((public.cms_current_user_role() = 'content_editor'::public.cms_user_role) AND (is_locked = false) AND (is_system = false))));


--
-- Name: cms_offline_sync_queue; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.cms_offline_sync_queue ENABLE ROW LEVEL SECURITY;

--
-- Name: cms_presets; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.cms_presets ENABLE ROW LEVEL SECURITY;

--
-- Name: cms_presets cms_presets_delete; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_presets_delete ON public.cms_presets FOR DELETE USING ((public.cms_current_user_role() = ANY (ARRAY['super_admin'::public.cms_user_role, 'marketing_manager'::public.cms_user_role])));


--
-- Name: cms_presets cms_presets_insert; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_presets_insert ON public.cms_presets FOR INSERT WITH CHECK ((public.cms_current_user_role() = ANY (ARRAY['super_admin'::public.cms_user_role, 'marketing_manager'::public.cms_user_role, 'content_editor'::public.cms_user_role])));


--
-- Name: cms_presets cms_presets_read; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_presets_read ON public.cms_presets FOR SELECT USING (((deleted_at IS NULL) AND ((is_global = true) OR (created_by = public.cms_current_user_id()))));


--
-- Name: cms_presets cms_presets_update; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_presets_update ON public.cms_presets FOR UPDATE USING (((public.cms_current_user_role() = ANY (ARRAY['super_admin'::public.cms_user_role, 'marketing_manager'::public.cms_user_role])) OR ((public.cms_current_user_role() = 'content_editor'::public.cms_user_role) AND (created_by = public.cms_current_user_id()) AND (is_locked = false))));


--
-- Name: cms_release_items; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.cms_release_items ENABLE ROW LEVEL SECURITY;

--
-- Name: cms_release_items cms_release_items_delete_policy; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_release_items_delete_policy ON public.cms_release_items FOR DELETE USING ((EXISTS ( SELECT 1
   FROM public.cms_releases r
  WHERE ((r.id = cms_release_items.release_id) AND (r.status = 'draft'::public.cms_release_status) AND ((public.cms_get_current_user_role() = ANY (ARRAY['super_admin'::public.cms_user_role, 'marketing_manager'::public.cms_user_role])) OR (r.created_by = (((current_setting('request.jwt.claims'::text, true))::jsonb ->> 'sub'::text))::bigint))))));


--
-- Name: cms_release_items cms_release_items_insert_policy; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_release_items_insert_policy ON public.cms_release_items FOR INSERT WITH CHECK ((EXISTS ( SELECT 1
   FROM public.cms_releases r
  WHERE ((r.id = cms_release_items.release_id) AND (r.status = 'draft'::public.cms_release_status) AND ((public.cms_get_current_user_role() = ANY (ARRAY['super_admin'::public.cms_user_role, 'marketing_manager'::public.cms_user_role])) OR (r.created_by = (((current_setting('request.jwt.claims'::text, true))::jsonb ->> 'sub'::text))::bigint))))));


--
-- Name: cms_release_items cms_release_items_select_policy; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_release_items_select_policy ON public.cms_release_items FOR SELECT USING ((EXISTS ( SELECT 1
   FROM public.cms_releases r
  WHERE (r.id = cms_release_items.release_id))));


--
-- Name: cms_release_items cms_release_items_update_policy; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_release_items_update_policy ON public.cms_release_items FOR UPDATE USING ((EXISTS ( SELECT 1
   FROM public.cms_releases r
  WHERE ((r.id = cms_release_items.release_id) AND (r.status = 'draft'::public.cms_release_status) AND ((public.cms_get_current_user_role() = ANY (ARRAY['super_admin'::public.cms_user_role, 'marketing_manager'::public.cms_user_role])) OR (r.created_by = (((current_setting('request.jwt.claims'::text, true))::jsonb ->> 'sub'::text))::bigint))))));


--
-- Name: cms_releases; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.cms_releases ENABLE ROW LEVEL SECURITY;

--
-- Name: cms_releases cms_releases_delete_policy; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_releases_delete_policy ON public.cms_releases FOR DELETE USING ((public.cms_get_current_user_role() = 'super_admin'::public.cms_user_role));


--
-- Name: cms_releases cms_releases_insert_policy; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_releases_insert_policy ON public.cms_releases FOR INSERT WITH CHECK ((public.cms_get_current_user_role() = ANY (ARRAY['super_admin'::public.cms_user_role, 'marketing_manager'::public.cms_user_role])));


--
-- Name: cms_releases cms_releases_select_policy; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_releases_select_policy ON public.cms_releases FOR SELECT USING (((public.cms_get_current_user_role() = ANY (ARRAY['super_admin'::public.cms_user_role, 'marketing_manager'::public.cms_user_role])) OR (created_by = (((current_setting('request.jwt.claims'::text, true))::jsonb ->> 'sub'::text))::bigint)));


--
-- Name: cms_releases cms_releases_update_policy; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_releases_update_policy ON public.cms_releases FOR UPDATE USING (((public.cms_get_current_user_role() = ANY (ARRAY['super_admin'::public.cms_user_role, 'marketing_manager'::public.cms_user_role])) OR ((created_by = (((current_setting('request.jwt.claims'::text, true))::jsonb ->> 'sub'::text))::bigint) AND (status = 'draft'::public.cms_release_status))));


--
-- Name: cms_reusable_blocks; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.cms_reusable_blocks ENABLE ROW LEVEL SECURITY;

--
-- Name: cms_reusable_blocks cms_reusable_blocks_delete; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_reusable_blocks_delete ON public.cms_reusable_blocks FOR DELETE USING (((public.cms_current_user_role() = 'super_admin'::public.cms_user_role) OR ((created_by = public.cms_current_user_id()) AND (NOT is_locked) AND (usage_count = 0))));


--
-- Name: cms_reusable_blocks cms_reusable_blocks_insert; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_reusable_blocks_insert ON public.cms_reusable_blocks FOR INSERT WITH CHECK ((public.cms_current_user_role() = ANY (ARRAY['super_admin'::public.cms_user_role, 'marketing_manager'::public.cms_user_role, 'content_editor'::public.cms_user_role])));


--
-- Name: cms_reusable_blocks cms_reusable_blocks_select; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_reusable_blocks_select ON public.cms_reusable_blocks FOR SELECT USING (((is_global = true) OR (created_by = public.cms_current_user_id())));


--
-- Name: cms_reusable_blocks cms_reusable_blocks_update; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_reusable_blocks_update ON public.cms_reusable_blocks FOR UPDATE USING (((public.cms_current_user_role() = ANY (ARRAY['super_admin'::public.cms_user_role, 'marketing_manager'::public.cms_user_role])) OR ((created_by = public.cms_current_user_id()) AND (NOT is_locked))));


--
-- Name: cms_revisions; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.cms_revisions ENABLE ROW LEVEL SECURITY;

--
-- Name: cms_revisions cms_revisions_insert; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_revisions_insert ON public.cms_revisions FOR INSERT WITH CHECK ((public.cms_current_user_role() = ANY (ARRAY['super_admin'::public.cms_user_role, 'marketing_manager'::public.cms_user_role, 'content_editor'::public.cms_user_role, 'weekly_editor'::public.cms_user_role])));


--
-- Name: cms_revisions cms_revisions_read; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_revisions_read ON public.cms_revisions FOR SELECT USING ((public.cms_current_user_role() = ANY (ARRAY['super_admin'::public.cms_user_role, 'marketing_manager'::public.cms_user_role, 'content_editor'::public.cms_user_role, 'developer'::public.cms_user_role])));


--
-- Name: cms_schedule_history; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.cms_schedule_history ENABLE ROW LEVEL SECURITY;

--
-- Name: cms_schedule_history cms_schedule_history_delete_policy; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_schedule_history_delete_policy ON public.cms_schedule_history FOR DELETE USING ((public.cms_get_current_user_role() = 'super_admin'::public.cms_user_role));


--
-- Name: cms_schedule_history cms_schedule_history_insert_policy; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_schedule_history_insert_policy ON public.cms_schedule_history FOR INSERT WITH CHECK (((public.cms_get_current_user_role() = ANY (ARRAY['super_admin'::public.cms_user_role, 'marketing_manager'::public.cms_user_role])) OR true));


--
-- Name: cms_schedule_history cms_schedule_history_select_policy; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_schedule_history_select_policy ON public.cms_schedule_history FOR SELECT USING (((public.cms_get_current_user_role() = ANY (ARRAY['super_admin'::public.cms_user_role, 'marketing_manager'::public.cms_user_role])) OR (performed_by = (((current_setting('request.jwt.claims'::text, true))::jsonb ->> 'sub'::text))::bigint) OR (EXISTS ( SELECT 1
   FROM public.cms_schedules s
  WHERE ((s.id = cms_schedule_history.schedule_id) AND (s.created_by = (((current_setting('request.jwt.claims'::text, true))::jsonb ->> 'sub'::text))::bigint)))) OR (EXISTS ( SELECT 1
   FROM public.cms_releases r
  WHERE ((r.id = cms_schedule_history.release_id) AND (r.created_by = (((current_setting('request.jwt.claims'::text, true))::jsonb ->> 'sub'::text))::bigint))))));


--
-- Name: cms_schedule_history cms_schedule_history_update_policy; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_schedule_history_update_policy ON public.cms_schedule_history FOR UPDATE USING (false);


--
-- Name: cms_schedules; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.cms_schedules ENABLE ROW LEVEL SECURITY;

--
-- Name: cms_schedules cms_schedules_delete_policy; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_schedules_delete_policy ON public.cms_schedules FOR DELETE USING ((public.cms_get_current_user_role() = ANY (ARRAY['super_admin'::public.cms_user_role, 'marketing_manager'::public.cms_user_role])));


--
-- Name: cms_schedules cms_schedules_insert_policy; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_schedules_insert_policy ON public.cms_schedules FOR INSERT WITH CHECK ((public.cms_get_current_user_role() = ANY (ARRAY['super_admin'::public.cms_user_role, 'marketing_manager'::public.cms_user_role, 'content_editor'::public.cms_user_role])));


--
-- Name: cms_schedules cms_schedules_select_policy; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_schedules_select_policy ON public.cms_schedules FOR SELECT USING (((public.cms_get_current_user_role() = ANY (ARRAY['super_admin'::public.cms_user_role, 'marketing_manager'::public.cms_user_role])) OR (created_by = (((current_setting('request.jwt.claims'::text, true))::jsonb ->> 'sub'::text))::bigint)));


--
-- Name: cms_schedules cms_schedules_update_policy; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_schedules_update_policy ON public.cms_schedules FOR UPDATE USING (((public.cms_get_current_user_role() = ANY (ARRAY['super_admin'::public.cms_user_role, 'marketing_manager'::public.cms_user_role])) OR ((created_by = (((current_setting('request.jwt.claims'::text, true))::jsonb ->> 'sub'::text))::bigint) AND (status = 'pending'::public.cms_schedule_status))));


--
-- Name: cms_offline_sync_queue cms_sync_queue_own; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_sync_queue_own ON public.cms_offline_sync_queue USING ((user_id = public.cms_current_user_id()));


--
-- Name: cms_user_editor_preferences; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.cms_user_editor_preferences ENABLE ROW LEVEL SECURITY;

--
-- Name: cms_user_editor_preferences cms_user_prefs_own; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY cms_user_prefs_own ON public.cms_user_editor_preferences USING ((user_id = public.cms_current_user_id()));


--
-- PostgreSQL database dump complete
--



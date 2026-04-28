ALTER TABLE user_memberships
    DROP CONSTRAINT IF EXISTS uq_user_memberships_user_plan;

ALTER TABLE user_memberships
    ADD CONSTRAINT uq_user_memberships_user_plan UNIQUE (user_id, plan_id);

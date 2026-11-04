-- Never infer historic membership from current mutable rules or member facts.
ALTER TABLE member_segments ADD COLUMN authoring_version BIGINT NOT NULL DEFAULT 1 CHECK(authoring_version>0);
CREATE TABLE member_segment_observations (
    segment_id BIGINT NOT NULL REFERENCES member_segments(id) ON DELETE CASCADE,
    authoring_version BIGINT NOT NULL CHECK(authoring_version>0),
    observed_date DATE NOT NULL DEFAULT ((NOW() AT TIME ZONE 'UTC')::date),
    observed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    member_count BIGINT NOT NULL CHECK(member_count>=0),
    PRIMARY KEY(segment_id,authoring_version,observed_date)
);
CREATE INDEX member_segment_observation_time ON member_segment_observations(segment_id,authoring_version,observed_at DESC);

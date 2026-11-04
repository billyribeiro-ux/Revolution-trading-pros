CREATE TABLE performance_samples (
    measurement_id TEXT NOT NULL CHECK(length(measurement_id) BETWEEN 1 AND 128),
    metric TEXT NOT NULL CHECK(length(metric) BETWEEN 1 AND 80),
    value DOUBLE PRECISION NOT NULL CHECK(value>=0 AND value<=86400000 AND value<>'NaN'::float8),
    page_url TEXT,
    device TEXT NOT NULL CHECK(device IN ('mobile','desktop','unknown')),
    observed_at TIMESTAMPTZ NOT NULL,
    received_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    navigation_type TEXT,
    PRIMARY KEY(measurement_id,metric)
);
CREATE INDEX performance_samples_window ON performance_samples(observed_at DESC,metric);
CREATE INDEX performance_samples_page ON performance_samples(page_url,device,observed_at DESC);
